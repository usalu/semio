#!/usr/bin/env python3
"""✂️ Remove `.await` immediately after (or inside the argument of) a `resolve_ready(...)` call.

WHY
---
`resolve_ready<F: Future>(fut: F) -> F::Output` already drives the future to completion and
returns its plain (non-Future) output. Two residue shapes put an ADDITIONAL, illegal `.await`
right next to it — both are E0728 ("await outside async") when (as is almost always the case
here) the whole expression sits inside a sync closure:

  1. `semio_framework_plugin::resolve_ready(CALL(...)).await` — `insert-await.py` or a hand pass
     added `.await` on the OUTER call not realizing `resolve_ready` already unwraps it.
     `wrap-sync-closure-await.py` correctly refuses to re-wrap these (its own "already bridged"
     guard), which is why they survive as bare E0728 residue instead of becoming a corruption.
  2. `semio_framework_plugin::resolve_ready(RECEIVER.method().await)` — the `.await` landed
     INSIDE the resolve_ready argument instead of the wrap replacing it.

WHAT IT DOES
------------
For each in-scope E0728 diagnostic whose primary span is literally `.await` (verified byte
content), checks the immediately preceding text: if it is exactly a `resolve_ready(...)` call's
closing paren (shape 1) OR the whole enclosing call is itself the argument of an ENCLOSING
`resolve_ready(` visible via a bounded backward scan (shape 2), deletes the `.await` (and its
preceding `.`). Refuses (reports, does not touch) anything else — this tool does not attempt the
harder "await on a bare place expression" residue (`hoist-place-await.py`'s job).
"""
from __future__ import annotations
import argparse, json, os, subprocess, sys
from collections import defaultdict

REPO = "/Users/ueli/Documents/semio"
BRIDGE = b"semio_framework_plugin::resolve_ready("


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


def find_matching_open_paren(data: bytes, close_idx: int) -> int:
    depth = 0
    i = close_idx
    while i >= 0:
        c = data[i:i + 1]
        if c == b")":
            depth += 1
        elif c == b"(":
            depth -= 1
            if depth == 0:
                return i
        i -= 1
    return -1


def collect(diags, scope):
    sites, other = [], []
    for d in diags:
        if d.get("level") != "error":
            continue
        if (d.get("code") or {}).get("code") != "E0728":
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
        key = (path, bs, be)
        if key in guard:
            continue
        guard.add(key)
        by_file[path].append((bs, be))

    applied, refused = 0, []
    for path, items in by_file.items():
        with open(path, "rb") as fh:
            data = fh.read()
        items = sorted(set(items), key=lambda t: t[0], reverse=True)
        last_start = None
        for bs, be in items:
            if last_start is not None and be > last_start:
                continue
            if data[bs:be] != b"await" or bs == 0 or data[bs - 1:bs] != b".":
                refused.append((path, f"span {bs}:{be} = {data[bs:be]!r}, not '.await'"))
                continue
            dot_idx = bs - 1

            fixed = False
            # Shape 1: `resolve_ready(...).await` — dot preceded by ')'.
            if dot_idx > 0 and data[dot_idx - 1:dot_idx] == b")":
                open_paren = find_matching_open_paren(data, dot_idx - 1)
                if open_paren != -1:
                    callee_start = open_paren
                    while callee_start > 0 and (data[callee_start - 1:callee_start].isalnum()
                                                 or data[callee_start - 1:callee_start] in (b"_", b":")):
                        callee_start -= 1
                    if data[callee_start:callee_start + len(BRIDGE)] == BRIDGE:
                        data = data[:dot_idx] + data[be:]  # delete ".await"
                        last_start = dot_idx
                        applied += 1
                        fixed = True
            if fixed:
                continue

            # Shape 2: `.await` sits inside an ENCLOSING resolve_ready( ... ) argument list —
            # scan outward through enclosing parens looking for one opened by "resolve_ready(".
            depth = 0
            i = dot_idx - 1
            found_enclosing = False
            while i >= 0:
                c = data[i:i + 1]
                if c == b")":
                    depth += 1
                elif c == b"(":
                    if depth == 0:
                        callee_start = i
                        while callee_start > 0 and (data[callee_start - 1:callee_start].isalnum()
                                                     or data[callee_start - 1:callee_start] in (b"_", b":")):
                            callee_start -= 1
                        if data[callee_start:callee_start + len(BRIDGE)] == BRIDGE:
                            found_enclosing = True
                        break
                    depth -= 1
                elif c == b";" or c == b"{":
                    break  # left the statement without finding an enclosing resolve_ready
                i -= 1
            if found_enclosing:
                data = data[:dot_idx] + data[be:]
                last_start = dot_idx
                applied += 1
                continue

            refused.append((path, f"span {bs}:{be}: no resolve_ready(...) found nearby"))
        with open(path, "wb") as fh:
            fh.write(data)
    return applied, refused


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--crate", required=True)
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--scope", default=None)
    ap.add_argument("--max-passes", type=int, default=8)
    ap.add_argument("--target-dir", required=True)
    args = ap.parse_args()

    if not args.apply and not args.dry_run:
        ap.error("choose --apply or --dry-run")

    guard = set()
    for npass in range(1, args.max_passes + 1):
        diags = run_check(args.crate, args.target_dir)
        sites, other = collect(diags, args.scope)
        print(f"[pass {npass}] E0728 candidate sites={len(sites)} skipped={len(other)}")
        if args.dry_run:
            for path, bs, be in sites[:60]:
                print(f"    STRIP {os.path.relpath(path, REPO)} [{bs}:{be}]")
            break
        if not sites:
            print("  fixpoint reached")
            break
        applied, refused = apply_edits(sites, guard)
        print(f"  applied {applied}")
        for path, why in refused[:20]:
            print(f"  !! {os.path.relpath(path, REPO)}: {why}")
        if applied == 0:
            print("  no progress — stopping")
            break
    return 0


if __name__ == "__main__":
    sys.exit(main())
