# Master tracking doc — Crate Consolidation & Plugin Taxonomy Restructure

Full plan: `/Users/ueli/.claude/plans/the-codebase-has-currently-declarative-ritchie.md` (approved).
This ticket path: `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`.

**Rule for every subagent spawned under this initiative: never call `ticket_close` on this ticket path or on any other agent's ticket. Open and close only your own ticket, with an explicit path.**

## Discovery contract (normative — read before touching registry/policy code)

- Plugin crate = `✏️s/🔌️plugins/<plugin>/📦️packages/🦀️rust/Cargo.toml` with `[package.metadata.semio] role = "plugin"`. All other `[package.metadata.*]` keys (component.package, contributes/consumes, playground, assets) move verbatim from the old bundle crate.
- Framework crate = any `🧰️framework/**/📦️packages/🦀️rust/Cargo.toml` with `[package.metadata.semio] role = "framework"`.
- Taxonomy validator requirements per plugin: every `🗿️artifacts/<a>/` has all of `{🔺️diff,🗣️dsl,🎒️pack,🔧️op,📡️spr}` as `🦀️component.rs`; every `🪟️windows/<w>/` contains only `{🍱️panes,🪀️widgets,🪛️utilities,🎬️actions,🎚️options}` children, each leaf `🦀️component.rs`; plugin `📦️lib.rs` declares a `mod`/`#[path]` for every component file on disk and none dangle; no `📡️protocol` path segment remains under `✏️s/🔌️plugins/`.
- Migration tolerance: `LEGACY_LAYOUT_TOLERANT = true` accepts both old (7-crate) and new (taxonomy) shapes per plugin during W0–W3; a plugin present in both is "in-flight", not an error. Flag flipped off in W4.

## Single-File-Repo hazard ruling

**Status: mitigated in ticket `26/08/05/SINGLE-FILE-REPO-GOAL-RESCOPE-AND-HAZARD-DOCUMENTATION` (goal.json amended; AGENTS.md deliberately left untouched — CLAUDE.md forbids editing AGENTS.md files, no exceptions).**

The `.🦑️repo/🎯️goals/AI-OPTIMIZED-REPO/SINGLE-FILE-REPO/🎯️goal.json` goal and any agent following `.cursor/plans/single_file_consolidation_bdb40556.plan.md`'s playbook (inline `#[path]` modules back into lib.rs, delete `src/*.rs`) **must not act on `✏️s/🔌️plugins/**` taxonomy `component.rs` files or on consolidated-framework-crate module files**. "Single file" for this repo means *one file per semantic component*, which the new taxonomy already satisfies — it is not "one file per crate".

Mitigation actually landed (defense in depth, revised from the plan's original "amend goal.json + AGENTS.md" — the AGENTS.md leg is dropped, not replaced, per the absolute CLAUDE.md prohibition):

1. **Goal content amendment (done).** `🎯️goal.json`'s `description` and `prompt` fields (the only free-text fields the schema — `Goal` struct, repo CLI `🐹️main.go` — exposes; there is no separate exclusions/scope field) now carry an explicit, clearly-delimited scope note and exclusion list, quoted here:
   - *description* appends: *"files under ✏️s/🔌️plugins/\*\*/🗿️artifacts, 🎛️apps, 🎭️modes, 🪟️windows, 🍱️panes, 🪀️widgets, 🪛️utilities, 🎬️actions, 🎚️options, 📌️panels, 🎮️commands, 🛠️tools taxonomy directories, and consolidated-framework-crate module files (🦀️\<name\>.rs wired via #\[path\] from a crate-root 📦️lib.rs), are OUT OF SCOPE for single-file consolidation and MUST NOT be inlined back into their parent lib.rs or module file. These are already 'single file' at the correct grain (one file per component); merging them regresses the taxonomy, not simplifies it."*
   - *prompt* appends an explicit STOP instruction: before inlining any `#[path]`-attributed `.rs` file, check whether it sits under a plugin taxonomy directory or is a consolidated-framework-crate module file; if so, stop, do not inline, do not delete the split file; the `single_file_consolidation_bdb40556.plan.md` playbook is called out as *not applicable* to these paths, with a pointer back to this master ticket section for full context.
   - `status`, `dates`, `github`, `client`, `llm`, `parent` were left byte-identical; this was a content-only edit, not a goal lifecycle action (goal was not opened/closed/reopened).
2. **AGENTS.md — explicitly NOT amended.** The original plan's hazard section proposed amending AGENTS.md alongside goal.json. CLAUDE.md's root rule "You MUST NOT edit AGENTS.md files" is absolute and admits no task-specific exception, so this leg of the mitigation is dropped. The goal-content amendment (1) and the structural lints (3) below are the full mitigation; no AGENTS.md compensation was attempted through any other route (no symlink, no included/generated file, no adjacent doc masquerading as AGENTS.md).
3. **Structural backstop (owned by W0-B, tracked separately).** The taxonomy validator + a new `TaxonomyLibShape` policy lint in root `📜️script.ts` make any inlining fail `verify gate` immediately, independent of whether an agent reads the goal text at all. This is the mitigation of record for agents that skip `repo://goals` entirely — the goal-content edit above helps agents that *do* read goals before acting, but the lint is what actually blocks a bad merge.
4. **This breadcrumb.** Any agent that finds the goal via `repo://goals` and then searches tickets (per CLAUDE.md's "work inside a ticket" rule) lands here before starting; that's this section's job.

## Plugin-ownership schedule (for concurrent human devs — avoid editing a plugin while its wave agent owns it)

Template only below — W1's findings and the orchestrator's batch split populate the TBD rows in later waves; this session does not assign the 31 remaining plugins.

| Wave | Batch | Plugin(s) | Agent | Status | Notes |
|---|---|---|---|---|---|
| W1 | pilot (solo) | 🌊️flow | TBD | not started | end-to-end pilot; writes `TEMPLATE.md` for W2 to reuse |
| W2 | batch 1 (~8) | TBD | TBD | not started | assigned by orchestrator after W1 template lands |
| W2 | batch 2 (~8) | TBD | TBD | not started | — |
| W2 | batch 3 (~8) | TBD | TBD | not started | — |
| W2 | batch 4 (~7, incl. 🎪️demonstrator) | TBD, 🎪️demonstrator always last | TBD | not started | 🎪️demonstrator depends on other plugins' crates — must be the final plugin migrated in this wave, after all its dependencies |

Heavier plugins flagged in the plan for strongest-model assignment (not yet batched): 🧩️puzzle (multi-app), 📐️cad, 🌍️gis, ➗️mathematical, 🏛️architect (spine decomposition), 📕️norm (15 apps).

(Updated live as waves dispatch — see wave status below.)

## Wave status

- [x] W0 — mechanism preparation (7 parallel agents, all closed their own tickets: REGISTRY-DISCOVERY-CONTRACT-TOLERANCE-AND-RUST-TWIN-COLLAPSE, ROOT-SCRIPT-POLICY-REVIVAL-AND-TAXONOMY-LINT-PREP, LAUNCH-JSON-GENERATOR-FROM-PLAYGROUND-REGISTRY, STALE-CONFIG-FIXES-AND-CAPABILITY-LINT-REVIVAL, ROOT-WORKSPACE-DEPENDENCIES-AND-REGISTRAR-PROTOCOL, SINGLE-FILE-REPO-GOAL-RESCOPE-AND-HAZARD-DOCUMENTATION, plus a framework AppBuilder/app_commands!/widget-extraction ticket). Orchestrator additionally fixed a blocking bug in `🧰️framework/⚡️implementations/🦀️rust/📜️script.ts` (wrong `root/rs`+`root/js` subdirectory assumptions broke `framework-core-rs:check/generate/lint/test` — independently hit by 3 agents; corrected to the crate's real flat layout + sibling `🟦️typescript/generated/manifest.ts` output path). Known pre-existing, unrelated, out-of-scope issues surfaced and deliberately left for W5: `@semio-tech/ui-wgpu-rs:check` (ui-axes generated file stale, confirmed pre-dates this session), a dangling Anta font path breaking all Storybook builds, a circular `ui-react`/`assets` nx dependency + stale `js/` path in mit-bestand's build script, 2 plugins with forbidden wasm-bindgen deps (grandfathered as warnings), a syntax error in 🪵️sourcing's ui lib.rs (likely concurrent human edit), and 3 pre-existing errors in the wgpu renderer crate (also concurrently staged).
- [x] W1 — pilot: 🌊️flow. Ticket `26/08/05/FLOW-PLUGIN-PILOT-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION` (closed). 8 old crates -> `semio-s-plugin-flow` at `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust`. 83/83 tests pass (baseline 44 preserved), wire-format byte-identical (verified via before/after hex dump of all 41 command variants). Fixed 4 real gaps in W0's framework prerequisites (`app_commands!` wire-key/dispatch-context/attribute-passthrough, `semio_plugin!` testkit taxonomy-awareness) plus repointed the dsl fixture-sweep crate off the deleted app facade. `📋️TEMPLATE.md` written into this ticket folder (419 lines) — the recipe for W2. **Registrar step done by orchestrator**: root Cargo.toml member lines swapped (9 old -> 1 new, bim extension line kept), `workspace.dependencies` entry renamed, `cargo metadata`/`cargo check --workspace` clean, registry regenerated (36 plugin crates), launch.json confirmed fresh (no-op). **Also fixed a real gap surfaced by this being the first real migration**: `validateConstitutionalCrates` in the registry script didn't exempt plugins already recognized under the new taxonomy contract, so it hard-failed flow (correctly missing its old 7 crate slots) instead of deferring to `validateTaxonomyTree` — now exempts migrated plugins by id; registry `check` passes clean and taxonomy tree audit is clean for flow. Full `framework-os-dev:plugin -- flow` wasm build + jco transpile succeeded via the real pipeline (not just an isolated manual approximation). Dev boot smoke: no crash after 20s (still in legitimate cold-compile of shared framework surface crates — consistent with known slow-cold-boot behavior of this dev server).
- [ ] W2 — plugin fan-out (31 remaining, 4 batches)
- [ ] W3 — framework consolidation
- [ ] W4 — mechanism finalization
- [ ] W5 — verification sweep

## Registrar Protocol

Landed in W0 (ticket `26/08/05/ROOT-WORKSPACE-DEPENDENCIES-AND-REGISTRAR-PROTOCOL`), governs root Cargo.toml/Cargo.lock ownership for W1–W3:

- **Only one serialized "registrar" agent may edit root `Cargo.toml`/`Cargo.lock` during W1–W3**, run once per plugin (W1) or per batch (W2–W3) — never concurrently with another registrar pass, matching the shared-file-ownership table above (root Cargo.toml/lock: registrar only, serialized per batch, W1–W3).
- **Migrating agents never touch root Cargo.toml/Cargo.lock.** A migrating agent creates its plugin's new crate directory (`<owner>/📦️packages/🦀️rust/Cargo.toml` per the discovery contract) and, if the future-shape glob member trick is ever wired up per-batch (see note below — it is NOT wired up yet), the new crate auto-enrolls without any root-file edit. Until then, the new crate simply isn't a workspace member until the registrar adds it.
- **Sequence per plugin/batch:** (1) migrating agent(s) build + self-verify their new crate(s) in isolation (`cargo check -p <new-crate>` against the still-unmodified root members list — the new crate is reachable via its own manifest even before being a workspace member, since `cargo check -p` on a path outside `members` still works as a standalone package check); (2) once green, the registrar deletes that plugin's OLD literal member lines from root `Cargo.toml` and adds the new crate's member line(s); (3) registrar runs `cargo metadata` to settle `Cargo.lock`; (4) registrar regenerates the plugin registry + `.vscode/launch.json` (per the mechanism-updates table) in the same pass.
- **Glob members are NOT enabled yet.** W0 tested `members` globs with emoji path segments (e.g. `✏️s/🔌️plugins/*/📦️packages/🦀️rust`) — the emoji syntax itself parses fine, but Cargo hard-errors (`failed to read .../Cargo.toml: No such file or directory`) on any glob matching zero directories, which every future-shape glob does today since no crate has moved yet. So globs can only be added incrementally, one pattern per batch, once at least one real matching directory exists — this is registrar work, not something a migrating agent enables preemptively. Until a batch's registrar pass adds the matching glob (or literal line), new crate dirs are inert to `cargo metadata`/`cargo check --workspace`.
- **`[workspace.dependencies]` adoption is opt-in per crate**, not automatic: a migrating agent MAY rewrite its new crate's `[dependencies]` entries to `dep = { workspace = true }` for names already listed in root `[workspace.dependencies]` (see the externals + internal-path survey landed in this ticket), but this still requires the registrar to already have the crate registered as a member for `cargo metadata` to resolve `workspace = true` — so in practice this rewrite happens in the SAME registrar pass that adds the member line, or after.
- **Red gate blocks the next batch.** If a registrar pass leaves `cargo metadata`/`cargo check --workspace`/`verify gate` red, no further batches dispatch until a scoped forward-fix agent (never a revert — no git-modifying commands allowed) restores green.
