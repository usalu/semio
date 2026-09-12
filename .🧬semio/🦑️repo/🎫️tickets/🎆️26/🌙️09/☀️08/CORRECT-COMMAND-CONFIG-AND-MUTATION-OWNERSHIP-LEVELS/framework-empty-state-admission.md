# Framework Empty State Admission

FEM empty app state now reuses framework NoConfig and NoPresence. Source inspection found that the shared NoConfig, NoPresence and NoTransient FromValue derives do not reject unknown fields, and all three Pack decoders accept arbitrary bytes. NoDraft aliases NoConfig. This shared admission gap must be corrected at the framework owner so every no-state app receives the same behavior.

A test-owned schema and neutral fixture cover seven JSON cases, three text cases and three Pack cases. The independent TypeScript/Ajv oracle uses the framework declared-record parser for empty JSON records. The first invocation passed the oracle but strict TypeScript rejected the untyped Ajv narrowing; the fixture validation now supplies its explicit test type. The second registered neutral invocation is running. The native test checks all four framework absence types against the same 13 admission vectors each and is mounted under the plugin test owner. Native red and production hardening have not run yet.

Commands are registered as framework-empty-state-contract and framework-empty-state-contract-native in root and ticket Nx projects, with launch orders 311.198 and 311.199 in both launch configurations. Evidence is retained under ticket generated output. No claim of corrected shared runtime admission is made until the native red/green cycle completes.


## Native Red And Shared Correction

The registered neutral second run passes the independent Ajv comparison and strict TypeScript. Native red runs one compiled law and fails exactly on NoConfig accepting the foreign camera JSON field (0 passed, 1 failed, 682 filtered). The shared framework owner now rejects unknown fields for NoConfig, NoPresence and NoTransient and rejects nonempty Pack bytes. NoDraft inherits the same contract. Native green is running on the configured 2 MiB stack. No plugin-specific empty replacement types were restored.

## Native Green

framework-empty-state-native-green.log records the independent oracle and strict TypeScript pass, then the native law passes all 52 checks across NoConfig/NoDraft/NoPresence/NoTransient (1 passed, 0 failed, 682 filtered; test runtime 0.00s). The explicit configured test stack is 2 MiB; total registered Nx duration is 1m37s. Runtime DEBUG confirms all four types accept only empty state.

Production changes are limited to the three shared absence-type derive attributes and Pack decoder admission in the plugin SDK. Test corpus, schema and both implementations live under its empty-state test owner; command registration is in root/ticket scripts/projects and both launch files.
