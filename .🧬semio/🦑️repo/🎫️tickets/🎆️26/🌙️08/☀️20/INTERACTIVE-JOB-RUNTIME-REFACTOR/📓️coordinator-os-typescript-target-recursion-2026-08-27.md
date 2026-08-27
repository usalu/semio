# OS TypeScript Test Target Recursion

The executor's actual Nx project output resolves `@semio-tech/framework-os:test` to `nx:run-script`, which executes its package script `bun nx run @semio-tech/framework-os:test` and recursively calls the same target. The explicit `📋️project.json` correctly declares `nx:run-commands` calling its local `📜️script.ts`; inferred package-script metadata overrides it. The unaffected `test-quick` target is being used for the current client behavioral fixture, not accepted as a repair of `test`.

Coordinator source confirmation:

- The OS TypeScript package contains the recursive package script and no `nx.includedScripts` restriction.
- Installed Nx `node_modules/nx/src/utils/package-json.js` selects inferred script targets from `nx?.includedScripts || Object.keys(scripts ?? {})`.
- The existing repo library package already uses `nx: { includedScripts: [] }` to keep its explicit target authoritative.

The narrow assigned fix is the same explicit inference exclusion for the OS TypeScript package. Preserve its package script, declared target and task-router naming; add no dependency or alternate script. Verification requires both the resolved target's `run-commands` executor and a real canonical `test` execution. No broad Nx configuration rewrite is needed.

The executor separately fixed an unawaited existing asynchronous Vitest helper in the local script. That change and the recursion fix need real execution; source inspection alone is not a test pass.

The coordinator did not stop or alter any process. The executor reports stopping only its validated recursively launched process group; peer work remained untouched.
