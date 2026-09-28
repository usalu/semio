# Bounded Typed Snapshot Copy Design

## Finding

The current reusable seams do not provide a bounded native snapshot copy.

- `ToValue::value_at_path`, `value_shape_at_path`, and `value_key_at_path` in `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs` project selected wire values but do not construct native owners incrementally.
- `FromValue::edit_value_at_path` mutates an already-owned native value. It cannot create an arbitrary root in pages without first materializing a complete `DslValue` or native snapshot.
- `ArtifactCanonicalJsonCursor` in Store emits canonical JSON incrementally, but the inverse reader hydrates a complete value. Encoding then decoding would add a second full representation, loses native-only fields skipped by value codecs, and does not solve bounded native allocation.
- `RetireOwned` and `artifact_retire_struct!` already decompose native owners incrementally for destruction. They are the closest domain-neutral precedent, but they are one-way.
- DOCX, Mesh, BREP, and ZIP therefore maintain artifact-specific structural copy cursors. The repeated state is field sequencing, string/byte paging, collection child ownership, close/retirement, and output assembly.

## Viable reusable seam

Add a domain-neutral structural-copy contract beside Store retirement, independent of wire serialization:

```rust
pub trait RetainedClone: Send + Sync + Sized + 'static {
    type Cursor: RetainedCloneCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor;
}

pub trait RetainedCloneCursor<T>: Send {
    fn advance(
        &mut self,
        source: &T,
        maximum_items: usize,
        maximum_bytes: usize,
    ) -> Result<RetainedCloneStep, String>;
    fn take(&mut self) -> Option<T>;
    fn begin_close(&mut self);
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize)
        -> InteractiveJobCloseStep;
    fn terminal_is_empty(&self) -> bool;
}
```

The derive expansion should generate a cursor state machine for structs and enums. Base implementations cover scalar `Copy` values, UTF-8 `String`, byte vectors, `Option`, `Box`, tuples, sequences, and ordered maps. Each generated field transition charges at least one item; string and byte owners charge their actual copied byte count. Nested cursors are boxed where recursive types require indirection. Cancellation drains partially built owners through the existing `RetireOwned` contract.

This contract copies native fields rather than their value-codec view. It therefore preserves skipped/internal fields, custom value codecs, enum representation, exact integer widths, and map-native semantics. The existing typed path APIs remain the edit seam: a generic preparation first completes a bounded native copy, then applies a bounded `ValueEdit` or a format callback before sealing.

## Map constraint

A fully generic implementation cannot page `HashMap` in deterministic near-linear time while receiving only a fresh `&HashMap` on every turn: its iterator cannot be retained across calls, and repeated `nth(index)` is quadratic. Long-term artifact schemas should use the first-party ordered-map type, backed by `BTreeMap`, for persisted maps. Its cursor can retain the last copied key and resume with `range((Excluded(last), Unbounded))` in logarithmic time. A temporary `HashMap` implementation would need a strict small-map envelope and must not be advertised as the complete large-document path.

## Preparation integration

After the structural trait exists, replace the Semio-local `StructuralMutationCopy` implementations with one framework `RetainedClonePreparationFactory<S, M>`. It should:

1. retain the authoritative `SnapshotRead<S>`;
2. copy `S` through `S::Cursor` under the Store grant;
3. invoke a bounded typed patch or editor callback on the completed output;
4. derive the inverse from the authoritative source before publication;
5. pass the result to the existing incremental sealer;
6. expose progress and cancel through the cursor and existing retirement ladder.

This removes per-artifact whole-snapshot `Clone` calls while preserving CQRS publication and exact undo/redo. It does not provide structural sharing; one edit remains O(snapshot size), but every turn, allocation, cancellation, and retirement step is bounded. A later persistent owner type can reduce total work without changing the preparation API.

## Proposed execution scope

1. Add schema-first neutral fixtures for nested structs, enums, optional values, UTF-8 split boundaries, byte vectors, ordered maps, cancellation at every phase, and a multi-megabyte untouched sibling.
2. Implement `RetainedClone` base cases and derive expansion, plus an independent test oracle that compares the result with ordinary `Clone` for the neutral corpus.
3. Add `RetainedClonePreparationFactory` and registered host-action laws for progress, cancellation, exact inverse, undo/redo, and terminal-empty retirement.
4. Replace the current DOCX and Semio Mesh/BREP handwritten post-copy cursors as representative migrations.
5. Migrate persisted unordered artifact maps to the ordered map and then mount the remaining media roots.

Do not generalize the current artifact-specific cursors into the framework before the map representation and cancellation fixtures are fixed. A canonical-JSON round trip is not a valid replacement.

## Foundation checkpoint 2026-09-28

The first staged implementation is present beside Store retirement. `RetainedCloneGrant` keeps item, copied-payload byte, and retained-capacity byte credits independent. `RetainedCloneCursor` exposes incremental advance, owner transfer, explicit close, and a terminal-empty witness. Base cursors currently cover scalar values, UTF-8 strings, vectors, options, boxes, and `BTreeMap`. Strings and vectors require their complete logical capacity credit before their first allocation, then copy payloads under separate byte grants. The ordered-map cursor resumes from its last retained key and has no unordered-map fallback.

The derive package now publishes `RetainedClone` and `RetireOwned` derives for named, tuple, and unit structs and enums. Generated state machines delegate each field to its native cursor and route partial fields and completed output through the existing `RetireOwned` ladder when cancelled. A schema-first fixture combines nested records, a tagged enum, an optional field, ordered Unicode keys, and a generated 2 MiB byte payload. Its Rust laws compare the result with ordinary `Clone` and independent `serde_json` serialization, assert that every reported step fits all three grants, assert capacity refusal before the large vector allocation, and close partial owners at several cancellation phases.

The narrow derive Nx check is running as exec session `16587`; its log is `🗑️generated/retained-clone-derive-check-1.log`. It is waiting behind active shared Cargo work and has emitted no compiler diagnostic. No compile or test pass is claimed. The earlier Semio session `93040` and DOCX session `94934` predate this foundation and cannot validate it.

The staged API still has deliberate proof gaps before artifact migration:

- standard arrays and tuple values have no base retained-clone cursor yet;
- recursive derived types need an explicit boxed-cursor strategy and a compile fixture;
- all enum shapes, empty/unit records, generic records, and cancellation between every generated field need compile/runtime laws beyond the current nested variant;
- collection capacity credit is a logical owner-capacity contract; allocator overhead is not represented and needs an explicit invariant in the schema/API;
- no generic Store preparation factory consumes `RetainedClone` yet;
- DOCX and Semio still use their existing artifact cursors until the foundation is independently audited and its focused kernel laws pass.

Foundation file ledger:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🧫️fixtures/📦️nested/{🧬️schema/🔣️.json,🔣️.json}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/{🦀️.rs,♻️retirement/🦀️.rs}`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/{🦀️.rs,🧬️retained-clone/🦀️.rs}`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs`

## Reviewed resource contract and preparation draft

The corrected cursor is one-shot. Its source must remain the same immutable owner at the same address until completion or cancellation. String, vector, optional, boxed, derived, tuple, and ordered-map cursors pin the source owner and the available shape metadata; this detects moved owners and structural growth/shrinkage but does not turn interior mutation into a supported operation. Callers must retain an immutable `SnapshotRead` for the cursor lifetime.

`RetainedCloneGrant` now separates item work, copied payload bytes, retained logical capacity, and structural depth. Box descent decrements the structural depth. A source exactly at the declared envelope is accepted, while the next box is refused before its cursor allocation. This bounds generic recursive control traversal and cancellation stack depth; arbitrary-depth recursive owners are intentionally unsupported. Copy fixed arrays initially use a single admitted envelope equal to the complete inline byte size. They do not claim paged arbitrary-size inline array construction.

Successful child owners are not assembled immediately. Each parent takes the child owner, closes the spent child cursor through bounded retirement steps, and only then advances its field phase. This prevents normal successful copies from dropping a deep cursor scaffold synchronously. Derived assembly and zero-sized unit records consume an item credit, so collections of zero-sized records cannot pass through a single turn. Derived retirement wraps each field in a deferred owner instead of recursively constructing every descendant retirement cursor.

The future Store integration should remain generic over the artifact-owned edit callback while keeping the copy protocol domain-neutral:

```rust
pub trait RetainedCloneEdit<S, M>: Send + Sync + 'static {
    type Cursor: RetainedCloneEditCursor<S, M>;
    fn begin(&self) -> Self::Cursor;
}

pub trait RetainedCloneEditCursor<S, M>: Send {
    fn advance(&mut self, source: &S, copied: &mut S, grant: RetainedCloneGrant)
        -> Result<RetainedCloneEditStep<M>, String>;
    fn begin_close(&mut self);
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize)
        -> Result<SnapshotRetirementStep, String>;
    fn terminal_is_empty(&self) -> bool;
}

pub struct RetainedClonePreparationFactory<S, M, E> {
    edit: E,
    grant: RetainedCloneGrant,
    _owner: PhantomData<(S, M)>,
}
```

The preparation state machine retains the authoritative `SnapshotRead<S>`, advances `S::Cursor`, closes its spent cursor, advances the editor cursor after a complete native owner exists, validates exact forward/inverse publication, and then hands both owners to the existing one-item sealer. The edit cursor keeps target search, inverse construction, and patching subject to the same grants; it cannot hide those operations inside one callback. Cancel first closes the active cursor, then the copied snapshot or mutations, then the retained read. The factory must publish copy bytes, logical capacity, depth, and retirement progress separately rather than converting them into semantic history rows.

This API remains a design draft until the focused foundation laws pass. Ordered-map insertion is still an acceptance blocker: standard `BTreeMap` does not expose its node-allocation or comparison work, so the current logical node charge is not an allocator proof for arbitrary long keys or split propagation. Artifact migration must wait for a first-party ordered map with paged ordinal traversal and explicit node ownership, or another exact bounded construction API. No unordered-map or canonical-JSON fallback is allowed.

## Foundation validation checkpoint 2026-09-28

The schema-first source oracle now passes through the permanent Nx target `@semio-tech/framework-os-kernel:retained-clone-check`. The run validated the draft-2020 schema with Ajv 2020, reconstructed the 2 MiB payload independently, and compared the fixture with `structuredClone`; it completed in 826 ms. The exact log is `🗑️generated/retained-clone-source-check-2.log`. The first source run, retained as `retained-clone-source-check-1.log`, correctly failed because the initial generic Ajv constructor did not include the draft-2020 metaschema; the permanent script now imports the draft-2020 implementation explicitly.

The source review blockers have corresponding neutral Rust laws in the queued focused kernel run: every close result asserts `Complete => terminal_is_empty`; zero grants cannot construct scalar, string, vector, optional, or derived owners; UTF-8 copying rejects a same-owner, same-length boundary shift; vectors reject a source shape change before indexing; repeated completed optional advances cannot overwrite output; unit records consume item credit; Copy arrays require their complete inline byte envelope; tuple and enum shapes retain their native representation; recursion accepts the declared depth boundary, rejects the next box before descent, and cancels a deep partial prefix through one-item/two-byte close grants. The concrete run is exec session `53501`, with log `🗑️generated/retained-clone-kernel-native-1.log`; it remains blocked on the shared Cargo artifact lock and has produced no Rust diagnostic. No native pass is claimed.

The derived cursor excludes concrete field predicates that syntactically contain the deriving type, while retaining predicates for non-recursive field types and generic parameters. This avoids an explicit self-referential where-clause for `Option<Box<RecursiveRecord>>`; the focused native run is the required proof that the associated cursor indirection resolves without a trait-solver cycle. `BoxCursor` owns its nested cursor behind `Box`, so recursive cursor storage has a finite native layout.

Artifact migration remains held. Standard `BTreeMap` construction and comparison still lack an exact allocator/work budget, and logical `try_reserve_exact` capacity accounting does not expose allocator metadata overhead. The foundation currently proves a declared logical retained-capacity envelope, not a platform allocator byte ceiling. A first-party ordered owner remains necessary before arbitrary persisted maps can use this path.

The concrete map replacement is a first-party paged ordered owner rather than an adapter around `BTreeMap`. It stores entries in fixed-capacity pages and exposes ordinal reads. Retained cloning walks source ordinals and fills destination pages in the same order, so it performs no key comparison and allocates at most one declared page payload per capacity grant. The retained-capacity contract counts page payload slots explicitly; platform allocator metadata remains outside that contract and is named as such rather than estimated.

Normal lookup and mutation use a separate resumable `BoundedOrdCursor`. Scalar keys compare in one admitted item. String keys retain left/right offsets and compare under a byte grant. Binary search retains its ordinal interval and comparator state between turns. Insertion reserves one spare page directory slot during retained cloning, allocates a page only after its complete logical payload capacity is admitted, then shifts at most one fixed page per turn. Moving owner tuples charges their native inline byte size without scanning string payloads. Clone, lookup, and insertion therefore report independent progress while codecs continue to expose ordinary map semantics. Persisted artifact schemas should use this ordered owner directly.

The implementation is now under `retained-clone/🗺️ordered-map/`. The old public `RetainedClone for BTreeMap` implementation was removed, so production code cannot accidentally select the comparison/allocation path rejected by the audit. `BTreeMap` remains only as the independent Rust oracle in the ordered-map laws. The schema-first paging fixture declares a 16-entry page envelope, 70 generated entries, found/missing/insert/duplicate cases, and a 64 KiB key compared under seven-byte turns. Ajv 2020 plus the JavaScript ordered-array oracle passes in `🗑️generated/retained-clone-source-check-3.log` (1.6 seconds, resulting entry count 71). Rust clone/BTreeMap/serde, insertion, duplicate-refusal, long-key, and close laws are queued in native session `1408`, log `retained-clone-kernel-native-2.log`; no native pass is claimed.

The source oracle is registered as `⚖️retained-clone-check💻️os🦀️` in `.vscode/launch.json`, ordered with the other OS-kernel gates. The permanent command remains `bun nx run @semio-tech/framework-os-kernel:retained-clone-check`; no auxiliary script was added.

The ordered-map cancellation law now stops after an actual fixed-page shift, drains the cursor-owned candidate through one-item/seven-byte close grants, then retires the unpublished partially shifted workspace through the same bounded envelope. This makes the cancellation contract executable: shifted work is never published or rolled back in place; its exclusive workspace is discarded and retired. The law was authored while native session `1408` was still waiting on the artifact lock, so it is included in that compilation. The fourth source/oracle run passed in `🗑️generated/retained-clone-source-check-4.log` in 1.5 seconds with `orderedMap=71`.

## Ordered-map audit handoff

Native session `1408` remains alive and blocked on the shared Cargo artifact-directory lock. Its exact log is `🗑️generated/retained-clone-kernel-native-2.log`; it has emitted no Rust diagnostic and no native result is claimed. The command is the focused `@semio-tech/framework-os-kernel:test` target filtered to `retained_clone`. Keep this process alive and inspect its result before changing the foundation further.

The complete current foundation ledger is:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🧫️fixtures/📦️nested/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🧫️fixtures/📦️nested/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🗺️ordered-map/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🗺️ordered-map/🧫️fixtures/📦️paging/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🗺️ordered-map/🧫️fixtures/📦️paging/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🗺️ordered-map/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts`
- `.vscode/launch.json`

The DOCX registered-route expectation repair is in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs`; root owns its next complete suite run. No artifact uses the new retained ordered owner yet. Store preparation and artifact migration remain held for native proof and independent audit.

## Foundation audit repair checkpoint 2026-09-28

The follow-up repair makes source authority an owned API value rather than a caller promise. `RetainedCloneSource<T>` retains either a production `SnapshotRead<T>` or a test owner, and `RetainedCloneRef` carries that lease plus an exact projected identity. Projection uses a higher-ranked closure whose returned borrow is tied to its retained input. The former arbitrary-reference `project_ref` entry point has been removed. Derived structs, derived enums, vectors, tuples, and ordered-map entries all project through the retained root. The native sibling-projection law refuses crossing from one equal-valued field to another, while the language-neutral immutable-lease fixture changes copied and uncopied bytes outside the retained owner and requires the captured value.

`begin_close` is now an idempotent constant-time state flip. Parent `close_step` implementations introduce at most one child frontier per admitted turn, and `RetainedCloneClose` owns only one active retirement cursor. The recursive cancellation law advances a 64-level partial prefix, starts close without descending through it, and requires more than 64 bounded close turns before `Complete => terminal_is_empty`. Empty zero-sized vectors use Rust's sentinel capacity, so their close witness now treats empty ZST storage as terminal instead of repeatedly retiring the same allocation-free sentinel.

The first-party ordered map relocates a full page directory through explicit allocate, take-header, one-page-header-per-turn, and install phases. Repeated fixture growth begins at 32 entries, performs 17 inserts through more than one page-boundary event, and compares all 49 entries with a `BTreeMap` oracle. Insertion progress has disjoint comparison, moved-owner/inline-byte, and retained-capacity channels. Page shifts include the inserted owner tuple in both item and inline-byte movement; pure capacity allocation no longer reports a moved owner. A 64 KiB common-prefix insertion law asserts comparison-only turns report no movement/capacity and allocation turns report no comparison/moved bytes.

Focused native run 4 compiled the generic struct and enum derives without explicit fixture bounds, proving the generated helper cursor now carries its derived `where` clause. It then exposed four fixture/runtime defects: an explicit `fixedArray` decode mismatch, non-terminating ZST-vector close, an under-admitted optional-string test, and the resulting long-running test process. The process was terminated after the exact diagnostics were captured. All four source defects are repaired, and focused native run 5 is session `84936`, log `🗑️generated/retained-clone-kernel-native-5.log`. It is currently waiting behind shared work; no native pass is claimed.

The independent Ajv 2020, `structuredClone`, ordered-array, and JavaScript `Map` oracle remains green after the audit repair: `🗑️generated/retained-clone-source-check-7.log` records 980 ms, `orderedMap=71`, and `growth=49`. Its schema now declares the comparison-only, capacity-only, and minimum moved-owner progress invariants used by the native long-key insertion law. Artifact migration remains held until native run 5 proves the lease, close, recursion, repeated-growth, and separated-progress laws together.

The production-source evidence gap from the follow-up audit now has a focused law. It obtains the same opaque `SnapshotRead<String>` used by Store through its exact lease registry, constructs `RetainedCloneSource::from_snapshot_read`, completes a one-byte-per-turn UTF-8 copy, and separately drops the public source during a partial copy. The active cursor binding keeps the read lease live until bounded cancellation reaches terminal-empty; only then may the registry recover its exact slot. The fixture's immutable-lease values remain the language-neutral input for this law.

The low-level cursor contract now states that its owner must retain the cursor through `begin_close` and bounded `close_step` calls until `terminal_is_empty`. Raw Rust callers dropping an active cursor are outside that contract. Production preparation must encode this ownership structurally: one preparation owner retains the source lease, cursor, partial output, and inverse work through success, rejection, cancellation, error, or supersession; each terminal route enters one bounded close state machine before the preparation itself can retire. A generic preparation mount remains blocked until native run 5 proves the foundation and must include Store laws for all five terminal routes before any artifact migration.

## Production preparation lifecycle checkpoint 2026-09-28

The generic production owner is now implemented under `retained-clone/🧩preparation`. `RetainedClonePreparationFactory<P, M, E>` implements the existing `ArtifactStoreOneItemPreparationFactory` contract. It receives the Store-owned `SnapshotRead<P>` only after preflight, converts it into the immutable `RetainedCloneSource<P>`, retains the clone cursor, copied owner, typed edit cursor, inverse mutations, forward mutation, live authority, canonical sealer, and every retirement factory in one preparation owner, and exposes no raw-owner escape.

The domain seam is deliberately cursor-shaped:

- `RetainedCloneEdit::preflight` declares semantic edit rows and total retained logical capacity before owner transfer.
- `RetainedCloneEdit::begin` creates an empty cursor.
- `RetainedCloneEditCursor::advance` receives the retained base projection, the exclusive copied owner, the forward mutation, and the same item/copy/capacity/depth grant.
- `take_inverse` transfers exact inverse rows only after the domain cursor reports `Complete`.
- cancellation and retirement are explicit through `cancel`, constant-time `begin_close`, one-frontier `close_step`, and `terminal_is_empty`.

The preparation advances clone, spent-clone close, typed edit, spent-edit close, fixed owner assembly, and the existing canonical Store sealer as separate phases. Copied bytes and newly retained logical capacity must fit the current grant and the cumulative admitted capacity. The Store's `work_items` remains the semantic forward-plus-inverse row census; physical copy and retirement progress stays in the retained checkpoint and is not reinterpreted as history rows.

The language-neutral lifecycle fixture declares five outcomes: successful publication, preflight rejection, mid-copy cancellation, stale supersession, and an injected edit fault. Its Ajv 2020 schema and independent TypeScript state oracle are green in `🗑️generated/retained-clone-source-check-9.log`: the permanent Nx target also revalidated the 2 MiB payload, immutable lease, recursive depth, ordered-map paging, repeated directory growth, and separated comparison/move/capacity channels.

The native Store law mounts this factory on the real `ArtifactStore<DemoSnapshot, DemoMutation>`. It drives the production `SnapshotRead` lease through success, cancel, stale, and fault paths; verifies no rejected/cancelled/faulted history mutation; checks the superseding edit is the only stale-case history row; checks successful undo/redo; and closes each publication under a one-item grant while asserting every close turn releases at most one frontier and never exceeds its byte grant. The test snapshot now derives both `RetainedClone` and `RetireOwned`, so the same generated owner machinery is used by the Store law.

Focused native run 5 ended before assertions after 33 minutes 25 seconds on two stale-checkpoint compile errors: the preparation fixture include crossed one directory too far and the isolated production lease law named the facade instead of its component ancestor. Both are repaired. The missing authority method exposed by downstream WAV compilation was also corrected from the nonexistent `stamped_mutation_id` to the Store's canonical `stamped_edit_id`. Focused native run 6 is session `13046`, log `🗑️generated/retained-clone-kernel-native-6.log`; no native pass is claimed until it completes.

New production lifecycle files and mounts:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🧩preparation/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🧩preparation/🧪️fixtures/📦️lifecycle/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🧩preparation/🧪️fixtures/📦️lifecycle/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🧩preparation/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts`

Artifact migration remains held until native run 6 compiles and executes the complete foundation plus production lifecycle law.

### Contiguous allocation boundary

The generic production preparation has two separate byte budgets. `ArtifactStoreOneItemFootprint::retained_bytes` admits the operation's total live retained capacity and is capped by the Store at 1 MiB. Each `ArtifactStoreOneItemGrant::maximum_bytes` bounds one turn's copying and allocation. The preparation passes that turn grant to `RetainedCloneGrant::maximum_copy_bytes` and `maximum_capacity_bytes`; it does not accumulate unused turn bytes as an allocation credit.

This distinction creates a deliberate capability boundary for Rust's contiguous `String` and `Vec<T>` owners. Their cursor must allocate the complete backing capacity before it can copy elements. An 8 KiB `String` under the neutral 4 KiB Store grant therefore cannot make safe progress: repeated `reserve` calls could move and recopy the entire previously built allocation within one supposedly bounded turn. The preparation now refuses a nonzero-grant clone, edit, or fixed-assembly turn that reports zero progress with the stable code `retained-clone.step-grant-too-small`. It does not remain `Blocked` forever and does not publish an edit.

The language-neutral lifecycle fixture declares `largeCapacity.stringByteLength = 8192`, the 4096-byte grant, and the refusal code. Its independent Ajv/TypeScript oracle asserts the capacity relation and stable code. The native Store law uses a separate typed `RetainedTextSnapshot { text: String }` with generated `RetainedClone` and `RetireOwned`, real Store snapshot/mutation owners, and the generic production preparation. It requires the first preparation turn to refuse with that code, preserves all 8192 source bytes, leaves history empty, and drains the failed publication and Store through bounded one-frontier close turns.

This foundation is suitable only when each contiguous native allocation fits one turn and the complete retained owner graph fits the operation footprint. General large artifact migration requires schema-owned paged/chunked containers whose pages each fit a grant, such as the first-party retained ordered map, or a host grant explicitly large enough for the largest contiguous allocation. Raising caps, accumulating fictional allocation credits, or repeatedly growing a native `String`/`Vec` would invalidate the bounded-work proof. Source validation for this addition is green in `🗑️generated/retained-clone-source-check-10.log`: Ajv 2020, `structuredClone`, the JavaScript ordered-map oracle, and the capacity/refusal oracle completed in 820 ms. Native run 6 remains queued at this checkpoint, so the real Store law is authored but not yet claimed passing.

The next paged-owner design review must start from the existing first-party `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs` `PagedList`, which already separates fixed-fanout metadata and payload page admission and exposes explicit retirement. A second generic page owner is unwarranted unless that implementation fails a concrete typed-snapshot requirement.

## Native run 6 diagnostic repair

Focused native run 6 ended after 28 minutes 22 seconds before executing any test. Its log is `🗑️generated/retained-clone-kernel-native-6.log`. The 25 diagnostics reduced to four source causes in the newly added Store lifecycle fixture: test-local mutation derives were rejected by the repository's source-authority check, `DemoMutation` lacked the borrowed canonical JSON view required by the new preparation sealer, the retained text snapshot lacked the no-child `ArtifactCompositionFields` witness required by Store, and the generic publication-close helper lacked Store's `Send + Sync + 'static` snapshot bound. The remaining missing Store methods were cascades from those trait failures.

The fixture now implements its one mutation aggregate directly through `Mutation<RetainedTextSnapshot>`, preserves the existing typed value/text/binary codecs, supplies borrowed canonical JSON without serializing, supplies the empty composition visitor, and constrains the close helper to Store's snapshot bounds. `DemoMutation` now has an exact borrowed canonical JSON object for all four fixture variants. The complete Store unit source parses with `rustfmt --edition 2024 --emit stdout`; no replacement Cargo run has been launched because coordinated artifact builds are active. Native run 7 must compile and execute the lifecycle, 8 KiB capacity-refusal, lease, cancellation, recursion, and ordered-map laws before artifact migration can use this foundation.

## Canonical DOCX retained route checkpoint — 2026-09-28

The base DOCX editor now binds `set-page` to the canonical nested address `{partPath,nodePath,expectedName,revision}` and routes `SetRunText`/`ReplaceXmlNode` through an artifact-owned Store preparation factory. The work stage validates the immutable canonical snapshot and optimistic address before copying draft text, so a no-op emits no history row and stale/refused drafts do not mutate the document. Preparation clones and mutates the canonical `{schema,opc,xmlParts}` snapshot, seals the compact target XML inverse, returns its `SnapshotRead`, and retains every owner through cancellation/close.

This checkpoint deliberately admits only a fixed envelope: 2,048 measured dynamic owner bytes, 128 structural items, depth 32, with one 4,096-byte clone/drop turn. It rejects DOCTYPE/prolog/epilog owners and larger snapshots with `stdio-docx-base-set-page.paged-owner-required` or `...paged-xml-envelope-required`. This is an explicit safety boundary, not complete large-document support. The next required slice is migration of canonical XML/OPC collection owners to the first-party paged/persistent list/map owners so normal DOCX files can progress over multiple grants without a contiguous whole-root clone.

The meter counts OPC part payloads, content-type tables, relationship table capacity and strings, XML part allocations, element children/attribute allocations, strings, node depth, and mutation/address payload. Address-revision validation is safe within this fixed envelope even though it scans direct siblings along the path. Post-state and inverse are measured again before sealing.

Language-neutral admission assets:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/📬️preparation/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/📬️preparation/🧫️fixtures/🧵️admission/🔣️.json`

Touched source at this checkpoint:

- shared `DocumentWindowKit` typed static arguments and its schema/fixture/law
- base DOCX main canonical projection/address binding
- base DOCX editor command/parser/work/preparation mount
- replacement canonical preparation module and compact canonical JSON encoder
- base editor and main-window unit laws
- removal of the orphaned semantic `DocxBlockCopy` module

Validation performed: `rustfmt --edition 2021` completed for the owned base editor, preparation, main window, and unit-test sources; a stale-field scan found no old `SetRunText {path,run_index}`, `project_document`, `DocxPostCopy`, or `DocxBlockCopy` references in the mounted base editor. No native compile or runtime success is claimed. Root-owned retained-clone native7 remains the active compilation lane; DOCX/full component validation is intentionally held until this source checkpoint and the strict/transitional sibling mounts are coherent.

## Native run 7 runtime repair checkpoint

Focused native run 7 compiled and executed the foundation, then reported 16 passing and four failing laws in `🗑️generated/retained-clone-kernel-native-7.log`. The failures were concrete runtime-contract defects rather than compiler fallout:

- The production clone grant discarded the Store's declared item budget with `maximum_items.min(1)`. A 2 MiB `Vec<u8>` therefore required multiple Store turns for every byte and exceeded the neutral 10,000-turn envelope. Clone and typed-edit work now receive the complete admitted item grant; close still introduces at most one retirement frontier per turn.
- Wrapper cursors could take a completed child, begin its close, or advance past a terminal child without charging their own structural transition. Vectors, options, boxes, tuples, and generated structs/enums now reserve and report one item for these transitions. The vector cursor uses one grant across repeated child-copy, scaffold-close, push, and final-assembly phases; string and vector completion each charge their final assembly transition.
- `RetainedCloneSource::from_snapshot_read` retained the immutable owner only outside the lease. When the public source was dropped during cancellation, the cursor binding kept the lease alive but not a distinct root alias; returning the read therefore entered the Store's displaced-owner queue. The lease owner now retains a separate type-erased source alias after its authority field. The `SnapshotRead` authority drops while that alias is live, so the exact registry slot releases in constant time, and the alias continues pinning every projected address until the cursor binding closes.
- The 8 KiB contiguous-capacity law assumed the first outer Store turn had already entered preparation. It now advances through the Store's bounded staging turns and requires the stable refusal within eight turns, while preserving the snapshot and empty history.

All four affected native laws already existed before the implementation repair and remain the acceptance gate: the 2 MiB Clone/serde oracle, production SnapshotRead copy/cancel registry law, real Store lifecycle/undo/redo law, and real Store 8 KiB refusal/close law. The repaired source is formatted. No native pass is claimed until root launches the coordinated focused run 8.

### Native run 8 coherent-source fingerprint

The source checkpoint consumed by the coordinated focused run 8 was frozen at `2026-09-28T02:08:46Z`. Its SHA-256 identities are:

```text
8eb94040b63d533e2e967676818990636a4cd21bcc428e1d2a2ea397182ff5e9  retained-clone/🦀️.rs
a051fc8805821b527c7cdead0b44d40d2e975a6bdc46331dac7b9cbab25744f1  retained-clone/🧩preparation/🦀️.rs
c2f2d7ff935b68b6e9db42134abfd1ef21974643d071d3793021a5d90d98b500  derive/🧬lretained-clone/🦀️.rs
6683d961117cc38746a829513f0cb8873a8961e46623ed6eb0743d1d428e8a69  retained-clone unit law
```

This checkpoint supersedes the preceding ownership-order wording. `RetainedCloneSource` declares its direct `owner` before the lease, and `RetainedCloneLeaseOwner` declares `_source_alias` before `_authority`. Rust therefore drops the public/cursor source alias first, the lease-owned alias next, and the `SnapshotRead` authority last. If another Store root remains, returning the read releases the registry slot in constant time. If no other root remains, the read returns its final owner to the registry and the registered bounded retirement pump drains it. No type-erased alias can become the hidden final owner after the read has returned.

The strengthened cancellation law covers that second case with the full 2 MiB neutral snapshot: it moves the only source owner into `SnapshotRead`, drops the public retained source during a partial clone, closes the cursor one frontier at a time, requires a returned registry owner, and then drains that owner through `OwnedValueRetirementFactory` while enforcing each turn's item and byte limits. Native run 8 is the first build intended to execute this exact fingerprint; its log is `🗑️generated/retained-clone-kernel-native-8.log`, and it was still blocked on the shared artifact directory when this checkpoint was recorded.

### Canonical DOCX retained-route source checkpoint

The base, strict, and transitional editor roots now share one early-admission and retained-work implementation. Each `set-page` parser decodes the canonical `DocxXmlAddress`, `build_set_page_mutation` calls `prepare_set_run_text`, and `canonical_docx_set_page_work!` retains and pages the submitted text before the common canonical preparation route runs. Strict and transitional roots mount the same preparation route rather than falling through to generic synchronous preparation. Runtime proof remains pending in the coordinated DOCX/full-component lanes.

The preparation route recognizes every source-frozen canonical addressed XML mutation with the exact wire spellings: `setRunText`, `replaceXmlNode`, `setRunFormatting`, `setParagraphStyle`, `insertTableRow`, `removeTableRow`, `insertXmlNode`, and `removeXmlNode`. A schema-first admission fixture declares the ordered eight-variant census, and the Ajv 2020 oracle is green in `🗑️generated/docx-preparation-ajv.log`. The Rust law constructs and canonically encodes every variant, but no native pass is claimed yet.

This route still has an explicit interim capability boundary: the canonical owner must fit 2,048 admitted bytes, 128 owner items, and depth 32. That refusal is safe and preserves history, but it is not a complete normal-document editing solution. The next model foundation must replace contiguous XML/OPC collections and text payloads with retained paged owners before raising those limits.

### Existing paged-owner reuse assessment

The first-party `value::list::PagedList<T, N>` already provides the required allocation mechanics for a retained canonical owner: fixed 16-way metadata fanout, at most 4 KiB payload pages, separately admitted metadata and payload reservations, owner placement without reallocation, ordinal reads and mutation, and explicit payload-before-page retirement. It is already used in FEM, Pack, UI, and Flow retained paths. Its cold `Clone` implementation is explicitly unsuitable for Store preparation because it deep-copies synchronously.

The minimal reusable foundation is therefore a `RetainedClone` cursor and a `RetireOwned` cursor for `PagedList<T, N>`, backed by its existing `next_capacity_allocation_bytes`, `reserve_capacity_one`, `place_reserved`, `truncate_retired_last`, `next_release_allocation_bytes`, and `release_empty_page` operations. A neutral law must copy and cancel a multi-page list under sub-page grants, compare the result to an ordinary `Vec`/serde oracle, and prove `Complete => terminal_is_empty` for retirement. This foundation cannot by itself make DOCX complete because `DocxXmlPart`, `XmlNode`, OPC names/content, and text still contain contiguous `String`/`Vec` owners. Those fields need schema-owned paged text/bytes and paged child/list/map owners, with lossless codecs at the external boundary. No artifact model has been migrated at this checkpoint.

### Paged-list retained primitive source checkpoint

The page-level schema and implementation are authored under `retained-clone/📋️paged-list/` without mounting the Rust module into the active foundation yet. Keeping it unmounted preserves native run 8's exact fingerprint while that run waits for the shared artifact lock.

The new `PagedListCursor<T, N>` uses the list's existing one-allocation reservation API, clones one projected ordinal through `T::Cursor`, closes the child scaffold, and moves the completed native owner into an already-reserved slot. Capacity allocation, child copy, inline owner placement, and final assembly remain separate progress. Its cancellation path keeps the immutable source binding until every child, completed child value, output, and partial paged workspace reaches terminal-empty.

`PagedListRetirement<T, N>` pops one initialized owner into the ordinary typed retirement stack, then releases one empty payload or metadata backing per admitted byte grant. It never invokes the cold `Clone` implementation and does not recursively destroy an active list. The neutral 513-entry Unicode fixture crosses multiple `String` payload pages, cancels after a partial prefix, and requires ordered Vec/serde equality, source preservation, multi-turn copy, multi-turn close, and terminal-empty retirement.

The registered `bun nx run @semio-tech/framework-os-kernel:retained-clone-check` source/oracle target is green in `🗑️generated/retained-clone-source-check-11.log`: Ajv 2020 validated the schema and the independent platform `structuredClone` array retained all 513 ordered values. No native pass is claimed for the unmounted Rust primitive. The source fingerprints are:

```text
02c501cbb576960f65dc4966b49d569bd38ad4952f7b0c3e203474fad2ffe53f  paged-list/🦀️.rs
f87ef42e1eb388385d6b7115025f24e34e2699431bfa64b6ad6b728d16fd7468  paged-list unit law
60d0848ce8715ea98feb76087e90e698ea5ea3b49c1a9bf8ccee294aea9e6056  paged-list fixture
db6bcf88f99783018562adefc784d2c14a1b121a064d8d3e21e87cb1465c4d31  paged-list schema
```

The canonical XML/OPC model migration must build four schema-owned primitives on this base: paged UTF-8 text with UTF-8 boundary-preserving mutation, paged opaque bytes, paged ordered sequences, and a domain-neutral paged ordered map. External codecs may materialize ordinary `String`/`Vec` only at already-retained I/O boundaries; Store preparation and in-place canonical mutation must stay on paged owners. `DocxSnapshot.xml_parts`, XML node children/attributes/text, OPC parts/content bytes/content-type tables/relationships/comment, and their address paths must migrate together so no contiguous sibling silently restores aggregate clone cost. The current OS-local `RetainedOrderedMap` is not a persisted value type and cannot become the artifact schema authority; its allocation/cursor laws can be moved beside the domain-neutral owner after the list primitive compiles.

## Mounted paged-owner audit repair checkpoint — 2026-09-28

The paged list retained primitive is now mounted in the Store retained-clone module. Every owner-bearing `PagedListCursor` field lives inside one `ManuallyDrop<PagedListCursorState<...>>`. A terminal cursor drops that state normally. Abandoning an active cursor fails fast and retains the state instead of synchronously destroying partial pages or payloads; the production owner remains responsible for `begin_close` and bounded `close_step` until terminal-empty.

Child work is admitted centrally before parent progress changes. `admit_retained_clone_progress` checks item, copy-byte, and capacity-byte residuals. `admit_retained_clone_retirement` checks item and byte residuals. The checks cover Vec, Option, Box, tuples, generated records/enums, ordered maps, paged lists, and the production preparation clone/edit/close phases. `RetainedCloneClose` also validates the erased owned-retirement result at its own boundary. The generic edit traits and factory are crate-private because unrestricted external `&mut P` edit implementations cannot prove bounded work; no artifact can mount that generic seam as a production capability.

The neutral preparation corpus now includes `overBudget`, in which a dishonest edit reports above all three grants. The real Store law requires refusal before publication, unchanged snapshot, empty history, and terminal-empty close. Paged native laws include dishonest clone progress, dishonest child retirement, and a destructor probe. The destructor probe abandons a multi-page partial cursor, requires the fail-fast panic, and proves that no retained payload destructor ran. The 513-entry retirement law uses the production `owned_retirement` scheduler and enforces the item and byte limit on every turn.

The registered source/oracle target is green in `🗑️generated/retained-clone-source-check-12.log`: Ajv 2020 validated six lifecycle rows, and the independent JavaScript oracles reported `payload=2097152`, `orderedMap=71`, `growth=49`, and `paged=513`. This is source/schema/oracle evidence, not Rust runtime evidence.

The first downstream compilation of the mounted module, renderer WASM 31, reached the new source and found eight borrow-splitting diagnostics caused by field access through the cursor's `Deref` wrapper. The cursor now explicitly projects its inner state before borrowing child/value/output fields. No downstream retry has yet compiled that repair.

The frozen source identities after that repair are:

```text
026a2cae0a3051307ca9c2d5e8f5540231f1d37f94603c5f57c77530190872c0  retained-clone/🦀️.rs
f2bb3386b261580ce9e9c348f55938a492867c8fe4eb51f2bb79d2ac3adf4f11  retained-clone/🧩preparation/🦀️.rs
8ee8ff92e3cf28f91cd61ba6ae248f81fd90d31a5cdc4b616e5b09546aae9bbb  derive/🧬retained-clone/🦀️.rs
42ff6eb61e5302fd536b5f5b4fda2fa9b9c7f948e4b419b8542a24a62ec41a82  retained-clone/📋️paged-list/🦀️.rs
d86321a9b2ccfb4544b99fb23fd4695be2a453a3750b6e310e249c7a60af46e7  retained-clone/📋️paged-list unit laws
f04ac442da5c15429c8676779af44e99319fbc4fd4c78099f525f599a075629d  Store retained-clone lifecycle laws
```

Focused native run 8 remains PID `33698`, command `cargo test --manifest-path Cargo.toml --lib retained_clone -- --nocapture`, with log `🗑️generated/retained-clone-kernel-native-8.log`. At this checkpoint it still reports only `Blocking waiting for file lock on artifact directory`; it has not compiled or executed the mounted paged laws. No native pass or artifact migration is claimed.
