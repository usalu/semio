# Captured Authorizer Entry Review

Read-only bounded Source trace for actual4818 failure; no compiler execution, Source edit or stage action. Missing output alone does not identify which dispatch boundary failed.

Tools bootstrap204 captures the authorizer runtime snapshot and separately compiles captured authorizers/📜script.ts to a Bun-target emitted .mjs. Line205 invokes that emitted program with compile/root/snapshot/rawSHA/staging and awaits owned success. Line208 then requires actual semio-authorizers.mjs bytes for receipt publication. Source/program hashes are rechecked; the owner never treats child0 alone as artifact proof.

Current compiler entry registers CompileScript; only its import.meta.main branch receives an Original and dispatches runScriptMain. Actual router.run calls CompileScript.run, which delegates runScriptProgram. That delegated function in authorizers/🧪tests/🔬script-commands/🟦ts receives a process Original again, parses actual process.argv, opens the canonical tools store, validates captured snapshot/stage and compiles node-target runtime, then publishes semio-authorizers.mjs. Thus main evaluation and nested receiver ownership are concrete boundaries to test; this Source trace does not prove which caused child0/no output. No silent no-op branch was found inside actual ScriptRouter dispatch.

Existing release compiler test launches the raw Source worker, then verifies Node output and altered-snapshot refusal. Other snapshot tests execute emitted runtime fixtures, but the examined release test does not launch the emitted captured compiler entry used by tools205. A meaningful regression must execute that actual captured compiler handoff and require its real output/content/receipt plus invalid-input nonzero, preserving the supported exact live Original reuse and exact captured-source authority. Avoid a broad import.meta.main define that would activate every bundled module. Runtime informed.

## In-progress Peer Entry-Role Regression

Bounded reread found a new actual emitted-entry test, while production role signature was still being authored. It compares real captured Bun emission with independent esbuild per-module defines and SQLite expected program/library roles: only explicit program entry may execute, imported main guards stay inert, output bytes and forbidden imported output absence are checked. This is the right regression boundary; no completed production disposition is inferred. All actual compiler call sites must choose explicit library/runtime versus program/compiler roles, The delegated nested receive is supported and needs no separate handoff correction (see correction below). Existing peer work was preserved; no audit edits to Source.

## Correction: Nested Receiver Is Supported

Independently read actual process invocation145–147 and156–157: an incoming AsyncLocalStorage invocation admits its live birth, requires the exact same parent envelope and tracks the nested callback on that birth. receiveScriptProcessInvocation only prevents a second fresh birth when no live context exists. Therefore compiler runScriptProgram nested receiving is intentional supported reuse; the earlier concern is withdrawn and no authority handoff refactor is recommended. New entry-role tests are peer-authored, not Root-authored. Review remains limited to explicit program/library entry selection and emitted output proof.

## Landed Explicit Role Candidate

Current peer production requires closed module|program role. Per-source Transpiler defines import.meta.main true only when role=program AND physical captured path equals manifest.entry; every imported guard remains false. Tools emits captured compiler with bun/program; compiler emits authorizer runtime with node/module. All concrete snapshot callers in the authorizer subtree use explicit module except the program-role regression, plus tools program caller; no missing argument found in that bounded scope. No global main=true or closure/source authority widening. Supported nested Original remains unchanged.

Actual emitted role regression executes both captured Bun and independent esbuild output, verifies expected output/forbidden imported guard absence, rejects missing/invalid roles, and compares closed DATA through Ajv/SQLite. This establishes prepared coverage of the role defect; full native compiler publication success is still required, not inferred.

Current seven-file observed roster:
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🟦️.ts`: 9989 bytes / `4eb8645a785f78b15d6898986d3ce133fa4f5b917447f004c7222929e7bdd791`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts`: 35699 bytes / `ad8aeaeebdf6c14d607423ddd21427be4fd4b870dfec880d88602813a55dba62`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🔬️script-commands/🟦️.ts`: 4646 bytes / `fc4463e4363f92fa272d60024bcbe623d5017e47927ed2c6209dfb98af1de126`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts`: 44527 bytes / `68e639c4b819e1360fb4584fcfefa0862a9953ee4924a9e2a5b1b750aa69edef`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️workspace/🧪️tests/🟦️.ts`: 10537 bytes / `08a4e9a5742c41ac7c0122414461942da5a13bf89754ad757af6cbc8bab493f3`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🧬️schema/🔣️.json`: 1645 bytes / `5d19c5d1de3ab23ab65012793cf763a082ea22f9cd3e89be59488e48bfd4df46`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🧪️tests/🔣️.json`: 927 bytes / `d1bd91f210b58be7b33da0899a251026475450d33f086f12f381db373ebe5df9`.

No material blocker found for native regression of this exact landed role wiring. No tests executed by audit.
