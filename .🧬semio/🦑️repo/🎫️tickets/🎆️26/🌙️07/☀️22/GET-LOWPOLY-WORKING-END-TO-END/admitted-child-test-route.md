# Admitted Child Test Route

Read-only investigation, 2026-11-05. No test launch, source edit or environment construction.

## Exact Setup Refusal

Actual-child file is process /🧭️routing/📥️invocation/🏃️process/📤️child/🧪️tests/🟦️.ts. Its top-level test directly invokes receiveScriptProcessInvocation(process.env). Receiver requires SEMIO_SCRIPT_PROCESS_INVOCATION from an actual parent; absence raises Original parent Script process invocation required before assertions.

## Available Routes

- repo tools exec --light bun -- test <exact file> has no original parent birth guarantee and already refused at setup.
- Removing --light does not add parent authority. Build tool-registry command exec changes only heavy:!head.includes(--light); both invoke the same runToolV1 with sanitized ambient environment and capacity. No createScriptProcessEnvelope occurs there.
- Direct repo ./process/📜️script.ts test routing-owners enters a script that itself calls receiveScriptProcessInvocation, not a fresh authoring function. Its source therefore does not solve missing parent authority on its own. A generic launcher adding authority was not established by this audit; do not assume it.
- Declared official owner target is repo run @semio-tech/framework-process:test-routing-owners. It invokes repo ./📜️script.ts test routing-owners with cwd process owner. Suite row selects routing tree, budget300000ms, and owned runner supplies each file inherited envelope. However row platforms are darwin/linux only due python3-command; native Windows reports SKIPPED. test-routing does not include child subtree.
- No existing dedicated cross-platform actual-child target/suite was found. General owner target still depends on canonical build admission; this audit establishes no supported alternate bypass of unpublished JS dependencies.

## Authoritative Documentation

Repo 📓️build-guide.md:204 identifies missing parent Script invocation as envelope-only setup, not product/test defect. :206-209 documents declareBuildTestEnvelopeV1 with withBuildTestInvocationV1/registerBuildTestLawV1. Crucially :209 explicitly says a file calling receiveScriptProcessInvocation(process.env) itself ignores declaration and must first convert to withBuildTestInvocationV1. Therefore simply adding declaration or CLI environment would be incorrect for this exact child test.

Supported path today is official owning target on its supported platform. A deliberately authored focused cross-platform suite or conversion to documented test birth helper would be source work requiring review, rather than a different tools exec flag. No such source changes performed.

## References

Process /📋️project.json:test-routing-owners; /📜️script.ts TestScript and main receiver; /🧪️testing/🗂️suites/🔣️.json:routing-owners; /🧪️testing/🗂️suites/🏃️run/🟦️.ts passes live invocation and suite capabilities to runOwnedBunFilesV1; /🧪️testing/🟨️bun/📁️files/🟦️.ts carries issued envelope to each owned child. Repo build /🧰️tool-registry/⌨️command/🟦️.ts:42 distinguishes light only through heavy flag.
