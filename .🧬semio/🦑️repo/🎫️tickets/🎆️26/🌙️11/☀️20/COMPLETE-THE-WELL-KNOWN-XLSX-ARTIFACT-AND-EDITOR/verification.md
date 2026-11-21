# XLSX artifact verification

## Completion checks

| Requirement | Evidence |
| --- | --- |
| Public kind and file recognition | The published Office descriptor registers the editor and viewer for all three `s.stdio.xlsx` profiles. The production file resolver and all 282 natural-file contract tests passed with 985 assertions. |
| XLSX TypeScript package and history input | The package and route tests passed 14 cases with 136 assertions. The history input audit passed. |
| Native workbook creation, import, export and history | The latest four-package run passed 91 of 97, including the generic tool proof and file codecs. The isolated history tests exposed a preview cancellation defect and a retained reader source defect. A focused Base replay is running after both fixes. |
| Independent mutation differential | The genuine Rust subject and independent oracle each passed 18 scenarios. The production comparator and pair coordinator found all 18 equivalent. The standard Nx target has no successful terminal result. See [parity verification](xlsx-parity-host-profile.md). |
| Rendered editor and durable file round trip | The standalone session, catalogs, Office WASIp2 build and semantic descriptor publication passed. Chromium independently loaded the transpiled Office guest and returned the native descriptor bytes. Managed browser publication is pending. The full S English and German journeys, Hub I/O matrix and standalone editor journey await their respective hosts. |
| Graph authority repair | The compact receipt passed 40 focused tests with the existing size limit. The full observation and repair replay passed 84 tests. The managed XLSX host is exercising the live implementation. |

## Observed results

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

A read-only registry audit confirmed that the authored profile and dialect statuses appear as public capability metadata. Executable codec, composer, validator, editor and viewer registrations are independent of those declarations. The general ECMA conformance ledger remains at its evidence-backed status. The current ledger contract requires normative-source, validator and fixture evidence for an implemented claim, and the available profile states do not describe a narrower implementation scope. No general conformance claim was made.

The local selected graph store and fleet were checked for reusable exact XLSX graph scopes. No local or fleet snapshot was available for reuse, and the current selected worker had already attempted the fleet read before starting inventory. The two active browser requests and parity request therefore continue through the same managed graph service. The native retry entered a fresh pooled Cargo lane for the same crate set and continues compiling.

The Strict test import retry exited 101 before XLSX because `semio-os-dsl` used `step_work_demand` against an older compiled `OpBinaryReceivingCursor`. The current trait and implementation match. The two source files were updated at 16:33:06 UTC, within a millisecond of each other, before Cargo reported the inconsistent compile at 16:34:57 UTC. The exact four-package replay is active under `xlsx-native-step-work`. The live graph worker continued inventory and memo verification alongside it.

The requested Sol and Terra agents worked on the implementation and read-only audit. This continuation ran on the available Sol main model.

The path-length audit covered 59 changed or new files in the XLSX, acceptance, shared serialization, and ticket paths, excluding transient build output. The longest absolute path was 178 characters and 194 UTF-16 code units in this checkout.

The native document validator, byte sniffing, and independent archive oracle match the registered XLSX workbook media type by its ASCII case-insensitive type and subtype. The shared six-case fixture covers the standard declaration, capitalization, the registered UTF-8 charset parameter, generic XML, a substring lookalike, and the Excel macro-enabled workbook type. The uppercase fixture failed before the correction. The XLSX-filtered acceptance contract then passed 23 tests and 220 assertions. The native regression is included in the active four-package replay. [IANA Workbook part registration](https://www.iana.org/assignments/media-types/application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main%2Bxml) [RFC 6838 §4.2](https://www.rfc-editor.org/rfc/rfc6838.html#section-4.2)

The native Cargo storage controls isolate build products. The supported command still compiles from the shared live checkout, so an isolated storage lane cannot provide a stable copy of concurrently modified source files. The active replay uses the supported warm pool and continues past shared prerequisites.

The wider offline acceptance replay passed 281 of 282 tests. IDML was the sole failure because the Office component omitted its editor and viewer even though the IDML composition declares both factories. The matching component wiring is being verified separately.

A direct invocation of the test domain's parity script was rejected before test selection because it did not carry the original native capabilities required to open the managed diagnostics store. The original admitted parity command remains queued through the repository graph.

The graph worker’s live diagnostic placed the longest observed pause in the workspace owner's command-source discovery. Owner memo checks also repeated shared top-level probes across multiple owners. The admission pass now reuses those results within one pass, and the separate final seal still verifies the captured facts independently. The dedicated regression and both refreshed fixtures passed. The already-running host worker continues with its original code. [Graph authority verification](graph-authority.md)

A managed current-graph query resolved the dev project from a single-owner inference. The parity command has a second admitted route using the current graph and excluding upstream task execution. That route advanced into its own selected inventory. The original cold product runs are still active.

The native run executed all four XLSX packages. Base passed 32 of 34, Strict 19 of 21, Strict mutations all 12, and Transitional 26 of 28. Both of the remaining cases failed in each editor profile. The natural XLSX codec, new blank and filled workbooks, implicit-coordinate edit and exact undo, the workbook media-type check, and the Strict VML regression passed.

The regular history report enters the replay path without an installed semantic issuer. All three editors now opt into the repository's shared cold replay owner factory, which admits that issuer alongside the original snapshot and mutation retirement issuers.

The conflict case sends a genuine document archive through the mounted load path. Its parent history decoded, then the forwarded operation failed at `DecodeForward`. All three editors use the base content mutation, which wrote a handcrafted frame while its bounded receiver expected a complete Pack value. The retirement refusal was reported after the wire failure. The binary owner is aligning the encoder and receiver before the next native replay.

The history operation encoder now uses the same complete Pack value as the original receiver. The XLSX-filtered acceptance contract passed again with 220 assertions after the wire change. The dedicated model-laws invocation is compiling its protocol walk and Calamine witness. [Persisted mutation wire](xlsx-mutation-wire.md)

A fresh native source admission caught a 33-file test closure against the existing 32-file crate bound. The coordinate and media-type cases now share the native-document scenario fixture. All seven coordinate edits and six media declarations were preserved. The next source admission passed all 680 units and the native test recompilation started.

The current-graph parity alternative sealed its selected graph after one automatic retry for changed graph implementation inputs. Its next phase is waiting for the required host memory grant. The standalone app graph is still in selected inventory and continues observing new source facts.

The separate model-laws command repeated the same shared dependencies as the editor run while the parity task was waiting at the macOS memory gate. It was cancelled normally to give the editor and parity work priority. The root protocol-walk and Calamine re-run remain pending.

The first current-graph parity invocation exited successfully but reported `cases=0 executed=0 parity=0/0`. The discovery API returns the complete directory name `🔀️mutate-xlsx-ecma-376` as the case selector. The corrected managed invocation selects the exact Base owner with `--owner-root` and uses that complete name. The earlier queued request with the same empty selector was cancelled. No differential result is counted yet.

The parity, oracle, and subject runners now record a failure when an explicit `--case` is missing its value or matches no authored case. The exact directory name appears in the diagnostic. Broad empty discovery filters retain their normal behavior. Six neutral selector cases passed, including the previously successful zero-case invocation, and an independent Ajv check agreed on every result.

The next native attempt compiled past DSL and stopped in a shared mesh dependency after a concurrent API update. Its already-built Pack JSON dependency lacked `member_at` and `swap_members` while the live mesh source called those methods. The current Pack JSON source declares both. The active replay is tracked under `xlsx-native-history-current`. The Office WASIp2 replay cleared its prior DSL source-skew failure and continues through its dependency closure.

The corrected case invocation selected a different argument vector but the outer incremental gate returned one up-to-date task and never invoked the runner. The earlier zero-case result caused a false reuse. The exact case is now running with the supported `--skip-cache` flag. The build fingerprint is being checked separately for the missing forwarded arguments.

The full native replay cleared the shared mesh dependency and again ran 95 tests. The 89 passes include physical workbook I/O and editing in Base, Strict and Transitional. The history report reached the authored conflict-path issuer after the catalog change. The conflict archive reached terminal cleanup without repeating the forward SPR checksum error. Its close stalled with a funded 1,872-byte child capacity quote. The mounted XLSX mutation now lends its authored target path, with a native regression covering all 13 mutation kinds. The next replay also records which dedicated archive owner remains at close, so the initial load failure can be distinguished from cleanup. [Native verification](native-verification.md)

The focused archive status and retirement checks passed. In the broader runtime source-contract file, 13 unrelated assertions still expect older worker, publication, envelope, or actor shapes. That run passed 19 checks overall, with one skip. The native archive replay remains the behavioral witness for load and close.

The uncached parity runner selected one case and attempted both configured Rust phases. Each phase stopped before execution because its generated host used raw Cargo, and the authentic compiler guard refused the version probe. Rust host preparation now uses the shared repository Cargo launcher. It selects the generated package and binary explicitly, and runs the host inside the retained compiler result callback with the separate scenario budget. The updated launcher-routing regression failed before the change and passed with the managed route. The exact XLSX replay is queued under `xlsx-parity-managed-cargo`.

The conflict-path native regression passed all 13 mutation classes. The Base browser-independent suite passed 33 of 35. Its archive reload decoded the mutation history and produced a candidate envelope, then refused it because its reusable mutation constructor still held the original Source binding. A diagnostic at the failed handoff identified `CloseEnvelopeOwners` and `mutation_admission`. Hydration now closes that constructor in history retirement before an Envelope or Store handoff. Archive cancellation delegates still-retained admission control through the original close path. The constructor now closes one metadata slot per paid turn, and its native fixture exercises an exact one-item loan without bypassing retained failure transfer. The remaining regular-history failure is being traced separately.

The direct browser session build completed and published `playground-session:stdio-xlsx`. Its canonical receipt enumerates three present outputs, including the session and both generated catalogs. Live graph inference completed two admitted owner jobs while retaining ten project identities. The independent final seal covered the graph and 25,778,837-byte indexed authority. The Office browser component remains a separate build prerequisite.

The focused `semio-framework-job` native replay passed all eight construction tests. It includes retiring completed and rejected constructor control under exact one-item and per-slot copy limits after transferring any original failure. The next four-package XLSX replay has the hydration fix and a temporary diagnostic for the independent regular-history invariant.

The broker socket replay passed four tests and 55 assertions on the current RSS contract. The first managed parity report contains 18 passing oracle results and no subject results, so its parity total is 0/0. The original subject compile, the native XLSX replay and the WASIp2 compile each stopped when the older broker rejected the newer packet. The exact commands resumed with the current implementation and existing stores. [Compiler broker audit](compiler-broker-audit.md) [Parity host evidence](xlsx-parity-host-profile.md)

The live Office retry passed the RSS boundary and reached a subsequent source skew after `OriginalReturnEscrowTable::supports_core_header_extent` appeared in the caller and its defining module during compilation. The older payload library was built at 23:48 UTC and both live files changed at 23:53 UTC. The coordinated Office replay restarted after both changes. The latest retirement and test paths were also checked for length. The 39 checked paths use at most 155 UTF-16 units.

The exact Office Cargo build linked a 257,230,014-byte WASIp2 component and recorded a SHA-256 for the retained bytes. The managed emitter is describing that component. A separate portable-backend command is producing the Office browser artifact. The full S graph initially stopped on the missing generated Puzzle WASM module. Its project now declares both public compile outputs, and the native graph index passed all three focused tests with no generated files present. [Office verification](office-idml-descriptor.md) [Puzzle WASM graph](puzzle-wasm-graph.md)

The current Nx loader accepted the neutral manifest and compiler output size laws, including the shared 32 MiB bound. The focused test passed 342 assertions with Ajv and WebCrypto checks. The earlier authorizer staging failures predate the current streamed reader. The browser publisher now includes its required native-transpile build.

### Current managed replay

The Office WASIp2 component linked successfully and its captured compiler artifact is 257,230,014 bytes. The exact descriptor command sealed its graph, restored the required dependency generations, and restarted automatically under the repository's fresh-process admission rule (exit 76). The new instance kept its resource grant. The scoped browser publisher and restarted descriptor command are waiting while the same graph worker checks the full S source inventory. The offline Puzzle regression passed with the browser package outputs absent. The retained parity subject passed the earlier kernel JSON compile failure and has advanced to plug-in registration. Neither Cargo replay has produced a terminal test result.

### Replay after the first full native verdict

The latest native suite reached all four packages and passed 91 of 97 tests. The next focused Base run compiled the updated Job Context and Store replay initializer, then stopped before its tests because the compiled ArtifactApp declaration predated the live picker method. The current trait and both implementation surfaces agree. The managed Base retry records up to four distinct candidate refusal paths, including the authored row edit, and records the archive maintenance stage for any remaining failure.

The retained parity adapter replay could not use a restored third-party Rust metadata artifact. The next run uses the repository's isolated target store and shared registry. The same invocation automatically refreshed its generated lock after concurrent Media manifest changes and restarted. Its previous terminal 17-of-18 comparison and independent oracle are preserved while the corrected adapter awaits its verdict.

The full S plan finished its long development, Puzzle and S composition project checks. The shared graph continues its remaining owner and memo verification and still holds the Office component publisher and exact descriptor emitter behind the same source authority.
