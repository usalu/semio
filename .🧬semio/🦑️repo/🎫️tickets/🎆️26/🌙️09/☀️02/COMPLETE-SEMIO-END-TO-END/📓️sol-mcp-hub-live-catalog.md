# MCP Authenticated Hub Live Catalog

Status: source, neutral oracle, and both exact native laws green on 2026-09-06. This packet closes only trusted live discovery for Hub-backed MCP. It does not claim cold Map bytes, GIS execution, durable approval, undo, or collaborator propagation.

## Authority boundary

- `DirectoryClient` now calls the authenticated, operation-scoped execution-target manifest and descriptor routes with the exact `DocumentOpenIntentV1`. Both response bodies are bounded before decoding; non-success diagnostics remain redacted.
- `HubRemoteBinding` hydrates descriptors only after the authenticated Directory snapshot is ready. Each descriptor must match the document owner, artifact identity, descriptor digest, catalog generation, component hashes, declared length, SHA-256, and canonical Pack projection.
- The retained catalog is tied to the private authority generation. Refresh, stream loss, expiry, member removal, and revocation clear it before another caller can observe a package row.
- `HeadlessWorkspace` exposes Hub-selected descriptors from that retained snapshot. Hub mode has no repository-path or installed-registry fallback; folder mode keeps installed discovery.
- The MCP tool registry rebuilds discovery for every `tools/list` and call observation. The `inference_list`/`inference_get` and catalog search tools carry the exact live selected-package identity in private MCP metadata while authority is ready; revoked/refreshing authority removes it and returns gateway-only discovery tools.
- Inference roster discovery consumes the same `HeadlessWorkspace` descriptor capability rather than independently scanning the local registry.

## Neutral contract

The language-neutral fixture is `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️fixtures/🔐️hub-live-catalog/`. Its JSON Schema fixes three authority states, the exact selected package/component/descriptor identity, a tempting local-only package, and seven hostile substitutions. The independent Bun/AJV oracle rejects every hostile identity and proves that refreshing/revoked states project neither selected nor local-only packages.

## Registered acceptance

- `@semio-tech/framework-os-mcp-rs:hub-live-catalog-check`
- `@semio-tech/framework-os-mcp-rs:hub-live-catalog-native-check`
- launch orders `411.132` and `411.133`

Source receipt:

```text
hub-live-catalog-oracle: AJV=1 states=3 hostile=7 local-fallback=denied
hub-live-catalog-source: workspace=3 remote=4 inference=no-registry-fallback tools=live-selection-projection
NX Successfully ran target hub-live-catalog-check
```

Formatting and generation receipts:

```text
bun nx format:check --files=<seven owned MCP/Directory files>
exit 0
plugin registry generated catalog and launch bytes are fresh.
```

Exact native receipt:

```text
hub-live-catalog-native: laws=2 authenticated-selection=1 revoked-fallback=denied
exact binary: public-member-open-sol-target/debug/deps/semio_framework_os_mcp-0493ac12391c7509
SHA256: 0314af99d1eb67f1a819a70a0fb0f4f0ff1423356e7e5fb9af71266382a37424
NX Successfully ran target hub-live-catalog-native-check
```

The native target released `public-member-open-sol-target` after its terminal GREEN2 result. An earlier six-law catalog gate, `exact-cargo-laws-G9BHZB/00`, was build-red before any law because it captured an intermediate typed-journal recovery API. That target was released, the journal owner confirmed current source coherence, and the failure is not an MCP runtime verdict.

## Remaining P4 frontier

This slice deliberately does not fabricate canonical Map context or mutation authority. A complete P4 journey still requires the authenticated cold Map projection, retained three-Store GIS committer, author-scoped undo handle, and a real two-client/Shell process acceptance path described by `📓️terra-mcp-p4-shared-map-current-acceptance-audit.md`.
