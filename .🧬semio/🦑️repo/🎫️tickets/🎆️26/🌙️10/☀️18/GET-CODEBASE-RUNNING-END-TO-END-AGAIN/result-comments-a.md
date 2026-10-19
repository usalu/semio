# Result comments-a: comments inside definition bodies, `🧰️framework/**` (without `📚️library/`)

Executor `comments-a` (foreground, no sub-agents, no git writes, no cargo, no dependency sync). `C` = this ticket folder. Input: `C/audit-agents-rules-1019.md` item 5 and the AGENTS.md rule "You MUST NOT comment inside definitions". HARD FREEZE respected: nothing under `📚️library/`, no root `📜️script.ts`, no `package.json`, no `🔣️taxonomy.json` touched (0 applied paths contain `📚️library`).

## Outcome

* 9,329 comments (9,511 lines: 4,905 TS/TSX lines in 4,723 comments, 4,606 Rust lines) removed from 1,042 files (14,666 candidate files scanned, 14,572 analysed). The audit's 4,439 lines were only the indented `//` heuristic over recently touched files; the tool covers every non-library framework source file and block comments too.
* Final re-scan of all 14,666 candidates after the run: only 22 lines in 2 files newly created by peers were left (applied right after, verified); everything else idempotent.

| Tree | Files | TS/TSX lines | Rust lines | Total |
| --- | ---: | ---: | ---: | ---: |
| `🧰️framework/🔨️modules` | 480 | 533 | 3,603 | 4,136 |
| `🧰️framework/🛍️products/💻️os` | 479 | 3,665 | 990 | 4,655 |
| `🧰️framework/🛍️products/🦑️repo` | 52 | 507 | 13 | 520 |
| `🧰️framework/🛍️products/🎤️presentation` | 4 | 21 | 0 | 21 |
| `🧰️framework/🛍️products/📓️print` | 4 | 10 | 0 | 10 |
| `🧰️framework/🧪️tests` | 2 | 13 | 0 | 13 |
| Sum of the six rows (first pass; the two retry passes add 156 lines in 21 files, mostly `🖱️ui` and the wgpu plugin loader) | 1,021 | 4,749 | 4,606 | 9,355 |

Per-module breakdown (top: renderer 2,976, ui 1,387, 3d 1,255, repo 520, dsl 454, db 267, plugin 231): `C/🗑️generated/comments-a/trees.md`.

## What was kept (rule counts over the final state, comments inside definition bodies)

| Rule | Kept |
| --- | ---: |
| Rust `///`, `//!`, `/** */`, `/*! */` doc comments (also inside bodies) | 32,668 |
| TS/JS JSDoc attached to a declaration, member or `it/test/describe` call | 2,326 |
| `// #region`, `// #endregion`, `🔖️Name` section markers | 939 |
| Rust `SAFETY` comments (also emoji-prefixed `// 🔓️ SAFETY:`) | 129 |
| Continuation lines of multi-line `SAFETY` blocks | 194 |
| Tool directives (`@ts-*`, `eslint-*`, `biome-ignore`, `prettier-ignore`, `istanbul`/`c8`/`v8`, `@vite-ignore`, `webpack*`, `@__PURE__`, `/// <reference`, `@license`, `@preserve`, `@jsx`, `sourceMappingURL`, `rustfmt::`, `clippy::`, ...) | 23 |
| Comments outside any definition body (top level, `mod` level, `const`/`static` initializers, module-level objects): untouched by scope | 132,231 |

`#[allow]`-style attributes are not comments and were never touched. Comments inside parse-error regions would be kept (0 occurred).

Skipped files: 78 by path (51 `jcoprobe/browser-bundles` vendored bundles, 2 `preview2-shim`, 22 under `🤖️` generated dirs, 3 whose directory name contains "generated" but which are hand-written: dry-run found 0 removals in them), 6 by generated-header heuristic (dry-run with the check off: 0 removals in all six), 3 Rust files using `line!()` (comment removal would shift reported lines): `🏃️process/📦️artifacts/🏗️native-build/📥️resources/🦀️.rs`, `.../📥️resources/🧮️compiler/🦀️.rs`, `🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🔄️interactive-state-sync/🦀️.rs` (not processed, so they may still hold body comments). 19 + 2 files edited by others in the last 15 minutes were skipped in the first pass and applied in two retry passes after the window closed.

## Decisions to review

* 1,642 of the removed Rust lines were `// 🚫️async: E1/U1/E4 ...` async-exemption justifications (first statement of a body). No code or gate reads them (grepped `.ts`/`.json`/`.toml`/`.rs` outside tickets), so they went out under the rule; if you want the exemption register kept, it must move to a docstring or a schema field. The full text of every removed comment is in `C/🗑️generated/comments-a/ap-*/apply-removals.jsonl` (file, line, text) and in git history.
* 35 `/** 🖱️ ... */` blocks that precede plain statements inside component bodies (`useEffect(...)`) were removed because they are not attached to a declaration; the same blocks before `it/test/describe/beforeEach/...` calls are treated as test-definition docstrings and kept.
* Blocks emptied by a removal collapse (`fn f() {\n}` becomes `fn f() {}`, `catch {\n}` becomes `catch {}`); a blank line next to a removed full-line comment is dropped when it would double up or hug a brace.

## Tool

`C/comments-a-tools/strip.ts` (Bun; TS via the `typescript` compiler API, Rust via `web-tree-sitter` + `tree-sitter-wasms`, as `C/rs-sweep-tools/parse.ts`). Modes `--mode dry|apply --list FILES --out DIR`. Per file it:

1. finds comments exactly (TS: token walk with leading and trailing trivia, JSX text and template literals aware; Rust: tree-sitter `line_comment`/`block_comment` nodes with an ancestor in `function_item`, `closure_expression`, `impl_item`, `trait_item`, `struct_item`, `enum_item`, `union_item`, `macro_definition`, `function_signature_item`; TS ancestors: functions, methods, constructors, accessors, arrows, static blocks, class/interface/enum/type-literal bodies),
2. classifies keep/remove by the rules above,
3. removes with whitespace and blank-line cleanup (JSX `{/* */}` containers removed whole),
4. proves equivalence before writing: TS compares the comment-free printed AST of old and new (empty JSX containers and whitespace-only JSX text normalised); Rust compares the full leaf-token sequence, the tree-sitter error count and the exact list of surviving comments. A mismatch skips the file (55 TS files mismatched on empty-block printing in the first design, then passed after normalising `{ }`),
5. re-stats the file immediately before the write and skips it if size/mtime changed, and skips files younger than 15 minutes.

Also in `C/comments-a-tools/`: `verify.ts` (per-file `bun build --no-bundle` / rustfmt syntax gate), `scope.ts` (touched Rust crate manifests via the rs-sweep module trees), `xref.ts`, `agg.ts`.

## Verification (exact commands, outcome)

* Dry-run over all 14,666 candidates, 3 shards: `bun C/comments-a-tools/strip.ts --mode dry --list cand.txt --offset N --limit 4900 --out dryN`. 50 removals sample-reviewed (seeded random): all prose rationale comments, no directive/doc/SAFETY. A second sample of 18 block and 22 trailing removals reviewed. The sample found two defects, fixed before applying: emoji-prefixed `// 🔓️ SAFETY:` comments (125 were about to go) and `/** */` docstrings before `it(...)` test definitions.
* Apply in six batches plus two retries (`--mode apply --list batch-*.txt`), 1,042 files, 0 write skips except the recent-edit ones.
* TS (`bun build --no-bundle FILE`, `C/comments-a-tools/verify.ts`): 89 + 57 + 329 + 19 files from the batches and retry, 0 failures.
* Rust (`rustfmt +nightly-2026-07-20 --edition 2021 --check --config skip_children=true,disable_all_formatting=true FILE`, same gate as rs-sweep): 391 + 5 + 150 + 2 files, 0 syntax errors.
* rs-sweep checker on the 220 touched crate manifests (`bun C/rs-sweep-tools/main.ts --scope scope.txt --tag base|after`): 136 findings before, 136 after, 0 new, 0 gone (`C/🗑️generated/comments-a/rssweep/findings-{base,after}.jsonl`).
* Cross-reference scan for tests or tools that read comment text from other files (first 36 chars of every removed comment against 3,091 source-reading files): only same-text duplicates in unrelated files, no consumer of a removed comment found.
* Not run: `cargo check`/`cargo test` (builds are serialized by the coordinator), `tsc`, vitest/`bun test` suites. The equivalence proofs cover token identity, but line-number-sensitive consumers (docs/tickets quoting `file.rs:123`) were not scanned.

## Open items

* The three `line!()` Rust files and any file edited by others after the retry passes (a rerun is `bun C/comments-a-tools/strip.ts --mode apply --list LIST --out DIR`; it is idempotent).
* Decision on the `🚫️async:` register (above).
* Comments outside bodies (132k, mostly module-level section comments and Rust `//` between top-level items) are out of scope of this task and unchanged.
* `C/🗑️generated/comments-a/` holds the audit trail (applied lists, removed-comment logs, rs-sweep findings, about 10 MB); delete when the coordinator no longer needs it.
