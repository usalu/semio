# Public Leaf Derive Independent Review

## Accepted Boundary

The public metadata-only `MutationLeaf` derive is accepted for genuine lower and OS-facade contracts. It consumes actual declaration source authority and the complete sibling descriptor, preserves generic parameters and where clauses, emits the lower by-value descriptor and six-field provenance, and tracks the two workspace markers, taxonomy, and descriptor as compiler dependencies. It does not implement mutation behavior or the mandatory aggregate/registry transaction.

## Executed Gates

The final registered derive suite passed 8 tests with 0 skipped and exit 0. Evidence: `🧪️public-leaf-derive-registered-final.log`, Nextest run `e225517c-591c-4c19-89a4-547369f28c92`, artifacts `🧪️public-leaf-derive-registered-final-artifacts/semio-nextest-XdYrDz`. Earlier failures for sibling test-helper visibility and the exact macro-export roster are retained, not relabeled as passing runs.

The coordinator's fresh workspace-scoped kernel library build completed in 1 minute 17 seconds with exit 0: `🧪️public-leaf-kernel-build.log`. It produced the real proc macro `libdsl_derive-f44e247812382dc4.dylib`, the kernel's unhashed rlib/rmeta pair, and its compatible protocol pair `6c1330d456d23eb4`. The kernel's pair is intentionally unhashed because of its crate output configuration; absence from a hashed-filename search was not evidence that the build produced no artifact.

The executor's full real-client run `🧪️public-leaf-derive/🧫️run-KM4oe4` passed all 12 primary compiler cases and the genuine OS facade case. Valid clients compare all fourteen descriptor fields and all six provenance fields using the actual lower types, including a non-static generic payload. Invalid source, owner, descriptor, contract syntax, and union cases produced the required diagnostics. Same-workspace leaves agree on the token; an identical relative layout in another physical workspace differs and matches the independent SHA-256 oracle.

The root separately recompiled and executed the actual direct client using the real macro and paired protocol artifacts. `🧪️public-leaf-root-review/🧪️root.log` and run `🧫️run-26DNPu` record runtime agreement with an independently computed Node SHA-256 token, all four metadata paths present in rustc dep-info, and unchanged hashes for the source and compiler artifacts during replay. Compiler argv, output, runtime output, dep-info, and result JSON are retained.

## Review Corrections

The initial proposed harness used handwritten lookalike provider types. Those were replaced with genuine lower and kernel artifacts. Further review required a real borrowed generic parameter, exact complete metadata comparisons, nonzero/signal/error rejection, explicit rustc crate names for emoji source filenames, coherent rlib/rmeta pairs, schema-driven expected outcomes, draft-2020 schema validation, and retained per-case evidence. The facade case is an executed real-crate case, not a local module alias.

This does not prove aggregate rejection of wrong-root/foreign/manual metadata, policy package resolution, canonical production filename conversion, or full mutation/runtime success. FND-AGGREGATE-AUTHORITY-14 now extends the shared source resolver before the complete mandatory consumer transaction.
