# 🧪 Wave 0 — Ticket + Normative Spec

## Changed

- `🎫️ticket.json` — written by hand (repo MCP absent, see `mcp-unavailable.txt`). Goal
  `AI-OPTIMIZED-REPO`, verified present in `.🦑️repo/🎯️goals/`.
- `📜️normative-spec.md` — the single contract every later wave reads.

## Decisions locked by the spec

1. Three facets per artifact, five formats each, 15 leaves × 54 artifacts = 810 handcrafted leaves.
2. `🔣️component.json` (JSON Schema) is the normative leaf inside each facet; the other four mirror it.
3. `schemaFormats` is its own taxonomy key, not an `ecosystems` / `taxonomyLeafFilenames` entry —
   `validateTaxonomy` cross-asserts those two agree, and GraphQL/JSON Schema/Protobuf are not package
   ecosystems.
4. The normative-leaf map is a new `artifactSchemaSpecFilenames` key, not `artifactSpecFilenames`,
   because `validateTaxonomy` requires every value of the latter to end in `.semio`.
5. `🛰` is free repo-wide → `🛰️component.proto`.
6. `XSnapshot` replaces every heterogeneous snapshot type name, not just `XProjection`. The bare
   `Document` shared by all fifteen norm artifacts is the strongest argument for the rename.
7. `XDiff` is a sparse field delta over the artifact, with one `artifact:` whole-replacement entry;
   it implements `MutationDiff<XSnapshot>` over its `persistent` entries and adds
   `apply_to_artifact` for the rest. Lowpoly's current mutation-list diff is therefore a rewrite.
8. Diff coverage rule: every non-`effect` field must have a diff entry; `effect` fields must not.

## Artifact census

54 artifacts across 31 plugin crates, enumerated with keys, type prefixes and the exact current
snapshot type name in §10 of the spec. Two are structurally incomplete today and need building rather
than renaming: `🎪️demonstrator/🎪️playground`, `🔋️energy/🔋️model`.

## Gate

None — W0 produces documentation only. No source file was touched.

## Left for later waves

- W1 owns the taxonomy JSON and its three non-root consumers.
- W2 owns root `📜️script.ts` (policy region + the nested-facet fix in `policyTaxonomyDirsBreaches`).
- W4 appends its finished lowpoly leaves to the spec as §15.
