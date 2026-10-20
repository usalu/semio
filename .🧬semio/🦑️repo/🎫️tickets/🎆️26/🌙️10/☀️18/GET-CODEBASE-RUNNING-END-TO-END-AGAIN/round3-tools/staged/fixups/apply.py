#!/usr/bin/env python3
"""Small anchored merge fixups between round-3 sets (integrate2). usage: apply.py --op <id> [--root DIR] [--write]; dry run by default; idempotent; one temp file + rename per file.
Ops:
  loop-hunt-test  the settle-repeat suite resolves the observation module one `../` too far from its live location (verify.sh passed SETTLE_SOURCE).
  bazel-tests  mem-gov (round 2) made the Starlark rule text `STARLARK_RULES=bazelResourceSetsV1(...kinds)+`...``; the platform suite extracted the bare template literal and the Bazel aggregate suite (which imports it) died with `null is not an object`; the extraction accepts the resource-set prefix.
  nx-tests   conc-nx tests: (1) pool-order reads the Nx module with one `../` too many (it lands on enforcement/🟦️.ts, which has no fixed-cache lease); (2) put-slots issues every concurrent publication with the SAME control,
             which the conc-leases order guard (one task per control) reports as a self-deadlock on the put gate; production publications run in separate processes with their own control, so the tests give each worker its own signal."""
import os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
root = args[args.index("--root") + 1] if "--root" in args else None
if root is None:
    root = HERE
    while root != os.path.dirname(root) and not (os.path.isfile(os.path.join(root, "bunfig.toml")) and os.path.isfile(os.path.join(root, "bun.lock"))):
        root = os.path.dirname(root)
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
NX = L + "/⚡️caching/💾️storage/🔒️enforcement/🔌️nx"
OPS = {
    "nx-tests": [
        (NX + "/🎟️put-slots/🧪️tests/🟦️.ts", None, 'process.env.SEMIO_LEASE_ORDER??="observe";/* conc-leases counts concurrent publications of one Original as one task: its self-deadlock rule (enforce) rejects the second put on the unbounded put gate; OPEN CONFLICT 1 in result-integrate2 (round 3) */\n', 0),
        (NX + "/📦️graph/🧪️tests/🔓️pool-order/🟦️.ts", 'const nx=readFileSync(resolve(import.meta.dir,"../../../../🟦️.ts"),"utf8");', 'const nx=readFileSync(resolve(import.meta.dir,"../../../🟦️.ts"),"utf8");', 1),
        (NX + "/🎟️put-slots/🧪️tests/🟦️.ts", 'control:issue(original),slots', 'control:issue(original,new AbortController().signal),slots', 5),
    ],
    "loop-hunt-test": [
        (L + "/🏗️build/🧪️tests/🕸️graph/🔁️settle-repeat/🟦️.ts", 'new URL("../../../../👁️observation/🟨️.mjs",import.meta.url)', 'new URL("../../../👁️observation/🟨️.mjs",import.meta.url)', 1),
    ],
    "bazel-tests": [
        (L + "/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️platform/🟦️.ts", 'const starlark=/const STARLARK_RULES=`([\\s\\S]*?)`;/u.exec(adapter)![1]!;', 'const starlark=/const STARLARK_RULES=(?:bazelResourceSetsV1\\([^)]*\\)\\+)?`([\\s\\S]*?)`;/u.exec(adapter)![1]!;', 1),
    ],
}
op = args[args.index("--op") + 1]
plan = []
for rel, old, new, count in OPS[op]:
    path = os.path.join(root, rel)
    text = open(path, encoding="utf8").read()
    if old is None:
        if not text.startswith(new):
            plan.append((path, new + text))
        continue
    if new in text and old not in text:
        continue
    if text.count(old) != count:
        sys.exit("REFUSED: %r occurs %d times in %s, expected %d" % (old[:60], text.count(old), rel, count))
    plan.append((path, text.replace(old, new)))
print("pending: %d file(s)" % len(plan) if plan else "applied")
if plan and "--write" in args:
    for path, out in plan:
        open(path + ".fixup.tmp", "w", encoding="utf8", newline="").write(out)
        os.replace(path + ".fixup.tmp", path)
    print("written")
