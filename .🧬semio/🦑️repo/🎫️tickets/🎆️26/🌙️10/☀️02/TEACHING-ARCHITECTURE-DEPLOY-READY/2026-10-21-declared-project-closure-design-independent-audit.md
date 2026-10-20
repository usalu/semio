# Declared Project Closure Design: Independent Audit

Static read-only review of the design and current script-inputs lines 197–218 and 313–319. No Source edits, test execution or native invocation.

The proposed separation is sound: retain exact command compiled provenance on the newly joined array, and collect compiler-program producer dependencies locally from actual produced rows. Current code demonstrably loses the WeakMap annotation at line 211, while line 318 can reenter the strict guard at 217. This is a reachable defect independent of attribution of the earlier settlement event. A missing-evidence guard must remain; copying all mixed compiler inputs into compiled command membership is unsafe.

Concrete additional acceptance cases:

- Exercise a generated-only compiler project: every project.files row is produced, names remains empty, yet local generated dependencies survive. Exercise scripts.generatedImports initially empty, populated only inside the project join; branch guards must not drop those local dependencies.
- Exercise multiple declared tsconfigs with different producers and duplicated output observations. Dependencies must come from all actual project.files encounters, not only the first config or old config-indexed cache rows. Include the same config shared by two commands and stale/polluted rows already indexed under that config.
- Mutate the original compiled Set after the join and verify the joined Set is independent. Also verify moduleSets is not prepopulated from cache-global modules or copied under the wrong workspace root. Equal fresh arrays without owned metadata must still refuse.
- Preserve the real current gate: targetScriptSources only joins a project when a command closure exists. A project declaration on a target with no entries currently does not invoke declaredProjectClosure; this repair should explicitly test/preserve that behavior unless Root deliberately expands scope. An empty owned compiled Set is different from an absent closure.
- Keep the produced-file branch ahead of physical Source reading. A generated config/data/declaration encountered by the compiler must add its producer without an ordinary Source read; a nonproduced compiler file must still pass physical reads/confinement. Add a negative physical-read failure case rather than only membership assertions.

Real AST tests should invoke targetScriptSources through its actual declared-project branch and execute extracted real helpers, including the returned array identity binding. SQLite should independently derive separate command membership and compiler producer union from fixture facts. A stubbed compiler summary is adequate for provenance joining only; the separate real compiler test must establish mixed config/extends/JSON/declaration facts and multiple declared configs. Ensure selected Bun titles collect both tests later; prepared vectors provide no pass evidence.

The design correctly avoids attributing the native final self-check failure to this metadata boundary. No unsafe semantics are required by the proposal, subject to the above cases and unchanged strict admission/self-producer checks.
