# Rs Graphql Js React Sketchpad Pipeline

## Golden schema vs `metabolism.kit.compose.json` (`wip.initialKit`)

**Top-level keys on `wip.initialKit` in the fixture** (first lines of `compose/assets/fixtures/metabolism.kit.compose.json`): `id`, `name`, `version`, `createdAt`, `updatedAt`, `description`, `icon`, `image`, `remote`, `homepage`, `license`, `preview`, `types`, …

**`type Kit` in `schema.golden.graphql`** already exposes the scalar/metadata surface (`name`, `description`, `icon`, `image`, `preview`, `remote`, `homepage`, `license`, `uri`, `createdAt`, …). The fixture field **`version`** was present in RS (`Kit::version`) but **missing from the golden SDL**; **`version: String # data`** was added so GraphQL matches RS and the fixture.

**Nested collections** (e.g. `types.items[]` with `representations`, `families`, …) are not “fields on `Kit`” in GraphQL; they map to connections such as `types`, `designs`, `files`, … on `Kit`. A full structural equality between every nested JSON key and SDL would be a separate codegen/contract pass.

**Hydration:** `hydrate_kit_from_initial_projection_value` now applies string scalars from the projection (`description`, `icon`, `image`, `preview`, `remote`, `homepage`, `license`, `uri`, `version`). `initial_kit_projection_value` round-trips those fields so `Graph.initialKit` / `theKit.kit` stay aligned after clone.

## Native store + GraphQL

- **`ParentStore::spawn_from_install_json_value`**: same branch as wasm `bootstrap_runtime_from_json_value` — bundle schema → `hydrate_into_graph` on a spawned store; otherwise overlay from bare projection JSON.
- **`compose-store` `POST /install`**: `create` and `importFile` both use that helper (full `*.kit.compose.json` bundle or bare `initialKit` DTO).

## Play / sketchpad

- Set **`VITE_COMPOSE_NATIVE_STORE=1`** and **`VITE_COMPOSE_STORE_URL=http://127.0.0.1:4000`** (or your bind URL) to wrap the playground in **`ComposeStoreKitLineHost`** (RS over HTTP → `Session` → contexts → sketchpad). Otherwise the default **WASM + `InMemoryKitStore` + `/metabolism.zip`** path is unchanged.
- Install the store first, e.g. `curl -X POST http://127.0.0.1:4000/install -H "Content-Type: application/json" -d "{\"importFile\":{\"path\":\"C:/.../metabolism.kit.compose.json\"}}"` (path must be readable by the `compose-store` process).

## Tests run

- `cargo test -p compose-store` (all 5 tests passed after bundle install + `version` projection fixes).
