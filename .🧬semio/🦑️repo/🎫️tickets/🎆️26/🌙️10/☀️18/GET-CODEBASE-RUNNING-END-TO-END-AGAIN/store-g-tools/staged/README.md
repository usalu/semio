Staged by store-g, apply ONLY when result-repair-g.md line 1 says PRODUCTS GREEN (freeze 11:28):
1. python3 apply-managed-parent-guard.py --apply ; copy managed/owners-managed-test.ts -> 💾️storage/🍎️image/👤️owners/🧪️tests/🚧️managed/🟦️.ts (fix the import depth if needed), managed/grace-test.ts -> 💾️storage/🧹️cleanup/🎛️command/🧪️tests/⏳️grace/🟦️.ts, managed/imports-law-test.ts -> 💾️storage/🧪️tests/🧭️imports/🟦️.ts
2. python3 apply-foldsafe-preflight.py --apply ; then add "caseAccess" to protocol.foldSafeRefusals (directory schema) and to the fold test list (non-elevated users get access denied; Microsoft documents elevation for the flag).
3. run: owners tests, cleanup command tests, directory tests, preflight matrix, bazel readiness tests, then ft.sh suite native-storage.
