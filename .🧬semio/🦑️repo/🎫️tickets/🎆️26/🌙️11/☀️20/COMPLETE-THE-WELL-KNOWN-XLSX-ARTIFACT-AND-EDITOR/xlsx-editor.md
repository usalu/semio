# XLSX editor

New ECMA-376 workbooks open with an empty worksheet named `Sheet1`. The worksheet is authored by the corresponding subset builder for Strict and Transitional. Those editors use their own dialect checks when opening and saving native files.

The canonical fixture test loads physical XLSX bytes through the editor's native media port, renders the resulting workbook, commits a change to A1 using the grid's action binding, and saves the workbook. It checks the saved workbook with the shared decoder, independent ZIP and XML readers, and Calamine for the fixtures it supports. The subset tests also insert a formula in a newly created workbook and reopen the saved file.

The editor files pass `git diff --check`. The first Rust run reached XLSX test compilation and exposed a missing test fixture feature and incorrect API references. The second run reported only a crate alias error in the new demo test. Those issues were corrected.

The native suites use `repo cargo --store=shared --configuration=prototype --parallelism=1 -- test -p semio-s-artifact-stdio-xlsx-base -p semio-s-artifact-stdio-xlsx-strict -p semio-s-artifact-stdio-xlsx-transitional --lib` from the XLSX workspace.

One run hit source skew in `semio-plugin-retained-command` during a concurrent change. A retry compiled all three suites. The Base suite reported 24 passed and eight failed, stopping Cargo before it ran the other two. Blank workbook creation and Calamine, the native demo, and all three physical codec tests passed. Five direct grid tests failed because they had not registered the UI arena. The Base, Strict, and Transitional direct grid tests now use the framework's own fixture arena. Three history acceptance tests reported fixture, payload schema, and retirement failures.

The next run used `--no-fail-fast`. Base passed 26 of 32, Strict passed 18 of 21, and Transitional passed 24 of 28. Both subset editors saved and reopened a new workbook with the formula entered in its first cell. The input schema and conflicting history tests passed in all three subsets. The run identified missing exact tree retirement in direct grid tests, a missing canonical field ordinal in the insert-cell history seed, and a Transitional namespace inverse failure. The grid tests now retire their built trees explicitly before closing the arena.

The combined rerun added `-p semio-s-artifact-stdio-xlsx-strict-mutations`. It exited 1 at source admission. The build tool reported that the shared `semio-plugin-instance-app` crate exceeded its source boundary with 262,215 bytes against a 262,144-byte maximum. No XLSX tests ran in that attempt.

Two redundant mounted-instance docstrings were shortened without changing executable code. The repository source scanner then counted 17 files and 257,997 bytes under native macOS with the artifact test feature, both with and without the Rust test configuration. Cargo source admission passed on rerun.

That build stopped while compiling `semio-plugin-composition-publication`. Four E0599 errors reported that dependency artifacts lacked `preparation_refusal_progress` for store and window-config publications. The call sites and definitions were edited concurrently in the shared worktree and now agree. The single subsequent native rerun captures the full compiler output and exit status.

The recorded rerun exited 101 after a concurrent edit introduced `artifact_mutation_pages` in both the `Emit` declaration and its dispatch consumer. The earlier dependency artifact lacked the new field and the later consumer expected it. The current source agrees. The revised rerun retains its own compiler output and exit status.
