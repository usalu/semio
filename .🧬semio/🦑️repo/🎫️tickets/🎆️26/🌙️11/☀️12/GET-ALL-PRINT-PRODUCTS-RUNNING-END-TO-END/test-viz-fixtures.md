# test-viz-fixtures

Root cause: the `test-viz-fixtures` target in print's `📋️project.json` had no `dependsOn`, but `test viz fixtures` renders gallery PDFs with tectonic. The tectonic toolchain (`deps-tectonic`) was therefore not guaranteed to be published, so reading `.cache/b/t/m/print/tectonic/<triple>/.semio-artifact.json` failed with ENOENT. Sibling targets (test-thesis-render, test-native-grammar) declare `fonts`, `deps-tectonic`, `deps-tex`, `generate`.

Fix: added `"dependsOn": ["fonts","deps-tectonic","deps-tex","generate"]` to `test-viz-fixtures` in `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📋️project.json`. No graph/walk files touched.

Verification: run r7 (`🗑️generated/r7-viz.log`) passed the original ENOENT; deps-tectonic and deps-tex now publish and the test itself runs (long, sequential tectonic renders). See final status below if appended.

## Final status
Run r7 ended with exit 1 after ~6140 s because it received SIGINT ("Cancelled (SIGINT); leases released"), not because of a test failure. The ENOENT is gone and the test body ran without any logged error, but it did not finish. The target regenerates the full gallery evidence by rendering every variant sequentially with tectonic (16 s each), which needs well over 100 minutes on this host. run.lock is released.
