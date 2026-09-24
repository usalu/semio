# Plugin Audit — Batch A (17 s plugins)

Audit date: 2026-09-23. Scope: read-only inspection of ✒️writer, ➗️mathematical, 🀄️wfc, 🌀️procedural, 🌊️flow, 🌍️gis, 🌿️vcs, 🎞️animate, 🎥️shooting, 🎪️demonstrator, 🎬️sequence, 🏗️fem, 🏛️architect, 🏭️process, 💠️lowpoly, 💡️reasoning, 📋️forms.

Raw command output: `🗑️generated/plugins-a/` (`cargo-check.txt`, `cargo-check-mathematical-retry.txt`, `nx-test-quick.txt`, `direct-test-quick.txt`, `stubs.txt`, `surfaces.txt`, `structure.txt`).

---

## Plugin contract summary

**OS model** (`🧰️framework/🛍️products/💻️os/AGENTS.md`): a plugin is a **manifest + artifact collection**. Each artifact subset exposes surfaces addressed as `<kind>@<standard>/<subset>#<role>` — an **editor** (mutating) and **viewer** (read-only). Artifacts carry schema-first definitions, **commands** (cmd/cde), **mutations** (event-sourced ops/patches), pack/dsl representations, and optional **engines** (stateful headless compute with streaming).

**s instance** (`✏️s/AGENTS.md`): collaborative design OS unifying the monorepo.

**Registration & load path**

1. Each plugin root holds `🛂️.descriptor.semio` + `🔣️.json` (component descriptor), produced by `@semio-tech/<name>-plugin:describe` (`cargo` wasm32-wasip2 build + `describePluginComponent`).
2. `🧰️framework/…/🔌️plugin/📇️registry/📜️script.ts` scans plugin crates and emits `🤖️generated/🧩️plugins/🟦️.ts` → `PLUGIN_BUILD_TARGETS` (pluginId, cratePath, wasmOut, activationEvents, capabilities, dependsOn, …).
3. OS activation (`🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts`) materializes staged browser modules from the registry; Storybook matrix in `📖️stories/🎭️plugins/🧪️.story.tsx` boots `FrameworkOsShell` per registry `pluginId`.
4. Policy vocabulary for semantic mutations is merged from `✏️s/🔌️plugins/🔒️policy-allowlist.json` (all 17 scoped plugins contribute mutation paths).

**Nx targets** (uniform): `@semio-tech/<plugin>-plugin` at `📦️packages/🦀️rust/📋️project.json` — `test`, `test-quick`, `test-long`, `describe`. Router: `bun ./📜️script.ts test [quick|long|…]`.

**Hub / collaboration mechanism (common)**

- Artifact edits are **semantic mutations** under `🗿️artifacts/<kind>/…/🧬️schema/🧬️mutations/` (aggregate `🦀️.rs` + per-kind leaves with inverse/diff/mutation).
- OS backbone (`🧰️framework/…/💻️os/🟦️.ts`) transports **mutation envelopes** (`mutationEnvelopeFromWire` / `mutationEnvelopeToWire`) over document backbones — event-sourced CQRS, not CRUD.
- Editor surfaces also carry config/presence/transient mutation schemas where needed (e.g. writer main window, animate/shooting presence).
- Plugins with `stdio.artifact-catalog.v1` contribution (gis, vcs) participate in hub catalog interchange.

---

## Per-plugin audit

### ✒️writer (`writer`)

**Structure:** Rust plugin crate `semio-s-plugin-writer`; TS package; artifact `✒️writer` (standard 1 / subset any) with ✏️editor + 👁️viewer; mutations: rename, change-uri, change-language, edit-text. LSP-oriented editor on infinite canvas (`AGENTS.md`).

**Registered:** Yes — `on-artifact-kind:text.document`, `s.writer.writer`.

**Builds:** `cargo check -p semio-s-plugin-writer` → exit 0. Wasm present at `📦️packages/🦀️rust/dist/component-dev/semio_s_plugin_writer.wasm`.

**Tests:** `@semio-tech/writer-plugin:test-quick` → **6 passed** (see `nx-test-quick.txt`).

**Hub collaboration:** Full mutation stack + policy allowlist entries; editor config/transient mutations.

**Blockers:** None critical. Minor: txt/md/pdf IO bridges exist via stdio deps; no `unimplemented!()`.

---

### ➗️mathematical (`mathematical`)

**Structure:** Rust + TS; artifact `➗️equation` (editor/viewer); graph subset oracles; equation mutation vocabulary.

**Registered:** Yes — `computation.equation`, `s.mathematical.equation`.

**Builds:** First `cargo check` hit 300s alarm (exit 142); **retry exit 0** (~47s, 8 warnings in `semio-s-artifact-mathematical-equation`).

**Tests:** **Not completed in audit window** — nx and direct `test quick` both SIGALRM at 300–600s (suite is large; not a confirmed failure).

**Hub collaboration:** Mutations registered in policy allowlist; partial oracle coverage (csv discharges 5/15 graph kinds per oracle JSON).

**Blockers:** (1) txt/md IO serializers are honest stubs (`…/🚪️io/…/🔤️txt/🦀️.rs`). (2) 10 equation graph mutation kinds lack qualifying third-party oracle (`noOracleDecisions` debt). (3) Run `test-long` in CI with adequate budget to confirm green.

---

### 🀄️wfc (`wfc`)

**Structure:** Rust + TS + **⚙️engine**; six artifacts: `◻️2d`, `🔲️grid2d`, `🖼️bitmap`, `🧊️3d`, `🧱️grid3d` — each with editor/viewer + mutations.

**Registered:** Yes — wfc2d/wfc3d/grid/bitmap activation kinds (10 events).

**Builds:** `cargo check` exit 0.

**Tests:** Audit timeout (300–600s); not confirmed.

**Hub collaboration:** Per-artifact mutation aggregates; grid/bitmap/wfc editors.

**Blockers:** None in compile surface; verify tests with extended budget.

---

### 🌀️procedural (`procedural`)

**Structure:** Rust + TS; artifacts `🌀️generation2d`, `🧊️generation3d`; flow-linked BRep generation; depends on **8 flow extensions** (bim, brep, dictionary, list, logic, math, primitive, text).

**Registered:** Yes — `2d.generation`, `3d.generation`, `s.procedural.generation2d/3d`.

**Builds:** `cargo check` exit 0.

**Tests:** Audit timeout.

**Hub collaboration:** Mutations on both artifacts; 3d viewer presence/transient schemas; consumes `forms.questionKind`.

**Blockers:** (1) txt IO import/export stubs on both generation artifacts. (2) **Runtime dependency on linked flow extensions** — contributed stubs vs linked installers must be wired for 3d eval chain (see `🧪️tests/🔬️flow-operators`, `🔬️brep-extension`). (3) End-to-end 3d requires flow extension packs built and registered.

---

### 🌊️flow (`flow`)

**Structure:** Rust + TS; artifact `🌊️flow`; **9 flow extensions** under `🧩️extensions/` (bim, brep, dictionary, draw, list, logic, math, primitive, text); framework flow registry (`🔨️modules/🌊️flow/📔️registry/🦀️.rs`) admits extension manifests.

**Registered:** Yes — `computation.flow`, `s.flow.flow`; contributes/consumes `flow.extension`.

**Builds:** `cargo check` exit 0.

**Tests:** Audit timeout.

**Hub collaboration:** Flow artifact mutations; extension contributions are hub-visible via flow registry replacement generation.

**Blockers:** txt IO stubs; extension WIT invoke vs linked installers still split (registry comments). Procedural/demonstrator blocked until extensions load.

---

### 🌍️gis (`gis`)

**Structure:** Rust + TS; artifacts `🗺️gismap`, `🏔️gisterrain`; native codecs (`📇️native-codecs`); `shell.navigate` + `stdio.artifact-catalog.v1` contribution.

**Registered:** Yes — `s.gis.gismap`, `s.gis.gisterrain`.

**Builds:** `cargo check` exit 0.

**Tests:** **`test-quick` FAILED (exit 1)** — `proveGisNativeCodecReceipts` in `🧪️tests/📇️native-codecs/🟦️.ts:25` rejects fixture `🧫️fixtures/🌱️artifact-document-id-v1/🔣️.json` because cases use **`artifactId`** but registry schema `#/$defs/ArtifactDocumentIdV1` requires **`documentId`**.

**Hub collaboration:** Both artifacts have mutation stacks; gismap editor config mutations; gisterrain viewer config.

**Blockers:** **P0 test failure** — align `artifact-document-id-v1` fixture field names with registry schema (or update GIS oracle to new shape). txt IO stubs on both artifacts. Fix unblocks native codec receipt gate before hub catalog claims.

---

### 🌿️vcs (`vcs`)

**Structure:** Rust + TS; artifact `🌿️vcs`; contributes `stdio.artifact-catalog.v1`.

**Registered:** Yes — `vcs.vcs`, `s.vcs.vcs`.

**Builds:** `cargo check` exit 0.

**Tests:** Audit timeout.

**Hub collaboration:** VCS artifact mutations; catalog contribution for hub artifact discovery.

**Blockers:** txt IO stub + TS IO descriptor stub (`🚪️io/🟦️.ts`).

---

### 🎞️animate (`animate`)

**Structure:** Rust + TS; **`🎛️apps/🎬️presentation`** with TS implementation (`⚡️implementations/🟦️typescript`); artifact `🎬️presentation`; stories under `📖️stories`.

**Registered:** Yes — `animate.presentation`, `s.animate.presentation`.

**Builds:** `cargo check` exit 0.

**Tests:** Audit timeout.

**Hub collaboration:** Presentation mutations; editor config + **presence** mutations (collaboration-aware).

**Blockers:** txt IO stub; video raster path returns honest `Adapter` error on wasm (documented in editor video module). TS play-app registration noted as minimal stub in TS host.

---

### 🎥️shooting (`shooting`)

**Structure:** Rust + TS; artifact `🎥️shooting`; saved cameras, assets, shots mutation families.

**Registered:** Yes — `2d.shooting`, `s.shooting.shooting`.

**Builds:** `cargo check` exit 0.

**Tests:** Audit timeout.

**Hub collaboration:** Full mutation stack + editor config/presence mutations.

**Blockers:** txt IO stub only (core shooting mutations implemented).

---

### 🎪️demonstrator (`demonstrator`)

**Structure:** Rust + TS; artifact `🎪️playground` (minimal schema stub); **integration plugin** — `dependsOn`: cad, gis, procedural, process, puzzle, sourcing, **8 flow extensions**; consumes forms, process, cad, sourcing capabilities.

**Registered:** Yes — `s.demonstrator.playground` only (no `artifacts.write` capability).

**Builds:** `cargo check` exit 0.

**Tests:** Audit timeout.

**Hub collaboration:** Playground mutations including `🫙no-mutation` test kinds; editor/viewer presence schemas.

**Blockers:** **Integration hub** — OS end-to-end requires all dependency plugins + extensions built/activated. Playground artifact is intentionally minimal (`🎪️playground/🦀️.rs` schema stub). No standalone AGENTS.md.

---

### 🎬️sequence (`sequence`)

**Structure:** Rust + TS; artifact `🎬️sequence`; step/dependency mutation subsets.

**Registered:** Yes — `computation.sequence`, `s.sequence.sequence`.

**Builds:** `cargo check` exit 0.

**Tests:** Audit timeout.

**Hub collaboration:** Sequence mutations; csv oracle covers 4/8 step kinds.

**Blockers:** txt/csv import stubs; 4 sequence kinds uncarried by csv oracle (edges, layout fields).

---

### 🏗️fem (`fem`)

**Structure:** Rust + TS + **⚙️engine**; artifacts `◻️2d` (fem2d), `🧊️3d` (fem3d); subset `🌐️any`; rich mutation vocabulary (~25–29 kinds per dimension).

**Registered:** Yes — fem2d/fem3d activation kinds.

**Builds:** `cargo check` exit 0.

**Tests:** Audit timeout.

**Hub collaboration:** Both artifacts mutation-ready; mesh oracles (obj/stl) + json oracle for non-geometric fields.

**Blockers:** txt import/export explicitly `not yet implemented`; 22+ kinds invisible to mesh carriers (documented in oracle JSON). Analysis/settings mutations need json oracle path for hub verification.

---

### 🏛️architect (`architect`)

**Structure:** Rust + TS; artifact `🏛️program`; 266-mutation architectural brief; Python cross-implementation oracle registered; CSV register upsert **stubs** for relationships/adjacency/knowledge/benchmarks in editor.

**Registered:** Yes — `data.program`, `s.architect.program`.

**Builds:** `cargo check` exit 0.

**Tests:** Audit timeout.

**Hub collaboration:** Full program mutation stack; presence + config mutations.

**Blockers:** (1) Storybook documents **missing prebuilt web artifact** for `architect` (artifact-missing panel). Run `describe` + activation. (2) Editor register import stubs (`upsert_*_stub` in `✏️editor/🦀️.rs`). (3) External qualifying oracle still owed per oracle registration notes. txt IO stub.

---

### 🏭️process (`process`)

**Structure:** Rust + TS; artifact `🧊️process3d`; **4 machine extensions** (concrete, metal, robotic, wood) contributing `process.machines`.

**Registered:** Yes — `3d.process`, `s.process.process3d`; consumes `process.machines`.

**Builds:** `cargo check` exit 0.

**Tests:** Audit timeout (nx still running at audit cut-off).

**Hub collaboration:** Process3d mutations (steps, machines, snapshot); extension manifests for hub machine catalog.

**Blockers:** txt IO stub; demonstrator/process integration requires machine extensions built.

---

### 💠️lowpoly (`lowpoly`)

**Structure:** Rust + TS; artifact `💠️lowpoly`; object add/remove/move/patch mutations.

**Registered:** Yes — `3d.lowpoly`, `s.lowpoly.lowpoly`.

**Builds:** `cargo check` exit 0.

**Tests:** `test-quick` **failed** — `ENOENT .tmp-wp-c3` in cargo workspace index walk (likely transient; re-run to confirm).

**Hub collaboration:** Full mutation stack in policy allowlist.

**Blockers:** **Architectural IO gap** — mesh is content-addressed handle; **dwg/gltf/las/stl** export/import are honest error stubs (`io-lowpoly-1` tests document this). png oracle exists separately.

---

### 💡️reasoning (`reasoning`)

**Structure:** Rust + TS; artifact `🔌️wires` (graph reasoning); editor canvas config/transient mutations.

**Registered:** Yes — `graph.wires`, `s.reasoning.wires`.

**Builds:** `cargo check` exit 0.

**Tests:** Audit timeout.

**Hub collaboration:** Wires mutation aggregate.

**Blockers:** csv/txt/svg/png IO stubs (`🚪️io/🟦️.ts`); json path is real.

---

### 📋️forms (`forms`)

**Structure:** Rust + TS; artifact `📋️forms`; contributes/consumes `forms.questionKind` (used by procedural, demonstrator).

**Registered:** Yes — `form.dictionary`, `s.forms.forms`.

**Builds:** `cargo check` exit 0.

**Tests:** Audit timeout.

**Hub collaboration:** Forms mutations.

**Blockers:** csv/xlsx/zip import/export stubs; downstream plugins depend on questionKind surface being loadable.

---

## Cross-cutting findings

| Check | Result |
|-------|--------|
| All 17 in `PLUGIN_BUILD_TARGETS` | Yes |
| `unimplemented!()` / `todo!()` in scoped trees | **0** occurrences |
| `cargo check` (17 crates) | **16/17 exit 0 in first pass**; mathematical timeout then **pass on 600s retry** |
| `test-quick` (full nx batch, 300s/plugin) | **writer pass**; **gis fail** (document-id schema); **lowpoly fail** (`ENOENT .tmp-wp-c3` during cargo workspace index — likely transient/env); **14× exit 142** (SIGALRM timeout) |
| Direct `bun ./📜️script.ts test quick` (600s, 15 plugins) | **6× exit 142** (mathematical, wfc, procedural, flow, vcs, animate); **9× exit 1** — all **`loadCatalogTaxonomy()` invalid taxonomy** (`dev-jco-*` fixed directory contracts: “Fixed parent scope requires distinct literal parent authorities”) |
| Prebuilt wasm in `dist/component-dev` | **All 17** present (`semio_s_plugin_<id>.wasm`); flow/process extensions also built. OS **staging/activation** may still be required for browser load |
| txt IO stub pattern | Present on most plugins (honest `Lossy` stubs) |
| Hub mutation participation | All scoped artifacts expose `🧬️mutations` aggregates listed in `surfaces.txt` |

---

## Prioritized blocker list

1. **🌍️gis `test-quick` failure** — Fix `🧫️fixtures/🌱️artifact-document-id-v1/🔣️.json` vs `ArtifactDocumentIdV1` schema (`artifactId` → `documentId`) and update `proveGisNativeCodecReceipts` if needed. Blocks GIS native codec gate and catalog receipts.

1b. **💠️lowpoly `test-quick` failure** — `ENOENT` on `.tmp-wp-c3` in `getCargoWorkspaceIndex` during test startup; re-run outside sandbox / verify temp worktree cleanup; may be environmental rather than plugin logic.

2. **OS activation / staging gap** — All 17 plugins have local `dist/component-dev/*.wasm`, but registry **browser staging** may be stale; run dev activation so Storybook/shell loads current descriptors (architect story still documents artifact-missing when staging absent).

3. **Flow extension chain for procedural + demonstrator** — Build/link 8 flow extensions; verify linked installers shadow contributed stubs (`flow/📔️registry`, procedural 3d tests).

4. **Demonstrator integration dependency fan-in** — Requires cad, gis, procedural, process, puzzle, sourcing, flow extensions simultaneously for playground end-to-end.

5. **Repo taxonomy schema drift** — Direct test runs for shooting→forms fail before cargo tests start: `loadCatalogTaxonomy()` rejects `dev-jco-*` fixed directory/filename contracts. Fix taxonomy generator or contract definitions repo-wide to unblock plugin test scripts.

6. **Test budget / CI** — Mathematical, wfc, fem, architect suites exceed 300–600s quick budget; configure `SEMIO_BUILD_BUDGET_MS` or use `test-long` in CI to get definitive pass/fail (audit inconclusive except writer/gis).

7. **Architect editor register stubs** — `upsert_relationship_stub`, `upsert_knowledge_stub`, etc. in `🏛️program/…/✏️editor/🦀️.rs` limit CSV/register import fidelity.

8. **Lowpoly mesh-handle IO** — dwg/gltf/las/stl blocked by content-addressed mesh architecture; document or implement mesh resolution layer for export.

9. **Forms questionKind + IO stubs** — Procedural/demonstrator consume forms; csv/xlsx/zip stubs limit external form interchange over hub.

10. **Oracle / second-implementation debt** — Mathematical (10 kinds), sequence (4 kinds), fem non-geometry kinds, architect external oracle — weak hub mutation verification until addressed.

11. **Minor IO stub backlog** — Shared txt (and plugin-specific csv/svg/png/xlsx) stubs across batch; does not block core OS load but blocks file round-trips.

---

## Suggested next commands (for implementers, not run in this audit)

```bash
# Fix gis then verify
bun nx run @semio-tech/gis-plugin:test-quick

# Build wasm descriptors for OS load
bun nx run @semio-tech/architect-plugin:describe

# Extended test budget example
SEMIO_BUILD_BUDGET_MS=3600000 bun nx run @semio-tech/mathematical-plugin:test-quick
```
