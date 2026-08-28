# GIS 3D Config Direct 45 Plan

## Observed Contract

The actual Rust owner is [config component](../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs). Its concrete mutation roster has exactly two variants:

| Existing variant | Canonical direct leaf | Payload | Existing semantics | Descriptor identity |
| --- | --- | --- | --- | --- |
| `SetCamera { camera_json }` | `🎥️set-camera` / `SetCamera` | required `cameraJson: string` | no-op warning if equal; otherwise updates only camera; inverse sets the exact old string | kind/opcode `set-camera`, tag `0`, `set/camera/SetCamera` |
| `SetLocale { value }` | `🗣️set-locale` / `SetLocale` | required `value: string` | no-op warning if equal; otherwise updates only locale; inverse sets the exact old string | kind/opcode `set-locale`, tag `1`, `set/locale/SetLocale` |

Both descriptors will have schema version 1, the full leaf owner path, `payloadSchema: "🧬️schema/🔣️.json"`, `invertibility: "explicit-mutation"`, `diffParticipation: "apply-only"`, outcomes `["applied", "warning"]`, `composition: "atomic"`, and required surfaces `["rust", "json-schema", "text", "binary"]`.

The current aggregate has handwritten `DslOps`, text codec, binary codec, `Mutation`, and an invalid whole-record diff (`type Diff = Gis3dConfig`). The direct replacement must mount a transparent two-newtype aggregate deriving `dsl::Mutations` and `dsl::DslOps`, retain the generic text/binary envelopes, and replace whole-record output with `Gis3dConfigDiff`: an ordered sparse camera-or-locale field delta. It must never use the full config as a diff. Neither current operation is nullable, map-backed, or default-removal based; its inverse preserves the exact prior string, including an explicit default string.

## Required Direct Tree

```text
config/
  🧬️schema/
    🔺️diff/🦀️.rs
    🔺️diff/🔣️.json
    🧬️mutations/
      🦀️.rs
      🔣️.json
      🎥️set-camera/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}
      🗣️set-locale/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}
  🧪️tests/🧬️mutations/🦀️.rs
```

The aggregate has strict internally tagged serde (`operation`, camel case, deny unknown fields); its JSON schema is a strict `oneOf` over the two leaf payload `$defs.payload` references and the exact `setCamera`/`setLocale` discriminants. Each leaf owns `MutationKind<Gis3dConfig, Gis3dConfigMutation>` diff/inverse/label/target and direct serde/text/binary/descriptor tests. Aggregate tests are structural only.

## Consumer Cutover

Only these current GIS 3D sites construct or destructure the old inline shapes:

- [view command](../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️view/🦀️component.rs) constructs `SetCamera`.
- [locale command](../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️locale/🦀️component.rs) constructs `SetLocale`.
- [terrain editor](../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs) matches both in validation, retained-byte accounting, store preparation, action decoding, and existing tests.
- The owning config component’s own tests construct both.

All must switch directly to the canonical payload wrappers; no alias or inline compatibility variant remains. Renderer, runtime, lifecycle, gismap config, Plugin/publication, launch, and seed sources are excluded.

## Schema Discrepancy Requiring Coordinated Cutover

Rust `Gis3dConfig` contains only `camera_json` and `locale`, while its current TypeScript, GraphQL, protobuf, and JSON schema facets also declare required `selectedIds`. No Rust mutation, constructor, or config state owns `selectedIds`. The direct-leaf roster is therefore two operations, not a copied 2D/three-field roster. To make the source schema contract truthful, the direct packet must remove `selectedIds` from those four sidecars in the same cutover, rather than inventing a selection mutation or retaining a stale field. This needs root confirmation because it broadens the packet from mutation schemas to the existing config facet parity repair.

## Schema-First Test Matrix

The ticket controller will use Ajv 2020 plus jsonc-parser as an independent reference. It will compile both actual leaf schemas and the aggregate, require valid strict envelopes, reject missing/unknown/wrong-type fields and bad/missing operations, test text opcode and binary-tag identity from descriptors, and independently model sparse forward/inverse application for camera and locale. Vectors include no-op (preserves the other field), both sequential updates, exact-default restoration, and explicit non-default restoration. It will record first hashes, recheck before exit, reject outside/`compose` paths, and no-follow every ancestor. Native Rust laws will be authored but not compiled or executed.
