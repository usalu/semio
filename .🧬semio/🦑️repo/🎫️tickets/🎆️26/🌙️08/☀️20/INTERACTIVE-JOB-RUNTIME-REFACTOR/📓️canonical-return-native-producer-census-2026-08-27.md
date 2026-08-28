# Canonical Return Native Producer Census

Read-only current source findings while the return framing RED is queued; this document does not claim source capture or live paging is mounted.

## Native Source Lifetime

`plugin/⚛️reactor/🦀️component.rs::poll_kernel` currently constructs effects, presence and the final `UiTurnPatches` as callback-local roots and returns `TurnResult` by value at its final expression. The WIT wrapper awaits that result and immediately calls `kernel_turn_result_to_wit`, which builds variable WIT payloads. A new owner that is installed only after encoding, validation, or WIT conversion would be too late. The original typed root and all incremental encoder/retirement cursors must be structurally retained before any of those operations can fail.

`PluginRuntime<PA>` is an exact per-embedding owner with persistent app/close registries and checked generations, but currently has no return-owner slot. The macro's persistent runtime is the actual native ownership location, not an actor-name lookup. One active return and its monotonic last-admitted origin must be attached to that exact runtime/activation. A public numeric origin alone cannot authorize resolving a replacement runtime. An executing semantic future's local descendants also cannot be dropped on cancellation merely because a return slot exists; producer cancellation must hand off or retain its real future/work owner.

The current output root includes `Vec<Effect>`, `Vec<PresenceUpdate>`, variable `TurnStatus` fault/checkpoint bytes and command-ingress fault bytes. UI patches now have the exact typed owner path already covered by the 18 Kernel tests. The remaining output vectors and nested effect `DslValue`/String/Vec fields do not acquire bounded final release merely by being placed in a `ManuallyDrop` wrapper. Their actual producers, storage backing and typed close cursors remain part of the required cutover. Header/body lengths and scalar metadata are not physical allocation certificates.

## Existing Entry Points To Cut Over Together

The component-model guest uses the existing async reactor export. Its eventual fixed result and mandatory drive are specified in `📓️canonical-return-contract-2026-08-27.md`; no second export or old/new result branch is proposed.

The existing repository-owned guest bridge also has `plugin_exports!::semio_owned_poll_v1`. It currently uses `owned_abi::take_json<PollInput>`, invokes the same `poll_kernel`, maps faults to whole encoded fault bytes, and calls `owned_abi::return_json(&result)`. This is an already existing entry point, not a newly proposed API. It must consume the same exact drive/fixed result and retain the same original typed source; otherwise changing only WIT leaves the whole native JSON result path intact. No change to this bridge was made during this inspection.

The existing raw WIT event lift still allocates a `kernel_events` vector. General event/request input ownership and the sixteen ordinary command route gaps are separate required work; the return protocol must reject control-plus-semantic input before executing or discarding any such root. Exact pre-admission protocol faults retain incoming ownership and execute zero semantic work.

## Immediate Verified Boundary

Actor return-codec4 and exhaustive Actor112 are actual native GREEN. WIT `return-page` is currently type-only, not connected to poll. Kernel record-header2 tests are mounted for an actual missing-API RED. None of these results establish the native semantic owner, full borrowed PACK encoding, host lifetime aggregate, or cancellation-safe output release described above.

## Required Native State Boundaries

The proposed retained slot has distinct admitted/executing, source-measure/encode, issued-page, source-retiring, terminal-release and retired-receipt states. This is not yet a mounted state machine. Original source and cursor fields must be outside unwind closures and never removed simply to advance a callback. The exact borrowed root cannot be rebound between measuring and encoding, and a partially retired source cannot be exposed to a formatter/encoder again.

An input ACK stages its exact receipt. It must not erase or overwrite the one issued page before the final clock verdict: a late failure would otherwise make an exact page replay impossible. The fixed page backing can remain physically resident, unchanged, while an accepted scalar state transition makes that page no longer issued. Only a later granted operation can overwrite/reinitialize it for a new page. There is no second hidden page or successful ACK inferred from an absent result. Preparing and validating the output envelope belongs before the final clock as well.

Final typed source destruction is different from the scalar input-ACK commit. All descendant release and final source-shell destruction must occur before the final clock, with a separately pre-admitted small terminal-release state retained if the clock fails. That state records the exact identity and completed physical release; retry must not demand a source root that has already been correctly destroyed, and must not synthesize terminal completion from an empty generic callback. This is the same sealed domain-release distinction required by the native guest lifecycle owner, not a generic `Drop` or `terminal=true` shortcut.

No successful source retirement may wait for a semantic UI ACK: the paired published UI owner is separately transferred to the original lifetime aggregate before raw source terminalization. Otherwise the one-active-return rule would block the semantic ACK execute that is needed to close the very return being held.

## Borrowed Field And Existing Effect Projection Audit

The current canonical UI owner exposes bounded `try_read`/`exact_node`, with typed close/copy/compare; its owner confirms no general borrowed typed-field visitor or PACK writer is provided. `pack::RetainedValueCursor` and `RetainedRecordBodyCursor` are input decoders, not native output encoders. The Store wire bridge at `🏪️store/🦀️component.rs::pack_rt::encode_wire_value` still clones the `DslValue` into a synthetic field-1 record before whole record-body encoding. None is an adequate substitute for retained borrowed symbol and value encoding.

The shared UI `🧬️contract/🧬️typed/📋️fields.rs` is a concrete reusable typed field roster for the future borrowed visitor. It enumerates the exact Rust fields/types for nodes and component payloads and is already used by typed retirement. It is not itself a serialized-field catalog: serde names, enum representation, optional-field omission and dynamic UiValue map symbols still require exact byte-oracle coverage. No UI roster or encoder source was modified during this inspection, and no bounded encoding claim follows from the roster's existence.

The existing reactor `kernel_effect_to_wit` is also not a pure byte iterator: its SetTimer branch validates the pre-admitted ARMED_TIMERS owner; a retained encoder must preserve that exact producer validation without turning its boolean lookup into new ownership. Other branches convert typed params into PACK, map option/byte fields and preserve SendMessage payload bytes. Current `kernel_endpoint_to_wit` silently turns an invalid Shell/PluginInstance string into numeric zero with `parse().unwrap_or(0)`. The staged message cursor instead rejects malformed, noncanonical or overflowing numeric identifiers while leaving the original source borrowed and unchanged. This strict behavior must replace the old helper on the eventual single poll cutover; no compatibility zero is being introduced.
