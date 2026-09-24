# WP-O1b — Hang Fixes for OS Gate Suites

Slice O1b, session 9. Finished 2026-09-23.

## Scope

1. `@semio-tech/framework-os:test` hang
2. `@semio-tech/framework-os-mcp:test` budget kill
3. `@semio-tech/plugin-registry:test` host-activation + budget
4. `@semio-tech/framework-os-host-rs:test` vitest hang after wasm-pack
5. `framework-os-dev` transform-freshness 60s timeouts

## Peer awareness

- O2b: warm-start + transform freshness / serve boot hashes — do not revert; os-dev green attributed to O2b root fixes plus verification here.
- P1/P3: plugin describe/generate (wasm mutex queue); O1b did not wait on mutex for norm — synced checked-in descriptor from leaf DSL examples already registered in Rust `editor_with_examples`.
- C4c: hub suites (untouched).

## Status

| Item | State | Root cause | Fix | Evidence |
|------|-------|------------|-----|----------|
| 1 framework-os:test | PASS | Presence/untracked suites hung: stale WS path/protocol expectations, incomplete FakeHub state (`ingestedMutationIds`), lease expiry waited without `lease.drop()`, uiPatchReceipt as bytes vs WIT `{tag:some\|none}`. Also `resolveTestLevel` defaulted to fundamental (15s wall) so buffered vitest looked like a hang. | Correct FakeHub + fixture WS path/protocol; `lease.drop()` before retire await; WIT option receipt; floor TestScript to `quick`. | `generated/os-official-test.txt` — 7 files, 373 passed, 65.86s |
| 2 framework-os-mcp:test | PASS | Same fundamental 15s budget kill; default reporter buffers until end so size stayed ~303 while tests ran. Not a handle leak (verbose run advanced through e2e; each `openServer` pays catalog compile). | `resolveTestLevel(segments, "quick")` on mcp TestScript. | `generated/mcp-test2.txt` — 7 files, 52 passed, 48.44s; verbose progress `generated/mcp-verbose.txt` |
| 3 plugin-registry:test | PASS | (a) Host-activation expected full `alpha:materialize-dev` fan-out after boot-scoped web prepare. (b) Budget fundamental. (c) `norm` published 30 editor/viewer apps with empty `manifest.examples` while leaf DSL examples already exist under artifacts. | Align host-activation to boot-scoped web prepare; floor registry TestScript to quick; sync norm `descriptor` JSON + pack with 15 dialect examples + recompute `descriptorSha256` via `encodePackValue`. | `generated/plugin-test3.txt` — 8 files, 60 passed / 1 skipped, 254.99s; navbar `generated/plugin-navbar-after.txt`; host-activation `generated/plugin-host-activation2.txt` |
| 4 framework-os-host-rs:test | PASS | Host-rs TestScript runs the same OS vitest config; hung/killed under fundamental 15s, and previously under real OS hangs. After (1) + quick floor, suite completes. | Floor host-rs TestScript to quick; OS hang fixes above. | `generated/host-rs-vitest.txt` — 7 files, 373 passed, 58.98s (wasm pkg already present; no rebuild this verify) |
| 5 framework-os-dev freshness | PASS | Predecessor timeouts were cold-start / serve freshness cost (tree walks + content hash + vite reopt), not a forever-stale transform. O2b rooted warm path (`resolveBootSourceContentHashes`, marker trust, vite cache hit). | Verified only; no O2b revert. | `generated/os-dev-test.txt` — 3 files, 163 passed / 28 skipped, 30.46s |

## Commands

```bash
# OS
cd <os-ts-pkg> && bun ./<script.ts> test
# MCP
cd <mcp-ts-pkg> && bun ./<script.ts> test
# Plugin registry
cd <plugin-registry> && bun ./<script.ts> test
# Host-rs (vitest after wasm pkg exists)
cd <host-rs-pkg> && bun ./<script.ts> test
# os-dev
cd <os-dev-ts-pkg> && bun ./<script.ts> test quick
```

## Files changed (this slice)

- `os/.../space-artifact-creation-owner` tests — FakeHub / WIT / lease expiry
- `os/.../browser-document-open-v1.json` — WS path + actor/surface
- `os/.../store/worker` — connect protocol (aligned with peer)
- `os/packages/ts/script.ts`, `mcp/.../script.ts`, `host/rs/script.ts`, `plugin-registry/catalog-verification` — `resolveTestLevel(..., "quick")`
- `plugin-registry/.../host-activation` test — boot-scoped web prepare expectation
- `repo/library.mjs` — playgroundPreparationTargets bootSelected (shared with O2b; do not revert)
- `plugins/norm/descriptor.json` + `.descriptor.semio` — 15 navbar examples + self-hash

## Notes

- Machine load ~90: distinguish stall vs hang with `--reporter=verbose` and stall-tick size watches; MCP/plugin default reporter stays mute until finish.
- Norm full `describe` (wasm rebuild) was queued behind p1/s14/o3b fleet wasm mutex; JSON/pack sync matches Rust `editor_with_examples` leaf DSL bodies without taking the lock.
- Kill only own pids; ports 6230–6239 / 7730–7739 unused this slice.
