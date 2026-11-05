# Native Source Budget

The actual root native attempt in forms-native-paged.log refused before compiler in semio-s-artifact-forms-forms-forms: answer.json was4923 bytes with2272 bytes remaining and4 files remaining. This was an exact Source-byte refusal, not a source-file-count refusal. No Source/capacity/storage limit was raised.

## Inventory And Savings

Canonical readRustSourceClosureV1 was used read-only with explicit macOS test cfg (recorded in editor-source-before.json and editor-source-compact.json). This diagnostic inventory is not a replacement for actual native admission. It found33 files and282180 bytes before compaction, then253884 bytes after, net28296 bytes saved and8260 bytes under262144. File count is unchanged.

| Included fixture | Before | After |
|---|---:|---:|
| Shared TextEditor value-source |30276|6251|
| Shared Forms long answer |4923|267|
| Try target vectors |2482|2482|
| PatchQuestions vectors |1935|1935|

The Rust graph decoder grew186 bytes, and the Rust Try answer decoder grew199 bytes in this source closure. Total fixture savings28681 minus385 decoder bytes yields net28296. The579-byte source-recipe schema is a test-only TypeScript oracle resource, not an embedded Rust input; the existing answer schema is likewise test-only. Neither has a production$id. Responses test decoder changes are in their separate owner, outside this editor closure.

## Exact Expanded Equivalence

The graph now stores a closed prefix/unit/repeat/suffix recipe. Expansion remains16375 raw UTF-8 bytes and24516 JSON-encoded bytes. Raw SHA256 is e5ac7a81825d5e2e171f195f7ca077923ff51c4da6ae6e73b0cd70935a6b0e2b; encoded SHA256 is be714619072ab6354cd57d20afa6c6d20c879d86423359bbe5aa4b830893eaec. The original graph was compared byte-for-byte before replacement; green bounded validation compares both original hashes after reading the authored recipe. The existing at/over encoded-boundary object and all limits remain unchanged.

The answer now stores unit ä🧱 repeated800 times, exactly4800 raw UTF-8 bytes and4802 JSON-encoded bytes. Raw SHA256 is5db77edee49cbfe74793c6a45711dc59ab9658b1d4a9053bddc6ec6f90c6a3b4; encoded SHA256 is15f5db98eec353d55392fce5f597a03065f2acd500a3c2f97433bdd8bf38ce71. Strict Ajv validates the new closed recipe and refuses the previous scalar shape. The kinds array requires both distinct text and longText cases; duplicate-kind negative validation passes. Root independently verified expansion using Buffer concatenation rather than string.repeat; see fixture-representation-review.md.

## Source Changes And Consumers

- TextEditor test fixture value-source/🔣️.json and new source-recipe.schema.json.
- TextEditor pure value-source/🟦️.ts: expands both recipes, validates recipe schemas, checks original hashes/bytes and preserves transport assertions.
- Forms editor PatchQuestions TypeScript tests: expands the graph before normalization/action dispatch.
- Forms editor unit Rust laws: expands the graph before public command decoding and application.
- Forms answer.json and answer.schema.json: closed repeated answer plus exact byte/hash metadata, unique kinds.
- Forms Try native unit and Responses native tests: expand the same answer and assert its byte count before rendering.
- Play acceptance/🟦️.ts long-answer case: validates the recipe, expands it, asserts exact800-unit content, and preserves full paste/submission/readonly value checks.

All direct shared fixture readers were manually updated; no old scalar or expanded-graph compatibility route was introduced. Browser/provider owners confirmed no overlapping edits. Main descriptors/index owners19405/76837 remain retained. Normal Forms consumer suite91085 (forms-recipe-consumers.log) and browser focused69935 (source-editor-recipes-final.log) are pending at this report. Native verification must follow provider19689 terminal and root’s fresh owned invocation. No fresh native pass is claimed.

## Exact Changed Files

- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧪️tests/🧫️fixtures/📄️value-source/🔣️.json
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧪️tests/🧫️fixtures/📄️value-source/source-recipe.schema.json
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧪️tests/📄️value-source/🟦️.ts
- ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-questions/🧪️tests/🟦️.ts
- ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🧪️tests/🧫️fixtures/📝️answer.json
- ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🧪️tests/🧫️fixtures/📝️answer.schema.json
- ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📨️responses/🪟️windows/📊️results/🧪️tests/🦀️.rs
- 🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts

## Final File-Boundary Correction

The byte-only intermediate inventory was insufficient: actual policy also limits each unit to32files/4000activeLines. The compact editor closure had33files. A single public PatchQuestions admission/application law now lives in semio-s-artifact-forms-forms-patch-tests, beside its existing TypeScript implementation and neutral vectors. Its Cargo manifest is explicitly a Forms workspace member and depends only on existing public editor/framework packages. All literal dependency targets exist. Private mounted factory laws remain in editor. No assertion, byte-boundary vector, scalar refusal, normalization/application check, cap, or authority guard was removed.

All three platform cfg inventories, passing actual owner manifestDirectory to the canonical reader, now agree: editor31files/241049bytes/3624activeLines, headroom21095bytes; new Patchowner3files/13309bytes/63activeLines. These are bounded diagnostics; actual native compilation remains required. Inputs and outputs are source-inventory.ts and generated/{editor,patch}-split-{macos,linux,windows}.json.

Additional changed files: Forms Cargo.toml workspace membership; editor/commands/patch-questions/tests/🦀️.rs (moved public law); adjacent tests/packages/rust/Cargo.toml (new dedicated owner). Root must include semio-s-artifact-forms-forms-patch-tests as its10th package in the final native command.

## Actual Consumer Result

Normal owned Forms consumer suite 91085 completed with exit 0: 25 tests passed, 0 failed, 78 assertions in 1.038 s. This includes canonical question patch ingress and the updated shared fixture consumers. Full output is forms-recipe-consumers.log. Native execution and browser focused execution remain separate required gates.
