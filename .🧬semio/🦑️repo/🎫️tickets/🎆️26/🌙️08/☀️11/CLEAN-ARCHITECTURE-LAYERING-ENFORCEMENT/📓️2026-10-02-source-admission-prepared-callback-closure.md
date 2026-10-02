# Prepared Source Admission Callback Closure

Read-only follow-up inspection; no edits to source/tests, no tests or runtime jobs executed. Line evidence describes the observed pre-fix source. All paths are relative to the shared repository. `N` is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization`.

## Exact Production Callers

Whole nonhidden repository `*.ts`/`*.json` symbol search found precisely two preparation and two collection invocation sites:

- `N/🟦️.ts:4601`: `inventoryTaxonomyWithSourceParentPruning` prepares raw inventory options.
- `N/🟦️.ts:4609`: that function calls `collectTaxonomySourceAdmission(options, taxonomy, prepared)`.
- `N/🚪️source-admission/📁️io/🟦️.ts:298`: public `inventoryTaxonomySources` prepares options.
- `N/🚪️source-admission/📁️io/🟦️.ts:300`: that function calls `collectTaxonomySourceAdmission(options, taxonomy, prepared)`.

No other invocation was found in that search scope. The umbrella physically imports both helpers from IO at line 13.

## Canonical API Recommendation

Change collection to exactly `(taxonomy: LoadedTaxonomy, prepared: SourceAdmissionPreparedOptions)`. The prepared owner should own nine fields: existing `repoRoot`, `scope`, `taxonomyPath`, `ticketDir`, `cancelFile`, `indexRows`, `repositoryFences`, plus detached/frozen `structuralDirectoryNames` and captured `progress`. Capture every raw scalar and callback reference and copy/freeze the structural roster at function entry, before validation, filesystem work or the first progress callback. Validate and return only these captured values. Freeze the prepared record itself; capture index rows and fences immutably if the whole prepared authority is intended to be immutable.

Use prepared progress and structural roster exclusively in collection. Both production callers then pass `(taxonomy, prepared)`. There is no compatibility requirement or justified raw-options alias.

The normalizer declares structural names on `TaxonomyInventoryOptions` at `N/🟦️.ts:263`; `inventoryTaxonomyWithSourceParentPruning` never derives or assigns them. It supplies raw options to preparation at line 4601, emits setup callbacks at lines 4602 and 4607, then collection currently reads raw structural names after those callbacks. Thus the capture must precede preparation's own callbacks, not merely precede collection. Setup progress should use prepared.progress for the source admission phase. Workers and subsequent normalization options are a distinct larger orchestration snapshot concern; they are not needed to preserve the nine-field admission authority.

## Borrowed Test Boundaries

`N/🧪️tests/🚪️source-admission-io/🟦️.ts:18-29` builds syntax providers from the actual umbrella plus six physical owners and requires one canonical declaration per selected symbol. `invoke` transpiles the selected current declaration with explicitly injected free dependencies, so tests bind physical current declarations.

Existing loaded-hash case boundaries:

- Line 76 extracts actual collect and injects its dependencies. Collection's parameter change is naturally picked up, but any new free dependency must be explicitly injected.
- Line 77 extracts actual public source, injecting a prepared stub with the existing seven fields. Add progress and structuralDirectoryNames to the stub if these become required fields. The current public caller's new two-argument call must flow through the same actual collect declaration.
- Line 82 extracts actual prepare with a narrow dependency list. This path intentionally refuses opaque input before filesystem/callback work. A new snapshot helper/free dependency called before lexical refusal must be injected here, even though the old callback/Git dependencies were unnecessary on that early-refusal path.
- Line 83 extracts actual SourceParentPruning, injecting actual prepare and setup report/load stubs. Preserve its opaque refusal and zero filesystem expectations; do not remove them as part of the new callback law.

Inline capture avoids adding an extra declaration/helper injection obligation. If a helper is used for semantic clarity, explicitly update those borrowed boundaries; do not restore an umbrella implementation.

## Closed Corpus Updates

Retain all nine current IO cases unchanged: strict-git-framing, raw-git-spelling, opaque-untracked-pruning, root-and-candidate-nofollow, fifo-and-permission, nested-git-terminal, directory-identity-drift, loaded-hash-and-opaque-setup, ticket-generated-output-exclusion.

The IO test line 39 currently asserts `cases` length nine. The single-line schema at `N/🧬️schema/🚪️source-admission/🧪️io/🔣️.json:1` fixes minItems/maxItems nine and oneOf references framing/raw/opaque/nofollow/fifo/git/drift/setup/generatedOutput. Append one new closed callback-capture case, change both count bounds and test count to ten, and add its exact closed input/expected definition and dispatch branch. Keep all existing schema references, fixture entries and assertions.

The inspected source-services fixture fixes six owners and their local edges, not the IO case count or prepare parameter/field roster; no existing topology fixture update is indicated by this API change alone. The source-services support byte roster remains nine physical files and should remain unchanged. A new schema-first AST roster law for the nine prepared fields and two collect parameters would make the chosen API explicit without weakening current topology or parser facts assertions.

An effective new case mutates scope, taxonomyPath, ticket/cancel paths, progress callback and structural names from the first tracked-enumeration callback; records initial validated inputs, callback identity, structural walker arguments and prepared values; attempts post-preparation roster mutation; and invokes actual collect from the captured authority. Observe both preparation callbacks and later setup/collection callbacks. Third-party schema validation should still validate the same closed fixture. All expected outcomes remain proposed until Root executes the actual baseline and implementation laws.
