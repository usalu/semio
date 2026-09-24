# WP-C7 — Two Users Collaborating Over The Hub In Real Browsers

Session 10 slice C7. Ports: hubs 7820–7829, serves 6320–6329. Captures `wp-c7/generated/`.

## Status (2026-09-24 04:10) — not 10/10; blocker root-caused

Measured on hub 7820 (H3 binary) + gis2d serve 6320, two headless Chromium contexts (`c7-collab-scenario.mjs`):

| Step | Before C7 (o3c9k) | Now (c7ab/c7af) |
|---|---|---|
| 1a boot, 1b sign-in, 1c both attached | PASS | PASS (1 sustained document socket per user, no storm) |
| 1d rosters, 5 presence distinct colours | FAIL (empty) | **PASS** (both peers in both rosters, distinct hub colours) |
| "Verifying document component…" stuck | stuck | cleared on mount |
| A authors addFeature | refused `action-owner-mismatch` | **guest-applied, 1 mutation, hub persisted** (reload shows Positions 153) |
| B authors addFeature | refused | **guest-applied** (after cold-pair fix) |
| B ingests A's edit live | — | ingested (`MergeReport`, canvas changed) — inspector panel stays stale |
| 2/3 witness | FAIL | FAIL: auto check-in `commitCheckpoint` traps the guest (below) and kills the actor |

**Blocker (guest/framework, not host):** `commitCheckpoint` panics the gis guest —
`resolve_ready: future was not ready on first poll` at `🧰️framework/🔨️modules/🚪️io/🦀️.rs:898` → wasm `unreachable`.
Reproduced on the LOCAL shard lane too (`c7ag`, no hub), so it is independent of collaboration. The shell's auto
check-in (`ShellHost` `autoCheckinSchedulerRef` → `commitCheckpoint`) fires it after every first edit, the trapped child
is closed and every later action is refused. Likely site: `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:783`
`resolve_ready(plugin_complete_reserved_spawned_job(...))` — the reserved-job completion genuinely suspends in wasm
(`commitCheckpoint` is `FrameworkCommitCheckpointJob`, `🔌️plugin/🦀️.rs:16626`). Needs a framework Rust fix + guest
rebuild (W1 owns guests).

Other open items: remote-ingest does not refresh plugin panel bodies (inspector) on the peer; STEP 14 needs a catalog
with writer/draw/puzzle — W1: writer/puzzle currently uncatalogable (`ArtifactPack::record_spec()` missing, W1 §4.2),
draw catalog run 3 in flight (`w1-catalog-c7`).

## Root causes fixed (each measured live, then pinned by tests)

`action-owner-mismatch` was secondary: `dispatchAction` closes the reservation when an invoked action throws anything but
`action-guest-refused`; every later action then sees a closed owner. The real faults, peeled in order:

1. **Command ingress shape** (c7b): WIT `command-ingress-status` is a record `{kind: u8, …}`; worker parsed a retired `{tag}` variant.
2. **History patch** (c7c/c7d): guest encodes `Option<HistoryPatch>` as pack `null` (5 bytes); host demanded 0 bytes.
   Then real patches (c7t: "Clear Selection" row): host refused them → now carried to the Shell (schema-first `historyPatches`).
3. **`Ephemeral` frame** (c7f): appended to every exchange (contract-freeze §C7.6); refused.
4. **`spawn-job`** (c7h): reserved tool verbs are spawned jobs; the actor lane never drove jobs → now `driveSpawnedJob` +
   `job-completed` poll (input copied before transfer, c7k).
5. **Zero-mutation invocation id** (c7j): guest names empty invocations `{verb}:{instance}`.
6. **Unsolicited completion `Invocation{in_reply_to:0}`** (c7m/c7n) and **`OperationCompleted`** (c7r) refused.
7. **`OperationCompleted.revision` u64 precision** (c7q): hash-derived revision > 2^53 decoded as `number` → non-canonical.
8. **Painted revision** (c7v): the Shell mailbox queues one-in-flight and stamps at issue; worker required exact equality.
9. **Pack integers in UI values** (c7v): `invalid intent value integer` refused canonical pack integers.
10. **Execution-target status re-armed after mount** (c7zc): `actor-view`/`actor-ready` emitted after `browser-actor-ui-mounted`.
11. **Typed-operation result pages** (c7zg): `semio.typed-operation-page.v1` bytes are not AppFrames; they must be ACKed.
12. **Typed-operation edits** (c7zi): the edit is owned by an Artifact-lane page, not by an Invocation projection.
13. **Remote ingest `MergeReport`/`Conflicts`** (c7ad): refused on ordinary turns → socket closed ("malformed hub frame").
14. **Cold pair pinned to baseline frontier** (c7ae): every Commands/Ack advances the frontier → admission refused after
    the first live edit (`actor-document-port.admission-refused`). Applied pair now pins ownership only.

Also: empty presence (inherited) was hub-side (framework `HubDocumentAuthority` had no roster replay/leave); H2's hub fixes it.

## Landed (files)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts` — items 1,4,6,8–14; `?surface=` only (H2 contract).
- `…/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts` — history bytes, ephemeral, completion,
  operation-completed, merge-report, unsolicited decoder, zero-mutation rule.
- `…/🎯️action-handoff/🟦️.ts`, `🧬️schema/🔣️.json`, `🧫️fixtures/🔣️.json` — `historyPatches` on the action result (schema-first).
- `…/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts` — `COMMAND_INGRESS_KINDS`; `…/🏗️materialization/🟦️.ts` emits it.
- `…/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — applies browser-actor history patches.
- `…/🔌️PluginRuntime/🟦️.tsx`, `🧰️framework/🛍️products/💻️os/🟦️.ts`, `🧰️framework/🔨️modules/📡️replication/🟦️.ts` — exact u64
  `OperationCompleted.revision` (`readVarintU64Exact`/`writeVarintU64Exact`).
- `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` — `TYPED_OPERATION_LANE_ARTIFACT`.
- `…/🧫️fixtures/📡️channel/🏁️app-frame-operation-completed.json` + `…/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs` — wide-revision vector.
- Tests: `🧪️tests/🧪️backbone-envelope-io/🟦️.ts`, `🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts`, renderer tests (bigint revision).
- Coordinator item `os-hub:browser-document-open-check`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts`
  `viteConfigLoader()`; `🌎️hub/📦️packages/🦀️rust/📜️script.ts` runtime proof uses it, `waitUntil: "commit"`, and the
  current `semio-hub-session-port`/`initialize {capability}` protocol (stale broker-proof relay removed).
- Zero-touch: `…/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts` `ensureTrustedCatalog` publishes through
  `os-hub:trusted-catalog-bootstrap` (packages stdio,gis,note,writer,draw,puzzle) instead of copying session roots;
  `…/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts` seeds from the canonical `.🧬semio/🌐hub/collab-catalog` built the same way.
  Not yet proven on a clean state (catalog publish is W1's wasm lane; writer/puzzle currently fail W1's codec probe).

## Measured

| Check | Result | Capture |
|---|---|---|
| framework-os vitest (all in-source) | **373/373** | `vitest-framework-os-all.txt` |
| framework-os tsc | exit 0 | (inline) |
| `cargo test -p semio-framework-os-kernel --lib app_frame_operation_completed` | 2 passed | `cargo-op-completed.txt` |
| `browser-document-open-check` oracle + runtime + OS test-quick 5/5 + plan oracles | PASS (tail `open-plan-server-check` not run to end: hub cargo, H3) | `browser-document-open-check.txt` |
| two-user scenario c7ab/c7af | 1a,1b,1c,1d,5,1e PASS; A and B author edits; B ingests; blocked by commitCheckpoint trap | `c7ab-*`, `c7af-*` |
| local commitCheckpoint (no hub) | guest panic `resolve_ready` | `c7ag-*` |

## Infra (pids)

- hub 7820: hold `36087` / os-hub `36093` (`wp-c7/bin/os-hub` = copy of `wp-h3/bin/os-hub` 01:29), data `.🧬semio/🌐hub/c7-boot`.
- serve gis2d 6320: `26260` (vite `26487`), `wp-c7/serve.sh gis2d 6320 http://127.0.0.1:7820`.
- Scenario/probes: `wp-c7/c7-collab-scenario.mjs` (settle env, tab-button-only panel clicks, staged addFeature args),
  `wp-c7/c7-probe.mjs`, `wp-c7/c7-local-checkpoint.mjs`.

## Dependency note (04:20)

- H4 (`📓️wp-h4.md`): Store edit/op ids now carry a per-instance replica identity (wasi:random); the hub REJECTS a reused
  id with different content. Current gis guest in `c7-boot` predates this, so two users' first gestures can collide.
  The next C7 run must use W1's rebuilt guests + an H4-era hub binary; until then edit-convergence steps (4, 6, 7) are not
  a valid measurement.
