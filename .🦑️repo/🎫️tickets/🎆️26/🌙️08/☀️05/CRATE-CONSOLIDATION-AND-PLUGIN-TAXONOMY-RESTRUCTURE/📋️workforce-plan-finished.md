# Implementations-Folder Eradication & Package Consolidation — Finished Workforce Plan

> **Status:** FINISHED & DISPATCH-READY (reconciled 2026-08-06 ~12:00 against live repo).
> **Master ticket:** `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`
> **Goal:** `🎯aioptimizedrepo` (no goal lifecycle actions without explicit instruction)
> **Supersedes:** the 2026-08-05 draft of this file and any W2-era wave numbering in older master.md sections.

---

## ⚡ SHAPE V2 (normative — unchanged)

Inside every owner's domain tree, **only three things may exist**: `component.<ext>` files, the `📦️packages` folder, and `<component>` folders. No `⚡️implementations`/`⚡️implementation`, and no files with any other name.

```
<owner>/
  📦️packages/🦀️rust/{Cargo.toml, 📋️project.json, 📜️script.ts, build.rs?, lib.rs}
  📦️packages/🟦️typescript/{package.json, tsconfig…, index.ts}
  📚️examples/ 🧫️fixtures/ 🤖️generated/ 🛂️manifest.json  AGENTS.md README.md   # owner-root ONLY
  <component>/…/<component>/🦀️component.rs (+ 🟦️component.ts / 🐹️component.go / 🐍️component.py siblings)
```

Rules (clarified, binding):
- (a) `📦️packages` = packaging code ONLY (manifests + bundling entry `lib.rs`/`index.ts`).
- (b) data dirs live at **owner root**, never inside packages.
- (c) in-tree `AGENTS.md`/`README.md` relocate to owner root content-untouched; on collision, flag — never merge.
- Leaf variants (`config.rs`, `terminology.rs`, topic files) → `<folder>/🦀️component.rs` with vocabulary emoji; module names unchanged; only `#[path]` strings change.
- `#[path = "."]` grouping resets are **NOT** prefixed with `../../` after lib.rs relocation (leaf paths only).

Finalization tripwire: `find` for any in-tree file not matching `component.<ext>` / root-data / root-docs / packages allowlist = 0, AND zero `⚡️implementations` dirs.

---

## Reality check (2026-08-06) — what the Aug-5 draft got wrong

| Claim in Aug-5 draft | Live truth 2026-08-06 |
|---|---|
| W2 plugin fan-out still mid-flight (~10 migrated) | **All plugins migrated** (registry ~38 plugin crates). `🏗️fem` still has **17 residual `⚡️implementations`** under apps/modules (packages already present — finish+delete). |
| Shape V2 broadcast still pending | **Broadcast closed.** Most retrofit tickets closed. Still open: `WRITER-…-RETROFIT`, `REASONING-…-RETROFIT`. Stale open migration tickets: `SPACE-…`, `RASTER-…` (packages already exist — close or mop). |
| W4 mechanisms not started | **Mostly landed:** vocabulary+discovery, nx plugin v2+mjs move, registry refactor, bun workspaces generator, storybook generator, ts-rs retarget, dep-cruiser modernization, hub dress rehearsal — all **closed**. |
| W5d demonstrator last | **Demonstrator closed.** |
| W6 residuals not started | **Partial:** flow TS/ext + cad residual **closed**. Still open: sourcing extensions, trinity jack tools. Other plugin residuals (imperative 5 ext crates, process/playbook leftover dirs) untracked as tickets. |
| W7/W8 not started | **In flight:** open tickets for s-modules, framework math/ui/surface/assets/singletons+core. Hub rehearsal **closed**. |
| ~584 implementations dirs | **~201 remain** (framework/os dominant). |
| ~536 cargo members | **~227 members** (still above ~61 target). |

**Human coordination blockers (unchanged):**
- `26/08/05/COMPILER-MODULE-CALL-SITE-SWAP-AND-TYPST-EVICTION` — gates compiler-family (W8b).
- `26/08/05/CONVERGING-FLOW-EVALUATION-AND-EXPLICIT-NODE-STATUS` — flow-plugin overlap risk; snapshot-and-flag.

**Session note:** repo MCP is not connected in some agent sessions — do not invent ticket IDs; reopen via MCP when available. Scratch only under ticket folders. No git-modifying commands.

---

## Target architecture (unchanged end-state)

**Package-survival rule:** a package survives only if consumed as-a-whole from outside its tree (dynamically-loaded wasm, installable artifact, or library with external consumers). Everything else becomes co-located `component.<ext>` files.

| Kind | Final count | Notes |
|---|---|---|
| Rust crates | **~61** | 32 plugins + 4 plugin extensions + 8 os flow-modules + 12 framework + 4 s-kernels + 1 hub |
| npm packages (authored) | **~26** | framework/os + repo + print + s + plugin-js; wasm `pkg/` is generated |
| Go modules | 4 | cli, mcp, lib, coordinator — **full domain-tree split** |
| Python | 1 | `@semio-tech/ui-styling-py` |
| LaTeX | 0 packages | manifest-less components under `📓️print/🖋️latex/` |

Key merges, deletions, architecture risks: retain from prior plan (os kernel pivot, Go live-infra C8, nx atomicity, wasm names frozen, C4 workspace deps). Compose remains exempt except path repoints.

---

## Remaining inventory (dispatch map)

```
~201 ⚡️implementations|⚡️implementation dirs left

  92  💻️os/**                 → W8c (kernel/host/dev SERIAL)
  52  🧮️math/**              → W8b math family (open ticket)
  17  🏗️fem/**                → W5-finish (open ticket; packages exist)
   8  🦑️repo/**               → W8d
   6  📚️compiler/**          → W8b (BLOCKED on human compiler ticket)
   5  🗺️surface/**            → W8b (open ticket)
   5  📜️imperative/ext/**     → W6 residual
   5  ✏️s/🔨️modules/3d+mm     → W7 (open ticket)
   1 each: assets, hash, schema, editor, framework-root, os-root,
           print, mindmap leftover?, process app leftover, playbook ext,
           mit-bestand ×2
```

---

## Mechanism ledger (W4) — close the gaps, don't redo

| Step | Ticket / artifact | Status | Residual work |
|---|---|---|---|
| M1 vocabulary+discovery | `MECHANISM-VOCABULARY-…`, `TAXONOMY-VOCABULARY-…`; files still under repo-lib `⚡️implementations/🟦️typescript/` | **done functionally** | Move those files into Shape V2 packages during W8d (repo product) — do not rewrite consumers twice |
| M2 nx plugin v2 + `.mjs` move | `NX-PLUGIN-V2-…` | **closed** | Freeze `nx.json` — registrar-only thereafter |
| M3 registry refactor | `REGISTRY-SCRIPT-REFACTOR-…` | **closed** | Ensure `role=framework|product|hub|s-module|extension` discovery feeds catalogs |
| M4 root policy | (partial via earlier revival) | **gap** | Extend `package-shape/*` + area-map `legacy|mixed|clean|exempt` beyond plugins; warn→error at W10 |
| M5 Cargo globs | registrar incremental | **partial** (~227 literals) | Adopt per-area globs only when area has ≥1 `📦️packages/<lang>`; never zero-match |
| M6 bun workspaces | `GENERATED-BUN-WORKSPACES-…` | **closed** | Regen every registrar pass touching TS |
| M7 dep-cruiser | `DEPENDENCY-CRUISER-CONFIG-MODERNIZATION-…` | **closed** | Promote `no-impl-segment` warn→error at W10 |
| M8 ts-rs | `TS-RS-TYPEGEN-DRIVER-RETARGET-…` | **closed** | — |
| M9 storybook | `GENERATED-STORYBOOK-…` | **closed** | Regen with ui-react move (C9) |
| M10 periphery | not opened | **todo W9** | go.work final, devcontainer, gitignore, vitest KNOWN_BROKEN, ~39 md |
| M11 project.json sweep | not opened | **todo W9** | needs M2 (done) |
| M12 finalization flip | not opened | **todo W10** | delete legacy paths; permanent tripwire policy |

---

## Workforce waves — REMAINING ONLY

```
W5f fem-finish + stale-ticket mop ─┐
W6r residual mop (imperative/sourcing/trinity/process/playbook) ─┼─→ W7 s-modules
                                                                   │
W8a hub ✅ ─→ W8b families (math∥ui∥surface∥assets∥singletons; compiler GATED)
         ─→ W8c os SERIAL (data → host → dev) + registrar between
         ─→ W8d repo+Go ─→ W8e print ─→ W8f mit-bestand
W7 + W8f ─→ W9 periphery+project.json ─→ W10 finalization ─→ W11 verification
```

Hard constraints (carry forward): C1–C10 from prior plan, especially **C4** workspace deps, **C7** compiler gate, **C8** Go build-alongside, **C9** ui-react+storybook same pass.

### Wave ownership locks (live — do not collide)

| Area | Owner ticket (open) | Lock |
|---|---|---|
| `✏️s/🔌️plugins/🏗️fem/**` | `…/FEM-PLUGIN-MIGRATION-…` | exclusive until closed |
| `✏️s/🔌️plugins/✒️writer/**` (retrofit) | `…/WRITER-PLUGIN-SHAPE-V2-…` | exclusive |
| `✏️s/🔌️plugins/💡️reasoning/**` (retrofit) | `…/REASONING-PLUGIN-SHAPE-V2-…` | exclusive |
| `✏️s/🔌️plugins/🔱️trinity/**` residuals | `…/TRINITY-PLUGIN-RESIDUAL-…` | exclusive |
| `✏️s/🔌️plugins/🪵️sourcing/**` extensions | `…/SOURCING-PLUGIN-EXTENSIONS-…` | exclusive |
| `✏️s/🔨️modules/**` | `…/S-MODULES-CRATE-CONSOLIDATION-…` | exclusive |
| `🧰️framework/🔨️modules/🧮️math/**` | `…/FRAMEWORK-MATH-FAMILY-…` | exclusive |
| `🖱️ui/**` + renderer pair | `…/FRAMEWORK-UI-FAMILY-…` + `UI-ELEMENT-CO-LOCATION-…` | **serialize these two** — UI-ELEMENT is the split follow-up; family consolidates packages first |
| `🗺️surface/**` | `…/FRAMEWORK-SURFACE-FAMILY-…` | exclusive |
| `🖼️assets/**` | `…/FRAMEWORK-ASSETS-FAMILY-…` | exclusive |
| singletons+core | `…/FRAMEWORK-SINGLETONS-AND-CORE-…` | exclusive |
| `S-AND-PLUGINS-END-TO-END` | open | verification/orchestration — do not steal owned trees |
| root `Cargo.toml`/`package.json`/`go.work`/`nx.json` | **registrar only** | serialized |

Stale opens to resolve in W5f (packages already present): `SPACE-PLUGIN-MIGRATION-…`, `RASTER-PLUGIN-MIGRATION-…` — verify green + `ticket_close`, or reopen only if residual found.

---

## Agent roster & prompts (copy/paste)

Shared preamble for EVERY agent:

```
You are a senior developer in the semio monorepo.
- Work ONLY inside your ticket path. Scratch files only there.
- NEVER git commit/stash/checkout/worktree. Never edit root Cargo.toml / package.json workspaces / go.work / nx.json (registrar owns them).
- Shape V2 only. TEMPLATE.md (+ TEMPLATE-TS/EXT/FAMILY/GO as relevant) is law.
- Synchronous verification only (300–600s timeouts). Never background cargo/bun.
- End with registrar-handoff JSON block: {owner, ticketPath, newCrates, oldMemberLines, workspaceDepRenames, crossDepsFlagged, residualsDeferred, tests:{baseline,now}, wireProof, status}.
- Snapshot-and-flag foreign dirt; do not "fix" concurrent humans' work.
- Goal association: 🎯aioptimizedrepo — do not open/close goals.
```

### A0 — Orchestrator (this session / master ticket) — continuous
1. Keep `📋️master.md` wave boxes truthful.
2. Red-gate: if `cargo metadata` or workspace check fails for reasons outside an owned in-flight ticket, dispatch forward-fix before further fan-out.
3. Registrar passes every ~4–8 landings.
4. Close master only after W11.

### A1 — FEM finish (strong) — NOW
Ticket: reopen/continue `26/08/05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`
Exclusive: `✏️s/🔌️plugins/🏗️fem/**`
Task: collapse remaining 17 `⚡️implementations` into Shape V2 tree under existing `📦️packages/{🦀️rust,🟦️typescript}`; match test baseline exactly; wire byte-proof; delete old impl dirs only after green; registrar-handoff.

### A2–A3 — Shape V2 retrofit closers (2 light/medium) — NOW ∥ A1
- Writer retrofit ticket; Reasoning retrofit ticket.
- Re-verify with real `cargo check -p …` (not manual-only) for `#[path="."]` prefix bug.
- Close tickets when clean.

### A4 — Stale migration ticket mop (1 light) — NOW
For SPACE + RASTER open migration tickets: confirm packages+tests green, zero impl dirs, then `ticket_close` with summary. If residuals, convert into residual mop tickets instead of leaving stale opens.

### A5–A8 — W6 residual mop (∥ after A1 or dir-disjoint now)
| Agent | Ticket | Scope |
|---|---|---|
| A5 | `SOURCING-PLUGIN-EXTENSIONS-DE-SANDWICH` (open) | extensions stay packages; Shape V2 de-sandwich |
| A6 | `TRINITY-PLUGIN-RESIDUAL-MOP-UP-JACK-TOOLS` (open) | jack tools; LSP stays installable package |
| A7 | new `IMPERATIVE-PLUGIN-EXTENSIONS-DE-SANDWICH` | 5 extension impl dirs → packages or fold per TEMPLATE-EXT |
| A8 | new `PROCESS-PLAYBOOK-LEFTOVER-IMPL-DIRS` | process `🎛️apps/3d` leftover + playbook procedural ext |

Pilot recipes already exist: `📋️TEMPLATE-TS.md`, `📋️TEMPLATE-EXT.md` (flow/cad).

### A9–A10 — W7 s-modules (2 agents, after residuals that touch s-deps or ∥ if dir-disjoint)
Ticket: `26/08/06/S-MODULES-CRATE-CONSOLIDATION-AND-NAMING-FIX` (open)
9→4: `semio-s-2d`, `semio-s-3d`, `semio-s-mindmap`, `semio-s-imperative`; kill wrong `semio-framework-os-kernel-3d-*` names; npm `@semio-tech/s-{2d,3d}-js`.

### A11+ — W8b families (∥, dir-disjoint; strongest models on math+ui)
Already-open tickets — **do not open duplicates**:
- Math (strong): delete 25 stub crates after dependents grep; merge 52 dirs → `semio-framework-math` + `@semio-tech/framework-math`
- UI (strong): package consolidation first; coordinate with `UI-ELEMENT-CO-LOCATION-RESTRUCTURE` (serialize: packages before element split, or explicit handoff in master.md)
- Surface: 5→1 + single wasm wrapper; lockstep renderer-react imports
- Assets: de-sandwich
- Singletons+core: hash/editor/schema + framework-core
- Compiler: **DO NOT DISPATCH** until human compiler ticket closes (C7)

Hub rehearsal ✅ — use `📋️TEMPLATE-FAMILY.md`.

### A-OS1..3 — W8c SERIAL (3 strong + registrar between)
1. **Data plane:** store(313)+spr(rename protocol)+dsl+pack+infinite+flow-core+db → `semio-framework-os-kernel` at `💻️os/`; `wasm32-wasip2` admission BEFORE old-crate deletion.
2. **Host plane:** `💻️os/🖥️host/` absorbs host-only modules; SDK stays `🔌️plugin/`; flow-modules remain separate packages.
3. **Dev plane:** os-dev de-sandwich; delete 399MB `🔌️plugin-modules` + gitignore.

With C4 done, dependent repoints collapse to root `[workspace.dependencies]` edits (registrar).

### A-GO1..2 — W8d (sequenced)
1. Go pilot on smallest module → author `TEMPLATE-GO.md` (missing today).
2. CLI domain split (69k `main.go` → per-domain packages); `go.mod` at owner roots; `📦️packages/🐹️go/` holds project.json+script only.
**C8 protocol:** build alongside → MCP smoke out-of-workspace → ONE registrar `go.work` swap → restart daemon → delete old. Publish short freeze window in master.md.
nx `.mjs` already at final home (M2) — no second move.

### A-PRINT / A-MIT — W8e / W8f
- Print: `@semio-tech/print` de-sandwich; LaTeX → manifest-less components.
- mit-bestand: unwrap both singular `⚡️implementation` sandwiches + demonstrator normalization; repoint compose's ~9 framework path refs only.

### A-PERIPH / A-SWEEP — W9
- Periphery configs + docs.
- project.json simplification (~200 files) with before/after `nx show project --json` command-equality.

### A-FINAL — W10 (strong + registrar)
All areas → `clean`; delete legacy validators/path regexes; promote taxonomy+package-shape+dep-cruiser to error; permanent `⚡️implementations` tripwire policy; regenerate everything.

### A-VERIFY — W11 (1–2 agents, long timeouts)
Full matrix below; only then close master ticket.

---

## Registrar protocol v2 (unchanged ownership table)

| File | Owner |
|---|---|
| root Cargo.toml / Cargo.lock | registrar, serialized |
| root package.json workspaces + bun.lock | registrar (apply generator) |
| go.work / go.work.sum | registrar (W8d atomic swap) |
| nx.json | frozen after M2; registrar-only if touched |
| .storybook + registry generated + launch.json | regen every registrar pass |
| dsl fixture-sweep | line-region ownership per plugin |
| 📋️master.md | orchestrating session only |

Pass recipe: consume handoffs → member/dep swaps → glob if newly populated → `cargo metadata` → `cargo check --workspace` → registry/launch/workspaces/storybook regen → master.md update → green gate.

---

## Cursor agent workforce orchestration (how to run this)

No worktrees. Cap **≤8 cargo-heavy** agents. Prefer Task/subagent fan-out with the shared preamble + exclusive globs.

Recommended dispatch order for the **next 24h**:
1. Parallel NOW: A1 fem, A2 writer retrofit, A3 reasoning retrofit, A4 stale-ticket mop, A5 sourcing ext, A6 trinity residual (6 agents).
2. Registrar barrier.
3. A7–A8 leftover residuals + A9 s-modules start.
4. Only when A1 green and no cargo red: start W8b math+assets+singletons (ui/surface if their tickets confirm idle).
5. Never start W8c until W8b non-compiler families landed + hub still green + C4 confirmed on new crates.

Handoff schema (required):
```json
{
  "owner": "🏗️fem",
  "ticketPath": "26/08/05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION",
  "newCrates": ["semio-s-plugin-fem"],
  "oldMemberLines": [],
  "workspaceDepRenames": [],
  "crossDepsFlagged": [],
  "residualsDeferred": [],
  "tests": {"baseline": 0, "now": 0},
  "wireProof": "byte-identical|n/a|documented-delta",
  "status": "green|flagged"
}
```

---

## Estimates (remaining)

| Phase | Agent-runs | Wall-clock @ ≤8 |
|---|---|---|
| W5f+W6r mop | ~8–10 | 0.5–1 day |
| W7 | 2 | 0.5 day |
| W8b families | ~6 (+compiler later) | 1–1.5 days |
| W8c os serial | 3 + 3 registrar | 1.5–2 days |
| W8d–f | 4–5 | 1–1.5 days |
| W9–W11 | 4–5 | 1 day |
| **Total remaining** | **~30–35** (+ forward-fix reserve) | **~5–7 working days** |

Long poles: fem finish, math stubs, **os kernel serialization**, Go CLI split.

---

## Top risks (remaining)

1. os-kernel 313-dep pivot → C4 + serial + wasm admission gate.
2. Go self-hosting outage → C8.
3. Concurrent humans on flow/compiler → published locks; snapshot-and-flag.
4. UI-ELEMENT vs UI-FAMILY collision → serialize in master.md.
5. fem dual-layout mid-state → delete-only-after-green.
6. Auto-commit daemon committing red → registrar green gates.
7. Stale open tickets misleading dispatch → A4 mop first.
8. `#[path="."]` double-prefix regressions → real cargo check required.

---

## Tickets to open (only if missing when MCP available)

- `IMPERATIVE-PLUGIN-EXTENSIONS-DE-SANDWICH`
- `PROCESS-PLAYBOOK-LEFTOVER-IMPL-DIRS`
- `PERIPHERY-CONFIG-SWEEP`
- `PROJECT-JSON-SIMPLIFICATION-SWEEP`
- `MECHANISM-FINALIZATION-FLIP`
- `RESTRUCTURE-VERIFICATION-MATRIX`
- `OS-KERNEL-MERGE-AND-SPR-RENAME-{DATA,HOST,DEV}` (3)
- `GO-SPLIT-PILOT-…` + `REPO-PRODUCT-GO-DOMAIN-SPLIT-AND-RELOCATION`
- `PRINT-PRODUCT-CONSOLIDATION`
- `MIT-BESTAND-IMPLEMENTATION-UNWRAP-AND-DEMONSTRATOR-NORMALIZATION`
- `ROOT-POLICY-PACKAGE-SHAPE-AND-AREA-MAP` (M4 gap)

Do **not** reopen closed mechanism/hub/demonstrator/cad-residual/flow-residual tickets.

---

## End-to-end verification (W11 gate)

1. `bun ./📜️script.ts verify gate` (incl. checkCargoMembers, checkWorkspaces, storybook/launch freshness).
2. `cargo check/clippy/test --workspace` native + `wasm32-wasip2`; all-plugin wasm+jco via `framework-os-dev:plugin`.
3. `bun install --frozen-lockfile`; nx project count sane; `nx run-many -t test --all`; storybook build.
4. `go build ./...` per module; repo MCP smoke (ticket_open/close round-trip on scratch).
5. Boot smokes (`dev flow`, puzzle 3d, cad, space) + parity; strip `[DEBUG]` logs.
6. Structural: zero `⚡️implementations`/`⚡️implementation`; no dangling `#[path]`; member count ≈61; tripwire grep green.

---

## Appendix — completed waves (do not redispatch)

- W0 mechanisms (initial), W1 flow pilot, W2 all plugin batches, W5d demonstrator
- Shape V2 broadcast + nearly-all retrofits
- W4 M1–M3, M6–M9, M2 nx, hub dress rehearsal (W8a)
- Flow TS/ext pilot + cad residual mop
- Templates present: TEMPLATE.md / TS / EXT / FAMILY (GO still to author in W8d pilot)
