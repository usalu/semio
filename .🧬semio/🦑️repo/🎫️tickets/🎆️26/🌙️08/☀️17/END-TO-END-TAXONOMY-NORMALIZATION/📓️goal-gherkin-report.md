# Gherkin description-prose backtick scanner — Report

## Root cause, verified (not re-derived)

`gherkinTokens` only ran `structuralTokensInFragment` against the old
`🏅️standards/🔖️X/🪆️subsets/✳️Y/🧬️mutations/…` mutation-test-path grammar — it had no generic
inline-code scanner, so any bare asset path written as Markdown-style prose in a `Feature:`
description fell through to `unsupportedReferenceTokens`'s whole-file fallback scan and could never
become a rewritable token (`resolveReferenceTokenPath` requires literal span coverage by a
*supported* token at that exact offset; a different occurrence of the same value elsewhere in the
file does not count).

Checking the 7 rows individually: **5 of 7 were already backtick-quoted** in description prose
exactly as diagnosed (svg-basic ×2, svg-tiny, svg, semio). **2 of 7 (ply line 7, obj line 7) were
bare, unquoted prose** — confirmed with `xxd` (no `` ` `` byte before the token). Per this ticket's
own no-loose-regex/no-migration-script rules, the fix does not special-case bare prose: the two
fixture lines were hand-edited to wrap the token in backticks (matching the sibling files'
convention), and the scanner stays a strict inline-code-span scanner.

## Change

**`🧹️normalization/🟦️.ts`** — added `gherkinDescriptionInlineCodeSpans(content)`, mirroring
`markdownSourceCoordinateSpans`'s per-line backtick-run discipline (escaped backticks, longest-match
same-length nested runs, no span crossing a line, blank-line paragraph reset) but scoped to the
region between the `Feature:` header and the first tag/`Background`/`Scenario`/`Rule` keyword line —
never inside a step. Only single-backtick runs are reported (double/triple-backtick spans are
consumed but not reported, matching the Markdown scanner). `gherkinTokens` now merges these spans in
as plain `adapter: "gherkin"` tokens (no `rewriteKind` needed — bare full-repo-relative values
already resolve and rewrite correctly through the existing generic path, the same way the old
structural tokens' fully-qualified targets do).

**Fixtures** — `☁️ply/🧪️tests/mutate-ply-1-0/🥒️.feature:7` and
`🧊️obj/🧪️tests/mutate-obj-3-0/🥒️.feature:7`: wrapped the bare `…/🖼️images/🧊️pattern-sphere.glb`
token in backticks (pure prose formatting, zero effect on Cucumber parsing).

## Test (fails before, passes after — verified both directions)

New `🧪️tests/🧪️gherkin-description-inline-code/` (`🟦️.ts`, `🔣️.json`, `🧬️schema/🔣️.json`), registered
in `📋️project.json`, `📜️script.ts`, and both launch catalogs. Extracts the real
`gherkinDescriptionInlineCodeSpans` (a pure, dependency-free closure) from the actual source via
`ts.createSourceFile`, runs it through two independent compilers (Bun transpiler, `tsc`), validates
against a 12-case JSON-Schema vector (escaped backtick, unclosed run, cross-line non-recognition,
blank-line reset, double-backtick non-report, tag/`Background`/`Scenario` boundary exclusion, tags
before `Feature:`, no-`Feature:` file) plus the 6 real fixture files, and cross-checks the two
simplest cases against `markdown-it`'s own `code_inline` tokenizer as an independent third-party
oracle. A separate AST-level test asserts `gherkinTokens` actually calls the new function.

Verified live, not asserted from memory: commented out the call site in `gherkinTokens` → 4/8 tests
failed exactly where expected (extraction, oracle, real-fixture, wiring-source tests); restored →
8/8 pass. Separately stubbed `gherkinDescriptionInlineCodeSpans` to `return []` → same 4 failed;
restored → 8/8 pass. `bun ./📜️script.ts test gherkin-description-inline-code` (from
`📦️packages/🟦️typescript`): **8 pass, 0 fail, 84 expect() calls**. `tsc --noEmit` clean.

## CLI verification (real, pasted)

```
B=bb06c41f73f0122fbed315b7487428b976f99921
bun ./📜️script.ts clean taxonomy plan --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION \
  --scope "🧰️framework/🔨️modules/🖼️assets" --baseline "$B" --plan .../🗑️temp/🔣️gherkin-plan.json --workers 6
```

| | before | after |
|---|---:|---:|
| `moves` | 1089 | 1089 |
| `unresolved` | 16 | 9 |
| `edits` | 40 | 47 |

The 7 `gherkin unsupported-path-syntax` rows are gone; the 9 remaining rows are unchanged,
pre-existing, unrelated classes (`python`/`markdown`/`rust`/`typescript` unsupported-path-syntax and
one `rust-path-join`). A later confirmation run showed `unresolved=7` (2 fewer, from a concurrent
peer edit to `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md` unrelated to this slice) — gherkin rows still 0.

**Confirmed rewrite, not just non-detection** — diffed `edits` between the before/after plans: all 7
new edits are `structuredLocation: gherkin-description-inline-code`, one per target `.feature` file,
each with a real `oldValue`/`newValue` pair, e.g.:

```
🎨️svg/🧪️tests/mutate-svg-1-1-tiny/🥒️.feature | gherkin-description-inline-code:7:18@329 |
  🧰️framework/🔨️modules/🖼️assets/🪧️logos/🔣️qr-code.svg -> 🧰️framework/🔨️modules/🖼️assets/🪧️logos/🖼️qr-code/🔣️.svg
☁️ply/🧪️tests/mutate-ply-1-0/🥒️.feature | gherkin-description-inline-code:7:4@259 |
  🧰️framework/🔨️modules/🖼️assets/🖼️images/🧊️pattern-sphere.glb -> 🧰️framework/🔨️modules/🖼️assets/🖼️images/🖼️pattern-sphere/🧊️.glb
```

All 7 rows disappeared because they became rewritable — not because detection stopped (they still
show up as plain `adapter: "gherkin"` tokens in `gherkinTokens`, just no longer unsupported).

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` (real fix)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🧪️tests/mutate-ply-1-0/🥒️.feature` (backtick-wrap, line 7)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🧪️tests/mutate-obj-3-0/🥒️.feature` (backtick-wrap, line 7)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️gherkin-description-inline-code/🟦️.ts`,
  `🔣️.json`, `🧬️schema/🔣️.json` (new)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`,
  `📜️script.ts` (new test wired in)
- `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json` (new launch entry, order `410.2153`)
- This file.
- `🗑️temp/` scratch (sanity/verify probes, plan-run JSON artifacts): deleted after use.
