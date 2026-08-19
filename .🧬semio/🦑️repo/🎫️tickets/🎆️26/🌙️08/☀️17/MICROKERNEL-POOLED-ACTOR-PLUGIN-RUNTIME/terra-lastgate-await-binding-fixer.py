#!/usr/bin/env python3
"""🪢 Diagnostic-driven `.await`-at-binding fixer for the `--all-targets` test-code residue.

WHY THIS EXISTS
---------------
`insert-await.py` covers diagnostics where rustc's OWN suggestion inserts `.await` right at the
error site (AWAIT_CODES). A large remaining class in this crate's test code is different: a test
fixture is bound via `let def = some_async_fn(...);` (no `.await`), then used many lines later as
`&def` or `def.field`. rustc reports the error at the USE site ("expected &T, found &impl
Future<Output = T>", or E0600/E0369/E0716/moved-value E0382 variants) and, critically, offers NO
`suggested_replacement` for this shape — `insert-await.py` correctly refuses to touch it and drops
it into "other". The real fix is at the BINDING, not the use site.

METHOD (still diagnostic-driven, still span-keyed — see R10)
--------------------------------------------------------------
For each error whose primary span covers a bare `&IDENT` or `IDENT` (verified by reading the exact
bytes at the reported span, never by guessing from the rendered text), search BACKWARD from that
byte offset in the same file for the nearest preceding line of the exact shape
`let [mut] IDENT = <expr>;` that does not already end in `.await;`, bounded by the nearest
preceding `async fn` / `#[...test]` line (never crossing into a different test function — this is
the safety boundary that keeps this from becoming the name-keyed global editor R10 bans). If found,
appends `.await` right before the statement's closing `;`.

Only handles the "await the binding" shape. Does NOT touch inline-call arguments, closures, or
anything ambiguous — those are left for the report's "skipped" list and a human.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from collections import defaultdict

REPO = "/Users/ueli/Documents/semio"

IDENT_RE = re.compile(r"^&?([A-Za-z_][A-Za-z0-9_]*)$")
LET_RE_TMPL = r"let\s+(mut\s+)?{name}\s*=\s*(.*?);\s*$"


def in_scope(abs_path: str, scope: str) -> bool:
    rel = os.path.relpath(abs_path, REPO)
    parts = rel.split(os.sep)
    want = [p for p in scope.strip("/").split("/") if p]
    if not want:
        return True
    for i in range(len(parts) - len(want) + 1):
        if parts[i:i + len(want)] == want:
            return True
    return False


def run_check(crate: str, target_dir: str, all_targets: bool) -> list[dict]:
    cmd = ["cargo", "check", "-p", crate, "--message-format=json",
           "--all-targets" if all_targets else "--lib"]
    env = dict(os.environ, CARGO_TARGET_DIR=target_dir)
    proc = subprocess.run(cmd, cwd=REPO, env=env, capture_output=True, text=True)
    out = []
    for line in proc.stdout.splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            msg = json.loads(line)
        except json.JSONDecodeError:
            continue
        if msg.get("reason") == "compiler-message" and msg.get("message"):
            out.append(msg["message"])
    return out


TARGET_CODES = {"E0308", "E0369", "E0600", "E0716", "E0277", "E0382", "E0599"}


LET_START_RE_TMPL = r"^(\s*)let\s+(mut\s+)?{name}\s*="


def _statement_end(full_text: str, start_idx: int) -> int | None:
    """🧮 From the `=` sign of a `let NAME = ...` (index just after it), bracket/paren/brace-match
    forward to find this statement's own terminating top-level `;` — handles multi-line
    initializers. Returns the absolute index of that `;`, or None if unbalanced/not found."""
    depth = 0
    i = start_idx
    n = len(full_text)
    while i < n:
        c = full_text[i]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif c == ";" and depth == 0:
            return i
        i += 1
    return None


def find_binding_fix(abs_path: str, file_bytes: bytes, byte_start: int, byte_end: int):
    """🔎 Reads the exact span text; if it's a bare identifier (optionally `&`-prefixed), searches
    backward for its nearest un-awaited `let` binding (single- or multi-line initializer), bounded
    by the nearest preceding test fn."""
    span_text = file_bytes[byte_start:byte_end].decode("utf-8", errors="replace")
    m = IDENT_RE.match(span_text.strip())
    if not m:
        return None
    name = m.group(1)

    full_text = file_bytes.decode("utf-8", errors="replace")
    prefix = full_text[:byte_start]
    lines = prefix.splitlines(keepends=True)

    boundary = 0
    for i in range(len(lines) - 1, -1, -1):
        if re.search(r"\basync fn\b", lines[i]) or "#[" in lines[i] and "test" in lines[i]:
            boundary = i
            break
    boundary_byte = sum(len(l.encode("utf-8")) for l in lines[:boundary])

    let_start_re = re.compile(LET_START_RE_TMPL.format(name=re.escape(name)), re.MULTILINE)
    candidates = list(let_start_re.finditer(prefix))
    for mm in reversed(candidates):
        if mm.start() < boundary_byte:
            break
        eq_idx = mm.end()  # index right after the `=`
        semi_idx = _statement_end(full_text, eq_idx)
        if semi_idx is None:
            continue
        stmt = full_text[eq_idx:semi_idx].rstrip()
        if stmt.endswith(".await"):
            return None  # already awaited — some other issue, not ours to fix
        insert_at = len(full_text[:semi_idx].encode("utf-8"))
        return (abs_path, insert_at, insert_at, ".await", f"await binding of `{name}`")
    return None


def collect_edits(diags: list[dict], scope: str | None):
    edits, skipped = [], []
    for diag in diags:
        if diag.get("level") != "error":
            continue
        code = (diag.get("code") or {}).get("code") or ""
        if code not in TARGET_CODES:
            continue
        primary = None
        for sp in diag.get("spans", []):
            if sp.get("is_primary"):
                primary = sp
                break
        if primary is None:
            continue
        path = primary.get("file_name", "")
        abs_path = path if os.path.isabs(path) else os.path.join(REPO, path)
        abs_path = os.path.normpath(abs_path)
        at = f"{path}:{primary.get('line_start')}"
        if scope and not in_scope(abs_path, scope):
            continue
        try:
            with open(abs_path, "rb") as fh:
                data = fh.read()
        except OSError:
            skipped.append((at, code, "cannot read file"))
            continue
        fix = find_binding_fix(abs_path, data, primary["byte_start"], primary["byte_end"])
        if fix is None:
            skipped.append((at, code, diag.get("message", "")[:120]))
            continue
        edits.append(fix)
    return edits, skipped


def apply_edits(edits, guard: set) -> int:
    by_file = defaultdict(list)
    for path, bs, be, repl, why in edits:
        key = (path, bs)
        if key in guard:
            continue
        guard.add(key)
        by_file[path].append((bs, be, repl, why))

    applied = 0
    for path, items in by_file.items():
        with open(path, "rb") as fh:
            data = fh.read()
        items.sort(key=lambda t: t[0], reverse=True)
        taken, last_start = [], None
        for bs, be, repl, why in items:
            if last_start is not None and be > last_start:
                continue
            taken.append((bs, be, repl, why))
            last_start = bs
        for bs, be, repl, _why in taken:
            data = data[:bs] + repl.encode("utf-8") + data[be:]
        with open(path, "wb") as fh:
            fh.write(data)
        applied += len(taken)
    return applied


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--crate", required=True)
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--scope", default=None)
    ap.add_argument("--target-dir", required=True)
    ap.add_argument("--all-targets", action="store_true")
    ap.add_argument("--max-passes", type=int, default=10)
    args = ap.parse_args()
    if not args.apply and not args.dry_run:
        ap.error("choose --apply or --dry-run")

    guard: set = set()
    for npass in range(1, args.max_passes + 1):
        diags = run_check(args.crate, args.target_dir, args.all_targets)
        errors = [d for d in diags if d.get("level") == "error"]
        edits, skipped = collect_edits(diags, args.scope)
        print(f"[pass {npass}] errors={len(errors)} binding-edits={len(edits)} skipped={len(skipped)}")
        if args.dry_run:
            for path, bs, be, repl, why in edits[:40]:
                print(f"    EDIT {os.path.relpath(path, REPO)} @{be} -> insert {repl!r}   ({why})")
            for at, code, msg in skipped[:20]:
                print(f"    SKIP {code} {at}: {msg}")
            break
        if not edits:
            print("  fixpoint reached")
            break
        applied = apply_edits(edits, guard)
        print(f"  applied {applied}")
        if applied == 0:
            print("  no progress — stopping")
            break
    return 0


if __name__ == "__main__":
    sys.exit(main())
