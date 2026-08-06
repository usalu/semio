# S-Modules Crate Consolidation — Gap Report

**Ticket:** `26/08/06/S-MODULES-CRATE-CONSOLIDATION-AND-NAMING-FIX`  
**Diagnosed:** 2026-08-06 (read-only; `find`/`rg`/`ls`/`cargo`)  
**E2E ticket:** `26/08/06/S-AND-PLUGINS-END-TO-END`

---

## 1. New consolidated crates — workspace membership

| Crate | Path | `name` in manifest | Root `Cargo.toml` member? |
|-------|------|-------------------|---------------------------|
| **semio-s-2d** | `✏️s/🔨️modules/◻2d/📦️packages/🦀️rust` | `semio-s-2d` | **Yes** (line ~153) |
| **semio-s-3d** | `✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust` | `semio-s-3d` | **No** — not in `workspace.members`; `cargo metadata` has no `semio-s-3d` |
| **semio-s-mindmap** | `✏️s/🔨️modules/💭️mindmap/📦️packages/🦀️rust` | `semio-s-mindmap` | **No** — old crate still member instead (see §2) |
| **semio-s-imperative** | `✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust` | `semio-s-imperative` | **Yes** (line ~213) |

**Notes**

- `semio-s-3d` manifest uses `version.workspace = true` but is excluded from the root workspace → isolated `cargo check` fails: *"current package believes it's in a workspace when it's not"*.
- `semio-s-mindmap` manifest still has a **temporary verification overlay** workspace block at the top (ticket comment: delete before registrar handoff).

---

## 2. Old kernel-3d (and related) crates — still members?

All five legacy **3d** split crates are still `workspace.members` and **directories still exist**:

| Legacy member path | Package name (typical) | Dir exists? |
|--------------------|------------------------|-------------|
| `✏️s/🔨️modules/🧊️3d/🎬️scene/⚡️implementations/🦀️rust` | `semio-framework-os-kernel-3d-scene` | Yes |
| `✏️s/🔨️modules/🧊️3d/🥽️mesh/⚡️implementations/🦀️rust` | `semio-framework-os-kernel-3d-mesh` | Yes |
| `✏️s/🔨️modules/🧊️3d/📐️brep/⚡️implementations/🦀️rust` | `semio-framework-os-kernel-3d-brep` | Yes |
| `✏️s/🔨️modules/🧊️3d/📐️brep/⚙️engine/⚡️implementations/🦀️rust` | `semio-framework-os-kernel-3d-brep-engine` | Yes |
| `✏️s/🔨️modules/🧊️3d/🗺️spatial/⚡️implementations/🦀️rust` | `semio-framework-os-kernel-3d-spatial` | Yes |

**Mindmap (not 3d but same consolidation wave):**

| Path | Status |
|------|--------|
| `✏️s/🔨️modules/💭️mindmap/⚡️implementations/🦀️rust` | **Still a member** (~line 152); `name = "semio-s-kernel-reasoning-mindmap"` |
| `✏️s/🔨️modules/📜️imperative/⚡️implementations/🦀️rust` | **Not** in `workspace.members` (imperative handoff partially done) |

◻2d never had explicit old rust members (orchestrator map); only the new `📦️packages/🦀️rust` crate is a member.

---

## 3. `[workspace.dependencies]` and Cargo.toml dependents

### Root `[workspace.dependencies]` (old names)

Still present (~lines 313–314):

- `semio-framework-os-kernel-3d-brep` → old brep path
- `semio-framework-os-kernel-3d-brep-engine` → old brep-engine path

**No** workspace aliases yet for `semio-s-2d`, `semio-s-3d`, `semio-s-mindmap`, `semio-s-imperative`.

### Dependents still on old names (`rg` across `Cargo.toml`)

**By published crate name (any mention in file):**

| Crate name | `#` Cargo.toml files |
|------------|---------------------|
| `semio-framework-os-kernel-3d-scene` | 5 |
| `semio-framework-os-kernel-3d-mesh` | 2 |
| `semio-framework-os-kernel-3d-brep` | 12 |
| `semio-framework-os-kernel-3d-brep-engine` | 11 |
| `semio-framework-os-kernel-3d-spatial` | 1 |

**By legacy dependency keys (`^key = `):**

| Key | `#` manifests |
|-----|----------------|
| `kernel_3d_scene` | 4 |
| `kernel_3d_mesh` | 1 |
| `kernel_3d_brepkit` | 6 |
| `kernel_3d_engine` | 9 |
| `reasoning_mindmap` | 2 (puzzle, reasoning → still path to `💭️mindmap/⚡️implementations`) |
| `imperative_engine` | 2 (sequence, imperative plugin → already `package = "semio-s-imperative"` + new path) |

**New names (partial migration):**

| Name | `#` manifests |
|------|----------------|
| `semio-s-2d` / `semio_s_2d` | draw + flow draw extension repointed to `◻2d/📦️packages/🦀️rust` |
| `semio-s-3d` | 0 dependents (crate not in workspace) |
| `semio-s-mindmap` | 0 dependents |

Full dependent list and per-crate fix notes:  
`/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/S-MODULES-CRATE-CONSOLIDATION-AND-NAMING-FIX/🧭️orchestrator-dependent-map.md`

---

## 4. Root `package.json` workspaces (TS)

| Entry | Status |
|-------|--------|
| `✏️s/🔨️modules/◻2d/📦️packages/🟦️typescript` | **Present** (~line 20) — updated |
| `✏️s/🔨️modules/🧊️3d/📐️brep/⚡️implementations/🟦️typescript` | **Still listed** (~line 21) — old path |
| `✏️s/🔨️modules/🧊️3d/📦️packages/🟦️typescript` | **Does not exist** on disk yet |

Orchestrator target: replace brep TS workspace with `✏️s/🔨️modules/🧊️3d/📦️packages/🟦️typescript` once that package exists.

---

## 5. `cargo metadata` / `cargo check` (top issues)

### `cargo metadata --format-version 1`

- **Exit 0** — workspace resolves; no manifest load failure at metadata time (flow BIM member is `…/🏗️bim/📦️packages/🦀️rust`, which exists).

### Consolidation-specific (blocking s-module targets)

1. `error: package ID specification 'semio-s-3d' did not match any packages` (not a workspace member)
2. `error: package ID specification 'semio-s-mindmap' did not match any packages` (not a member; old `semio-s-kernel-reasoning-mindmap` is)
3. `semio-s-3d` isolated check: *add `✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust` to `workspace.members`*
4. Dual graph: five old `semio-framework-os-kernel-3d-*` members + `semio-s-2d` member; plugins/framework still depend on old 3d crates via `kernel_3d_*` keys
5. Puzzle/reasoning still depend on `semio-s-kernel-reasoning-mindmap` path, not `semio-s-mindmap`
6. Root `workspace.dependencies` still advertise old brep / brep-engine paths (consumers using `.workspace = true` for those keys still resolve to legacy crates)
7. `semio-s-mindmap` Cargo.toml still contains temp nested-workspace overlay (must strip before root adoption)
8. Old rust crate dirs not removed (expected until post-handoff green checks)
9. `semio-s-imperative` is a member but E2E `cargo check -p semio-s-imperative` did not reach imperative code (blocked earlier in graph — see env errors below)
10. No `semio-s-3d` in metadata → any future `path` dep with `package = "semio-s-3d"` cannot be resolved from root until member add

### Environment / proc-macro (blocks **all** s + plugin checks on this host right now)

Repeated for `cargo check -p semio-s-2d`, `-p semio-s-plugin-note`, and E2E sweep (`work/cargo-check/summary.txt`):

11. `warning: failed running "xcrun" "--sdk" "macosx" "--show-sdk-path" to find MacOSX.sdk`
12. `error: linking with cc failed: exit status: 69`
13. `You have not agreed to the Xcode license agreements` (`sudo xcodebuild -license`)
14. `error: could not compile semio-framework-os-kernel-dsl-derive (lib) due to 1 previous error`
15. `warning: semio-framework-os-kernel-dsl-core (lib) generated 1 warning` (unused `len`)
16. `warning: semio-framework-os-kernel-dsl-notation (lib) generated 1 warning` (`print_edge_label` never used)
17. `warning: semio-framework-os-kernel-dsl-derive (lib) generated 1 warning`
18. `warning: build failed, waiting for other jobs to finish...` (parallel check runs)
19. E2E: `semio-s-language-bundle` — `package ID specification did not match any packages` (separate naming/membership issue)
20. E2E: `semio-s-kernel-reasoning-mindmap` check fails on same dsl-derive link error before mindmap-specific compile is validated

**Interpretation:** Workspace **structure** for 3d/mindmap blocks targeted `-p` checks; **compile** validation for s/plugins on this machine is currently dominated by Xcode license / linker, not by s-module source errors (those remain unverified here).

---

## 6. Registrar handoff steps remaining

From `🧭️orchestrator-dependent-map.md` (registrar-only block; orchestrator explicitly does **not** touch root `Cargo.toml` until handoff):

### Root `Cargo.toml` — remove `workspace.members`

- `✏️s/🔨️modules/🧊️3d/🎬️scene/⚡️implementations/🦀️rust`
- `✏️s/🔨️modules/🧊️3d/🥽️mesh/⚡️implementations/🦀️rust`
- `✏️s/🔨️modules/💭️mindmap/⚡️implementations/🦀️rust`
- `✏️s/🔨️modules/🧊️3d/📐️brep/⚡️implementations/🦀️rust`
- `✏️s/🔨️modules/🧊️3d/📐️brep/⚙️engine/⚡️implementations/🦀️rust`
- `✏️s/🔨️modules/🧊️3d/🗺️spatial/⚡️implementations/🦀️rust`
- (imperative old `⚡️implementations` member already absent)

### Root `Cargo.toml` — add `workspace.members`

- `✏️s/🔨️modules/◻2d/📦️packages/🦀️rust` (already present)
- `✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust` (**missing**)
- `✏️s/🔨️modules/💭️mindmap/📦️packages/🦀️rust` (**missing**)
- `✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust` (already present)

### Root `Cargo.toml` — `[workspace.dependencies]`

- **Remove:** `semio-framework-os-kernel-3d-brep`, `semio-framework-os-kernel-3d-brep-engine`
- **Add (optional C4):** `semio-s-2d`, `semio-s-3d`, `semio-s-mindmap`, `semio-s-imperative` path entries

### Root `package.json` `workspaces`

- Replace `✏️s/🔨️modules/🧊️3d/📐️brep/⚡️implementations/🟦️typescript` → `✏️s/🔨️modules/🧊️3d/📦️packages/🟦️typescript` (after TS package exists)

### Pre-handoff (orchestrator / agents, not registrar-only)

- Repoint all entries in `🧭️orchestrator-dependent-map.md` (cad, lowpoly, process, procedural, os, wgpu, renderer, infinite world, flow brep, draw, puzzle, reasoning, sequence, imperative plugin, etc.)
- Delete unused deps (procedural extension `kernel_3d_engine`, flow-core `kernel_3d_engine`)
- Remove mindmap temp overlay from `💭️mindmap/📦️packages/🦀️rust/Cargo.toml`
- Delete old crate directories once new crates match baselines (`cargo check` / clippy / test per ticket)

`S-AND-PLUGINS-END-TO-END/📌️important.md` is empty; orchestrator map is the source of truth for handoff.

---

## 7. Recommendation: E2E registrar handoff now or wait?

**Wait** — do **not** run registrar-only root `Cargo.toml` / `package.json` handoff yet.

**Reasons:**

1. **~22+ manifests** still reference old `semio-framework-os-kernel-3d-*` names or `kernel_3d_*` keys; puzzle/reasoning still use `semio-s-kernel-reasoning-mindmap`. Removing old members before repointing will break resolution immediately.
2. **`semio-s-3d` and `semio-s-mindmap` are not workspace members**; handoff is incomplete (2/4 new crates adopted).
3. **Mindmap consolidated crate** still has the temporary nested-workspace overlay — must be removed before root membership.
4. **3d TS package** at `📦️packages/🟦️typescript` is not on disk; `package.json` still points at old brep TS path.
5. Orchestrator workflow: finish **dependent repointing** (map §1–§4), then **atomic registrar swap**, then delete old dirs.

**Safe parallel work for E2E now:** dependent fixes from the orchestrator map, mindmap overlay cleanup, 3d TS scaffold, local `cargo check` on consolidated crates (after overlay/member fixes) — **not** the final member swap until the map is green.

**Host note:** Accept Xcode license (or use a CI/devcontainer with working SDK) before treating any `cargo check` as proof of s/plugin health.
