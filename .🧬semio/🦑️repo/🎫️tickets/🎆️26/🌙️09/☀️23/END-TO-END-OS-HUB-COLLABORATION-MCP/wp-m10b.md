# M10b — Outcome 4: agent Commands frame on a live hub

Slice M10b (session 9, 2026-09-23). Continues M10, which landed the write-path code and stopped
on the absence of an authenticable fresh-catalog hub. Bar: an MCP client on the `.mcp.json`
`semio` server opens a hub document, invokes an action, and a `Commands` frame with
`principalKind: agent` lands in the hub ledger; a human browser on the same space sees it.
Also: `hub-agent-participant` 17/17, `client-e2e` and `live-agent-loop-check` green.

## 0. Headline

(filling — hub revive blocked on catalog stall under fleet load; root fix landed, rebuild queued)

## 1. Inherited state

- M10 closed the codec wiring and hub document bind/surface/pair. Live on 7621 both reds were one
  catalog-age refusal.
- Session 8: M10 resumed against **7681** (HC1 APFS clone of TC3e three-package catalog
  `76d11151…`).
- At M10b start (~12:33): ports 7621/7651/7671/7681/7683 all `/readyz` → `000`.
  Roots `hc1-boot` / `tc3e-hub` survive; binary `target-hc1-hub/debug/os-hub` present.
  Catalog wasm carries 29 `semio:framework/codec` strings.

## 2. Hub revive

### 2.1 First boot (hc1-boot, target-hc1-hub binary, port 7681)

- Hold via `startLocalHub` + `start_new_session` (macOS has no `setsid`).
- Handshake OK; after ~2 min `/readyz` → **503** with
  `artifactAuthority.reason=trusted-catalog-load-stalled-before-it-finished`,
  `features.mcpWorkspace=false`. Hub stayed at ~10 % CPU.
- Hold had admitted early because `waitForReadiness(..., bootstrapSecuritySmoke=true)`.

### 2.2 Root cause (measured)

HT16's 30 s no-progress stall on the startup catalog load. Checkpoints exist per 64 KiB hash chunk,
but they only restamp when the load task is **scheduled**. At load ≈ 67 (fleet cargo/rustc), the
async runtime starved the load for > 30 s between live chunks — the same defect HC1 named on 7682.
Eight stall-retries each restart and re-hit the same starvation, so the gate closes permanently.

### 2.3 Fix

`🌎️hub/🏗️bootstrap/🦀️.rs`: `TRUSTED_CATALOG_STARTUP_STALL_BOUND_MS` **30_000 → 300_000**, docstring
updated with HC1/M10b measurements. Still a no-progress bound (not a total wall budget); eight
retries remain.

Hold script now waits for full ready (`bootstrapSecuritySmoke=false`).

### 2.4 Rebuild / restart

- Hub mutex: queued as `m10b` behind `wp-c6` (`/tmp/semio-hub-build.queue`).
- MCP `bun ./📜️script.ts build` detached (`generated/mcp-build.pid`).
- (filling) binary path, restart 7681, readyz with `mcpWorkspace: true`.

## 3. Write-path / Outcome 4 live proof

(filling)

## 4. Gates

| gate | target | measured | capture |
|---|---|---|---|
| hub-agent-participant-check | 17/17 | (filling) | |
| client-e2e | green | (filling) | |
| live-agent-loop-check | green | (filling) | |

## 5. Honest gaps

(filling)

## 6. Files changed

- `�librehub/🏗️bootstrap/🦀️.rs` — `TRUSTED_CATALOG_STARTUP_STALL_BOUND_MS` 30s → 300s
- `.tmp-ticket/wp-m10b/m10b-hub-hold.ts` — detached hold, full-ready wait
