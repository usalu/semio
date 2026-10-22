# Closed Actual Reply Admission Design

Current unknown-role requests conservatively reserve the entire 16,764,800-byte reply quantum and then unconditionally close their cohort. Small actual replies therefore cannot amortize the immutable worker startup. Merely deleting the break leaves only12,416 bytes of reusable conservative admission and does not solve the general case. No implementation change is made by this report.

## Minimal Closed Protocol

Before each unchanged physical ReadRequest, send one closed ReadCredit frame {version:1,maximumReplyBytes:N}. N is bounded by the existing reply quantum and by the cohort’s observed remaining16MiB stdout after ready, stderr, failure and diagnostic reserves. This separate small frame preserves the existing64KiB physical read request limit. It is a budget, never custody or capacity authority. The child requires strict credit→request alternation, with explicit null termination only between pairs. The channel counts both input frames and their exact UTF8 bytes; no cap changes.

The child already computes a complete bounded result packet before publication. It compares actual UTF8 wire bytes to N. A fitting packet is emitted normally. An otherwise valid packet that fits the fixed per-reply quantum but exceeds N emits only a closed tiny {version:1,deferred:true,requiredBytes,readBytes} frame and then terminates with EOF/status0. No references, links or partial result are emitted. The parent validates exact shape, requiredBytes>N and <=fresh quantum, readBytes within the requested read budget and known physical lower extent, then requires joined EOF/status0 before retrying the exact same rows in a fresh cohort of the SAME immutable Source image and Original. Fresh-cohort deferral is an error, so this cannot spin.

The parent must not send null after a defer; the defer is already terminal. Normal completion still sends null and joins EOF/status0. Failure packets, bad terminal status, cancellation, deadline and cleanup retain current first-error behavior. A fresh cohort has the full fixed per-reply quantum; each cohort except the last must have accepted at least one result. Each request is read at most twice.

## Read Budget Is Not Refunded

This is essential: captureClosureProof currently reserves its remaining16GiB work budget before sending a request, and accept restores only unspent credit. A deferred worker HAS physically read and inflated the payload. Add an explicit deferred-debit callback to charge readBytes to proof.work.bytes, including compressed input, pack index and inflated bytes, without adding any references/candidates or calling normal accept. The next request receives the remaining cumulative budget after that debit. Resetting the precharge or ignoring the deferred work would weaken the current budget. If no valid defer frame arrives, the proof fails with its original reservation intact.

The worker can derive readBytes exactly from request.remainingBytes minus the actual budget.remaining after its complete serialization; this equals the sum of per-row expandedBytes, which already includes physical and inflated bytes. A tiny closed deferral carries only that debit and the required wire length.

## Test-First Changes

1. Extend the existing ReadSession schema/neutral fixture with ReadCredit and ReadDeferred, exact-fit/one-byte-over, coalesced replies beyond12,416 bytes, terminal defer, fresh-defer rejection, malformed credit/debit, and unchanged total-frame/input/output/failure/ready/diagnostic bounds. Use actual encoder/admission functions and SQLite arithmetic.
2. Update the current worker/session implementation coherently: explicit credit state, complete-packet comparison, tiny terminal deferral, actual remaining-budget calculation, joined fresh-cohort retry, and real deferred read debit. Remove the unconditional unknown-role success break only with this protocol.
3. Update every direct Node/channel caller to send credit before its request, preserving bootstrap/readiness and explicit terminal tests. Add actual immutable Node/channel coverage for small reuse, oversized-for-current-cohort defer, exact rows retried, physical worker turnover after capture, cancellation and primary-error cleanup.
4. Preserve the other owner’s evaluated-parent Source selection port, stronger per-unlink proof, fixed reply quantum and all Native/Original/heap/physical/Source guards. No heavy Source job before combined review.

## Root Credit Boundary Review

The canonical reply quantum16764800 equals16777216 total minus128ready,8192failure and4096diagnostic bytes. Fresh credit must still admit that quantum when already-observed stderr is within the4096diagnostic reservation. Counting observed diagnostics and another full4096reserve would double count the same allowance. A conservative remaining-credit calculation accounts actual stdout plus8192future failure plus max(observed diagnostics,4096), capped at the canonical quantum. If diagnostics exceed their reserve, credit decreases; total capture remains fixed. Add neutral exact-ready128/diagnostic0,200,4096 and overflow boundaries. Credit/request consume two input frames and exact UTF8 bytes, with a terminal-frame reserve. Parent must distinguish accepted-result count from bootstrap/credit frame count when rejecting fresh-cohort deferral.

## Combined Saved-Code Review

Root read the saved parent loop and actual owner debit: successful terminal deferral validates exact physical minimum including packed index extent, remaining cumulative read budget, output credit/quantum, accepted progress and joined EOF/status0 before debit or retry. Deferred requests publish no rows/references/links and retain the full precharge if validation fails. Existing Original/captured selection and first-error accepted-disposal helper are preserved. Low found one concrete missing receiver fence: normal success UTF8 wire bytes must also be <=issued credit before accept can publish references. Runtime is adding that guard with a regression. Root requested a mandatory seventh deferred-debit callback for every owned caller rather than an optional historical call shape. Native acceptance remains pending.

## Saved Implementation And Freeze

Schema/fixture and actual-function/SQLite tests were saved before implementation. The current schema contains ReadCredit/ReadDeferred and fixed maximumDeferredBytes128. Fourteen credit, twelve debit and seven actual-session lifecycle vectors are authored, alongside real-channel unknown-role reuse and repeated-row debit tests. All four owned runner callers now declare the required seventh deferred-budget callback.

The saved parent uses exact observed diagnosticBytes, reserving only the unspent portion of4096 diagnostic bytes; ready128 with observed stderr0/200/4096 retains the canonical16,764,800-byte reply quantum. Parent success is refused before accept if its actual wire exceeds the issued credit or fixed quantum. Terminal deferral is joined EOF/status0 before actual deferred debit and unchanged-row retry. No accept/refs publication or null frame follows defer. The owner updates only actual cumulative work bytes, preserving the fail-closed precharge on every malformed/fault path.

The concurrent evaluated-parent Source port is preserved: requireAdmittedBuildSourceClosureV1 consumes compiler/loader-injected admitted Source identity, not physical hashes observed at module load. No filesystem fallback or compatibility mode was introduced. The stronger current per-unlink/prefix implementation is unchanged by this protocol.

Combined coordinator/independent source review is clean. All own protocol files are frozen after coordinated test alignment. No new Node or full Source/Native probe was launched: the existing native frontends lack the admitted Source birth map, and their owner is repairing that upstream gate before dependency sync/retry. Earlier immutable-map smoke is historical evidence only and does not execute this new credit/defer path.

Exact current hashes and all regular/existent bounded close inputs are in generated/worker-credit-runtime-boundary.json. There is no new protocol-probe input; retained Node inputs remain node-worker-capability-probe.mjs and immutable-node-smoke.mjs.

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/👷️read.mjs: 37a28e404c0c25dabcbf54239b7763aacd87f8654aa95ee8bcd592299cf94247 (29006 bytes)
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟦️.ts: 92ae28a53303265368d90cfd8db6743c35745c0e57fb998fde44c7ee4d86e190 (9406 bytes)
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟨️.mjs: 78dc09e44dbeebfd4a0fedd15eb9684606df261109359cfc5a3c6276bc564840 (5868 bytes)
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟨️.d.mts: 1ca5739bb887d1d9f528dc588c47d6f7f7f7cf917ec62a1598bda01446d152ec (1340 bytes)
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts: ddd9e8909cc4f052d017b44950b4b72ec68381b9a745292cd4ea59064c666bac (50580 bytes)
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🟨️.mjs: 571c1d5d0b9b040158a4c7c5fd3185d15408e1970fd29cda04cc1eba9ac49db2 (8617 bytes)
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🟨️.d.mts: 9934e6e028e3a355f017716105b5a7bfd5884315d6ad838d498b454301420f66 (440 bytes)
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/🧬️schema/🔣️.json: 3ec9c98380ffb6c722bd1f0391ec191abfff3cf1e8eb587cfe7d738a986b14f9 (1663 bytes)
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/🧫️cases.json: 42eac49c980397577229a6e6454865d4a110d469dedaea9592f4b053f092a929 (34558 bytes)
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/🟦️.ts: 4401fae15ee84ea1c0b430aed7ed4b12f12853f8f4880932bd7e1dfa3e8ad48b (4932 bytes)

## Initial Captured-Source Input Reserve

Root found the Source preparation precheck still used bootstrap+maximumRequestBytes+6, accounting only for the old request/newline/null framing. The new cohort admission correctly includes a credit frame, so a near-bound Source descriptor could pass preparation then truthfully refuse the first cohort. Runtime is adding an independent boundary case and the exact maximum-credit JSON wire reserve to initial preparation; no transport cap changes or fallback. This is a narrow credit-feature coherence correction during the shared pre-sync edit window.
