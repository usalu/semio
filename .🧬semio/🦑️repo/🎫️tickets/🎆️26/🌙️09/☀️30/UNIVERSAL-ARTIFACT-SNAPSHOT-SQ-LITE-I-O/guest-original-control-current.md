# Guest Original Control Current

Read-only observation 2026-10-09; concurrent signatures preserved, no execution.

Paths relative to `🧰️framework/` unless stated otherwise.

## Existing original boundaries

`🔨️modules/🚪️io/⏱️control/🦀️.rs:8–29`: `IoRunControl` borrows original decode/encode controls and immutable full `RetainedCloneGrant`. `decoding`/`encoding` bind one direction; `decode`, `encode`, `snapshot_encode` reborrow that actual ledger, while opposite-direction selection on a single-direction owner refuses. Guest export therefore receives original decoder; Guest import receives original snapshot encoder owner. Do not use an encoder borrowed from export's decoder-only route.

Current Store `🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:12361–12364` already gives provider export an original `NativeDecodeControl`, and import an original `NativeSnapshotEncodeOwner`. These added arities are sufficient to carry the original directional ledger into MCP; they do not carry an existing `EntityIdentityAuthority` object.

`🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs:4–11` keeps the borrowed encoder and independent five-axis grant. `receive` at 29 retains admitted intermediate/output ownership in the original retirement recipient on refusal. Its lifetime must remain inside the provider call: do not detach `native()` or consume a copied local continuation while losing original allocation/retirement ports.

Native encode/decode `scoped_observer` (`🌱️value/🛫️encode/🦀️.rs:27`, `🛬️decode/🦀️.rs:29`) reborrow original allocation ports, retirement slots, counters and callbacks; their scope drop restores counters even on unwind. Plain pause/resume retains scalar receipt but does not preserve allocation port/retirement recipient. Forwarded continuations explicitly retain those ports (`🛫️encode/🛂️allocation/🦀️.rs:30–44`, decode sibling 24–34).

## Identity receiving contract

Actual authority is `🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🦀️.rs:11–29`. `new_forwarded` accepts explicit ceiling plus original observer/preallocation port; `encode` saves every admitted charge before returning an error. `pause_forwarded` returns the original forwarded receipt, not a local default receipt.

`OriginalOperationReceiver` methods at 66–76 enforce exact remaining allowance, prior owned bytes, guest preallocation before ledger advance, progress and return reservations. `begin` derives remaining from original ceiling minus cumulative ownership. Forwarding must preserve this remaining ceiling; using total ceiling with a zero-origin child would widen authority after prefunding.

The authentic guest-side helper is `🔌️plugin/🚪️io/🛂️authority/🦀️.rs:21–32`: `with_operation_authority` obtains actual imported `operation_begin`, forwards imported progress/allocation, and sends `operation_finish(receipt.owned_bytes())` even after the operation fails. It never supplies an unlimited default. The full grant is a separate caller-owned field; this helper does not derive five currencies from allocation allowance.

## Smallest clean host integration

Keep provider directional signatures and reborrow the original control. Introduce a scoped receiving bridge that splits the original control's callback/preallocation/counter fields safely, rather than trying two closures that both mutably capture the whole `native`. Construct forwarded identity with exact remaining allowance; debit actual original `charge` for each accepted request and forward actual original callback/cancellation for every progress observation. Use one enclosing scope restoring original counters/ports on success, refusal and unwind, analogous to existing scoped observer implementation. `EntityIdentityAuthority::new` or `resume` from a newly created local receipt would discard the original allocation port and is insufficient.

For import keep `owner.grant()` unchanged and retain failure owners in `owner.receive`/original recipient. For export the decoder bridge maps physical authoring requests to the original decode ledger; operation progress must not reset or charge an unrelated encode controller. A new generic receiving helper belongs beside original native control APIs; MCP should consume it rather than access private fields or invent a fresh owner.

## Current callers requiring qualification

Current original SDK macro/WIT export calls in plugin root 45508/45531 and owned exports 45763/45775 lack controls; current helper functions 42767/42791 require controls. MCP 3198/3221 lacks new runtime identity argument. Host methods 1794/1799 and 2833/2845 require identity, but Wasmtime still copies payload with `to_vec` without using it. Compiled component tests and owned-instance SQLite lease tests retain old method signatures. Native providers now take snapshot encoder owners; all genuine factory/fixture calls must forward an independently authored full grant, never derive it from SQL allocation limits. Window execution agent owns SDK/Host five-axis changes; root owns this bridge.

Verification must prefund the actual original ledger, reject original allocation port requests, cancel after physical ownership, and prove original receipt/recipient survives refusal. No runtime qualification here.
