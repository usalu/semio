# P2-FG2 jpg Registration Fix — Independent Verification

**Verifier**: subagent, invoked to independently confirm FG2's claimed jpg registration fix.
**Date**: 2026-08-11

## 1. Grep confirmation — real hits exist

`grep -rn "register_language\|LanguageSpec\|register_schema_spec" "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg"` against
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/⚙️engine/🦀️component.rs` returns real, non-report hits:

- `register_pilot_languages()` (line 1074) calls `dsl::register_language(dsl::LanguageSpec { .. })` **5 times** (lines 1075, 1083, 1091, 1099, 1107) — one per `dsl::LanguageRole`: `Document` (`stdio.jpg`), `Ops` (`stdio.jpg.op`), `Diff` (`stdio.jpg.diff`), `Pack` (`stdio.jpg.pack`), `Spr` (`stdio.jpg.spr`). Each `LanguageSpec` wires real grammar/protocol constants from `crate::artifacts::jpg::schema::{snapshot,mutations,diff}::{text,binary}::COMPONENT_GRAMMAR_SEMIO` / `COMPONENT_PROTOCOL_SEMIO` (not placeholders), plus `dsl::passthrough_hooks(id)`.
- Both `register_pilot_languages()` and `register_schema_specs()` are called from `pub fn register()` (lines 1038-1039), so they run on plugin init — not dead code.
- `register_schema_specs()` (line 1124) is an intentionally empty `{}` body, documented inline (lines 1117-1123) as deliberately not calling `register_schema_spec` (P2-M3's `FullResolver` API) because `JpgSnapshot`/`JpgDiff`/`JpgMutation` fail `#[derive(dsl::DslRecord)]` — cited compile error: `error[E0277]: the trait bound (u8, u8): DslField is not satisfied` on `jfif_version: (u8, u8)`, plus independently-blocked `JpgDiff`/`JpgMutation` (data-carrying enum / `Option<Option<T>>` fields with no `DslField` impl).

**Cross-check against sibling artifacts** — the same two-function split (`register_pilot_languages` with real `register_language` calls + a documented-empty `register_schema_specs`) is the established pattern repo-wide, confirmed present in 26 other stdio artifact engines (json, dxf, ifc, epw, bmp, ply, svg, obj, deflate, zip, step, txt, md, tiff, xml, binary/raw, png, dwg (both ac1018/ac1024), las, gif (87a/89a), gltf, tsv, csv, stl). jpg's fix matches this exact convention — not a fabricated or one-off pattern.

**Verdict: real hits confirmed.** `register_language`/`LanguageSpec` are genuinely wired (5 real registrations with real grammar/protocol refs), and `register_schema_spec` is genuinely and consistently-with-siblings deferred, not silently dropped.

## 2. Full crate test suite

Command: `cargo test -p semio-s-plugin-stdio --lib`

```
test result: ok. 1773 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 8.59s
```

Matches FG2's claimed baseline exactly: **1773 passed / 0 failed** (0 new failures, no regression).

## Result

| Check | Result |
|---|---|
| `register_language`/`LanguageSpec` real hits in jpg | Confirmed (5 real registrations, matches 26 sibling artifacts' convention) |
| `register_schema_spec` handling | Confirmed deliberately deferred, documented, consistent with siblings (not a gap left unaddressed) |
| Full crate test suite | 1773 passed, 0 failed, 1 ignored |
| Regression vs FG2 baseline | None |

**registration_confirmed: true**
