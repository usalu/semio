# Current Canonical Retirement Replacement Policies

Read-only source audit against the retained509 OS compiler diagnostics; no production edits, API restoration, test execution or caller repair performed. Diagnostics are historical current-run observations from the named VCS gate; source may be changing concurrently.

## Full-grant contract

`🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🦀️.rs:5` defines independent copy/capacity/release/depth demands. `ErasedSnapshotRetirement` at11 takes `close_step(RetainedCloneGrant)` and returns `RetainedCloneStep`; every implementation supplies copy, capacity, release and depth demand observers. The removed `SnapshotRetirementStep` and pair `(maximum_items,maximum_bytes)` are not the replacement policy. `🧬️retained-clone/🦀️.rs:135` separates all five grant fields. Its one-capacity/payload/release constructors at147–157 grant exactly one currency and zero the other currencies. Consequently, replacing an old byte limit with the same number in copy/capacity/release simultaneously would add authority, not preserve old semantics.

The new step carries full progress on both Progress and Complete. `fits` at179 checks each currency separately, and `admit_retained_clone_close` at189 additionally rejects Complete without actual terminal-empty. Callers must retain progress receipts and the exact completion witness; enum-name substitution or collapsing Complete(progress) to a bare boolean loses accounting. Outer cursor/frame destruction is separately funded after child terminal-empty, as the canonical `factory_ticket_demands` and `close_factory_ticket` law does at `♻️retirement/🏭️factory/🦀️.rs:99` and105. An unchanged live original owner must remain retained on insufficient birth/release admission or failure.

## Construction and transfer

The actual replacement for removed `retirement::owned_retirement(value)` is `admit_owned_retirement(value,grant)` at `♻️retirement/🦀️.rs:473`, with constructor extent measured by `owned_retirement_birth_bytes::<T>()`. It returns `(ticket,progress)` on success or `(error,original_value)` on refusal. The factory traits additionally require `FactoryRetirement`, `retirement_birth_bytes`, and full-grant transfer, rather than constructing an unmeasured cursor Box.

`RetainedCloneSource::from_authority` is absent from the public source contract. Current `🧬️retained-clone/🔗️source/🦀️.rs:42` measures `constructor_capacity_bytes::<A>()`;43 admits `(owner:Arc<T>,authority:A,grant)` and returns both original owner and authority on refusal. The fixture-only helper at83 is cfg(test), not a production replacement. Callers must admit capacity and depth before alias/payload transfer and keep original retained closure authority until terminal-empty.

The queue replacement is `RetirementQueue` at `♻️retirement/📋️queue/🦀️.rs:8`. Reserve a backing slot with reserve_step and then admit_owned; admission refuses without reserved slot, item or depth. It reports frontier capacity, payload work, original release and nested depth through separate observers. A plain Vec push/drop would bypass the canonical pre-admitted backing and original physical retirement.

## Generic clone-handoff issue

Current `RetainedCloneCursor<T>` at `🧬️retained-clone/🦀️.rs:219` exposes close copy/capacity/release demands but does not expose a generic close-depth observer. Concrete `RetainedCloneSource` does expose `next_close_depth_demand` at source60. Existing OS handoff `🏪️store/🧬️snapshot-clone/🚚️handoff/🦀️.rs:36` is still written against removed pair-style close APIs. Its policy cannot be completed by guessing depth0 or pretending closure supports no birth. The actual typed cursor authority or original issuer depth must be retained and used; if no such generic authority exists, refuse UnsupportedOwner until an explicit domain-neutral contract can represent it. No removed compatibility API should be restored.

## Diagnostic scope

The509 retained diagnostic list has52 erased implementations with removed three-parameter close methods and52 missing all four new demand observers;57 unresolved owned_retirement calls;33 erased next_close_byte_demand methods;8 owned factory implementations missing grant and8 missing birth extent. This aligns with a real lifecycle contract transition. The full-grant source/caller join must precede fresh native qualification; the compiler count by itself says nothing about domain semantic SQLite rows.

## Independent provider proposal refinement

`📥️inputs/stdio-eighty-nine-independent-provider-grouped-branches.rs` preserves every89 literal kind/standard/subset/schema patterns, grouped into63 arms only where concrete Snapshot, concrete Mutation and literal schema are identical. The local typed_snapshot_owner macro explicitly constructs independent TypeId, concrete SQL schema and bare typed provider hooks. STEP cc1–cc6 mutations retain separate groups. The original explicit89-arm proposal remains retained for review. Neither version has been mounted or compiled by this audit.

Actual old OS scheduling boundaries were also inspected: store9917/9955 Blocked yields the interactive job;9173 turns Blocked into registryContended;12123 aborts a cold bounded close as outstanding read-lease blockage while Pending continues. store583/586 distinguish mutex/lease contention from underfunded release at589. Therefore a blanket zero-progress mapping loses actual caller meaning: Progress(default) neither implies lease contention nor grants another action. Preserve caller-specific scheduler yielding and observe original lease/lock versus independent funding demands; avoid busy loops or falsely reporting terminal completion. These anchors were sent to the physical execution agent.
