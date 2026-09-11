# wgpu contributions pack sender — 2026-09-10

Correction of the earlier attribution: empty windows and `invokeExtension` 0× on wgpu are **not** the React blank-body delay. React pushes once (397 921 chars, 99 pages, ≈202 s). wgpu **never sent** until this lane (`🗑️generated/wgpu-boot-14-console.jsonl`: 24 records, zero `contributions`, 29× `effects=0`). The React sender in `🏛️ShellHost/🟦️.tsx` was read only and not edited.

## What this lane owns

wgpu's **send** path, not the React push:

- `🐚️plugin-bridge.ts`: `primeContributionManifest`, `wgpuBuildScopedContributionsPack`, `pushScopedContributions`
- One `.wit` pack crossing. **Not** 99-page 4 KiB string paging (boot-13 §5–§6).
- Scope: receiver (procedural) plus flow-graph-reachable operators (brep via `manifestJson` `id`s), not an allowlist and not all thirteen plugins.
- Slim view: keep `locale` / `terminology` / window roster; **drop** live `contributionsJson` so the pack is the only bulk field (that fat view is what blew the 64×4 KiB command-ingress ceiling on boot-16).

Guest admission (procedural, this lane after the 12 288-byte reject):

- `GENERATION3D_CONTRIBUTIONS_RAW_BYTES` / viewer twin = `PUBLIC_INVOCATION_BODY_BYTES` (262 144).
- Still `page: 0, pageCount: 1` so the addressed schema matches.

## Probes (real numbers, probe not edited)

Reuse: `bun 🔍️browser-probe.ts --url=http://127.0.0.1:6118/?plugin=generation3d --label=wgpu-boot-N --settle=260`. Probe early-settles ≈9 s.

| Label | Console | Pack | Outcome |
| --- | --- | --- | --- |
| boot-15 | 22 | skipped `empty-or-unscoped` | Sender fired; brep not primed / kinds not extracted |
| boot-16 | 20 | **190 719 chars / 191 535 B**, crossings 1, encoding pack | `command ingress exceeds 64 pages` (fat view + pack) |
| boot-17 | 24 | same 190 719 | Past ingress; `missing field locale` (`{}` too slim) |
| boot-18 | 22 | empty | Trunk failed wasm rebuild wiped `extension-modules` |
| boot-19 | 23 | 190 719 / 191 535 | `tool factory … rejected 191550 raw bytes … maximum is 12288` |
| **boot-20** | **28** | **190 719 / 191 535, 1 crossing** | **`contributions installed`**. Guest emitted 2× `dispatch-action` (dropped unmapped). Hardcoded `flowEvalTick` then `decodeAppFrame: unknown tag 115`. `invokeExtension` **0×**. windows `[]`, meshes **0**. `effects=` all 0 on pre-push renders. `RuntimeError: memory access out of bounds` in page-faults (unchanged class, not chased). |
| boot-21 | 21 | empty-or-unscoped | Trunk wiped brep overlay again (`flow-extension-brep` `🔚.json` → HTML) |

**Settled payload (boot-20):** 190 719 chars, 191 535 UTF-8 bytes, **one** pack crossing. Scoped brep+procedural (React's 397 921 / 99 pages is the unscoped thirteen-plugin string path). Reachability JSON from examples was 257 704 chars; that is **not** sent in the command.

## What followed

- **Registry:** install ran (`pageCount=1` → `sync_host_flow_extension_contributions_page`). That is a non-empty push, not a no-op skip.
- **`effects` / `invokeExtension`:** guest re-armed via `dispatch-action`; wgpu `wireEffectToFriendly` dropped the tag on boot-20. Mapping + leftover re-dispatch landed after boot-20; boot-21 could not re-verify because brep vanished from 6118. **No `invokeExtension` on any wgpu probe this session.**
- **Windows / geometry:** never appeared on 6118. wgpu is **not** first-to-geometry.

`dispatch-action` is now mapped in `🖼️wire-turn.ts`. Leftover `dispatchAction` is re-entered as `handleCommand`. That path is unverified on a live primed boot after the map.

## World suite (not this lane)

`🗑️generated/wgpu-world-suite.txt` (`test result: FAILED. 105 passed; 10 failed` in `world::tests::*`, mtime 04:30) is **session 1's log**. Not re-run here. Already on the punchlist. Do not treat as a regression of this sender.

## Suite

`🔬️wgpu-extension-dispatch`: **8 passed / 8** (was 7). New law: slim-view pack ingress ≤ 64 pages; fat `contributionsJson` view exceeds it. Last run 2.33 s (`🗑️generated/wgpu-ext-ingress-vitest.txt`).

## Constraints honored

- 6118 only. 6018 not touched. Probe not edited. `🏛️ShellHost` not edited.
- No git-modifying commands. Disk: existing cargo/nx dirs only.
- Trunk watch on the wgpu rust package **wipes** `🧩️extension-modules` on failed overlay rebuilds. Brep must be re-copied after those events; boot-18/21 are that, not a sender logic revert.
