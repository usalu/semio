# W1b mirror replacement — `✳️animation` subset

Replaced the generic "🚧 scaffolded by W1b" `entries:[{key,value}]` mirror files with real,
exhaustively-typed mirrors for the `✳️animation` subset of the `🧿️semio` artifact, matching the
already-landed convention (primary template: `✳️mesh`; tagged-union-to-GraphQL-`union` pattern
from `📐️step/✳️any`).

Directory: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/`

## Source of truth read

- `🧬️schema/📸️snapshot/🦀️component.rs` — `SemioAnimationSnapshot{schema, timelines}`,
  `AnimTimeline{name:Option<String>, channels}`, `AnimChannel{target, interpolation, keyframes}`,
  `AnimTarget{node, property:AnimTargetProperty}`, `AnimTargetProperty` (tag=`kind`:
  Translation/Rotation/Scale/Weights/Custom{name}), `AnimInterpolation` (unit: Linear/Step/
  CubicSpline), `AnimKeyframe{t, value}`, `AnimValue` (tag=`kind`: Scalar{value:f64}/
  Vec3{value:SemioPoint3}/Quat{value:SemioQuaternion}/Weights{values:Vec<f64>}).
- `🧬️schema/🔺️diff/🦀️component.rs` — `SemioAnimationDiff{timelines: Option<IndexedTripleDiff<AnimTimelineDiff, AnimTimeline>>}`,
  no full-replace slot; `AnimTimelineDiff{name: Option<Option<String>> tri-state, channels}`,
  `AnimChannelDiff{target, interpolation, keyframes}`, `AnimKeyframeDiff{t, value}` — all sparse,
  index-keyed collections at 3 nesting levels via the shared `engine::triples::IndexedTripleDiff<D,T>`
  / `IndexModified<D>{index,diff}` / `IndexAdded<T>{index,item}`.
- `🧬️schema/🧬️mutations/🦀️component.rs` — `SemioAnimationMutation` (tag=`mutation`), 13 variants:
  NoMutation, SetSnapshot, Insert/RemoveTimeline, SetTimelineName, Insert/RemoveChannel,
  SetChannelTarget, SetChannelInterpolation, Insert/RemoveKeyframe, SetKeyframeTime,
  SetKeyframeValue.
- `🧬️schema/🦀️component.rs` — `SemioAnimationArtifact{schema, timelines}`, mirrors the snapshot
  1:1.
- Confirmed `SemioQuaternion{x,y,z,w}` shape (and cross-checked its own already-real mirrors in
  the `✳️model` subset) since animation's `AnimValue::Quat` reuses the shared
  `engine::geometry::SemioQuaternion` type; `SemioPoint3{x,y,z}` confirmed against `✳️mesh`.

## Files written (16 total, 4 facets × 4 languages)

Root (Artifact facet, mirrors `SemioAnimationSnapshot` field-for-field):
- `🧬️schema/🔗️component.graphql`
- `🧬️schema/🟦️component.ts`
- `🧬️schema/🔣️component.json`
- `🧬️schema/🛰️component.proto`

Snapshot facet:
- `🧬️schema/📸️snapshot/🔗️component.graphql`
- `🧬️schema/📸️snapshot/🟦️component.ts`
- `🧬️schema/📸️snapshot/🔣️component.json`
- `🧬️schema/📸️snapshot/🛰️component.proto`

Diff facet:
- `🧬️schema/🔺️diff/🔗️component.graphql`
- `🧬️schema/🔺️diff/🟦️component.ts`
- `🧬️schema/🔺️diff/🔣️component.json`
- `🧬️schema/🔺️diff/🛰️component.proto`

Mutations facet:
- `🧬️schema/🧬️mutations/🔗️component.graphql`
- `🧬️schema/🧬️mutations/🟦️component.ts`
- `🧬️schema/🧬️mutations/🔣️component.json`
- `🧬️schema/🧬️mutations/🛰️component.proto`

## Key convention decisions

- **Tagged enums** (`AnimTargetProperty`, `AnimValue`) → GraphQL `union` of per-variant types
  (unit variants get a `{ _: Boolean }` marker type, per the `📐️step/✳️any` `StepValue` precedent);
  TS discriminated union on `kind:` (matches the real `#[serde(tag = "kind", rename_all =
  "camelCase")]`); JSON Schema `oneOf` with `const` discriminants; proto3 `message X { oneof kind
  { ... } }` (unit variants as `bool`, `Custom{name}` as a bare `string` field since it carries a
  single scalar — same idiom as `StepValue`'s `string enum_value`).
- **Unit enum** `AnimInterpolation` → GraphQL/proto bare SCREAMING_SNAKE enum values (`LINEAR
  STEP CUBIC_SPLINE`), no type-name prefix — matches the observed repo convention in `✳️mesh`,
  `✳️document`, `✳️drawing`, `✳️model` (the task brief's "type-name prefix" phrasing was not
  literally followed since it doesn't match any landed subset; declaration order preserved so the
  Rust `#[default]` variant `Linear` stays index 0).
- **Index-keyed collection diffs** (`timelines`/`channels`/`keyframes`, all
  `IndexedTripleDiff<D,T>`): monomorphized by hand per nesting level for GraphQL/JSON/proto since
  those languages can't express the Rust generic — `KeyframeTripleDiff`/`ChannelTripleDiff`/
  `TimelineTripleDiff`, each with its own `*Modified{index,diff}` / `*Added{index,item}` wrapper
  types, following the task brief's explicit `TimelineTripleDiff` naming hint. TS kept the real
  generic (`IndexModified<D>`, `IndexAdded<T>`, `IndexedTripleDiff<D,T>`) since TS supports
  generics natively — same choice `✳️mesh`'s diff TS mirror made for its own (name-keyed)
  `NamedTripleDiff<K,D,T>`.
- **Tri-state `AnimTimelineDiff.name`** (`Option<Option<String>>`: absent=unchanged,
  null=cleared, string=renamed) — documented inline in every facet; proto carries it as `optional
  string name` + `bool has_name`, matching `✳️mesh`'s own `materialId` tri-state idiom.
- **Shared geometry types** `SemioPoint3`/`SemioQuaternion` redefined locally in the snapshot
  facet's own mirror files (not imported cross-subset) — matches how `✳️mesh` and `✳️model`
  each redefine `SemioPoint3` locally; GraphQL/JSON/proto have no cross-file linking in this repo
  (`include_str!` leaves), only TS uses relative imports (from `📸️snapshot` into the root/diff/
  mutations facets of the SAME subset).
- Root/diff/mutations facets reference `AnimTimeline`/`AnimChannel`/`AnimTarget`/etc. by name
  without redefining them (GraphQL/JSON) or via relative `import type ... from
  "../📸️snapshot/🟦️component"` (TS) / `import "../📸️snapshot/🛰️component.proto";` +
  fully-qualified names (proto) — matches `✳️mesh` exactly.

## Verification

- All 4 written JSON files parse cleanly: `python3 -m json.tool` succeeded on
  `🧬️schema/🔣️component.json`, `📸️snapshot/🔣️component.json`, `🔺️diff/🔣️component.json`,
  `🧬️mutations/🔣️component.json`.
- `git status --porcelain` confirms exactly the 16 intended files under
  `✳️animation/🧬️schema/` are modified (`M`), no stray files added/removed elsewhere.
- Ran `cargo check -p semio-s-plugin-stdio`. It currently fails, but **not** because of anything
  in this change: the only compile errors are `E0063: missing fields 'label' and 'semantic_kind'
  in initializer of 'command::MutationMeta'` inside
  `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` (lines 1181, 3001) — a file this
  ticket never touched, that `git status` shows as independently modified (`M`,
  uncommitted) by another concurrent session mid-refactor of `MutationMeta`. All 16 files this
  ticket wrote are non-Rust `include_str!` leaves (GraphQL/TS/JSON/proto) with no parser wired
  into the Rust build, so they cannot be the cause of a Rust compile error. Re-running
  `cargo check` once that concurrent `MutationMeta` refactor lands elsewhere should confirm clean
  compilation; not re-verified further since this ticket's scope is `✳️animation`-only and that
  file is outside it.

## Note on an accidental typo (self-corrected)

While writing the mutations `🔗️component.graphql` file, a path typo used the literal CJK
characters `🏅️标准` instead of the emoji-spelled `🏅️standards`, creating a stray one-file
directory tree at `🧿️semio/🏅️标准/🔖️v1/🪆️subsets/✳️animation/🧬️schema/🧬️mutations/🔗️component.graphql`.
Caught immediately, removed with `rm -rf` before any further writes, and confirmed absent via
`find ... -iname "*标准*"` (no results) and clean `git status` (no untracked leftovers). The
correct file was then written to the real path.
