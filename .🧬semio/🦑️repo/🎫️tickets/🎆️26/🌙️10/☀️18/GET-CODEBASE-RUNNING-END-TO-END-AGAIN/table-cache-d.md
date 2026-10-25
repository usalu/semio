# table-cache-d: caching test files -> suite -> verdict -> cause

Totals: rc 0: 208, rc 1: 85, rc WALL: 4
Verdict = latest census row (`k*` = authored-envelope census, `official` = official route). `rc 0` green.

| file | suite | verdict | counts | cause / fix |
|---|---|---|---|---|
| `🌐️vite/🏗️compiler/💧️drain/🧪️tests/🟦️.ts` | vite | rc 0 (k4) | pass=3 fail=0 |  |
| `🌐️vite/💾️storage/🧪️tests/🟦️.ts` | vite-processes | rc 0 (overlay) | pass=8 fail=0 | FIXED stale: one Original = one build mode (other mode refused, asserted); listener stop = AbortError rejection => isolated declared-rejection test; reader publish as sibling task (descendant of reader lease task = self-deadlock by law); tamper law = AggregateError, isolated |
| `🌐️vite/📥️environment/🧪️tests/🟦️.ts` | vite-processes | rc 0 (k4) | pass=5 fail=0 |  |
| `🌐️vite/📥️inputs/🧪️tests/🟦️.ts` | vite | rc 0 (k4) | pass=4 fail=0 |  |
| `💾️storage/🔒️enforcement/🏗️compiler/⏳️phase/🧪️tests/🟦️.ts` | storage-compiler | rc 0 (k4) | pass=6 fail=0 |  |
| `💾️storage/🔒️enforcement/🏗️compiler/📍️layout/🧪️tests/🟦️.ts` | storage-compiler-processes | rc 0 (k4) | pass=2 fail=0 |  |
| `💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/⏳️declared-wait/🟦️.ts` | storage-compiler | rc 0 (k4) | pass=3 fail=0 |  |
| `💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/♻️pressure/🟦️.ts` | storage-compiler-processes | rc WALL (k7) | pass=0 fail=1 | OPEN: ENOENT .../i/m/t/p witnessed directory missing in fresh fixture + 180 s hang (descriptor witness of cargo cache maintenance) |
| `💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/⛓️cpu-last/🟦️.ts` | storage-compiler-processes | rc 0 (overlay) | pass=5 fail=0 | FIXED: law scanned git ls-files incl. files moved in the worktree; now tracked+untracked and existing |
| `💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/🔒️exclusive-writer/🟦️.ts` | storage-compiler-processes | rc 0 (overlay) | pass=1 fail=0 | FIXED: same as cpu-last |
| `💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/🟦️.ts` | storage-compiler-processes | rc 1 (k7) | pass=4 fail=2 | OPEN: nested image fixed (scratch); 'Native persistent receiver operation is not ready' in physicalCapacityLaw + 120 s capacity wait |
| `💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/🤝️shared/🟦️.ts` | compiler-headroom | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/🪢️borrowing/🟦️.ts` | storage-compiler | rc 0 (k4) | pass=6 fail=0 |  |
| `💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/🌍️environment/🧪️tests/🟦️.ts` | storage-compiler | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/📥️arguments/📤️forwarding/🧪️tests/🟦️.ts` | storage-compiler | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/📥️arguments/🧪️tests/🟦️.ts` | storage-compiler | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/🧪️tests/🟦️.ts` | cargo-ingress | rc 0 (k4) | pass=2 fail=0 |  |
| `💾️storage/🔒️enforcement/🏗️compiler/🧪️tests/🕸️units/🟦️.ts` | storage-compiler-processes | rc 0 (k7) | pass=11 fail=0 | FIXED stale: Source gate wording; source-growth refusal isolated with declared rejection |
| `💾️storage/🔒️enforcement/🏗️compiler/🧪️tests/🟦️.ts` | storage-compiler-processes | rc 0 (k4) | pass=2 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/⚡️recovery/🧪️tests/🟦️.ts` | storage-opening | rc 0 (overlay) | pass=5 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/📋️profiles/🟦️.ts` | storage-opening | rc 0 (k7) | pass=18 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/🟦️.ts` | cache-reset | rc 0 (k7) | pass=12 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/📇️format/🧪️tests/🟦️.ts` | storage-opening | rc 0 (k4) | pass=5 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/🆔️identity/🟦️.ts` | storage-opening-processes | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️phase/🟦️.ts` | storage-opening-processes | rc 1 (k7) | pass=4 fail=3 | first error: expect(received).toContain(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️queued/🟦️.ts` | storage-opening-processes | rc 1 (k7) | pass=1 fail=4 | OPEN: roster law ahead of production (dependencies-install must use withRepositoryCacheStoreQueuedMaintenanceV1) |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️required/🟦️.ts` | storage-opening-processes | rc 1 (k7) | pass=4 fail=5 | first error: <anonymous_script>:3 ;  |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📦️child/🟦️.ts` | storage-opening | rc 1 (k7) | pass=6 fail=4 | first error: expect(received).toBe(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/🟦️.ts` | storage-opening-processes | rc 1 (k7) | pass=3 fail=6 | first error: Exact original parent child allocation required ;  |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/📂️parents/🧪️tests/🟦️.ts` | storage-opening-processes | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🏗️native-build/📦️owner/🟦️.ts` | storage-opening-processes | rc 1 (k7) | pass=2 fail=10 | first error: expect(received).toBe(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🏗️native-build/🔎️receipt-conflict/🟦️.ts` | storage-opening | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🏗️native-build/🟦️.ts` | storage-opening-processes | rc 1 (k7) | pass=3 fail=4 | first error: Explicit original setup-all asset purpose required ; CATEGORY original-authority gate: needs an Original born as `tools setup all` or an issued purpose; no test route supplies it |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/🟦️.ts` | storage-opening-processes | rc 0 (k4) | pass=2 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/🏪️scope/🟦️.ts` | cache-scope | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/📐️layout/🍎️physical/🟦️.ts` | storage-opening-processes | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/📐️layout/🟦️.ts` | storage-opening | rc 1 (k7) | pass=2 fail=1 | first error: expect(received).toContain(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/🧹️stamp/🟦️.ts` | storage-opening-processes | rc 0 (k4) | pass=8 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📇️records/🧪️tests/🟦️.ts` | storage-graph-processes | rc 0 (k4) | pass=5 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🔀️.ts` | storage-graph-processes | rc 0 (overlay) | pass=1 fail=0 | FIXED stale: nested image under diagnostics image -> scratch; extracted-function ports ambientLeases/readBuildGraphHeadroomRequestV1/runtimeExecutableForV1 |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🔁️revalidation/🍎️.ts` | storage-graph-processes | rc 0 (overlay) | pass=2 fail=0 | FIXED stale: nested image under diagnostics image -> scratch |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🔁️revalidation/🟦️.ts` | storage-graph | rc 0 (overlay) | pass=1 fail=0 | FIXED stale: maintenance request now carries yielding:true when waiting |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🟦️.ts` | storage-graph-processes | rc WALL (k7) |  | FIXED stale: scenarios split per process, expected refusals isolated with declared rejection, optional-queued uses local retire (production refuses non-canonical images by law), Nx pressure test follows production flow; REAL DEFECT fixed in 🔌️nx/📦️graph/🟦️.ts prepareNxGraphCacheStorageV1 (coarse queued recovery ran although the scoped optional recovery funded the floor) |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/👷️child/🟦️.ts` | storage-graph-processes | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/🔎️scan/🟦️.ts` | storage-graph | rc 0 (k4) | pass=11 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/🟦️.ts` | storage-graph-processes | rc 0 (overlay) | pass=10 fail=0 | FIXED stale: publication loops over paths |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/🧱️resolve/🟦️.ts` | storage-graph-processes | rc 0 (k4) | pass=9 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/🧹️supersede/🟦️.ts` | storage-graph-processes | rc 0 (k4) | pass=11 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/♻️groups.ts` | storage-graph-processes | rc 1 (k7) | pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=2 fail=8 | OPEN: needs admitted program (import.meta.semioCaptured* maps exist only inside a compiled authorizer bundle): run through 🔏️admission harness |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📁️census/🟦️.ts` | storage-graph-processes | rc 0 (k4) | pass=2 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📇️formats/🟦️.ts` | storage-graph | rc 0 (overlay) | pass=4 fail=0 | FIXED stale: Ajv allowUnionTypes; non-optional role declines (false, no effect) |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📏️cardinality/🟦️.ts` | storage-graph | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📡️session/💳️.ts` | storage-graph | rc 0 (k4) | pass=2 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📡️session/📣️failure/🟦️.ts` | storage-graph | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📡️session/🔄️.ts` | storage-graph | rc 0 (overlay) | pass=1 fail=0 | FIXED stale: extracted-function free names runtimeCommandNameV1, retainClosureReadFailureV1 |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📡️session/🚦️startup/🟦️.ts` | storage-graph | rc 0 (k4) | pass=2 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📡️session/🟦️.ts` | checkout-scratch-processes | rc 1 (k7) | pass=3 fail=1 | first error: Closed authorizer relative source path required ; CATEGORY checkout-resident scratch: the test needs sources/fixtures below the workspace; route `scratch: checkout` |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📤️packet/🟦️.ts` | storage-graph-processes | rc 0 (overlay) | pass=1 fail=0 | FIXED stale: worker returns packet after readDebit |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📥️inference/🟦️.ts` | storage-graph | rc 0 (k4) | pass=2 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔏️admission/🧪️.ts` | checkout-scratch-processes | rc 1 (k7) | pass=1 fail=2 | first error: compileNxAuthorizerSnapshotV1 ;  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔢️batch/🟦️.ts` | storage-graph-processes | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🕰️generation/🟦️.ts` | storage-graph | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🟦️.ts` | storage-graph-processes | rc 1 (k7) | pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=11 fail=15 | OPEN: same admitted-program cause; aggregator imports removed |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🪟️window/🟦️.ts` | storage-graph | rc 0 (k4) | pass=4 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🕸️graph/🧪️tests/🟦️.ts` | graph-bounded | rc 0 (k4) | pass=3 fail=0 |  |
| `💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent/🟦️.ts` | storage-budgets-processes | rc 1 (k7) | pass=19 fail=29 | first error: absent ;  |
| `💾️storage/🔒️enforcement/📦️budget/🧪️tests/🟦️.ts` | storage-budget | rc 0 (k4) | pass=10 fail=0 |  |
| `💾️storage/🔒️enforcement/🔌️nx/🎟️put-slots/🧪️tests/🟦️.ts` | storage-nx-processes | rc 0 (k4) | pass=13 fail=0 |  |
| `💾️storage/🔒️enforcement/🔌️nx/📦️graph/📥️inputs/📋️data/🧪️tests/🟦️.ts` | storage-nx | rc 1 (k7) | pass=10 fail=1 | first error: Required original cache consumer unavailable ;  |
| `💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🎛️operation/🟦️.ts` | storage-nx-processes | rc 1 (k7) | pass=0 fail=1 pass=2 fail=3 | first error: expect(received).toBe(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🏦️funded/🟦️.ts` | storage-nx-processes | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️purpose/📜️script.ts` | storage-nx-processes | rc 1 (k7) | pass=0 fail=2 | first error: Exact graph purpose context required ; CATEGORY original-authority gate: needs an Original born as `tools setup all` or an issued purpose; no test route supplies it |
| `💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️purpose/🧪️tests/🟦️.ts` | storage-nx | rc 1 (k7) | pass=2 fail=2 | first error: Exact carried graph purpose receipt required ; CATEGORY original-authority gate: needs an Original born as `tools setup all` or an issued purpose; no test route supplies it |
| `💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🔓️pool-order/🟦️.ts` | storage-nx-processes | rc 0 (k4) | pass=7 fail=0 |  |
| `💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts` | storage-nx-processes | rc 1 (k7) | pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=25 fail=22 | PARTIAL: reentry/forged/expired-narrower split + isolated; 3 pool tests stay red (private fixture workspace inside products image violates nested-image + Original-workspace laws; FOR nx/exact owner) |
| `💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🤝️yield/🟦️.ts` | storage-nx-processes | rc 0 (k4) | pass=11 fail=0 |  |
| `💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🧭️ambient-scope/🟦️.ts` | storage-nx | rc 0 (k4) | pass=2 fail=0 |  |
| `💾️storage/🔒️enforcement/🔌️nx/📦️outputs/📍️paths/🧪️tests/🟦️.ts` | storage-nx | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/🔌️nx/📦️outputs/📚️cohorts/🧪️tests/🟦️.ts` | storage-nx | rc 0 (k4) | pass=3 fail=0 |  |
| `💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/📋️registration/🟦️.ts` | storage-nx | rc 1 (k7) | pass=1 fail=1 | first error: expect(received).toContain(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/📥️purpose/🧪️tests/🟦️.ts` | storage-nx | rc 1 (k7) | pass=1 fail=2 | first error: Expected values to be strictly deep-equal: ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🗄️persistent-state/🟦️.ts` | storage-nx | rc 0 (k4) | pass=1 fail=0 |  |
| `💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🟦️.ts` | toolkit-outputs | rc 1 (k7) | pass=6 fail=7 | first error: Original Nx SDK identity required ; CATEGORY env-official: needs SEMIO_NX_TOOLING_IDENTITY / Nx SDK capabilities of the official target environment (a probe with the installed identity turned `📦️artifacts/🔒️bounded` 7/7 green); verify through the official route |
| `💾️storage/🔒️enforcement/🧵️session/🏊️pool/🧪️tests/🟦️.ts` | storage-session-processes | rc 0 (k4) | pass=7 fail=0 |  |
| `💾️storage/🔒️enforcement/🧵️session/🏊️pool/🧪️tests/🧮️lifecycle/🟦️.ts` | law-coverage | rc 0 (k4) | pass=4 fail=0 |  |
| `💾️storage/🧾️ledger/🧪️tests/⚖️law/🟦️.ts` | storage-session-processes | rc 0 (k4) | pass=7 fail=0 |  |
| `💾️storage/🧾️ledger/🧪️tests/📏️needs/🟦️.ts` | storage-session | rc 0 (k4) | pass=8 fail=0 |  |
| `💾️storage/🧾️ledger/🧪️tests/🔮️sqlite/🟦️.ts` | storage-session | rc 0 (k4) | pass=3 fail=0 |  |
| `💾️storage/🧾️ledger/🧪️tests/🟦️.ts` | storage-session | rc 0 (k4) | pass=10 fail=0 |  |
| `📖️texts/🧪️tests/🟦️.ts` | texts | rc 0 (k4) | pass=3 fail=0 |  |
| `📥️inference/🧪️tests/🔍️imports/🟦️.ts` | inference | rc 1 (k7) | pass=2 fail=1 | NX-ONLY: needs an Nx-published repository graph and file map |
| `📥️inference/🧪️tests/🟦️.ts` | inference-processes | rc 1 (k7) | pass=1 fail=2 | first error: Exact Original workspace required ;  |
| `📦️artifacts/🏗️native-build/📥️published/🧪️tests/🟦️.ts` | checkout-scratch-processes | rc 1 (k7) | pass=0 fail=2 | first error: Published Cargo manifest escapes workspace ; CATEGORY checkout-resident scratch: the test needs sources/fixtures below the workspace; route `scratch: checkout` |
| `📦️artifacts/🐳️containers/🧪️tests/🔁️lifecycle/🚪️gate/🧪️tests/🟦️.ts` | artifacts | rc 0 (k7) | pass=1 fail=0 | FIXED stale: target now carries cache metadata (schema const updated) |
| `📦️artifacts/🐳️containers/🧪️tests/🔁️lifecycle/🪝️automatic-hooks/🟦️.ts` | artifacts | rc 0 (k4) | pass=1 fail=0 |  |
| `📦️artifacts/👓️reading/📚️group/🧪️tests/🟦️.ts` | artifacts | rc 0 (k4) | pass=5 fail=0 |  |
| `📦️artifacts/👓️reading/📥️inspection/🧪️tests/🟦️.ts` | artifacts | rc 0 (k4) | pass=3 fail=0 |  |
| `📦️artifacts/👓️reading/🧪️tests/🔒️scope/🟦️.ts` | artifacts | rc 0 (k4) | pass=1 fail=0 |  |
| `📦️artifacts/👓️reading/🧪️tests/🟦️.ts` | artifacts | rc 0 (k4) | pass=1 fail=0 |  |
| `📦️artifacts/💾️storage/🧪️tests/📏️extent/🟦️.ts` | artifacts | rc 0 (k4) | pass=2 fail=0 |  |
| `📦️artifacts/💾️storage/🧪️tests/📥️physical/🟦️.ts` | artifacts-processes | rc 1 (k7) | pass=0 fail=16 | first error: Nested persistent image requires actual parent scope ; CATEGORY nested-image law: the test opens a native image below an open image (diagnostics/products); needs scratch outside images or the parent child-API |
| `📦️artifacts/💾️storage/🧪️tests/🟦️.ts` | artifacts-processes | rc 1 (k7) | pass=13 fail=2 | first error: expect(received).toContain(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `📦️artifacts/📋️native-orchestration/🌱️environment/🧪️tests/🟦️.ts` | artifacts | rc 0 (k4) | pass=1 fail=0 |  |
| `📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🧪️tests/🟦️.ts` | artifacts | rc 0 (k4) | pass=1 fail=0 |  |
| `📦️artifacts/📋️native-orchestration/📥️invocation/🧪️tests/🏠️caller-root/🟦️.ts` | artifacts | rc 0 (k4) | pass=1 fail=0 |  |
| `📦️artifacts/📋️native-orchestration/📥️invocation/🧪️tests/🟦️.ts` | artifacts | rc 1 (k7) | pass=2 fail=2 | first error: declaredCapabilitiesV1 is not defined ; CATEGORY extracted-function port drift: production function gained free names; test ports/extraction list stale |
| `📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts` | artifacts-processes | rc 0 (k4) | pass=11 fail=0 |  |
| `📦️artifacts/📋️native-orchestration/🧪️tests/📐️grant/🟦️.ts` | artifacts | rc 0 (k4) | pass=1 fail=0 |  |
| `📦️artifacts/📋️native-orchestration/🧪️tests/📬️test-body/🟦️.ts` | artifacts-processes | rc 1 (k7) | pass=0 fail=9 | first error: createRenewableIssuedResourceControlV1 is not defined ; CATEGORY extracted-function port drift: production function gained free names; test ports/extraction list stale |
| `📦️artifacts/📋️native-orchestration/🧪️tests/🗺️owner-test-manifests/🟦️.ts` | artifacts-processes | rc 0 (k4) | pass=5 fail=0 |  |
| `📦️artifacts/📝️derived-config/🧪️tests/📝️derived-config/🟦️.ts` | artifacts | rc 0 (k4) | pass=1 fail=0 |  |
| `📦️artifacts/🔒️bounded/🧪️tests/🍎️image/🟦️.ts` | nx-bounded-image | rc 1 (k7) | pass=0 fail=1 | first error: Original immutable SDK required ; CATEGORY env-official: needs SEMIO_NX_TOOLING_IDENTITY / Nx SDK capabilities of the official target environment (a probe with the installed identity turned `📦️artifacts/🔒️bounded` 7/7 green); verify through the official route |
| `📦️artifacts/🔒️bounded/🧪️tests/🟦️.ts` | nx-bounded | rc 1 (k7) | pass=6 fail=1 | first error: Original immutable SDK required ; CATEGORY env-official: needs SEMIO_NX_TOOLING_IDENTITY / Nx SDK capabilities of the official target environment (a probe with the installed identity turned `📦️artifacts/🔒️bounded` 7/7 green); verify through the official route |
| `📦️artifacts/🟦️typescript/🔗️binding/🧪️tests/🟦️.ts` | law-coverage | rc 0 (k4) | pass=4 fail=0 |  |
| `📦️artifacts/🟦️typescript/🧪️tests/🔒️held-capacity/🟦️.ts` | artifacts | rc 0 (k4) | pass=1 fail=0 |  |
| `📦️artifacts/🟦️typescript/🧪️tests/🧭️bundle-runtime/🟦️.ts` | artifacts-processes | rc 0 (k4) | pass=1 fail=0 |  |
| `📦️artifacts/🟦️typescript/🧪️tests/🧭️commands/🟦️.ts` | artifacts | rc 0 (k4) | pass=6 fail=0 |  |
| `📦️artifacts/🧪️tests/🟦️.ts` | artifacts-processes | rc 0 (k4) | pass=5 fail=0 |  |
| `📦️bundle/🟦️bun/📤️stream/🧪️tests/🟦️.ts` | law-coverage | rc 0 (k4) | pass=5 fail=0 |  |
| `📦️bundle/🟦️bun/🧪️tests/📝️text-loader/🟦️.ts` | bundle | rc 0 (k4) | pass=2 fail=0 |  |
| `📦️bundle/🧪️tests/🔬️concurrency/🟦️.ts` | bundle | rc 0 (k4) | pass=1 fail=0 |  |
| `📦️bundle/🧪️tests/🟦️.ts` | bundle | rc 0 (k4) | pass=4 fail=0 |  |
| `🕸️graph/✅️verification/🧪️tests/🟦️.ts` | graph | rc 0 (overlay) | pass=2 fail=0 | FIXED: REAL typing defect (internals declaration lacked resourceEntries) + withWasmTooling depends on workspace:deps-wasm |
| `🕸️graph/🏁️startup/📥️phase/🧪️tests/🟦️.ts` | graph | rc 0 (k4) | pass=1 fail=0 |  |
| `🕸️graph/🏁️startup/🚏️admission/🧪️tests/🟦️.ts` | graph-processes | rc 0 (k4) | pass=34 fail=0 |  |
| `🕸️graph/🏁️startup/🚦️service/🧪️tests/🟦️.ts` | graph-processes | rc 1 (k7) | pass=0 fail=2 | first error: Original loaded SDK root required ; CATEGORY env-official: needs SEMIO_NX_TOOLING_IDENTITY / Nx SDK capabilities of the official target environment (a probe with the installed identity turned `📦️artifacts/🔒️bounded` 7/7 green); verify through the official route |
| `🕸️graph/🏁️startup/🚦️service/🧪️tests/🧮️handoff/🟦️.ts` | law-coverage | rc 0 (k4) | pass=3 fail=0 |  |
| `🕸️graph/🏁️startup/🧬️lifetime/🧪️tests/🟦️.ts` | graph | rc 0 (k4) | pass=8 fail=0 |  |
| `🕸️graph/🏇️waiter/🧪️tests/🟦️.ts` | graph-cache-waiter | rc 1 (k7) | pass=0 fail=1 | first error: Original installed tooling identity and ticket artifact directory required ; CATEGORY env-official: needs SEMIO_NX_TOOLING_IDENTITY / Nx SDK capabilities of the official target environment (a probe with the installed identity turned `📦️artifacts/🔒️bounded` 7/7 green); verify through the official route |
| `🕸️graph/🏗️scheduling/🧪️tests/🟦️.ts` | graph-processes | rc 0 (k4) | pass=3 fail=0 |  |
| `🕸️graph/📤️publication/🧪️tests/📦️transaction/🟦️.ts` | graph-processes | rc 1 (k7) | pass=0 fail=3 | first error: node:path:1257 ;  |
| `🕸️graph/📤️publication/🧪️tests/🟦️.ts` | graph-publication | rc 1 (k7) | pass=0 fail=6 | first error: node:path:1257 ;  |
| `🕸️graph/📤️publication/🧪️tests/🧭️workspace-root/🟦️.ts` | graph | rc 0 (k4) | pass=1 fail=0 |  |
| `🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests/🟦️.ts` | graph-processes | rc 1 (k7) | pass=6 fail=12 | first error: Required original cache consumer unavailable ;  |
| `🕸️graph/🔏️seal/🎯️selection/🧪️tests/🎛️phase/🟦️.ts` | graph | rc 0 (k4) | pass=1 fail=0 |  |
| `🕸️graph/🔏️seal/🎯️selection/🧪️tests/🟦️.ts` | selected-graph | rc 1 (k7) | pass=0 fail=1 | first error: Actual incoming immutable Nx tooling identity required ; CATEGORY env-official: needs SEMIO_NX_TOOLING_IDENTITY / Nx SDK capabilities of the official target environment (a probe with the installed identity turned `📦️artifacts/🔒️bounded` 7/7 green); verify through the official route |
| `🕸️graph/🔏️seal/📌️generation/🧪️tests/🟦️.ts` | graph-processes | rc 0 (k4) | pass=19 fail=0 |  |
| `🕸️graph/🔏️seal/📥️membership/📥️journal/🧪️tests/🟦️.ts` | journal-source | rc 0 (k4) | pass=4 fail=0 |  |
| `🕸️graph/🔏️seal/📦️facts/🧪️tests/🟦️.ts` | graph-processes | rc 0 (k4) | pass=5 fail=0 |  |
| `🕸️graph/🔏️seal/🧪️tests/🟦️.ts` | graph-seal | rc 1 (k7) | pass=0 fail=4 | first error: Nested persistent image requires actual parent scope ; CATEGORY nested-image law: the test opens a native image below an open image (diagnostics/products); needs scratch outside images or the parent child-API |
| `🗂️suites/🧪️tests/🟦️.ts` | route-table-processes | rc 0 (k4) | pass=5 fail=0 |  |
| `🗃️records/🧪️tests/🔌️descriptor-limit/🟦️.ts` | records-processes | rc 0 (k4) | pass=3 fail=0 |  |
| `🗃️records/🧪️tests/🟦️.ts` | records | rc 0 (k4) | pass=4 fail=0 |  |
| `🚀️bootstrap/📖️texts/🧪️tests/🟦️.ts` | bootstrap | rc 0 (k4) | pass=3 fail=0 |  |
| `🚀️bootstrap/📥️admission/💾️storage/🧪️tests/🟦️.ts` | bootstrap | rc 0 (k4) | pass=2 fail=0 |  |
| `🚀️bootstrap/📥️invocation/🧪️tests/🟦️.ts` | bootstrap-original-invocation | rc 0 (overlay) | pass=4 fail=0 | FIXED stale: runner moved to process 🧪️testing/🗂️suites/🏃️run; law reads runner + route table |
| `🚀️bootstrap/📦️dependencies/⌨️commands/🧪️tests/🟦️.ts` | bootstrap-dependencies | rc 0 (k4) | pass=1 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🌐️platform/🟦️.ts` | bootstrap-dependencies | rc 0 (k4) | pass=1 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/🟦️.ts` | bootstrap-dependencies-processes | rc 1 (k7) | pass=44 fail=36 | OPEN: first test is hollow (loop deleted by an earlier destructive edit: vectors stays 0); later failures cascade |
| `🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📏️paths/🟦️.ts` | javascript-installed-paths | rc 0 (k4) | pass=2 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📣️report/🟦️.ts` | bootstrap-dependencies | rc 0 (k4) | pass=3 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📤️diagnostics/🟦️.ts` | bootstrap-dependencies | rc 0 (k4) | pass=1 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🔭️providers/🟦️.ts` | javascript-provider-walk | rc 0 (k4) | pass=1 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🚧️fence/🟦️.ts` | javascript-endpoint-fence | rc 0 (k4) | pass=5 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts` | javascript-storage | rc 1 (k7) | pass=28 fail=19 | first error: undefined is not an object (evaluating 'file.statements.find((node) => ts.isFunctionDeclaration(node) && node. ;  |
| `🚀️bootstrap/📦️dependencies/🔌️managers/🍞️bun/🧪️tests/🟦️.ts` | bootstrap-dependencies-processes | rc 0 (k4) | pass=102 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/🔌️managers/🧪️tests/🟦️.ts` | package-manager-seam | rc 0 (k4) | pass=88 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/🔒️lock-sync/🧪️tests/🟦️.ts` | bootstrap-dependencies-processes | rc 0 (k4) | pass=23 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/🚦️concurrent/🧪️tests/🟦️.ts` | javascript-concurrent-generations | rc 0 (k4) | pass=16 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/🚦️concurrent/🧪️tests/🪟️free-space/🟦️.ts` | bootstrap-dependencies | rc 0 (k4) | pass=2 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/🧪️tests/🟦️.ts` | bootstrap-dependencies-processes | rc 0 (k4) | pass=15 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🌡️heap-environment/🟦️.ts` | bootstrap-dependencies | rc 0 (k4) | pass=2 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🚪️ingress/🟦️.ts` | javascript-generation-ingress | rc 0 (k4) | pass=10 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🛡️followup/🟦️.ts` | javascript-generation-followup | rc 0 (k4) | pass=7 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🟦️.ts` | javascript-generation | rc 0 (k4) | pass=12 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🟦️.ts` | javascript-incremental-sync | rc 0 (k4) | pass=15 fail=0 |  |
| `🚀️bootstrap/📦️dependencies/🪞️adopt/🧪️tests/🟦️.ts` | javascript-adopt | rc 0 (k4) | pass=8 fail=0 |  |
| `🚀️bootstrap/🔌️nx-plugin/📌️graph-pin/🧪️tests/🟦️.ts` | bootstrap | rc 0 (k4) | pass=2 fail=0 |  |
| `🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧪️tests/📥️native-caller/🟦️.ts` | bootstrap-processes | rc 1 (k7) | pass=0 fail=1 | first error: Original bounded Nx namespace required ; CATEGORY env-official: needs SEMIO_NX_TOOLING_IDENTITY / Nx SDK capabilities of the official target environment (a probe with the installed identity turned `📦️artifacts/🔒️bounded` 7/7 green); verify through the official route |
| `🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧪️tests/🟦️.ts` | bootstrap-processes | rc WALL (k7) |  | first error: Original bounded Nx namespace required ; CATEGORY env-official: needs SEMIO_NX_TOOLING_IDENTITY / Nx SDK capabilities of the official target environment (a probe with the installed identity turned `📦️artifacts/🔒️bounded` 7/7 green); verify through the official route |
| `🚀️bootstrap/🔌️nx-plugin/🧪️tests/🦀️native-dependency-closures/🟦️.ts` | bootstrap-processes | rc 0 (k4) | pass=1 fail=0 |  |
| `🚀️bootstrap/🛠️tools/⚙️operations/🧪️tests/🟦️.ts` | tools-processes | rc 0 (k4) | pass=8 fail=0 |  |
| `🚀️bootstrap/🛠️tools/⚙️operations/🧪️tests/🧮️arguments/🟦️.ts` | law-coverage | rc 0 (k4) | pass=6 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🌡️host-capacity/🧪️tests/🟦️.ts` | tools-processes | rc 0 (k4) | pass=1 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🌡️host-capacity/🧪️tests/🧮️settings/🟦️.ts` | law-coverage | rc 0 (k4) | pass=2 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🏗️bazel/🧪️tests/📥️pins/🟦️.ts` | tools-processes | rc 0 (k7) | pass=4 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🏗️bazel/🧪️tests/🟦️.ts` | tools | rc 0 (k4) | pass=1 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🏗️bazel/🩺️readiness/🧪️tests/🟦️.ts` | tools | rc 0 (k4) | pass=7 fail=0 |  |
| `🚀️bootstrap/🛠️tools/👷️workers/🧪️tests/🟦️.ts` | tools-processes | rc 0 (k4) | pass=3 fail=0 |  |
| `🚀️bootstrap/🛠️tools/👷️workers/🧪️tests/🧮️mapping/🟦️.ts` | law-coverage | rc 0 (k4) | pass=3 fail=0 |  |
| `🚀️bootstrap/🛠️tools/📥️use/🧪️tests/📥️purpose/🧪️tests/🟦️.ts` | tools | rc 1 (k7) | pass=1 fail=3 | first error: Exact same-wire toolkit refusal receipt required ;  |
| `🚀️bootstrap/🛠️tools/📥️use/🧪️tests/📬️task-child/🟦️.ts` | task-child | rc 1 (k7) | pass=0 fail=2 | first error: Original SDK and artifacts required ; CATEGORY env-official: needs SEMIO_NX_TOOLING_IDENTITY / Nx SDK capabilities of the official target environment (a probe with the installed identity turned `📦️artifacts/🔒️bounded` 7/7 green); verify through the official route |
| `🚀️bootstrap/🛠️tools/📥️use/🧪️tests/🟦️.ts` | toolkit-use | rc 1 (k7) | pass=0 fail=1 | first error: Original artifacts and immutable SDK identity required ; CATEGORY env-official: needs SEMIO_NX_TOOLING_IDENTITY / Nx SDK capabilities of the official target environment (a probe with the installed identity turned `📦️artifacts/🔒️bounded` 7/7 green); verify through the official route |
| `🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/📤️derivative/⚠️failure/🟦️.ts` | tools-sdk-processes | rc 0 (k4) | pass=1 fail=0 pass=1 fail=0 pass=2 fail=0 |  |
| `🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/📤️derivative/🟦️.ts` | tools-sdk-processes | rc 1 (k7) | pass=3 fail=5 | first error: expect(received).toThrow(expected) ;  |
| `🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/🟦️.ts` | tools-sdk-processes | rc 1 (k7) | pass=2 fail=7 | first error: expect(received).toBe(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🚀️bootstrap/🛠️tools/📦️sdk/📊️material/🧪️tests/🟦️.ts` | sdk-snapshot | rc 0 (k4) | pass=3 fail=0 |  |
| `🚀️bootstrap/🛠️tools/📦️sdk/📥️snapshot/🧪️tests/🟦️.ts` | sdk-snapshot | rc 1 (k7) | pass=7 fail=3 | first error: undefined == true ;  |
| `🚀️bootstrap/🛠️tools/📦️sdk/📥️source/📐️projection/🧪️tests/🟦️.ts` | law-coverage | rc 0 (k4) | pass=4 fail=0 |  |
| `🚀️bootstrap/🛠️tools/📦️sdk/📥️source/📥️acquisition/🧪️tests/🧮️fetch/🟦️.ts` | law-coverage-fixes | rc 0 (k4) | pass=3 fail=0 |  |
| `🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📚️layers/🧪️tests/🟦️.ts` | tools-sdk | rc 0 (k4) | pass=2 fail=0 |  |
| `🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📤️copy/🧪️tests/🟦️.ts` | tools-sdk-processes | rc 1 (k7) | pass=9 fail=2 | first error: expect(received).toHaveLength(expected) ;  |
| `🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🔏️index/🧪️tests/🟦️.ts` | tools-sdk | rc 0 (k4) | pass=4 fail=0 |  |
| `🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🧪️tests/📥️phase/🟦️.ts` | tools-sdk | rc 0 (k4) | pass=2 fail=0 |  |
| `🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🧪️tests/🛂️sdk-fences/🟦️.ts` | tools-sdk | rc 1 (k7) | pass=3 fail=4 | first error: expect(received).toBe(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🧪️tests/🟦️.ts` | tools-sdk-processes | rc 1 (k7) | pass=0 fail=2 pass=22 fail=1 | first error: Original issued native storage reservation required ;  |
| `🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🗃️archive/🧪️tests/🟦️.ts` | tools-sdk | rc 0 (k4) | pass=6 fail=0 |  |
| `🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🧊️pristine/📋️plan/🧪️tests/🟦️.ts` | law-coverage | rc 0 (k4) | pass=4 fail=0 |  |
| `🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🧊️pristine/🧪️tests/🧮️restore/🟦️.ts` | law-coverage-fixes | rc 0 (k4) | pass=3 fail=0 |  |
| `🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🧪️tests/🟦️.ts` | tools-sdk | rc 0 (k4) | pass=1 fail=0 |  |
| `🚀️bootstrap/🛠️tools/📦️sdk/🧊️binary/🧪️tests/🎟️phase/🟦️.ts` | tools-sdk | rc 0 (k4) | pass=2 fail=0 |  |
| `🚀️bootstrap/🛠️tools/📦️sdk/🧊️binary/🧪️tests/🟦️.ts` | sdk-snapshot | rc 0 (k4) | pass=1 fail=0 |  |
| `🚀️bootstrap/🛠️tools/📦️sdk/🪪️identity/🧪️tests/🧮️keys/🟦️.ts` | law-coverage | rc 0 (k4) | pass=4 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🔁️probe/🧪️tests/🟦️.ts` | tools | rc 0 (k4) | pass=3 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🔌️sdk/🧪️tests/🟦️.ts` | tools | rc 0 (k4) | pass=4 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🔏️authorizers/📥️loading/🧪️tests/🟦️.ts` | toolkit-preflight | rc 0 (k4) | pass=1 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🔏️authorizers/📦️runtime/🧪️tests/🟦️.ts` | tools-authorizers | rc 0 (k4) | pass=1 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🧪️tests/🟦️.ts` | law-coverage | rc 0 (k4) | pass=5 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🔏️authorizers/📦️workspace/🧪️tests/🟦️.ts` | tools-authorizers-processes | rc 0 (k4) | pass=2 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📥️data/🟦️.ts` | tools-authorizers | rc 1 (k7) | pass=1 fail=2 | first error: expect(received).toBe(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📥️scope/🟦️.ts` | tools-authorizers | rc 0 (k4) | pass=3 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📦️compiler-init/🟦️.ts` | tools-authorizers-processes | rc 0 (k4) | pass=2 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts` | authorizers | rc 1 (k7) | pass=4 fail=9 | OPEN: extracted-function ports fixed; capture needs checkout-resident scratch; mojibake heuristic collides with real Ã©.ts; DATA capacity; runtime companion roster |
| `🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧭️closure-law/🟦️.ts` | tools-authorizers-processes | rc 0 (k4) | pass=3 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧪️tests/🟦️.ts` | tools-authorizers-processes | rc 0 (k4) | pass=3 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🔗️activation/🧪️tests/🟦️.ts` | tools | rc 0 (k4) | pass=2 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🟦️bun/🧪️tests/🟦️.ts` | law-coverage | rc 0 (k4) | pass=4 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🦀️rustup/🧪️tests/🟦️.ts` | rustup-protocol | rc 0 (k4) | pass=11 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🧪️tests/📦️install-images/🟦️.ts` | law-coverage-tests | rc 0 (k4) | pass=3 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🧪️tests/🟦️.ts` | tools-processes | rc 1 (k7) | pass=6 fail=2 | first error: expect(received).toThrow(expected) ;  |
| `🚀️bootstrap/🛠️tools/🧪️tests/🧹️cleanup/🟦️.ts` | tools | rc 0 (k4) | pass=1 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🧰️native/🧪️tests/🟦️.ts` | tools-processes | rc 0 (k4) | pass=11 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🧹️retention/📏️headroom/🧪️tests/🟦️.ts` | tools-retention-processes | rc 1 (k7) | pass=1 fail=2 | FIXED stale: scenarios split per process, expected refusals isolated with declared rejection, optional-queued uses local retire (production refuses non-canonical images by law), Nx pressure test follows production flow; REAL DEFECT fixed in 🔌️nx/📦️graph/🟦️.ts prepareNxGraphCacheStorageV1 (coarse queued recovery ran although the scoped optional recovery funded the floor) |
| `🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/📥️negative/🧪️tests/🟦️.ts` | tools-retention | rc 1 (k7) | pass=1 fail=3 | first error: Expected values to be strictly deep-equal: ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/🟦️.ts` | tools-retention-processes | rc WALL (k7) | pass=0 fail=2 pass=1 fail=0 pass=1 fail=0 pass=1 fail=0 pass=1 fail=0 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 pass=0 fail=2 | first error: none ;  |
| `🚀️bootstrap/🛠️tools/🧹️retention/📦️generation/🧪️tests/🟦️.ts` | tools-retention-processes | rc 1 (k7) | pass=1 fail=2 | first error: Nested persistent image requires actual parent scope ; CATEGORY nested-image law: the test opens a native image below an open image (diagnostics/products); needs scratch outside images or the parent child-API |
| `🚀️bootstrap/🛠️tools/🧹️retention/🧪️tests/🟦️.ts` | toolkit-retention | rc 0 (k4) | pass=1 fail=0 |  |
| `🚀️bootstrap/🛠️tools/🩹️patch/🧪️tests/🟦️.ts` | law-coverage | rc 0 (k4) | pass=7 fail=0 |  |
| `🚦️ci/🧭️baseline/🏃️resolve/🧪️tests/🧮️matrix/🟦️.ts` | law-coverage | rc 0 (k4) | pass=4 fail=0 |  |
| `🦀️cargo/📦️units/🏗️compiler/💾️state/🧪️tests/🎟️funding/🟦️.ts` | cargo-units-processes | rc 0 (k4) | pass=13 fail=0 |  |
| `🦀️cargo/📦️units/🏗️compiler/💾️state/🧪️tests/🟦️.ts` | cargo-units-processes | rc 1 (k7) | pass=22 fail=1 | first error: expect(received).toBe(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🦀️cargo/📦️units/🏗️compiler/📍️workspace/🧪️tests/🟦️.ts` | law-coverage | rc 0 (k4) | pass=3 fail=0 |  |
| `🦀️cargo/📦️units/🏗️compiler/🔏️code/🧪️tests/📖️reader/🟦️.ts` | cargo-units-processes | rc 1 (k7) | pass=1 fail=2 | first error: expect(received).toBeLessThanOrEqual(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🦀️cargo/📦️units/🏗️compiler/🦀️rustc/🎟️capacity/🧪️tests/🟦️.ts` | law-coverage-tests | rc 0 (k4) | pass=5 fail=0 |  |
| `🦀️cargo/📦️units/🏗️compiler/🦀️rustc/📚️dependencies/🧪️tests/🟦️.ts` | cargo-units-processes | rc 0 (k4) | pass=5 fail=0 |  |
| `🦀️cargo/📦️units/🏗️compiler/🦀️rustc/🧪️tests/🟦️.ts` | cargo-units | rc 0 (k4) | pass=2 fail=0 |  |
| `🦀️cargo/📦️units/🏗️compiler/🦀️rustc/🧾️invocation/🧪️tests/🟦️.ts` | cargo-units-processes | rc 0 (k4) | pass=13 fail=0 |  |
| `🦀️cargo/📦️units/📋️plan/🔏️identity/🧪️tests/🟦️.ts` | cargo-units | rc 0 (k4) | pass=3 fail=0 |  |
| `🦀️cargo/📦️units/📋️plan/🧪️tests/🔗️binding/🟦️.ts` | cargo-units-processes | rc 0 (k4) | pass=7 fail=0 |  |
| `🦀️cargo/📦️units/📋️plan/🧪️tests/🟦️.ts` | cargo-units-processes | rc 0 (k4) | pass=22 fail=0 |  |
| `🦀️cargo/📦️units/📤️receipt/🧪️tests/🟦️.ts` | cargo-units-processes | rc 0 (k4) | pass=15 fail=0 |  |
| `🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/📦️batch/🟦️.ts` | cargo-units-processes | rc 1 (k7) | pass=4 fail=3 | first error: expect(received).toEqual(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/🟦️.ts` | cargo-units-processes | rc 1 (k7) | pass=5 fail=13 | first error: {"target":"source","stage":"entry","accepted":false}: ;  |
| `🦀️cargo/📦️units/📥️descriptor/🎯️selection/🧪️tests/🧫️fixture/🧪️tests/🟦️.ts` | cargo-units-processes | rc 0 (k4) | pass=3 fail=0 |  |
| `🦀️cargo/📦️units/📥️descriptor/💾️cache/📂️files/🗄️catalog/🧪️tests/🟦️.ts` | cargo-units-processes | rc 0 (k4) | pass=8 fail=0 |  |
| `🦀️cargo/📦️units/📥️descriptor/💾️cache/📤️publication/🧪️tests/🟦️.ts` | cargo-units-processes | rc 1 (k7) | pass=4 fail=3 | first error: expect(received).toEqual(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🦀️cargo/📦️units/📥️descriptor/💾️cache/🧪️tests/🟦️.ts` | cargo-units-processes | rc 0 (k4) | pass=21 fail=0 |  |
| `🦀️cargo/📦️units/📥️descriptor/📖️read/🧪️tests/🟦️.ts` | cargo-units-processes | rc 0 (k4) | pass=33 fail=0 |  |
| `🦀️cargo/📦️units/📥️descriptor/📖️read/🧪️tests/🧺️batch/🟦️.ts` | cargo-units-processes | rc 1 (k7) | pass=5 fail=2 | first error: expect(received).toEqual(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🦀️cargo/📦️units/📥️descriptor/🕸️nodes/🧪️tests/♻️composition/🟦️.ts` | cargo-units-processes | rc 0 (k4) | pass=3 fail=0 |  |
| `🦀️cargo/📦️units/📥️descriptor/🕸️nodes/🧪️tests/🟦️.ts` | cargo-units-processes | rc 0 (k4) | pass=24 fail=0 |  |
| `🦀️cargo/📦️units/📥️inputs/🔏️snapshot/🧪️tests/🟦️.ts` | cargo-units-processes | rc 0 (overlay) | pass=9 fail=0 | FIXED stale: Rust unit paths must be workspace-relative; fixture directory is the workspace |
| `🦀️cargo/📦️units/📥️inputs/🧪️tests/🟦️.ts` | cargo-units-processes | rc 1 (k7) | pass=2 fail=1 | first error: expect(received).toBe(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🦀️cargo/📦️units/🧪️tests/📐️profile/♻️preparation/🟦️.ts` | cargo-units-processes | rc 1 (k7) | pass=1 fail=2 | first error: error: Cannot use afterAll() outside of the test runner. Run "bun test" to run tests. ;  |
| `🦀️cargo/📦️units/🧪️tests/📐️profile/🟦️.ts` | cargo-units | rc 1 (k7) | pass=6 fail=1 | first error: expect(received).toBeLessThan(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🦀️cargo/📦️units/🧪️tests/📥️source/🟦️.ts` | cargo-units-processes | rc 0 (k4) | pass=1 fail=0 |  |
| `🦀️cargo/📦️units/🧪️tests/🟦️.ts` | rust-units | rc 1 (k7) | pass=11 fail=17 | first error: Original Nx SDK required ; CATEGORY env-official: needs SEMIO_NX_TOOLING_IDENTITY / Nx SDK capabilities of the official target environment (a probe with the installed identity turned `📦️artifacts/🔒️bounded` 7/7 green); verify through the official route |
| `🦀️cargo/📦️units/🧭️context/🧪️tests/🟦️.ts` | cargo-units-processes | rc 0 (k4) | pass=9 fail=0 |  |
| `🦀️cargo/🗃️unit-cache/💾️store/🧪️tests/📦️contract/🟦️.ts` | law-coverage-tests | rc 0 (k4) | pass=23 fail=0 |  |
| `🦀️cargo/🗃️unit-cache/🔏️key/🧪️tests/🟦️.ts` | law-coverage | rc 0 (k4) | pass=8 fail=0 |  |
| `🦀️cargo/🗃️unit-cache/🧪️tests/🟦️.ts` | cargo-unit-cache-processes | rc 1 (k7) | pass=22 fail=1 | first error: one byte above the high-water mark the oldest idle incremental directory goes first ;  |
| `🦀️cargo/🗃️unit-cache/🧹️gc/🌱️incremental/🧪️tests/🟦️.ts` | cargo-unit-cache-processes | rc 0 (k4) | pass=7 fail=0 |  |
| `🦀️cargo/🗃️unit-cache/🧹️gc/📈️fill/🧪️tests/🟦️.ts` | cargo-unit-cache-processes | rc 0 (k4) | pass=2 fail=0 |  |
| `🦀️cargo/🗃️unit-cache/🧹️gc/🧪️tests/🟦️.ts` | cargo-unit-cache-processes | rc 0 (k4) | pass=13 fail=0 |  |
| `🦀️cargo/🗃️unit-cache/🧹️gc/🧱️units/🧪️tests/🟦️.ts` | cargo-unit-cache-processes | rc 1 (k7) | pass=7 fail=1 | first error: within the unit interval and without pressure nothing is scanned ;  |
| `🧪️tests/🌍️caller-environment/🟦️.ts` | caller-environment | rc 1 (k7) | pass=1 fail=9 | first error: Original immutable SDK required ; CATEGORY env-official: needs SEMIO_NX_TOOLING_IDENTITY / Nx SDK capabilities of the official target environment (a probe with the installed identity turned `📦️artifacts/🔒️bounded` 7/7 green); verify through the official route |
| `🧪️tests/🎮️playground-preparation/🧪️tests/🟦️.ts` | playground-preparation-source | rc 0 (k4) | pass=4 fail=0 |  |
| `🧪️tests/📋️values/🧪️tests/🟦️.ts` | module-tests | rc 0 (k4) | pass=4 fail=0 |  |
| `🧪️tests/📐️phases/🟦️.ts` | module-tests-processes | rc 1 (k7) | pass=0 fail=2 | first error: undefined == true ;  |
| `🧪️tests/📚️library-search-path/🟦️.ts` | module-tests-processes | rc 1 (k7) | pass=1 fail=1 | first error: expect(received).toBeGreaterThan(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🧪️tests/📦️artifact-packages/🔮️oracle/🟦️.ts` | module-tests | rc 0 (k4) | pass=1 fail=0 |  |
| `🧪️tests/📦️native-dependencies/🧪️tests/🟦️.ts` | module-tests | rc 1 (k7) | pass=1 fail=5 | first error: prepareDependencies is not a function. (In 'prepareDependencies(row.kind, root, invocation, storage, phaseFor( ;  |
| `🧪️tests/🔗️import-edges/🏛️graph/🔑️authority/🟦️.ts` | import-edges | rc 1 (k7) | pass=0 fail=1 | first error: Original bounded Nx namespace required ; CATEGORY env-official: needs SEMIO_NX_TOOLING_IDENTITY / Nx SDK capabilities of the official target environment (a probe with the installed identity turned `📦️artifacts/🔒️bounded` 7/7 green); verify through the official route |
| `🧪️tests/🔗️import-edges/🏛️graph/🟦️.ts` | import-edges | rc 1 (k7) | pass=0 fail=2 |  |
| `🧪️tests/🔗️import-edges/📁️paths/🟦️.ts` | import-edges | rc 1 (k7) | pass=19 fail=8 | first error: Original bounded graph writer custody required ;  |
| `🧪️tests/🔗️import-edges/🔁️context/🟦️.ts` | import-edges | rc 1 (k7) | pass=2 fail=11 | first error: Original bounded graph writer custody required ;  |
| `🧪️tests/🕸️wasm-bindgen-locks/🟦️.ts` | module-tests | rc 1 (k7) | pass=1 fail=1 | first error: expect(received).toEqual(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🧪️tests/🗃️compiler-state/🟦️.ts` | module-tests | rc 1 (k7) | pass=15 fail=2 | first error: 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/🏃️execution/📜️script.ts ;  |
| `🧪️tests/🗿️artifact-source/🟦️.ts` | artifact-source | rc 1 (k7) | pass=0 fail=1 | first error: Select the active ticket with --ticket <directory> or SEMIO_TICKET_DIR ; CATEGORY checkout-resident scratch: the test needs sources/fixtures below the workspace; route `scratch: checkout` |
| `🧪️tests/🛑️coordinator/🔮️oracle/🟦️.ts` | module-tests | rc 1 (k7) | pass=0 fail=1 | first error: import.meta is only valid inside modules. ;  |
| `🧪️tests/🧱️command-source/🟦️.ts` | cache-command-source | rc 0 (k4) | pass=11 fail=0 |  |
| `🧪️tests/🧾️cargo-provenance/🟦️.ts` | cargo-provenance | rc 0 (k4) | pass=6 fail=0 |  |
| `🧹️pruning/⏱️continuation/🧪️tests/🟦️.ts` | pruning | rc 0 (k4) | pass=3 fail=0 |  |
| `🧹️pruning/⏱️measurement/🧪️tests/🟦️.ts` | pruning-laws-processes | rc 0 (k4) | pass=2 fail=0 |  |
| `🧹️pruning/⏱️recency/🧪️tests/🟦️.ts` | law-coverage | rc 0 (k4) | pass=5 fail=0 |  |
| `🧹️pruning/🌐️workspace/🧪️tests/🔗️imports/🟦️.ts` | pruning-laws | rc 1 (k7) | pass=0 fail=1 | first error: expect(received).toEqual(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🧹️pruning/🌐️workspace/🧪️tests/🧮️projection/🟦️.ts` | law-coverage | rc 0 (k4) | pass=4 fail=0 |  |
| `🧹️pruning/🏪️budget/📜️history/🧪️tests/📏️recensus/🟦️.ts` | pruning-laws-processes | rc 1 (k7) | pass=0 fail=1 |  |
| `🧹️pruning/🏪️budget/📜️history/🧪️tests/📦️compact/🟦️.ts` | pruning-laws-processes | rc 0 (k4) | pass=0 fail=0 |  |
| `🧹️pruning/🏪️budget/📜️history/🧪️tests/🟦️.ts` | pruning-laws-processes | rc 1 (k7) | pass=1 fail=2 | first error: Original historical control belongs to another caller ;  |
| `🧹️pruning/🏪️budget/🧪️tests/🟦️.ts` | cargo-retirement | rc 1 (k7) | pass=10 fail=2 | first error: expect(received).not.toBe(expected) ; CATEGORY roster/assertion drift: stale expectation vs refactored production |
| `🧹️pruning/📉️pressure/🧪️tests/🟦️.ts` | pruning-laws-processes | rc 0 (k4) | pass=4 fail=0 |  |
| `🧹️pruning/📦️metadata/🧪️tests/🟦️.ts` | pruning-laws-processes | rc 0 (k4) | pass=3 fail=0 |  |
| `🧹️pruning/🔒️incremental/🧪️tests/🟦️.ts` | pruning | rc 0 (k4) | pass=1 fail=0 |  |
| `🧹️pruning/🔒️store/🧪️tests/🟦️.ts` | pruning | rc 0 (k4) | pass=1 fail=0 |  |
| `🧹️pruning/🧪️tests/⏱️recency/🟦️.ts` | pruning-laws-processes | rc 1 (k7) | pass=0 fail=2 | first error: Exact finite scanner fixture output required ;  |
| `🧹️pruning/🧪️tests/🧮️plan/🟦️.ts` | law-coverage | rc 0 (k4) | pass=5 fail=0 |  |
