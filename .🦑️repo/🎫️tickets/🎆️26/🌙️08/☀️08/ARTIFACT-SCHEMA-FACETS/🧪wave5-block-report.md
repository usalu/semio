# Wave 5 Report — Block (`semio-s-plugin-block`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Plugin `✏️s/🔌️plugins/🎱️block/`. Crate `semio-s-plugin-block`.

Three artifacts × fifteen leaves each:

| Artifact dir | key | prefix | Former → new snapshot | Schema id |
| --- | --- | --- | --- | --- |
| `🗿️artifacts/◻2d/` | `block2d` | `Block2d` | `Block2dDefinition` → `Block2dSnapshot` | `s.block.block2d` |
| `🗿️artifacts/🖐️5d/` | `block5d` | `Block5d` | `Block5dDefinition` → `Block5dSnapshot` | `s.block.block5d` |
| `🗿️artifacts/🎊️3d/` | `block3d` | `Block3d` | `Block3dDefinition` → `Block3dSnapshot` | `s.block.block3d` |

## 1. Field inventories (state classes)

### block2d

**Persistent (= snapshot):** `schema`, `nodeKind`, `presentation`, `handleKinds`, `handles`, `compatibility`, `attributes`, `authors`, `camera2d`, `meta`.

**SharedUi:** `selectedIds` (from `Block2dConfig`).

**LocalUi:** `locale`.

**Preview / Effect:** none (`NoDraft`).

### block5d

**Persistent:** `schema`, `partKind`, `part2d`, `part3d`, `representations`, `gripKinds`, `grips`, `compatibility`, `attributes`, `authors`, `camera2d`, `camera3d`, `meta`.

**SharedUi:** `selectedIds`.

**LocalUi:** `locale`.

**Preview / Effect:** none.

### block3d

**Persistent:** `schema`, `objectKind`, `representations`, `vortexKinds`, `vortices`, `compatibility`, `attributes`, `authors`, `camera3d`, `meta`.

**SharedUi:** `selectedIds`, `activeRepresentationId?`, `wantedTags`.

**LocalUi:** `locale`, `windows`, `brushVortexKindId?`, `brushRadius`, `brushFlip`, `camera?` (session).

**Preview:** `brushPreview?`, `hoveredVortexFullId?`.

`Block3dWindowView` / `Block3dBrushPreview` moved from the app config module into the 3d artifact root so the artifact schema does not depend on `apps` (avoids an artifacts↔apps cycle). Config re-imports them from the artifact.

## 2. Diff-delta shape

Each `BlockXdDiff` is a sparse field delta:

- `artifact: Option<Box<BlockXdArtifact>>` — whole replacement (wins; replaces former `document:`)
- optional entry per non-effect artifact field
- identified collections → `{ added, removed, patched, reordered }` deltas (`HandleKinds` / `Handles` / `Representations` / `VortexKinds` / `Vortices` / `GripKinds` / `Grips` / `Compatibility` / `Attributes`)
- optional lists wrapped as `BlockXdStringList` / `BlockXdAuthorList` / `Block3dWindowsList`
- `Option` artifact fields use `Option<Option<T>>` / JSON `oneOf [null, T]` in the diff facet
- `MutationDiff<BlockXdSnapshot>` applies persistent entries; `apply_to_artifact` applies all classes
- `absorb` merges field-wise; later `artifact` clears everything

Mutations construct deltas via `diff_set_*` / `diff_remove_*` / `diff_set_snapshot` helpers. Folder `📄set-document` → `📄set-snapshot` (`SetSnapshot { snapshot }`).

## 3. Pack + glue

- `🎒️pack/` moved under `📸️snapshot/🎒️pack/` for all three (no `Projection` segment name in pack protocol to rename).
- Glue: leaf-prefixed + grouping `#[path = "."]`; nested `schema`, `snapshot::{schema,pack}`, `diff::{component,schema}`.
- `extern crate semio_framework_schema as schema`.
- TypeScript index mirrors schema / snapshot_schema / diff_schema exports; pack paths under snapshot.

## 4. Engine

Each engine owns real `BlockXdArtifact` + cached `BlockXdSnapshot` (`type Artifact = BlockXdArtifact`, never collapsed). `apply` diffs against snapshot, mutates snapshot, then `artifact.set_snapshot(...)`. `register()` also registers the fifteen-leaf `ArtifactSchemaDescriptor`.

## 5. Other preemptive fixes

- Config envelopes: `#[dsl(id = "block2d.config" | "block5d.config" | "block3d.config")]`
- `store::test_support` → `store::os_store::test_support`
- DocumentView/ConfigView `.projection` → `.snapshot`; `DocumentApp::Snapshot` / `initial_snapshot`
- SPR protocols: `tag N` → `tag=N`
- Example DSL `include_str!` paths restored (capsule / forest-right were wrongly pointed at forest-left)
- Removed duplicate `#[dsl(keyword = "leaveSurface")]` on empty `LeaveSurface` (macro alias already supplies the wire key)

## 6. Gate tails (verbatim)

### cargo check -p semio-s-plugin-block

```
warning: `semio-s-plugin-block` (lib) generated 60 warnings (run `cargo fix --lib -p semio-s-plugin-block` to apply 60 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 10.53s
```

### cargo test -p semio-s-plugin-block --lib

```
test result: ok. 100 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'block'

```
```

(empty — no lines matched)

Direct `policyArtifactSchemaBreaches` filter: **block artifact-schema breaches: 0** (total breaches elsewhere in monorepo: 660).

## 7. Shared-surface notes

- None blocked completion inside the plugin tree. Kernel `DocumentApp` / `ArtifactEngine` snapshot rename and `store::os_store::test_support` glob-export quirk were handled in-plugin (same pattern as GIS/draw).
- Accidentally created a parallel wrong-emoji `block` path during early writes; content was moved to the Cargo-rooted `🎱️block` package and the stray tree removed.

## 8. Not validated

- Full `workspace:verify-gate` / other plugins’ completeness.
- Runtime playground / UI smoke beyond lib tests.
- TypeScript vitest package tests (not required by the wave-5 block gates).
- MCP ticket open/close (wave-5 fan-out writes into the existing ticket folder).
