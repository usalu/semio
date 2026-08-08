# Wave 5 Report — Procedural (Artifact Schema Facets)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Wave W5 agent owns `✏️s/🔌️plugins/🌀️procedural/**` plus this ticket folder.

Crate: `semio-s-plugin-procedural`. Artifacts: `procedural2d` (`Procedural2d*`), `procedural3d` (`Procedural3d*`).

## 1. What changed

### Fifteen facet leaves × 2

| Facet | Dir | Types |
| --- | --- | --- |
| artifact | `🗿️artifacts/<a>/🧬️schema/` (5 formats) | `Procedural2dArtifact` / `Procedural3dArtifact` |
| snapshot | `🗿️artifacts/<a>/📸️snapshot/🧬️schema/` (5) | `Procedural2dSnapshot` / `Procedural3dSnapshot` |
| diff | `🗿️artifacts/<a>/🔺️diff/🧬️schema/` (5) | `Procedural2dDiff` / `Procedural3dDiff` |

Schema ids: `s.procedural.procedural2d`, `s.procedural.procedural3d`.

### Pack relocation

`🎒️pack/` moved under `📸️snapshot/🎒️pack/` for both artifacts (plain `mv`).

### Rename

`Procedural2dDocument` / `Procedural3dDocument` replaced by `Procedural2dSnapshot` / `Procedural3dSnapshot` everywhere in the plugin (no alias). Defined in `snapshot::schema`; artifact roots re-export.

### Diff as sparse field delta

`Procedural*Diff` is a sparse field delta (not a mutation list):

- `artifact: Option<Box<XArtifact>>` whole replacement
- `fixture: Option<FlowFixture>` / `generation: Option<GenerationPlayState>` persistent entries
- UI entries for every non-effect artifact field
- optional lists wrapped as `Procedural*StringList`
- `MutationDiff<XSnapshot>` applies persistent entries; `apply_to_artifact` applies all
- Mutations construct deltas via `diff_fixture_from_helpers` / `diff_generation_from_ops`

### Engine

Each engine owns a real `XArtifact` + cached `XSnapshot` (`type Artifact = XArtifact`, never collapsed to Snapshot). `apply` diffs + mutates snapshot then `artifact.set_snapshot(...)`.

### Registry

`procedural2d_artifact_schema_descriptor()` / `procedural3d_artifact_schema_descriptor()` `include_str!` all 15 leaves. Registered from `engine::register()`.

### Glue

Leaf-prefixed + grouping `#[path = "."]`. Nested `snapshot` / `diff` keep `../../`. Diff runtime: `pub use super::schema::*;`. TS index mirrors schema/snapshot/diff/pack paths.

## 2. Field inventory

### procedural2d

**Persistent** (= snapshot): `fixture` (`FlowFixture`), `generation` (`GenerationPlayState`).

**SharedUi**: `selectedIds`, `selectedGenerationId?`.

**LocalUi**: `graphCamera`, `showMode`, `locale`.

**Preview**: `generationPreviewText?`.

**Effect:** none.

### procedural3d

**Persistent** (= snapshot): `fixture`, `generation`.

**SharedUi**: `selectedNodeIds`, `selectedGenerationId?`, `activeUtilityId`.

**LocalUi**: `lodMode`, `showMode`, `selectionMethod`, `graphCamera`, `previewCamera` (named XYZ scalars), `sunJson`, `locale`, `contributionsJson`.

**Preview**: `hoveredNodeId?`, `generationPreviewText?`.

**Effect:** none.

### Diff-delta shape

`artifact?`, `fixture?`, `generation?`, plus `Option` / `Option<Option<_>>` / `Option<XStringList>` entries for every non-effect artifact field.

## 3. Glue convention

Leaf-prefixed + grouping `#[path = "."]` (same as lowpoly pilot §15.5).

## 4. Gate tails (verbatim)

### 1. `cargo check -p semio-s-plugin-procedural`

```
warning: `semio-s-plugin-procedural` (lib) generated 28 warnings (run `cargo fix --lib -p semio-s-plugin-procedural` to apply 28 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 9.94s
```

**PASS**

### 2. `cargo test -p semio-s-plugin-procedural --lib`

```
test result: ok. 193 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 14.69s
```

**PASS**

### 3. `bun ./📜️script.ts policy 2>&1 | rg -i 'procedural'`

```
(empty — no lines matched)
```

Direct `policyArtifactSchemaBreaches()` filter: **procedural artifact-schema breaches: 0** (`🧪wave5-procedural-policy-probe.ts`).

## 5. Follow-up fixes (post framework unblock)

After `semio-framework-os-flow` switched to `.snapshot()`, crate-local compile/test failures were driven green:

- Test doubles: prefer `semio_framework_os_kernel::os_store::test_support::*`; SPR protocols use `tag=N`.
- Handcrafted real `🗣️example.dsl.semio` for procedural2d (no stub).
- Promoted `ensure_linked_flow_extensions()` on the procedural3d engine; plugin setup + both app testkits call it so bare `app()` tests get linked flow operators / brep tessellate.

## 6. Validated

- `cargo check -p semio-s-plugin-procedural` green.
- `cargo test -p semio-s-plugin-procedural --lib` — 193 passed.
- Policy: zero procedural artifact-schema breaches.

## 7. Ticket probes

- `🧪wave5-procedural-gen-leaves.py`
- `🧪wave5-procedural-policy-probe.ts`
- `🧪wave5-procedural-report.md`
