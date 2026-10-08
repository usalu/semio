# PDF Stream Roles and Pure Graph Projection

The PDF 1.7 production diff owner projects dictionary/resource structure in the schema owner. Native parsing, packed words, CMaps, font codecs and foreign resource resolution remain in IO/native modules. There is no native `io::carry_graph_edit` dependency or replacement snapshot escape hatch.

## Current Source

The schema declares twelve admitted semantic roles: operators, sampled u32 words, calculator source, Unicode mappings, character maps, typed font programs, typed images, metadata text, literal attachment bytes, palette components, glyph ordinals and canonical artifact references. `BinaryWords` staging has been eliminated. ICC profiles, output profiles and mesh bodies identify independently admitted resources through the first-party native artifact resource port; Indexed palettes are logical component values. Host peer owns the corresponding image/font/inline/CID logical bodies and resource port, documented separately in `pdf-semantic-body-current.md`.

Each admission carries an exact object generation and structural path plus consumed logical dependencies. Equal direct values bind every structural owner; conflicting semantic aliases refuse pure lookup. Page-content dependencies include inherited Font bindings and transitive reference inputs. Changed known inputs require an explicit replacement of the same identity and role kind. Missing dependencies, duplicate replacements and unresolved identities refuse. Every resulting mutation role lane is checked, and role-only changes also invoke pure projection. Canonical Rust/TypeScript snapshot admission and normalized SQL reads validate the complete bound role lane; SQL projections refuse invalid bindings before serialization.

Snapshot/artifact state carries the lane; text/binary record field 32 persists it. SQLite uses explicit identity/path/dependency tables and typed payload relations, with u32/u16 child rows and canonical artifact-reference rows. Borrowed native census includes the role lane and shared reference rows. Native JSON admission converts native numeric fields at IO and preserves the roles. Canonical snapshot/diff TypeScript guards require owned Binary64 words and retain referenced-document context.

SetObjectValue, InsertObject and RemoveObject schema-first payloads now carry optional indexed role edits with explicit inverse restoration. Net graph leaves track their current role lane, carry each new role once and refresh affected existing roles, followed by any remaining role delta on a retained logical object. Exact replay laws cover role-only edits and admitted orphan-stream insertion/removal; these new Rust laws require native execution before closure.

JSON Schema, TypeScript, GraphQL and Protobuf role/body facets are synchronized. GraphQL input/output roles and protobuf word/reference facets have an independent third-party law registered within the existing role suite; its fresh third-party law is GREEN in current16. All twelve neutral semantic role cases now validate; extra physical font-body fields refuse.

## Runtime Evidence

- Actual GREEN: TypeScript current21 handle38592 exit0, strict7 exports/all12 suites, including independent insert/remove/set-object Protobuf exact role u32 transport. Current20 RED exposed unresolved generic T in existing Protobuf diff; all indexed/keyed payload records are now concretely typed and imported model namespaces explicit. Log `🗑️generated/pdf-stream-roles/typescript-current-21.log`.

- Actual GREEN: TypeScript current19, handle85095 exit0, strict7 exports/all12 suites including independently normalized SQLite persistence for all12 roles and exact generation refusal. Current17/18 exposed witness expectation/null-facet omissions that were repaired; neither is claimed passing. Log `🗑️generated/pdf-stream-roles/typescript-current-19.log`.
- Actual GREEN: TypeScript current16, handle67379 exit0, strict7 exports and all12 suites, `🗑️generated/pdf-stream-roles/typescript-current-16.log`. This includes twelve neutral role cases, Ajv refusal of physical font payload extras, GraphQL input/output resolution and protobuf word/reference roundtrip.
- Actual GREEN: registered snapshot SQLite source target current2, handle9819 exit0, 56 pass/0 fail, `🗑️generated/pdf-stream-roles/snapshot-sqlite-source-current-2.log`. The neutral producer roster now includes role field32.
- Actual RED before PDF: native role target21375 exit1, `🗑️generated/pdf-stream-roles/native-3.log`: MeshEngine5 errors and shared OS kernel452 obsolete retirement-consumer errors. No PDF test executable was reached. Native retries remain on hold for parent-owned foundations.
- Prior native17585 RED compilation was followed by source repairs. Native91133 RED before PDF on fourteen replication retirement consumers; root repaired those shared consumers. These old failures do not establish current PDF runtime behavior.
- Prior native PDF/PNG/BMP handles39875/57282/95430 are missing, with unknown former final receipts.

Independent TypeScript witnesses cover Ajv admitted roles, exact generations, sample words513/4095/u32max, dependency omission/kind mismatch/duplicate refusal, SQLite logical image/font/inline/glyph rows, mesh/profile reference joins, native ICC custody, complete document roundtrip and malformed ordinal refusal. Temporary runtime logs use the required DEBUG prefix.

## Remaining Closure

Actual native stream-role projection/refusal/inverse/net-replay and native ICC/Indexed/mesh witnesses remain pending. Native12/32 sampled-word stream extents now have authored independent oracle laws in the role filter. All direct third-party references were replaced by the actual first-party test-oracle provider APIs. The native role filter now includes authored nine-role full-document CMap/Unicode/calculator/font/image/attachment/metadata/glyph/operator capture, independent stream bodies and exact pure/persisted projection; separate sampled and ICC/palette/mesh laws cover the remaining role kinds. These require actual execution before closure. Native SQL retention still requires actual native evidence. Arbitrary graph reorder/deletion across mutually dependent objects needs an exact replay proof; the current narrow insertion/removal law does not prove every graph transition. Bounded multi-item ownership retirement remains root-owned and unclosed. The full artifact IO goal must remain open.

The generated `nx-native` and `nx-native-2` workspaces each consume approximately326MiB. They belonged to completed17585/91133 attempts before current21375 reused `nx-native-2`; both `nx-native` and `nx-native-2` are terminal and safe for scoped cleanup. The reused `nx-ts` directory also has no currently live owned run; keep it if further source runs are planned. No shared cache or active process was removed.

## Shared PDF Manifest

The following read-only Git census lists all current PDF changes, including co-owned host body edits and earlier work. It is a shared scope manifest, not exclusive authorship attribution.

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🧬️schema/🧬️mutations/📉️collapse-page-size/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🧬️schema/🧬️mutations/📐️set-page-size/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🧬️schema/🧬️mutations/📝️set-page-text/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🧬️schema/🧬️mutations/🧹️clear-page-text/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎓️bachelor-thesis/🧪️tests/🧩️example/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/♻️replace-page-text/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📐️resize-page/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📥️insert-page/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔀️move-page/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🗑️remove-page/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/⬆️lower/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/⬇️lift/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🎨️colour/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🎨️colour/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔗️xref/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖋️content/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖼️images/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖼️images/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖼️images/🧪️tests/🔬️unit/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/💾️binary/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/📄️document/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/📇️metadata/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🔤️font/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🖋️content/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🖼️resource/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🖼️resource/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🪪️stream-roles/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📦️foreign-artifacts/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🌈️color/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🌈️color/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📄️document/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📄️document/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📇️metadata/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📇️metadata/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📇️metadata/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📦️artifact-reference/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📦️artifact-reference/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🔤️font/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🔤️font/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🔤️font/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🖋️content/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🖋️content/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🖋️content/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🖼️resource/🌈️shading/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🖼️resource/🌈️shading/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🖼️resource/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🖼️resource/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🖼️resource/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🌈️color/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/📇️metadata/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🔤️font/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🖋️content/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🖼️resource/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🌈️color/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📄️document/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📇️metadata/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔤️font/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🖋️content/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🖼️resource/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🪪️stream-roles/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🪪️stream-roles/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧫️fixtures/🧬️mutations/🏞️set-image/🧾️wire-witness/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔗️.graphql`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛰️.proto`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/♻️retirement/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔎️graph-projection/🎨️colour/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔎️graph-projection/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔗️graph-source/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔗️.graphql`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🛰️.proto`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🏞️set-image/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️insert-object/🔗️.graphql`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️insert-object/🛰️.proto`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️insert-object/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️insert-object/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️insert-object/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️insert-object/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔏️set-mark-info/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔧️set-object-value/🔗️.graphql`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔧️set-object-value/🛰️.proto`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔧️set-object-value/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔧️set-object-value/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔧️set-object-value/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔧️set-object-value/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/⚖️lopdf-vectors/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧹️remove-object/🔗️.graphql`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧹️remove-object/🛰️.proto`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧹️remove-object/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧹️remove-object/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧹️remove-object/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧹️remove-object/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🪪️stream-roles/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🪪️stream-roles/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🪪️stream-roles/🧪️tests/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🪪️stream-roles/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🪪️stream-roles/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/📜️script.ts`

## Current Path-Edit Receiving Repair and Sparse Object Replay

Later shared source replaced net_mutations with path-scoped edit_mutations. The obsolete generator was not restored. Current receiving body lacked seven imported types/traits and two object leaf role fields; imports now name canonical first-party value/plugin/editing types and role-less object edits explicitly carry None, so known native role edits remain centrally refused.

A separate neutral replay fixture now covers two exact-generation detached metadata owners, object order change, admitted-role order change, explicit typed deletion and inverse, bulk two-owner deletion and inverse, and path edits that omit the semantic native-role payload. The Rust actual law uses PdfDiff between/apply/inverse and real graph mutation leaves; native execution pending. Five-test TS role suite independently parses current Rust test, receiving and diff bodies with Tree-sitter and checks neutral Ajv shape/deletion.

Static replay review found objects_diff_between ignored existing-owner relative order. It now emits sparse remove/add pairs only for displaced owners and recursive value patches for retained owners. HashMap/HashSet indexes retain linear identity matching; no opaque whole-document replacement or whole-state mutation generator was introduced. Native law now exercises that real sparse reordered-object diff.

TS22 receipt95075 actual RED1 first role fixture law: new replay contract was initially placed among the twelve role fixtures, causing intentional strict role admission to refuse it. The replay contract is now a distinct schema-neutral fixture at stream-roles/fixtures/replay; twelve role corpus remains exactly twelve. TS23 receipt32144 actual GREEN exit0: strict seven exports/all twelve suites and five role laws, including independently parsed receiving/diff/test Rust syntax. No native replay pass is claimed.

Additional source manifest: schema/mutations/rs (narrow imported current path-edit types and None role fields), schema/diff/rs (sparse owner reorder), schema/stream-roles/tests Rust+TS, new schema/stream-roles/fixtures/replay JSON. Main twelve-role fixture retains its original shape.

## Transition Obligations After Indexed Role Reorder/Deletion

Static replay review exposed two adjacent problems in old removed-index filtering: bulk owner deletion after indexed role reorder falsely kept deleted-role obligations, while directly removing a role index could hide modification of a retained native stream. The new Rust and TS validate_role_transition retain all known-role obligations unless both logical role identity and role lane disappear, and no changed native dependency survives. Exact identity/kind replacements remain required for every surviving changed input; the resulting lane always passes complete identity/dependency validation. PdfDiff invokes this shared semantic transition boundary.

The sixth TypeScript role law runs the actual TS transition implementation: owner+role single/bulk deletion passes; a stale unbound role fails; role removal hiding changed bytes fails; explicit same-role replacement succeeds. The actual Rust multiowner law additionally refuses a physical byte edit hidden by role removal. These remain distinct from native stream parsing; no physical decoder moved into schema. TS24 receipt73634 actual GREEN exit0: strict seven exports, all twelve suites and six role laws, including actual TS transition positive/negative runtime.

Sparse object replay validates removed/modified keys against base and additions against retained keys after removals. This permits explicit displacement remove+reinsert of the same exact owner while retaining duplicate/final-key refusal. Consumption-path deletion is now separately witnessed: removing Contents without changing the detached native stream passes; dropping Contents while mutating that still-present native input refuses. TS25 receipt61067 actual GREEN exit0: strict seven exports/all twelve suites, six role laws and independently parsed current mutation/diff/test syntax; consumption-removal and surviving input edit runtime cases pass. Native actual multiobject reorder/delete/inverse and all known stream parsing laws remain pending shared kernel compilation.

The authored role-only projection law now also exports the edited snapshot through real native IO, reads page operator spellings through the independent lopdf grammar, compares neutral replay nativeOperatorNames [q,Q], and re-admits the exported page. This checks that semantic-only role edits cross the retained graph reconciliation boundary on native output. It remains unexecuted pending shared kernel. New test-oracle API returns only first-party primitive string rows; no third-party type escapes. The source syntax gate now covers role owner, current path receiver, diff, actual native laws and oracle bodies. TS26 receipt48669 pending latest complete source gate.

Latest owned replay: TS26 session48669 terminal exit0, strict seven production exports/all twelve suites/six role laws, including actual role transition runtime and current Rust owner/diff/receiver/native-oracle grammar. New native role-only export operator witness remains unexecuted due the shared OS compiler floor.
