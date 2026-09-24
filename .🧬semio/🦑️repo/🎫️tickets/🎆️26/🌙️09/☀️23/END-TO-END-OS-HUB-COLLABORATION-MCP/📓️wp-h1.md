# WP-H1 — Hub Zero-Touch Readiness, Publication Lock, Typecheck

Slice: H1. Ports: 7740-7749. Private cargo target: `.tmp-ticket/wp-h1/target`.

## Status

| Item | Status |
|------|--------|
| 1. Fresh hub `/readyz` ready after bootstrap | PASS (content-hash bind, no wasm) |
| 2. `trusted_publication_owner_process_crash_releases_exact_lock` | PASS |
| 3. `presence-lease-check source` | PASS |
| 4. Hub TS typecheck green + non-zero exit | PASS |
| 5. `bun ./script.ts test quick` / `os-hub:test-quick` | PARTIAL — see below |

## Landed

### 1. Zero-touch trusted catalog bind (lock-order fix)
- Coordinator interrupt: killed inverted hub-mutex job nesting wasm/`trusted-catalog-bootstrap` (`87388`/`96724`/`96728`); hub lock released.
- Rule 9: wasm → hub only; never hold hub around wasm/`trusted-catalog-bootstrap`.
- `materializeTrustedCatalogBundle` short-circuits via `OS_HUB_TRUSTED_CATALOG_SOURCE` or auto-discover of published hub roots (exact package-set preferred; skips legacy singular `openTarget`).
- `bindTrustedCatalogFromPublished`: verify publication + generation content-hash, copy generation, write `current.json` — logs `wasm=skipped`.
- Proof: bind ~2s from `c8-boot`; `/readyz` `status=ready`, `artifactAuthority.ready=true` (port 7748). Evidence: `wp-h1/generated/bind-readyz.txt`, `bind-receipt.json`, `readyz.json`.

### 2. Publication lock
- Root cause: child `fs::write(holder-ready.json)` races parent `exists()` on empty file.
- Fix: `holder-ready.json.tmp` then atomic `rename`. Verified: `ok. 1 passed` in 0.03s.

### 3. Presence lease
- Path `.../fixture/json` → `.../json`; fence `self.refresh_presence(`. EXIT:0, checks=20.

### 4. Typecheck
- Typed GIS/VCS parses; `creation.ready` narrow; `dataClass: "persistedShared"`; runtime `*.d.mts`→match mjs; `runBunxStatus` non-zero exit. EXIT:0.

### 5. Test quick
- Root `bun ./script.ts test quick` → EXIT:0 (Nx prerequisites).
- Hub mutex used only for `os-hub:test-quick` (hub-only cargo).
- Unblocked compile: `IoFidelity::Lossless` → `Exact` (32 plugin files; enum no longer has Lossless).
- Fixed vacuous stall law: creation stall test `steps` used integer truncation (`1` step of 90s < 180s bound) → `div_ceil(...).max(2)`.
- Latest hub run: **229 passed, 2 failed, 9 skipped** / 231 run (`generated/test-quick.txt`). Remaining failures outside H1 bind work: `an_unauthenticated_document_route_is_refused`, `admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen`.

### Build unblocker
- Reactor `PROCESS_POOL_*` consts moved above use (peer mid-edit).

## Evidence
- bind+readyz: `wp-h1/generated/bind-readyz.txt` — `H1_BIND_READYZ_OK`, BIND_MS≈2071, wasm=skipped
- publication: `wp-h1/generated/publication-lock-test.txt`
- presence: `wp-h1/generated/presence-lease-source.txt`
- typecheck: `wp-h1/generated/ts-typecheck.txt`
- test quick: `wp-h1/generated/test-quick.txt` — 229/231

## Files changed
- hub rust `script.ts`: catalog bind/auto-discover; `import.meta.main` guard; exports; presence; typecheck-related
- hub publication test (atomic ready handshake)
- hub-ts typecheck exit; GIS/VCS codecs; runtime dts rename; reactor consts
- VCS/plugin txt IO: `IoFidelity::Exact`
- creation stall unit test steps ceil
- scratch: `wp-h1/h1-bind-readyz.ts`
