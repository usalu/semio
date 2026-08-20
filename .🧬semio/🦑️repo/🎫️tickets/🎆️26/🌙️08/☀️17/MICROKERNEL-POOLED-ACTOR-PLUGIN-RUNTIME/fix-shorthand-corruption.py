#!/usr/bin/env python3
"""🩹 Repair struct field-init-shorthand `.await` corruption (coordinator-flagged failure mode 2).

WHY
---
`IDENT` used as Rust field-init shorthand (`StructName { diff }` meaning `diff: diff`) was, at
some point in this ticket's tool history, wrongly given a trailing `.await` by an over-eager
await-insertion pass wherever `IDENT` also appeared elsewhere needing a genuine await (e.g.
`is_array_diff_empty(&diff).await` on the line above `Some(JsonValueDiff::Array { diff.await })`).
`diff.await` is not valid field-shorthand syntax (field names are plain identifiers), so this is a
hard PARSE error — `expected one of ',', ':', or '}', found '.'` — which blocks ALL further
diagnostics for that file, not just a type error.

WHAT IT DOES
------------
Reads rustc's own parse-error diagnostics (message starts with "expected one of `,`, `:`, or `}`,
found `.`") and uses the diagnostic's own span, which points EXACTLY at the illegal `.` token.
Verifies the six bytes at that position are literally `.await` and that the immediately preceding
character is an identifier character (so this really is `IDENT.await` in shorthand-field
position, not some other `.` misparse), then deletes exactly `.await` (6 bytes). Nothing else.

Span-keyed (R10): only ever touches a byte range rustc's own parser flagged.
"""
from __future__ import annotations
import argparse, json, os, subprocess, sys
from collections import defaultdict

REPO = "/Users/ueli/Documents/semio"
IDENT_BYTE = set(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_")


def run_check(crate: str, target_dir: str) -> list[dict]:
    cmd = ["cargo", "check", "-p", crate, "--message-format=json", "--lib"]
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


def in_scope(abs_path: str, scope: str) -> bool:
    rel = os.path.relpath(abs_path, REPO)
    parts = rel.split(os.sep)
    want = [p for p in scope.strip("/").split("/") if p]
    if not want:
        return True
    return any(parts[i:i + len(want)] == want for i in range(len(parts) - len(want) + 1))


def collect(diags, scope):
    sites, other = [], []
    for d in diags:
        if d.get("level") != "error":
            continue
        msg = d.get("message") or ""
        if not msg.startswith("expected one of `,`, `:`, or `}`, found `.`"):
            continue
        for sp in d.get("spans", []):
            if not sp.get("is_primary"):
                continue
            fn = sp.get("file_name", "")
            path = os.path.normpath(fn if os.path.isabs(fn) else os.path.join(REPO, fn))
            if scope and not in_scope(path, scope):
                other.append((path, "out-of-scope"))
                continue
            sites.append((path, sp["byte_start"], sp["byte_end"]))
    return sites, other


def apply_edits(sites, guard):
    by_file = defaultdict(list)
    for path, bs, be in sites:
        key = (path, bs)
        if key in guard:
            continue
        guard.add(key)
        by_file[path].append(bs)

    applied, refused = 0, []
    for path, positions in by_file.items():
        with open(path, "rb") as fh:
            data = fh.read()
        positions = sorted(set(positions), reverse=True)
        last = None
        for dot_pos in positions:
            if last is not None and dot_pos + 6 > last:
                continue
            if data[dot_pos:dot_pos + 6] != b".await":
                refused.append((path, f"at {dot_pos}: {data[dot_pos:dot_pos+6]!r}, not '.await'"))
                continue
            if dot_pos == 0 or (data[dot_pos - 1:dot_pos] not in [bytes([b]) for b in IDENT_BYTE]):
                refused.append((path, f"at {dot_pos}: preceding byte not an identifier char"))
                continue
            data = data[:dot_pos] + data[dot_pos + 6:]
            last = dot_pos
            applied += 1
        with open(path, "wb") as fh:
            fh.write(data)
    return applied, refused


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--crate", required=True)
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--scope", default=None)
    ap.add_argument("--max-passes", type=int, default=6)
    ap.add_argument("--target-dir", required=True)
    args = ap.parse_args()

    if not args.apply and not args.dry_run:
        ap.error("choose --apply or --dry-run")

    guard = set()
    for npass in range(1, args.max_passes + 1):
        diags = run_check(args.crate, args.target_dir)
        sites, other = collect(diags, args.scope)
        print(f"[pass {npass}] shorthand-corruption sites={len(sites)} skipped={len(other)}")
        if args.dry_run:
            for path, bs, be in sites[:60]:
                print(f"    FIX {os.path.relpath(path, REPO)} [{bs}:{be}]")
            break
        if not sites:
            print("  fixpoint reached")
            break
        applied, refused = apply_edits(sites, guard)
        print(f"  applied {applied}")
        for path, why in refused:
            print(f"  !! {os.path.relpath(path, REPO)}: {why}")
        if applied == 0:
            print("  no progress — stopping")
            break
    return 0


if __name__ == "__main__":
    sys.exit(main())
