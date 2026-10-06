# Runtime Module Boundary

Explicit test commands now load test factories and examples lazily. Native plugin runner test dependencies are initialized by the test entrypoint, fixing missing Ajv/parseArgs/readFileSync module bindings. Fresh-component and repository cache helpers no longer initialize test modules when imported for runtime operations. CAD interaction records are actual model catalog data, and their names reflect that ownership.

Neutral regression and runtime factory boundaries: Nx @semio-tech/framework-test:test-adapter-ownership passed8tests184assertions, including independent TypeScript import inspection. Cache-policy entrypoint passed35portablelaws through Nx against installed Nx. Fresh component behavior passed:17physical source laws,2bounded captures,7unsafe path cases,19+3dep-info laws,11staging laws and10process laws. Actual runtime stages success/compiler-error/stderr-error/spawn-error/cancelled/deadline/output-limit/queued-cargo-cancel/queued-cargo-deadline were logged; independent AJV/stable-stringify/WebCrypto/Pack/BLAKE3/fast-deep-equal checks passed. Cargo resolver/dep-info integration is explicitly unqualified by the source suite. The fresh factory and its dependency type have since been extracted entirely into the test module; runtime source owns no test facades. Test output resolves automatically for ordinary launch use or honors caller ticket output.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/📜️script.ts`

Ordinary-launch evidence location was exercised by the first isolated fresh run without an explicit artifact directory. Its three exact generated proof directories were moved into the ticket generated directory after completion. Only those owned directories were touched; existing cache artifacts from other work were preserved. The redundant original run was stopped after the isolated gate passed.
