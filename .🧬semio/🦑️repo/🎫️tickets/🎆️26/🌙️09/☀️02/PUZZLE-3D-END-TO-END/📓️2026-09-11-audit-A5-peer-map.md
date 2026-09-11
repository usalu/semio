# Audit A5 — Peer Map for Puzzle 3D Waves (2026-09-11)

Read-only. Areas scanned: `✏️s/🔌️plugins/🧩️puzzle`, `🧰️framework/.../📺️renderer/🧑‍🎨engine/🧱️elements`,
`🧰️framework/.../🔌️plugin/📦️packages/🦀️rust`, `🧰️framework/.../🧑‍💻dev`. Commit dates via
`git log --date=iso` (auto-commit bot messages carry a frozen date, ignored).

## Method note on attribution

`--since='2026-09-10 12:00'` on these paths returns only **2** commits: `46c3cb9` (2026-09-11 12:39:02
+0200) and `f39d4b0` (2026-09-10 13:54:54 +0200). Each is an auto-commit-bot sweep bundling *all* live
agents' work repo-wide in one commit (see body: puzzle3d, generation3d/procedural, taxonomy, VS Code
launch, ticket records all in the same commit) — commit-level attribution is meaningless. Attribution
below is per-file, from in-file docstrings citing `ticket 26/MM/DD/SLUG` near the changed lines, or from
directory ownership (`✏️s/🔌️plugins/🧩️puzzle/**` = our plugin's own tree).

## 1. Hot files (touched in ≥3 of the last 8 commits touching these areas)

Last-8-commits window: `46c3cb9` 09-11 12:39, `f39d4b0` 09-10 13:54, `6ad7b0e7bc` 09-10 01:31,
`ebbace9b32` 09-09 17:57, `9b605a4550` 09-09 12:10, `599a5d8450` 09-09 07:58, `5dae35ae71` 09-09 00:35,
`de617a7c17` 09-08 23:25.

### A. Our own tree — `✏️s/🔌️plugins/🧩️puzzle/**` (verdict: **coordinate**)

Not a peer collision by directory, but per `📓️2026-09-10-cursor-coordination.md` / `📓️2026-09-11-claude-coordination.md`
in this ticket folder, cursor-chat and claude-code sessions are both live in this tree concurrently —
coordinate within-ticket, don't treat as free.

| File | Touches/8 | Last commit |
|---|---|---|
| `…/🧧standards/1/subsets/any/editor/🦀️.rs` | 8 | 46c3cb9 09-11 12:39 |
| `…/editor/🧪️tests/🔬️testkit/🦀️.rs` | 7 | 46c3cb9 09-11 12:39 |
| `…/editor/🧪️tests/🔬️unit/🦀️.rs` | 7 | 46c3cb9 09-11 12:39 |
| `…/🗿️artifacts/🧊️3d/🦀️.rs` | 7 | f39d4b0 09-10 13:54 |
| `…/editor/🗣️terminology/🦀️.rs` | 6 | f39d4b0 09-10 13:54 |
| `…/editor/⏳️precompute/🦀️.rs` | 6 | 46c3cb9 09-11 12:39 |
| `…/editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs` | 6 | 46c3cb9 09-11 12:39 |
| `…/editor/🎭️modes/edit/windows/main/🦀️.rs` | 6 | 46c3cb9 09-11 12:39 |
| `…/🗿️artifacts/◻️2d/…/editor/🦀️.rs` (sibling standard) | 4 | 9b605a4550 09-09 12:10 |
| `…/🗿️artifacts/🖐️5d/…/editor/🦀️.rs` (sibling standard) | 4 | 9b605a4550 09-09 12:10 |
| `…/editor/⏳️precompute/🪣️fill/🧪️tests/unit/🦀️.rs` | 4 | 9b605a4550 09-09 12:10 |
| `📦️packages/🟦️typescript/📜️script.ts` (puzzle) | 4 | ebbace9b32 09-09 17:57 |
| `🗿️artifacts/5d/🦀️.rs` | 4 | ebbace9b32 09-09 17:57 |
| `editor/🎚️config/🦀️.rs`, `editor/🪟️window/🦀️.rs` | 4 each | 6ad7b0e7bc 09-10 01:31 |
| `🎮️commands/retained/🦀️.rs` | 4 | f39d4b0 09-10 13:54 |
| `editor/⏳️precompute/📐️geometry/🦀️.rs` | 4 | f39d4b0 09-10 13:54 |
| `editor/panels/artifact/🦀️.rs` (+unit tests) | 4 each | f39d4b0 09-10 13:54 |
| `🗿️artifacts/3d/📦️packages/rust/Cargo.toml` | 4 | f39d4b0 09-10 13:54 |
| ~15 more standards/panels/commands files at 3/8 | 3 | 09-09 07:58 → 09-10 13:54 |

### B. Shared renderer-engine tree — `🧰️framework/…/🧑‍🎨engine/🧱️elements/**` (verdict: **avoid / coordinate — owned mostly by a peer ticket**)

| File | Touches/8 | Last commit |
|---|---|---|
| `🔌️PluginRuntime/🟦️.tsx` | 8 | 46c3cb9 09-11 12:39 |
| `🏛️ShellHost/🟦️.tsx` | 7 | 46c3cb9 09-11 12:39 |
| `🌐️World3dHost/🟦️.tsx` | 6 | 46c3cb9 09-11 12:39 |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | 6 | 46c3cb9 09-11 12:39 |
| `🛠️ShellHelpers/🟦️.tsx` | 6 | 46c3cb9 09-11 12:39 |
| `🗣️Interpreter/🟦️.tsx` | 4 | 46c3cb9 09-11 12:39 |
| `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` | 4 | f39d4b0 09-10 13:54 |
| `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` | 3 | f39d4b0 09-10 13:54 |
| `📃️UiDocumentStore/🧪️tests/typedwire/🟦️.tsx` | 3 | f39d4b0 09-10 13:54 |
| `🏛️ShellHost/dialog-origin/admission/document/🟦️.ts` | 3 | 46c3cb9 09-11 12:39 |
| `🐚️Shell/🧪️tests/wgpu-ui-prefs-themes-i18n/🦀️.rs` | 3 | 9b605a4550 09-09 12:10 |

**`🏛️ShellHost/🟦️.tsx` docstring citations near the hot lines** (grep `ticket 26/09` in-file): lines
896, 1597, 1703, 4479, 5942 all cite `ticket 26/09/09/PROCEDURAL-3D-END-TO-END`; only line 5897 (the
`plugin.handleAction` path) cites our `ticket 26/09/02/PUZZLE-3D-END-TO-END`. **This whole file is
currently driven mostly by the PROCEDURAL ticket** — puzzle-3d waves should treat it as shared/contended,
not free real estate.

### C. `🧑‍💻dev` tooling — verdict: **avoid direct edits; NX-CACHING ticket is actively rewriting these**

`📦️packages/🟦️typescript/📜️script.ts` (4/8, last f39d4b0), `.../vite.config.ts` (3/8), `.../project.json`
(3/8), `🔌️plugin/📦️packages/🦀️rust/📜️script.ts` (4/8, last ebbace9b32) — all part of the shared
cargo-target-dir / nx-cache-output migration (see §3, NX-COMPLETE-TASK-CACHING). Do not hand-edit
`project.json` `outputs` arrays or `buildPluginCargo`/`cargoTargetDirectory` call sites for puzzle work;
extend `📜️script.ts` additively instead and expect churn underneath you.

## 2. Live uncommitted peers right now (`git status --porcelain` + `git diff --stat HEAD`, filtered areas)

8 files, 154 insertions / 68 deletions total. Diff excerpts and attribution:

| File | Live uncommitted? | Attributed owner | Evidence | Verdict |
|---|---|---|---|---|
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📋️project.json` | yes | NX-COMPLETE-TASK-CACHING | adds `"outputs": []` to a cache-enabled target only | **avoid** (don't re-touch; additive-only work by that ticket) |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📋️project.json` | yes | NX-COMPLETE-TASK-CACHING | adds `"outputs": []` / descriptor-file outputs to `fixtures lint` + `describe` targets | **avoid** |
| `🧰️framework/…/🧱️elements/🏛️ShellHost/🟦️.tsx` | yes | PROCEDURAL-3D-END-TO-END | new `pendingRefreshEffects` deferral + `[DEBUG]` logs around `setContributions`/`requestedEffects`, adjacent to the line-4479 `ticket 26/09/09/PROCEDURAL-3D-END-TO-END` comment | **avoid** — active peer edit mid-flight in the exact block; a puzzle-3d change here right now will conflict |
| `🧰️framework/…/🔌️plugin/📦️packages/🦀️rust/📋️project.json` | yes | NX-COMPLETE-TASK-CACHING | `"outputs": []` added to 7 check targets | **avoid** |
| `🧰️framework/…/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts` | yes | NX-COMPLETE-TASK-CACHING | `playgroundCacheDir` switched from `node_modules/.vite-os-dev/*` to `repoCacheDirectory(repoRoot, "vite", "os-dev", …)`, matches memory note "Shared Cargo Build-Dir, Fine-Grain Locking" | **avoid** |
| `🧰️framework/…/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` | yes | NX-COMPLETE-TASK-CACHING | `outputs` added to 4 checks + a `plugin` target output path | **avoid** |
| `🧰️framework/…/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` | yes | NX-COMPLETE-TASK-CACHING | `buildPluginCargo` rewritten to always use shared `cargoTargetDirectory`, drops the `ownedTargetRoot`/private-`CARGO_TARGET_DIR` path, new `stageWgpuPluginModules` fn | **avoid** — large in-flight rewrite of shared build plumbing |
| `🧰️framework/…/🧑‍💻dev/🧪️tests/ticket-owned-browser-host-staging/🟦️.ts` | yes | NX-COMPLETE-TASK-CACHING | test assertions updated to match the `buildPluginCargo` signature change above | **avoid** |

Outside the filtered areas but ticket-relevant: our own `🎫️ticket.json` and two coordination `📓️` notes
under this ticket folder are also modified/added — that's our own session's state, not a peer.

No file in the filtered areas is simultaneously (a) live-uncommitted AND (b) attributable to our own
`PUZZLE-3D-END-TO-END` ticket — the one puzzle-adjacent live edit (`ShellHost/🟦️.tsx`) is PROCEDURAL's.

## 3. Active peer tickets (folders modified in the last 24h)

`find .🧬semio/🦑️repo/🎫️tickets -maxdepth 5 -type d -mmin -1440` (canonical hits, ignoring mojibake
duplicate path spellings of the same folders from earlier encoding drift):

- **`🎆️26/🌙️09/☀️09/NX-COMPLETE-TASK-CACHING`** — newest note `📓️2026-09-11-coordinator-log.md`:
  moving all cargo/nx cache state to one shared root under `.🧬semio/🦑️repo/⚡️cache/` with
  `[unstable] fine-grain-locking`, sccache removed, new `⚡️caching/🦀️cargo/🟦️.ts` (`cargoTargetDirectory`,
  `cargoBuildDirectory`) and `⚡️caching/🟦️.ts` (`repoCacheDirectory`) resolvers. Scope: `.cargo/config.toml`,
  `nx.json`, `policy.json`, every `project.json`'s `outputs`, `📜️script.ts`/`vite.config.ts` build-dir
  call sites repo-wide — this is why puzzle's own `project.json` files show uncommitted diffs above.
- **`🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END`** — newest note `📓️assembly-artifact-mount-2026-09-11.md`:
  mounting a real `s.assembly@1` app (`AssemblyEditor`/`AssemblyViewer`) for the procedural plugin (id
  `s.procedural.assembly`), playground variant `assembly`, ports react **6019** / wgpu **6119**. This
  ticket is the live driver of `ShellHost/🟦️.tsx`'s `setContributions`/effect-deferral logic and most of
  the shared `🧑‍🎨engine/🧱️elements` hot files in §1B.
- **`🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END`** (ours) — newest notes `📓️2026-09-11-claude-coordination.md`,
  `📓️2026-09-10-cursor-coordination.md`: cursor-chat and claude-code both active in the puzzle plugin tree
  concurrently; coordinate within-ticket per §1A.

(Also present but pre-dating the 24h window / not independently active: `🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION` shows only a stale `📓️energy-support-acceptance` subfolder timestamp, no fresh top-level note — not treated as a live peer.)

## 4. Port map (`lsof -iTCP -sTCP:LISTEN`, `ps -eo … | grep serve|vite|nx.js run`)

| Port | PID (leaf → root) | Owner script | Ticket |
|---|---|---|---|
| **6013** | 26789 ← 74413 (ppid 1) | `📜️script.ts serve puzzle3d react release` (vite) | PUZZLE-3D-END-TO-END — stale/background instance, up 1d1h |
| **6014** | 47048 ← 47046 ← 47040 (cursor sandbox zsh, ppid 92663) | `SEMIO_RENDERER=react SEMIO_PLUGIN=puzzle3d … serve puzzle3d react release` (vite) | PUZZLE-3D-END-TO-END — this is the **cursor-chat** peer's live serve (matches `📓️2026-09-10-cursor-coordination.md`), up ~15h |
| **6018** | 71097 ← 71065 ← 71056 (`serve-generation3d-react.sh`, `SCREEN -dmS g3dreact`) | `📜️script.ts serve generation3d react dev` (vite) | PROCEDURAL-3D-END-TO-END, up ~1h |
| **6019** | 49348 ← 49346 ← 49345 (`serve-generation3d-viewer.sh`, `SCREEN -dmS semio-g3d-viewer`) | vite (assembly/viewer variant — matches ticket note's "react 6019") | PROCEDURAL-3D-END-TO-END, up ~1h |

No wgpu (`trunk serve`) instance found on a `60xx` port in this listing; a `trunk serve --port 6118` for
the wgpu renderer is running (PID 45306 ← 45299) but 6118 is outside the `60[0-9][0-9]` grep filter used —
noted for completeness, not double-counted.

None of the listening ports are ours-and-idle: 6013/6014 are both puzzle3d already (ours + cursor's peer
instance — don't start a third), 6018/6019 are procedural's.

## Bottom line for puzzle-3d implementation waves

- **Safe to edit now:** anything under `✏️s/🔌️plugins/🧩️puzzle/**` not listed in §1A/§2 above — but
  coordinate file-by-file with the live cursor-chat session (port 6014, `📓️2026-09-10-cursor-coordination.md`)
  before touching the §1A hot list, since those are the exact files churning every recent sweep.
- **Avoid right now:** `🏛️ShellHost/🟦️.tsx` (PROCEDURAL mid-edit in the effects-deferral block), all
  `project.json` `outputs` arrays and `📜️script.ts`/`vite.config.ts` build-dir plumbing under `🧑‍💻dev` and
  `🔌️plugin/📦️packages/🦀️rust` (NX-CACHING mid-rewrite).
- **Coordinate:** the rest of `🧰️framework/…/🧑‍🎨engine/🧱️elements/**` (PluginRuntime, World3dHost, Shell
  wgpu target, ShellHelpers, Interpreter, EngineCanvas/Scenes wgpu targets) — high churn, mostly
  PROCEDURAL-driven but shared infrastructure puzzle-3d also depends on.
