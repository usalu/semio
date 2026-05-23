---
name: Introduce Typology Entity
overview: Introduce a new first-class `Typology` entity that owns `Type`s and `Design`s, making `Kit` own `Typology`s instead of directly owning types/designs. `Family` (port compatibility) stays at kit root, and type/design `families` references are unchanged. Refactor across Rust authority, GraphQL, all language clients, schemas, persistence, fixtures, and tests, leaving no legacy.
todos:
  - id: ticket
    content: Open repo ticket (read repo://goals first); keep temp files in ticket folder
    status: completed
  - id: rust
    content: "Add Typology entity in lib.rs: Kit owns typologies, Type/Design owner_typology, computed Kit.types/designs, diff ladder, VFS, hydration; re-export GraphQL SDL"
    status: completed
  - id: graphql
    content: Update schema.golden.graphql (Typology type/connection/diff, Kit.typologies, Folder.typologies, owner->Typology); regenerate schema.graphql
    status: completed
  - id: json-openapi
    content: "Update kit.json/type.json/design.json/context + openapi: add Typology/TypologyId/typologies, type/design owning typology"
    status: completed
  - id: sql
    content: "Update sqlite + hub postgres + repo server postgres: typology tables, type/design owned by typology, keep family tables"
    status: completed
  - id: yaml
    content: "Update schema.yaml conceptual model: typology anchor, kit owns typologies+families"
    status: completed
  - id: client-go
    content: "Go main.go/kit_graph.go/main_test.go: Typology type, Kit.Typologies, owner, hashing/diff/sqlite"
    status: completed
  - id: client-py
    content: "Python main.py + engine main.py: Typology model, owner, parse/serialize, helpers, tests"
    status: completed
  - id: client-net
    content: "C# Semio.cs + Tests: Typology class, Kit.Typologies, owner, export snapshots"
    status: completed
  - id: client-ts
    content: "TS js/index.ts (+ react, query): Typology class, Kit.typologies, computed types/designs, Folder.typologies"
    status: completed
  - id: client-rb
    content: "Ruby semio.rb: mirror Typology"
    status: completed
  - id: sketchpad
    content: Sketchpad VFS node kind + readers + kit-tree nesting under typologies; storybook command schema; embedded tests
    status: completed
  - id: fixtures
    content: Add typologies and owning-typology to all kit fixtures, dev metabolism tree, assets metabolism + assets/index.ts lookups
    status: completed
  - id: grasshopper
    content: "Grasshopper components.json: typology params/components"
    status: cancelled
  - id: validate
    content: Regenerate schema; run Rust/Go/Python/C#/JS/sketchpad/algorithms tests; confirm metabolism round-trip with debug logs; close ticket
    status: completed
isProject: false
---

## Target model

- `Kit` owns: `typologies[]`, `families[]`, `folders[]`, `files[]`, qualities, etc. It NO LONGER stores `types[]`/`designs[]` directly.
- `Typology` (Artifact: `id, name, description, icon`, folder-placeable in VFS) owns `types[]` and `designs[]`.
- Each `Type`/`Design` has exactly ONE owning `Typology` (`owner` resolves to `Typology`, not `Kit`). They KEEP `families: FamilyId[]` references unchanged.
- `Family` (e.g. `L`, `J`, `s`) stays at kit root and keeps owning `ports[]` for connector compatibility.
- `Kit.types` / `Kit.designs` become COMPUTED flatten views across all typologies (kept as derived convenience accessors so the large query surface keeps working; they are derived, not stored, so this is not legacy).
- Metabolism typologies: `base, capsule, tambour, capital, bridge, tower` own their respective type(s) and design(s). Other kits/fixtures get a sensible default typology so the single-owner invariant always holds.

Note: `AGENTS.md` files (which contain the math/rank spec for `Kit = (T_K, D_K, ...)`) must NOT be edited per repo rules, so the formal spec update is out of scope; this is a known doc gap.

## Workflow (repo MCP)

- Read `repo://goals`, then open a ticket via `ticket_open` (e.g. `Introduce Typology Entity`) and keep all temp files inside the ticket folder.
- Use regions/subregions when adding code to existing files. Extend existing test files only; do not create new test files.
- Validate at runtime (run tests, confirm with logs) before closing the ticket via `ticket_close`.

## 1. Rust authority (source of truth)

`semio/client/lib/rs/lib.rs`:
- Add a `Typology` entity (mirror the `Family` macro shell at lines ~1450-1485): `id, name, description, icon, folder_id, owner_kit`, plus owned `types`/`designs` weak-id maps.
- `Type` (~3183) and `Design` (~4006): replace `owner_kit: Weak<Kit>` with `owner_typology: Weak<Typology>`; resolver `owner` returns the typology.
- `Kit` (~4734): remove stored `types`/`designs` vecs+maps; add `typologies: Vec<Arc<Typology>>` + id map. Add resolvers `typologies()`, `typology(id)`; make `types()`/`designs()` computed by flattening typologies. Keep `families` store and `snapshot_families_projection`.
- Diff/modification ladder: add `TypologyDiff`/`TypologyModification`/`TypologiesCollectionDiff` and wire into `KitDiff` (mirror the `Family*` diff machinery and `apply_families_collection_diff` at ~4897-5006, ~8049-8089). Type/design move-between-typology must update ownership.
- VFS: register typologies under kit root and folders (mirror family VFS at ~2465, ~13418). Hydration/round-trip: ensure typology placement + ownership persist.
- Re-export GraphQL SDL via `cargo test export_semio_graphql_schema_file` (driven by `semio/client/schema/graphql/script.ts`).

## 2. GraphQL contracts

- `semio/client/schema/graphql/schema.golden.graphql`: add `#region Typology` (type, `TypologyConnection/Edge`, diff ladder) mirroring `Family` (~1626-1747). On `Kit`: add `typologies`/`typology(id)`; keep `types`/`designs` as computed. On `Typology`: `types`, `designs`. On `Folder`: `typologies`. `Type`/`Design` `owner` -> `Typology`.
- `semio/client/schema/graphql/schema.graphql`: regenerated from Rust (do not hand-edit beyond what Rust emits).

## 3. JSON Schema / OpenAPI

- `semio/client/schema/json/kit.json`: add `Typology` + `TypologyId` defs and `typologies[]`; remove top-level `types`/`designs` from the stored kit shape (they become computed). Keep `Family`.
- `semio/client/schema/json/type.json` & `design.json`: add owning `typology` reference field; keep `families`.
- `semio/client/schema/json/{type,design}-context*.json`: mirror.
- `semio/client/schema/openapi/schema.json`: add `Typology`, `TypologyId`, `TypologyInput`, `typologies` on Kit; adjust Type/Design owner.

## 4. Persistence (SQL)

- `semio/client/schema/sqlite/schema.sql`: add `typology` table (kit_id FK, like `family` at ~77-86). Replace `type.kit_id`/`design.kit_id` with `typology_id` FK (+ indexes). Keep `family`, `type_family`, `design_family`, `port_compatible_family`. Update `sqlite/AGENTS.md`? (AGENTS.md is read-only - skip; the .sql is the source of truth.)
- `semio/server/hub/postgres/schema.sql`: add `core.typology`; point `core.type`/`core.design` ownership at typology. Keep `core.family`, `core.type_family`, `core.design_family`.
- `repo/server/schema/postgres/schema.sql`: add typology snapshot tables analogous to `kit_snapshot_*` kind/family tables (~275-501).

## 5. Conceptual schema

- `semio/client/schema/semio/schema.yaml`: add `typology` anchor (implements artifact, owns `types`/`designs`); `kit` owns `typologies` + `families`; remove direct `types`/`designs` from kit fields (now via typology).

## 6. Language clients (parallelizable - one generalist each)

- Go `semio/client/lib/go/main.go` (+ `kit_graph.go`, `main_test.go`): add `Typology`/`TypologyId` (mirror `Family` region ~879-961), `Kit.Typologies`, `Type/Design.TypologyId` owner; hashing (`HashTypology`, kit hash), diff algebra, SQLite read/write (`INSERT INTO typology`, type/design owner). Keep `Families`.
- Python `semio/client/lib/py/main.py`: add `Typology` model + `Kit.typologies`; `Type`/`Design` get owning `typology`; keep `families` fields/helpers. Update parse/serialize and family-helper regions. `semio/client/bin/engine/main.py`: typology-aware tools + tests.
- C# `semio/client/lib/net/Semio/Semio.cs` (+ `Semio.Tests`): add `Typology` class, `Kit.Typologies`, `Type/Design` owner typology; cross-language export snapshots.
- TS `semio/client/lib/js/index.ts`: add `class Typology extends Entity`, `Kit.typologies()/typology(id)`, `Typology.types()/designs()`, `Folder.typologies()`; keep computed `Kit.types()/designs()`. React `semio/client/lib/react` + `query`: hooks/selectors for typologies.
- Ruby `semio/client/lib/rb/semio.rb`: mirror.

## 7. Sketchpad / UI

- `semio/client/lib/sketchpad/js/index.ts`: register `Typology` VFS node kind (mirror `family` model at ~12240), readers for typology rows; keep family/port hydration. Update kit-tree to nest types/designs under typologies. Extend embedded tests.
- `.storybook/semio/algorithms/kit-store/commandSchema.ts`: add typology read commands.

## 8. Fixtures, assets, tooling

- Add `typologies[]` to kit fixtures and assign every type/design an owning typology:
  - `semio/fixtures/*.kit.semio.json` (metabolism, synthetic, architect, invalid handling) and the dev kit tree `semio/fixtures/kit/dev/metabolism/wip/initialKit/**` (~45+ type/design files): metabolism maps to `base, capsule, tambour, capital, bridge, tower`.
  - `semio/assets/semio/metabolism/**` and `semio/assets/index.ts` (add `MetabolismKitTypologies*` lookups next to `MetabolismKitFamilies*`).
- `semio/fixtures/script.ts`: typology-aware iteration if needed.

## 9. Grasshopper / engine

- `semio/assets/grasshopper/components.json`: add Typology params/components where Kit exposes types/designs; keep connector `family` param.

## 10. Validation

- Build/regenerate GraphQL from Rust; run Rust tests, Go `main_test.go`, Python pytest (`main.py`, engine), C# tests, JS/sketchpad Vitest/Playwright, dev/algorithms.
- Confirm round-trip: load metabolism kit, verify typologies own the right types/designs, `Kit.types`/`designs` flatten correctly, families/ports still resolve. Use `[DEBUG]` logs to confirm runtime behavior, then remove.