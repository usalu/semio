# Mounted Retained-Paged Checkpoint Audit — 2026-09-28

## Scope and frozen source

Read-only audit of the current mounted Store retained-clone checkpoint, with focus on child-grant admission, abandonment ownership, retirement scheduling, edit visibility, wasm32 readiness, and readiness for canonical XML/OPC owners.

The reviewed fingerprints are:

- `retained-clone/🦀️.rs`: `026a2cae0a3051307ca9c2d5e8f5540231f1d37f94603c5f57c77530190872c0`
- `retained-clone/📋️paged-list/🦀️.rs`: `42ff6eb61e5302fd536b5f5b4fda2fa9b9c7f948e4b419b8542a24a62ec41a82`
- `retained-clone/🧩preparation/🦀️.rs`: `f2bb3386b261580ce9e9c348f55938a492867c8fe4eb51f2bb79d2ac3adf4f11`
- retained-clone derive: `8ee8ff92e3cf28f91cd61ba6ae248f81fd90d31a5cdc4b616e5b09546aae9bbb`
- retained ordered map: `f17f6f7516036ca0129f12790889712dcc6f709c5bc235bad2bdcc15c024c374`

The `paged_list` module is now deliberately mounted at `retained-clone/🦀️.rs:13-18`. Native 8 remains waiting on Cargo's artifact-directory lock; its log ends at `Blocking waiting for file lock on artifact directory`, with no compiler output or executed law. The old renderer WASM31 borrow diagnostics predate the frozen `42ff…1a82` source and must not be attributed to this checkpoint. A fresh renderer report confirms this paged-list source crossed the wasm32 shared compile boundary; renderer dependencies are still running and no publication or runtime law has completed.

## Confirmed repairs, pending execution

The parent boundary now rejects over-reported clone work across all three dimensions through `admit_retained_clone_progress` (`retained-clone/🦀️.rs:128-144`) and rejects over-reported retirement work through `admit_retained_clone_retirement` (`:146-151`). The mounted paged cursor applies those checks before adding child work or placing an owner (`📋️paged-list/🦀️.rs:151-152,162-191,210-226,257-288`). The vector, option, box, tuple, derive, ordered-map, and preparation routes also call the same admission helpers before accepting child-reported work.

The new paged laws include an over-budget clone child, over-budget retirement child, partial cancellation, and a direct-drop probe (`📋️paged-list/🧪️tests/🔬️unit/🦀️.rs:335-421`). The retirement test now drives the production `owned_retirement` wrapper rather than recursively draining a child in the fixture (`:287-301`). These are appropriate source-level repairs, but Native 8 has not compiled or run them.

The edit seam is now crate-private (`🧩preparation/🦀️.rs:13-57`), including `RetainedCloneEditCursor`, `RetainedCloneEdit`, and the factory. Store error handling starts close on a preparation fault (`🏪️store/🦀️.rs:4074-4085`), so the ordinary publication driver has a close path for reported errors and cancellation.

## Remaining blockers

### P0 before an artifact root may mount: direct abandonment still has no scheduler handoff

`PagedListCursor` protects its entire state with `ManuallyDrop` and fails fast when dropped nonterminal (`📋️paged-list/🦀️.rs:74-111`). That prevents synchronous page/payload destruction, as the new probe asserts. It does not transfer the nonterminal state into Store-owned retirement: normal direct drop panics before dropping it, and unwinding leaves the state inside `ManuallyDrop` without a registered retirement owner. This is a deliberate leak-on-contract-violation, not a bounded cancellation path.

The generic contract still explicitly says dropping an active cursor is outside the low-level ownership contract (`retained-clone/🦀️.rs:167-181`). Other primary cursor roots retain ordinary owner-bearing fields and have no comparable `Drop` or transfer mechanism: `VecCursor` (`:423-433`), `OptionCursor` (`:623-631`), `BoxCursor` (`:764-772`), and generated derived cursors. The new paged wrapper shields those fields only when they are descendants of that exact cursor.

Before any Stdio snapshot mounts this path, make the owning Store driver the sole destruction authority for nonterminal cursors, with an explicit handoff into `owned_retirement` or an equivalent registered owner. Add an end-to-end Store publication law that cancels, faults, and aborts a partially copied multi-page root, then proves that every page and payload destructor runs only through bounded scheduler turns. A direct-drop panic probe alone cannot establish that result.

### P1 before normal large XML/OPC files: there is no artifact migration

Neither DOCX nor XLSX has a `RetainedClone`/`RetireOwned` implementation or uses `RetainedClonePreparationFactory`; repository search found no Stdio use of that factory or edit trait. DOCX still persists `opc: OpcPackage` and `xml_parts: Vec<DocxXmlPart>` (`📜️docx/.../📸️snapshot/🦀️.rs:150-161`) and its actual preparation performs `base.clone()` followed by full in-place mutation in one turn (`📜️docx/.../📬️preparation/🦀️.rs:295-313`). XLSX likewise retains ordinary `Vec<XlsxXmlPart>` and `OpcPackage` (`📕️xlsx/.../📸️snapshot/🦀️.rs:116-131`). The mounted primitive has therefore not replaced the 2 KiB DOCX admission boundary or enabled resumable XML/OPC saves.

`PagedList<T, N>` has a fixed total logical capacity: source values above `N` are rejected by the cursor (`📋️paged-list/🦀️.rs:120-122`) and by the owner capacity API (`🌱️value/📋️list/🦀️.rs:217-257`). A future artifact owner needs declared, tested envelopes for XML part count, XML child count, non-XML parts, content-type rows, and relationship rows. No such canonical XML/OPC owner or migration fixture exists yet.

### P1 for map-backed OPC metadata: ordered-map directory growth is still contiguous

The first-party ordered map's clone cursor admits and reserves its complete page-directory capacity in one turn (`🗺️ordered-map/🦀️.rs:152-164`). For a directory larger than the grant it returns zero progress indefinitely; it does not page directory growth. This does keep a successful turn within its declared capacity grant, but it cannot support arbitrary large relationship/content-type owners under interaction-sized turns. Its `Vec<Vec<(K,V)>>` page directory also remains separate from `PagedList`.

Use a paged directory or another one-allocation-at-a-time owner before it is proposed as the canonical OPC relationship/content-type representation. Cover a directory larger than one turn, a long key, cancellation during directory growth, and bounded retirement of the unpublished directory.

### P2 before crediting bounded edits: private visibility does not make `&mut P` bounded

The seam is safely crate-private but still hands the edit implementation the entire cloned snapshot in one call (`🧩preparation/🦀️.rs:26-33`). Admission validates only the reported `RetainedCloneProgress` after the edit returns (`:282-299`). The type system cannot prevent an implementation from scanning the full root, allocating a full inverse, or mutating many nodes before it reports a small progress value.

Keep the seam unmounted for Stdio until it uses bounded primitive operations, or prove each concrete edit against a multi-page untouched sibling under cancellation, stale, and fault paths. The existing over-budget reporting checks do not prove work performed inside a conforming-but-dishonest implementation.

## Completion status

The source-level child-credit and production-retirement-fixture repairs are present. wasm32 compilation has newly crossed the shared Store boundary for the frozen paged source, while Native 8 remains unexecuted. The checkpoint is not ready to carry a Stdio canonical XML/OPC root: ownership handoff on abandonment, paged map directories, concrete canonical owners, artifact migration, and fresh native/runtime proofs remain required.
