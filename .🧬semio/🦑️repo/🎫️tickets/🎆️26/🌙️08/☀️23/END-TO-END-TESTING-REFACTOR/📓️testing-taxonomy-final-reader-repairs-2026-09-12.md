# Testing Taxonomy Final Reader Repairs

Ten active Rust test readers joined repository paths from `CARGO_MANIFEST_DIR` with a duplicated artifact prefix. Each reader now joins from its owning artifact package manifest at `📦️packages/🦀️rust` directly to the artifact's `🏅️standards` tree.

## Exact source repairs

Each change replaced only the reported string literal.

1. `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️fixtures/🦀️.rs`
   - `../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
   - `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
2. `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs`
   - `../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
   - `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
3. `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🧪️tests/🔬️unit/🦀️.rs`
   - `../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures`
   - `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures`
4. `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs`
   - `../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
   - `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
5. `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs`
   - `../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
   - `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
6. `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs`
   - `../../🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
   - `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
7. `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs`
   - `../../🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
   - `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
8. `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs`
   - `../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets`
   - `../../🏅️standards/🔖️1/🪆️subsets`
9. `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs`
   - `../../🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
   - `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
10. `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🧪️tests/🧩️example/🦀️.rs`
    - `../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🖼️assets`
    - `../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🖼️assets`

## Retained verification input

`🧑‍💻coordination/🧪️testing-taxonomy-final-reader-repairs/📜️script.ts` parses the audit's language-neutral JSON rows and independently resolves every old and new literal with Node's path implementation. It also checks the source text and verifies that each manifest owns the repository Rust target through `[lib] path = "../../🦀️.rs"`. Its generated evidence is `🗑️generated/testing-taxonomy-final-reader-repairs/resolution.json`.

## Verification

- Private Nx `layout-probe` execution of the retained Bun verifier: passed. Result: `[DEBUG] manifest-relative readers=10 current=10 obsolete=0 owned-targets=10`. All ten new resolved paths exist; all ten obsolete resolved paths are absent; all ten source files contain only the corrected literal; all ten manifests and owned Rust targets exist.
- `cargo test --manifest-path /Users/ueli/Documents/semio/Cargo.toml -p semio-s-artifact-energy-model --lib --no-run`, executed through the private Nx project: passed and produced the current energy library test binary.
- Direct execution of `standards::v1::subsets::any::schema::mutations::component::structural_correspondence_tests::direct_owner_descriptors_and_catalog_correspond` from that binary: passed 1/1. This exercises Rust's `env!("CARGO_MANIFEST_DIR")` join with the corrected standards path.
- The bounded EPJSON reader reached the corrected fixture root, then failed because the expected leaf `🏛️bestest-600/⚡️model.epJSON` is absent. No fixture regeneration was performed. This is existing fixture availability after successful path correction.
- The energy fixture round-trip runtime was cancelled after 90 seconds without output because it was not bounded. A later combined seven-package check was cancelled with no result after Cargo remained blocked on the shared artifact-directory lock. Neither cancellation produced a test failure.

The repository-wide final taxonomy census independently completed with zero findings over 139,975 file and directory entries during the final correction phase. These literal-only edits preserve the accepted canonical file placement.

## Exact Machine-Readable Changed Paths

```json
{
  "updated": [
    "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️fixtures/🦀️.rs",
    "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs",
    "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🧪️tests/🔬️unit/🦀️.rs",
    "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs",
    "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs",
    "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs",
    "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs",
    "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs",
    "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs",
    "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🧪️tests/🧩️example/🦀️.rs"
  ]
}
```
