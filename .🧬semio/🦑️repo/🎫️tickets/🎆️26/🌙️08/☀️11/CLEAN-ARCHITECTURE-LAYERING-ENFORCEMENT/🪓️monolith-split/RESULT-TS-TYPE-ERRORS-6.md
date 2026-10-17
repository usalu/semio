# Result: TS-TYPE-ERRORS-6 (prefix `tt6`)

Executor: Sonnet, started 2026-10-17 20:10. Foreground, no sub-agents, no modifying git command, no ticket tool. Logs `🗑️generated/tt6-*`, tools `tt6-tools/`. Continues `RESULT-TS-TYPE-ERRORS-5.md`.

## 0. Verdict

IN PROGRESS (rewritten at the end).

## 1. Coordinator addition: whole-renderer tsc 7 -> 73

Cause: not a code defect, a race. The `c4pp` type-check `c4pp-tsc-after.txt` ended 19:44:58; the peer carve of `🧰️framework/🔨️modules/📇️directory/🧬️schema/🟦️.ts` created the 11 child folders (`🌊️stream`, `🎫️command`, `🎯️execution-target`, `👀️views`, `📂️document-open`, `📄️event-page`, `📋️admin-page`, `📜️event`, `🛠️admin`, `🧊️artifact-checkpoints`, `🪢️checkpoint-pair`) at 19:43:15 and their `🟦️.ts` at 19:43:26, so tsc loaded the importers (directory root, `🏘️spaces`, `🚪️io/📝️text`, `📞️directory-client`, `🛰️backbone`, the `📦️packages` directory facade) before the children existed (TS2307 x45 and the implicit-any TS7006 that follow from unresolved types). Re-run now (20:25, `tt6-ren.json`, the same tsconfig as `c4pp-tsconfig.json`): 4 diagnostics, all TS7016 (jsdom 2, lodash-es 1, graphlib 1 because that program does not include `🌿️ambient`), the 59 `📇️directory` rows and the 9 `SpaceAdministration` / `directory-client` / `catalog` rows are gone. An import-resolution scan over every relative import in the directory module also finds 0 unresolved paths.

## 2. Ruling 1: swapped matcher sites (restored, typed from the expected side)

Method (`tt6-tools/tt6_swaps.mjs`, TypeScript AST): every `expect(A).matcher(B)` call of the working tree is compared with the same file at HEAD (2026-10-16, before TT4/TT5). A call is a swap when HEAD has `expect(B).matcher(A)`, a matcher change when HEAD has the same A and B with another matcher (`toEqual` -> `toMatchObject`). Scanned: the 163 files TT5 listed plus every test file that differs from HEAD (2,141 files; 258 have no HEAD version because a split created them).

- Found 71 swapped sites in 45 files (all restored to the HEAD order by `--apply`) and 3 matcher changes. A second population TT5 hid behind spreads or helper variables (`expect(row.expected).toEqual([...actual])`, `expect(fixture.lanes).toEqual([...LANES])`) was found with a token-similarity pass and restored by hand (about 20 sites).
- The 16 mutual `expect(A).toMatchObject(B); expect(B).toMatchObject(A)` pairs (a weaker, direction-free stand-in for `toEqual`) in 13 files became `expect<typeof B>(A).toEqual(B)` again (`tt6-tools/tt6_pairs.py`).
- Typing instead of swapping: the 74 type errors the restored order produces were fixed from the expected side by `tt6-tools/tt6_widen.mjs`: `expect<typeof expected>(actual).toEqual(expected)` (the actual value must be assignable to the type of the expected value, which the compiler checks; a non-name expected expression is hoisted into a `const` before its statement). Where the actual type is not assignable (a JSON fixture type with `string` against a literal-union domain type, `never[]` empty arrays, `readonly` tuples, `null` rows from `bun:sqlite`), the second `bun:test` overload is used: `expect(actual).toEqual<typeof expected>(expected)`; no `unknown`, no `any`, no cast. Two sites got explicit element unions (`expect<(string | undefined)[]>`).
- Order restored in all of them; a failure now prints received as the produced value and expected as the fixture again.

