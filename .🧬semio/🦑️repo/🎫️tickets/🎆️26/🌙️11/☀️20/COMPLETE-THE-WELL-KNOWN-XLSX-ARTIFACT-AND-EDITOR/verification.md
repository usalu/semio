# XLSX artifact verification

The well-known kind is `s.stdio.xlsx`. The Office component publishes the ECMA-376 editor and viewer. The host resolves both uppercase filename extensions and the spreadsheet MIME type. The routing probe exercised the production resolver and checked its result against the Office component's native open and save declarations.

The TypeScript package runs the shared worksheet viewport and unchanged-cell fixtures through its standard suite runner. The cell values are checked independently with Ajv. The package build, check and test targets now include the XLSX source tree and fixture changes in their cache inputs.

The independent acceptance oracle follows the package's office-document relationship, resolves the workbook content type and parses the workbook XML. Its checks use fflate and fast-xml-parser. It accepted all three valid XLSX samples and rejected both malformed samples and the pinned Word and PowerPoint packages. The 38-row acceptance matrix validated against its JSON schema.

The two XLSX TypeScript suites passed 13 tests with 127 assertions. They cover the shared editor fixtures, Strict and Transitional profile policy, snapshot persistence, validation, cancellation and ownership. The normal `repo run @semio-tech/stdio-xlsx:test` target exited successfully.

The S publication renderer produced the Office activation claim `on-artifact-kind:s.stdio.xlsx` and reported no withheld Office entry. The offline acceptance suite now uses this renderer's emitted catalog directly and can check registration in a clean checkout. All 22 matrix and XLSX tests passed with 212 assertions. See [editor verification](xlsx-editor.md), [native codec verification](xlsx-io.md) and [host verification](routing.md) for the other test paths.

```sh
repo run '@semio-tech/stdio-xlsx:test'
repo tools exec --light bun -- test './✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧪️contract/🟦️.ts' -t '^matrix|B/xlsx'
```

A full acceptance contract run passed 279 of 281 tests, including all XLSX rows. It reported a missing JSON sample and a missing published IDML owner in the Office descriptor. A concurrent checkout update supplied the JSON sample and corrected its manifest path after that run.

The original browser journey failed before opening a page because the development launcher started a script without the required repository invocation. The launcher now routes through the managed serve targets. The next exact XLSX acceptance run reached the managed launcher and then timed out after 300 seconds while the graph was still loading. Explicit S and XLSX development servers remain queued. The differential job exited before reaching XLSX after the graph service closed its request.

The base demo now contains the authored two-sheet workbook. Chromium accepted that workbook and the acceptance fixture, and refused truncated copies. The new demo test exposed a test-only Rust import error during the full native build. That import is corrected. A later build stopped in `semio-plugin-retained-command` because its `ArtifactApp` dependency lacked the newly referenced retirement method. The current declaration and caller agree and their modification times are within 50 milliseconds, which is consistent with the shared checkout changing during compilation. The native suite restarted from the current sources. The ticket remains open pending native and browser results.

The fourth native run compiled all three profiles and executed the base suite. The workbook creation, demo, and three native codec tests passed. The base suite reported 24 passes and eight failures, then stopped. The grid tests now admit the registered UI arena. The three history suites now point to the committed fixture root, and the insert-sheet schema hides the verbatim restoration slot. The TypeScript mutation input reader reproduced the original refusal and resolved every XLSX mutation input after the fix. The shared acceptance caller now lends the two items required to retire the original decoder. The coordinated native rerun covers all three profiles without stopping at the first failure.

The full S browser, standalone XLSX browser and Hub I/O matrix requests are live. The graph inventory is still advancing, and neither browser port is listening. The repository has no configured remote execution service and the launcher requires a managed serve when there is no already-running host.

The ticket runner waits for the existing S host before dispatching the exact XLSX browser acceptance journey. It records a failure if that host exits first. A separate Office compiler cache warm-up used the managed prototype recipe for `wasm32-wasip2`, the Hub lockfile, and default features. I stopped that optional run after it held about 2.8 GB while the native tests were queued for memory. It exited 130 and released its compiler leases. The managed browser builds remain scheduled.

The fifth native run passed 26 base, 18 Strict and 24 Transitional tests. Strict and Transitional both created a workbook, inserted a formula, saved and reopened it. History input resolution and conflict retirement passed for all profiles. The remaining failures exposed three issues. The grid tests needed to retire their rendered trees before the test UI arena closed. Canonical JSON rejected tagged non-object payloads such as XLSX numbers, even though the existing value wire wraps them under `value`. Strict construction stamped one XML root but left Transitional workbook authorities elsewhere, so the inverse test started from a mixed package. The latest changes address all three, with a separate serialization regression using a neutral JSON fixture and serde_json.

Releasing the optional Office reservation let the canonical serializer test build enter Cargo. Compilation then stopped because the persistent-tree test still used the old retained-grant API. The test now uses the current allocation-effect and retirement interfaces while retaining its identity, heap accounting, output, and cancellation checks. A combined run of the persistent-tree and tagged-value regressions is queued.

The first combined run compiled and reached the persistent-tree test, which failed while admitting its owned projection. The test now follows the funded identity projection and both source-publication turns. That run also showed that the tagged-value regression was in an unwired test file. It now has an explicit module in the canonical encoder and compares its typed projections with the shared JSON fixture. Both tests are scheduled together again.

The next canonical encoder run discovered both tests. The tagged-value regression passed. The persistent-tree test passed source admission and reached cursor retirement, where it reported that sealed retirement requires the original producer invocation. The test now uses the producer driver for traversal and retirement. The final focused run passed both the tagged-value and persistent-tree tests with exit code 0.

The sixth native run reached source admission and refused `semio-plugin-instance-app` at 262,215 bytes against its 262,144-byte limit. Redundant documentation was trimmed without changing behavior. The repository source scanner found 17 current source files and 257,997 bytes with the tested macOS configuration and the artifact testing feature enabled. The seventh run cleared the complete source admission gate and entered native compilation.

The seventh run stopped in `semio-plugin-composition-publication` before XLSX test execution. Its saved Cargo compiler diagnostic reports four E0599 errors for `preparation_refusal_progress` in the store and window-config publication interfaces. Both definitions and their caller are present in the current checkout. Their modification times fall during the failed build, after the dependency units had compiled. A fresh all-profile rerun keeps its full output and exit status under the ticket.

A clean checkout has no activated browser artifacts. The React `serve-*` target depends on the projected session and expects prepared components. The companion `dev-*` target runs preparation and activation first. The shared launch adapter now issues `dev-*` for both renderers and profiles. The two launcher regressions failed against the warm target and passed with the cold target. The full local-hub suite then passed 28 tests and the time-travel boot suite passed 31 using a separately staged production catalog with 158 deployed playgrounds. The XLSX picker and binary I/O routing checks also passed again. The standalone browser watcher now monitors the cold target. The full S cold host and native suite remain active.

The current full natural-file contract passed 280 of 281 tests. All XLSX checks passed. The only failure is the IDML editor declaration missing from its own family component descriptor. The previous JSON sample failure cleared in the shared checkout.

The staged production catalog contains the S shell on React port 6070 and `stdio-xlsx` on port 6272, where it selects `s.stdio.xlsx@ecma-376/*#editor` from `stdio-office`. Both browser probes wait for their assigned managed process to answer before starting. The documented Gherkin differential target is also queued.

```sh
SEMIO_LANGUAGE=en repo run @semio-tech/repo-test-domain:test-parity -- exhaustive --owner '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx' --case mutate-xlsx-ecma-376
```

The native tests use Calamine to read subject-encoded workbooks. The separate Gherkin case also uses rust_xlsxwriter to construct reference workbook bytes.

The named Rust test target previously selected only the public crate. Its package router now also selects the base, Strict, Strict mutation and Transitional crates, so routine XLSX testing reaches the editor and codec laws. The ticket test invokes the actual package router with a recording provider and compares its package selection against the owned Cargo manifests using Bun's TOML reader. It passed with all five expected workspace packages.

The original Hub I/O matrix ended at owner registry admission with `BUILD_OWNER_REGISTRY_INCONSISTENT` for the BREP axial-face preview crate. That owner appeared in the shared checkout immediately after the refusal. The repository's owner check found one more missing BREP owner, `semio-framework-3d-brep-spline-trim`. Scoped reconciliation added that owner to `📋️build-owners.json` and the scoped check passed. The Hub matrix is scheduled to retry when the full S host answers.

The full owner-registry audit found 15 further manifests with missing owner rows in flow, FEM, layout, note and Hub norm. The repository's scoped reconciliation added the 15 rows without accompanying removals, renames, relocations or package changes. The full owner-registry check then passed at 4,805 owners with zero findings.

The eighth native run passed source admission and compiled through the OS kernel. It stopped in `semio-plugin-instance-dispatch` with E0026 before XLSX test execution. Its compiled `Emit` type did not have `artifact_mutation_pages`. The current declaration at 07:09:02 UTC and its dispatch caller at 07:11:43 UTC both specify the field. Both files changed during this build, which ended at 07:12:04 UTC. The next all-profile run keeps separate durable output.

The ninth native run stopped before XLSX at window-config E0061, where the owner registry passed four arguments to an `advance` implementation requiring five. The current caller and trait both accept and forward the retained clone grant. The fresh native run logs to `xlsx-native-current.log`; admission first waited on host memory.

The standalone cold React host stopped at graph publication. It reported 50,262,248 graph bytes and 856,037,661 authority bytes against the configured 536,870,912-byte limit. The parity worker exited with `Error: closed`. The full S host and browser watchers also ended before any app served. The browser cache contains shared vendor and worker files but lacks an activated standalone runtime and Office module. Further work addresses indexed storage of graph source authority so browser activation can proceed within the current bound.

The standalone browser probe now drives a fresh Sheet1, commits A1, checks the XLSX download with fflate and Chromium DOMParser, and verifies the undo and redo exports. Chromium passed the further checks that the acceptance workbook and the two-sheet native demo expose the intended workbook names and first-cell content. This verifies the oracle. Live browser acceptance remains open. The focused natural-file contract rerun passed all 22 tests and 212 assertions.

A valid-workbook audit identified another import issue. Worksheet position attributes may be omitted. The new portable fixture has implicit and explicit rows and cells, a skipped column, and an intervening XML comment. The existing workbook projection changes resolve positions in document order. The mutation path is being updated so edits and their inverses preserve those positions and XML nodes. [Office's omitted cell reference behavior](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oi29500/2fd4e47f-0965-4c60-95bd-cff980b6c325) and [row bounds](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oi29500/5cf07e31-6868-4d0c-b0e8-6de69c57569f) describe the supported positions.

The subsequent native run passed source admission for 680 units and entered Cargo. The new language-neutral implicit-coordinate fixture drives the actual base editor command, then checks exported values with Calamine, verifies exact undo and redo, and exercises text and binary mutation codecs. The updated history input schema audit passed. The package and route TypeScript runs passed 14 tests and 136 assertions. The full owner-registry recheck again reported 4,805 owners and zero findings.

A fresh standalone cold run is waiting on host memory and will exercise the compact authority publisher. Its watcher will perform the live download checks once port 6272 answers. The previous publisher's failure and the withdrawn watcher status are retained with `publication-v3-` prefixes in the ticket output directory.

The new graph receipt stores repeated nested observations by index. Its focused suites passed 40 tests across encoding, stale-fact recovery, graph sealing, and fleet exchange. A synthetic 1,118,991,401-byte expanded receipt encoded to 16,269,071 bytes and preserved shared observation identity on decode. The 512 MiB publication bound is unchanged. [Graph authority verification](graph-authority.md)

The standalone and full S managed React hosts are active. The full S host has English and German natural-file watchers and the Hub I/O matrix watcher. The scoped differential suite was relaunched through the same current graph publisher.

The repository's default explicit promotion policy kept the managed retry on the older sealed implementation. The owned requests were restarted using the documented `SEMIO_REPO_SEAL=live` mode. The older service and its worker stayed live for other requests. The new service descriptor and worker started after the authority fix, confirming that the current standalone, full S and differential runs can exercise the new implementation. Earlier retries remain under `sealed-retry-` in the ticket's output directory.

undefinedA read-only registry audit confirmed that the authored profile and dialect statuses appear as public capability metadata. Executable codec, composer, validator, editor and viewer registrations are independent of those declarations. The general ECMA conformance ledger remains at its evidence-backed status. The current ledger contract requires normative-source, validator and fixture evidence for an implemented claim, and the available profile states do not describe a narrower implementation scope. No general conformance claim was made.

The local selected graph store and fleet were checked for reusable exact XLSX graph scopes. No local or fleet snapshot was available for reuse, and the current selected worker had already attempted the fleet read before starting inventory. The two active browser requests and parity request therefore continue through the same managed graph service. The native retry entered a fresh pooled Cargo lane for the same crate set and continues compiling.

undefinedThe requested Sol and Terra agents worked on the implementation and read-only audit. This continuation ran on the available Sol main model.

The path-length audit covered 59 changed or new files in the XLSX, acceptance, shared serialization, and ticket paths, excluding transient build output. The longest absolute path was 178 characters and 194 UTF-16 code units in this checkout.
