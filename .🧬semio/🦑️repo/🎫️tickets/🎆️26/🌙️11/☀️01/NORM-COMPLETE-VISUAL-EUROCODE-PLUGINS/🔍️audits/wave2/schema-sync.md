# Schema sync, wave 2

No cargo, no build, no git write. Rust was never edited. Everything below is verified only by the static checker and by `node --check` (type-stripped) on every touched TypeScript twin.

## 1. Is there a schema generator?
- `repo ./📜️script.ts schema generate` regenerates only the derived scope catalog (`🔣️schema-catalog.json`, "5249 scopes") from the schema modules on disk. It does not read field lists and writes no twin.
- The `ArtifactSchema` derive in Rust and the nx target `framework-schema:generate:prototype` do not write the committed `🔣️.json` / `🟦️.ts` / `🛰️.proto` / `🔗️.graphql` either. The five files of a module are committed hand/script products. TS twins say "generated from `./🔣️.json` by `🧪️s2-norm-ts-twins.ts`": that is a one-shot bun script of an old ticket (`🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING`, hard-coded `/Users/ueli/...`, bun not installed here); proto and GraphQL twins come from per-owner scripts (`wp01-gen-twins.py`, `🧰️scripts/en1998-gen-root-twins.py`, `en1995-wave2-twins.py`). Nothing keeps the five in step, which is why tonight's Rust edits left the JSON and the twins behind.
- Therefore: JSON repaired to the intended model (newest Rust, which compiles and is what the code writes), twins repaired to the JSON.

## 2. Tool (language-agnostic, no code shared with what it judges)
Installed in the repo as a test and kept in the ticket:
- `✏️s/🔌️plugins/📕️norm/🧪️tests/🧬️schema-sync/🐍️check.py` (also `🧰️scripts/schema-sync/check.py`): for every schema module of en1990..en1999 and every shared norm crate it parses JSON, Rust (struct/enum, serde rename and rename_all, Option, `#[state]`, ints), TS (interfaces, aliases, `parse*` readers incl. `normWireRange`/`normWireLiteral`/required-ness), proto (messages, enums, `optional`/`repeated`) and GraphQL, and compares field names (wire camelCase), order, type, optional/required, integer bounds against the Rust int type, enum members and order, state class, duplicate proto numbers, unbalanced reader syntax and extra/missing types. 2243 modules, 2583 Rust types.
- `🐍️sync.py` (also `🧰️scripts/schema-sync/sync.py`), dry run by default, `--apply` writes; each file is re-read right before it is written and untouched files are never rewritten; idempotent. Phase A: JSON := Rust (field set and order, `required` from Option, `["T","null"]` for Option scalars, integer bounds from the Rust int type, enum members, missing `$defs` for record types, annotation keys such as `x-semio-ui`/`x-semio-state` preserved, original compact or two-space layout preserved). Phase B: TS interface and reader, proto message (renumbered 1..n), GraphQL type := JSON; existing field lines, docs and type spellings are kept when still correct; twin types renamed to the JSON name (`En1996Wall` to `MasonryWall`), orphaned twin types removed, missing ones added.
- `🟦️.ts` in the same folder: bun test `schema sync` runs the checker (python3, python or py -3) and requires `mismatches=0`.
- Conventions the checker accepts (not mismatches): proto `string` and GraphQL `JSON` as the blob lowering of record lists/records in artifact and diff roots; nested proto/GraphQL types may be absent when the root lowers them; TS `number` for JSON `integer`; TS/proto/GraphQL type names may carry the `En1990`-style artifact prefix; enum member spelling in proto/GraphQL is case/prefix-insensitive; diff row types defined by the `list_delta` macro have no Rust struct and are judged JSON against twins only; the `mutation` tag member of a leaf payload is the envelope tag and is not a Rust field.

## 3. Result
Before: 1846 mismatches (en1990 178, en1991 126, en1992 195, en1993 341, en1994 125, en1995 51, en1996 50, en1997 233, en1998 237, en1999 272, shared 38; by twin: ts-reader 555, GraphQL 423, proto 318, ts 302, Rust-vs-JSON 248). Main classes: 1352 optional/required (diff roots required in twins, optional in JSON; Rust `Option` fields required in JSON), 154 type (enum members stale, e.g. en1996 `designSituation` still listing `Seismic`, untyped `{"type":"object"}` list items where Rust has records, en1995 `cutout face` enum), 98 field sets (proto/GraphQL of en1990 `PermanentAction.kind/gk`, en1991 `tMax..bridgeLoadGroup`, en1992 `PrestressSteel`, en1993 `SteelMaterial`, en1990 artifact `structureKind/altitudeM/kFiDeclared`), 98 integer bounds (JSON without `maximum` for u8/u32, TS readers without `normWireRange`), 29 field order (en1991 snapshot JSON order differed from the Rust struct, en1997 `Pile` proto, compliance `InputSpec`), 104 extra twin types (en1996 `En1996Wall`, en1997 `SoilLayer`, `CaseGeometry`), 10 syntax (the diff readers `En1991BridgeLaneDelta`, `ConstructionStageDelta`, `RoofSurfaceDelta`... had one `)` too many per member, i.e. an invalid TypeScript file).
After: **0 mismatches** (`python3 -I 🐍️check.py`), second sync pass changes 0 files. 463 files written: 297 `🟦️.ts`, 99 `🔣️.json`, 42 `🔗️.graphql`, 25 `🛰️.proto` (en1990..en1999 and the shared `⚖️compliance/🧰️design-case` and `🪟️viewport` schemas). 297 changed TS files pass `node --check`; all changed JSON parse; brace/paren balance of proto/GraphQL checked.

## 4. Decisions worth knowing
- `required` follows Rust, not `serde(default)`: a non-`Option` field is required in JSON, TS, proto and GraphQL (what `Serialize` always writes). Diff delta/patch rows (macro types) get `required` = all members, the way their twins already read.
- Option scalars become nullable (`["integer","null"]`); TS keeps `| null`.
- `en1991` snapshot JSON lost `requiredDeltaT` (not in the Rust model any more) and reordered to the struct order; the twins followed. Snapshots of en1990 `PermanentAction` gained `kind`/`gk` in proto/GraphQL.
- en1997 and other roots whose list items were untyped objects in JSON now carry typed `$defs` (SoilLayer, ...) and TS/proto interfaces for them.

## 5. Open
- Owners who change a Rust record must now run `🐍️sync.py --apply --artifact enXXXX` (or the wire test fails with the exact field). Not rebuilt: TS consumers of the twins that relied on the stale optionality (a diff or snapshot field that became required/optional) will show in the next `tsc`/bun run; Rust is untouched so cargo sees no change.
- `repo ./📜️script.ts schema generate --check` was started at the end to see whether the scope catalog is still current (result in section 6).

## 6. Catalog check
`schema generate --check` reported the scope catalog stale (new scopes from the fleet, not from field lists); `schema generate` was re-run to refresh it (derived file, no hand edit).

## Files
Created: `✏️s/🔌️plugins/📕️norm/🧪️tests/🧬️schema-sync/{🐍️check.py,🐍️sync.py,🟦️.ts}`, `🧰️scripts/schema-sync/{check.py,sync.py}`, this report. Edited: the 463 schema files above; `📌️important/requests.md` (announce); appended `- ready for rebuild: schema sync` to `🗑️generated/build-en1990..en1999/latest.md`.
