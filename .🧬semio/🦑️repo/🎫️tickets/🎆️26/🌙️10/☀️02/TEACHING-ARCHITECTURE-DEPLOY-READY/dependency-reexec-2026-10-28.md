# Dependency Publication Reexecution — 2026-10-28

The terminal full-gate failure reported by the parent (54529) followed JavaScript dependency publication: the captured producer returned the declared dependency-readmission status 76, then its caller attempted a second launch inside the failed Original. That admission refused; the first tracked child failure remained status 76, which the outer command treated as an ordinary error and returned 1.

## Exact boundary

The captured command kernel in build/⌨️command/📥️entry recognizes only a privately issued dependencyReadmissionV1 request, after receiveScriptProcessInvocation has settled, and assigns the schema-declared exit 76. The sealed executeImplementationV1 previously passed a readmitted boolean callback to withDependencyReadmissionV1; the helper called that callback again after an OwnedCommandFailure outcome of 76. That callback tried executeCapturedBuildSourceV1 using the same already-failed invocation/control. This was the source of the later Original admission closed consequence.

The helper now translates a normal owned-child terminal 76 (signal null) into existing staleImplementationV1. The sealed caller performs exactly one implementation launch. The failed Original fully settles and retires before the existing runWithStaleReexecV1 boundary handles that recognized restart signal. Nested commands retain the existing exit-75 propagation and the outermost supervisor performs its bounded external replacement. No new environment variable, restart layer, process routing rule, registry policy, or failure-erasure rule was added.

The original argv, chosen language/tool environment, and serialized process envelope pass through the existing supervisor. A replacement uses the original envelope creation time and finite deadline rather than obtaining a fresh elapsed-time budget. A genuine earlier failure remains the authoritative error and prevents a restart.

## Test-first evidence

The actual-Original regression failed against the previous helper with the first OwnedCommandFailure 76 after the attempted recapture. The earlier-real-failure case already passed and remains green. The fixed lifecycle test starts a real owned child process that exits 76, verifies retirement of both old invocation and resource control before external replacement, and verifies original command/environment/envelope and remaining deadline.

Language-neutral JSON vectors cover ready, publication handoff, ordinary failure, and replacement failure. Ajv validates the schema and SQLite supplies an independent decision oracle. The cold-source fixture also launches a real captured child: the immutable cold snapshot still refuses an unrecorded lazy package after installation, while an externally admitted fresh snapshot resolves the published package; native Node createRequire independently verifies its answer. Private issuance rejects copied or newly constructed errors.

Final verification: 26 tests passed, 396 assertions, zero failures across dependency readmission, existing format-stamp restart, and existing graph-yield restart suites. The broader suites include real-process exit-75 replacement and graph-yield propagation. Their existing wiring assertions required updating to the current captured-kernel catch suffix and the current door/🏃️main entry location; production front-door behavior was unchanged.

## Changed-file ledger

All paths are beneath 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/:

- 📥️source/🔏️closure/🧬️schema/🔣️.json: declare use of existing external reexecution instead of an in-process maximum-readmission count.
- 📥️source/🔏️closure/🔁️dependencies/🟦️.ts: map normal owned-child exit 76 to the existing stale-implementation signal; remove in-process retry.
- 📥️source/🔏️closure/🔁️dependencies/🧪️tests/🔣️.json: external-boundary vectors.
- 📥️source/🔏️closure/🔁️dependencies/🧪️tests/🟦️.ts: schema/oracle, cold snapshot, private issuance, and actual Original lifecycle regressions.
- 🏛️sealed/📥️execution/🟦️.ts: one implementation launch under each Original.
- 🧷️format-stamps/🧪️tests/🟦️.ts: follow existing current entry wiring.

## Live checks

Framework generation check 48809 remains running. Its latest authoritative output was graph generation 0, all one compute slot busy, 29 stale projects still open at approximately 2,783 seconds; it has not reached framework Cargo metadata or compiler diagnostics. No duplicate framework/proctor build or manual fetch was launched. Parent full gate 4072 remains separately owned by the parent. These unit and process regressions do not establish release or E2E success.
