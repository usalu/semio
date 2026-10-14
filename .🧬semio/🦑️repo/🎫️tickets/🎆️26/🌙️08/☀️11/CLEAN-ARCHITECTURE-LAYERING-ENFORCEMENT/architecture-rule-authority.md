# Architecture Rule Authority

The current TypeScript/JavaScript graph verifier checks the names in `summary.ruleSetUsed.forbidden`, but does not check selectors or severity. A clean graph can therefore admit a same-name rule whose source, target, exemption, severity, or scope differs from the authored policy. Deletion independence requires the graph receipt to retain the exact policy used to enforce owner direction.

The existing dependency-direction schema and portable fixture will declare graph rule authority cases first. An independent dependency-cruiser run provides real normalized rule receipts; Ajv validates the authored strict contract. The first-party checker will accept parser normalization of path arrays into alternations and the default module scope while rejecting semantic policy changes and selectors its evaluator cannot enforce. Rule order and explanatory comments do not change authority.

The change uses the existing `@semio-tech/repo-lib:test-dependency-direction` Nx route and launch registration. No runtime library or script is added. All generated test output is kept in this ticket's `🗑️generated` folder and removed after verification. Existing root, product, and repository AGENTS.md instructions were read; no Git mutation or AGENTS.md edit is involved.

Verification evidence will be appended after the failing test and implementation runs.

The initial normal Nx invocation stalled after a daemon termination during graph preparation. It was cancelled; a daemon-free run also waited in shared graph preparation and was cancelled. Verification now uses the same registered Nx target with a ticket-owned `NX_WORKSPACE_DATA_DIRECTORY` and `NX_CACHE_DIRECTORY`, plus `NX_DAEMON=false`, to isolate its graph state from other simultaneous developers. These preparation failures are not test results.
