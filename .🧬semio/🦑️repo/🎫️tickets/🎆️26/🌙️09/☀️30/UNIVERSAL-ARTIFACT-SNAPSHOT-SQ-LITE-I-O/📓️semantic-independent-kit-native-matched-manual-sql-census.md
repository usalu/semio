# Kit Native Matched Manual SQLite Census

Independent Bun 1.3.14 SQLite execution used actual Kit SQL and manually authored semantic cells. No owner projection, decoder, row visitor or provider supplied expected rows. Input and retained audit script are `📥️inputs/semio-kit-complete-semantic/handcrafted-native-matched-demand-specifications.json` and sibling `📜️script.ts`.

| Case | Semantic Rows | Semantic Value Bytes | Public Rows | Public Binary Value Bytes | Public Text Value Bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| Full | 21 | 1342 | 22 | 1382 | 1380 |
| Metadata Retaining Empty | 1 | 23 | 2 | 63 | 61 |

Semantic schema accounting is 3641 bytes: each independent SQLite master table's SQL without trailing semicolon plus UTF-8 table name. Raw authored SQL is 3459 bytes and is not the schema budget. Public metadata adds 293 schema bytes, producing 3934. Semantic schema has eleven tables, public schema twelve; maximum width is 35 physical piece columns. Public metadata adds 40 Binary or 38 Text value bytes, measured with actual Kit three-letter subset and literal artifact kind/standard/encoding fields.

Full Native fixture fields are matched literally: Chair 世界, furniture, layout/Design, left/right pieces, joint a/b connection, object-one/object-two, model-one, value-one, three representation targets and all three history pin kinds. Ten f64 transform fields per piece are captured as exact sixteen-digit words with numeric query cells plus signed-word/class sidecars; full identity has one translation x=1 and identity quaternion/scales. Seven reference rows independently retain artifactId, artifactKind, standard and subset. Blob size is decimal maximum u64, not a JavaScript number. SQL connection query returns left/right, blob query returns `18446744073709551615`; integrity and foreign key checks both passed on all six manual databases.

Metadata retaining empty clears types, designs, objects, models and representations; properties is absent, matching Rust None ToValue and the actual TypeScript parser's optional own-field behavior. Schema remains stdio.semio.kit. Hydration converts sixteen-digit numeric words to bigint and explicitly converts snapshot-pin blob decimal size to bigint. Actual Bun execution of the exact specification hydrate via Bun.Transpiler and actual parseSemioKitSnapshot admitted both cases: full 1 type/1 design/2 objects/3 representations/properties present; empty zero collections/properties absent. Initial direct parser checks caught and corrected input-only null properties and decimal-string blob size before handoff.

Actual framework parseBinary32/parseBinary64 sweep separately admitted Mesh 78 Binary32 number words and 144 Binary64 bigint words, and Kit 20 Binary64 bigint words. The previous Mesh review's all-BigInt hydration claim is withdrawn in its report. Root Source91760's observed Binary32 failure requires owning Source replay after width-correct hydration. These manual SQLite and parser checks do not claim Native provider execution, strict TypeScript compilation, owning Source laws or cancellation/retirement qualification.

A separate actual Bun SQLite text binding/query check admitted every authored literal child identifier (`""`, `node!@/`, `nul\u0000id`, 世界) unchanged in both child and target fields. SQL hex queries returned empty, `6E6F646521402F`, `6E756C006964`, and `E4B896E7958C` respectively, confirming embedded NUL bytes without relying on string rendering. This is an independent SQLite field transport check, not an owner codec law.
