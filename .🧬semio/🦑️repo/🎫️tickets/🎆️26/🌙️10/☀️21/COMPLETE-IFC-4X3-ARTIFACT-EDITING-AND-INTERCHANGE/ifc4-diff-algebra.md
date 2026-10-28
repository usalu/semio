# IFC4 Ordered Diff Algebra

Entity removals now carry id and original index; identity alone cannot correctly transport insertion positions through later removals. Rust and TypeScript implement exact composition and inverse, including annihilation and same-id reinsertion. The central applier validates original positions before writes, and unchecked Rust collection/field helpers are private. Rust modification lookup is keyed rather than quadratic. JSON schema, text grammar and codec, binary codec, and remove-entity mutation agree on the new records. No old wire support is retained. Binary diff rejects unknown format/flags and trailing payload.

## Executed Validation

Strict TypeScript compiler validation: no diagnostics. Five test groups passed, comprising 1,043 sequential edit pairs, 588 three-edit structural sequences under both associations, exact inverse, immutable source, same-id relocation, checked invalid targets, omitted complex inverse, and absent optional fields. Independent RFC6902 oracle: 1,075 comparisons with fast-json-patch 3.1.1. Harness: [ifc4-diff-check.mjs](ifc4-diff-check.mjs). Source tests: IFC4 schema/diff/🧪️tests/🟦️.test.ts. Language-neutral cases: IFC4 schema/diff/🧪️tests/🥒️.feature.

## Native Verification Limit

Rust position/codec regressions authored but not run. Repository-routed command bun run repo run @semio-tech/stdio-ifc-rs:test --output=stream fails before IFC compilation: Windows refuses per-directory case sensitivity for the native-build cache (access denied). The cached native repo test command silently returns with no tests, so it is not validation. Remote agents own build infrastructure.

## Independent Schema Checks

AJV accepted indexed removal records and rejected bare removal ids/missing indices. protobufjs 8.8.1 and GraphQL 17.0.2 parsed the aligned protocol contracts. Protobuf header/complex lists use presence-preserving message wrappers so an omitted edit differs from clearing a list.

## Changed Files

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🛰️.proto
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🟦️.test.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🥒️.feature
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/📝️text/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/📝️text/🔺️diff/📖️.grammar.semio
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-entity/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs

Longest coordinator source path: 151 characters.

The IFC TypeScript package suite router now registers the permanent diff law tests, preserving the concurrently added IFC4x3 suites. Test-only fast-json-patch resolves from the repository dependencies.
