# H40 — representation export / architect / kit-store census
> Agent returned this report inline (read-only toolset, could not write). Transcribed verbatim by
> the coordinator. Claims marked ⚠️ are LOW CONFIDENCE and flagged for the red-team pass.

# A. REPRESENTATION EXPORT (IO)

## 1. `compose/fixture/export-design-representation.cases.compose.json`
1 case: `nakagin_capsule_tower`. Keys: `name`, `kit`, `designName`.
Kit reference: `kit/dev/metabolism/wip/initialKit/kit.compose.json`.

## 2. Binary fixtures
| File | Size | Detail |
|---|---|---|
| `nakagin-capsule-tower.gltf` | 598K | generator `https://github.com/mikedh/trimesh`; 1 node, 12 meshes, 24 buffers, buffers EMBEDDED |
| `nakagin-capsule-tower.ifc` | 2.4M | `FILE_DESCRIPTION` = `ViewDefinition[DesignTransferView]`; 24,792 entities; schema IFC4 |
| `placeholder.3dm` | 995K | referenced only from Nx `file-map.json` build artifacts — no code reference found |

## 3. NONDETERMINISM INVENTORY (must be normalized before parity)
- `uuid::Uuid::now_v7()` — `compose/client/lib/rs/lib.rs:662,666` — **time-seeded ids**
- `createdAt`/`updatedAt` ISO-8601 — `compose/client/lib/rs/lib.rs:12615-12618` — every Kit/Design/Type
- IFC `FILE_NAME` header timestamp — `compose/fixture/nakagin-capsule-tower.ifc:4` = `'2026-03-20T21:51:48+00:00'`
  (frozen in the fixture, but a live export regenerates it)
- glTF `asset.generator` string — trimesh version drift
- Float precision in glTF accessors — JSON number serialization rounding

## 4. Context fixtures
- `representation.selection.compose.json` — 5 cases for tag-based representation selection:
  default fallback, exact-match-beats-partial, subset filtering, no-match-returns-null, cast filtering
- `nakagin.kpi.representation.compose.json` — geometric KPIs: aspect ratios, bbox min/max, centroid,
  surface area `5569.8`, vertex count `18255`, face count `16812`, **not watertight**

## 5. ⚠️ Rust export symbols
No direct `export_*` function for glTF/IFC located in the public Rust API. Export appears to route
through GraphQL mutations (`createRepresentation`, `uploadFile`). Representation metadata lives in a
`representations` collection with a `file` reference and `tags`.
**⚠️ LOW CONFIDENCE — the trimesh generator string implies the glTF fixture was produced by a PYTHON
tool, not by Rust at all. Red team must establish who actually generates these exports before the
migration promises Rust-side export parity.**

## 6. `placeholder.3dm` — UNKNOWN. Only Nx file-map references.

# B. ARCHITECT / PLANNING

## 7. Fixtures
- `architect.cases.compose.json` — 13 cases in 4 tiers: e2e (5), memory (4), plan (2), parse (1)
- `architect.harness.kit.compose.json` — minimal kit: 3 typologies (Tower, Capsule, Base),
  2 designs, 3 types, pieces + connections

## 8. Rust — **RETURNS A PROPOSAL, DOES NOT MUTATE**
`pub async fn architect_run(query: &str, transport: &dyn Transport) -> Result<QueryResult>`
— `compose/client/lib/query/rs/lib.rs:1092`. Calls `Executor::run(plan, transport)` :987, which
applies plan steps and returns `env.finish(plan.return_items)` — a read-only projection.
No write operations, no kit-state side effects.
**Note: architect lives in `compose/client/lib/query/rs/lib.rs`, a DIFFERENT crate from the main
`compose/client/lib/rs/lib.rs` monolith.**

## 9. Determinism — deterministic
Plan steps applied in order (:686-691). No randomness, no search heuristic, no iteration-order
dependence, no time input. Single caveat: subscription streams return the first result only (:982-986).

# C. KIT-STORE / BUNDLE / ROUND-TRIP

## 10. `kit-store.contract.compose.json`
Append-only semantic op log with kind strings (`createdFixedPiece`, `renameKit`, …).
Operations: rootSnapshot mutation, semanticOpLog replay, checkpoint/draft/transaction lifecycle.
Persistence backends: `DEV_JSON` (single JSON file, atomic rewrite) or `LOCAL_DOT_COMPOSE`
(SQLite + blob files). GraphQL surface: `Mutation.kitStore.batch(KitStoreBatchInput)` with
create / importFile / importFromFolder / importFromZip.
**This is the closest legacy analogue to the migration's `Puzzle5dOperation` envelope — an
append-only, replayable, transactional op log. Study it before designing the envelope.**

## 11. `metabolism.zip`
182 members. Order: `.semio/kit.db` (5.0MB), `kit.json` (7.5MB), then `representations/` tree
alphabetically. **Timestamps are REAL (`05-20-2026 10:25`), not zeroed** → archive parity
comparison must normalize both member order and timestamps.

## 12. Codec projections
| Projection | Kept | Dropped |
|---|---|---|
| Kit shallow | authors, concepts, createdAt, description, designs, families, files, folders, hash, homepage, icon, id, image, license, name, preview, qualities, remote, tags, types, typologies, updatedAt, version | — |
| Kit meta | createdAt, description, homepage, icon, id, image, license, name, preview, remote, updatedAt, version | authors, concepts, designs, families, files, folders, hash, qualities, tags, types, typologies |
| Design shallow | authors, connections, createdAt, description, icon, id, image, layers, name, pieces, props, unit, updatedAt | — |
| Design meta | createdAt, description, icon, id, image, name, unit, updatedAt | authors, connections, layers, pieces, props |
| Type shallow | authors, connectors, createdAt, description, icon, id, image, name, props, representations, stock, unit, updatedAt, virtual | — |
| Type meta | createdAt, description, icon, id, image, name, stock, unit, updatedAt, virtual | authors, connectors, props, representations |

`metabolism.kit.light.compose.json` is a DIFFERENT shape: carries `schema` + `wip` (wrapping
initialKit), not a shallow/meta projection.

**Rule extracted: `meta` = scalars only, all child collections dropped. `shallow` = everything,
child collections present but presumably by reference/stub. Migration must confirm whether shallow
collections hold full objects or id-stubs.**

## 13. Store runtime surface
`compose/client/lib/js/kit-store.worker.ts:1-79` — `init(uri)` (only `dev://empty` allowed),
`execute(graphql_body)`, `subscribe(graphql_body)`. JSON postMessage with reqId correlation.
`compose/client/bin/store/AGENTS.md` — `POST /install`, `POST /graphql`, `GET /graphiql`;
first-install-wins lifecycle; `POST /server/shutdown`; tracing target `semio_store_event`.

## 14. Tests
- architect: `architect_fixtures_hydrate_and_cases_catalog()` — `compose/client/lib/rs/lib.rs:21060`
- kit snapshot VCS roundtrip rename — `compose/client/lib/rs/lib.rs:7962`
- metabolism.kit.light kind+port validation — `compose/client/lib/rs/lib.rs:21085`
