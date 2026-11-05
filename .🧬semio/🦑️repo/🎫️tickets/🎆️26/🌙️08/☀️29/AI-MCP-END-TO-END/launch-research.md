# MCP Launch Research

The normal Cargo producer publishes in issued compiler storage while the TypeScript binary consumer still selects dist/build. Existing publication vectors expose this mismatch but omit a storage injection port. The stdio gateway also starts the real server before the client initialized notification, and can start a server after client EOF if staging later settles. The authored catalog root does not currently exist; zero-touch startup needs the catalog owning producer.

## Source-Owned Native Production

Direct repo <crate script> build has no root-issued Original and cannot legitimately publish on its own. The global build-target path remains blocked in repository source graph parsing. The approved route factors the normal crate BuildScript call into one MCP-owned buildMcpArtifactV1 function. Both the normal build task and canonical dev stdio binary stage invoke the same producer, fixed by schema to the MCP crate manifest, cargo package, executable and sources receipt. The dev route supplies its genuine signal-refined compiler control and exact scriptInvocationEnvironment. The neutral admitted Cargo artifact provider retains compiler/source/provenance/publication policy. No raw compiler command, cache-root inference, copied binary, forged envelope or new generic command is used.

The first live production attempt exposed options.environment defaulting to process.env inside the provider; actual root Original was in memory. Passing exact scriptInvocationEnvironment from the owned producer fixes that issue. A subsequent bare-readiness run is active under the same canonical dev route; build completion remains pending.

## Publication Reader and Diagnostic Lane Audit

The former launch preopened withAdmittedCompilerStoreV1 around the server lifetime. The shared native producer then opened its own compiler store. Ordinary withRepositoryCompilerStoreV1 does not reuse such an outer scope; reuse requires its explicit retained-store context. The actual attempt eventually passed preparation, so this was an avoidable nested reservation and repeated preparation risk, not a demonstrated permanent deadlock. The final design produces first and then holds the exact shared publication reader, through the existing neutral Cargo provider interface. The TypeScript test task uses that same reader.

Source inspection found native artifact producer writes to stdout for raw non-JSON compiler lines, selected compiler-artifact/build-script JSON and final staging notices. Its recipe/provenance diagnostic sink also defaulted to console.log. These writes are valid build CLI output but corrupt in-process MCP stdio. A neutral explicit diagnostic callback now covers all those sites, and the MCP shared producer supplies stderr through its gateway progress port. There is no global stdout or console patch. The authored diagnostic fixture includes plain text, compiler JSON and staging text, and the independent SQLite oracle reproduces the selected stderr sequence with empty protocol stdout. The regression was observed failing before implementing the output port.

The warm proxy also needed a separate modern opener: server/discover-first now starts the staged native peer without a synthetic initialize/initialized exchange, queues modern lists and forwards the real discovery answer. The language-neutral journey was observed red before the implementation, then passed against the official SDK result schema.

The source suite after these corrections passed 48 tests and 408 assertions. The current corrected monitored readiness attempt is session 70070, with live stderr, a finite 30 min diagnostic bound and a 15 s outer cleanup allowance. Native readiness remains pending and must not be inferred from the immediate warming response.
