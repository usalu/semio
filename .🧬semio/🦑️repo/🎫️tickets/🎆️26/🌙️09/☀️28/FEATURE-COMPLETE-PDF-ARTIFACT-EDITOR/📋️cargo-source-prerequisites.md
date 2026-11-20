# PDF Cargo source prerequisites

The PDF snapshot test crate pulls in the icon contract and UI locale crates through workspace path dependencies. Rust source admission originally stopped at `icon_name`, then at `axes`. Both modules mount ignored outputs that had yet to be published on this host.

The generator contracts now name their immediate Rust consumers. The assets producer serves the icon contract, canvas icon codec and canvas runtime assets. The UI axes producer serves the locale crate and the umbrella UI crate. The existing native dependency traversal selects these producers transitively when the PDF snapshot test is planned.

The generated source topology regression resolves each mounted file independently using Bun's TOML parser and compares its producer with the registered native consumer. It failed on the icon contract before the fix and passed all three cases afterward. A direct call to the build planner for the PDF snapshot test selected assets, UI axes, framework schema, UI styling and the mutation schema index.

Direct Cargo verification uses the existing generators to publish the ignored inputs on this host before invoking the Cargo ingress. It bypasses prerequisite scheduling in the outer task graph.

## Rust verification

The PDF snapshot-value test command admitted all 704 compilation units after generating the missing files. Rust compilation subsequently reached the shared stdio contract test harness and reported 31 errors from retired signatures. The harness now uses funded retirement turns for both native directions, a typed reply slot for encoding, and the settled SQLite output slot for the wire assertions. Its registered-job fixture uses original construction admission, and the test package with the inline page-text tests is `semio-s-artifact-stdio-pdf-tests-2`.

The focused Cargo check passed for the shared harness after the concurrent tool-machine and value crate changes settled. The combined editor test initially hit the 32-file budget because the snapshot-value crate also read Page fixtures. The Page tests now have a dedicated build owner and remain part of the aggregate PDF test script. The repository source reader measured 29 inputs and 2571 active lines for snapshot value, 16 inputs and 1227 active lines for Page, and 29 inputs and 3931 active lines for PDF/VT. The combined test run was resubmitted with the dedicated Page package.

## Concurrent Cargo changes

A registered Cargo run records manifests and reads the bounded source closure before compiling. The compiler still uses the live workspace. A provider and its client can therefore change together after admission, causing a later client compilation to see the new call beside the old provider artifact. The launcher rejects a manifest whose byte content changed after admission.

The shared store checks matching local and fleet units and restores complete outputs before compiling. The isolated store changes the compiler output location. Remote Rustc offload can stage an individual eligible unit, and it falls back locally if the remote fleet declines. The Bazel Cargo action delegates to the same live Cargo ingress with sandbox and remote execution disabled. The hermetic Cargo task-view helper is exercised by synthetic fixture tests and has no registered project or CLI entry point. These mechanisms leave the registered Cargo build tied to the live workspace. Verification retries after a detected concurrent edit use the current manifest graph and any matching completed units.

## Xref test admission

The conformance-support test target reached 33 inputs including embedded PDF fixtures. The existing graph-projection owner now runs the indirect stream-length fixture, which exercises the cross-reference reader against a PDF containing an embedded endstream word. The repository source reader measured 32 inputs and 3182 active lines for conformance support, 26 inputs and 3761 active lines for graph projection, and 17 inputs and 1708 active lines for the xref-stream package.

The three-package editor run compiled the plugin, stdio test harness, PDF lexer, filters, inferences, xref-stream and graph projection. PDF lift then picked up a concurrent source update requiring the xref-stream manifest edge, which was absent from that run’s admitted graph. The current manifest contains the edge. A fresh combined Cargo run covers the three editor packages, conformance support, graph projection, and xref-stream.

## Production Page owner

The PDF/VT test target included the entire Page editor and reached 4011 active lines as editor coverage expanded. Page now has a production Cargo owner. PDF/VT exposes Page through the same public module as before, and the aggregate package test command includes the Page owner’s unit tests. The native resize and first-draft PDF oracle regressions run in the existing Page editor test package alongside the other PDF round trips. The repository source reader measured 20 inputs and 3539 active lines for the production Page unit target, 18 and 1421 for Page integration tests, and 10 and 427 for PDF/VT.

The next combined run compiled the updated PDF lift and the PDF/VT Page code. The testkit’s generic fixture helper used the retained mutation trait for a borrowed reference operation. It now uses the ColdMutation trait and the same cold diff applier for which the helper was designed. A fresh registered Cargo run selected the two Page packages, PDF/VT, snapshot-value, and the xref targets with the editor, reference storage, and writer filters.

The new Page owner’s 533 reachable path manifests contain no reverse edge to PDF/VT. The PDF workspace lock resolves the new owner and the existing tests. The refreshed seven-package run admitted all 717 units including the Page owner, then waited for a compiler CPU grant held by two builds.

The compiler grant opened after about 26 minutes. The run passed the paged-genesis VCS, OS kernel, mesh engine, and plugin tool-machine crates. It stopped at two stale private field accesses in instance action ingress. The current source already replaces them with the registry’s public empty backing demand and reuses that demand for close. The same seven-package run was resubmitted after confirming the source fix.

The warm build passed instance stores, the extracted production Page owner, and the cold-reference fixture helper. It then found that schema-4 reexported the testkit from a library build whenever the feature was enabled, while declaring the provider exclusively as a development dependency. The schema feature now activates the existing testkit package as an optional normal dependency. The analogous PDF/A schema mirror uses schema-4’s existing glob reexport in feature builds and keeps its explicit development dependency only for its own unit tests. Schema-4’s dependency source closure is 21 files and 2231 active lines with the feature; selecting schema-4’s own unit tests expands it to 42 files and 3586 lines. The focused editor run does not select that larger unit target.

A later build stopped when the shared window-config client called preparation_refusal_progress after its provider had already compiled. The method now exists on ArtifactStoreBatchPublication; no duplicate source change was required. The next admission overlapped publication of the new content program module and was retried once its author confirmed the child source had landed. The prior native compiler lane disappeared between attempts, and the new invocation repopulates artifacts through the repository cache.

After rebuilding the cache view, the native run passed the current original-receiving refusal API and stopped at the new paged mutation emission type after the earlier store provider had compiled. The next admission compiled paged-one-item, window-config and emission. Typed operations then called for_gesture_iter while the build still held the older one-item artifact. The provider method and client call now agree, and a fresh admission includes both.
