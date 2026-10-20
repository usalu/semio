# Result gran-hubs: interface/implementation split of the hottest hubs (plugin, os-kernel, kernel, job, os-store-sync)

Executor `gran-hubs` (wave 1020e), stage-only. Tools: `gran-hubs-tools/` (`analysis/` measurements, `staged/` apply + plans + verification). Generated data: `🗑️generated/gran-hubs/`. Nothing applied, no cargo, no `bun repo`, no git; census and planner read only the module trees of the hubs' direct dependents (manifest graph), `rustc`/`rustfmt`/Bun TOML only on overlays under `/private/tmp/gran-hubs`.

READY: `gran-hubs-tools/staged/apply.py --plan umbrella,umbrella-dead[,job-dead,os-kernel-dead,plugin-dead]` (dry run valid on the checkout: 1,256 source files + 1,125 manifests, 606 of them rebasable onto concurrent edits). Waits for coordinator GO. Honest scope: the five hubs are not interface/implementation monoliths but **facades and pins**; the staged carve is the first slice of the facade dissolution (umbrella `semio-framework`, the only gateway to `kernel`), worth -0.06 % of total churn x reach alone and -0.71 % when its 99 residual dependents are fixed by hand (section 6). The measured upper bound of dissolving all three facades is -7.5 % to -8.5 % (section 4).

Tags: **VERIFIED** = ran or read; **INFERRED** = computed from manifests/text by me.

## 0. Answer

1. **os-store-sync is not hot.** Its dependency from `os-kernel` is `optional` behind the `sync` feature. Resolved feature-aware, its reach is **211, not 2,445** (churn x reach 107.6 k -> 9.1 k) (VERIFIED by `graphfeat.py`; the whole repo drops from 1,134,824 to 1,105,518 summed reach). The earlier graphs (audit, gran-value) ignored `optional`/feature gating.
2. **Four of the five hubs are facades or pins, not implementations.** `plugin` (1,154 re-export lines, 13 `macro_rules!`), `os-kernel` (1,029 re-export lines over ~100 `os-*` crates, plus a 549-line store component and 4 macros), `kernel` (types; reached only through the umbrella), `semio-framework` umbrella (191 re-export lines, 6 own lines). Only `job` is real implementation, and its hot history is the carve history of `WorkerJobSession` and friends, which are already in other crates.
3. **Dead edges are hygiene, not reach.** 2,287 dead or test-only hub edges removed change total churn x reach by 0.00 %: every dependent keeps another path (dense tower). They are still staged (`*-dead` groups) because they shrink 1,300 build cones.
4. **Reach is pinned by a few carriers and by macros.** Greedy cover: `os-kernel` and the umbrella reach is carried by one crate, `plugin-abi` (cone 2,240; 27 and 22 names); `job` by `os-channel` (cone 2,513, **2 names**) and 6 more; `plugin` by `semio-s-artifact-stdio-contract-wire` (cone 1,515, 4 names). Exported `macro_rules!` expand to `$crate::<module>` paths (plugin: 594 sites into 24 crates incl. `reactor-core`, `os-kernel`; os-kernel `space_members!`: 20 crates), so every macro user keeps the whole closure.
5. **What the full dissolution buys** (replayed on the feature-aware graph, W = sum of prod churn x reach over all 3,213 crates = 13.94 M): umbrella alone -0.22 % (-0.71 % when the umbrella disappears), plugin -3.6 % (macro users keep the edge) to -4.5 %, os-kernel -3.2 %, all three -7.5 % (users of own definitions keep the hub edge) to -8.5 % (pure re-export modules expanded). The remaining tower floor matches gran-value: the long chain is the value/schema/replication base, not these hubs.

## 1. Method and corrections

- `manifests.ts` (Bun TOML, 3,213 manifests): kind, `optional`, `features`, `default-features`, target cfg, `[features]` tables, workspace roots. `graphfeat.py`: per root crate built with default features, the closure with optional deps only when a feature activates them (`dep:`, `n/f`, `n?/f`), target-gated deps kept as a union. reach_f(X) = roots whose closure contains X. Unchanged for the other four hubs; **-29,306 summed reach** overall (os-store-sync and feature-gated crates).
- Churn = commits touching the crate's mounted **production** files (`#[cfg(test)]`-gated mounts and `🧪️` paths excluded, `churn_all.py`, one `git log --name-only`, 43,731 total). Note: `git log` shows 711 commits and the hub churn is dominated by the monolith-split era, e.g. the 136 k changed lines of the os-kernel store file are extraction history. Future hotness is not measurable; the ranking uses history as the brief asks.
- Source census (`census.py`, Rust skeleton reader `rs.py`, crate model `cratemodel.py`): for every direct dependent, the paths it names from the hub (use trees, inline paths, `extern crate .. as`), resolved through `pub use`, globs, `#[path]`, `include!`, inline modules, renamed dependency keys, `#[macro_export]`, proc-macro derive names, across crates to the defining crate and file. Aliases of `use` and `extern crate` are crate-wide.
- Tool bugs found on the way and fixed (so the numbers above are post-fix): identifiers containing `as` were split as aliases in use trees (`SqliteDatabaseLimits`), private-module descent was pub-only, renamed dependency keys (`replication`) did not resolve, macros were not parsed, `include!` and inline modules were ignored.

## 2. Per-hub analysis

| hub | role | prod lines (re-export / own) | churn (prod) | reach (old -> feature-aware) | direct dependents | M = churn x reach |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| `plugin` | facade over ~95 `plugin-*` + 13 exported macros | 1,154 / 1,379 | 102 | 2,130 -> 2,130 | 1,498 | 217,260 |
| `os-kernel` | facade over ~100 `os-*` + store component + 4 macros | 1,029 / 999 | 77 | 2,441 -> 2,441 | 2,125 | 187,957 |
| `kernel` | event/effect vocabulary + re-exports; only the umbrella depends on it | 205 / 1,153 | 44 | 2,290 -> 2,290 | 1 (umbrella) | 100,760 |
| `job` | real implementation (`StepContext`, protocol, failure, outcome) | 328 / 2,182 | 38 | 2,632 -> 2,632 | 1,270 | 100,016 |
| `os-store-sync` | wire sync, `sync` feature of os-kernel | 207 / 2,210 | 43 | 2,445 -> **211** | 4 | 107,580 -> **9,073** |
| `semio-framework` (umbrella) | pure facade, gateway of `kernel` | 191 / 6 | 30 | 2,289 -> 2,289 | 1,051 | 68,670 |

Dependent classes (census; macros counted as use): 

| hub | pure producer users | users of own definitions | test-only | dead (no text, no macro) | edge carries features |
| --- | ---: | ---: | ---: | ---: | ---: |
| plugin | 528 (+53 partly unresolved) | 285 + 117 mixed | 86 | 416 | 154 |
| os-kernel | 1,393 (+215 partly unresolved) | 31 + 10 mixed | 84 | 149 | 797 |
| umbrella | 399 | 5 | 92 | 547 | 0 |
| job | 22 | 194 own-only + 174 mixed | 67 | 801 | 30 |

- **plugin**: hot part is the facade itself: inline `pub mod app {` 86 of 102 commits, `plugin_runtime` 54, i.e. re-export lists edited whenever a producer is added. Own content is the 13 macros (`app_commands`, `view_commands`, `subset`, `derive_artifact_facets`, `transient_root`, ...), 2-3 commits each. Artifacts name `plugin_decl_views` (2,973 paths), `app` (module alias, 201 users), `ui_layout`, `plugin_decl_app`, `os_store_codec_registry`; they do not name the reactor/runtime crates, but the macros do.
- **os-kernel**: root 28 commits (module re-export lines), store component 76 (history). 797 of 2,125 importers declare the dependency with `features = [..]` (protocol-laws, mutation-testing, sync, ...), which forward to producer features: repointing them needs feature mapping, left fail-closed. Own users: modules `os_directory` 39, `io` 18, `os_store` 10, `os_pack`/`os_spr` 6 each; `space_members!` pins 20 `os-store-*`/`os-spr`/`job` crates.
- **kernel**: 16 own items are named through the umbrella by 97 crates, all in the plugin tower with cones >= 2,150: `Effect` 82 users, `InvocationResult` 19, `MessageEndpoint` 17, `Event` 16, `JobPlacement` 13, `TurnResult` 10. Hot items: `Effect` 9 commits, `Event` 6, `HistoryPatch`/`HistoryEntry` 6, `Budget`/`CommandContext` 5. The vocabulary is the plugin protocol itself; no cold/hot split is possible without changing the protocol. Recommendation: freeze law for `Effect`/`Event` (changes only in a ticket named `kernel-protocol-*`).
- **job**: 63 % of the 1,270 direct edges are dead (the artifact template depends on job, os-kernel, plugin, schema, ... without using them). Prod users concentrate on `OriginalTurnFailure` 333, `StepContext` 278, `InteractiveJobCloseStep` 206, `Operation` 163. Current hot item: `StepContext` (15 commits, 327 lines). Interface carve not recommended: the names used by the 7 carriers are the core types.
- **os-store-sync**: nothing to carve (section 0.1). Optional follow-up for the `sync` feature consumers: none.

## 3. Why the hubs cannot be cut by an interface crate alone

1. Carriers: reach(H) is the union of the cones of its direct dependents; for every hub the first carrier covers 94-98 % (`carriers.py`). Cutting one carrier moves nothing while the others (and the 2,000 leaf artifacts, each depending on the hub directly) remain.
2. Dissolving a facade removes its direct dependents only for importers that name nothing the facade defines. Pins found: umbrella `FRAMEWORK_RESERVED_CONTRIBUTIONS` (the only own item; users `plugin-app-builder`, `os-mcp-catalog`, `os-mcp-port`), 10 emitter macros that expand to `semio_framework::..` (in `plugin-abi`, `plugin-decl-surface`, `plugin-extension`, `plugin-job`, `os-renderer-wgpu-host-base`, 3 artifact crates) with 55 invoking crates, plugin/os-kernel macros with `$crate::` paths, 3 glob imports, 2 names that the umbrella does not even export (`RetainedToolWireInput`, `optional_json_to_dsl`: pre-existing breakage in `plugin-job` and `wgpu-host-base`).
3. Therefore the clean order is: umbrella (pure, no macro of its own) -> fix its emitter macros -> plugin/os-kernel after a decision on macro `$crate` policy (macros must emit producer paths, or the macro crates keep the closure).

## 4. Target structure and expected churn x reach

All numbers replayed on the feature-aware graph (`rewire*.py`, `scen_*.py`, `sim_plan.py`); W baseline 13,943,601.

| scenario | W | hub reach after |
| --- | ---: | --- |
| feature-aware correction (no work) | -98 k on os-store-sync alone | os-store-sync 2,445 -> 211 |
| dead + test-only hub edges removed (4 hubs, 2,287 edges) | 0.00 % | plugin 2,128, os-kernel 2,426, job 2,624 |
| **staged** umbrella (412 crates repointed) + umbrella-dead (540) | **-0.06 %** | umbrella 2,289 -> 2,244, kernel 2,290 -> 2,264, abi 2,293 -> 2,249 |
| umbrella fully dissolved (residual 99 fixed, const relocated to {time-travel-actions, tool-run-actions, manifest-app-surface}) | **-0.71 %** (-99 k) | umbrella -> 0 (30 x 2,289 = 68.7 k gone), abi 2,293 -> 519, source-projection 2,290 -> 1, action-bus 2,292 -> 2,206, kernel -> 2,262 |
| plugin dissolved, macro users keep the edge / pure modules expanded | -3.6 % / -4.5 % | plugin 2,130 -> 1,826 / 1,587; plugin-reactor-turn 2,139 -> ~1,640 |
| os-kernel dissolved | -3.2 % | os-kernel 2,441 -> 2,278 / 2,235 |
| all three | -7.5 % / -8.5 % | plugin 1,826 / 1,587, os-kernel 2,205 / 2,145, kernel 2,176, umbrella 2,147 (own users) |

Target structure per hub:
- `semio-framework` umbrella: deleted. `FRAMEWORK_RESERVED_CONTRIBUTIONS` becomes a const of a new leaf crate `semio-framework-manifest-reserved` (deps: the two `RESERVED` owners and `manifest-app-surface`); importers name `semio_framework_<producer>` directly.
- `kernel`: stays the protocol vocabulary crate, frozen; every module re-export of other crates in it (67 of 94 names) moves to the importers.
- `plugin`: becomes macro-only (`plugin` keeps the 13 macros and a `#[doc(hidden)]` private module the macros use); `app`, `plugin_runtime`, `world3d_host`, `owned_abi` re-export lists disappear (hub-own churn 102 -> ~12). Artifacts depend on `plugin-decl-*` plus the macro crate. The macro `$crate` paths decide the floor (24 crates).
- `os-kernel`: macro and store-component crate only; 1,029 re-export lines disappear; importers name `os-spr`, `os-pack`, `os-store-*`. The 797 feature-carrying edges need an explicit feature map per producer.
- `job`: only hygiene (771 dead edges). No split.
- `os-store-sync`: unchanged.

Hub-own churn x reach after the full target (estimate, INFERRED from item churn): plugin 217 k -> ~20 k (churn 102 -> ~12, reach ~1,830), os-kernel 188 k -> ~60 k (store-component churn stays), umbrella 68.7 k -> 0, kernel and job unchanged, os-store-sync 9 k.

## 5. Staged carves (`gran-hubs-tools/staged/`)

`python3 apply.py --plan <tag>[,<tag>...] [--root DIR] [--write] [--ledger DIR] [--status] [--files] [--only-crates a,b]`

| plan | content | numbers |
| --- | --- | --- |
| `umbrella` | importers of `semio-framework` repointed to the producer crates; hub line replaced by producer lines (workspace style where the workspace declares the producer, relative path otherwise; test-only producers to `[dev-dependencies]`) | 412 crates, 1,256 .rs files (4,133 edits), 412 manifests, 1,106 added edges, 0 cycles |
| `umbrella-dead` | manifest line removal for 540 dependents that never name the crate nor invoke its macros | 540 manifests |
| `job-dead` | same for job | 771 manifests |
| `os-kernel-dead` | same (feature-carrying and multi-table entries kept) | 34 manifests |
| `plugin-dead` | same | 2 manifests |

Rules: dry run by default; `--write` validates every file first, then writes atomically with first-touch backups in the ledger, reads back, rolls back on any mismatch; an applied tree refuses. Anchoring: a pristine file must match its planned sha256 and the planned result digest; a file changed since (for example by gran-value's `apply2`) is **rebased**: every edit is located as the n-th occurrence of its exact old text (the count must be unchanged), manifests by exact-once line operations; anything else is a drift refusal. Several plans may touch one file (applied in order). Pure standard library (macOS, Linux, Windows, devcontainer). Split rules (`repoint_core.py`): comments/strings never edited; `use` trees split by owner with attributes and visibility copied (inner attributes not); glob and whole-crate imports, comments inside a statement and unresolved names fail the whole **crate**; crates with own-definition users, feature-carrying hub edges, renamed entries, several non-dev tables, or invocations of macros that expand to hub paths stay untouched (79 crates). No compat re-exports are added; the umbrella is not deleted by this plan.

### Verification (VERIFIED, all on the final plans)

- `apply.test.py`: 15 language-agnostic cases (`cases.json`: use split, inline, colon-prefixed field type, own definitions kept, module alias, `extern crate` alias, crate alias leaf, glob/comment/unresolved refusals, strings untouched, inner attributes; 4 manifest cases), cross-checked with `rustfmt` (expected outputs parse) and Bun's TOML parser (added/removed keys): pass.
- `verify_manifests.py <tag>`: Bun TOML of all 1,759 planned manifests: after-text parses, only the hub key removed, every added key is a workspace-declared producer (the owning root declares it) or a path to a crate of that name, no other value changed: 0 problems for all five plans.
- `verify_rustfmt.py umbrella`: all 1,256 rewritten files still parse under `rustfmt` (0 regressions, 0 unparsable before). It found two real bugs (a crate-alias leaf produced `{, X}`; an inner attribute `#![cfg(test)]` was copied onto split statements); both fixed and in the cases.
- `cycle_check.py umbrella`: 0 cycles among 1,106 added edges.
- Oracle differential (`overlay.sh umbrella`, POS union sandbox `rustc +nightly-2026-07-20 --emit=metadata`, lib cfg, family of 35 crates: umbrella + its 27 producers + 7 sampled importers incl. `plugin-instance-app`, `plugin-decl-registry`, `plugin-host-wit`): pre **8,278** errors, post **8,278**, **0 new, 0 lost** (key = code + normalised message + label, `Tr<n>` placeholders normalised).
- Seeded regressions on the post overlay (`seed.sh`): wrong owner crate for `AppDefinition` -> **+1 E0432**; missing item -> **+4 (E0432, E0425)**. An earlier seed with a private import gave E0603. Resolution and privacy classes both fire.
- Rebase test: with a comment line prepended to a planned file and manifest in a copy of the overlay, the plan applied (2 rebased), a second run refused.
- Not verifiable without cargo: feature resolution after the manifest edits, `--locked` lock state (the Cargo.lock of every touched workspace changes; lockref after apply), tests with glob/trait-method use through removed globs (globs are refused, not guessed), wasm/p2 cfg variants of a re-export, doc-comment intra-doc links to `semio_framework::..` (left, rustdoc warnings only).

## 6. GO sequence and coordination

1. After cargo-fix's compile window (static fixes first), before `lockref/all.sh`: `python3 gran-hubs-tools/staged/apply.py --plan umbrella-dead,umbrella --write` (then `job-dead,os-kernel-dead,plugin-dead` if wanted). Then re-run `bun repo cargo --store=shared ... check` for `--only-crates` lists from `plans/*.json` (`crates` key) and `bun test` for `crate-boundaries`.
2. gran-value's `apply2` lists 579 files; **264 of them are also in my plans** (149 manifests of `umbrella`, 115 of `umbrella-dead`; no `.rs` overlap checked beyond the manifest paths). Both appliers recompute from the live tree; mine rebases line-level edits, so either order works. Applying `umbrella*` after `apply2` is the safer order (their manifest edits are single-line removals and added owner lines).
3. Residual 99 umbrella dependents (INFERRED list: `residual.py umbrella umbrella,umbrella-dead`): 55 invoke the 10 emitter macros (fix the macros to emit `semio_framework_<producer>` paths, then re-plan), `plugin-app-builder`/`os-mcp-catalog`/`os-mcp-port` need the relocated const, `plugin`, `plugin-retained-command`, `os`, `wgpu-shell-window-actions` import by glob, `plugin-job`/`wgpu-host-base`/`os-run` name items the umbrella does not export (broken today), 20 manifests where the removed line is not unique. Only after these are done delete the umbrella crate (workspace members, `crate-boundaries` rows).
4. plugin and os-kernel planner runs (not staged): 44 of 1,498 and 117 of 2,125 crates plan cleanly. Blockers: plugin `app` module alias resolution (505 files), feature-carrying edges (plugin 154, os-kernel 797), os-kernel store-alias globs and `protocol::` aliases (208+). Re-plan after the macro policy decision.

## 7. Open items

- Decision for the macro `$crate` policy (macro crates keep their closure) and for the feature maps of os-kernel importers: they bound everything after the umbrella.
- `audit-granularity-1020e.md` and `result-gran-value.md` should use the feature-aware reach (os-store-sync) and prod-only churn.
- One minute of overlap with a second process (`churn_all.py` during a census run) early on; otherwise one process at a time under `nice -n 10`.
- Reproduce: `analysis/manifests.ts` -> `graphfeat.py` -> `macro_emit.py` -> `run_census.sh` -> `run_plans.sh` / `plan_dead.py` -> `assemble_plan.py <hub>` -> `staged/overlay.sh umbrella` -> `staged/seed.sh`; scenarios `scen_all.py`, `scen_umbrella.py`, `sim_plan.py`. Generated data is in `🗑️generated/gran-hubs/` (delete when the ticket closes).
