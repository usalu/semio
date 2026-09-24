# Audit: Plugins B (layout → sourcing)

**Date:** 2026-09-23  
**Scope:** 17 plugins under `✏️s/🔌️plugins/`  
**Raw logs:** `🗑️generated/plugins-b/`

---

## Plugin Contract Summary

Per `🧰️framework/🛍️products/💻️os/AGENTS.md` and `✏️s/AGENTS.md`:

| Concept | Meaning |
|---------|---------|
| **Plugin** | `🛂️.descriptor.semio` + `🔣️.json` manifest + `🗿️artifacts/` collection |
| **Surface** | Editor (`✏️editor`) or viewer (`👁️viewer`) per artifact subset; addressed as `<kind>@<standard>/<subset>#<role>` |
| **Engine** | Stateful headless actor; maintains pack buffer; applies patches on the fly |
| **Command / Cde / Cmd** | Mutation operations; binary (cde) for wire/storage, text (cmd) for logging/LLMs |
| **Registration** | `🔎️discovery/🟦️.ts` scans workspace crates → `🤖️generated/🧩️plugins/🟦️.ts` (`PLUGIN_BUILD_TARGETS`) |
| **Policy** | `🔒️policy-allowlist.json` gates file access for sandboxed agents |
| **Nx targets** | Each plugin crate: `@semio-tech/<id>-plugin` with `test`, `test-quick`, `describe` via `📜️script.ts` |

**Hub collaboration mechanism (common):** Artifact subsets define `🧬️schema/🧬️mutations/` (event-sourced patches) and `🎮️commands/` (operations). The `🪐️space` plugin owns the version-controlled Space container. Hub replicates artifact state via `semio-framework-replication`; MCP binds with `--hub <url> --space <id>`. No scoped plugin declares `HUB_INFERENCE_ROUTES` — inference jobs route through framework tables.

---

## Summary Table

| Plugin | Artifacts | Registered | cargo check | test-quick | Hub mutations |
|--------|-----------|------------|-------------|------------|---------------|
| 📏️layout | 1 (`📏️layout`) | ✅ | ✅ | ⏳ not completed* | ✅ (4 dirs) |
| 📐️cad | 1 + 4 extensions | ✅ | ✅ | ⏳ not completed* | ✅ (2 dirs) |
| 📕️norm | 15 norm standards | ✅ | ✅ | ⏳ not completed* | ✅ (30 dirs) |
| 📖️playbook | 1 + 1 extension | ✅ | ✅ | ⏳ not completed* | ✅ (3 dirs) |
| 📜️imperative | 1 + 5 extensions | ✅ | ✅ | ✅ 3/3 pass | ✅ (3 dirs) |
| 📸️remodel | 1 (`📸️remodeling`) | ✅ | ✅ | ✅ 3/3 pass | ✅ (2 dirs) |
| 🔋️energy | 1 (`🔋️model`) | ✅ | ✅ | ✅ 4/4 pass | ✅ (5 dirs) |
| 🔱️trinity | 2 (`🔌️jack`, `♻️rewriting`) | ✅ | ✅ | ✅ 5/5 pass | ✅ (11 dirs) |
| 🕸️dag | 1 (`🕸️dag`) | ✅ | ✅ | ⏳ not completed* | ✅ (5 dirs) |
| 🖍️draw | 1 (`🖍️drawing`) | ✅ | ✅ | ⏳ not completed* | ✅ (10 dirs) |
| 🖨️raster | 1 (`🖨️raster`) | ✅ | ✅ | ⏳ not completed* | ✅ (2 dirs) |
| 🗄️stdio | 40+ file formats | ✅ | ✅ | ⏳ not completed* | ✅ (152 dirs) |
| 🗒️note | 1 (`🗒️note`) | ✅ | ✅ | ⏳ not completed* | ✅ (19 dirs) |
| 🧩️puzzle | 3 (2d/3d/5d) | ✅ | ✅ | ⏳ not completed* | ✅ (6 dirs) |
| 🧱️block | 3 (2d/3d/5d) | ✅ | ✅ | ⏳ not completed* | ✅ (7 dirs) |
| 🪐️space | 2 (`🏠️home`, `🪐️space`) | ✅ host | ✅ | ⏳ not completed* | ✅ (4 dirs) |
| 🪵️sourcing | 1 + 3 extensions | ✅ | ✅ | ⏳ not completed* | ✅ (3 dirs) |

\*Tests blocked by concurrent `cargo` file-lock contention during audit window; see `🗑️generated/plugins-b/*-audit.log`.

**All 17 plugins:** present in `PLUGIN_BUILD_TARGETS`, have `🔣️.json` + `🛂️.descriptor.semio`, listed in `🔒️policy-allowlist.json`.

---

## Per-Plugin Sections

### 📏️layout

- **Languages:** Rust (primary), TypeScript package shell
- **Artifacts:** `📏️layout` — 2d layout document (pages, spreads, frames, stories)
- **Surfaces:** 1 editor + 1 viewer at `🏅️standards/🔖️1/🪆️subsets/✳️any/`
- **Registered:** `pluginId: "layout"`, activation `on-artifact-kind:2d.layout`, `s.layout.layout`
- **Builds:** ✅ `cargo check -p semio-s-plugin-layout` (warnings only)
- **Tests:** Not completed (cargo lock contention with parallel audit jobs)
- **Blockers:** None in source; 2 `unnecessary qualification` warnings in artifact crate

### 📐️cad

- **Languages:** Rust, TypeScript; optional `⚙️engine`
- **Artifacts:** `📐️cad` (3d CAD)
- **Extensions:** `🏢️aec-building`, `🔥️aec-building-energy`, `🏛️aec-building-structure`, `📐️spatial-shape` (all `cad.computer` contributors)
- **Surfaces:** 1 editor + 1 viewer
- **Registered:** `pluginId: "cad"`, consumes `cad.computer`, depends on cad for extensions
- **Builds:** ✅
- **Tests:** Not completed during audit
- **Blockers:** Extensions require parent `cad` plugin loaded at runtime

### 📕️norm

- **Languages:** Rust; 15 separate norm artifact kinds (EN1990–1999, DIN4108, DIN16798, DIN18599, ISO16757, VDI3805)
- **Artifacts:** One editor + viewer per norm standard under `🗿️artifacts/`
- **Apps:** Separate evaluate apps per norm under `🎛️apps/`
- **Registered:** `pluginId: "norm"`, 30 activation events for computation.norm.* kinds
- **Builds:** ✅
- **Tests:** Not completed during audit
- **Blockers:** Large surface area (15 standards × editor/viewer); compliance oracle fixtures need hub round-trip validation

### 📖️playbook

- **Languages:** Rust
- **Artifacts:** `📖️playbook` (text playbook)
- **Extensions:** `🌀️procedural` (`playbook.blockKind` contributor)
- **Registered:** `pluginId: "playbook"`, consumes `playbook.blockKind`
- **Builds:** ✅
- **Tests:** Not completed during audit
- **Blockers:** Extension `playbook-module-procedural` must load with parent

### 📜️imperative

- **Languages:** Rust
- **Artifacts:** `📜️procedure` (computation procedure)
- **Extensions:** `🎮️control`, `📣️effect`, `🧠️logic`, `🧮️math`, `📝️text` (all `imperative.module` contributors)
- **Builds:** ✅ | **Tests:** ✅ 3 passed (nextest quick)
- **Blockers:** None observed

### 📸️remodel

- **Languages:** Rust; `📖️stories` for Storybook
- **Artifacts:** `📸️remodeling` (3d remodeling)
- **Capabilities:** `ui.dialog`
- **Builds:** ✅ | **Tests:** ✅ 3 passed
- **Blockers:** None observed

### 🔋️energy

- **Languages:** Rust; `🔨️modules` for domain logic
- **Artifacts:** `🔋️model` (data.model / energy model)
- **Builds:** ✅ | **Tests:** ✅ 4 passed
- **Blockers:** None observed

### 🔱️trinity

- **Languages:** Rust; `🔨️modules/🔌️jack` (LSP + shell subsystems)
- **Artifacts:** `🔌️jack` (graph.trinity), `♻️rewriting` (text.rewriting)
- **Builds:** ✅ | **Tests:** ✅ 5 passed
- **Blockers:** Jack LSP/shell modules add complexity; verify wasm component size for browser boot

### 🕸️dag

- **Languages:** Rust
- **Artifacts:** `🕸️dag` (graph.dag)
- **Builds:** ✅ (re-verified after transient framework error)
- **Tests:** Not completed during audit
- **Blockers:** Had transient `PROCESS_POOL_PUMPS_PER_TURN` compile error in framework reactor during concurrent edits (resolved)

### 🖍️draw

- **Languages:** Rust
- **Artifacts:** `🖍️drawing` (2d.drawing) — dual editor paths (legacy + standard/1/any)
- **Builds:** ✅
- **Tests:** Not completed during audit
- **Blockers:** `unimplemented!("host tests never step a machine")` in `🖱️canvas-pointer-down/🔄️fsm/🧪️tests/🔬️host-unit/🦀️.rs:13` — test-only, not production

### 🖨️raster

- **Languages:** Rust
- **Artifacts:** `🖨️raster` (2d.raster)
- **Builds:** ✅
- **Tests:** Not completed during audit
- **Blockers:** None observed

### 🗄️stdio

- **Languages:** Rust, TypeScript (`🟦️.ts` at plugin root); largest plugin (40+ artifact kinds, 152 mutation dirs, 353 command dirs)
- **Artifacts:** File-format codecs (IFC, STEP, GLTF, PNG, PDF, CSV, …)
- **Contributes:** `stdio.artifact-catalog.v1` (shared with gis, vcs)
- **Builds:** ✅ (2m28s compile)
- **Tests:** Not completed during audit
- **Blockers:** `AGENTS.md` states *"Builder/decomposer use local traits until SDK Wave 3"* — partial abstraction. Massive artifact surface needs selective hub smoke per format family.

### 🗒️note

- **Languages:** Rust
- **Artifacts:** `🗒️note` (s.note.note) with math subset mutations
- **Builds:** ✅
- **Tests:** Not completed during audit
- **Blockers:** None observed; commonly used as MCP catalog smoke reference

### 🧩️puzzle

- **Languages:** Rust; `🎯️targets` for renderer integration; **actively edited** (5d editor churn)
- **Artifacts:** `◻️2d`, `🧊️3d`, `🖐️5d` — each with editor + viewer
- **Capabilities:** `ui.dialog`, `shell.clipboard`
- **Builds:** ✅ (re-verified; 4 warnings in 5d artifact)
- **Tests:** Not completed during audit
- **Blockers:** Active concurrent edits on 5d editor (selection sync, vortex suggestions, 2d window layout, drag preview). No production `unimplemented!()` found; expect test churn until 5d tickets land.

### 🧱️block

- **Languages:** Rust, TypeScript (`🟦️.ts` at plugin root)
- **Artifacts:** `◻️2d`, `🧊️3d`, `🖐️5d` block kinds (parallel structure to puzzle)
- **Builds:** ✅
- **Tests:** Not completed during audit
- **Blockers:** None observed

### 🪐️space

- **Languages:** Rust; `⚙️engine`, `🫀️core`
- **Artifacts:** `🏠️home` (studio landing), `🪐️space` (version-controlled container)
- **Host metadata:** `landingAppId: "home"`, `hostAppId: "studio"` — **OS shell host plugin**
- **Builds:** ✅
- **Tests:** Not completed during audit
- **Blockers:** **Critical path** — OS boot, hub space binding, and MCP `--hub --space` all depend on this plugin working end-to-end

### 🪵️sourcing

- **Languages:** Rust
- **Artifacts:** `🗂️curation` (catalogue.sourcing)
- **Extensions:** `🪵️beams`, `🧱️slabs`, `🪟️windows` (`sourcing.module` contributors)
- **Builds:** ✅
- **Tests:** Not completed during audit
- **Blockers:** Extensions require parent; consumed by `demonstrator` plugin

---

## Hub Collaboration Participation

| Mechanism | Participation |
|-----------|---------------|
| `🧬️mutations/` schemas | All 17 plugins — patch-based event sourcing |
| `🎮️commands/` | All plugins; stdio has 353 command dirs (per-format IO) |
| `📸️snapshot/` + `🔺️diff/` | Present on artifact schemas (pack/dsl round-trip) |
| Space container | `🪐️space` plugin owns `s.space.space`, `s.space.home`, `s.space.studio` |
| Hub MCP binding | Framework `🌉️mcp` module; `--hub` + `--space` flags; not plugin-specific |
| Inference jobs | No scoped plugin declares hub inference routes; framework `HUB_INFERENCE_ROUTES` table applies |
| Catalog contribution | `🗄️stdio` (+ gis/vcs outside scope) contributes `stdio.artifact-catalog.v1` |

Collaboration verification harness: `🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts` (framework-level, not per-plugin).

---

## Prioritized Blocker List

| Priority | Blocker | Affected | Suggested Fix |
|----------|---------|----------|---------------|
| **P0** | `🪐️space` host boot + hub space binding untested in this audit | OS shell, MCP, collaboration | Run `framework/os` collaboration test suite with space plugin; verify `home`/`studio` surfaces render |
| **P0** | Test-quick not verified for 13/17 plugins (cargo lock contention) | All except imperative, remodel, energy, trinity | Run `bun nx run @semio-tech/<plugin>-plugin:test-quick` sequentially outside concurrent agent builds |
| **P1** | `🧩️puzzle` 5d editor active churn | puzzle OS integration | Complete in-flight tickets (selection sync, vortex, 2d window, drag preview); re-run puzzle tests after merge |
| **P1** | Transient framework compile breaks during concurrent edits | dag, draw, raster, stdio, note, puzzle (observed `PROCESS_POOL_*` errors) | Coordinate framework reactor edits; add CI gate on `cargo check --workspace` before plugin PRs |
| **P1** | `🗄️stdio` SDK Wave 3 builder/decomposer stubs | stdio format round-trip via hub | Implement framework SDK traits per `🗄️stdio/AGENTS.md`; smoke-test IFC/STEP/PNG families |
| **P2** | Extension plugins need parent at runtime | cad (4), imperative (5), playbook (1), sourcing (3) | Verify `dependsOn` closure in dev session staging (`♻️activation/🏃️execution`) |
| **P2** | `📕️norm` 15-standard matrix untested end-to-end | norm compliance workflows | Hub round-trip smoke per norm kind with oracle fixtures in `🧫️fixtures/` |
| **P3** | `🖍️draw` host-unit FSM test stub | draw pointer interaction tests | Implement `host tests step a machine` or mark `#[ignore]` with ticket |
| **P3** | Compile warnings (unnecessary qualification) | layout, puzzle-5d, sourcing, others | `cargo fix` pass (cosmetic) |

---

## Audit Methodology Notes

- **cargo check:** Per-plugin `cargo check -p semio-s-plugin-<id>` — all 17 exit 0 (final re-verification 2026-09-23T12:15Z)
- **test-quick:** `bun nx run @semio-tech/<id>-plugin:test-quick` (nextest `quick` profile); 4/17 confirmed pass
- **Registration:** Parsed from `🤖️generated/🧩️plugins/🟦️.ts`
- **Stubs:** `rg 'unimplemented!|todo!' --glob '*.rs'` — 1 hit (draw test-only)
- **Concurrent editing:** Other agents modified framework reactor and puzzle 5d during audit; transient compile failures observed and resolved on re-check

**Log files:** `🗑️generated/plugins-b/{plugin}-audit.log`, `cargo-check-summary.txt`, `cargo-check-final.txt`, `{plugin}-structure.txt`
