# IFC4 Integer Precision and Optional Constituents

GraphQL Int carries only signed 32-bit values, while IFC identities/references use u64, integer arguments use i64 and collection coordinates use native unsigned positions. Snapshot, diff and mutation GraphQL facets now use owned IfcUInt64, IfcInt64 and IfcIndex scalars. Each transports a canonical decimal string. The snapshot facet also exposes its previously missing referenceValue field. Root artifact GraphQL and protobuf facets now expose the actual header/entity state instead of stale text placeholders. The optional GraphQL complex field follows canonical JSON and TypeScript omission.

The owned TypeScript helpers parseIfcGraphqlInteger and printIfcGraphqlInteger preserve bigint, reject JavaScript Number coercion, reject noncanonical/out-of-range values, and expose no external GraphQL runtime types. ifcGraphqlIndex converts only exact, safely addressable TypeScript positions. Package exports are explicitly named for IFC4. No external runtime dependency was added.

## Independent Validation

The permanent three-group suite uses GraphQL 17.0.2 to build/validate all four facets and assert every identity/integer/index mapping. Actual GraphQL string AST decoding/serialization preserves full u64/i64 extremes; numeric GraphQL literals, unsafe JS numbers, malformed decimal strings and out-of-range integers are refused. Protobufjs 8.8.1 independently parses all corresponding protobuf facets and round-trips native unsigned/signed extremes through encoded bytes without numeric conversion. All three groups pass through the actual source loader, with zero strict TypeScript diagnostics. The existing permanent four-group mutation suite was rerun after the optional complex interface update; all groups pass.

Input runner: ifc4-permanent-mutation-check.mjs run(root, ticket, relativeTest, expectedGroups), using pathToFileURL. All generated modules were deleted.

## SQLite Optional Complex Integration

The IFC4 SQLite projector now binds parts = entity.complex ?? [] and uses that list for admission and traversal. Existing test graph traversal also handles omission. Added a native SQL regression that removes both optional fields, verifies zero complex-type rows with the independent SQLite reader, and reconstructs explicit empty lists. The SQL suite was authored but not executed: its broader dependency graph currently has unresolved workspace aliases/types in framework/UI/manifest packages. Those infrastructure files were preserved. A complete IFC4 TypeScript source scan found no remaining unguarded complex member dereference.

## Files

- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔢️integers/🟦️.ts`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔢️integers/🧪️tests/🟦️.test.ts`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔢️integers/🧪️tests/🧫️fixtures/🔣️.json`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️.graphql`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔗️.graphql`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🛰️.proto`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🟦️.ts`
- `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️21/COMPLETE-IFC-4X3-ARTIFACT-EDITING-AND-INTERCHANGE/ifc4-permanent-mutation-check.mjs`
