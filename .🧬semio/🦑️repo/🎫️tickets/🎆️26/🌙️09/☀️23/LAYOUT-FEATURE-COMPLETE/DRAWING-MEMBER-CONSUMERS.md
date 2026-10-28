# Reference Only Layout Drawing Consumers

## Completed Slice

Layout SQLite now owns only layout_drawing_member rows containing the Document or Page owner and the exact child handle/target reference. Its schema does not append a child codec schema, project child content, or reconstruct a second current child snapshot. Native JSON child/pageChild accepts exactly a handle record and refuses inline payload properties. Real child state and composed envelopes remain the actual member store's responsibility. Core concurrently migrated the canonical LayoutDrawingMember root/model type and separate typed authored births; those Core edits are outside this path list.

Reference alias validation sorts original numeric positions with first-party controlled metadata and compares borrowed UTF-8 keys in bounded chunks. Both child identity and target identity must remain unique across Document/Page ownership. Reconstruction also rejects ambiguous, missing, multiply owned and unsupported-dialect references. The SQLite grammar intentionally uses no secondary indexes or generated indexes; native and portable semantic controls enforce uniqueness.

The neutral ChildAdmissionProviderV1 returns one ChildAdmissionRead containing source and mandatory registered admitter together. Its opaque ChildAdmissionAddress contains a prequalified provider-local ordinal and original fixed stamp: parent, expected reference, child, capability, command and both fixed 32-byte revisions. Its birth has an actual original copy/items/depth gate. Full authored key strings are qualified before token issue; provider read only checks fixed identity/address fields. Core implemented the qualification trait on its actual ChildEditorStamp and integrated expected-reference substitution laws. The address cannot mint an original source issuer.

The Layout drawing_member::source facade selects Document/Page references in constant work, gates a fixed original metadata demand before lookup, checks the exact original handle address/object against the qualified address, and asks the provider for the paired source/admitter. It returns DrawingReadAdvance {progress,done,read}. Denial returns pending/zero receipt; funded absence returns done plus actual receipt; errors preserve paid metadata and any original nested effects. The facade never reads inline content or a genesis as the edited current head.

## Executed Evidence

The durable drawing-member-read-check.mjs input harness checks the three implementation test roots with strict TypeScript, then executes actual sources in-process. Latest result: zero strict diagnostics; three independent Document/Page member references round-trip through the independent SQLite C engine; seven invalid datasets refuse in both implementations/oracle; inline content refuses and childTables is zero. Source facade has six neutral laws, three denied grant axes, exact source/producer pairing and original handle replacement refusal. Provider has seven source stamp field substitution laws, three denied births, three foreign ordinals and exact portable object identity. Native real-issuer/source/relational laws are authored. All changed native sources in this slice parsed without syntax errors; native execution/compilation remains unavailable because the required repo runner is not found. Syntax is not compilation.

## Outstanding Vector Consumer Work

The old snapshot-only Semio VectorSceneCursor cannot represent genuine Draw paged UTF-8, affine shear, gradient paints, RGBA sample sources or boolean/trace production. Core removed the inline codec model helpers with the canonical reference migration; old scene/export call sites must be rebuilt against qualified current member sources. Existing Draw DocumentVectorPreparationJob has scalar work grants for birth/advance and funded physical closure; it cannot be wrapped as an original-funded producer without upgrading its birth/advance storage contract. This audit does not claim that scene/vector/IO interchange consumers or full ReplaceDocument Drawing admission are already complete. No raw-content fallback is introduced.

## Exact Agent-Owned Changed Paths

Shared mount files preserve concurrent work. All listed paths are shorter than 256 characters.

- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/📖️provider/🔣️.json`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/📖️provider/🟦️.ts`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/📖️provider/🦀️.rs`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/📖️provider/🧪️.json`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/📖️provider/🧪️tests.rs`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/📖️provider/🧪️tests.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/📖️source/🔣️.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/📖️source/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/📖️source/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/📖️source/🧪️.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/📖️source/🧪️tests.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/📖️source/🧪️tests.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️member/🔣️.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️member/🔮️oracle.mjs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️member/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️member/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🪆️member/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🪆️member/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🔍️rows/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/🟦️.ts`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/🦀️.rs`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/🟦️.ts`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🦀️.rs`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🟦️.ts`
- `🧰️framework/📦️packages/🟦️typescript/🚪️io/🔌️ports/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE/drawing-member-read-check.mjs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE/DRAWING-MEMBER-CONSUMERS.md`
