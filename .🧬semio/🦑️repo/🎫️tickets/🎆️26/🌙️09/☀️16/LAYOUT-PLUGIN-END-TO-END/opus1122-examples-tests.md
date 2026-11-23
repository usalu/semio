# Layout examples test gaps

Changes (artifact root: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout`)
1. `📚️examples/🛂️manifest.json` (stale `layout-sample`, `📏️sample.layout`, nothing referenced either) now lists the real examples demo, demo-session, linked-asset with fixture paths; `schema: "manifest"` as in the sibling draw/puzzle manifests.
2. `print-target`: the examples are right, the schema is not wrong. Rust `print_target: Option<String>`; the text grammar (`🚪️io/📝️text/📸️snapshot/📖️.grammar.semio`) makes it an optional field; the JSON schema lists it required but nullable (anyOf string/null) because serialization always emits null. No change to schema or DSL; now pinned by a test.
3. New `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧫️cases/🧫️cases.json` (language-agnostic) and `…/🧪️tests/🟦️.test.ts`: boot-query cases through `resolveBootQueryExampleId` + `examplesForApp` (offered to layout dialect, none to another), fixtures exist, DSL sections vs schema required keys (only printTarget missing), ajv 8.20 (already in node_modules) validates a schema projection (required + printTarget) since the full schema has unresolvable external $refs.

Results: new test 11 pass 0 fail (bun). Logs: `🗑️generated/opus1122/examples-*.log`.

Open finding: `🎠️playgrounds.json` lists example ids with emoji prefix (`🎬️demo`) while manifest ids are `demo`; same for draw/lowpoly, generated, not changed. `?example=🎬️demo` would be dropped by examplesForApp. Not covered by a test yet. linked-asset still has no TS twin/Rust tests.
