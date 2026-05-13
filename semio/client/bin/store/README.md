# semio-store

`semio-store` is a small **HTTP** service that holds one native [`kit_store::KitStore`](../rs) (WIP + coordinator + optional backbone) and serves the same **GraphQL** control-plane schema as `semio::kit_graphql` (used by `semio/js` and the Rust kit).

- **`POST /install`**: first call installs the sole in-memory kit (e.g. `{ "create": { "dto": { ... } } }`); later calls return `409`.
- **`POST /graphql`**: standard GraphQL JSON body (`query`, optional `variables`, `operationName`). Mutations are nested under `kitStore { batch(input: …) { … } }` (no JSON-RPC surface).
- **`GET /graphiql`** (and `GET /graphql` in the browser): **GraphiQL** for ad-hoc queries.
- **`GET /healthz`**: liveness.
- **`POST /server/shutdown`**: best-effort process exit (dev/tests).

On startup, the first line of **stdout** is a single JSON object with `port`, `semioStoreReady`, and `graphiql` (so tools can discover the bound port when `SEMIO_STORE_PORT=0`).

## Build

```bash
cargo build --release -p semio-store
```

Binary: `target/release/semio-store` (or `semio-store.exe` on Windows).

## Environment

| Variable                 | Description                                                                 |
| ------------------------ | ----------------------------------------------------------------------------- |
| `RUST_LOG`               | `tracing` filter (e.g. `info`, `semio_store=debug`)                        |
| `SEMIO_STORE_PORT`       | TCP port (default `4000`; use `0` to bind an ephemeral port)                |
| `SEMIO_STORE_NO_EVENTS`  | `1` / `true` / `yes` — do not attach the event log thread to the graph        |

See [`AGENTS.md`](AGENTS.md) for architecture notes.
