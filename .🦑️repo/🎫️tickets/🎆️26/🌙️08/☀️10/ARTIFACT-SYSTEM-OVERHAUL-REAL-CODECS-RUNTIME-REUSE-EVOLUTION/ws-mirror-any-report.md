# W1b Closer — `✳️any` (envelope/union subset) Real Mirrors

Replaced the generic W1b scaffold mirrors for `🧿️semio`'s `✳️any` subset — the LAST semio subset
still on the scaffold — with real, exhaustively-typed mirrors across all 4 facets × 4 languages
(16 files). The Rust `🦀️component.rs` files in each facet were already real (not touched, not
part of this ticket's edit set) — only the generic-scaffold `🔗️component.graphql`,
`🟦️component.ts`, `🔣️component.json`, `🛰️component.proto` leaves were replaced.

## Rust source of truth (read, not modified)

- `🧬️schema/📸️snapshot/🦀️component.rs` — `SemioSnapshot{schema, subset: SemioSubsetSnapshot}`,
  `SemioSubsetSnapshot` is `#[serde(tag = "subset", rename_all = "camelCase")]`, 13 newtype
  variants (`Brep(SemioBrepSnapshot)` … `Workflow(SemioWorkflowSnapshot)`), internally tagged
  (flat wire shape: `{"schema":..., "subset": {"subset":"mesh", ...mesh fields flattened...}}`).
- `🧬️schema/🔺️diff/🦀️component.rs` — `SemioDiff` is `#[serde(tag = "kind", rename_all =
  "camelCase")]`: `NoChange` + 13 same-kind wrappers (`Brep(SemioBrepDiff)` … each delegating
  straight to that subset's own already-real `DiffAlgebra`/`MutationDiff` impl) + `Replace(Box<
  SemioSnapshot>)` for cross-kind changes / explicit `SetSnapshot`.
- `🧬️schema/🧬️mutations/🦀️component.rs` — `SemioMutation` is `#[serde(tag = "mutation", content =
  "payload", rename_all = "camelCase")]` (ADJACENTLY tagged, not internal — avoids a `mutation`-key
  collision with a wrapped variant's own internally-tagged `mutation` field): `NoMutation`,
  `SetSnapshot{snapshot}`, + 13 wrapper variants each carrying that subset's own mutation enum.
- `🧬️schema/🦀️component.rs` — `SemioArtifact{schema, subset: SemioSubsetSnapshot}`, field-for-field
  identical shape to `SemioSnapshot`.

## Cross-subset reference convention applied

Per the repo's established precedent (✳️presentation's `DocBlock` reuse of ✳️document's own type;
the step ap214 ✳️any `StepValue` union for the one-type-per-variant GraphQL/TS-discriminated-union
shape):

- **TS**: real relative imports to each of the 13 domain subsets' own snapshot/diff/mutations
  mirrors, 3 `../` hops from `✳️any/🧬️schema/<facet>/🟦️component.ts` (e.g.
  `"../../../mesh/schema/snapshot/component"`), matching ✳️presentation's own hop count (same
  directory depth). Same-subset, cross-facet references (e.g. diff → this subset's own snapshot)
  use the real emoji-named relative path (e.g. `"../📸️snapshot/🟦️component"`), matching ✳️mesh's
  own diff/mutations convention.
- **GraphQL**: referenced type names used bare, each annotated with a `# defined in ✳️<subset>'s
  own <facet> mirror` comment — no redeclaration of internal fields.
- **proto**: NEVER cross-imports between subset packages — every cross-subset field is a
  length-prefixed opaque `bytes` field with a `// SemioXSnapshot (x subset's own wire form)`
  comment. Same-subset, cross-facet proto imports (diff/mutations → this subset's own snapshot)
  use a real relative `import "../📸️snapshot/🛰️component.proto";` and a real message reference,
  matching ✳️mesh's own convention.
- **JSON Schema**: cross-subset fields are absolute-URI `$ref`s to that subset's own published
  `$id` (e.g. `"$ref": "https://semio.tech/schema/s.stdio.semio.mesh/snapshot.json"`), each with a
  `description` naming the referenced Rust type — preferred over an opaque `{"type":"object"}`
  leaf per the task's own stated preference. `$id`s were read directly from each of the 13
  subsets' own `🔣️component.json` files, including two irregular singular `mutation.json` ids
  (model, drawing, audio) reproduced verbatim rather than "corrected" (not this ticket's file to
  fix); ✳️presentation has no declared `$id` in either its snapshot or mutations json — referenced
  by the same `s.stdio.semio.presentation/...` convention every other subset follows, as the best
  available choice.

## Tagged-union modeling

`SemioSubsetSnapshot` (tag `subset`), `SemioDiff` (tag `kind`), `SemioMutation` (tag `mutation`,
content `payload`) are each modeled consistently across all 4 languages as one-wrapper-per-variant:
- GraphQL: one named `type` per variant (carrying the tag field + one referenced-type field),
  unioned via `union X = A | B | C …`.
- TS: a discriminated union of object-literal members (`{ subset: "brep"; brep: SemioBrepSnapshot }
  | …`), the same idiom ✳️presentation's own `SlideShape`/`PlaceholderKind` already establishes.
- proto: a `oneof` with one field per variant.
- JSON Schema: a `oneOf` with one object schema per variant (`const` tag + named `$ref`ed field).

13 variant order kept identical to the Rust source's declaration order in all 4 files of all 3
tagged-union facets (Brep, Mesh, Model, Object, Document, Cad, Drawing, Image, Video, Audio,
Animation, Presentation, Workflow) for easy side-by-side diffing against `🦀️component.rs`.

## Files written (16)

`🧬️schema/` (root/`SemioArtifact`):
- `🔗️component.graphql`, `🟦️component.ts`, `🔣️component.json`, `🛰️component.proto`

`🧬️schema/📸️snapshot/` (`SemioSnapshot`/`SemioSubsetSnapshot`):
- `🔗️component.graphql`, `🟦️component.ts`, `🔣️component.json`, `🛰️component.proto`

`🧬️schema/🔺️diff/` (`SemioDiff`):
- `🔗️component.graphql`, `🟦️component.ts`, `🔣️component.json`, `🛰️component.proto`

`🧬️schema/🧬️mutations/` (`SemioMutation`):
- `🔗️component.graphql`, `🟦️component.ts`, `🔣️component.json`, `🛰️component.proto`

All paths rooted at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/`.

No files outside this subtree were touched (no script.ts, glue.rs, catalog.json, launch.json,
other subsets, or artifact-root files outside `🪆️subsets/✳️any/`).

## Verification

- **JSON validity**: all 4 `🔣️component.json` files (root, snapshot, diff, mutations) pass
  `python3 -m json.tool` — confirmed valid JSON.
- **`cargo check -p semio-s-plugin-stdio`**: run twice (once immediately, once after a ~9-minute
  wait). Both runs fail with the SAME single pre-existing error, entirely unrelated to this
  ticket's files:
  ```
  error[E0063]: missing fields `label` and `semantic_kind` in initializer of `command::MutationMeta`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:1181:5
  ```
  This is in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — a shared framework/os
  store module completely outside the semio artifact tree and outside this ticket's scope.
  `git status` confirms that file is currently modified (`M`, uncommitted) by a concurrent session
  — matches this repo's known "concurrent cargo workspace churn" pattern (another in-progress
  session's own refactor of `MutationMeta`'s call sites). It was still `M` after the second,
  ~9-minute-later check, i.e. still actively being worked on.

  This error cannot be caused by this ticket's edits: all 16 files written here are
  `include_str!`-only leaves (GraphQL/TS/JSON/proto text mirrors) — the surrounding Rust
  (`🦀️component.rs` in each facet, not touched) already `include_str!`s them unconditionally
  regardless of their content, so nothing about their text can affect whether the crate compiles.
  The ✳️any facet's own real `🦀️component.rs` files (which the task explicitly says were already
  landed, W2b-complete, and out of scope to touch) were not modified.

  **Not independently re-verified once the unrelated error clears** — recommend a follow-up
  `cargo check -p semio-s-plugin-stdio` once the concurrent session's `MutationMeta` edit lands, to
  confirm a fully clean build, though nothing in this ticket's own edit set is expected to regress
  it.

## Summary

All 16 real mirror files for `✳️any` are written, internally consistent with each other, JSON-valid,
and structurally faithful to the Rust source of truth (`SemioSubsetSnapshot`/`SemioDiff`/
`SemioMutation`'s real tag keys, tag values, and 13-variant vocabularies). This completes the last
semio subset still on the W1b generic scaffold. `cargo check -p semio-s-plugin-stdio` currently
fails, but only on a pre-existing, unrelated, concurrently-in-progress edit in
`🏪️store/🦀️component.rs`, not on anything in this ticket's scope.
