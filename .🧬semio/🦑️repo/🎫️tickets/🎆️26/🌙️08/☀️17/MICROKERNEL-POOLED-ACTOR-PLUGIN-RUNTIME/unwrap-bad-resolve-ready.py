#!/usr/bin/env python3
"""🌉 Unwrap `resolve_ready(EXPR)` where EXPR is no longer a Future — post-R9-revert fallout.

WHY
---
`wrap-sync-closure-await.py` bridges `.await` trapped in a sync closure by wrapping the callee in
`semio_framework_plugin::resolve_ready(...)`, which is correct ONLY when the callee genuinely
returns a `Future`. After an R9 whole-file revert (e.g. `geometry-core`'s 44 pure-math fns went
from `async fn` back to plain `fn`), a call site that used to need the bridge no longer does — but
the bridge-wrapped text from an earlier pass is still there, now wrapping a plain value:
`semio_framework_plugin::resolve_ready(dot(a, a))` where `dot` is now sync, producing
`E0277: f64 is not a future`.

WHAT IT DOES
------------
For every E0277 "X is not a future" diagnostic whose primary span's byte content is EXACTLY
`semio_framework_plugin::resolve_ready(` + something + `)` (verified before touching anything),
deletes the `semio_framework_plugin::resolve_ready(` prefix and the matching trailing `)`,
leaving the inner expression bare. Span-keyed per R10 — only ever unwraps a call rustc itself just
flagged as not implementing `Future`.
"""
from __future__ import annotations
import argparse, json, os, subprocess, sys
from collections import defaultdict

REPO = "/Users/ueli/Documents/semio"
PREFIX = b"semio_framework_plugin::resolve_ready("


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
        if not ((d.get("code") or {}).get("code") == "E0277" and "is not a future" in msg):
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
            chunk = data[bs:be]
            if chunk.startswith(PREFIX) and chunk.endswith(b")"):
                inner = chunk[len(PREFIX):-1]
                data = data[:bs] + inner + data[be:]
                last_start = bs
                applied += 1
                continue
            refused.append((path, f"span {bs}:{be} = {chunk[:60]!r}, not a resolve_ready(...) wrapper"))
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
        print(f"[pass {npass}] candidate sites={len(sites)} skipped={len(other)}")
        if args.dry_run:
            for path, bs, be in sites[:60]:
                print(f"    UNWRAP {os.path.relpath(path, REPO)} [{bs}:{be}]")
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
