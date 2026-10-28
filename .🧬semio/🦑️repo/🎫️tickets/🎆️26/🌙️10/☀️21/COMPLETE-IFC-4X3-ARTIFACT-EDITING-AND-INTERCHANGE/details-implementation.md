# IFC4x3 Details Implementation

## Implemented Surface

The new `semio-s-artifact-stdio-ifc-4x3` Rust domain package mounts the retained catalog, an IFC-specific `SnapshotDetailsProvider`, and `Ifc4x3AnyEditor`. The editor uses dialect `s.stdio.ifc@4x3/*`, controller `s.stdio.ifc@4x3/*#editor` and host document envelope `s.stdio.ifc@4x3/*`. Its snapshot remains the directly shared `IfcSnapshot` with graph identity `stdio.ifc`, matching TypeScript interchange. The initial FILE_SCHEMA is `IFC4X3_ADD2`.

The real editor render branch calls `render_snapshot_details_provider_revisioned`. It preserves the framework's revisioned drafts, explicit Apply/Discard, tree windows, accessible stock controls, retained command jobs, progress/cancellation interface and central mutation publication authority. It renders a details window without the IFC4 editor's placeholder box geometry. English/German locale is supplied explicitly by the host.

The provider names inherited positional attributes using the complete catalog and hides argument transport discriminants. It provides schema enum members, recursively flattened SELECT alternatives with typed wrappers, optional unset and derived presentations, nested scalar/typed/aggregate editing, literal aggregate bounds, fixed ARRAY cardinality, LIST/ARRAY reordering, SET uniqueness-aware insertion templates, header scalar/list editing, and additional complex constituent insertion/removal/reordering/nested edits. The complex constituent attribute projection uses direct attributes, because complex constituents serialize partial entity records rather than flattened inherited records.

Entity type choices use the existing indexed variant tree window and preserve shared attribute values by declared name. Type switching emits existing concrete remove/insert entity mutations with the same identity, in one host transaction. Ordinary nested edits resolve to existing `SetEntityArg`; complex edits resolve to the shared `SetEntityComplex` vocabulary contributed concurrently by the external fleet. No independent state store, patch application algorithm, CRDT or history mechanism was introduced.

Reference suggestions inspect at most the first 256 model entities and filter admissible EXPRESS targets. Every reference slot, including unset slots and entity-valued SELECTs, also provides an exact decimal identity input. This reaches every target beyond the suggestion window and preserves values above 2^53. The IFC-owned command converts decimal strings to positive u64, checks target existence/subtype and optional unset, and emits the ordinary argument/complex mutation. Whole-model structural analysis is intentionally separate so incomplete imported models remain repairable and small edits do not clone/validate the entire model.

Entity creation allocates identity in the retained command, not during render: max+1 when available, otherwise the first positive gap. This handles unsorted entities and u64::MAX. GlobalId encoding is owned 128-bit IFC compression using the standard alphabet. Newly required GlobalIds use std's randomized hash state plus instant/entity context, UUID version/variant bits and a collision guard against existing GlobalIds. No external runtime UUID library was added.

## Files Owned

- `🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/{🦀️.rs,details.rs,tests.rs,tests.json}`
- `🏅️standards/🔖️4x3/🪆️subsets/✳️any/📦️packages/🦀️rust/{🦀️.rs,Cargo.toml}`
- IFC domain `Cargo.toml`, root artifact package manifest and `🦀️.rs` document codec registration.
- IFC `🧩️composition/🔣️.json` editor contribution and symbol package; `📜️artifact-definition.json` truthful declared standard/profile/document representation with exact retained source URL/SHA256.
- Shared contract facade `📇️registry/🧬️contract/🦀️.rs`: explicit reexports of the existing pointer decoder, detail capabilities and variant types, required by a domain provider.
- Ticket inputs `details-meta.py`, `details-oracle.py` and this report.

Catalog source/data and shared complex mutation changes belong to concurrent workers and were consumed directly.

## Verification

Tests and language-neutral cases were added before the provider/editor implementation. Rust tests cover inherited attribute names, enum choices, German optional states, hidden discriminants, aggregate cardinality, nested argument mutation resolution, transitive SELECT alternatives, distinct editor dialect, exact references beyond the suggestion window, missing-target refusal, entity identity allocation with unsorted/max-u64 inputs, GlobalId encoding/uniqueness, revisioned host tree labels/controller bindings and sparse complex diff/inverse behavior.

- **Passed:** `rustfmt --check` for the editor/provider/tests; this establishes Rust parsing and formatting, not type checking.
- **Passed:** Cargo metadata resolves `semio-s-artifact-stdio-ifc-4x3` as a domain workspace package and the artifact package's dependency on it.
- **Passed:** independent IfcOpenShell **0.9.0.post1** comparison of five language-neutral attribute/control positions, declared optionality, STANDARD enum membership, and three retained GlobalId compression vectors. Reproduction: `py -3.13 <ticket>/details-oracle.py`. The independent package is temporary under the shared ticket generated directory and must be removed by parent cleanup after all workers finish. Python 3.14 and the bundled 3.12 runtime do not match its cp313 wheel; no source changes were needed for that environment mismatch.
- **Passed:** JSON metadata parse, fixture-to-catalog declaration checks, and own details source paths remain below 256 characters.
- **Blocked/unverified:** focused `cargo test --manifest-path <IFC>/Cargo.toml -p semio-s-artifact-stdio-ifc-4x3 --lib --no-default-features`. The repository compiler ingress refuses the compiler version probe with `Original compiler ingress requires the repository launcher and authentic enforcing store`. The cached repo CLI has no general execution route; no enforcing-store or compiler configuration was bypassed or modified. Rust unit/host tests therefore have not executed and production compilation remains unverified.

## Required Follow-Up and Limits

The new package needs its own checked Rust IFC4x3 STEP IO/composer registration: the shared IFC4 codec now correctly rejects IFC4X3_ADD2. The host document codec and editor are registered, but actual natural IFC import/export cannot be claimed complete until that composer is connected. Parent has been informed to assign this follow-up.

Full EXPRESS WHERE/UNIQUE/inverse/contextual rules and MVD conformance remain outside the structural catalog implementation. Complex records remain editable and losslessly retained while full complex schema validation is unsupported. Templates for required references without an existing target may remain unset for progressive editing; whole-model conformance analysis reports that separately. Unique aggregates requiring multiple initial elements are not automatically fabricated from duplicate defaults. Reference suggestions are bounded, with exact identity input as the complete addressing mechanism; dedicated search/navigation to a referenced entity is not implemented.

No actual browser keyboard/focus, screen-reader or large-model scheduler/cancellation run has been performed. The host rendering/binding tests are written but blocked by compiler ingress. Existing stock UI accessibility/draft contracts are reused, not independently re-certified. Framework edit-rule resolution still materializes its document value tree; creation identity allocation and reference existence lookup are linear in the entity count, so strict bounded-step guarantees for those expensive commands require further host work and measurement. Initial catalog construction is lazy static initialization and has not been profiled.

These limitations mean this work is an integrated implementation with partial independent evidence, not a claim that the user's complete IFC4x3 UI/spec objective has been achieved.
