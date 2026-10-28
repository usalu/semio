# Retained Original Native Pointer Checkpoint

## Source Boundary

The mounted shared IFC details worker now finishes its original event payload/Store canonical-wire admission, then drives `pointer.rs::PathAdmission` under the initiating `StepContext`, then captures the snapshot. It no longer invokes the synchronous `snapshot_edit_retained_path_is_admitted` hook at that boundary. Original event, snapshot, path and Move-source pointer/length identities are checked from actual borrowed owners. Rebinding is refused. Controlled close clears those pins before the original event or captured snapshot owners retire; existing wire/admission/publication close ordering is preserved.

The cursor keeps 128 inline source byte ranges and native child ordinals per pointer, not decoded strings, temporary path vectors, a graph DTO or encoded body. It scans at most 64 raw bytes per turn and publishes at most one segment range per turn. Key and Move-ancestor comparisons inspect at most 64 decoded bytes per turn directly from the original UTF-8/`~0`/`~1` source and native key owner. Native ordinal reconstruction is bounded by the explicit 128 segment extent. No text normalization, identity conversion, collection truncation or graph limit was introduced. Set retains its existing 8,192-byte extent and other events retain the declared raw extent.

Ordinary native addressing supports all actual header/entity/complex/value fields and arrays, canonical numeric array indices, insertion at length and `-`, existing source lookup, Rename object-parent ownership, nonempty Move endpoints and decoded strict-descendant refusal. Move destination admission deliberately preserves the prior structural contract: it need not exist yet. Native collection/whole-entity reduction remains responsible for actual destination-owner/bounds validation. This is not a claim that RFC6902 accepts a missing destination parent.

Empty omitted `complex` insertion and virtual `reference`/`appendReference` fields have explicit native structural cases. Reference declaration, optionality, uniqueness, cardinality and target checks still run in the existing later normalization/domain path. Structural acceptance is not reference-domain acceptance. The IFC4 editor opts into a narrow optional `snapshot_edit_semantic_path_is_admitted` hook for its existing `/ifc-<id>/<field>` inspector contract; ordinary native paths never fall back to the allocating generic pointer helper. IFC4x3 retains the default absent semantic hook. `IfcHeader` and `IfcSnapshot` add only the same existing CanonicalJsonTree derive already used by their native children.

## Independent Review Correction

The independent auditor found that the first Scan loop could publish several ranges while quoting one Range. Scan now ends immediately after one slash/range publication; end-of-input likewise publishes at most one range. The language-neutral scan grant law independently states two range words, three scalar words, one boolean byte and one explicit `repr(u8)` phase byte. The authored native separator-heavy test denies this independently computed byte count by one, checks no state/custody/receipt/heap change, then grants exactly that count and checks one range maximum. This test has not executed. Compared input/key bytes remain borrowed reads; no compared payload text is copied or allocated.

## Tests And Exact Evidence

Authored before the production cursor: `details/🧪️tests/pointer.json` and `details/pointer_tests.rs`. The language-neutral fixture has 23 generic pointer cases, six generic Move cases, six native Move cases and eight IFC structural cases. Native tests also author every-prefix cancellation, original owner addresses, six grant/fuel axes, real heap witnesses, escaped UTF-8/one-pass escapes, canonical array indices/overflow, exact `u64::MAX` preservation, 128/129 separator boundaries, semantic hook selection, rebound event/snapshot refusal and canceled controlled-close denial/funding. The existing real-worker every-admission-prefix test now includes the retained address frontier before source capture. Native tests remain source only.

Executed command: `bun <ticket>/pointer-oracle.ts`. Existing `fast-json-patch` 3.1.1, behind a test-only interface with explicit RFC6901/canonical-array grammar, checked all 23 generic pointer and six Move structural cases. Its independently applied output replaces the escaped Unicode path, appends the fourth array item, leaves the source graph unchanged and retains `18446744073709551615` exactly as its original string. Output: `{"oracle":"fast-json-patch 3.1.1 with explicit RFC6901/array grammar","paths":23,"moves":6,"utf8EscapedReplacement":true,"appendOutput":true,"exactU64Unchanged":true,"nativeCursorExecution":false}`.

Rustfmt parsed the new pointer source and tests. The actual focused command `cargo test --manifest-path <IFC4x3>/📦️packages/🦀️rust/Cargo.toml --lib original_pointer -- --nocapture` refused at authentic compiler ingress's `-vV` probe: `Original compiler ingress requires the repository launcher and authentic enforcing store`. No rustc type checking, native test, original scheduler/heap/receipt/cancellation proof or cleanup execution occurred. No bypass was attempted. `nativeExecution` remains false.

## Remaining Work

Post-capture reference normalization still allocates decoded pointer/detail vectors and constructs normalized events synchronously. Current work routing also creates the native-value prefix with `format!` and reparses entity indices; these are reduction-frontier allocations, not part of the new path admission claim. Action-to-command conversion can still clone/parse eagerly before the retained raw-wire hook. Owned entity/constituent retyping, default arguments, derived normalization, GlobalIds, collections and mutation assembly remain synchronous in `owned_entity_edit` and semantic emission. They require retained original-owner reduction/action receivers, not fuel ticks around existing helpers. The canonical Store physical depth contract remains separate from the DSL/pointer extent.

## Exact Files Changed

- Shared IFC details: `pointer.rs`, `pointer_tests.rs`, `🧪️tests/pointer.json`, `work.rs`, `admission.rs`, `🦀️.rs`.
- Shared IFC4 native model: `🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` (two derives/owner attributes only).
- Shared editing contract: `📇️registry/🧬️contract/✏️editing/🧰️editor-traits-and-retained-copies/🦀️.rs` (optional semantic address hook only).
- Actual IFC4 editor: `🏅️standards/4️⃣4/🪆️subsets/✳️any/✏️editor/🦀️.rs` (semantic hook mount only).
- Ticket: `pointer-oracle.ts`, `host-pointer-plan.md`, this report and the shared coordination append.

No AGENTS files, Git state, lifecycle, goals, shared dependencies, TS exchange source or geometry files were changed. No persistent generated output was created by this worker.
