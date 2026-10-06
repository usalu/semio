# PDF Owned Registered Source Type And Binding Validation

The existing 13-test Source roster, own14 Snapshot and Mutation facades, and base17 Snapshot facade are now covered by the registered `verify-snapshot-sqlite-source` task through the existing package script and both launch surfaces. The task uses the installed TypeScript compiler with strict type checking; no new dependency was added.

The first actual run exited 1 with four Bun SQLite signature errors: two parameterized fixture inserts and the two independent profile edit statements. The statements now use Bun's actual prepared-statement `.run` API with exactly the same SQL and bound values. All test assertions, independent raw profile comparisons, and edited owner readbacks remain unchanged.

The registered strict type task replay completed with exit 0; Nx reports 6.8 seconds. The separate actual Source assertion replay after the four prepared-binding changes completed with exit 0: 53/53 tests across 13 files, 1687 assertions, 6.64 seconds of Bun execution, 14.5 seconds on the Nx critical path. Compilation-only failures do not count as test assertions. Generated evidence remains under `🗑️generated`.
