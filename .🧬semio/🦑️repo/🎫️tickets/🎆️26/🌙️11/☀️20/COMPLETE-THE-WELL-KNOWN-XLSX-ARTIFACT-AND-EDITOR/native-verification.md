# Native XLSX verification

The previous four-package native run ended with exit code 101 in `semio-plugin-window-config`. The live owner registry now takes and forwards `turn_ceiling: RetainedCloneGrant` to the store-owner trait, which resolves its four-argument call against the five-argument method. The earlier `artifact_mutation_pages` error also involved a declaration and its caller changing during compilation.

The current run uses the XLSX workspace and the repository's shared prototype Cargo store with one admitted core.

```sh
repo cargo --store=shared --configuration=prototype --parallelism=1 -- test -p semio-s-artifact-stdio-xlsx-base -p semio-s-artifact-stdio-xlsx-strict -p semio-s-artifact-stdio-xlsx-transitional -p semio-s-artifact-stdio-xlsx-strict-mutations --lib --no-fail-fast
```

Source admission passed for 680 units, 4,468 files and 54,134,721 bytes. The run records compiler diagnostics, test output and exit status under the ticket's generated-output folder.

The base library test target includes the implicit-coordinate regression through its native-document module. Calamine 0.36.1 uses rolling row and column positions when the XLSX source omits the corresponding attributes. Its position rules match the fixture's expected row and column offsets.

The current run compiled the three plugin packages where earlier attempts stopped. It reached XLSX then exited 101 because the Strict VML fixture did not import `ColdMutation`, which provides its borrowed `diff` and `inverse` methods. The fixture now imports that trait. The four-package retry has separate generated logs and status under the `xlsx-native-strict-imports` basename.

The retry acquired a different pooled Cargo lane. The lane pool uses the crate set to prefer an idle matching lane and assigns a random ID to a new lane. Both commands used the same target namespace, and the previous lane was absent when inspected. The public Cargo ingress exposes the shared store but has no option to pin its execution lane.

The compiler wrapper used a different seal on the retry. Comparing each source in both 260-file seals found one difference in the shared host-capacity sampler. It now derives memory pressure from physical RAM when the simulated host size differs. The seal digest names the wrapper, and the portable unit key lists the compiler toolchain, normalized arguments, source, externs and selected environment. No cache trace attributed specific hits or misses to this change.
