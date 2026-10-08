# Facet Route Workspace Cwd Repair

The first two actual Nx runs failed before source execution because their explicit repository-relative options.cwd resolved under the private isolated-verification workspace and produced a nonexistent duplicated ticket path. Retired that option on exactly the two new facet routes to use the established private task cwd, matching the working native routes. Full immediate project preimage preserved. No source verdict had been emitted by the failed invocations.
