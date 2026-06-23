## Plan: Normalize GraphQL Interface Field Groups

Normalize inherited field-group headers in c:\git\compose\compose\graphql\target.schema.graphql so every definition uses the established ancestor grouping convention. The concrete cleanup is to replace shorthand `# WeakEntity` header blocks with separate `# Node` and `# Entity` sections wherever those inherited fields are spelled out, while preserving the existing deeper hierarchy patterns such as `# StrongEntity`, `# RichStrongEntity`, and `# Artifact`.

**Steps**

1. Audit all definitions in c:\git\compose\compose\graphql\target.schema.graphql that currently use `# WeakEntity` and classify them by implemented ancestor: direct `implements WeakEntity`, `implements Modification`, and aggregate `*Modifications` types. This is the blocking inventory step for the rest.
2. Update each direct `WeakEntity` definition so the inherited block is split into `# Node` for `id` and `# Entity` for `hash`, `owner`, and `owns`, then keep the local fields under the definition-specific header such as `# Event`, `# Vector`, or `# Attribute`.
3. Update each `Modification` interface/implementation to use the same inherited split: `# Node` for `id`, `# Entity` for `hash`, `owner`, and `owns`, then `# Modification` for `before`, `diff`, and `after`. This depends on step 1 and can run in parallel with step 2 once the inventory is complete.
4. Update each aggregate `*Modifications` type that currently starts with `# WeakEntity` so the inherited fields are split into `# Node` and `# Entity`, then leave aggregate-specific fields under their local header or existing trailing fields. This depends on step 1 and can run in parallel with step 2 and step 3.
5. Spot-check neighboring definitions that already follow the convention, especially Diff, Version, WeakEntity, StrongEntity, Artifact, and representative Artifact descendants, to ensure the cleanup does not accidentally normalize patterns that are already intentional. This blocks final verification.
6. Run schema-wide searches to confirm there are no remaining shorthand inherited headers for the targeted class of definitions and that the comment hierarchy remains internally consistent.

**Relevant files**

- c:\git\compose\compose\graphql\target.schema.graphql - only file to modify; reuse the ancestor-grouping patterns already present in `interface WeakEntity`, `interface StrongEntity`, `interface Diff`, `interface Version`, `interface Artifact`, and representative descendants.

**Verification**

1. Search c:\git\compose\compose\graphql\target.schema.graphql for `# WeakEntity` and confirm remaining matches are only intentional survivors, or ideally zero if every inherited block has been expanded.
2. Search for `implements WeakEntity`, `implements Modification`, and `implements Diff`, then spot-check each class to confirm the inherited headers are grouped as `# Node` + `# Entity` before the local header.
3. Manually compare representative sections: Event, Modification, Vector, VectorModification, VectorModifications, PlaceModification, LayerModification, SideModification, and KitModification against WeakEntity and Diff to confirm consistent ordering.
4. If the repo has schema linting or generation for GraphQL comments, run it to ensure the file still parses unchanged; otherwise rely on textual verification because this is a comment-structure cleanup, not a schema shape change.

**Decisions**

- Included: comment-header normalization for inherited fields in c:\git\compose\compose\graphql\target.schema.graphql.
- Excluded: renaming GraphQL kinds, changing field signatures, or broader semantic/schema refactors unrelated to field-group headers.
- Recommended rule: whenever a definition spells out inherited WeakEntity fields, group them by ancestor origin as `# Node` then `# Entity`, not `# WeakEntity`.
- Preserve existing intentional deeper hierarchy patterns where descendants inherit through StrongEntity / Artifact and already use immediate ancestor headers correctly.

**Further Considerations**

1. If any `# WeakEntity` headers remain after the initial pass, treat them as likely missed cleanup rather than intentional shorthand unless a nearby contrary convention is found.
2. Because the file is large, execution should use search-driven passes rather than manual scrolling to avoid missing late-file occurrences around Side, Design, and Kit definitions.
3. A short follow-up check could look for similar shorthand collapses in other ancestor groups, but that is outside this task unless new inconsistencies surface during implementation.
