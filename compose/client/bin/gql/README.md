# compose-gql

`compose-gql` is a small **HTTP** service that holds one native [`ParentStore`](../../lib/rs/lib.rs) and serves the same **GraphQL** control-plane schema as the Rust kit.

- **`POST /install`**: first call installs the sole in-memory kit. Body is exactly one of `create`, `importFile`, `importFromFolder`, `importFromZip`, `importFromRemote`. **`create.dto`** may be either a bare **`initialKit` projection** (id, name, types, designs, …) or a full **`DevBackboneBundleDoc`** JSON whose `schema` matches the kit-store bundle marker. The split metabolism fixture now uses `stores/metabolism/wip/initialKit/kit.semio.json` as the canonical JSON entrypoint. **`importFile`** reads UTF-8 JSON from `path` and uses the same rules. Later calls return `409`.
- **`POST /graphql`**: standard GraphQL JSON body (`query`, optional `variables`, `operationName`). Mutations are nested under `kitStore { batch(input: …) { … } }` (no JSON-RPC surface).
- **`GET /graphiql`** (and `GET /graphql` in the browser): **GraphiQL** for ad-hoc queries.
- **`GET /healthz`**: liveness.
- **`POST /server/shutdown`**: best-effort process exit (dev/tests).

On startup, the first line of **stdout** is a single JSON object with `port`, `composeGqlReady`, and `graphiql` (so tools can discover the bound port when `COMPOSE_GQL_PORT=0`).

## Build

```bash
cargo build --release -p compose-gql
```

Binary: `target/release/compose-gql` (or `compose-gql.exe` on Windows).

## Environment

| Variable           | Description                                                  |
| ------------------ | ------------------------------------------------------------ |
| `RUST_LOG`         | `tracing` filter (e.g. `info`, `compose_gql=debug`)          |
| `COMPOSE_GQL_PORT` | TCP port (default `4000`; use `0` to bind an ephemeral port) |

See [`AGENTS.md`](AGENTS.md) for architecture notes.
