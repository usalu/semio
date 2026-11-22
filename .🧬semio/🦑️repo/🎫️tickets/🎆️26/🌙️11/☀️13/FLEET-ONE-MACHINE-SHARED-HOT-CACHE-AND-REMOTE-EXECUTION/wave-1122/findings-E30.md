# Findings E30
1. TMPDIR of every repo process is `<checkout>/.cache/scratch/...` by design (`🏃️process/🧹️scratch`). Tests that discover the workspace root upward, build cargo, snapshot sources (machine-local `.cache` names are skipped) or assert path lengths silently depend on this; they passed raw (TMPDIR in /var/folders) and fail under the suite runner. Suggest a law: tests that spawn cargo/bazel or call workspace discovery must take their base from `SEMIO_TEST_ARTIFACT_DIR`. About 100 `tmpdir()` uses remain under `🔧️tools/🏗️bazel` and 60 under `🌐️remote-execution`; only failing ones were changed.
2. Files skipped for recent edits by other lanes: sidecar priority test (E27), resident key tests, package-metadata-edits, fleet identity test, repair-rounds, native-scope (touched < 1 h before the run; E24/E28/E29 were not asked).
3. `🌐️remote-execution/🚀️offload` tests may reach a real cache/endpoint instead of the test's: see result-E30.md "Still failing".
4. `🗂️workspaces/🦀️cargo` has no suite-table route (see result-E30.md).
5. Startup-law note from E25 still applies: fleet-door row depends on machine speed.
