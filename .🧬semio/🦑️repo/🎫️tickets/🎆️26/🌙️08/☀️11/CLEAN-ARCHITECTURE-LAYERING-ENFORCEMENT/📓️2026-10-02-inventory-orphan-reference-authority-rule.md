# Inventory Orphan Reference Authority Rule

Read-only source review; no tests/jobs. Owner-relative paths use Repo library `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Retained positives are physical observations too

Fixture `🧫️fixtures/📋️mutation-inventory/🧪️consumers/🔣️.json` deliberately contains orphan catalog, registry, operations and command Rust files with literal path mounts and bare-head `use insert_page::Mutation`. `🧪️tests/🔬️workspace-contract/🟦️.ts:7568–7577` requires their consumer edges alongside genuinely Cargo-mounted command and cross-owner edges. Requiring a Cargo context for every inventory reference would erase intentional physical consumer observations. The inventory output schema is `🧬️schema/📋️mutation-inventory/🔣️.json`; input fixture schema is inline at workspace test :7535, not that output schema.

Current evidence module :134 builds graph with conventionalRoots true, without strictManifests. :179–192 falls back from absent graph result to exactly one local literal mount after root/inline source selector. It correctly retains orphan observations, but does not distinguish orphan from rejected participation. A bare graph result [] is insufficient to choose physical fallback.

## Minimal non-restoring rule

Use an owned graph route result with state **admitted**, **denied**, or **unmounted**. Keep target bytes/physical reference observation separate from route authority:

- admitted: retain current graph route target and context provenance.
- denied: return no resolved Rust consumer target; record unresolved reason/key. Never retry physical declaration resolution.
- unmounted: allow a *physical orphan observation* only for a bare local head, one exact canonical authored declaration, proven root/enclosing inline source scope, !unresolved, non-inline explicit literal relative path, and captured readable target bytes with proven target root scope. This observation must not confer Cargo/provider membership. Preserve its existing inventory edge while adding explicit physical provenance if the output contract currently conflates the two.

Graph must publish denied source/origin participation independently of admitted contexts. Required evidence includes authored manifest roots and module/include candidate source origins before filtering plus canonical denied prefixes and invalid manifests. Empty contexts alone is ambiguous: a refused root can be omitted before addContext, whereas a pruned leaf currently leaves an empty map entry. Do not depend on incidental contexts.has(path) behavior. For a source with any actual known graph participation, require every relevant origin to be admitted rather than treat absence as an orphan. Distinct valid crate roots remain distinct.

The physical fallback should use captured contents, not files.includes(target) alone. Its current signature lacks readSource/contents; introduce the same captured source view already owned by inventory, rather than reread uncaptured bytes. Require strict authored manifest parsing in this inventory graph; malformed manifest status cannot be ignored merely to preserve orphan positives.

## Closed minimal oracle rows

Extend current consumers input schema with closed routeCases (or a specific existing owner facet) using fields id/sourcePath/sourceScope/specifier/files/unreadable/expectedState/expectedTargets/expectedReason. Keep all existing corpus positives unchanged.

| Case | Expected |
|---|---|
| orphan bare head, literal mount, readable target | unmounted + physical edge |
| orphan crate/self route | unmounted, no physical fallback |
| mounted bare head, exact admitted target | admitted + edge |
| mounted known + unresolved same canonical key, both declaration orders | denied + no edge |
| mounted source with valid local head plus nonportable sibling revoking manifest | denied + no edge |
| explicit root manifest malformed, otherwise same orphan-shaped local head | denied + no edge |
| known root/leaf origin pruned by unknown inner metadata | denied + no edge |
| orphan readable local declaration but unreadable target bytes | unresolved physical observation, no edge |
| orphan target readable empty bytes | physical edge remains |
| orphan source has unknown inline sibling, import at proven root | physical edge remains; only selected scope matters |

Default Rust native success can independently prove orphan row content by selecting that source as a temporary crate root; it does not prove repository Cargo participation. Callback unreadability is a captured-authority oracle, not a native acceptance claim. Existing registered workspace-contract inventory materialization/fast-glob source roster assertions :7548–7557 provide physical corpus evidence without changing original 30-row macro native cohort.
