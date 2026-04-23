# semio-store

`semio-store` is a stdio [JSON-RPC 2.0](https://www.jsonrpc.org/specification) sidecar (NDJSON: one object per line) that exposes the [`semio`](../rs) `KitStore` with the same surface as the wasm `KitStoreHandle` (see `semio/rs/lib.rs` `mod wasm`).

- **stdin**: requests as newline-delimited JSON
- **stdout**: responses and `event` JSON-RPC notifications (no `id`)
- **stderr**: logs (`RUST_LOG` / `RUST_TRACING` via `tracing-subscriber`)
- **One process / one in-memory kit**; call `kit.create` or an `io.import*` method to install the store

## Build

```bash
cargo build --release -p semio-store
```

Binary: `target/release/semio-store` (or `semio-store.exe` on Windows).

## Example

Request:

```json
{"jsonrpc":"2.0","id":1,"method":"kit.create","params":{"dto":{...}}}
```

Response (short):

```json
{"jsonrpc":"2.0","id":1,"result":null}
```

See [`AGENTS.md`](AGENTS.md) for the method list.

## Environment

| Variable                   | Description                                                                                            |
| -------------------------- | ------------------------------------------------------------------------------------------------------ |
| `RUST_LOG`                 | `tracing` filter (e.g. `info`, `semio_store=debug`)                                                    |
| `RUST_TRACING`             | alternate filter name, if `RUST_LOG` is unset                                                        |
| `SEMIO_STORE_NO_EVENTS`   | `1` / `true` / `yes` — do not stream `event` JSON-RPC notifications (avoids stdio backpressure)       |

## Shutdown

`server.shutdown` exits the process after handling the call (or close stdin / EOF).
