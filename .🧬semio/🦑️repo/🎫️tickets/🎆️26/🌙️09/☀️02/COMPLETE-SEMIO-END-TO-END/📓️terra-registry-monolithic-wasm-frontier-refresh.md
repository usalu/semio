# Registry Monolithic-WASM Frontier Refresh

Source-only refresh on 2026-09-04. No Rust, WASM, Nx, or catalog-root build was started because two owned native targets remain active.

## Verdict

The acceptance-matrix sentence that says the N=0/2/8 experiment is blocked by *three* removed `DirectoryClient` token calls is stale. The old calls no longer exist in the current Rust source. This removes that particular pre-diagnostic blocker, but it does **not** produce a new linked-core measurement: there is no current N=0/2/8 terminal, core file, function/code count, RSS value, or growth curve that supersedes the 2026-09-03 historical conclusion.

The monolithic `stdio` catalog remains **BLOCKED**. The last admissible full-root result is still the documented `wasm-component-ld`/wasmparser rejection before raw-component/core extraction and before publication. The component limit remains correctly fail-closed at one million defined functions.

## Current token-call cutover

The historical three errors were calls to removed `DirectoryClient::set_token` and `DirectoryClient::mint_session` in the native identity path. A current-source census of Rust under `🧰️framework`, `🌎️hub`, and `✏️s` found zero `DirectoryClient::set_token`, `.set_token(`, or `mint_session(` occurrences.

Current native identity instead has only the class-bound, process-one-shot credential claim:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️identity/🦀️.rs:63-93` stores `{ client_class, Result<Arc<LocalHubCredential>, _> }` in one `OnceLock`, rejects a later differing class, and exposes the result only for the matching class.
- `…/identity/🦀️.rs:98-102` restores identity by retrieving that claim and constructing `DirectoryClient::authenticated`, not by mutating a token.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:605-618` contains the current constructor shape: a transport/base URL plus optional `Arc<LocalHubCredential>`, with `authenticated` as the sole credential-bearing constructor.
- `…/client/🦀️.rs:631-640` reads the protected capability only for the immediate HTTP call. This is an authenticated credential path, not the retired mutable token API.

`DirectoryClient::new` remains only in unit/fake-client paths and MCP cancellation fixtures (`…/client/🦀️.rs:1984,2065,2085,2102`; `…/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:779,866,887`). It has no token argument and begins unauthenticated by design; it is not evidence of the historical compiler failure.

## What is and is not newly measured

`📓️sol-stdio-component-link-boundary.md:28,38-40` remains the only N=0/2/8 evidence record. It says the representative fixture was defined but never emitted a core, because of the then-current three identity errors. No newer report or current registered script contains an N=0/2/8 representative builder or result.

The current stdio script has useful *synthetic* neutral structure coverage, but it does not change that answer:

- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:212-241` independently parses a core's function and code sections, requires equality, applies the fixed one-million limit, and asks `WebAssembly.validate` for parser agreement.
- `…/📜️script.ts:271-318` tests empty, one-function, and declared-1,000,001-function fixture cores and uses Binaryen as an additional oracle.

Those are parser-limit and fail-closed tests. They neither compile N=0/2/8 real component roots nor recover a linked core from `wasm-component-ld`; they must not be relabelled as a real growth curve.

Likewise, the old expansion observation remains only a source-shape observation: the 176-variant `StdioApps` closure was measured as 1,702,133 expanded bytes, 254 `fn` tokens, and 13,728 match arms. It does not establish linked-WASM function cardinality.

## Existing next callable gate

After the active native targets are released, the only existing registered command that can retest the *whole* strict stdio root is:

```sh
NX_SKIP_NX_CACHE=true bun nx run @semio-tech/stdio-plugin:catalog-root -- --build-root <absolute-empty-ticket-owned-directory>
```

It is registered at `…/📜️script.ts:548-650`, rejects the ambient target/cache and a nonempty root (`:533-545`), creates its own isolated Cargo target (`:568-576`), builds the 176-app component (`:580-581`), and rollback-removes staged row/descriptor/target state on any failure (`:635-642`). It has a 1,200,000 ms bound (`:19-29,564-567`). The historic terminal for this same gate is the pre-publication `wasm-component-ld` function-limit failure documented in `📓️sol-stdio-catalog-root-completion.md:31,43`.

This command answers whether the full monolith now reaches a real extracted core and safely publishes; it does **not** supply the missing N=0/2/8 comparison. There is currently no registered N-parameterized linked-core gate. A future measurement packet must first add that bounded, source-owned representative gate and neutral output schema, then obtain three fresh isolated terminals before deciding about sharding or profile changes. It must retain the existing one-million limit and raw/core/descriptor identity checks.

`NX_SKIP_NX_CACHE=true bun nx run @semio-tech/stdio-plugin:test-quick -- catalog-root-contract` remains a lightweight structural/oracle check, but it cannot unblock or measure the component link.

## Remaining independent blockers and nonclaims

- The most recent recorded full stdio test compilation also has the separate 1,852 residual artifact-owner serde diagnostics (`📓️sol-stdio-catalog-root-completion.md:30`). This refresh did not rerun it and does not attribute those diagnostics to the former token calls.
- No current raw component, extracted core, descriptor triplet, registry row, commit marker, native codec receipt, or nonempty `linked_native_codec_bindings()` was observed or claimed.
- Do not alter wasmparser/component-model limits, hand-edit generated registry output, reuse a cached core, or treat the synthetic over-limit core as a successful stdio component.
