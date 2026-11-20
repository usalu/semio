# XLSX file routing and host acceptance

The normal S React entry supplies the generated stdio format catalog to the shell. The authored XLSX representation claims `.xlsx` and `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet`. Its generated route resolves to `s.stdio.xlsx@ecma-376/*`, whose editor is published by `stdio-office`. The live route picked `Budget.XLSX` by extension despite a ZIP MIME and picked `Budget` by XLSX MIME. The published editor's Open File and Save File actions declare matching binary workbook I/O.

Added a native XLSX row to the full S browser I/O matrix. The row saves the original worksheet, changes A1 through its rendered text input and commits with Enter, verifies the new cell text in the exported workbook using fflate and Chromium DOMParser, opens the original through the shell picker, waits for the durable open notice, and compares a fresh native export with the original. The same row still exercises the standard document archive flow. Play's pane brand is ephemeral and this work uses the full S host entry.

## Files

- `🌎️hub/🧪️tests/🚪️io-matrix/🟦️.ts`
- `🌎️hub/🧪️tests/🧫️fixtures/🚪️io-matrix.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/🧮️program-matrix.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🚀️local-hub/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/🚀️local-hub.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🚀️boot/🧪️tests/🟦️.ts`
- The XLSX base demo test and its native `📕️example.xlsx` asset.
- This ticket's `validate-xlsx-routing.ts`, `run-browser-io.ts`, `probe-xlsx-pane.ts`, `inspect-browser-run.ts`, `build-xlsx-demo.ts` and this report.

## Verification

The following command passed from the repository root. It derives claims with the checked-in generator, exercises the production picker route against those claims, verifies the published Office app's matching I/O declarations, checks the neutral browser fixtures, transpiles the changed harness and checks four managed launch plans against the fixture and independent Ajv schema validation.

```sh
SEMIO_LANGUAGE=en repo './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/validate-xlsx-routing.ts'
```

The first Hub matrix process exited without leaving a result. The rerun and dedicated shell host both wait for the shared repository build graph. Port 6070 refused connections before a shell was ready.

```sh
SEMIO_LANGUAGE=en repo run os-hub:io-matrix -- --serve http://127.0.0.1:6070/ --locale en --only stdio-office/xlsx --out "$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/🗑️generated/io-matrix"
```

The exact XLSX natural-file acceptance initially exited 1 with a blocked result before browser startup. Its dev server directly invoked an OS script that requires a parent Script invocation, so the process exited with `Original parent Script process invocation required`. The shared development serve adapter now invokes the existing repository-managed target for each declared variant, renderer and profile. The neutral fixture verifies React and WGPU prototype and release targets and the WGPU port and composition overrides. With the corrected adapter, the managed serve entered graph loading. The exact acceptance then exited 1 with a blocked result because the shell did not answer within the 300 s readiness bound. Single-owner `--exclude-dependencies` launch attempts for the Hub matrix and the XLSX host also waited on the shared graph service.

```sh
SEMIO_LANGUAGE=en repo './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/run-browser-io.ts' acceptance
SEMIO_LANGUAGE=en repo './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/run-browser-io.ts' host
```

The standalone XLSX app is an independent live check on its declared React port 6272. The managed variant target is waiting on the shared build graph. Its browser probe has reported connection refused while there is no serving variant.

```sh
SEMIO_LANGUAGE=en repo './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/run-browser-io.ts' xlsx-host
SEMIO_LANGUAGE=en repo './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/probe-xlsx-pane.ts'
```

## Demo workbook

The base native demo was a tracked zero-byte file. It now contains a 2,196-byte, reproducible XLSX matching the authored `Sheet1` and `Totals` projections, including `Name` in A1 and a cached `SUM(B2:B2)` formula in B3. The new Rust test compares the native decoder's workbook with the authored DSL and checks the two sheets, A1, the cached number and formula with Calamine. The focused run was canceled in favor of the shared three-crate native verification run. The separate Chromium run passed against both native demo and natural-file acceptance fixture and refused both truncated copies. The workbook is available through natural Open File. Menu publication would require regenerating the Office component pair and updating the Play default together. The optional menu change was withdrawn after the descriptor build could not enter the shared queue.

```sh
SEMIO_LANGUAGE=en repo './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/build-xlsx-demo.ts'
SEMIO_LANGUAGE=en repo './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/validate-xlsx-routing.ts' --oracle
SEMIO_LANGUAGE=en repo './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/run-browser-io.ts' test-demo
```

## Cold development serve

The managed warm React target starts from an issued session and staged browser module outputs. A clean checkout needs the managed `dev-<variant>-<renderer>-<profile>` target to build and activate those outputs. Both WGPU target families already share the same activation dependencies and options. The shared launcher now selects the cold target on a new port and reuses an existing ready host. Fixture tests reproduced the old launch choice before the correction. The complete local-hub suite passed 28 tests and the time-travel boot suite passed 31 using the production publication staged by `project-test-catalog.ts`. The fresh standalone command uses `dev-stdio-xlsx-react-prototype`. The repo graph service remains required for this command, and the tested checkout's browser output root was absent before the run.
