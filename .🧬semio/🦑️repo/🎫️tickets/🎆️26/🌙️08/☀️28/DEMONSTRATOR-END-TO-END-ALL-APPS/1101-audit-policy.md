STATUS: AUDIT DONE (aud-policy, 11/01). Gate NOT CLEAN: 2 `#[allow]` + 11 `[DEBUG]` prints (1 in production code) + 594 unwrap/expect/panic lines in 223 non-test Rust files + 1 hardcoded production default language, plus 6 review items. Comments-in-body, migration scripts and shim/adapter code: no violations found.

# 1101 Policy Audit (aud-policy)

## Scope and method

- Scope: `🧰️framework`, `✏️s`, `🌎️hub`, `♻️mit-bestand`. Changes since the last commit before 2026-10-30 (`1c319e98135`, 10/29 12:29) through the working tree (includes the 3 commits of 10/29 to 11/01 and all uncommitted work). 6158 changed files, 3226 of them code files (rs, ts, tsx, mts, mjs, cjs, js, py, sh, go, wit).
- Only added lines were scanned (`git diff 1c319e98135 -U0`, read-only). Findings are file:line in the working tree.
- Test code is excluded from the unwrap rule: path segments `tests`/`🧪️tests`, and lines after the first `#[cfg(test)]` in a file. Test code IS included for the `[DEBUG]` rule, because temporary logs must be removed everywhere.
- Scanner: `1101-tools/aud-policy/scan.py`. Raw data: `🗑️generated/1101/aud-policy-findings.json`, `aud-policy-diff.txt`, `aud-policy-unwrap-rows.txt` (all 594 file:line rows).
- Not counted as violations: `Original*` naming (in this codebase "original" is the caller's source value, e.g. `RetainedOriginalSlotKey`, not a legacy copy); explicit `"en"`/`"de"` passed as arguments in tests; `"en" | "de"` union types (a supported set, not a default); `adapter` used as a domain term (e.g. `FlowDomainAdapter`, "editor adapter"); `legacy VML` (ECMA-376 domain term); `pdfjs-dist/legacy` import path (third-party API).

## Summary

| Rule (AGENTS.md) | Hard violations | Review items | Status |
|---|---|---|---|
| `[DEBUG]` prints left behind | 11 lines (1 in production code) | 0 | FAIL |
| `#[allow]` added | 3 lines | 0 | FAIL |
| `.unwrap()` / `.expect()` / `panic!` in non-test Rust | 594 lines, 223 files (566 in 🧰️framework, 28 in ✏️s) | 0 | FAIL |
| Comments inside definitions | 0 | 0 | PASS |
| Shims / adapters / compat layers / legacy support | 0 | 6 | REVIEW |
| Migration scripts in source tree | 0 | 0 | PASS |
| Hardcoded default language | 1 production (+ 1 locked-locale design item, + 3 test fallbacks) | 1 | FAIL |

## 1. `[DEBUG]` prints left behind

| # | file:line | rule | suggested fix |
|---|---|---|---|
| 1 | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🔺️diff/🫴️receiving/🧪️tests/🦀️.rs:50` | `eprintln!("[DEBUG] original recipient refused ...")` left in test | delete the `eprintln!`; keep the assertion |
| 2 | same file `:53` | `eprintln!("[DEBUG] central retained receiving ...")` | delete |
| 3 | `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🌳️root/🧪️tests/🌳️root/🦀️.rs:34` | `eprintln!("[DEBUG] drain bytes=...")` | delete |
| 4 | `🧰️framework/🔨️modules/🧵️job/⚠️failure/🧪️tests/🦀️.rs:59` | `assert!(turn<99999,"[DEBUG] original failure retirement blocked ...")` | rename message to a plain assertion message, or delete the `[DEBUG]` prefix and keep only if it is a permanent diagnostic |
| 5 | same file `:74` | same assert message | same |
| 6 | same file `:91` | same assert message | same |
| 7 | same file `:112` | same assert message | same |
| 8 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts:130` | PRODUCTION: `console.error("[DEBUG] Authorizer required Source read failed", text)` shipped in the authorizer path | delete the line. If the diagnostic is needed, return it as a typed failure value that callers render, not a stderr print |
| 9 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧭️ancestry/🟦️.ts:66` | test searches the console messages for the `[DEBUG]` text from #8 | fix together with #8: assert on the typed failure instead |
| 10 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📡️request/📜️script.ts:18` | `console.error("[DEBUG] " + JSON.stringify(...))` twice in one line | delete both |
| 11 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📡️request/🟦️.ts:20` | `console.log("[DEBUG] " + JSON.stringify(observed.inventory))` | delete |

## 2. `#[allow]` added

| # | file:line | rule | suggested fix |
|---|---|---|---|
| 1 | `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs:209` | `#[allow(clippy::too_many_arguments)]` | group the arguments into a named schema-backed struct, remove the attribute |
| 2 | `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs:533` | `#[allow(clippy::too_many_arguments)]` | same |
| 3 | `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts:53` | `#![allow(async_fn_in_trait)]` inside the generated Rust source string | make the trait method return an explicit `impl Future<Output = ...> + Send` instead, remove the attribute from the generator template |

## 3. `.unwrap()` / `.expect()` / `panic!` in non-test Rust

594 lines in 223 files. Full list with file:line: `🗑️generated/1101/aud-policy-unwrap-rows.txt`. Largest concentrations (line counts in brackets):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️command/🛬️receiving/🪜️cursor/🦀️.rs` (24: lines 55, 60, 64, 70, 75, 78, 83, 85, 92, 94-96, 100, 103, 104, 110, 113-115, 141, 164, 165, 190, 196)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs` (344, 352, 381, 396, 399, 594)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🧳️replacement/🦀️.rs` (39, 41)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🔁️mutations-expansion/🦀️.rs:287` (`panic!` in a derive expansion)
- `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs` (14), `🧰️framework/🔨️modules/◻️2d/🔍️trace/🦀️.rs` (10), `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🧱️bulk/🦀️.rs` (9), `🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/🧳️receiving/🦀️.rs` (9)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️prepared/♻️retirement/🦀️.rs` (59, 60, 64)

Suggested fix (all rows): propagate with `?` into the owning crate's typed error (`Result`/`RetainedWorkOutcome`), or make the invariant a total type (e.g. `let Some(x) = ... else { return Err(...) }`, a non-optional field, or an enum variant carrying the data). For `Option` state machines (`self.row.as_mut().unwrap()` and similar), move the payload into the state variant so the unwrap disappears. Do not replace them with `expect("...")`. Test-module code after `#[cfg(test)]` is excluded.

## 4. Comments inside definitions

PASS. 0 added indented `//` comment lines, 0 trailing `//` after code (the single match is a string literal), 0 added `/* */` inside bodies. Region markers `// #region` / `// #endregion` sit at column 0 and are not body comments.

## 5. Shims, adapters, compat layers, legacy support

No compat shim or adapter layer was added. Review items:

| # | file:line | rule | suggested fix |
|---|---|---|---|
| 1 | `🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🤝️kerning/🟦️.ts:32` and `:81` | `private legacyReader` field and reset; likely a second reader kept for old font data | confirm it is not a fallback path; if it is, delete it and the branch that uses it |
| 2 | `🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🤝️kerning/🦀️.rs:53` | `11\|12=>"legacy"` table kind | confirm the kind is a real format variant; if it only exists for old data, remove it |
| 3 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/🟦️.ts:318` and `:390` | `if(vector.mode==="legacy") input.version=1` tests that the product reads version-1 input | AGENTS forbids legacy support: remove the legacy vector mode and the product branch that reads version 1 |
| 4 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:196` | doc: "falls back to `WorldSunConfig::default()` on any malformed/legacy value" | a silent fallback for a legacy value is legacy support: reject the value with a typed error instead |
| 5 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🏛️reserved/🦀️.rs:885` | doc: "generated migration join" | doc only; confirm no migration code exists behind it |
| 6 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🏛️reserved/🦀️.rs:835` and `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:90` | `Migrated` enum variant / "without materializing a whole-buffer adapter" | check that `Migrated` is not a legacy-classification shim; rename if it describes old jobs |

## 6. Migration scripts in the source tree

PASS. No added file is named like a migration, codemod, backfill, fixup, upgrade or rewrite, and no added line contains migration code. The new `.py` and `.mjs` files in the closure are normal source or tests.

## 7. Hardcoded default language

| # | file:line | rule | suggested fix |
|---|---|---|---|
| 1 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📏️grow/🟦️.ts:131` | PRODUCTION: `const language=request.language??"en"` | make `language` required in the request type and drop the `??"en"` default |
| 2 | `♻️mit-bestand/🧺️demonstrator/🔨️modules/🌐️locale/🔣️.json:3` and `♻️mit-bestand/🧺️demonstrator/🔨️modules/🌐️locale/🟦️.ts:17` | `"lockedLocale": "de"` and `DEMONSTRATOR_LOCKED_LOCALE`: the whole demonstrator is locked to one locale, which is a default in effect | review with the owner: AGENTS requires multi-language support with no default language. If the lock is intended for this demonstrator, make it an explicit per-session choice rather than a constant |
| 3 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🌳️owned-locale-detector-retirement-and-more/🟦️.tsx:28` | test: `detectShellLocale(...) ?? "en"` | pass the language explicitly, or assert the detection result without fallback |
| 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎞️timeline-paint-vfs-hosts/🟦️.ts:471` | test: `?? "en"` | same |
| 5 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️block-list-presentation/🟦️.tsx:33` | test: `?? "en"` | same |

Explicit test arguments (`language:"en"`, `SEMIO_LANGUAGE:"en"`, `locale="en"`) in the course, cargo and cache tests are not defaults and are not counted.

## Next steps

1. Delete the 11 `[DEBUG]` lines (#8 needs a typed failure instead of a print, and its test #9 must change with it).
2. Remove the 2 `#[allow(clippy::too_many_arguments)]` by introducing argument structs, and the generated `#![allow(async_fn_in_trait)]`.
3. Remove the `??"en"` default in `grow/🟦️.ts:131`.
4. Triage the 594 unwrap/expect/panic rows by owning crate, starting with `🪜️cursor`, `🧬️snapshot-clone`, `🧳️replacement` and the `mutations-expansion` derive.
5. Owner decision needed on the `lockedLocale` design item and on the 6 review items.
