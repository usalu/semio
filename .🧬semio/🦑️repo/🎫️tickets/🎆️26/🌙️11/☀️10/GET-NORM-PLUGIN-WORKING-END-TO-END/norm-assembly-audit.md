# Norm Assembly Audit

Static audit of every check that runs while a norm plugin assembles (`describe()` then `plugin()`), covering the five plugin builders and the editor and viewer `AppDefinition` of all 15 families. No cargo or wasm build was run. One violation was found (B1) and fixed by renaming the artifact identity owner segment (section 3).

## 1. Remaining violation (will fail describe of four of the five plugins)

### B1. Artifact kind ownership does not match the extension plugin ids (fixed, option 1, see section 3)

| | |
|---|---|
| Checks | `preflight_artifact_identity` (`🔌️plugin/🪪️identity/🦀️.rs:14`) called from `ArtifactDeclaration::preflight` (`🔌️plugin/📜️declaration/🧮️codec-spec-and-capability-sources/🦀️.rs:274`) in `PluginBuilder::try_build`, and `surface_dependency_breaches` plus `host_foreign_surface_kinds` (`🔌️plugin/📜️declaration/📇️registration/🦀️.rs`, near line 575) |
| Rule | `ArtifactKindId::plugin()` (second segment of `s.<plugin>.<artifact>`) must equal the declaring `plugin_id`, otherwise `plugin-assembly.artifact-owner`. A surface of a kind owned by another plugin needs that plugin in `depends_on`. |
| Norm | Every identity, dialect and schema is `s.norm.<family>` (`definition()`, `*_DIALECT`, 15 of 15 consistent). Plugin ids are `norm`, `norm-energy`, `norm-eurocode`, `norm-eurocode-structural`, `norm-eurocode-materials`. |
| Verdict | `norm` (din4108, din16798, iso16757, vdi3805) passes. The four extensions fail: `preflight_artifact_identity("norm-eurocode", "s.norm.en1990")` returns `plugin-assembly.artifact-owner`. Even past that, `surface_dependency_breaches` reports every app (owner `norm`, no dependency). `norm-plugin-split.md` states "no depends_on is needed", which is only true for the root plugin. |
| Not fixed because | The rule is deliberate (`🧪️tests/🪪️artifact-admission` asserts it) and stdio avoids it with `host_artifact` plus a real owner plugin that declares the kinds. Relaxing it globally (for example "plugin id starts with owner-") would change stdio-bim and the host router (`app-router-and-opening-resolver:75`) and would make `host_foreign_surface_kinds` re-home kinds. A rename touches 563 files (`s.norm.<en*|din18599>` appears 1116 times, plus the `@1/*#editor|viewer` controller ids, schema ids and mutation ids). |

Options, in order of cost:

1. Rename the identity owner segment per plugin (`s.norm-eurocode.en1990`, `s.norm-energy.din18599`, ...). Mechanical, no framework change, but it touches schema ids, controller ids, TS twins, fixtures and the catalog. It is the only option that keeps each plugin standalone as designed.
2. Add an explicit, opt-in owner namespace to `PluginBuilder` (for example `.artifact_namespace("norm")`) honoured by `preflight_artifact_identity`, `surface_dependency_breaches` and `host_foreign_surface_kinds`. Small framework change, but the hub router, io-router and describe composer filtering also compare owner to `plugin_id` and would have to follow. Five plugins sharing one owner namespace also needs a hub decision.
3. Host model like stdio: the root `norm` plugin owns all 15 kinds and the extensions use `host_artifact` plus `depends_on("norm")`. Not viable, because owning requires the codec and editor code, which is the bloat the split removed.

Recommendation: option 1, scripted. Decision needed before another rebuild.

## 2. Checks verified clean (all 15 editors and 15 viewers, all five plugins)

Method: read each check's source, then compared the 15 builders mechanically. All 15 viewer builders are byte-identical after family substitution; the 15 editor builders differ only in the interaction label (and cosmetic whitespace and description wording).

### `try_build_definition` (`🏗️try-build-definition/🦀️.rs`)

| Check | Verdict |
|---|---|
| Document breadcrumb non-empty segments (`.document(["semio","norm",<fam>])`) | OK |
| Terminology breadcrumbs | N/A, none declared |
| At least one window kind and one mode | OK (editor 5, viewer 3, one mode each) |
| Window id and `body_key` non-empty, unique | OK (`norm-<fam>-<role>`, constants checked for all 30 apps) |
| Panel tab ids unique, exactly one of body or children | OK (editor panels via `panel_definition`, leaf with body key; viewer none) |
| Layout windows reference declared window kinds (default and the 3 named layouts) | OK. Viewer roles `viewport`, `checks`, `diagrams` are subsets of the viewer windows. `visual_layout_roles` drops unavailable roles on desktop, mobile and tablet. Editor layouts use all five declared roles. |
| `default_mode_id` references a declared mode | OK (`edit` / `view`) |
| Action ids unique | OK (editor 16 declared in `app_commands!`, viewer 1) |
| `validate_arg_defs` (duplicate arg id, empty `Select`) | OK (no `Select`, unique arg ids, also for the viewport and diagram window actions) |
| Reserved verb audience (`gumball` only fixes `setTransformGumballFlag`; time-travel and tool-run fix their own ids) | OK, no norm id collides |
| Utility id non-empty and unique | OK (`transform` once; viewer fixed by the earlier change, editor already had it) |
| Interaction non-empty granularities, selection modes, methods and merges; transitive needs non-Flat hierarchy | OK (`HierarchyProvider::Topology`, transitive false, `check` and `loadCase` granularities always present) |
| Action arg entity kinds answered by an interaction | OK (norm args are text, number, any and index only) |
| Tool and command ids | N/A, none declared |
| Window `action_refs` resolve | N/A, none used |
| Window action ids non-empty and unique per window | OK (shortcut actions are de-duplicated with a `seen` set and use `norm.shortcut.*` and `norm.handle.*`; none equals the explicit `viewport*` ids) |
| Keybindings reference declared actions (only enforced when the app declares actions, which both roles do) | OK. Editor `mod+z` to `undo`, `mod+shift+z` to `redo` (history actions) and `escape` to `clearPreview` (declared). Viewer declares none, history keys are filtered by `viewer_rejects_action`. Utility key of `transform` maps to the declared `setActiveUtility`. iso16757 has the same three bindings (indentation only). |
| Window `utilities` declared | OK (`transform`, declared in all 30 apps after the earlier fix) |
| `initial_utility_id` inside the window's utility roster | OK (`None`) |
| Window `interactions` declared | OK (`norm.<variant>` equals `interaction_domain_id`; editor via `viewport_interaction_definition`, viewer via `viewer_interaction_definition`) |
| Mode commands and tools, tool referenced by a mode | N/A |
| Introduction, tutorials, element id grammar | N/A, none declared |
| Dialogs (`submit_action`, `cancel_action`, choices, args) | OK. Only the framework-reserved editor dialogs are added, whose actions come from `reserved_app_actions`. |
| Media ports direction and `MediaForm::Any` on outputs | OK (`media_inputs` and `media_outputs` empty; `norm_io` ports are in `io`, not checked here) |
| `validate_interactive_job_classification` | OK. The earlier `view_action` fix covers all window actions. Every app-level action is classified: scripted across the 30 builders, declared ids equal `action_interactive_job` ids in every editor and viewer. The framework appended actions run after this check and are not validated by it. |
| Canonical surface id (`parse_surface_app_id`) | OK (`Editor::builder` and `Viewer::builder` derive it from the dialect) |

### `AppBuilder` setters (`🎬️actions-commands-and-tools`, `🧱️builder-identity-and-modes`)

`action_describe`, `action_interactive_job` and friends silently ignore unknown ids, so none can panic. `build_definition` only unwraps `try_build_definition`. OK.

### `stamp_fault_notices`, `stamp_app_contracts`, `project_tool_cancellation` (`📜️declaration/🌳️tree`)

| Check | Verdict |
|---|---|
| Fault notice code grammar (3 or more kebab segments) | OK, 20 codes, all `norm.editor.*` |
| Duplicate codes | OK, none |
| Non-empty text in every terminology and locale cell | OK (`LocalizedLabel::native` fills every cell, en and de are non-empty) |
| Placeholders identical across cells and well formed | OK, no braces in any text |
| Table declared by every editor and viewer (`fault_notices()` returns `norm_fault_notices()`) | OK, scripted for 30 apps |
| `project_tool_cancellation`: controller id equals the definition id | OK. Proof controllers are `s.norm.<fam>@1/*#editor` and `...#viewer` for all 30. |
| Proof `artifact_schema` equals `DOCUMENT_SCHEMA` | OK for 30 (`semio.norm.<fam>/v1`) |
| Proof tools exactly equal the migrated `TOOL_JOB_IDS` of the command enum, no duplicates, and each is a declared action | OK, scripted: editor 16 rows equal 16 proofs, viewer 1 row (`viewportAct`) equals 1 proof, in all 15 families |
| Non-empty `owner_file` | OK |

### `PluginBuilder` registration and `try_build` (`🏗️builder/🦀️.rs`)

| Check | Verdict |
|---|---|
| `package_id` equals `semio:<plugin-id>`, lowercase kebab | OK for `norm`, `norm-energy`, `norm-eurocode`, `norm-eurocode-structural`, `norm-eurocode-materials` |
| `label` and `version` present | OK |
| `.editor` and `.viewer` stamp `io.artifact_schema`, then fault notices and contracts | OK. Editor `io` is `norm_io(VARIANT, DOCUMENT_SCHEMA)`. Viewer leaves `io` default and gets the schema stamped. |
| Natural file codec mounting | N/A, no norm editor declares one |
| Artifact kind specs non-empty and consistent | OK (`artifact_kind_spec` id `computation.norm.<fam>`, schema `semio.norm.<fam>/v1`) |
| `preflight_channels` (dialect artifact kind equals declaration kind, exact standard and subset, composer, subset validator, schema document scope) | OK as far as the shared declaration goes: `*_DIALECT.artifact_kind` equals `s.norm.<fam>` for 15 of 15 |
| Surface dependency gate and hosted kind rehoming | Root OK, extensions fail, see B1 |
| Artifact ownership | Root OK, extensions fail, see B1 |

### Describe-time validation (`🖨️describe/🛂️descriptor-emission`)

| Check | Verdict |
|---|---|
| Plugin id `assembly-failed` placeholder refused | Consequence of any failure above |
| Kind identity law: an app whose `io.artifact.id` is non-empty needs an `ArtifactKindSpec` with that id and the `io.artifact_schema` | OK for the 15 editors (`computation.norm.<fam>` declared via `.artifact_kind(model::artifact_kind())`, same schema). The 15 viewers present no artifact (empty `io.artifact.id`), so they are not judged. |
| `first_owned_codec`: at least one declared `(kind, schema)` has a codec with non-zero pack schema hash | Statically consistent: every editor and viewer `DOCUMENT_SCHEMA` equals its subset codec schema. The hash itself is runtime-only. |
| Descriptor carries no `hashes` | N/A |

### Editor versus viewer differences (looked for, none harmful)

- Viewers declare no `.artifact_kind`, no `.io`, no panels, no keybindings, and one action (`viewportAct`). All of these are legal, only the editor presents an artifact.
- The viewer interaction has all nine entity kinds as granularities while the editor lists only the scene kinds. Both satisfy the interaction checks.
- Editor-only windows (`outliner`, `properties`) are absent from viewer layouts, which are built from the three viewer roles, so no layout references a missing window.

## 3. Changes made: owner rename (option 1)

Script: `norm-owner-rename.py` (this folder; `--apply` writes, otherwise it lists). File list: `🗑️generated/norm-owner-rename-files.txt`.

Hyphen legality verified first: `is_canonical_artifact_kind` and the artifact kind JSON schema accept kebab segments; `ArtifactIdentity::parse` allows `-`; `parse_coordinate` and the artifact reference URI only split on `@`, `/`, `!`, `#`; the schema descriptor registry has no id grammar; the router, `surface_dependency_breaches` and `host_foreign_surface_kinds` read the owner through `ArtifactKindId::plugin()`. The one place hyphens are illegal is protobuf: `protoPackage` in the library twin projection already maps every non `[A-Za-z0-9_]` character to `_`, so packages become `semio.s.norm_eurocode.en1990`.

| Plugin | Families | New identity |
|---|---|---|
| `norm-energy` | din18599 | `s.norm-energy.din18599` |
| `norm-eurocode` | en1990, en1991, en1997, en1998 | `s.norm-eurocode.<family>` |
| `norm-eurocode-structural` | en1992, en1993, en1994 | `s.norm-eurocode-structural.<family>` |
| `norm-eurocode-materials` | en1995, en1996, en1999 | `s.norm-eurocode-materials.<family>` |
| `norm` | din4108, din16798, iso16757, vdi3805 | unchanged `s.norm.<family>` |

Three textual forms were rewritten for the eleven moved families: `s.norm.<F>` to `s.<plugin>.<F>` (kind, dialect, schema, inference, mutation and controller ids, activation kinds, playground `app =` rows, seeded `🧩️component/🔣️.json`, generated catalog, fixtures, `.feature` files), `s/norm/<F>` to `s/<plugin>/<F>` (schema document `$id` and `$ref` URLs), and `semio.s.norm.<F>` to `semio.s.<plugin_>.<F>` (252 checked-in `.proto` packages).

Scripted rewrite: 3253 files (2779 json, 252 proto, 179 rs, 27 ts, 11 feature, 4 toml, 1 md). Hand edits, 20 further files, for identity built from the family name:

- `📇️registry/🧬️contract/🦀️.rs`: new `artifact_owner(artifact)` (family to plugin id); `package_from_schema` expects `s.<owner>.<artifact>`.
- `🏷️ids-chrome-and-labels/🦀️.rs`: `editor_controller_id(owner, variant)` (it had no callers).
- `👁️viewer-surface/🦀️.rs`: `norm_viewer_surface!` takes `owner = "<plugin>"` and builds `s.<owner>.<variant>@1/*#viewer`; all 15 viewer invocations pass it (`norm` for the four root families).
- `🧪️tests/🪟️visual-imports/🟦️.ts`, `🌎️hub/…/🧪️tests/🔬️rust-package-commands/🟦️.ts`: owner lookup for the expected dialect and app id.
- `🌎️hub/…/🖥️app-surface/🧪️tests/🧰️shared/🦀️.rs`: expected kinds use `plugin.manifest.plugin_id`.

Deliberately unchanged: `semio.norm.<F>/v1` document schemas and codec ids (not an `s.<plugin>` identity, no check ties them to the owner), `computation.norm.<F>` report kinds (no owner check reads them), `norm.<F>` interaction domains, body keys and DSL ids, crate and package names, the shared `s.norm.norm` app schema descriptor. Prose mentions of `s.norm.*` in doc comments and python oracle docstrings were left.

Not verifiable without a build, and expected to refresh on the next cycle: schema and pack hashes, `.🧾️store` build records, and the seeded component descriptors, which the describe step regenerates. The shared crates `semio-s-artifact-norm-contract` and `semio-s-plugin-norm-app-surface` changed, so the `norm` component will rebuild once more.

Remaining risk: any proc macro that turns a schema id into a Rust identifier would now meet a hyphen. No such conversion was found in the schema, registry or manifest modules, but macros were not exhaustively read.

## 4. Not statically decidable

- Framework-appended actions (history, clipboard, interaction, document transfer, cancellation, host event) are not covered by `validate_interactive_job_classification`. They are shared by every plugin, so they behave as in the other plugins.
- `preflight_native_snapshots`, `preflight_artifact_codec_bindings` and the schema registry commit need the real registries at runtime. All 15 families declare their schema, inference, composer and language rows through the same `declaration()` shape.
