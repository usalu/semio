# Tree health repairs for the XLSX pipeline

Observed by `repo` CLI and `repo cargo` (store shared, configuration prototype); logs under `🗑️generated/` (`tree-health-graph*.log`, `*-check*.log`). The `repo` MCP tools were unusable (PreToolUse hook timeouts), so the `repo` binary was used instead.

## Breaks

| # | Break | Finding | Status |
|---|---|---|---|
| 1 | Graph: `@semio-tech/stdio-mp3` workspace import unresolved | mp3 package (`package.json`, `📋️project.json`, `📜️script.ts`) was regenerated at 06:22, older than 15 min. `repo show project @semio-tech/stdio-mp3` and `@semio-tech/stdio-js` now resolve without error. | Gone, no edit needed |
| 2 | Hub lock cycle `semio-s-artifact-stdio-pdf-tests-2` depends on itself | A path-dependency scan over the pdf closure (dev-dependencies and quoted keys included) found no cycle. `cargo metadata` on the crate succeeds. The pdf manifests are still being edited by others. | Gone (resolved by others' edits), no edit needed |
| 3 | `semio-plugin-app-builder` does not compile | `cargo check --lib` passes (exit 0). The first two attempts hit a transient `mio` registry-extraction race (concurrent cargo). | Gone, no edit needed |
| 4 | `semio_plugin::artifact_app_laws` missing in `contract-editing-details` tests | `semio_plugin::artifact_app_laws` is a facade re-export gated on feature `artifact-app-testing` (module now lives in `semio_plugin_testing_laws`). The details crate's dev-dependencies enabled only `mutation-testing` and had no `semio-plugin` dev-dependency with that feature. | Fixed |
| 5 | ENOENT on `🔏️index/🧊️stamps` | The directory exists (mtime 05:16). `repo graph` passes source admission now. | Gone, no edit needed |
| 6 | `resolveCanonicalTestTargetsV1` `route-undeclared:test-build-test-file` | The route `build-test-file` is now declared in the build suites table (06:26). The graph no longer refuses at aggregate resolution. | Gone, no edit needed |
| 7 | `jcoprobe.js` under `🌐️browser-bundles/📞️out-callback/` missing | It is a generated output declared in the taxonomy, produced by the jcoprobe browser bundle build, not a hand-written source. The two importers (`🧪️tests/🧩️jco-callback/🟨️.mjs`, `🧩️jcoprobe-callback/🟦️.ts`) are consistent with the taxonomy. | Not a source break; needs the bundle target to run (not run here) |
| 8 | `semio-plugin-document-media` missing `semio_framework_hash` | The manifest already declares `semio-framework-hash` (04:29). The crate compiled as part of the builder check. | Gone, no edit needed |
| - | `hostUnderPressureV1` export | Exported at `🌡️host-capacity/📏️decision/🟦️.ts:132` and imported by the gate. | Exists |

## Sweep

- `cargo check --workspace` on the xlsx workspace: pass.
- `cargo check` on `semio-hub-stdio-component`: pass (second run). The first run failed with `MutationLeaf source authority failed: Operation not permitted` in the semio value leaves; it did not recur. `mutation-source-authority/🦀️.rs` was being edited at 12:17.
- `semio-plugin-runtime-instances` failed once with `document_load_diagnostic` not found on `PA`, while another agent was adding the `PluginApp::document_load_diagnostic` default (files modified 12:50). I waited and rechecked: pass.
- `contract-editing-details` `--lib --tests`: pass, so its unit tests can now run.
- `repo graph` (whole workspace) is not usable: a whole-workspace read takes hours and ends with a transient `openat EINTR` in the dependency sync. Single-project queries work.

## Files changed

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/📦️packages/🦀️rust/Cargo.toml` (dev-dependency `semio-plugin` with feature `artifact-app-testing`)
- This report.

## Next

Run `repo test @semio-tech/stdio-artifact-contract-editing-details-rs`, then the hub catalog count test and the descriptor regeneration from `xlsx-descriptor-gates.md`.
