# WP-H1b — Schema Validation for Catalog Bind + Test Quick 231/231

Slice: H1b. Ports: 7750-7759. Private cargo target: `.tmp-ticket/wp-h1b/target`.

## Status

| Item | Status |
|------|--------|
| 1. Replace openTarget sniff with schema validation | DONE |
| 2. Fix two test-quick failures | IN PROGRESS — hub document WS restored on scopes path |
| 3. Propagate cargo test exit code from test quick | VERIFY (library already process.exit on fail) |

## Landed

### 1. Catalog bind schema validation
- `resolveTrustedCatalogBindSource` no longer string-sniffs `openTarget` / `openTargets`.
- Candidates are parsed and validated with `hubSchemaExport(..., TrustedBundleV1)`; invalid candidates log a diagnostic and are skipped.

### 2. Document WS 404 (hub-side, aligned with C4d path)
- C4d removed hub `document_ws_v1` / `handle_ws` and pointed tests at `/scopes/{scope}/document/ws` while test `spawn_server` only mounts hub `router()` (no framework merge) — both failing tests got HTTP 404.
- Restored hub document socket on `/scopes/{scope}/document/ws` with grant admission and existing presence/`handle_ws` behavior; scope is `{space}/{document}` (no caller `actor=` query).
- Framework gateway registers `/scopes/.../document/ws` only when `documents` is Some (C4d NoDocumentAuthority design).
- Hub `compose_hub_server` no longer attaches `HubDocumentAuthority` so production merge does not conflict; hub owns the document WS until DocumentAuthority carries presence.

### 3. Exit code
- `runTestBudgeted` already `process.exit(code)` on cargo failure; H1 EXIT:0 was likely a different invocation. Will confirm on full `test quick`.

## Evidence

(pending cargo check / test-quick / bind-readyz)

## Files changed

- hub rust `script.ts` — TrustedBundleV1 validation for published catalog auto-discover
- hub `bootstrap.rs` — scopes document WS + handle_ws restore
- framework server gateway — conditional document/ws registration

## Gaps / peer alignment

- C4d: identity from session on framework DocumentAuthority remains for non-hub products; hub keeps grant-bound handle_ws on the scopes URL (no `actor=` query).
- O3b: policy auto_apply left intact.
