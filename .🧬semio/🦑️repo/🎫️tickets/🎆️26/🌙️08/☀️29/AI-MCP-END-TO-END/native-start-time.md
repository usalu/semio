# Native `semio-os-mcp stdio` start time

## Measurement (before)

`sample` of the prototype binary (`.cache/b/r/m/u/l/q0c9bae/t/a3dd0102.../prototype/semio-os-mcp`, 673 MB, 504 capabilities) driven by `initialize` + `tools/list` over stdio with `SEMIO_PLUGIN_CATALOG` set. Under `sample` the start took 13.5 s (8.7 s unsampled); percentages are of the 971 samples under `server_for_workspace_options` on the serving thread (the only thread that works at start, the 9 pool workers idle):

| Phase | Samples | Share |
| --- | --- | --- |
| `discover_catalog_source` > `RegistryDiscovery::scan` > `load_package_descriptor` (serde_json into `PackageDescriptor`, sequential) | 612 | 63 % |
| of which file reads (`read_to_string`) | ~57 | 6 % |
| `find_repo_root` (`is_repository_root` JSON parse per ancestor) | 43 | 4 % |
| `catalog::compile` | 238 | 25 % |
| of which `compute_catalog_entries_hash` (`serde_json::to_vec` over all entries, sequential) | 166 | 17 % |
| of which entry building (`capability_from_action`, `framework_capabilities`) | ~70 | 7 % |
| everything else (arg parsing, policies, dyld, 0.77 s without a catalog) | rest | |

`initialize` was answered only after the whole `server_for_workspace_options` returned, because the server (and its tool registry built from the catalog) is constructed before the transport reads the first line.

## Changes

1. `initialize` no longer waits for the catalog: `StdioTransport::answer_handshake_while_building` (stdio transport) answers `initialize`, `server/discover`, `notifications/initialized` and `ping` from a bare `McpServer::with_defaults()` on a helper thread while `run_stdio_serving` builds the real server on the serving thread. Any other line is pushed back on the connection's deferred queue (arrival order kept, so `tools/list`, `tools/call` and the rest wait for the real server), and `serve_after_handshake` replays the answered handshake requests silently into the real server so its negotiated version, era, client features and `initialized` flag are identical. Cancellation is untouched: `notifications/cancelled` is still intercepted by the reader thread, progress and cancellation live in the real server.
2. Descriptor loading runs on the in-repo process worker pool (`pool_map` in the catalog crate, order preserving, inline on a pool worker or single core): `RegistryDiscovery::scan` loads every plugin descriptor (read and parse) in parallel; paths, diagnostics and the final sort stay in registry order.
3. The catalog hash is computed per entry in parallel on the pool: `hash = blake3(join("\n", [blake3(json(entry)) for entry in sorted entries]))`. Semantics (documented on `Catalog` and `compute_catalog_entries_hash`): it changes exactly when an entry's content, the entry count or the entry order changes, and is independent of the worker count. No file bytes are hashed, because the catalog also depends on gateway capabilities, locale and terminology that no publication file holds.
4. `server_for_workspace_options` builds the catalog once for a `--folder` workspace and reuses the same `Arc` for the shell route instead of compiling it a second time.
5. `StdioRead`, `StdioLines::read_line_direct_within` and `StdioLines::defer` became `pub` (consent crate) for the handshake helper. `semio-framework-async` was added to the catalog crate (in-repo path dependency, `Cargo.lock` edited by hand).

## Tests

- `the_catalog_hash_changes_exactly_when_the_catalog_content_changes` (registry quick tests): same content, same hash; hash is a pure function of the sorted entries; one changed character or one removed entry changes it.
- `parallel_descriptor_loading_keeps_registry_order_for_paths_and_diagnostics` (registry quick tests).
- `the_handshake_is_answered_before_the_server_exists_and_replayed_silently_into_it` (transport quick tests).

## Verification status

Nothing was compiled to completion: the only type-check (`check -p semio-os-mcp-catalog --lib`) was stopped by the coordinator while still building dependencies, so none of the edits (catalog crate, registry scan, stdio transport, `run_stdio_serving`) is type-checked and none of the three new tests has run. Known risks for the first compile: `PackageDescriptor`/`GatewayError` must be `Send + 'static` for `pool_map`; `Option::is_none_or` and the `pub` visibility changes in the consent crate; `Cargo.lock` was edited by hand (one added `semio-framework-async` line).

No post-change timing exists. Projection from the sample (unmeasured): `initialize` returns in about the 0.77 s no-catalog baseline plus handshake latency (target < 1.5 s) because it no longer waits for the catalog; the catalog itself should drop from ~8 s to roughly 2 to 3 s (descriptor parse and hashing spread over 10 cores, one catalog build instead of two for `--folder`), with `tools/list` and later calls waiting for it. Remaining sequential cost: entry building in `compile` (~7 %) and `find_repo_root` (~4 %). Third-party hash cross-check is not done (the workspace has no external blake3 crate; the hash is the in-repo `framework_hash`).
