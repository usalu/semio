# Presentation Document Contract Ownership

The native parent owns schema plus required Presentation and Animation child handles. External artifact, snapshot, and diff facets still embed source/tiles. The exact native diff replaces the presentation handle; animation remains present in every full parent and is not a direct sparse diff field. The neutral parser/Ajv test is authored before correction and includes all 18 committed snapshots and five diffs.

The native child-slot kind metadata incorrectly appends the subset to s.stdio.semio; actual target dialects use artifactKind s.stdio.semio and subset presentation or animation. The handles and committed fixtures also use target IDs unequal to childId, which the shared restore authority rejects. These metadata/identity corrections will be tested through ChildRestoreProjection.

This work does not establish child content persistence: the process-global scratch map and missing member factories remain open work in composed-child-full-document-persistence-audit.md. Shared archive design must cover nested owned children, not only a direct child list.

Validation pending.

## Red and Source Changes

The first Bun/Nx run failed before validation because the old diff facet did not export parsePresentationDiff. The twelve external facets now share ArtifactChild, remove embedded source/tile copies, and match the native root and sparse diff. Native unknown-field rejection matches the closed schemas. Both constructor target IDs now equal childId; eighteen committed snapshot handles were corrected by hand-authored field updates. Native tests assert the actual default and neutral loaded snapshot are accepted by ChildRestoreProjection. Procedure child metadata had the same subset-qualified pseudo-kind and is now s.stdio.semio, with an added native restore-projection assertion. No complete child persistence is claimed.

## File Ledger

- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛰️.proto`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🛰️.proto`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🦀️.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🦀️.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🗑️delete-tile/🧪️rejects-deleting-a-missing-tile/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🗑️delete-tile/🧪️rejects-deleting-a-missing-tile/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔁replace-tiles/🧪️no-ops-when-the-collection-is-already-empty/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔁replace-tiles/🧪️no-ops-when-the-collection-is-already-empty/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/✂️resize-tile-crop/🧪️rejects-a-zero-width-crop/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/✂️resize-tile-crop/🧪️rejects-a-zero-width-crop/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🖼️replace-source/🧪️no-ops-when-the-source-is-already-identical/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🖼️replace-source/🧪️no-ops-when-the-source-is-already-identical/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/✏️rename-tile/🧪️no-ops-when-the-tile-already-has-that-name/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧹delete-tiles/🧪️rejects-when-every-addressed-tile-is-missing/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/✏️rename-tile/🧪️no-ops-when-the-tile-already-has-that-name/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧹delete-tiles/🧪️rejects-when-every-addressed-tile-is-missing/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔲resize-source-frame/🧪️no-ops-when-the-frame-is-already-identical/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🆕create-tile/🧪️rejects-a-duplicate-tile-id/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🆕create-tile/🧪️rejects-a-duplicate-tile-id/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔀reorder-tiles/🧪️no-ops-when-the-tile-is-already-at-that-index/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔀reorder-tiles/🧪️no-ops-when-the-tile-is-already-at-that-index/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔲resize-source-frame/🧪️no-ops-when-the-frame-is-already-identical/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🦀️.rs`

## Oracle Result

`presentation-document-contract-green-1.log` completed through Bun/Nx with exit 0: first-party parsers matched Ajv for 18 committed snapshots, five diffs, and neutral positive/refusal vectors. Strict TypeScript validation of artifact/snapshot/diff passed. Native runtime remains pending while the shared transient owner interface is being migrated.

## Native Contract Result

`abstraction-ownership-validation:presentation-document-contract-native` exited 0. The targeted native test passed (one test, 316 filtered out), with debug confirmation that Presentation JSON/text/Pack preserve the presentation and animation child identities and reject inline content or foreign editor state. The same run executed the TypeScript/Ajv oracle and strict TypeScript facet check. Evidence: `🗑️generated/presentation-document-contract-native-1.log`. This validates the parent contract and child projection metadata; full owned-child persistence remains the separate open composition task.
