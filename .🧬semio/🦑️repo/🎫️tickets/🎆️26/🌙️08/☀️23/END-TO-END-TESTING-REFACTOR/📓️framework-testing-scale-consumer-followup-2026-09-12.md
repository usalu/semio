# Framework Scale Consumer Follow-up Audit

Date: 2026-09-12  
Scope: read-only call-path classification for the two `SCALE_COMPONENT_ARTIFACT` consumers. No source, configuration, Git, or artifact changes were made.

## Correction to the preceding acceptance audit

The prior report was accidentally written into a sibling ticket path whose `🧬` directory name includes U+FE0F (`.🧬️semio`) rather than the opened ticket root `.🧬semio`. This was my path-construction error: I inserted the variation selector in the `apply_patch` path and did not compare that literal path with the reopened ticket root before writing. The misplaced report is preserved as requested.

Its “production package scripts still depend on a fixture artifact” finding is withdrawn. The containing script files are broad dispatchers, but the actual consumers are test/benchmark execution paths:

| Consumer | Reachable command | Classification |
| --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts:320` | `ui-patch-marshalling-check --native` | Host test execution. The registered Nx targets at `📋️project.json:53-70` run this exact check and native-test check. It requires `SEMIO_TEST_ARTIFACT_DIR` and passes the component only to exact Cargo laws. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:5363` | `bench plugins native` | Native benchmark harness. The only use is in `BenchPluginsScript`; `bench-plugins-native` at `📋️project.json:268-280` builds the fixture component first and invokes that command. |

Neither is a production-runtime data dependency. The fixture component remains correctly located at `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/dist/component/semio_framework_os_scale_fixture.wasm`.

## Current reference validity

The host source reader is current: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts:306` resolves `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🦀️.rs`, and that file exists. The check does not refer to the removed `🧪️tests/⚖️scale` root.

No move of the existing host check or dev benchmark task is required by the testing taxonomy. Their command-level reachability establishes testing/benchmark scope. Keeping the locator declaration module-level is a possible local readability improvement, but it is not a taxonomy correction and should not be used to justify moving entire dispatchers.

The separate stale scale `sourceRoot` and two `:build` launch references remain genuine corrections from the earlier audit; this follow-up neither changes nor broadens them.
