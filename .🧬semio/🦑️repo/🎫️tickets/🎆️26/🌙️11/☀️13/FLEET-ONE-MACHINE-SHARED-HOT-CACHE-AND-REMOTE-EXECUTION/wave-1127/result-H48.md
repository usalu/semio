# H48 result: lease-order self-deadlock of the compiler-state image, sandbox inputs, unit-cache seal

## 1. Self-deadlock (measured, root cause fixed)
- Worker exposure: no worker log shows it (`S/origins/*/ready-log-*.txt` and `*.json`: no `Self-deadlock`, `Selbst-Verklemmung`, `SEMIO_LEASE_ORDER`, `Nested persistent image`, `Lock order violated`). Only a FIRST creation of a compiler-state image (`cs/v/m/<sha>`, fresh funded child) can hit it; the ready stages never create one. No fleet outage; no outage exception used.
- Cause (diagnosed with test-side hooks, since removed): the funded creation of the child takes the birth lease `exclusive` (`prepare`), then its reservation body (incl. `port.open`) runs from the journal-queue drain of the PARENT, so the AsyncLocalStorage store at the open was the parent's frame (`cs/v`, disposable). `admittedParent.options===input` was false, the open was treated as unframed: before H35 it was refused ("Nested persistent image requires actual parent scope"), since H35's nested allowance it fell into the non-carried branch and requested the image `shared` while the same task still held the birth `exclusive`: 15 s wait, then `LeaseOrderErrorV1 Self-deadlock` (19x).
- Fix (landed 08:00, lock process, `T/🧰️scripts/H48/land-open-frame-registry.py`): `🧹️cleanup/🎛️command` keeps `persistentOpenFrames` (WeakMap options object -> parent frame); the funded and disposable `open` hooks and `runPersistentChild` register the exact options they open with, `openPersistentNativeImageModeV1` resolves the parent frame by options identity first (ALS only as fallback). Strict order now enforced: parent image before child, a child opened inside its own creation carried (no second lease), and a non-carried open fails AT ONCE (bilingual) when `holdsExclusiveInTaskV1` says the task already holds the image exclusively, instead of waiting 15 s. Same task stays re-entrant via F74's `holdsExclusiveInTaskV1`.
- Proof: units run after the landing: no self-deadlock any more (error text gone, run 8 min -> 4 min).

## 2. Sandbox lacks the profile policy Source (landed 07:00)
`CARGO_PROFILE_POLICY_SOURCE_PATHS_V1` (profile module) names the declared Sources (constraints, schema); `adoptCargoProfilePolicySourcesForV1` copies exactly those into every private repository (`🧪️testing/🧫️private-repository`). `Cargo profile policy Source disappeared` is gone from the log.

## 3. Unit-cache seal `🏁️entry.ts` (landed 07:00)
Inside the compiled `rc/<sha>/entry.mjs` bundle `import.meta.url` is the bundle, so `UNIT_CACHE_SCRIPT_V1/ENTRY_V1` pointed at `rc/🏁️entry.ts`. Launch, buildscript and the fleet uploader now use `physicalSourceOriginV1(import.meta.url, import.meta.semioPhysicalSourceURL)`. Side effect: the uploader script was never found inside the bundle (probable source of the `spool-full` skips on m01). The units test adopts the unit-cache script and entry closures into the private repository.

## 4. Next fault found behind it (staged for 09:00)
`descriptors()` census refuses real `lsof -F0pcfatn` output: unnamed descriptors (kqueue, pipes) have an EMPTY `n` field and `Unambiguous lsof field required` threw (93 such records on this Mac). Patch `T/🧰️scripts/H48/land-lsof-empty-name.py` (allow empty data for `n` only; `name()` already ignores empty). Test side landed: corpus row "NUL unnamed descriptor keeps an empty name field" + Python oracle in `💾️cache/🧪️tests` (red until the product patch lands).

## 5. Typecheck
`🧹️cleanup/📥️receiver/🧪️tests/📎️existing`: 4 errors fixed (kernel-ex narrowing, removed unknown `owner`, `inode!`); typecheck passed, test 4/0.

## Status (09:15)
Units 22 pass / 4 fail (was 21/5; the fixed-physical-backing test now passes; run time 10 min -> 4 min). After the lsof landing (09:00; descriptors test 1/0, cache tests 3/0) the real compile runs and two NEW fault classes show; both are in units code, which the 08:15 seal quiet window blocks for me:
1. `Live Original resource control issuance required` (`routing/invocation/process:342`, via `requireIssuedResourceControlV1`) in the renamed-dependency test, and `Original reader control changed` in the descriptors-persist test: a released or foreign control is reused after the compiler-state reader closed.
2. `Rust unit output inventory differs from exact declared artifacts` (`units/receipt`) from the unit compile child, in the ordinary Nx and consumed-byte tests: the output holds files the receipt does not declare (first suspect: extra outputs of the `trim-paths` profile or the incremental dir).
Goal 26/0 NOT reached. Not verified: Windows path of `ownedNativeToolPortV1`; a dedicated unit test for the open-frame registry (the units test is the proof); first creation of a compiler-state image on a real worker.
