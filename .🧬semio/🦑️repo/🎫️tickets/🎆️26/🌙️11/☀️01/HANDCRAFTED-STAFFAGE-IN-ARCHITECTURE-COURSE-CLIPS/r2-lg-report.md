# r2-lg: no default language in the cast exporter

## Finding
`V/🧍️staffage/🐍️.py:275` (`ensure_casts`) passed `"--lang", "en"`. Casts print no text: `render()` switches off dimensions, scale bar, legend, labels, ground and free-body, and every text path in `depict` (labels, titles, legend, transport tags/emitter labels, dimension) is gated by those flags. The SVG `<title>` is the bilingual `en / de` from asset metadata. So a cast's output is language-independent. Language was only required by the type of `defaultDepictionOptions`, and by the localized stderr progress label.

## Design chosen
Language-independent casts, removed end to end (no fallback default, not in the cache key).

- Exporter `S/🔨️modules/📤️export/🎭️cast/🟦️.ts`: removed `--lang`, `CastOptions.language`, per-cast language and `LANGUAGES` import. `parseArguments` now rejects unknown flags (so `--lang` is refused). Progress line is bilingual `[done/total] Rendering cast frames / Besetzungsbilder werden gerendert`. `depict` still insists on a `Language`, so a single exported `TEXTLESS_LANGUAGE` constant (documented as never read) is passed; the no-text test proves it.
- Schema: `🧬️schema/🔣️.schema.json` and `🧬️schema/🟦️.ts`: `language` property, key, check and type removed (now `unknown property language`).
- Python `V/🧍️staffage/🐍️.py:275`: `--lang en` removed.
- Docs: `S/README.md:61`, `T/contract.md` (cast text rule, command lines, `--ids` line, `ensure_casts`, open-issue line removed).
- Fixtures: `cast-export` (removed `language`, invalid case now "language on a cast"), `cast-cancellation` (removed `language`, bilingual expected progress). Feature file updated.

## Tests
- Vitest `cast-export`: 56 passed, 1 skipped (new: `--lang` refused, no-`<text>` in any frame of any cast, `--out` required).
- Python `🧍️staffage.🧪️tests.🐍️`: 36 OK (new: `ensure_casts` command has no `--lang`; jsonschema rejects `language`).
- `staffage_only` 20 OK, `scenes` 11 OK, `integration` 9 OK.

## Notes / open
- The exporter source digest changed, so all cast cache keys changed (casts re-rendered once).
- The `[DEBUG] pf` hook was not touched.
- `TEXTLESS_LANGUAGE = "en"` remains an internal constant because `DepictionOptions.language` is required by the shared depiction library; making it optional would touch seven view modules and was out of scope.
- The general export CLI (`🎬️cli`) legitimately keeps `--lang` (it prints text).
- Ticket scripts `lg_sed.sh`, `lg_sed2.sh` kept in `T/`.
