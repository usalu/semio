# Norm split fallout (five-plugin layout)

Final layout: `norm` (din4108, din16798, iso16757, vdi3805), `norm-energy` (din18599), `norm-eurocode` (en1990, en1991, en1997, en1998), `norm-eurocode-structural` (en1992, en1993, en1994), `norm-eurocode-materials` (en1995, en1996, en1999). The first pass targeted a three-plugin layout; the second pass (below) redid everything for five. Generators: `norm-split-fallout-tests.py` (pass 1) and `norm-split-fallout-tests-5.py` (pass 2, regenerates every per-crate test file from the family blocks of pass 1). Where the pass-1 text below says three plugins, read the five-plugin table here instead.

## Pass 2 deltas (five plugins)

- Per-crate test files (`🧪️tests/{🔬️surface,🎯️action-args,🚦️compliance-gate,🖥️app-surface}/🦀️.rs`, norm root files plus extension-local files) regenerated: norm 4 families, energy 1, eurocode 4, structural 3, materials 3. Shared `🧰️laws`, `🧰️harness` and `🧰️shared` files unchanged. The gaming source scan stays in `norm` only.
- New crates `norm-energy` and `norm-eurocode-structural`: `[[test]]` `surface_render` and `compliance_gate`, dev-dependencies (own `*-examples`, fixture reader, compliance, `semio-plugin` with `artifact-app-testing`, async macros, serde, serde_json), lib `#[cfg(test)]` modules, `surface-render-test` route and target, `normTree` inputs. `norm-eurocode-materials` dev-dependencies trimmed to en1995, en1996, en1999 examples. `semio-hub-norm` lost `din18599-examples` and its description names four families.
- `🔬️rust-package-commands`: `NORM_PLUGINS` has five rows (package id, variants and component budget loops follow).
- Catalog fixture, stories (`NormEnergy`, `NormEurocodeStructural`), program-matrix TS set and JSON `pluginVerbs`, bootstrap selection (specs and `ALL_PACKAGES`), nextest overrides, TS budget table and its JSON fixture: all extended for `norm-energy` and `norm-eurocode-structural`.
- package-runner fixture lists five descriptors, test expects 5 (editors 4+1+4+3+3 = 15). time-travel contract regex `/^norm(-|$)/` already covers all five.
- Plugin README (EN and DE) names the three Eurocode plugins and `norm-energy`.
- Play: `prepare-prototype` gained `prepare-din18599` and `prepare-en1995` (activate lanes were already there, 39 lanes now); `playscopes` fixture has five activations in project order and five components (sorted); play README 39 lanes and five-lane text (EN and DE).
- Not verified: nothing was run (no cargo; the repo TS runner blocked on the graph load).

## Pass 1 report (three-plugin layout, superseded where it conflicts)

The 15 norm families now live in three plugins: `norm` (din4108, din16798, din18599, iso16757, vdi3805), `norm-eurocode` (en1990, en1991, en1997, en1998) and `norm-eurocode-materials` (en1992, en1993, en1994, en1995, en1996, en1999). This report lists everything outside the already-split sources that still assumed one `norm` plugin. Generator for the Rust test files: `norm-split-fallout-tests.py` (same folder).

No Rust, wasm, cargo or nextest was run. Checks done: JSON validity of every touched JSON file (python), cross-reading of paths and `include!` depths. The TS suites could not be executed: `repo run @semio-tech/semio-tech-play:test` waited more than four minutes on the project-graph load of another running checkout and was stopped (the 300 s limit). The affected suites are therefore unverified and should be run once the native build ends: play `playscopes` and `playactivation`, norm `package-runner`, library `test-level-budgets`.

## 1. Norm hub tests (Rust)

Shared law files hold the macros and helpers once; each plugin has a thin per-crate file with only its own family list, pulled in with `include!`.

| File | Change |
|---|---|
| `🌎️hub/🧩️compositions/📕️norm/🧪️tests/🧰️laws/🔬️surface/🦀️.rs` (new) | Shared `surface_law!`, `set_active_example_case!`, `roster_examples_case!` and `example_ids_on_disk`; the artifacts root comes from a per-crate `ARTIFACTS_ROOT` const because `fixture_path!` is manifest-relative. |
| `…/🧪️tests/🧰️laws/🎯️action-args/🦀️.rs` (new) | Shared value-tree argument helpers and `value_tree_case!`. |
| `…/🧪️tests/🔬️surface/🦀️.rs`, `…/🧪️tests/🎯️action-args/🦀️.rs` | Reduced to the five norm families, include the shared laws. |
| `…/🧪️tests/🚦️compliance-gate/🧰️harness/🦀️.rs` (new) | The family-independent gate harness (report type, copy/path/remedy/annex checks, `gate_family`). |
| `…/🧪️tests/🚦️compliance-gate/🦀️.rs` | Five families only (assert 5, not 15). It keeps the evaluate-path gaming source scan over the whole artifact tree, which therefore still covers all 15 families once. |
| `…/🖥️app-surface/🧪️tests/🧰️shared/🦀️.rs` (new) | The surface-render and retained-cohort test bodies, parameterised by `plugin()`, `VARIANTS` and `assert_family_pairs!()`; fixture rows and cohort apps are filtered by `VARIANTS` (30 rows and 120 bodies still asserted on the unfiltered fixture; 15 cohort apps still asserted by the fixture's own counts). |
| `…/🖥️app-surface/🧪️tests/🖥️app-surface/🦀️.rs` | Now only the `norm` root: five variants and five pairs. The `surface_render` `[[test]]` path is unchanged. |
| `…/📦️packages/🦀️rust/Cargo.toml` | Removed the ten `en*-examples` dev-dependencies, which are now unused; description updated to five families and the extension plugins. The `[[test]]` targets stay and now cover five families. |
| `…/🧩️extensions/📗️eurocode/🦀️.rs`, `…/📙️eurocode-materials/🦀️.rs` | Added the `#[cfg(test)]` surface and action-args test modules (same region as in `norm`). |
| `…/🧩️extensions/{📗️eurocode,📙️eurocode-materials}/🧪️tests/{🔬️surface,🎯️action-args,🚦️compliance-gate,🖥️app-surface}/🦀️.rs` (new, 8 files) | Per-plugin family lists: eurocode 4 families, materials 6. The eurocode gate carries `leaked_print_dsl` (only en1998 needs it). |
| `…/🧩️extensions/*/📦️packages/🦀️rust/Cargo.toml` | Added `[[test]]` `surface_render` and `compliance_gate`, and dev-dependencies (fixture reader, the family `*-examples`, `semio-s-plugin-norm-compliance`, `semio-plugin` with `artifact-app-testing`, async macros, serde, serde_json). `🌎️hub/Cargo.lock` already lists exactly this set. |
| `…/🧩️extensions/*/📦️packages/🦀️rust/📜️script.ts` and `📋️project.json` | Added the `surface-render-test` route (with `RUST_MIN_STACK`) and target, plus a `normTree` named input and `inputs` on both targets, so a change to the shared norm tests or artifacts invalidates the extension tests. |
| `…/🧪️tests/🔬️rust-package-commands/🟦️.ts` | `NORM_PLUGINS` table; `SurfaceRenderSourceScript` reads playground variants from the three component manifests (the hub manifest no longer carries them) and checks the exact `package_id` of all three plugins; `ComponentBudgetScript` weighs all three shipped components. The "15 editors/viewers/results windows" checks scan the whole artifact tree and stay correct. |

## 2. Other references

| File | Change |
|---|---|
| `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🧫️fixtures/🔣️.json` | `installedTargetIds` gained `norm-eurocode` and `norm-eurocode-materials` (source deployment owners, like `stdio-bim`). `installedVariants` already lists the en* variants and stays. |
| `✏️s/🧑‍💻dev/📖️stories/🎭️plugins/🧪️.story.tsx` | Added `NormEurocode` and `NormEurocodeMaterials` stories. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts` | The `setSnapshot` staging for norm kinds was gated on `pluginId === "norm"`; now a `NORM_PLUGIN_IDS` set of the three plugins. |
| `…/🧑‍💻dev/🧪️tests/🧫️fixtures/🧮️program-matrix.json` | Added `norm-eurocode` and `norm-eurocode-materials` to `pluginVerbs` (`setSnapshot`). |
| `🌎️hub/🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection/🟦️.ts` | Two new package specs and the two ids in `TRUSTED_BOOTSTRAP_ALL_PACKAGES`, in order after `norm`. |
| `🌎️hub/.config/nextest.toml` | 1800 s slow-timeout overrides for the two new packages, as for `semio-hub-norm`. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/⏱️budget/🟦️.ts` and `🧪️tests/🧫️fixtures/⏱️test-level-budgets/🔣️.json` | `quick` budget 1 800 000 ms for the two new packages, in both places (the test requires the two to be equal). |
| `✏️s/🔌️plugins/📕️norm/🧪️tests/⏪️time-travel/🧪️contract/🟦️.ts` | The playground port lookup filtered `pluginId === "norm"` and expected 15; now matches the three plugin ids with `/^norm(-\|$)/`. |
| `✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/🏃️package-runner/🔣️.json` | `descriptor` replaced by a `descriptors` list of the three component descriptors; `expectedEditors` stays 15 over the union. |
| `✏️s/🔌️plugins/📕️norm/🧪️tests/🔬️package-runner/🟦️.ts` and `✏️s/🔌️plugins/📕️norm/🧪️tests/🔬️typescript-package-commands/🟦️.ts` | Read and union all three descriptors (`normPackageTestPlan` returns `descriptors`). |
| `✏️s/🔌️plugins/📕️norm/README.md` | English and German intro name the two Eurocode plugins and say DIN/ISO/VDI stay in `norm`. |
| `🏢️semio-tech/🎡️play/📋️project.json` | `prepare-prototype` lacked `prepare-en1990-react-prototype` and `prepare-en1992-react-prototype` (activate lanes were already added); `playactivation` requires prepare and activate lane sets to be identical. |
| `🏢️semio-tech/🎡️play/🧪️tests/🧪️playscopes/🧫️fixtures/🔣️.json` and `🟦️.ts` | `activation` became `activations` (the Standards scope now depends on three lanes, and `components` pins `norm`, `norm-eurocode`, `norm-eurocode-materials`). |
| `🏢️semio-tech/🎡️play/README.md` | 35 to 37 lanes (EN and DE), `serve-standards` and e2e text name the three norm lanes and closures. |

## 3. Deliberately left

- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🧪️tests/🧫️fixtures/❌️invalid/🔣️.json`: `rust_package: "semio-hub-norm"` is an intentionally invalid vector.
- `🌎️hub/🧩️compositions/🧪️tests/🧫️fixtures/📇️ownership/🔣️.json`: it lists top-level plugin components only; stdio extension components are not in it either, and the new extension components lack the `🛂️.descriptor.semio` that test requires. No row added.
- `🌎️hub/Cargo.lock`: generated; it already has both extension packages with the dev-dependencies above. The `semio-hub-norm` block may still list the removed `en*-examples` until cargo rewrites it.
- `🔌️plugin` host tests, `ui` chrome tests and `choice-source-tests.rs`: use `"norm"` as a synthetic plugin id, no family ownership.
- `📋️build-owners.json` (sealed index) and the generated catalog: not hand-edited; regenerate with the repo tooling.
- Artifact-level fixtures that name `s.norm.en1990` (kind only): untouched per the brief. `🧑‍💻dev/🧹️fixture-sweep` and the `🌎️hub/package.json` artifact paths reference artifact crates, not the plugin.
- The `compliance_gate` source scan runs only in `norm` (it walks all artifacts); the two extension gates skip it.
