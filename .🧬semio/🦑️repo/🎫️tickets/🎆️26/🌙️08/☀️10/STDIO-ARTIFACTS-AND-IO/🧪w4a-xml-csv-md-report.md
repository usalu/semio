# W4a Report — Xml, Csv, Md Stdio Artifacts

## Scope
End-to-end stdio artifacts mirroring `💾️binary` / `📄txt` / `🔣️json` facet trees:

| Roster id | Dir | Neutral model | IO parent |
|-----------|-----|---------------|-----------|
| xml | `📰xml` | `XmlDocument` (elements, attrs, text nodes) | txt |
| csv | `📊️csv` | `TableDoc` (`headers`, `rows`) | txt |
| md | `📝️md` | `TextDoc` (`body`) | txt |

## Codecs
- **xml**: Hand-rolled well-formed XML parse/serialize on `XmlNode` / `XmlDocument`; DSL + pack (pack stores JSON-encoded document model inside semio envelope).
- **csv**: RFC4180-ish row parser (`csv_parse_row` / `csv_escape_row`); DSL body is CSV text; pack wraps UTF-8 CSV bytes.
- **md**: Lossless markdown-as-text (`body`); DSL + UTF-8 pack (same pattern as txt envelope).

## Wiring
- `📦️glue.rs`: `artifacts::{xml,csv,md}` modules (full schema / builder / decomposer / engine / io / examples).
- `🔌️plugin/🦀️component.rs`: `engine::register()` for xml, csv, md.
- `📦️packages/🟦️typescript/📦️index.ts`: exports `xml`, `csv`, `md`.

## Generators (ticket folder)
- `generators/w4a_scaffold.py` — copy json tree → xml/csv/md
- `generators/w4a_fix_codecs.py` — snapshot codecs, schema fields, IO txt bridges
- `generators/w4a_glue.py` — glue/plugin/index (glue brace fix applied manually)

## Examples
- `📰xml/.../example.xml` — small `<note>` document
- `📊️csv/.../example.csv` — `name,count` table
- `📝️md/.../example.md` — heading + bold line

## Verification
```text
cargo check -p semio-s-plugin-stdio  → Finished (dev)
cargo test -p semio-s-plugin-stdio --lib → SIGKILL during run (environment); check is the gate for this wave
```

## Out of scope (other agents)
- deflate / zip
- taxonomy / policy / framework edits
