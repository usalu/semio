# F99: red library test files

## Green now (`repo test-file`)
- `build/🗂️suites/🧪️tests` 8/4 -> **12/0**; `caching/🗂️suites/🧪️tests` 4/1 -> **5/0** (bundle test spawns since 13:43, budget 15 s).
  Causes: three tests (speed, counter-ledger, typecheck) in no suite; `generate-class` spawns python in the 15 s suite; `writer-control` and `bundle` test not routed to the 300 s process suites; native-dependencies and three build-inference births named a suite that does not route the file (script-inputs/project-nodes were split off build-inference); the graph test support helper imported `test` by name so it was discovered as a test file and two tests side-effect-imported it; the storage aggregate side-effect-imported two test files. No new suite and no project.json edit were needed (trees/files added to rows that already have a target).
- custody `🧾️custody/🧪️tests` 2/2 -> **3/0**: schema `allOf/then` and `else.not` required `suffix` without declaring it; both now carry `properties.suffix`. Ajv stays strict.
- `🪶️lean/🧪️tests` 4/2 -> **6/0**: not a host-memory dependency and not a regression. Today's farm-width policy (child 900 MB typical, parent footprint 1300, independent of the parent's resident size) replaced the 1700 MB lean grant; test 2 failed only because test 1 leaked its lease. Expectations rewritten to the new law (8 GB: 1 child at 900 MB; 16 GB: 4 children; 4 GB: none; lean off: 1). All hosts are passed explicitly.
- offload `📜️action/🧪️tests` 2/1 -> **3/0**: the test built a >4 GiB blob the store "already holds", but the store takes at most 1.25 GiB per entry, so production correctly declines it first. Test now uses several sparse files below the entry limit whose sum exceeds the upload bound, plus an assertion that one oversized file declines at once.

## Still red
- `🦀️cargo/🛠️preparation/🧪️tests/📥️original-control` 0/13. Not the custody schema (F95 conflated them). The test hand-builds `ScriptInvocation` objects; the runtime now requires issued Originals ("Live Original process issuance required"), `codeRoot` in the native owner grant, and a private repository workspace. Needs a rewrite on `privateRepositoryWorkspaceV1().invoke`; not done.
- `⚡️caching/🧪️tests/📦️native-dependencies/🧪️tests`: "Owned data capture output limit exceeded" (child bun stderr 4 MiB). Not verified whether it predates the birth rename (no live A/B allowed).
- `🏗️build/📥️inputs/💾️storage/🧪️tests` 21/1: "actual shared graph writer completes inside its retained graph artifact reader" times out at 20 s under load; the edit there only removed two imports (both removed files pass alone: torn-closure 1/0; refusal 1/0 on rerun, 2 fails on a first run under load).

## Checks
typecheck: all changed TS files, 0 errors (action test fixed after a ReadonlyMap error); dangling-imports exit 0 on every touched folder. Locks released at the coordinator's request.
