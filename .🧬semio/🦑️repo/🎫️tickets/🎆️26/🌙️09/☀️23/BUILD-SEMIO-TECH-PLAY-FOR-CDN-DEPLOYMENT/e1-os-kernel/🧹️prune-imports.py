#!/usr/bin/env python3
"""🧹️ Removes the imports rustc reports unused in the given files (E1 store moves).

The kernel allows `unused_imports` crate-wide, so the lint is forced for the kernel unit only
(`cargo rustc -p semio-framework-os-kernel --lib --profile check -- --force-warn unused-imports`, private E1 dirs,
through `🚦️cargo-slot.sh`); rustc's own removal suggestions are applied, back to front, in the listed files only.
Test-only uses then come back gated through `🔁️fix-loop.py --files .. -- --profile test`.
Usage: prune-imports.py FILE [FILE ...] [-- CARGO_ARGS]
"""
import json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
E1 = f"{REPO}/.🧬semio/🦑️repo/⚡️cache/e1/os-kernel"
SLOT = os.path.join(os.path.dirname(HERE), "🚦️cargo-slot.sh")


def main():
    args = sys.argv[1:sys.argv.index("--")] if "--" in sys.argv else sys.argv[1:]
    extra = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    files = {os.path.normpath(os.path.join(REPO, f)) for f in args}
    env = dict(os.environ, CARGO_TARGET_DIR=f"{E1}/t", CARGO_BUILD_BUILD_DIR=f"{E1}/b", CARGO_INCREMENTAL="0", CARGO_TERM_COLOR="never")
    cmd = [SLOT, "e1-os-kernel", "--", "cargo", "rustc", "-p", "semio-framework-os-kernel", "--lib", "--profile", "check", "--message-format=json"] + extra + ["--", "--force-warn", "unused-imports"]
    out = subprocess.run(cmd, cwd=REPO, env=env, capture_output=True, text=True).stdout
    edits = {}
    for line in out.split("\n"):
        try: j = json.loads(line)
        except ValueError: continue
        if j.get("reason") != "compiler-message": continue
        m = j["message"]
        if (m.get("code") or {}).get("code") != "unused_imports": continue
        for c in m.get("children", []):
            for s in c.get("spans", []):
                f = os.path.normpath(os.path.join(REPO, s["file_name"]))
                if f in files and s.get("suggested_replacement") is not None:
                    edits.setdefault(f, set()).add((s["byte_start"], s["byte_end"], s["suggested_replacement"]))
    for f, es in edits.items():
        data = open(f, "rb").read()
        last = None
        for a, b, r in sorted(es, reverse=True):
            if last is not None and b > last: continue
            data = data[:a] + r.encode() + data[b:]
            last = a
        open(f, "wb").write(data)
        print(f"[prune] {os.path.relpath(f, REPO)}: {len(es)} removals")
    if not edits: print("[prune] nothing unused")


if __name__ == "__main__":
    main()
