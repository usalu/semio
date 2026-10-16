# 🧱️ Result PLUGIN-WP6: the `VcsArtifactApp` god object cut into component structs, tier 1 of the instance family (S0 done, tier-1 methods converted, staged not flipped)

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool. Plan: `PLAN-PLUGIN-CRATES.md` (contract 12.5, decision D1, Appendices G to K), on top of `RESULT-PLUGIN-WP0.md`, `RESULT-PLUGIN-WP3.md`, `RESULT-PLUGIN-WP4.md` (there are no WP1, WP2 or WP5 result files in the folder). Tools: `pwp6-tools/` (this folder). Logs and measurements: `🗑️generated/pwp6-*`. `PLUG` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`.

## 0. Verdict

**Step S0 is DONE** (`S0 DONE` for WP7): the 21-file tangle (`🧵️instance/**`, the three ladders, the two feature `🧵️instance` files, the three `include!`d files, `🎮️command/📥️ingress`, `🪆️child/👁️read/🧪️test-view`) is cut into one staging file per (source file, target crate) pair, 67 staging files, mounted from the instance parent; the old tree stayed one compilation unit and the patched differential found no new name-resolution key (section 4).

**Step 2, the decomposition, is done for tier 1 and staged**, not flipped (no compiler, no flipped lower crates, same reason as WP3 and WP4):
* 15 component structs are defined (`📥️staging/🧱️components/🦀️.rs` of the five tier-1 crates and the ladder crate), each with the fields it owns and a `new` constructor; with the three existing field types (`ToolRunLedger`, `TimeTravelLedger`, `ToolMachineRuntime`) that are the 18 field groups of 7.3; `VcsArtifactApp` is now the composer: `app`, `mounted_policy`, the 15 components and the three ledgers (227 fields before, 20 now).
* **272 of the 678 methods** moved into `impl Component` blocks of the five tier-1 crates (stores 11 methods + constructor/roster/ingress/interaction components, history, children, jobs, docs; counts in 3.2), with the lower components as explicit parameters (0 parameters: 177 methods, 1: 71, 2: 24). No inherent `impl VcsArtifactApp` is left in a tier-1 crate (orphan scan, section 4). 44 public methods (41 `pub` plus the three `PluginApp` twins) keep their composer-level one-line delegations in `📥️staging/🧬️composer-api`.
* The remaining 406 methods (use-case and composer) stay `impl VcsArtifactApp` blocks in the staging directories of the six tier-2 crates and the composer; they use the component paths (`self.stores.store`) and call the converted methods as `self.stores.m(&mut self.constructor, ..)`. They are the inputs of WP7 (section 7).

**Nothing was compiled.** The launcher fails before cargo (`Live Original resource control issuance required`, `🗑️generated/pwp6-cargo-1.log`); borrow checking and type checking are not evidenced. Everything below is name-resolution, textual and structural evidence.

## 1. Deviations from the plan, each with its evidence

| id | plan said | I did | why |
|---|---|---|---|
| DEV-1 | component rank order of `plan_tools.py` (`hostevent`, `txn` last) | `core, constructor, stores, roster, ingress, interaction, history, hostevent, txn, children, pending, tools, media, ingest, …` | the plan's order put `HostEvents`/`Transactions` (crate `instance-history`) above `tools` (crate `instance-jobs`): a method homed there would call upward across the crate DAG (`record_settled_typed_operation_command` → `record_command`) |
| DEV-2 | field reads decide the home (`self.f`) | the regexes also accept line-broken chains (`self\n.field`) and the path values `Self::name` (no parenthesis) | the plan's graph missed 10 methods (`self\n.store_replacement_jobs`) and `.map(Self::lifecycle_retained)`; found by the first differential and by a flag of the rewriter |
| DEV-3 | a method touching four components is a use-case method | the same threshold on the **transitive** need (own component plus callees' homes, `core` excluded) is at most three | caps the parameter lists at two lower components; 12 methods more became use-case methods |
| DEV-4 | `ToolMachineRuntime`, `TimeTravelLedger`, `ToolRunLedger` are components with homed methods in `instance-jobs` and the turns | their methods are use-case methods (`typed-ops`, `time-travel-turns`, `tool-run-turns`) | an inherent impl of a type defined in `tool-run` / `time-travel` cannot live in a crate above them, and those crates are below the components (orphan rule and DAG); the turns take bundles in WP7 |
| DEV-5 | the turn crates "take their whole source file" | only the methods homed in the ledgers go there; history/children/stores helpers of the two files (`member_edit_histories`, `supersede_row_label`, …) stay in the tier-1 crates | generic code (`dispatch`, the history view) calls them; the whole-file rule produced 21 upward calls from tier 1 |
| DEV-6 | methods constructing `Self` and the constructors are free methods of `instance-history` | `new`, `with_registry`, `with_registry_on_bus`, `Drop::drop` and the `PluginApp` methods are composer methods (`instance-app`); free (no-self) functions go to the lowest crate that calls them | the constructor builds `Self { .. }` (now component by component) |
| DEV-7 | the 49 public methods are composer methods | the 41 `pub` methods and the three `PluginApp` twins stay homed in the components and get one-line composer delegations | internal callers use the components directly; external callers (284 files) keep their calls |
| DEV-8 | `🎮️command/📥️ingress` is one module of the root | it moves whole into `instance-stores` (`📥️staging/📮️action-ingress`), the hub-6 `#[path]` literal follows | its `OriginalParsedCommand*` items are used by `commands` (downward) and its module path `crate::action_ingress` must not change |
| DEV-9 | `S0` mounts the old parents | the three `include!`d files became `mod`s of the instance parent, their items widened exactly as far as other staging modules use them (definition scan) | an `include!` cannot cross a crate; plan 7.9 asks for it |

## 2. Census (gate counter, `pwp6-tools/census6.ts`, virtualised mounts; `🗑️generated/pwp6-census-after.log`)

Files / bytes / active lines of the lib closure on native (wasip2 and unknown-unknown differ by at most 7 lines; the test closure by at most 64 lines). Limits 28 / 230,000 / 3,500 (gate 32 / 262,144 / 4,000).

| crate | lib native | test native |
|---|---|---|
| `instance-stores` | 14 / 105,087 / 1,291 | 14 / 105,087 / 1,325 |
| `instance-history` | 5 / 30,425 / 460 | 5 / 30,425 / 460 |
| `instance-children` | 11 / 82,224 / 1,099 | 11 / 82,224 / 1,116 |
| `instance-jobs` | 14 / 134,639 / 1,695 | 14 / 134,639 / 1,720 |
| `instance-docs` | 14 / 162,733 / 1,808 | 14 / 162,733 / 1,811 |
| `instance-commands` | 6 / 206,277 / 2,425 | 6 / 206,277 / 2,489 |
| `instance-typed-ops` | 4 / 144,299 / 1,833 | 4 / 144,299 / 1,852 |
| `instance-render` | 1 / 47,231 / 603 | 1 / 47,231 / 603 |
| `instance-ladders` | 9 / 138,286 / 1,935 | 9 / 138,286 / 1,935 |
| `time-travel-turns` | 1 / 97,145 / 1,241 | 1 / 97,145 / 1,244 |
| `tool-run-turns` | 1 / 75,473 / 1,071 | 1 / 75,473 / 1,071 |
| `instance-app` | 7 / 218,013 / 2,935 | 7 / 218,013 / 2,972 |

Every row is inside the target. The tight ones: `instance-app` (218 KB of 230 KB, 95 percent) and `instance-commands` (206 KB, 90 percent). The tier-1 crates are small (30 to 163 KB), the plan estimated 100 to 190 KB: the use-case methods that do not fit a component stayed up. The test closure of the family is the 12 companion bins of WP7.

## 3. What changed

### 3.1 S0 (staging, 67 files)

Tool chain: `plan_tools.py home` (Appendix G) → `an6.py`/`home2.py`/`assign6.py` (homing with DEV-1 to DEV-6) → `s0plan.py`/`s0apply.py`/`s0final.py`/`s0write.py` (free items by users, widening, imports, mounts, transactional write with peer-edit check). Per (source file, crate) pair one staging file `<crate root>/📥️staging/<slug>/🦀️.rs` with a unique-emoji docstring, `use super::*;`, the source file's own extra imports and the assigned items and members inside their original `impl VcsArtifactApp` wrapper; the class-B root files (`command-routes`, `typed-operations`, `render`, `maintenance-ladder`, the two feature turn files, the parent, `plugin-app-impl` of the composer) keep their own part in place. 13 emptied files are deleted: the ten band and ladder files, `document-replacement`, `lease`, `lifecycle/stores`, `action-ingress` (moved), `test-view` (moved). The parent mounts every staging module (`//#region 📥️StagedInstanceMounts`) and re-exports the names that crossed the old module paths (`artifact_handle_of`, `genesis_member_schema`, `history_changed_event`, `MAINTENANCE_*`, `retire_document_window_registry_*`, `typed_operation_document_is_fresh`) with their old visibility. 102 visibilities were widened to `pub(crate)` exactly where another module uses the item (private struct fields of moved structs likewise); the two feature turn files' staging copies get the imports the old `use super::*` of `time_travel`/`tool_run` gave them.

### 3.2 Components and conversion

Components (crate: struct, fields): `instance-stores`: `ConstructorState` 9, `InstanceStores` 17, `PeerRoster` 14, `ActionIngress` 5, `LocalInteraction` 14; `instance-history`: `CommandHistory` 14, `HostEvents` 5, `Transactions` 2; `instance-children`: `ChildComposition<M>` 12; `instance-jobs`: `PendingReserved` 22, `ToolOperations` 29; `instance-docs`: `MediaTransfers` 12, `EnvelopeJobs<A, M>` 10; `instance-ladders` (WP7's crate, staged here because the fields are the composer's): `MaintenanceCursors` 17, `CloseCursors<A, M>` 40. Composer field names: `constructor, stores, roster, ingress, interaction, history, host_events, txn, child_composition, pending, tools, media, ingest, maintenance, close`. All component fields are `pub`; each component has `pub fn new(..)` taking the parts the composer builds first (the constructor `with_registry_on_bus` now builds the 15 components and hoists the one `.await` initialiser, `composition`).

Converted methods per component (members of the `impl Component` blocks, constructors excluded): stores 42, child_composition 52, tools 54, ingest 39, media 30, pending 25, history 15, ingress 10, constructor 6, interaction 5, roster 5, host_events 1, txn 1 (the figure of 272 is the sum without the 15 constructors and the two cursor `new`s). Rewriting rules (engine `rw6.py`, all edits mechanical, inverse-auditable): `self.f` → `self.<component>.f` in composer code and `<param>.f` in component code; `self.m(a)` of a converted `m` → `self.<c>.m(&mut self.<lower>.., a)` (composer) or `<c>.m(<lower>.., a)` / `self.m(<lower>.., a)` (component); `Self::assoc(..)` → `Component::<G>::assoc(..)` including path values; UFCS `VcsArtifactApp::m(self, ..)`; arguments of a call with lower-component references that name `self` or a parameter are hoisted into a `({ let __a0 = ..; call })` block (8 composer sites plus component sites) so the explicit `&mut` borrows never overlap an argument; core fields become `app: &mut A` / `mounted_policy: &MountedOwnerPolicyV1` parameters; method-level `A`/`M` generics are added where the component lacks them. External consumers rewritten with the same engine (receiver `app` and the named receivers of the time-travel, tool-run, acceptance and folder-reload tests): 25 files (`🧵️retained-command/🔄️full-operation` and 24 test files; list in `🗑️generated/pwp6-t1-touched.json`).

### 3.3 Manifests

`instance-docs` gained the two certain edges `instance-stores` and `instance-children` (its methods take `InstanceStores` and `ChildComposition` parameters; a transitive dependency cannot name the type). Name-evidence edges were computed (`pwp6-tools/edges6.py`) but not added: the evidence is polluted by member directories of other crates (`decl-views` members read as `decl-registry`, `window-kits-rich` members as `window-kits`), so the compiler must ask for them. No crate, registry row, launch route or workspace member was added (12.5.3 rule 4).

### 3.4 Source-text readers

`test-support` got a TypeScript helper `🧷️companions/🧯️test-support/🧵️instance-family/🟦️.ts` (`instanceText`, `ladderText`, `turnText`, `familyText`: the run-time source of the cut family). The Rust law `instance_source()` of `🧪️tests/🔬️plugin-runtime-plugin-builder-contract` reads the instance module tree through `read_module_tree_source` instead of six `include_str!`. The TS readers are in section 5.

## 4. Verification (commands run, real results)

@@VERIFY@@

## 5. TS source-text readers

@@TS@@

## 6. Files

Created: 6 component files (`<crate root>/📥️staging/🧱️components/🦀️.rs`), 1 composer API file (`🧵️instance/📥️staging/🧬️composer-api`), 67 S0 staging files, 1 TS helper; modified: the parent `🧵️instance/🦀️.rs`, the in-place roots (`🧷️command-routes`, `🧾️typed-operations`, `🖼️render`, `🔌️plugin-app-impl`, `🪜️maintenance-ladder`, `⏪️time-travel/🧵️instance`, `⏯️tool-run/🧵️instance`), hub `🪝️mounts/🧱️wp6-instance-tier1`, `instance-docs/Cargo.toml`, 25 consumer files, the builder-contract test, TS readers; deleted: the 13 files of section 3.1. Exact lists: `🗑️generated/pwp6-touched.json` (union), `pwp6-s0-touched.json`, `pwp6-t1-touched.json`. Not touched: `Cargo.lock`, registries and launch files, the facade manifest, any peer-owned crate.

## 7. Inputs for WP7

@@WP7@@

## 8. Residual risks

* **No compiler.** Borrow checking of the new call shapes (explicit `&mut self.lower` next to arguments; hoisting is conservative but not proven), type inference of method-level `A`/`M` generics, privacy of the moved fields (E0616) and trait scoping are unproven; the differential reports resolution only.
* **Drop order.** Fields of the composer are dropped component by component; the owners assert emptiness on drop and are `ManuallyDrop` where order mattered, but the relative order of plain fields changed.
* **Dense text laws.** The source-text laws of TS and Rust assert on exact code fragments; where fragments contain the converted paths they were rewritten by the engine (section 5); others still fail and are listed.
* **Widened visibilities** are `pub(crate)`/`pub`: WP7 and the facade phase decide `#[doc(hidden)]`.
* **The tier-2 crates are mutually recursive** (section 7): the plan's file-aligned partition cannot be acyclic.
* **Concurrency.** Peers edited the plugin tree while I worked (WP1 flips, WP4 pieces); every write checked the pre-state against the snapshot and aborted on a difference; none occurred.
