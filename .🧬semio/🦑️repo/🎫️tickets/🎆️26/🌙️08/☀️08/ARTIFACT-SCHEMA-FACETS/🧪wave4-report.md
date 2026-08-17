# Wave 4 Report — Lowpoly Pilot (Artifact Schema Facets)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Wave W4 owns `✏️s/🔌️plugins/💠️lowpoly/**` plus this ticket folder.

## 1. What changed

### Fifteen facet leaves

| Facet | Dir | Type |
| --- | --- | --- |
| artifact | `🗿️artifacts/💠️lowpoly/🧬️schema/` (5 formats) | `LowpolyArtifact` |
| snapshot | `🗿️artifacts/💠️lowpoly/📸️snapshot/🧬️schema/` (5) | `LowpolySnapshot` |
| diff | `🗿️artifacts/💠️lowpoly/🔺️diff/🧬️schema/` (5) | `LowpolyDiff` |

All quoted in `normative-spec.md` section 15 (verbatim from compiling leaves).

### Pack + set-snapshot

- pack moved under `📸️snapshot/pack/`
- set-projection renamed to set-snapshot (`SetSnapshot` / `set_snapshot` / grammar `set-snapshot`)

### Rename

`LowpolyProjection` replaced by `LowpolySnapshot` everywhere (no alias). Defined in `snapshot::schema` with DSL + Document codecs; artifact root re-exports.

### Diff as sparse field delta

`LowpolyDiff` is a sparse field delta (not a mutation list):

- `artifact: Option<Box<LowpolyArtifact>>` for whole replacement
- optional entry per non-effect artifact field
- `objects: Option<LowpolyObjectsDelta>` (`added`/`removed`/`patched`/`reordered`)
- paint ops under `LowpolyObjectPatchEntry.paint_layers`
- `selectedObjectIds` uses `LowpolyStringList` wrapper in the diff facet
- `MutationDiff<LowpolySnapshot>` applies persistent entries; `apply_to_artifact` applies all

Mutations build deltas via `diff_*` helpers in `🔺️diff/component.rs`. Per-mutation `🔺️diff/` folders hold thin wrappers; enum `type Diff = LowpolyDiff` only (no per-mutation `MutationDiff` impls).

### Engine

Owns real `LowpolyArtifact` + cached `LowpolySnapshot` (`type Artifact = LowpolyArtifact`, never collapsed to Snapshot). `apply` diffs + mutates snapshot then `artifact.set_snapshot(...)`.

`LowpolyDocument::with_context` seeds `next_object_serial` from max existing `obj-N` id so repeated app-level ObjectsAdd cannot collapse under sparse `reordered` (BTreeMap-by-id).

### Registry

`lowpoly_artifact_schema_descriptor()` include_str!s all 15 leaves. `engine::register()` into `OnceLock<Mutex<ArtifactSchemaRegistry>>`.

### Glue

leaf-prefixed + grouping `#[path = "."]`. Nested snapshot/diff keep `../../`. Diff runtime: `pub use super::schema::*;`.

## 2. Field inventory

**Persistent** (= snapshot): schema, objects.

**SharedUi**: activeObjectId?, selection, selectedObjectIds, paintUtility, activePaintLayer, activeUtilityId.

**LocalUi**: showEdges, sun*, worldCamera* (scalar X/Y/Z + fov), utilityParamsJson, paintColor*, selectionMethod, selectionModeDefault, engagementInput, locale.

**Preview**: hovered*, strokeDragActive, transformDragActive, previewSeq.

**Effect:** none.

Nested: meshJson (json blob), pixels (bytes), transform [f32;3].

## 3. Gate tails (verbatim)

### cargo check -p semio-s-plugin-lowpoly

```
warning: `semio-s-plugin-lowpoly` (lib) generated 22 warnings (run `cargo fix --lib -p semio-s-plugin-lowpoly` to apply 16 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 2.07s
```

### cargo test -p semio-s-plugin-lowpoly --lib

```
test result: ok. 139 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.64s
```

### cargo test -p semio-framework-schema

```
running 3 tests
test component::tests::graphql_state_preamble_matches_normative_sdl ... ok
test component::tests::registry_descriptors_carry_valid_snapshot_state_and_match_field_states ... ok
test component::tests::schema_catalog_still_registers_json ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

### bun ./script.ts policy 2>&1 | rg -i 'lowpoly'

```
(empty — no lines matched)
```

Direct `policyArtifactSchemaBreaches` filter: **lowpoly artifact-schema breaches: 0**.

Note: CLI `bun ./script.ts policy` currently exits without printing the breach table when stdout is a pipe/file (shared infra). Invoking `policyArtifactSchemaBreaches()` confirms artifact-schema scanners are clean for lowpoly.

## 4. Cross-crate blocker — RESOLVED

`semio-framework-plugin` now uses `DocumentApp::Snapshot` / `snapshot()` / `snapshot_with_conflicts()`.

## 5. Fan-out gotchas

1. First top-level type in each leaf must be XArtifact / XSnapshot / XDiff.
2. No top-level fixedList (GQL/proto become list).
3. Optional lists in Diff → scalar wrapper in all five formats.
4. Option<Option<T>> encoding per format.
5. Paint on ObjectPatchEntry.paint_layers, not DSL ObjectPatch.
6. With #[path="."] grouping, nested snapshot keeps ../../ .
7. Snapshot type only in snapshot::schema; root re-exports.
8. Do not redeclare GraphQL @state preamble.
9. Engine owns real XArtifact; snapshot() returns persisted subset only — never type Artifact = XSnapshot.
10. DocumentApp/views use .snapshot; SetSnapshot { snapshot }.
11. Sparse objects.reordered collapses duplicate ids — seed obj serials from existing obj-N.
12. Enum type Diff = XDiff only; no per-mutation MutationDiff unless DiffCodec/dsl::DslDiff in same file.
13. In diff runtime, pub use super::schema::*; (schema alone may hit extern crate).
14. Pack protocol segment name is Snapshot (not Projection).

## 6. Validation

- cargo check -p semio-s-plugin-lowpoly — green
- cargo test -p semio-s-plugin-lowpoly --lib — 139 passed
- cargo test -p semio-framework-schema — 3 passed
- lowpoly policyArtifactSchemaBreaches — 0
