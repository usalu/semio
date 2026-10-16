# Result: plugin SDK layering relocation (slice E)

**Updated verdict (follow-up, 2026-10-15):** items 1–3 are done. `ui_wgpu` is now removed from the plugin crate too, and the combined slice A + slice E differential shows no regression; details in the follow-up section at the end. W5 is still not done.

Executor: `relo-pluginsdk` (Opus 5.5, foreground, no sub-agents), 2026-10-15. Brief: `📓️RELOCATION-BRIEF.md`.
Paths: `plugin/` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`, `host/` = `plugin/🖥️host/`.

## 0. Verdict

Items 1, 2 and 3 are done. W5 (item 4) is not done (section 6).

- **Item 1 (tool-run and time-travel in the generic instance): done for routing, freezes, maintenance and framework bodies.** The new neutral extension point is `InstanceExtension`. The instance consults the list registered at the composition root, `crate::InstanceExtensions`, and the two domains register through it. The instance's tool-run and time-travel turn modules moved out of `🧵️instance/` into the domain modules. The instance's `use` block no longer imports `tool_run_contract::*`, `tool_jobs::{time_travel, tool_run}`, `is_time_travel_action_id` or `is_tool_run_action_id`, and `🧷️command-routes` no longer names either domain at the dispatch head. Some instance state still names the domains, because contracts owned by other slices or peers require it (section 6).
- **Item 2 (`ui_wgpu`): done; `ui_wgpu` is removed from the plugin crate as well (see the follow-up).** The plugin crate and the plugin host no longer depend on `ui_wgpu`. Every item they used is now imported from, or re-exported out of, the crate that defines it: `semio-framework-ui-layout`, `semio-framework-ui-declaration` or `semio-framework-ui-scene`. The crate-root glob is replaced by explicit re-export lists. Nothing renderer-specific remains.
- **Item 3 (`os_vcs` in `GuestRuntime`): done.** `GuestRuntime` and every host signature now name the owning neutral contract crate, `semio-framework-os-vcs`, directly. That is the framework-role crate `semio-framework-os-kernel` itself re-exports as `os_vcs`, so no conversion is needed at the os-kernel boundary: it is the same type. The `os_vcs`, `opening_config` and `config_io` re-exports are removed, and every importer now points at the owning crate.

No compiler was available. In the first round the sanctioned launcher failed before rustc for all three touched crates (section 5.6). In the follow-up it got as far as the compiler-ingress gate, which refused the plugin crate (follow-up section F.5). All evidence is static: the patched full-resolution differential (re-run against a combined slice A + slice E baseline in follow-up section F.1), a definition-level privacy scan with a seeded regression, rustfmt, path and emoji checks, direction scans, and TS test/tsc/dependency-cruiser comparisons before and after.

## 1. Design decisions and owner evidence

### 1.1 Item 1: a neutral instance extension point

Evidence:
- **Who implements the behaviour.** The tool-run turns (ledger driving, tick folding, finalize) and the time-travel turns (history-edit sessions, replay, supersession) were inherent `impl VcsArtifactApp` blocks in `🧵️instance/⏯️tool-run` and `🧵️instance/⏪️time-travel`, mounted as `mod tool_run_turns` and `mod time_travel_turns`. Their domain modules `plugin/⏯️tool-run` and `plugin/⏪️time-travel` (mounted as `tool_jobs::tool_run` and `tool_jobs::time_travel`) already own the ledgers, drivers and pure-session glue.
- **Who consumes it.** The instance dispatch head, the reserved-route freeze, the emit freezes in `📨️dispatch` and `🧾️typed-operations`, the tool machine, and the `PluginApp` maintenance pump, close predicate and render path in `🔌️plugin-app-impl`.
- **Layering.** Both domains sit in the same crate and the same layer (os product). The defect was naming: the generic instance hard-coded the domains. It was not a cross-crate direction edge.

Design (principle 1, "extension trait"):
- `🧵️instance/🧩️extensions/🦀️.rs` defines `pub(crate) trait InstanceExtension`. Every hook has a neutral default, and every app-taking hook is generic over `<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>`. The hooks:
  - `claims_action`, `reads_history`, `admitted_while_loading`;
  - `async dispatch_action`;
  - `reserved_action_refusal`, `mutation_refusal`, `freezes_document`, `document_refusal`, `transaction_refusal`;
  - `window_config_published`;
  - `has_pending_work`, `holds_receipt`, `terminal_is_empty`, `async drive_turn`;
  - `render_body`, `after_app_render`.
- A registered list is a right-nested tuple. `impl InstanceExtension for ()` contributes nothing. `impl<First, Rest> InstanceExtension for (First, Rest)` consults `First` before `Rest`: the first claimant of a route, refusal or body wins, predicates are or-ed (`terminal_is_empty` is and-ed), and maintenance turns run in order.
- **Composition root.** `plugin/🦀️.rs` (next to the `artifact_instance` mount) has `pub(crate) type InstanceExtensions = (tool_jobs::tool_run::ToolRunExtension, (tool_jobs::time_travel::TimeTravelExtension, ()));`. The order matches the original code: the tool-run arm and pump ran before time travel.
- **Static dispatch only (no `dyn Future`).** A boxed trait-object future would need a `Send` bound proven generically over `A` and `M`, or would silently make native futures `!Send`. Trait methods resolved on the concrete list type keep auto-trait leakage exactly as the inherent `async fn`s had it. The head still boxes the routed future (`Box::pin(InstanceExtensions::dispatch_action(..))`), as the time-travel arm did before.
- **Retained grant.** The maintenance-turn hook takes `retained: &mut RetainedCloneGrant`, and the combinator refreshes it with `cx.retained_grant()` before each entry. This reproduces the original sequence exactly:
  - tool run used the entry grant, which is identical because nothing between the function entry and the tool-run block consumes budget;
  - time travel used a grant refreshed after the tool-run block;
  - the local-interaction query then used that same pre-time-travel grant.

What each domain registers:
- `ToolRunExtension`:
  - claims `TOOL_RUN_ACTION_IDS`;
  - `mutation_refusal` returns `toolRun.busy`;
  - `window_config_published` calls `note_window_config_published`;
  - the pending, receipt and terminal predicates come from the ledger;
  - `drive_turn` is the former pump block, receipt retention included;
  - `render_body` renders the `FRAMEWORK_TOOL_RUN_BODY_KEY` panel;
  - `after_app_render` is the trace-lane injection.
- `TimeTravelExtension`:
  - claims and reads history for `HISTORY_EDIT_ACTION_IDS` (via `is_time_travel_action_id`);
  - admits `historyEditCancelReplay` while the document loads;
  - `dispatch_action` freezes the tool machines, then dispatches (as the old arm did);
  - its refusal hooks return `timeTravel.frozen` for reserved verbs, document emits and foreign transactions;
  - the pending, receipt and terminal predicates come from the ledger;
  - `drive_turn` is the former pump block.

### 1.2 Item 2: `ui_wgpu` types

`ContextMenuItemSpec`, `ContextMenuRequest`, `WindowEngagement`, `WindowMeasure`, `FRAMEWORK_HISTORY_BODY_KEY`, `organize_context_menu` and the reactor's `ContextMenuPoint/Response/SurfaceTarget` and `UiMenuRef` are all defined in `🧰️framework/🔨️modules/🖱️ui/📜️declaration/📐️layout/🦀️.rs`, `pub mod layout`. That is crate `semio-framework-ui-layout`, role framework, renderer-independent, with dependencies on the icon contract, the ui contract, value and serde only. `ui_wgpu::wgpu` merely re-exports them (`pub use component::layout::{..}` in `🎯️targets/🧊️wgpu/🦀️.rs`, where `component` = `semio_framework_ui_declaration`, which in turn re-exports `semio_framework_ui_layout::layout`). So nothing had to move: consumers now point at the owner.

### 1.3 Item 3: `GuestRuntime`'s identity authority

`EntityIdentityAuthority` is defined in `💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🦀️.rs`, crate `semio-framework-os-vcs`. That crate is role framework, described as "Independent vcs protocol and authoring boundary", and depends on framework crates only. `semio-framework-os-kernel` re-exports it as `pub use semio_framework_os_vcs as os_vcs;`.

The authority is a capability loan (`&mut EntityIdentityAuthority<'_>` threaded through wasmtime calls), not a value, so a wrapper "port type" would only alias it. The neutral contract already exists, so `GuestRuntime` names it directly and the host depends on `semio-framework-os-vcs`. Because it is the same type, the os kernel, the hub, the renderer and os-host implementors of `GuestRuntime` that still spell it through `semio_framework_os_kernel::os_vcs::…` stay compatible, with no conversion.

Three host-root re-exports were removed:
- `pub use semio_framework_os_kernel::os_vcs;`
- `pub use semio_framework_os_config::opening_config;`
- `pub use semio_framework_os_config::io as config_io;`

The six `mutate-os-config-*` cases under `host/🧪️tests/` reach their subject only through the owner's package (the generated test host links the owner ancestor's crate). I re-pointed them to `semio_framework_os_config::…` and declared `semio-framework-os-config` as a subject host package in `host/🔮️oracles/🔣️.json`; the schema's `SubjectHostPackage` has all four required fields. I did not move the cases to `💻️os/🎚️config/🧪️tests/`, because `taxonomy.json` registers them under the `plugin-contract-tests` owner-kind group. That is a follow-up (section 6).

## 2. Moved items (from → to)

| Item | From | To |
|---|---|---|
| Instance time-travel turns (1,368 lines, two `impl VcsArtifactApp` blocks) | `plugin/🧵️instance/⏪️time-travel/🦀️.rs` (`artifact_instance::time_travel_turns`) | `plugin/⏪️time-travel/🧵️instance/🦀️.rs` (`tool_jobs::time_travel::instance_turns`) |
| Instance tool-run turns (1,116 lines) | `plugin/🧵️instance/⏯️tool-run/🦀️.rs` (`artifact_instance::tool_run_turns`) | `plugin/⏯️tool-run/🧵️instance/🦀️.rs` (`tool_jobs::tool_run::instance_turns`) |
| `time_travel_freezes_reserved_action`, `time_travel_frozen_fault` | instance parent `🧵️instance/🦀️.rs` | `⏪️time-travel/🧵️instance/🦀️.rs` (private) |
| Dispatch-head arms (`is_tool_run_action_id` / `is_time_travel_action_id`, `freeze_tool_machines`) | `🧷️command-routes` | `ToolRunExtension::dispatch_action`, `TimeTravelExtension::dispatch_action` |
| Reserved-verb freeze | `🧷️command-routes`, `🚪️ingress` | `TimeTravelExtension::reserved_action_refusal` |
| Emit freezes (`toolRun.busy`, `timeTravel.frozen`) | `📨️dispatch`, `🧾️typed-operations`, `🛠️tool-machine` | `mutation_refusal` / `document_refusal` / `freezes_document` |
| Transaction-prepare freeze | `🔌️plugin-app-impl` | `TimeTravelExtension::transaction_refusal` |
| Maintenance pump blocks, pending/receipt/terminal predicates | `🔌️plugin-app-impl` | `drive_turn`, `has_pending_work`, `holds_receipt`, `terminal_is_empty` |
| Tool-run panel body and trace lane | `🔌️plugin-app-impl::render` | `ToolRunExtension::render_body` / `after_app_render` |
| Loading exemption `HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID` | `🧷️command-routes::document_loading_refusal` | `TimeTravelExtension::admitted_while_loading` |
| Window-config publication note | `📨️dispatch`, `🧾️typed-operations` | `ToolRunExtension::window_config_published` |

The moved files keep every original body line. Losslessness was checked on the non-blank line multiset: 3 lines lost per file, namely the old header docstring, the `use crate::tool_jobs::<domain>::*` line(s) now covered by `use super::*`, and in time travel a stray `//#endregion 🔖️HistoryViewStamps` that closed a region opened in the other file. The added lines are new imports, the docstring, the extension impl and the two helpers.

## 3. Registration point and its tests

- `plugin/🧵️instance/🧩️extensions/🧪️tests/🦀️.rs`: probe extensions `ProbeAlpha`/`ProbeBeta`:
  - `empty_list_claims_no_route`;
  - `registered_entry_is_honoured`, which proves a non-default implementation is honoured;
  - `every_entry_of_a_list_contributes`, which covers composition order and or-ing.
- `plugin/⏪️time-travel/🧵️instance/🧪️tests/🦀️.rs`:
  - `registered_extensions_route_every_history_edit_verb`;
  - `only_replay_cancellation_is_admitted_while_loading`.
- `plugin/⏯️tool-run/🧵️instance/🧪️tests/🦀️.rs`:
  - `registered_extensions_route_every_tool_run_verb`.
- **Not run.** There is no compiler (section 7.6). These tests check routing; hooks that need a live instance are covered only by the existing end-to-end tests (`🧪️tests/🧪️time-travel`, `🧪️tests/🔬️tool-run`).

## 4. Every edited file

Plugin crate (`semio-framework-plugin`):
- **New:**
  - `🧵️instance/🧩️extensions/🦀️.rs`, `…/🧪️tests/🦀️.rs`;
  - `⏪️time-travel/🧵️instance/🦀️.rs` (moved) and `…/🧪️tests/🦀️.rs`;
  - `⏯️tool-run/🧵️instance/🦀️.rs` (moved) and `…/🧪️tests/🦀️.rs`.
- **`🧵️instance/🦀️.rs`:**
  - removed the imports (four `tool_run_contract` types, `FRAMEWORK_TOOL_RUN_BODY_KEY`, `is_time_travel_action_id`, `is_tool_run_action_id`, `tool_jobs::{time_travel, tool_run}`), the two domain mounts and the two freeze helpers;
  - added `mod extensions; pub(crate) use extensions::InstanceExtension; use crate::InstanceExtensions;`;
  - widened 8 fields to `pub(crate)`: `history_backfill`, `live_runtime_instance_id`, `pending_render_tree`, `pending_transaction`, `shell_undone`, `supersedes`, `typed_effect_outbox`, `typed_ui_outbox`;
  - re-pointed the five `ui_wgpu` imports to `semio_framework_ui_layout::layout`.
- **Instance children:**
  - `🧷️command-routes`: head route, history gate, reserved freeze, loading exemption; `declared_action_kind` widened to `pub(crate)`;
  - `🚪️ingress`: reserved freeze, `use crate::tool_jobs::time_travel;` for the replay budget; `deliver_base_moved`, `deliver_host_event_to_every_window`, `publish_member_content`, `reclaim_document_snapshot_read_returns` and `send_member_lane` widened to `pub(crate)`;
  - `📨️dispatch`: emit freezes, config note, `use crate::tool_jobs::time_travel;` for the history helpers; `child_history_tails`, `empty_result` and `transaction_fault` widened;
  - `🧾️typed-operations`: emit freezes, config note;
  - `🛠️tool-machine`: three freeze sites; regions balanced;
  - `🖼️render`: `interaction_selection_snapshot` and `revalidate_interaction_on_document_change` widened;
  - `🔌️plugin-app-impl`: close predicate, pump, pending predicates, transaction freeze, tool-run body, trace lane, `TOOL_RUN_TRACE_DELTA_BYTES` import, `organize_context_menu` from ui-layout; `history_changed_event` widened to `pub(crate)`.
- **Domain module roots:**
  - `⏪️time-travel/🦀️.rs` mounts `instance_turns` and re-exports `TimeTravelExtension`; it also had pre-existing region defects, now fixed: the unclosed `🔖️Limits` is closed, and `🔖️HistoryViewStamps` is closed before `🔖️Driver`;
  - `⏯️tool-run/🦀️.rs` mounts `instance_turns` after the `selected_entry*` macros (textual macro scope) and re-exports `ToolRunExtension`; its pre-existing region defects are fixed too: the unopened `🔖️Limits` now has an opening marker, and the `🔖️Selection`/`🔖️Request` mismatch is resolved.
- **Other plugin files:**
  - `plugin/🦀️.rs`: the `InstanceExtensions` alias; the `plugin_app_close_prelude` re-export now uses ui-layout;
  - `🪜️maintenance-ladder/🦀️.rs`: `use crate::tool_jobs::time_travel;`;
  - ui-layout re-points: `⚛️reactor/🏃️runtime/🦀️.rs`, `⚛️reactor/🔌️app/🦀️.rs`, `🎮️command/🦀️.rs`, `👷️job/🦀️.rs`, `📜️declaration/🧩️app/🦀️.rs`, `📜️declaration/🪟️surface/🦀️.rs`, `🧪️testing/🦀️.rs`, `🪟️window-kits/🦀️.rs`;
  - `📦️packages/🦀️rust/Cargo.toml`: added `semio-framework-ui-layout`; the `ui_wgpu` comment now lists its remaining users.
- **Source-text readers re-pointed:**
  - `🧵️retained-command/🪟️mounted/♻️frontier/🧪️tests/🟦️.ts`: the receiving law now reads the pump, `ToolRunExtension::drive_turn` and the list's grant refresh;
  - `⏯️tool-run/👷️job/🎟️receiving/🧪️tests/🟦️.ts`: reads the new path `../../../🧵️instance/🦀️.rs`;
  - `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`: the history-edit route assertion now names the extension route; its pre-existing unopened `🧪️TestClipboardReservedJob` region now has an opening marker.

Plugin host (`semio-framework-plugin-host`):
- `📦️packages/🦀️rust/🦀️.rs`: the three re-exports are removed.
- `📦️packages/🦀️rust/Cargo.toml`:
  - removed `ui_wgpu` and `semio-framework-os-config`;
  - added `semio-framework-os-vcs`;
  - added dev-dependencies `semio-framework-ui-declaration` and `semio-framework-ui-layout`.
- `semio_framework_os_kernel::os_vcs::` / `crate::os_vcs::` became `semio_framework_os_vcs::` in:
  - `🎭️guest-runtime/🦀️.rs` (32 sites, the whole `GuestRuntime` trait and its impls, the test-only impl included);
  - `🦀️.rs`, `🧵️shard/🦀️.rs` (5), `🧵️shard/🪪️identity/🦀️.rs`, `🚪️io/🎛️operation/🦀️.rs` (2), `🚪️io/🪶️snapshot/🦀️.rs`;
  - `🧪️tests/🛂️receiving/🦀️.rs`, `📥️ui-patch/🧪️tests/🧪️component/🦀️.rs`.
- `🧪️tests/🔬️app-router/🦀️.rs`: `SurfaceKind` now comes from `semio_framework_ui_declaration::ui`, `WindowOptions` from `semio_framework_ui_layout::layout`.
- The six `🧪️tests/*mutate-os-config-*/🦀️.rs` subjects (plus `🎨️…/🧩️support` unchanged) now use `semio_framework_os_config::{io, opening_config}`. Their stale placement prose is updated in the six `🥒️.feature`, three `🦀️.rs` and two `🟦️.ts` docstrings.
- `🔮️oracles/🔣️.json`: `subjectHostPackages` entry for `semio-framework-os-config`.

OS run (`semio-framework-os-run`):
- `🔨️modules/🏃️run/🦀️.rs` (6 sites) now uses `semio_framework_os_config::opening_config::…`.
- Its `Cargo.toml` adds `semio-framework-os-config = { workspace = true }` to the native target table, beside `semio-framework-plugin-host`.

Importer search (repo-wide, `git grep`):
- Nobody else imported `semio_framework_plugin_host::{os_vcs, config_io, opening_config}`. The `✏️s` plugins, `🌎️hub`, `📺️renderer`, `🌉️mcp` and `🖥️host/🎠️activation` import only `GuestRuntime`, `PackageRef` and so on, and the spelling of the authority type in their `GuestRuntime` impls stays valid.
- Nothing outside the plugin crate names any moved item. The new names (`InstanceExtension`, `InstanceExtensions`, `TimeTravelExtension`, `ToolRunExtension`) have no repo-wide collisions.

## 5. Verification (commands and real results)

### 5.1 Patched full-resolution differential (`relo-pluginsdk-tools/orc.py`)

Method:
- `orc.py build` copies the plugin source tree into a sandbox: the baseline snapshot taken before my first edit (`scratchpad/base`, archived as `🗑️generated/relo-pluginsdk-baseline.tar`), or the live tree at check time. Everything else is symlinked to the repo.
- It applies the oracle patches identically to both trees: it strips 12 `extern crate` lines and neutralises 27 external globs.
- `orc.py run` executes `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null --error-format=json --cap-lints allow [--test] [--target …] [--cfg feature=…]` from the real lib root.
- `orc.py diff` compares error multisets keyed by (code, message, label, snippet). The file is left out of the key so that moved code compares.
- `--non-extern` drops only errors whose label is "unresolved module or unlinked crate" (external crates are never passed) and the final "aborting" count.
- Determinism check: baseline against the live tree before any edit gave native 9243 / 9243 errors and 0 differing keys.

Plugin crate (`semio_framework_plugin`, lib root `plugin/📦️packages/🦀️rust/🦀️.rs`), baseline / final:

| Config | All errors | Non-extern errors | Differing non-extern keys |
|---|---|---|---|
| native | 9243 / 9258 | 1461 / 1461 | 0 |
| `--test` | 20830 / 20845 | 2772 / 2772 | 0 |
| `--target wasm32-wasip2` | 9534 / 9549 | 1538 / 1538 | 0 |
| `--test --target wasm32-wasip2` | 21115 / 21130 | 2848 / 2848 | 0 |
| `--target wasm32-unknown-unknown` | 9257 / 9272 | 1461 / 1461 | 0 |
| all four features native | 10091 / 10106 | 1507 / 1507 | 0 |
| all four features `--test` | 20853 / 20868 | 2772 / 2772 | 0 |

The features are `artifact-app-testing`, `media-turn-testing`, `reserved-producer-testing` and `original-actor-context-testing`. The +15 all-error delta is the same in every configuration.

Every full-key difference falls into one of two classes, and both cancel under `--non-extern`:
- Unresolved-external-crate keys gained from new import lines and path spellings in the new or moved files. The per-file attribution in native is all in the moved files, `🧩️extensions` and the one fewer `semio_framework::` path in `🧷️command-routes`:
  - `semio_framework_os_kernel` E0433 +3;
  - `semio_framework_job` E0433 +7;
  - `semio_framework` and `store` E0432 +2 each;
  - `semio_framework_ui_contract` E0432 +1.
- The crate-name substitution `ui_wgpu` → `semio_framework_ui_layout` (26 ↔ 26).

No new E0412, E0425, E0422, E0423, E0432, E0603, E0616 or E0624 against an internal path appears.

Iterations:
- The first post-move run found 26 groups of unresolved names in the moved files (`VcsArtifactApp`, `ActionMeta`, `SpaceMember`, `MemberFactory`, `ViewModel`, `HISTORY_ACTION_IDS`, `history_changed_event`, …). I fixed them with explicit imports, and `history_changed_event` was widened to `pub(crate)`.
- One more group came from the external glob (`BuiltNode`, now imported explicitly in `🧩️extensions`).

Host differential (baseline against the live tree after items 2 and 3):
- native: 1335 / 1333, non-extern identical;
- `--test`: 2797 / 2796, non-extern identical.
- The remaining full-key differences are the crate renames:
  - `semio_framework_os_kernel` → `semio_framework_os_vcs` (30+1 ↔ 31 native, 42+1 ↔ 44 test);
  - the removed `os_config` re-export imports (−2);
  - `ui_wgpu` → `semio_framework_ui_declaration`/`semio_framework_ui_layout` in the app-router test.

### 5.2 Definition-level privacy scan (`relo-pluginsdk-tools/visscan.py`)

The scan covers every `VcsArtifactApp` field, every inherent method and associated fn (544 names across the instance subtree, its ladders and `include!`d files, and the moved files), and every instance-parent free item.

- The first run gave 22 findings: 8 fields and 13 methods (two methods used from both files), all of them `pub(super)` or private.
- After widening to `pub(crate)`, the scan gives 0 findings.
- **Seeded regression:** `--strip-vis tool_runs,freeze_tool_machines,transaction_fault,mounted_policy` reports all 4 seeded names (5 findings).
- The reverse check (moved methods called from the instance with a visibility that does not reach it) gives 0.

### 5.3 rustfmt, regions, paths, emoji

- `rustfmt +nightly-2026-07-20 --edition 2021 --emit stdout` exits 0 on all 18 touched Rust module roots, the new and moved files included.
- Region balance over every touched Rust file is OK, except one pre-existing defect I did not touch: `📜️declaration/🧩️app/🦀️.rs` has an unopened `🔖️Limits`. I only changed four import lines in that file, which a peer or slice A owns.
- Longest new path: 129 bytes.
- Emoji `🧩`, `🧵`, `🧪`: `/^\p{RGI_Emoji}$/v` true; all are already used as folder prefixes; no duplicate prefix among siblings in `🧵️instance`, `⏪️time-travel`, `⏯️tool-run` or the new folders.

### 5.4 Direction checks

- **Cargo:** I scanned every `path =` dependency in the three touched `Cargo.toml` files. 0 point upward. The plugin, the host and os-run sit in the os product layer; ui-layout and ui-declaration are framework; os-vcs and os-config are os product.
- **Rust source:** every `#[path]`/`include*` target in the 46 touched or new Rust files stays under `🧰️framework/`, and none uses a higher-layer crate name. One hit is a pre-existing doc comment in `🏃️run/🦀️.rs` mentioning a "semio_hub backbone".
- **dependency-cruiser** on the four touched TS files: 293 violations after. All are pre-existing and transitive (ui contract → `📦️packages/🟦️typescript`). The sandbox does not follow symlinks, so a baseline run there is not comparable. Instead I confirmed that the import set of all four files is byte-identical before and after, so no edge was added or removed.

### 5.5 TS tests and tsc

- `bun test` on the 18 affected TS files (all instance-list readers, the four time-travel/tool-run/tool-machine readers, and the four host readers): pass/fail counts and failing test names are identical in the baseline sandbox and the repo, with 31 pre-existing failures. The re-pointed frontier receiving law passes before and after.
- Scoped `tsc --noEmit --skipLibCheck --moduleResolution bundler …` on the four touched TS files: 49 unique errors before and after, all in dependencies, identical sets.

### 5.6 Sanctioned cargo launcher

`cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --message-format=short` was tried once each, for `semio-framework-plugin` (before editing), `semio-framework-plugin-host` and `semio-framework-os-run`. All three exited 1 before rustc with `error: Finite original maintenance authority required` (cache-maintenance step in `⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts:43`). Logs: `🗑️generated/relo-pluginsdk-cargo-*.log`. Nothing was compiled around the gate. The Cargo lock refresh (`… -- update --workspace --offline`) was not possible, so `Cargo.lock` still lacks the new edges (plugin → ui-layout, host → os-vcs and the dev ui crates, os-run → os-config) and the host's dropped ones. It needs the next refresh.

## 6. Residual risks and follow-ups

- **Typeck is unproven.** The oracle cannot reach typeck because externals are unresolved. Unproven points:
  - signature equality between the trait and its impls;
  - AFIT auto-trait leakage through the tuple impl (it should leak exactly as the inherent `async fn`s did);
  - borrow splitting at the new call sites: whole-app `&self` hooks are called where the original used disjoint field borrows. I checked each site by reading. In `settle_gesture_slot` the freeze is computed before the `tool_machines` mutable borrow, and in `🧾️typed-operations` `mounted` is a separate `&mut` parameter, not a borrow of `self`.
  Run `check -p semio-framework-plugin` (native, `--tests`, `--target wasm32-wasip2`) and `-p semio-framework-plugin-host` (native, `--tests`) first thing once the launcher works.
- **Instance state still names the domains.** These are contracts owned by others; moving them would mean editing the reactor's `PluginApp`, the declaration layer and the protocol presence:
  - the fields `tool_runs: ToolRunLedger<A>` and `time_travel: TimeTravelLedger<A>`;
  - the `PluginApp` contract methods `take_tool_run_receipt`, `take_history_command_receipt`, `tool_run_presence` and `tool_run_trace_delta`;
  - `EphemeralSnapshot { tool_run, history_edit }`;
  - `ArtifactView::with_tool_run(..)`;
  - the render overlays `time_travel.render_snapshot_or(tool_runs, …)` and `draft_references`;
  - the history panel body (`FRAMEWORK_HISTORY_BODY_KEY` with the time-travel panel);
  - the `history_unavailable` flag kept inside `TimeTravelLedger`.
- **History-view helpers live in the time-travel module.** `📨️dispatch` and `🚪️ingress` still call `time_travel::{member_backfill, history_store_change, history_store_stamp, history_mutation_views, MemberEditHistory, HistoryStoreStamp, time_travel_replay_turn_budget}`, now through explicit per-child imports. Their semantic owner is `command_history`; moving them is a follow-up.
- **The ladders still name the domains.** The close ladder (`CloseRung::{ToolRuns, TimeTravel}`) and the maintenance ladder (`MaintenanceUnit::{ToolRuns, TimeTravel}` with fixed ordinal codes 105/106) do. Making rungs per-extension needs a rung-registration design; follow-up.
- **(Resolved in the follow-up.)** The plugin crate no longer keeps `ui_wgpu` (section F.2).
- **The os-config test cases still live under `host/🧪️tests/`.** Moving them beside their crate (`💻️os/🎚️config/🧪️tests/`) needs a `taxonomy.json` group change; follow-up.
- **W5 is not done.** The three `include!`s in the instance parent (`../🪟️window/🫧️transient/🔁️document-replacement`, `../🎮️command/🎟️lease`, `../♻️lifecycle/🏪️stores`) remain. Converting them to `mod` turns every private method and struct in them, all currently visible to the whole instance subtree, into a field and method privacy change. With a typeck-blind oracle that is high risk for low benefit, so I deferred it.
- **Pre-existing source-text failures were left alone.** The TS readers `⏪️time-travel/♻️retirement` and `🛠️tool-machine/♻️retirement` slice module roots for functions that live in the instance turn files, and they already failed before this change (15 and 4 failures). Repointing them to `⏪️time-travel/🧵️instance/🦀️.rs` and `⏯️tool-run/🧵️instance/🦀️.rs` is now possible and would fix their reads, but they belong to their owners; follow-up.
- **Two `[DEBUG]` lines moved with the time-travel turns.** The `eprintln!("[DEBUG] …")` lines in `⏪️time-travel/🧵️instance/🦀️.rs` (lines 467 and 1055) were already in the original file. I moved them verbatim (losslessness preserved) and did not remove them.
- **Seam with slice A.** `is_time_travel_action_id` (`crate::app_contract`) and the `HISTORY_EDIT_*` ids (`semio_framework`) are imported, not moved. If slice A relocates them, the imports in `⏪️time-travel/🧵️instance/🦀️.rs` follow.
- **Concurrent edits.** Peers edit the same tree, and slice A has created `🏗️builder/🧩️app/🖐️gumball-audience/`. Every edit here was anchor-checked (`count == 1`) right before applying, and no change by anyone else was reverted.

## 7. Artifacts

- Tools (kept): `🪓️monolith-split/relo-pluginsdk-tools/orc.py` (sandbox build, oracle run, diff) and `visscan.py` (privacy scan with `--strip-vis` seeding).
- Baseline: `🗑️generated/relo-pluginsdk-baseline.tar` (pre-edit Rust/TOML/WIT sources of the plugin tree).
- Launcher logs: `🗑️generated/relo-pluginsdk-cargo-baseline-plugin.log`, `relo-pluginsdk-cargo-semio-framework-plugin-host.log`, `relo-pluginsdk-cargo-semio-framework-os-run.log`.

## Follow-up: combined oracle and ui_wgpu removal

The coordinator asked for this follow-up after slice A finished. Foreground only, no sub-agents, no modifying git commands.

### F.1 Combined pristine baseline and differential

Building the baseline:
- Slice A's archive `🗑️generated/relo-manifest-baseline.tar.gz` contains 16 plugin-crate files. I compared each one with my pre-edit snapshot (`🗑️generated/relo-pluginsdk-baseline.tar`, taken at 14:50, before either slice edited the plugin tree).
- **Files both slices touched** (`🧵️instance/🦀️.rs`, `📨️dispatch`, `⏪️time-travel/🦀️.rs`, `⚛️reactor/🔌️app`, `🎮️command`, `📜️declaration/🧩️app`, `📜️declaration/🪟️surface`, `🪟️window-kits`, `🧪️tests/🔬️plugin-runtime-plugin-builder-contract`): A's copies already contain my edits. For example, A's builder-contract copy has my 17:00 `🧪️TestClipboardReservedJob` marker and the new route assertion. So my snapshot predates both slices, and I took it.
- **Files only A touched** (`🏗️builder/🧩️app`, `👷️job/🏛️reserved`, `🧪️tests/🧪️folder-reload-route`, `🧪️tests/🧪️history-edit-acceptance`, `🧪️tests/🔬️app-app-builder`, `🧪️tests/🧪️time-travel`): A's snapshot is byte-identical to mine.
- A's copy of `⏪️time-travel/🧵️instance/🦀️.rs` is my moved file, so it does not exist in the pristine state.
- The combined baseline is therefore exactly my snapshot, and the baseline JSON results (`base-*.json`) are reused as they are.
- The current tree also carries peer edits made since 14:50 (`⚛️reactor/🔄️turn`, `⚛️reactor/🦀️.rs`, `🏗️builder/🦀️.rs`, and the new `⚛️reactor/🧪️tests/📦️package-assets` and `🖥️host/🎁️package-assets`). Their keys are attributed below.

Run: `orc.py build` on the live tree, then `orc.py run` for every configuration, then `orc.py diff [--non-extern]`.

| Crate / config | All errors, baseline / current | Non-extern, baseline / current | New non-extern keys | Gone non-extern keys |
|---|---|---|---|---|
| plugin native | 9243 / 9273 | 1461 / 1461 | 0 | 0 |
| plugin `--test` | 20830 / 20875 | 2772 / 2773 | 3 (A) | 2 |
| plugin wasm32-wasip2 | 9534 / 9564 | 1538 / 1538 | 0 | 0 |
| plugin wasm32-wasip2 `--test` | 21115 / 21160 | 2848 / 2849 | 3 (A) | 2 |
| plugin wasm32-unknown-unknown | 9257 / 9287 | 1461 / 1461 | 0 | 0 |
| plugin all features | 10091 / 10121 | 1507 / 1506 | 0 | 1 |
| plugin all features `--test` | 20853 / 20898 | 2772 / 2773 | 3 (A) | 2 |
| plugin-host native | 1335 / 1338 | 83 / 83 | 0 | 0 |
| plugin-host `--test` | 2797 / 2807 | 98 / 98 | 0 | 0 |

Every key is attributed:
- **New, 3 × `cannot find attribute serde in this scope`** in the `--test` configurations, all in slice A's new file `🏗️builder/🧩️app/🖐️gumball-audience/🧪️tests/🦀️.rs`. `serde` is the helper attribute of a derive from the unresolved external crate `serde`. It counts as non-extern only because the diagnostic has no error code, which my filter keys on. It is the unresolved-external class, not a regression.
- **Gone, `cannot find type WindowMeasure in the crate root`** (`🧪️tests/🧰️ui-action-collection`, `crate::WindowMeasure`). The crate root now exports `WindowMeasure` explicitly, where it used to come through the external glob that the oracle neutralises.
- **All-error deltas** are unresolved external-crate keys only. Per-file attribution:
  - Mine: the `ui_wgpu` → `semio_framework_ui_layout`/`semio_framework_ui_declaration`/`semio_framework_ui_scene` renames (46 `ui_wgpu` keys gone; 45 ui-layout, 5 ui-declaration and 1 ui-scene new); the moved turn modules; `🧩️extensions`.
  - Slice A: `semio_framework_tool_run` imports in `⚛️reactor/🔌️app`, `🧵️instance/🦀️.rs` and `🪟️window-kits`; `semio_framework` paths in `🏗️builder/🧩️app` (+3), `🖐️gumball-audience` (+1), `👷️job/🏛️reserved` (+1) and `📜️declaration/🪟️surface` (+1), with −1 / −2 / −1 in `🧷️command-routes` / `🪟️window-kits` / `⚛️reactor/🔌️app`.
  - Host: my `semio_framework_os_kernel` → `semio_framework_os_vcs` rename, the removed `semio_framework_os_config` re-exports, the ui crates in the app-router test; and the peer's new `🎁️package-assets` module (`semio_framework`, `semio_framework_hash`, `serde_json`).
- No new E0412, E0425, E0422, E0423, E0432, E0603, E0616 or E0624 against an internal path, so no visibility fix was needed.

### F.2 `ui_wgpu` removed from the plugin crate

How I found the plugin-side users:
- I built the set of names exported by `ui_wgpu::wgpu` with feature `wgpu` and without `wgpu-engine` (569 names; script output in the scratchpad).
- I searched the 4,735 repo files that name `semio_framework_plugin` for `semio_framework_plugin::X` paths and braced `use semio_framework_plugin::{…}` lists that hit those names. That gives 88 names.
- The two glob importers (`use semio_framework_plugin::*;` in `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d` and `🛍️products/📓️print/🧪️tests/🧬️chart-mutations`) use none of them.
- There are no Cargo renames of the crate and no `semio_framework_plugin::component::…` paths. The macros use only `$crate::component`, which is the plugin's own module.
- Inside the crate, I scanned every module that reaches the root through `use super::*` or `use crate::*` for names from the 569-set that are neither imported nor exported. Only false positives came up (string literals, locals, already-qualified paths). The one real root dependency, `crate::WindowMeasure`, is covered, and the oracle confirms it (F.1).

The crate-root `pub use ui_wgpu::wgpu::*;` is replaced by explicit lists from the owners (`plugin/🦀️.rs`):
- `semio_framework_ui_layout::layout::{…}`: 38 window-layout, engagement, measure, context-menu, `ActionDescriptor`, `UiMenuRef` and panel-tab names;
- `semio_framework_ui_layout::utilities::UtilityCategory`;
- `semio_framework_ui_declaration::ui::{…}`: `SurfaceKind`, `TableCell`, `BlockPaletteEntry`, `UiTreeActionPlacement`, `UiTreeItemAction`, `UI_INSPECTOR_MIXED_PLACEHOLDER`, the `ui_inspector_*` helpers, `text_identifier_occurrences_json`, `world3d_camera_json`, `world3d_chunking_json`, `world3d_fit_json`;
- `semio_framework_ui_scene::{…}`: 23 scene records and helpers.

Three groups needed no new line:
- The world3d scene family and `scene_lane_hash` were already re-exported explicitly through `world3d_host`.
- `IconName` stays reachable through the existing `pub use semio_framework::*;`.
- `decode_surface_doc` was only `ui_wgpu`'s rename of `semio_framework_ui_scene::decode`. Rather than re-export it under that name, I re-pointed its only callers, the 7 pdf unit tests `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/{♿️ua,⚕️h,📐️e,🖨️x,🗄️a,🧱️base,🧾️vt}/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs`, to `semio_framework_ui_scene::decode(props)`. The pdf crate already dev-depends on `semio-framework-ui-scene`.

The remaining in-crate users now point at the owners directly:
- `🏗️builder/🧩️app/🦀️.rs` (re-read first; slice A had edited it): 9 imports and 6 panel-tab paths now use `semio_framework_ui_layout::layout::…`, and two doc lines now name the owner;
- `🪟️window/🦀️.rs`: `WindowOptions` comes from ui-layout. The surface-kind bridge `contract_surface_kind`/`legacy_surface_kind` now converts `semio_framework_ui_declaration::ui::SurfaceKind` to and from the contract's `SurfaceKind`. That is a declaration-to-contract bridge and was never renderer-specific;
- `🪟️window-kits/🌍️world3d/🦀️.rs`: `world3d_camera_json` comes from ui-declaration; `ActionDescriptor`, `MeasureSelectItem` and `WindowMeasure` come from ui-layout;
- tests `🧪️tests/🔬️plugin-runtime-plugin-builder-contract`, `🧪️tests/🔬️app-app-builder` and `🧪️tests/🧪️time-travel` now use ui-layout.

`plugin/📦️packages/🦀️rust/Cargo.toml`:
- removed `ui_wgpu` (`semio-framework-ui` with feature `wgpu`); the crate has no dev-dependency on it;
- added `semio-framework-ui-declaration`. It is role framework, renderer-independent and wasm-safe; its dependencies are ui-layout, the icon contract, value, ui-contract, ui-scene, ui-locale and serde.

The only `ui_wgpu` text left in the plugin crate is in doc comments, plus a Rust fixture string inside the TS test `🧪️tests/🔬️tool-job-coverage/🟦️.ts` that models renderer code. That string is not a dependency. Nothing renderer-specific remains, so `ui_wgpu` is not a dependency of either the plugin crate or the plugin host.

### F.3 Lock file

`bun repo cargo --store=shared --configuration=prototype -- update --workspace --offline` exited 0 ("Locking 0 packages"). The lock already recorded the new edges:
- `semio-framework-plugin` → ui-declaration and ui-layout, with no `semio-framework-ui`;
- `semio-framework-plugin-host` → `semio-framework-os-vcs`, ui-declaration and ui-layout;
- `semio-framework-os-run` → `semio-framework-os-config`.

The refresh itself only dropped two stale `semio-framework-value` lines from the tiled-map packages, matching slice B's manifests. Log: `🗑️generated/relo-pluginsdk-cargo-lock-update.log`.

### F.4 Static checks (follow-up files)

- **rustfmt:** `rustfmt +nightly-2026-07-20 --edition 2021 --emit stdout` exits 0 on `plugin/🦀️.rs`, the builder, window and world3d kit files, the `app-app-builder` and `time-travel` tests, and the 7 pdf tests. `🔬️plugin-runtime-plugin-builder-contract` exits 1 with `left behind trailing whitespace` at line 4149; that is a formatting failure, not a parse failure, and the baseline file fails identically.
- **Regions:** marker counts are unchanged in every file touched in this round.
- **New paths:** none.
- **Direction:** the plugin `Cargo.toml` has no upward `path =` dependency.
- **Privacy scan:** `visscan.py` gives 0 findings, and the seeded run (`--strip-vis tool_runs,freeze_tool_machines`) gives 3 findings.
- **TS:** no TS source-text reader reads the changed `ui_wgpu` text. The only TS reader of the plugin `Cargo.toml`, `🧬️schema/🧩️composition/📥️admission/🧪️tests/🟦️.ts`, passes 4 / 0. No TS file changed in this round.

### F.5 Sanctioned launcher (follow-up)

1. `… -- check -p semio-framework-plugin` and `-p semio-s-artifact-stdio-pdf` (the crate that owns the 7 re-pointed pdf tests): once the cache-maintenance failure had cleared, both first stopped at `cannot update the lock file … because --locked was passed`. Logs: `🗑️generated/relo-pluginsdk-cargo-followup-*.log`.
2. After the lock refresh (F.3), `-p semio-framework-plugin` reached the gate: `Rust crate Source refused before compiler: semio-framework-os-kernel (Rust crate Source boundary exceeded before compiler byte allocation)`. Log: `🗑️generated/relo-pluginsdk-cargo-followup2-plugin.log`.
3. The plugin host and the pdf crate depend on the same os-kernel closure. I did not retry them after the refresh.

Nothing was compiled around the gate, so typeck is still unproven for both rounds.

### F.6 Residual risks (follow-up)

- **Smaller public surface.** The crate root used to re-export all 569 non-engine `ui_wgpu::wgpu` names through the glob. It now re-exports only the 88 that repo code uses. Nothing in the repo uses names such as `UiNode`, `Theme`, `DrawList`, the `kernel_3d_scene` math, or the prepared and raster types through the plugin crate. A future or out-of-repo plugin must import them from the owning crate. This is deliberate: there is no compatibility glob.
- **One SurfaceKind name, two types.** `SurfaceKind` at the crate root is the declaration type; `semio_framework_ui_contract::SurfaceKind` is a different type. Modules that glob both the root and the contract keep the ambiguity status they had before, because the root binding refers to the same item as before.
- **Typeck is unproven.** As in section 6. Run `check -p semio-framework-plugin` (native, `--tests`, wasm32-wasip2), `-p semio-framework-plugin-host` (native, `--tests`) and `-p semio-s-artifact-stdio-pdf --tests` once the os-kernel closure passes the gate.
