# Print tests and production-to-test edges

## Print `testing/*` consumers

- No consumer of the removed `@semio-tech/print/testing/*` exports remains (only historical ticket tooling mentions the string).
- The remaining export `./modules/print-font-catalog/metrics/testing` is not a cross-project edge (its source lives inside the print package); its only consumer is the probes file.

## print-viz-inference:test

- Earlier run: 629/631. This run found one error: `contract/owned-semantic-inference-and-text-boundary: no schema with key or ref "http://json-schema.org/draft-07/schema#"`.
- Root cause: `print/🧬️schema/💡️inferences/🔣️.json` declared `$schema` draft-07 but uses `$defs` (2019-09+ vocabulary) and its sibling native schema is 2020-12; the independent oracle compiles it with `Ajv2020`, which has no draft-07 meta-schema. Fixed at the source: the schema now declares 2020-12 (no assertion loosened).
- The other earlier error did not reproduce (the run before the fix already showed 630/631 with the single error above); cause of the second earlier error not determined (likely environmental).
- Result after fix: `level=quick checks=631 passed=631 failed=0 errored=0`.

## Production-to-test project edges in the closure

Computed from the sealed generation (`.cache/b/g/m/generations`, generation 37; closures: course 57 projects, proctor 194). Of 200 pairs (name heuristic `test-*`/`*-test`), three originate inside the course/proctor closure:

| edge | source of edge | action |
| --- | --- | --- |
| `@semio-tech/framework-kernel` -> `test-framework-modules-kernel-784658` | 7 re-export files in `kernel/📦️packages/🟦️typescript/🧪️testing/` | removed |
| `@semio-tech/framework` -> `test-framework-modules-raster-video-0c06e3` | `framework/📦️packages/🟦️typescript/🖌️raster/🎥️video/🧪️testing/🔬️unit` | removed |
| `@semio-tech/framework` -> `@semio-tech/framework-test` | deliberate SDK surface (`./test/adapter`, `./test/gherkin` re-export `framework/🔨️modules/🧪️test`, a production test-support module used by every plugin's adapter) | not a bug; left |

Changes:
- Deleted kernel `🧪️testing/` (7 files), framework `🎠️kernel/🧪️testing/` (re-export of the kernel one) and `🖌️raster/🎥️video/🧪️testing/`; removed the 7 `./testing/*` exports of kernel and `./kernel/testing/fixtures`, `./raster/video/testing/unit` of framework.
- Consumers now import the real test modules relatively: 9 files that imported `@semio-tech/framework-kernel/testing/fixtures`, `ShellHelpers` command-rejection test (`@semio-tech/framework/kernel/testing/fixtures`), `VideoRenderHost/🧪️tests/🔬️unit`, kernel `🧪️tests/🔬️owned-verify` (4 imports), os `🧪️tests/🔬️owned-verify` (3 imports).

Edges outside both closures, not touched (same bug class, from the generation-37 pair list and the earlier note): `fem-js`, `framework-plugin`, `norm-js`, `repo-lib` (2), a `breach-` script, many Rust `semio-s-artifact-*` -> `*-tests` pairs (e.g. bim-model-formula, norm-en1997-io, puzzle-5d, stdio-pdf, forms); also the os `VideoRenderHost/🧪️testing/🔬️unit` import in the os owned-verify script was left as is.

## Verification

See the final report for commands and exit codes.

- `repo run @semio-tech/print-viz-inference:test --skip-remote-cache`: exit 0, 631/631.
- `repo run @semio-tech/print:check --skip-remote-cache`: exit 1, pre-existing and unrelated: "Authored TypeScript artifact producer required". `print/🧪️tests/🔬️script-commands` `CheckScript` calls `runArtifactTypeScriptPackageMain`, but print's `📋️project.json` has no `metadata.semio.artifactPackage` (kind/source/target); unchanged at HEAD. Not fixed (needs a decision on the package source and a `build` target producing `dist`).
- `repo run @teaching/architecture-course:typecheck --skip-remote-cache`: exit 0.
- `repo run @teaching/architecture-course:check --skip-remote-cache`: exit 0.
- `repo run @semio-tech/framework-kernel:verify-ui-dirty-scope --skip-remote-cache`: exit 0 (exercises the rewired kernel owned-verify import).
