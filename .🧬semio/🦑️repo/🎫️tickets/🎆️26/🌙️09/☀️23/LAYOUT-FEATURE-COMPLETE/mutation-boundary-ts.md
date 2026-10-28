# Canonical Mutation And Graph Boundaries

Production TypeScript now has the canonical whole-document graph validator and targeted semantic admission helpers under Layout schema/validation. They work on owned Binary64/Binary32 words and bigint UTF-8 ranges; the named native JSON boundary remains separate. The validator mirrors native graph rules, including Unicode White_Space ids, finite geometry and style values, exact paint memberships, spread/page references, inherited frame override/collision rules and acyclic text chains. Parent normalization returns a new parent and never modifies input.

Actual executed validation: 61 whole-document witnesses,9 parent normalization witnesses,12 targeted UTF-8/IEEE/reference/refusal checks. Independent Ajv projection and Kahn topology oracle produce matching outcomes. Strict actual223-source closure: zero diagnostics.

Production mutation parsing and named native JSON conversion cover all58 externally tagged mutation payloads on the canonical root model. Canonical values require owned IEEE words and explicit nullable fields; native mutation fields use only defaults declared by each existing schema, including CreateLayer.remove=false. Unknown tags/extra fields, malformed colors, unsafe or invalid unsigned indices and incompatible numeric representations are refused. Canonical NaN bits remain representable and the finite native JSON projection explicitly refuses them. Nested snapshot nullable fields remain required, as the authoritative root JSON Schema requires. An omitted nested snapshot field witness was added as a refusal rather than introducing permissive legacy decoding.

Actual executed transport: all58 variants,3 native default cases and15 refusal assertions pass. Independent Ajv compiles all66 real primary JSON Schemas and admits every valid input witness. Strict actual221-source closure: zero diagnostics before the additional omitted-nested-field refusal; the final combined check is pending. Tests were written and the missing implementation failure observed before runtime implementation.

The central diff algebra rerun caught a handcrafted golden spread-order mismatch after global page reorder in moves-a-middle-row. The execution agent was notified; no pass is claimed for that rerun. Native compilation remains unavailable through the authorized repo runner; no native pass or complete Layout claim is made.

Files:
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🟦️.ts
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧪️tests/🟦️.ts
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️native-json/🟦️.ts
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️native-json/🧪️tests/🟦️.ts
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/📨️mutations/🔣️.json

Mounted explicit public package entries for validation, mutation model, evaluation and named native JSON. Added the owned verify layout-canonical route and its real test runner. Actual runner result:66 primary schemas;61 graph documents9 parents12 targeted checks;58 mutation variants3 defaults15 refusals;88 semantic portable cases51goldens58variants5wordcases for this loaded cohort. Complete strict451-source closure, including the public composed runtime, schemas and semantic/oracle tests: zero diagnostics. Native execution is still unverified.

The corrected central diff suite now passes37portable45golden500generated10refusal cases. A fresh semantic fixture load also independently passed88portable51golden58variants5IEEE cases. The source fixture cache initially retained an obsolete parent fixture; refreshing only source fixture/test modules resolved it, without changes to admitted runtime behavior.
