# Localize B2 Hardcoded English (follow-up)

## Changes (`🖥️app-surface`)

| Chrome | Fix |
|--------|-----|
| Inspection `"Id"` | `chrome("Id", "Kennung", locale)` |
| Null leaf display | `chrome("empty", "leer", locale)` (wire `"null"` in `value_arg_json` unchanged) |
| `Unknown body: …` | `render_unknown_body(body_key, locale)` → `chrome("Unknown body", "Unbekannter Inhalt", locale)` |
| `Model` / `Report` | `LocalizedLabel::native` → packed into `MediaPortSpec.label` (`String`) as `en / de` via `media_port_label` |

## `NormFieldMeta` choices

`choices: Option<&'static [NormFieldChoice]>` where each choice has `value`, `label_en`, `label_de`. Documented in `📓️impl-b2-app-surface.md`. Families must supply both labels (raw-code-as-label alone is not enough).

## Test

`de_locale_inputs_and_inspection_omit_hardcoded_english_chrome` — inputs + inspection + unknown body in `Locale::De`; asserts German chrome and rejects English chrome payloads.

## Runner

`bun nx run @semio-tech/norm-artifact-contract-rs:test` → **46 executed, 46 passed, 0 failed**.
