# 🧹️ Procedural cleanup (slice C) — every assembly referrer and what happened to it

Ticket `$T = .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/EXTRACT-WFC-PLUGIN/`. Logs under `$T/🗑️generated/cleanup/`.
Ticket start commit `7bea15c349`. Plan §5 minus the physical folder deletion.

> **`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly` is still on disk — delete it after A3/A5 finish
> porting mutation folders, fixtures and examples; then remove its `Cargo.lock` entries via a
> `cargo metadata` refresh.** (This run's cargo commands already dropped
> `semio-s-artifact-procedural-assembly` from `Cargo.lock` because the workspace member is gone, so a
> plain `cargo metadata` after the deletion is enough — nothing to hand-edit.)

The plugin folder emoji changed mid-slice from `🌊️wfc` to `🀄️wfc` (coordinator, collision with
`🌊️flow`); every path this slice wrote names `🀄️wfc`. Artifact ids are unchanged (`s.wfc.wfc3d`,
`s.wfc.wfc3d.solve`).

## 1. Referrer ledger

| # | file | what it was | what I did |
|---|---|---|---|
| 1 | `✏️s/🔌️plugins/🌀️procedural/🦀️.rs` | `ProceduralApps::{AssemblyEditor,AssemblyViewer}`; `register_assembly_inference_factory(&ActionBus::production())`; `.routed_inference(assembly_inference_metadata())`; `.artifact(…assembly::declaration())`; `.editor_with_examples/.editor_mutation_roster/.viewer/.viewer_mutation_roster` ×4; `.activation(OnArtifactKind{assembly})`; enum docstring "2D, 3D, and assembly"; `requests.reason` | all removed; docstring → "2D and 3D"; reason → "persist generation2d/generation3d editor edits to the open document". `ActionBus` came from the glob prelude, so no import removal was needed. Net −14 lines. |
| 2 | `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🔬️surface/🦀️.rs` | `assembly_editor_and_viewer_share_dialect`, `assembly_manifest_examples_are_registered_on_the_editor_surface`, `assembly_apps_are_declared_on_the_plugin` | all three deleted (−25 lines). No `use` lines named assembly. |
| 3 | `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml` | `semio-s-artifact-procedural-assembly` dependency; `[[package.metadata.semio.playground]] variant = "assembly"` (ports 6019/6119); crate `description` | dependency + playground row removed (**6019/6119 released — do not reuse**; note `.claude/launch.json` already binds 6019 to `shooting-react-attach`); description → "…for independently packaged generation2d and generation3d artifacts". |
| 4 | root `Cargo.toml` | workspace member `"✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/📦️packages/🦀️rust"` + `[workspace.dependencies] semio-s-artifact-procedural-assembly = { path = … }` | both single lines removed, re-reading immediately before the edit (peers edit this file concurrently — one cargo run raced a half-written peer state and was simply retried). **No `[profile.wasm-dev.package.semio-s-artifact-procedural-assembly]` override existed.** |
| 5 | root `Cargo.lock` | member + package entries | auto-refreshed by this slice's cargo runs; `grep -c procedural-assembly Cargo.lock` = 0. |
| 6 | `📜️script.ts` (root) | tool-run-payload row `{ toolId: "s.assembly.solve", root: …🧩️assembly…, lane: "W3 (3) assembly", inventory: "§2.3" }` | row removed (one line). No wfc replacement added — the wfc artifact slices own their own rows. |
| 7 | `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/📚️example-picker.json` | example row `two-room-corridor @ s.assembly` and case `another-artifacts-surface-never-sees-them` with `app.id = s.assembly@1/*#viewer` | both swapped to `s.wfc.wfc3d` / `s.wfc.wfc3d@1/*#viewer`; the example id `two-room-corridor` and the expectations are unchanged. Owning crate is `semio-framework` (`🧰️framework/📦️packages/🦀️rust/🦀️.rs` `#[path]`-mounts `🛂️manifest/🦀️.rs`). |
| 8 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🧪️tests/🔬️quick/🦀️.rs` | `procedural_only_catalog()`, `declared_inferences_for_workspace_finds_the_real_procedural_roster`, owner/contributor `"procedural"`, kind `"s.assembly"`, schema `"s.assembly.solve"` (12 sites) | helper renamed `wfc_only_catalog()` → `plugin_only_catalog("wfc")`; test renamed `…finds_the_real_wfc_roster`; every fixture string swapped to owner `wfc` / `s.wfc.wfc3d` / `s.wfc.wfc3d.solve`. **`…finds_the_real_wfc_roster` reads the COMMITTED `✏️s/🔌️plugins/🀄️wfc/🛂️.descriptor.semio` through `workspace.discovery_descriptors()`, so it only goes green once slice P has run `wfc-plugin:describe` with the wfc3d routed inference registered.** |
| 9 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/🪪️artifact-admission/🔣️.json` | `firstParty` row `{plugin: "procedural", kind: "s.procedural.assembly", definition: …🧩️assembly/🦀️.rs, root: …🌀️procedural/🦀️.rs}` | replaced by `{plugin: "wfc", kind: "s.wfc.wfc3d", definition: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🦀️.rs", root: "✏️s/🔌️plugins/🀄️wfc/🦀️.rs"}`, moved to the end of the plugin-sorted list. Row count stays **39** (the TS oracle asserts exactly 39) and `cases` stays **15**. The TS oracle's `repoRoot` arm reads those two files and asserts `ArtifactDefinition::new(ArtifactIdentity::parse("s.wfc.wfc3d")` + `.package_id("semio:wfc")` — green only once A5/P have written them. |
| 10 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️.rs:1` | module doc "Exact ActionBus routes such as `s.assembly.solve`" | → `s.wfc.wfc3d.solve`. |
| 11 | `.vscode/launch.json` | `🛠️dev🌀️procedural🧩️assembly⚛️react` (6019) and `🛠️dev🌀️procedural🧩️assembly🧊️wgpu🌐️wasm` (6119), both `SEMIO_APP: "s.assembly@1/*#editor"` | both configuration objects removed; JSONC re-parsed clean (267 configs). **This file is GENERATED** by `plugin-registry:generate` from the Cargo playground metadata, so the removal is now also reproduced from source: after regeneration `grep -c assembly .vscode/launch.json` = 0. |
| 12 | `.vscode/🧩️launch.seed.jsonc` | — | **no assembly rows existed**; the dev entries come from the Cargo playground metadata, not the seed. Nothing to do. |
| 13 | `.claude/launch.json` | — | **no assembly attach entry existed.** 6019 there belongs to `shooting-react-attach` and was left alone. |
| 14 | `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/🟦️.ts` | — | **no assembly barrel exports existed** (grep over the whole `📦️packages` tree returns only `component-app-assembly` / `PluginAssemblyError` / `UiAssemblyResult` false positives). Nothing to do. |
| 15 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | `members-of-artifacts: "🧩️assembly"`; `members-of-tests: "🧩️mutate-assembly-1"`; `members-of-inferences: "🧩️wfc-engine"` | all three removed. **Kept `🧩️appends-slot-c-at-index-2`** (`members-of-tests`) — it is a generic fixture-case name A3/A5 are porting verbatim into the wfc artifacts; removing it would break them. Generic taxonomy entries `assembly` (kind), `assembly-source`, `👥️assembly`, `webassembly` are unrelated and untouched. |
| 16 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` / `📓️schema-catalog.md` | 10 keys `s.procedural.assembly.1.any[.mutation.*]` | **left alone deliberately.** The catalog is rendered from the schema modules ON DISK (`📜️script.ts:14960` — `schema-catalog-stale` = "does not match the schema modules on disk"), so the keys cannot disappear while the folder exists. Run `bun ./📜️script.ts schema generate` AFTER the deletion. |
| 17 | `✏️s/🔌️plugins/🌀️procedural/🔣️.json` + `🛂️.descriptor.semio` | generated, carried `s.assembly` | regenerated by `procedural-plugin:describe`; both now contain **zero** assembly strings (`🔣️.json` shrank by 3 110 lines, descriptor 299 105 → 279 397 B). |
| 18 | `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorcompileclosure/🟦️.ts:26` | `expect(cargo).not.toContain("semio-s-artifact-procedural-assembly")` | **kept** — a negative assertion that stays true and gets stronger after the deletion. |
| 19 | `.storybook/**` | — | **no assembly or procedural references at all.** Nothing to do. |
| 20 | `🧰️framework/🔨️modules/🌱️value/✨️derive/🧪️tests/🆔️newtype-transparent/🦀️.rs:8` | doc comment pointing at `…🧩️assembly/…/💡️inferences/🧩️wfc-engine/🆔️ids/🦀️.rs` | repointed to `✏️s/🔌️plugins/🀄️wfc/⚙️engine/🆔️ids/🦀️.rs`. |
| 21 | `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/🦀️.rs:43-47` | comment "`wfc` dissolved into the Assembly artifact (`…🧩️assembly/…/🧩️wfc-engine/`)" naming `AssemblySolve`/`AssemblyContradiction`/`AssemblyEntropy` | rewritten to name the new engine crate `✏️s/🔌️plugins/🀄️wfc/⚙️engine/`; the 626/626 symbol-parity claim is kept verbatim. |
| 22 | `♻️mit-bestand/🔎️recherche/_archive/.../piece_bauteilpass_interface.md:1020` | `logistics.assembly_access_zones` — a false positive on the `s\.assembly` pattern inside an archived research note | untouched. |

Final re-grep of `procedural_assembly|procedural-assembly|s\.assembly|data\.assembly|AssemblyEditor|AssemblyViewer|assembly_inference|s\.procedural\.assembly|wfc_engine|wfc-engine`
(excluding `node_modules`, `target`, `dist`, `⚡️cache`, `🗑️generated`, `🎫️tickets`, `.git`, the
`🧩️assembly` folder and both wfc plugin folders) leaves **only** rows 16, 18 and 22 above, plus
legitimate `semio-s-plugin-wfc-engine` hits in `Cargo.toml`/`Cargo.lock`.

## 2. Commands and results

| gate | command | result |
|---|---|---|
| native check | `cargo check -p semio-s-plugin-procedural --lib --tests -j 4 --message-format=short` | 🟢 exit 0 (1 m 21 s). Log `check-procedural-native.log`. First attempt aborted on a peer's half-written `🀄️wfc` workspace member; retried, unrelated to this slice. |
| native lib tests | `RUST_MIN_STACK=33554432 cargo test -p semio-s-plugin-procedural --lib -j 4 -- --test-threads=4` | 🟡 8 passed, **1 pre-existing failure**: `surface_tests::generation2d_viewer_never_mutates` panics in `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81` "ordered-map root must be explicitly retired before drop". Fails identically with `--test-threads=1`. `git diff 7bea15c349` shows **zero** changes to `🗂️ordered`, `📡️replication`, `🔌️plugin/🦀️.rs` and `🌀️generation2d/` — a peer's framework state, not this cleanup. Log `test-procedural-native.log`, `test-procedural-single.log`. |
| integration targets | `cargo test -p semio-s-plugin-procedural --test close_ladder --test boot_deadline --test idle_turns -j 4` | 🟡 `boot_deadline` 1/1 🟢, `close_ladder` 3/3 🟢, `idle_turns` **2 pre-existing failures** (`an_unacknowledged_open_retains_nothing_per_turn` 31 324 B/turn; `generation2d_idle_turns_are_settled_and_retain_nothing` 29 365 B/turn) — `generation3d_idle_turns…` passes. Retention laws over untouched generation2d code. Log `test-procedural-targets.log`. |
| wasm | `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 -j 4` | 🟢 exit 0 (1 m 27 s). Log `check-procedural-wasm.log`. |
| manifest fixture | `RUST_MIN_STACK=33554432 cargo test -p semio-framework --lib -j 4 example_picker` | 🟢 `manifest::example_picker_tests::every_surface_of_a_dialect_resolves_the_same_example_picker` ok. Log `test-framework-example-picker.log`. |
| admission fixture | `cargo test -p semio-framework-plugin --lib -j 4 strict_artifact_identity_matches_independent_neutral_fixture` | 🟢 ok (15 cases). Log `test-plugin-admission.log`. |
| MCP inference quick | `cargo test -p semio-framework-os-mcp --lib -j 4 inference` | 🔴 **blocked by pre-existing peer breakage** — the `semio-framework-os-mcp` lib TEST target does not compile at all: 10 errors in `📇️registry/🧪️tests/🔬️quick` (`note_descriptor`/`cad_descriptor`/`note_and_cad_source` missing from the crate root), `🏠️workspace/🔗️remote` (`DirectorySessionAuthorityV1.expires_at_ms`), `🏠️workspace` (`ProbeSnapshot: ArtifactCompositionFields` unsatisfied) and `🚚️transport` (`HttpAdmission::HostSnapshot`). **None is in `💡️inference/🧪️tests/🔬️quick/🦀️.rs`**, and `git diff --stat 7bea15c349 -- 🌉️mcp/` shows my swap is the ONLY change in that module since ticket start, so the target was already red at the ticket start commit. Not fixed (peers' breakage). Log `test-mcp-inference-quick.log`. |
| describe | `NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false bun nx run @semio-tech/procedural-plugin:describe` | 🟢 exit 0 on the second attempt. **First attempt died on `No space left on device`** (volume at 100 %, 4.8 GiB free; the shared build dir is 323 GB — 196 G debug / 101 G wasip2 / 23 G wasm32-unknown-unknown). I pruned NOTHING: every `debug/incremental` session is <45 min old and `wasm32-unknown-unknown` had 18 936 files written today, so all of it is live fleet work. Space freed itself (24 GiB) when peers' builds finished. Logs `describe-procedural.log`, `describe-procedural-2.log`. |
| registry generate | `bun nx run @semio-tech/plugin-registry:generate` | 🟢 exit 0, "60 plugin crates, 65 playgrounds, 54 framework packages"; also regenerated `.vscode/launch.json`. `🔌️plugins.json` and `🎠️playgrounds.json` contain 0 assembly strings. Logs `registry-generate.log`, `registry-generate-2.log`. |
| registry check | `bun nx run @semio-tech/plugin-registry:check` | 🔴 exit 1, **2 315 findings repo-wide** — a long-standing red gate (🗄️stdio 1 248, 📕️norm 246, 🏗️fem 80, …). **Procedural: 25 findings, exactly ONE of which names assembly**: `🗿️artifacts/🧩️assembly/…/🧪️tests/🧩️mutate-assembly-1/🦀️.rs is not reachable from Cargo manifest 📦️packages/🦀️rust/Cargo.toml` — it disappears with the folder. The other 24 are pre-existing window/surface-shape findings identical in form to generation2d/generation3d's. **🀄️wfc: 128 findings** (missing example `🟦️.ts`/`🖼️assets`, `🧪️tests` under window dirs, modes missing `🎮️commands`/`🎚️config`/`👥️presence`/`🫧️transient`) — the in-progress wfc plugin, **not** procedural-related; reported here for the coordinator, not acted on. Logs `registry-check.log`, `registry-check-2.log`. |
| schema catalog | `bun ./📜️script.ts schema check` (read-only) | 🔴 exit 1 repo-wide (`schema-export-incomplete=5717`, `schema-ref-unresolved=271`, …) and `schema-catalog-stale=1`. **I did NOT run `schema generate`**: the catalog renders from the on-disk schema tree, so the 10 `s.procedural.assembly.*` keys cannot disappear until the folder is deleted, and regenerating now would only snapshot half-authored wfc scopes that go stale again minutes later. Log `schema-check.log`. |

## 3. What the coordinator must do when deleting `🧩️assembly`

1. `rm -rf "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly"` once A3/A5 confirm the port is finished.
2. `cargo metadata --format-version 1 >/dev/null` (or any cargo command) to refresh `Cargo.lock` — nothing to hand-edit, the workspace member and alias are already gone.
3. `bun ./📜️script.ts schema generate` — drops the 10 `s.procedural.assembly.1.any*` keys from
   `🔣️schema-catalog.json` / `📓️schema-catalog.md`.
4. `bun nx run @semio-tech/procedural-plugin:describe` is **not** needed again (the descriptor already
   has no assembly content), but `plugin-registry:generate` + `:check` should be re-run to confirm the
   last procedural finding (`🧩️mutate-assembly-1` unreachable) is gone.
5. `bun ./📜️script.ts verify taxonomy enforce --scope "✏️s/🔌️plugins/🌀️procedural"` — I removed
   `🧩️assembly` and `🧩️mutate-assembly-1` from `🔣️taxonomy.json`'s member registries ahead of the
   deletion, so that scope is expected to flag those two names as unregistered **until** the folder is
   gone. `🧩️appends-slot-c-at-index-2` was deliberately kept for the wfc port.
6. Ports **6019 / 6119** are released. Do not reuse them for wfc (6019 is already claimed by
   `shooting-react-attach` in `.claude/launch.json`).
