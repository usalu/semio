# Draw Retained Mutation Admission Budget Contract

## Scope

This is a read-only source and recorded-run audit of the live `drawing-store.mutation-aggregate-byte-capacity` refusal. It does not run Cargo or change source.

The recorded Native 4 failure is reproducible at the first intended boundary: a 4,096-byte `RenameLayer.new_name` on the production-style `nested_snapshot()` rejects with `drawing-store.mutation-aggregate-byte-capacity`, before the overlay is bound or the mutation lifecycle begins. The recorded primary failure and its seven downstream lifecycle failures are at `🗑️generated/astra-runtime/draw-native-4.log:590-739`.

## The Canonical Bounds Already in the Product

There are two existing bounds with separate meanings:

| Bound | Current value | Actual owner | Meaning |
| --- | ---: | --- | --- |
| `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` | 4,096 | owned scalar field decoder | Maximum decoded owned field page. Draw aliases it as `DRAWING_OWNED_FIELD_BYTES`. |
| `ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES` | 262,144 | envelope/document decode | Maximum accepted decoded envelope/document bytes. Draw aliases it as `DRAWING_MAXIMUM_NESTED_BYTES`. |

Source: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:8274-8278` and `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:11`, `714-725`.

The source-document census already enforces the document bound independently. It counts actual owned capacities for the snapshot, layers, strings, vectors, and assets; it rejects either source or candidate census above 4,096 items or 262,144 bytes. `DrawingSnapshotBoundsAuthority::add` is the authoritative check at `…/🧬️schema/🧰️owned/🦀️.rs:1515-1531`, and the traversal/capacity rules are at `1533-1677`.

The mutation field lane also has its own 4 KiB guard. `DrawingMutationDigestCredit::observe` rejects a field longer than `DRAWING_OWNED_FIELD_BYTES` with `drawing-store.mutation-field-capacity`; the decoded envelope mutation itself is an `OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>`. See `…/🧬️schema/🧰️owned/🦀️.rs:487-639`, `2629-2669`.

Those contracts are coherent and must stay. In particular, lowering or raising the 4 KiB field page would not repair this failure.

## The Incorrect Aggregate Contract

`DRAWING_MUTATION_AGGREGATE_BYTES` is currently assigned the **document decode** maximum (`262,144`) at `…/🧬️schema/🧰️owned/🦀️.rs:714-724`. `DrawingMutationAggregateReservation::admit` then compares a sum of nine different kinds of ownership to that unrelated document limit at `3481-3570`.

The present sum is:

```text
source snapshot
+ source “candidate” census
+ borrowed mutation source owners
+ mutation derived owners
+ duplicate-id owner
+ retained candidate authority object
+ reverse/output container backing
+ page backing
≤ one decoded document maximum
```

That equation is not a bounded memory model. It is why the normal 4 KiB rename cannot start: it adds an already-live source document, a borrowed mutation, a fixed candidate authority, and backing that has already been preallocated in the borrowed arena, then compares all of it to the size permitted for one decoded document.

### Term-by-term ownership audit

| Reservation term | What source proves | Correct treatment |
| --- | --- | --- |
| `source_items`, `source_bytes` | The live `&mut DrawingSnapshot` already exists before candidate admission. The snapshot census validates it against the document bounds. | Keep as an independent source-document validation. Do not reserve it a second time as candidate backing. |
| `candidate_items`, `candidate_bytes` from `DrawingSnapshotBoundsAuthority` | The current source census reports zero derived ownership: `direct_shape`, root, and asset census each pass zero candidate figures (`1533-1677`). | Do not use it as a pretend clone allocation. Replace it with an operation-aware clone/work planner when a candidate actually allocates a copy. |
| `mutation_source_*` | `DrawingMutationDigestCredit::source_string/source_vec` observes owners of the caller-provided, borrowed mutation (`2629-2658`). | Validate the incoming mutation at the decode/field boundary. It is not candidate-owned backing and must not be re-reserved here. |
| `mutation_derived_*` | This is intended to represent copies made by the candidate. For a derived string it deliberately adds no bytes because the result belongs in a preadmitted page (`2634-2639`). | Retain only after an operation-aware planner ties each credit to a specific post-admission allocation or an already-reserved page. |
| `duplicate_candidate_*` | The duplicate id string comes from `DrawingMutationArenaOwner.duplicate_id`; its capacity is already counted in that arena owner. | Remove this second charge. It is duplicate accounting. |
| `authority_*` | `size_of::<DrawingMutationCandidateAuthority>()` is fixed retained candidate state. | Charge once as fixed candidate working state. |
| `container_*` | Reverse/output vectors are preallocated by `DrawingMutationArenaOwnerBuilder` with 64 slots each (`801-821`). | Charge as part of exactly one borrowed arena owner, not independently and again through a generic aggregate. Preserve structural checks in `DrawingContainerRebuildAuthority::new` (`2167-2185`). |
| `page_*` | Sixteen 4 KiB overlay pages and their catalogue are preallocated by the same arena owner (`823-844`); individual mutation fields use those pages. | Charge as part of the same one arena owner. Keep the 4 KiB field and page-slot checks. |
| `maximum_container`, `container_slots` | These constrain a particular container rebuild, rather than total allocation. | Keep them as a structural precondition; do not treat them as a document-byte quota. |

`DrawingMutationArenaOwner::admitted_totals` is the canonical owner-level measurement. It counts the owner object, both vector capacities, the page catalogue, all 16 page capacities, and the duplicate-id capacity in one checked calculation (`735-744`). The candidate currently splits and partially recomputes that same backing, which makes the equation neither exact nor composable.

There is a second correctness hole that a larger aggregate constant would conceal: `DuplicateLayer` can clone the selected source subtree after admission, but its mutation digest takes the generic `DuplicateLayer(_) => finish` branch. The source census also reports zero candidate bytes. The clone’s capacities are therefore not represented by a pre-admission, operation-specific work credit. See `DrawingMutationDigestAuthority::step` at `3250-3456` and the real duplicate clone route at `4190-4240`.

## Required Bounded Contract

Define three named budgets rather than one overloaded aggregate:

1. **Document admission.** The existing source census admits `S ≤ D`, where `D = ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES`, and its item counterpart. This is a property of the retained document.
2. **Mutation-input admission.** The owned decoded mutation has `M ≤ P`, where `P = ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES`; every owned field remains bounded by `P`. This is a property of the incoming mutation.
3. **Candidate working-set admission.** Before it may bind an overlay or move source ownership, a candidate plans every owner it will create. Let:

```text
A = exact admitted totals of one borrowed DrawingMutationArenaOwner
Q = size_of::<DrawingMutationCandidateAuthority>()
C = exact dynamically cloned/candidate-owned bytes for this mutation
```

The candidate owns exactly `W = A + Q + C` while it runs. `A` is obtained from `DrawingMutationArenaOwner::admitted_totals`, not reconstructed from a separate list. `C` must come from an operation-aware planner that follows the same subtree/value route as `DrawingLayerCloneAuthority`, fill/stroke clone authorities, and the duplicate rewrite. For an operation that uses only an overlay page, `C` is zero because that page is already in `A`. For a duplicate, `C` is the selected subtree’s actual retained capacities. For create/fill/stroke it is the actual copy plan for the supplied value.

The total simultaneous ownership of a single operation is then observable as `S + M + W`. It is bounded without a magic new cap: `S ≤ D`, `M ≤ P`, `C ≤ max(D, P)` after the planner validates its source/value, and `A` and `Q` are fixed, checked implementation terms. Thus the conservative architectural maximum is `2D + P + A + Q`; it is a derived accounting identity, not a replacement document limit.

The four-slot arena bootstrap has a different boundary again: its allocation claim is `4 × A`, using the same checked one-owner total. It must not use `4 × D`; source documents are not preallocated arena slots. `DrawingMutationArenaBootstrapAdmission::fixed` currently uses the latter at `1120-1132`.

This repair keeps all values bounded, keeps the 4 KiB field rejection, and stops charging borrowed source/mutation values or one arena’s components twice.

## Minimal Production Repair

The owner-level repair is confined to `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs`:

1. Replace `DrawingMutationAggregateReservation` with an explicit `DrawingMutationWorksetPlan` containing independent validated document/mutation inputs, the one borrowed arena’s `admitted_totals`, candidate-authority size, and operation-specific clone credits.
2. Make the plan finish before `BindOverlay`. Reuse/extend the existing traversal machinery so duplicate plans the selected subtree before `PrepareOwnedValue`; do not estimate from layer count or serialized JSON length.
3. Derive the arena bootstrap maximum from a single owner’s actual checked `admitted_totals` multiplied by the four pool slots.
4. Leave `DrawingContainerRebuildAuthority`’s capacity/rollback predicates and `DrawingMutationDigestAuthority`’s 4 KiB field predicate intact. They protect different boundaries.

This is intentionally not an increase to `DRAWING_MUTATION_AGGREGATE_BYTES`. That constant should disappear from this path because no source-backed aggregate has that meaning.

## Fail-First Boundary Laws

Extend `…/🧬️schema/🧰️owned/🧪️tests/🔬️retained-mutation-authority/🦀️.rs` with these tests before changing the owner:

1. **Exact field page.** The existing 4,096-byte rename against `nested_snapshot()` applies, retains the source container pointer, and reaches terminal arena return. This presently fails at line 1034.
2. **Field page plus one.** The same 4,097-byte mutation rejects with `drawing-store.mutation-field-capacity`, never binds an overlay, and returns the exact source owner. The existing half of this law remains valid.
3. **Source document limit is independent.** A source whose owned capacity is `D + 1` rejects from `DrawingSnapshotBoundsAuthority` with `drawing-store.preflight-byte-capacity` before candidate work planning; a valid small mutation cannot evade that limit.
4. **Duplicate work is planned.** A duplicate of a maximal admitted group records clone credit equal to the actual clone traversal’s retained capacities and either applies within the derived candidate-work bound or rejects before an overlay/source handoff. No post-admission allocator rejection is allowed.
5. **Arena terms are single-counted.** The plan’s arena bytes equal `DrawingMutationArenaOwner::admitted_totals().1`; changing an overlay-page count, page capacity, container capacity, or duplicate-id capacity changes that exact plan. A rename must not receive a separate duplicate-id or page/container debit.
6. **Pool bound is exact.** Bootstrap admits four owners only when the sum of the four `admitted_totals` fits its derived claim, and every returned/retired slot retains its original backing. This preserves the current cancellation and rollback ownership protections.
7. **All fourteen mutations.** Re-enable the seven currently masked candidate lifecycle cases only after laws 1–6 pass. Each must reach its named cancellation, stale, false-terminal, duplicate, reorder, or phase-rollback point; an aggregate admission rejection is not accepted as evidence.

## Neutral Fixture and Third-Party Route

Add a neutral `drawing.mutation-admission/v1` fixture/schema alongside the existing Drawing mutation fixtures. It should declare the document and field limits, a source/mutation pair, the expected disposition, and the expected logical work classes (`overlay-page`, `clone-subtree`, `fixed-arena`, `none`). It must **not** encode Rust `Vec::capacity` as if JSON had an allocator model.

The native owner test records actual capacity figures from the real preflight/work planner and compares them with the fixture’s declared work classes and disposition. That is the only honest way to check Rust allocation ownership.

For accepted and rejected fixture cases, route the resulting before/after Drawing JSON through the existing registered third-party `serde-json-drawing-carrier-reader` (`serde_json 1`), recorded at `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json:40-73`:

- 4,096-byte rename: its JSON carrier contains the changed layer name;
- 4,097-byte field and `D + 1` source rejections: its JSON carrier remains byte/semantic-equivalent to the source; and
- duplicate: the carrier exposes the added layer and distinct identifier.

`serde_json` can validate the carrier outcome, but cannot observe Rust vector capacity. The capacity and terminal-owner assertions remain native owner laws; claiming a third-party JSON reader proves allocator reservations would be false.

## Confidence

High confidence: the current byte failure is caused by conflating the document decode bound with candidate working ownership; fixed arena terms are counted against the wrong bound and duplicate-id/container/page backing is at least partially counted twice. High confidence: a raw constant increase would leave duplicate subtree allocation unplanned. Medium confidence: the exact code shape of the new work planner, because it must be aligned with concurrent owner changes; the boundary and accounting requirements above are source-derived.
