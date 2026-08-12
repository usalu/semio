# Semantic Mutation Verb Taxonomy

Reference vocabulary for every `🧬️mutations/<slug>/` triad the fan-out waves author. Naming
convention locked by the dev: **imperative in Rust** (variant name, triad-dir slug, serde tag,
grammar keyword); **past tense lives only** in `protocol::SemanticDescriptor.record` (for a future
operation-log / GraphQL layer). Golden reference for style only:
`compose/client/schema/graphql/schema.golden.graphql` — not implemented by this program.

## Closed core verb set (`protocol::APPROVED_VERBS`)

Mirrored 1:1 in `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`'s
`APPROVED_VERBS` const — this table and that const must never drift; a new verb is a spine change
in both places plus this doc.

| Verb | Meaning | Canonical args | Inverse partner | Record |
|---|---|---|---|---|
| `create` | Bring an id-keyed entity into existence | full initial payload (+ optional `index`) | `delete` | `Created<Noun>` |
| `delete` | Remove an id-keyed entity (captures cascade) | `id` | `create` (+ re-`connect` severed links) | `Deleted<Noun>` |
| `insert` | Place into an ordered, index-addressed list | `index` (FINAL-state), item | `remove` | `Inserted<Noun>` |
| `remove` | Take out of an ordered list / detach | `index` (BASE-state) or `id` | `insert`/`add` (captured item) | `Removed<Noun>` |
| `add` | Attach a set-like member (attribute, tag, connector) | owner addr + member payload | `remove` | `Added<Noun>To<Owner>` |
| `rename` | Change the identity field (`name`/`key`/`code`) | addr, `new_name` | `rename` (old name from base) | `Renamed<Noun>` |
| `change` | Set one scalar field to a new value | addr, `new_<field>` | `change` (old value) | `Changed<Noun><Field>` |
| `update` | Set one cohesive multi-field facet atomically | addr, facet fields (all required) | `update` (old facet) | `Updated<Noun><Facet>` |
| `move` | Absolute spatial reposition | addr, position | `move` (old position) | `Moved<Noun>` |
| `drag` | Relative spatial offset | addr(s), offset | `drag` (negated offset) | `Dragged<Noun>` |
| `resize` | Change extent | addr, new extent | `resize` (old extent) | `Resized<Noun>` |
| `rotate` / `scale` | Domain spatial transforms | addr, new value | self (old value) | `Rotated<Noun>` / `Scaled<Noun>` |
| `reorder` | Position within an ordered list (never spatial) | `from`,`to` or `id`,`to_index` | `reorder` back | `Reordered<Noun>` |
| `edit` | Replace an authored content body (text, cell, code) | addr, new body | `edit` (old body) | `Edited<Noun>` |
| `replace` | Whole-value swap of a large structured sub-payload | addr, new payload | `replace` (old payload) | `Replaced<Noun>` |
| `duplicate` | Copy an element to a new identity/position | source addr, target id/index | `delete`/`remove` | `Duplicated<Noun>` |
| `connect` / `disconnect` | Create/remove a relationship between endpoints | endpoint addrs (+payload) / edge id | each other | `Connected<Nouns>` / `Disconnected<Nouns>` |
| `bind` / `unbind` | Attach/detach a parameterization | binding payload / binding addr | each other | `Bound…` / `Unbound…` |
| `group` / `ungroup` | Introduce/dissolve a grouping node | member ids / group id | each other (ungroup captures membership) | `Grouped…` / `Ungrouped…` |
| `flatten` / `unflatten` | Collapse a hierarchy / restore it | subtree addr / addr + captured hierarchy | each other | `Flattened…` / `Unflattened…` |
| `split` / `merge` | One element → many / many → one | addr + spec / ids + payload | each other | `Split…` / `Merged…` |
| `extract` / `inline` | Hoist a fragment into a reusable entity / dissolve back | fragment addr / ref addr | each other | `Extracted…` / `Inlined…` |
| `clear` | Empty a collection/field wholesale | addr | `add`/`create` for every captured member | `Cleared<Noun>` |
| `fix` / `toggle` / `apply` | Domain-native single-flag/state ops | addr (+value) | context-specific | regular `-ed` |
| `set` | Single-field setter on an ADDRESSED target only | addr, new value | `set` (old value) | `Set<Noun><Field>` |

`set` stays approved for narrow, addressed, single-field setters (`set-layer-visible`). **Only
`set-snapshot` (whole-document replacement) is banned** — and it has NO replacement mutation. Per
the dev's locked decision, whole-document replace is not expressible as an in-history mutation at
all; it goes through `ArtifactStore::reset` (a non-undoable rebase used for file-open/import/
load-example), entirely outside the `Mutation` enum. `NoMutation` is banned outright — a mutation
with nothing to undo returns `Vec::new()` from `MutationKind::inverse`, no sentinel variant needed.

## Domain verbs (open by registration)

An artifact MAY register a verb outside the core set when the gesture is genuinely domain-native
and not expressible as one core verb: `paint-stroke` (lowpoly), `resolve`/`reopen` (conflicts),
`decide`/`revisit` (decisions), `approve`/`reject`. Requirements: imperative present tense, its own
emoji + kebab slug, a real inverse partner verb in the SAME dispatch enum, handcrafted diff +
inverse. A domain verb must NOT be a synonym of a core verb — `patch`, `modify`, `assign`, `put`,
`write` are all rejected (use `change`/`update`/`bind` instead).

## Bulk / plural mutations

Separate plural mutations (`delete-pieces{ids}`, `drag-pieces{ids,offset}`), never a bare `Vec` arg
bolted onto a singular verb. A plural variant is minted only where multi-select is a real editor
gesture. Slug: pluralize the noun (`🗑️delete-pieces`); record: `DeletedPieces`.

## Naming mechanics

- **Rust variant / triad-dir slug / serde tag**: verb-first, e.g. `RenameLayer` / `✏️rename-layer/`
  / `"rename-layer"`. New-value fields are `new_<field>`; address fields are bare (`id`,
  `slide_index`).
- **`SemanticDescriptor.kind`** MUST equal the triad-dir stem (emoji stripped) AND the kebab of the
  variant name — `#[derive(dsl_derive::Mutations)]` enforces this as a compile error, not a policy
  finding.
- **`SemanticDescriptor.record`**: irregular forms fixed once (create→Created, delete→Deleted,
  insert→Inserted, split→Split, fix→Fixed, flatten→Flattened, drag→Dragged, bind→Bound); everything
  else regular `-ed`.
- **Grammar keyword** = slug without emoji: `rename-layer name=walls new-name=partitions`.
- **Diff constructor** in the `🔺️diff` leaf: `pub fn diff(...)`, always builds the artifact's sparse
  Diff type directly from the payload — never apply-then-capture (that reintroduces a hidden
  `between`/whole-state dependency the derive can't see).

## Addressing convention

1. **id-keyed** (default whenever the schema has a stable id): `id: String`/`EntityId`.
2. **name/code-keyed** (when the format's native key IS the name — cad layers, xlsx sheets): address
   by name; `rename`'s inverse looks up the OLD name from `base`, never a captured id.
3. **index-keyed** (only intrinsically ordered, anonymous collections — pptx slides/shapes,
   paragraphs): `usize`, with the law: **removed/modified indices are BASE-state; inserted indices
   are FINAL-state**; `reorder`'s inverse is `reorder{from: min(to, len-1), to: from}`.
4. **Nested targets**: concatenate address fields, outermost first — `{ slide_index, shape_index }`.
5. Inverse always computed from `base` (pre-state), never by inverting the diff structurally.
   `delete`/`remove` capture the full removed payload (+ severed cascade, re-`connect`ed after
   `create` in reverse dependency order). Missing target ⇒ `inverse` returns `Vec::new()`.

## Forbidden vocabulary (policy-enforced, grep-able)

`SetSnapshot`, `NoMutation`, `CollectionMutation` used directly in a public `pub enum *Mutation`,
any bare `Set<Whole-Object>` variant, raw option-bag `Patch` mutation payloads (option-bags may
survive only as diff-INTERNAL types, never as a mutation's own payload), apply-and-capture diffs,
`Vec`-arg pseudo-bulk on a singular verb, index addressing where a stable id already exists on the
same collection.
