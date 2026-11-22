# Result E57 (nx-orphan-funding)

## 1. Orphan reserved Nx funding allocation: product fix, landed and proven live
Cause: a funded child reserves its allocation (`reserved`) before it creates the namespace; a process that died in between left `516cc48fac9a...` (never a namespace receipt, no directory) in `.cache/b/n/m/storage-events.jsonl`. The deterministic child directory of the null-graph nx-namespace birth (`.../80eea0c5.../native`) matched it, so the Nx quote said `protected` ("Original committed Nx native funding required") and the reservation refused "already held". No path retired it.
Fix (event-sourced, no journal rewrite, no PID/age rule):
- Schema first: `native-build/💾️storage/🌱️birth/🧬️schema/🔣️.json` got `UnbornReservationRequest` / `UnbornReservationRetirement`; pure `nativeStorageUnbornReservationsV1` in `🌱️birth/🟦️.ts` names the reserved `compiler-state` rows of exactly the child directory, without namespace receipt or funding association, when neither the child nor the namespace directory exists.
- Recovery: `withNativeStorageFundedChildReservationV1` (`💾️storage/🟦️.ts`) appends `retiring` + `retired` for those rows inside the same journal transaction, before its own `reserved` event. A dead reserver is proven by lease exclusion: every live reserver holds the exclusive `native-fixed-image:<dir>` lease, which the new birth holds now.
- Nx admission: quote status `orphan` (`🪙️funding/🟦️.ts`: exact owner/kind/parent identity, unborn); `openNxGraphNativeChildV1` accepts it, the capacity demand subtracts the orphan's released bytes.
Proof:
- Language-agnostic: `🌱️birth/🧪️tests/{🔣️.json,📋️schema.json,🟦️.ts}` (11 cases; Ajv schema + SQLite oracle) 1 pass. Nx quote fixture got 8 orphan cases, oracle extended in SQLite: `🪙️funding` 3/0.
- Real birth: new test in `🏦️funded/🟦️.ts` seeds a reserved orphan, births the namespace, asserts exactly `retiring`,`retired` appended for it and the journal prefix untouched: `🏦️funded` 9/0 (128 s). `🧱️fixed` 7/0.
- Live: `repo test-file caller-environment` at 15:04 ran the plain nx-namespace birth; journal lines 158-159 now retire `516cc48f...` (no hand edit); no `reserved` row is left in the journal. The "native funding required" error is gone.
- `caller-environment` still FAILS: the first test, "daemon startup admits one producer across parallel clients", times out at its own 30 s under load 25-35 (see findings-E57.md); other tests of the file not individually verified.

## 2. native-caller fixture
Test + fixture drift fixed: `codeRoot` in four capability literals (+ the `bound` envelope), expected program `repo`, cargo default target dir `/repo/target` (target-dir now resolved like Cargo; fixture `producerCases[0]`, `launchResources`), copy-not-identity checks (envelopes are now read as copies: JSON equality), owner-handoff test takes the checkout's own `.cargo/config.toml` target-dir (cross-checked with `Bun.TOML.parse`) and sets `CARGO_TARGET_DIR` for the NativeScript. Result 7 pass / 10 fail -> 16 pass / 1 fail. The remaining test ("actual native owner process receives original build mode policy and deadline") reaches the live repository store: needs the private repository workspace (test-side).

## 3. The 19 "Original required" files (one at a time, `repo test-file`, load < 40, free memory >= 25 %)
| file (below caching) | before (E48, 14:17-14:28) | after (15:07-15:35) | what changed / class |
|---|---|---|---|
| storage/budget/graph | 5/0 | 5/0 | green |
| playground-preparation | 4/0 (E48 probe 3/1 deadline) | 3/1 | load-flaky "Original Script deadline refused" |
| storage/budget/tests/parent | 24/24 | 21/27 | `observe-envelope-drift` harness drift (E47/E59), 3 more detach tests red; no link to this change found, NOT verified by revert |
| native-orchestration/invocation | 2/2 | 2/2 | fixture drift "Original Repo build caller required" |
| native-orchestration/test-body | 0/9 | 0/9 | "Live Original process issuance required" test-side |
| native-orchestration/owner-cmd-policy | 5/6 | 5/6 | test authors own invocation JSON |
| bootstrap/admission/storage | 0/3 | 0/3 | live checkout store, needs private workspace |
| opening/publication/native-build | 3/4 | 3/4 | "asset purpose required" test-side |
| tools/use (toolkit-use) | 0/1 | 0/1 | with nx-namespace launcher: next error "reached the live repository store" (private workspace); row not kept |
| graph/waiter | 0/1 | 0/1 | with launcher: "Original graph snapshot owner required" (authorizer not loaded in test) |
| tools/use/task-child | 0/2 | 0/2 | with launcher: runs 41 s, fails on daemon-mode child test (deeper), row not kept |
| graph/startup/service | 0/2 | 0/2 | with launcher: first PRODUCT bug found and fixed (below), next: sandbox workspace has no nx.json (test-side) |
| nx-plugin/arguments `🧪️tests/🟦️.ts` | 1/5 | 3/3 | watcher birth (E49) + wedge gone; static owner command `toBe`, dispatcher "capacity sized at 46 %" refusal; nextest filter test slow (not analysed) |
| nx-plugin/arguments native-caller | 0/1 (E49: 10 fail) | 16/1 | section 2 |
| import-edges/authority | 0/1 | 0/1 | no fileLauncher row for it; needs nx-namespace row + test fix |
| import-edges/paths | 1/8 | 1/8 | graph-writer scope wrapper in test |
| graph/seal/selection/scope | 6/12 | 6/12 | "Required original cache consumer unavailable" (test must open the store first) |
| nx/graph/inputs/data | 10/1 | 10/1 | same class |
| caller-environment | fail (funding wedge) | fail (30 s daemon test timeout) | wedge fixed |

Product-side fix found by this pass: `🏗️build/🧠️memory/🦶️footprint/🟨️.mjs` imported `bun:jsc` with a literal specifier, which breaks the esbuild bundle of the daemon service ("Could not resolve bun:jsc"). Both imports now use a computed specifier; footprint tests 53/0. Landing recorded.
No `fileLaunchers` row was kept: with the nx-namespace launcher none of task-child, toolkit-use, service, waiter turns green; they only move to the next test-side error (E38 precedent: no rows that cannot pass). The suite file is back at its original sha.

## NOT verified
- Full `caller-environment` and full `arguments` file green; parent 21/27 vs 24/24 not bisected.
- Orphan retirement under a concurrently live reserver (lease exclusion argued from code, not stress-tested).
- Other journal-owner callers of `withNativeStorageFundedChildReservationV1` besides Nx (only `🧱️fixed` 7/0 and the Nx tests were run); `💾️cache/🧹️retirement` has 3 failures (descriptor custody), not on this path.
- `native-build/💾️storage/🧪️tests` 5/1: stray `.cache` directory (E52 class).

## Others must know
- Landings: `T/landings.md` sections "Wave 1122 E57" (two). Scripts: `T/🧰️scripts/E57/run1.sh`. Generated folder deleted. Locks released.
- Any further reserved row left by a crash between `reserved` and `namespace-created` is now retired by the next birth of the same namespace, never by age.
