# Native Contract Audit 44

Read-only production/schema/test inspection on 2026-10-13. Root AGENTS.md read. No tests, feature processes, git mutations, production changes, source copies, or generated outputs. Line numbers describe the inspected working tree and may advance independently. This report is evidence of contracts and source contradictions, not current acceptance. The n9-frame historical whole73/native62pass11fail observation supplies no current acceptance.

## Canonical Interpretation

`🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs:116–123` explicitly defines `maximum_copy_bytes` as bounding **payload copying**, independently of capacity and physical release. Its canonical `🌐️wire/🧬️schema/🔣️.json:1–42` requires five independent nonnegative axes but contains no machine-size reinterpretation. Capacity reservation, initialized transient metadata, owned transfer, and actual borrowed payload duplication are distinct operations. Neither blanket zero fees nor blanket `size_of` fees follow.

Current examples preserving that distinction:

- Job `🦀️.rs:2088–2096`: publication of the already-owned fault page and ledger alias has copy demand zero; initializing fresh `None` slots consumes an item and depth, without payload-copy bytes.
- Job `👷️worker/🚪️admission/🦀️.rs:108–118`: staged transfer really copies a still-owned source's byte ranges with `copy_nonoverlapping`; each range consumes its actual `bytes`. Its later params/presence custody handoff has copy bytes zero. Do not zero the range fee merely because the final semantics are a move. This is representation copying of the original source, distinct from deep cloning pointees.
- Job `🦀️.rs:3745`: borrowed scalar emission copies `source.len()` into inline pending storage and consumes that exact payload extent.
- Job `🦀️.rs:2119–2153`: original worker params move into cancellation retirement with zero copy demand; release remains independently demanded, and ledger release is conditioned on actual `Arc::into_inner` ownership.

## Prioritized Contradictions

### P1 — Public Payload Writing Bypasses Original Copy Credits and Receipt

`🧰️framework/🔨️modules/🧵️job/🦀️.rs:735–758`: `begin_staged_page(cx)` admits a page; subsequent public `write_staged(&[u8])` copies borrowed bytes with `copy_nonoverlapping` but takes no original context, copy grant, cancellation witness, or receipt recipient. Its sole bound is backing capacity. Repeated writes can copy data outside the caller's five-axis retained budget.

`🦀️.rs:703–715` admits the page using per-operation/process ledger limits and a one-page-opportunity boolean, without consuming the context's retained capacity/copy receipt. `🦀️.rs:772–787` checks yield once, then `JobPayloadPageGrant::write` (`836–845`) copies a full capacity-bounded slice without remaining-copy-credit admission or `consume_retained`. The page grant carries writer, ledger, source, length and commit flag (`823–828`), not original retained authority. This is a real payload copy, unlike page/header publication.

Concrete frontier: couple every payload copy to the same original control and cumulative recipient, cap copied range to independently remaining copy credits, account physical page birth/release separately, and preserve staged backing and source on cancellation/refusal. Avoid curing the bypass by pricing owned page publication by header size. Existing direct callers include `♻️retirement/📄️payload/🧪️tests/🦀️.rs:32` and `🧪️tests/🔬️retained-ownership/🦀️.rs:79`; those are source evidence only, not executed acceptance.

### P1 — Atomic Owned Job Admission Invents Copy Charges for Moves and Fresh Slots

`🧵️job/🦀️.rs:3106–3107` adds machine extents of the fixed `Option<JobPayloadPage>` array, cursor, `J`, `Option<J>`, `Option<BatchJobParams>`, bools and close-stage byte to copy demand. `3114` blocks on that invented copy sum and `3119` receipts it as copied bytes. `3117–3118` actually initializes fresh slots and transfers `job.take()`/`params.take()` into owned storage. The same constructor's fault string **is** genuinely copied (`2043–2047`) and therefore its literal length must remain charged. `2035` similarly adds `size_of::<J>()` to the direct owner's admission quote even though `2039` takes the owned job.

Concrete frontier: remove only metadata/move components of the atomic quote; retain actual fault literal copying, capacity births, items/depth and original control. Keep staged range-copy pricing above unchanged. Align direct-owner and session admission quotes and their actual receipts.

### P2 — Value Shared Ownership Still Prices Alias Bookkeeping and Owned Unwrap as Payload

`🌱️value/♻️retirement/🔗️shared/🔒️sealed/🦀️.rs:48–51` prices `Arc::clone` alias bookkeeping at `size_of::<Self>()`. `65–68` prices `Arc::try_unwrap`'s owned value return at `size_of::<T>()` in addition to genuine shared-header release. `25–29` prices returned-alias transfer by `size_of::<SealedShared<T>>()`; `55–56` prices initialization of a fresh fixed return slot by the return-slot machine size. These functions do not duplicate the pointee payload in the inspected bodies.

The fee propagates through `🌱️value/🧬️retained-clone/🔗️source/🦀️.rs:49–74` (source constructor quote/receipt) and `96–105` (authority take). Source constructor transfers `authority` into `SealedShared::admit`, clones the resulting shared ownership handle, creates lease/header scaffolds and projects a pointer; its `constructor_copy_bytes` is specifically a shared-handle size, not copied source payload. `🧬️retained-clone/🦀️.rs:71–80` similarly exports alias/binding machine-size demand.

Concrete frontier: classify these original ownership operations under item/depth/capacity/release as appropriate and make payload-copy receipts describe payload copying. Preserve source custody, exact pointer identity, unique unwrap refusal, shared-reference lifetime and typed incremental retirement. This finding does not imply zeroing all Value clones: borrowed scalar/string/collection payload duplicates still need actual fees.

Existing tests encode the current size-fee API rather than independently define payload semantics: source `🔗️source/🧪️tests/🔬️unit/🦀️.rs:14,21,57,70–71` obtain fee from `constructor_copy_bytes()` or `size_of::<SealedShared<String>>()`; line 21 also checks original owner pointer preservation. Thus keeping these assertions green by retaining invented fees would not settle the contradiction. No tests were run here.

### P2 — Original Cancellation Binding Is Optional and Uneven Across Admission Doors

`🧵️job/👷️worker/🚪️admission/🦀️.rs:4–10` gives `WorkerJobAdmissionControl::admission_cancel` a default `None`. `WorkerJobAdmissionContext` (`13–21`) carries operation, generation, budget, clock and recipient but no cancellation reference; open admission checks only fuel/deadline. The StepContext implementation (`23–28`) carries the actual original cancel reference and checks cancellation/yield.

Atomic admission `🧵️job/🦀️.rs:2034,3112` verifies cancel-node identity only when the control provides a token; parameter-token cancellation is still checked. Staged `👷️worker/🚪️admission/🦀️.rs:75–81` checks source/params/control operation and generation plus parameter cancellation, but has no cancel-node identity comparison even when the supplied control carries a token. Consequently the same source admission law differs between doors. This is an explicit-original-control gap, not evidence that parameter cancellation is absent.

Concrete frontier: bind the original cancellation witness consistently at the control boundary and enforce matching identity in staged and owned entry points. Preserve the already-established external cumulative recipient and remaining-credit deduction (`19–21`, Job `977–985`) and incremental cancellation retirement.

## Architecture Evidence and Limits

Actual Job package dependencies are Trace, Async, Value and Diagnostic (`🧵️job/📦️packages/🦀️rust/Cargo.toml:34–38`); inspected production body uses canonical Value grants and retirement, not product-specific payload types. Actual Value package dependencies (`🌱️value/📦️packages/🦀️rust/Cargo.toml:24–29`) include Value derive, IO Base64 and Schema Registry; body `🌱️value/🦀️.rs:444–450` actually constructs registry export records/calls registry registration. Dev dependencies (`31–36`) include Artifact Reference, DSL Record/derive and Diagnostic. Dependency presence and actual use must be compared with the current domain classification; a name/import alone is not sufficient to assert a forbidden General→Specific edge. No product Store/World/Renderer dependency was observed in these two package manifests. The peer public-import audit owns broader actual General→Specific evidence; this report does not invent or certify that boundary.

The genuine priority is preserving canonical original control/receipt and payload-versus-owner semantics before changing acceptance fixtures. After those fixes, owning Job/Value tests and language-neutral laws need a fresh authorized run; this read-only audit makes no runtime, heap-balance or current acceptance claim.
