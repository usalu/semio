# Original Flow Cause Custody Audit

Read-only source audit; no runtime edits, native/browser/server execution, or qualification. Read the October 9 receipt and original closure notes. Paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/` unless stated otherwise.

## Exact Loss Sites

- `🕸️wasm/📡️protocol/🦀️.rs:44–56`: `FlowFailure` owns ABI code, newly allocated String prose and four-axis progress. It has no original typed error owner. `new` accepts generic text and initializes zero effects; `with_retained_progress` preserves effects only.
- `🕸️wasm/🦀️.rs:862–863`: `flow_close_value_failure` converts every ValueError to Busy plus empty text, copies its receipt, then drops the original kind and original Cow prose. It is used by original frame and Host demands/turns at 811, 820, 837–838, 851–852.
- `🌿️vcs/♻️retirement/🦀️.rs:15–18`: `FlowVcsCloseFailure` is Copy and keeps only fault/progress. Its ValueError conversion maps every kind to ClosePending and discards original prose. `🕸️wasm/🦀️.rs:1123` loses any remaining non-ABI detail when converting that object to FlowFailure.
- Actual Host retirement returns `Result<RetainedCloneStep,ValueError>` at `🖥️host/🦀️.rs:2944`; the defining cause survives this boundary until the above Domain conversion. Domain turn custody is therefore the immediate missing receiver, not evidence of a missing Host error type.
- Bridge close paths at `🕸️wasm/📡️protocol/🦀️.rs:486–487` and 631–632 retain actual progress before mapping failure to code, then discard the failure owner. Normal `FlowFeatureStep::Failed` at 532–535 copies prose into a reply and does not assign `failure.retained_progress` to Bridge progress. This normal branch is a distinct receipt gap.
- `failure_reply` at protocol 731–733 creates bounded ABI message bytes from borrowed prose and falls back to empty text. Native failure ownership and the allocation/close effects of the reply are not represented by this helper.

## Whole Cause and Lifecycle Scope

Canonical defining `🧰️framework/🔨️modules/🌱️value/⚠️refusal/🦀️.rs:3–17` has eight closed kinds and `ValueError {kind, Cow<'static,str>, retained_progress}`. `ValueError` already implements RetireOwned in its `♻️retirement/🦀️.rs`, distinguishing borrowed prose from the original owned String. Move this same complete error across close converters; adding only a scalar kind is insufficient.

Minimal coherent production scope: define a closed original Flow failure payload that owns either its explicit Flow failure text or the complete original ValueError; move that payload through FlowVcsCloseFailure and FlowFailure without formatting/cloning it; retain failed payloads inside actual operation/session/Bridge custody until their admitted reply and failure retirement complete. Quote and pay failure text/reply capacity, copy, release and depth with canonical grants. Preserve actual normal failure receipts before projection, as close already does. Constructors must explicitly select the authority and avoid allocating empty duplicate prose for typed errors.

Remove Copy from FlowVcsCloseFailure once it owns ValueError. Existing Clone on FlowFailure/FlowFeatureStep permits uncontrolled duplication of owned refusal prose: audit actual clone callers and remove unnecessary Clone or route any necessary clone through controlled ownership. Derived RetireOwned for new payloads must cover every variant; a typed error owner in a transient returned Err still drops uncontrolled unless original retained state takes custody. Do not claim fullclosure from the converters alone: generic feature/argument/observer Box owners and reply/output owners also require actual funded closure.

## Actual Browser Caller and Bounded Gates

Raw export is `🕸️wasm/🦀️.rs:5688–5690`, four u64 axes; poll clears receipt at 5661 even on cached transport retry. Actual receiving caller is `🕸️wasm/🖥️host/🏃️runtime/🟨️.js:81–87` (`pollTurn`), reading all four axes immediately after raw poll and storing frozen progress. `preserveFailureProgress` at 89–91 attaches this receipt to caught errors. Pump at 190–202 treats negative poll as generic `Flow bridge closed`; raw typed cause is unavailable. Actual browser wrapper `🕸️wasm/🌐️browser/🏃️runtime/🟨️.js:60` delegates `stepProgress` to that Host. Thus current receipt receiver exists, but no complete typed cause wire exists.

Next source gates, without running them in this audit: syntax of defining Value/Flow/VCS/protocol owners; existing retained-receipt TypeScript oracle with independent BigUint64Array and strict schema; protocol-unit/domain laws extended for all eight typed kinds, owned-vs-borrowed prose custody, zero-effect refusal and nonzero failed receipts; original VCS/Host/Domain close native laws only after the selected-copy native lane is released. Full browser binary publication requires a fresh build and actual raw-export receiver replay; source receiving green cannot establish publication.


## 2026-10-10 Original Cause TDD and Native Terminal

Root selected-copy session46394 physically terminated Nx1 after31m43s before any selected-copy assertion. Six shared semio-framework-job compilation errors were observed: WorkerAdapter step return type mismatch and missing borrow_outcome; two stale JobOutcomeDescriptorSlot::return_original calls; checked_out_outcome and outcome still returning StepOutcome where JobOutcomeDescriptor is held. Later SDK compilation observed only the first two remaining. No selected-copy native pass is claimed, and no unrelated Job source was edited.

Root typed-cause portable24294 reached the intended assertion RED after the original receipt, strict corpus and independent JSONPatch/UTF8 checks: FlowFailure lacked complete typed cause custody. The original FlowFailure now owns FlowFailureCause::Value(ValueError), moves its kind/prose/receipt without formatting or cloning, and declares controlled retirement through the same original cause. VCS close failures retain the actual ValueError and both converters move it. The native96-case kind/prose/receipt/System/conservation law was authored before this production change and remains unexecuted. Source verification40878 is active. Full Bridge operation/session/Box/reply owner closure remains open; representation alone does not qualify that lifecycle.

Host40 portable61031 physically reached schema-absence assertion RED. A concurrent source change asserted that the existing natural Host custody schema must not exist. Root removed only that contradictory test check and preserved strict whole-corpus Ajv and independent JSONPatch/UTF8 validation. New Host40 source verification60634 is active; prior Host39 result is historical only.


## Funded Original Metadata Handoffs

Geometry original Snapshot baseline physically reached its expected public denial RED: zero copy/capacity/release authority caused48 System birth and units1 with receipt0. Its same-source funded cursor then stalled under fixedcopy1 because Deferred<T> charged sizeof(T) for moving the same original field into its child authority. Root repaired exactly that structural metadata handoff to item1/copy0; genuine child birth/depth/release and payload element copies remain independently paid.

The next same two native laws physically proved public denial System0/0, units0 and receipt0. Internal close remained RED after143 turns: fixedcopy1, nextcopy16, capacity0/release0/depth8. The original empty MeshVertex.normal Option<[f32;3]> was gated by sizeof(Option<T>). Root repaired Optional<T>'s same original variant/child handoff to item1/copy0, including None; eventual Some payload processing still delegates to its genuine semantic child cursor. This repair follows an executed original runtime RED; its subsequent native fullclose is pending. These results qualify only the observed public denial boundary, not all12 modeling phases or Generator/Mesh WidgetJob.
