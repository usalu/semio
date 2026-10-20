# Audit: `📚️compiler` path and `retainedCompilerStores` (aud-compiler-1020e)

Read-only audit. No builds, no `bun repo`, no modifying git commands. Labels: VERIFIED-READ (checked by command or file read in this audit), INFERRED (reasoned, not reproduced).

## Answers

1. `🧰️framework/📚️compiler` never existed in tracked history and is not in the live tree. VERIFIED-READ. `git log --all` over every path under `🧰️framework/` finds no path whose parent is `🧰️framework/` and whose name matches `*compiler*`. The live tree has only `🧰️framework/🔨️modules/📚️compiler` (the module, moved there long before this ticket) and `🧰️framework/📦️packages/🟦️typescript/📚️compiler` (a package-level SDK facade).
2. The spelling in result-conc-deps.md line 37 most likely drops either `📦️packages/🟦️typescript/` or `🔨️modules/` from a real path. INFERRED. The suites that result file names (generation, ingress, parser) contain no `📚️compiler` reference at all, and their relative import closure (251 files) resolves in the live tree. The "module missing" blocker was not reproduced statically.
3. No live file references the literal `🧰️framework/📚️compiler`. VERIFIED-READ (`git grep -F`; the only hit is ticket coordination.md line 456).
4. `retainedCompilerStores` is defined and used only in the live compiler store file `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts` (lines 55, 61, 64). VERIFIED-READ.
5. The birth failure is a name-extraction gap, not a missing definition. The birth test extracts only the `withRepositoryCompilerStoreV1` function text and runs it with a hand-built `ports` object (birth test lines 53-58). Module-scope symbols the function needs (`retainedCompilerStores`, `retainedCompilerKeyV1`) are not in `ports`. VERIFIED-READ.
6. The retained-store change exists only in the index (staged `M `). HEAD has 0 occurrences of `retainedCompilerStores` in the store file, and the index has 3. So the birth failure appears only with the staged store change. VERIFIED-READ (`git show HEAD:` vs `git show :`).

## Path history (VERIFIED-READ)

- `🧰️framework/🔨️modules/📚️compiler`: first commit `704df482ad4` (tag 🚩428); last commit `bfe5c8314f5` (🚩698). Renames inside it are R100 or R9x moves within the module, for example `fe7c8a8f8bd` and `67fb4216b2c` (fonts), `21fbcd3538f` and `79da34bb408` (`component.rs` to `.rs`). No commit moves the module root.
- `🧰️framework/📦️packages/🟦️typescript/📚️compiler`: added as new files in `36fbe9da786` (🚩691), modified in `7880cc36f78` (🚩693) and `8f91299152c` (🚩695). These two files are SDK re-export shims. `module-tree` re-exports `@semio-tech/framework-compiler-ts/syntax/module-tree`. `rust` re-exports `@semio-tech/compiler-syntax-rust-ts`.
- Package wiring: `🧰️framework/📦️packages/🟦️typescript/package.json` lines 62-63 map `./compiler/syntax/module-tree` and `./compiler/syntax/rust` to `./📚️compiler/...`. That is relative to the package root `📦️packages/🟦️typescript`, which is the likely origin of a truncated `🧰️framework/📚️compiler` spelling.
- Workspace links: `.cache/b/d/m/j/b719b3e90ef2b405198b/w/0/node_modules/@semio-tech/framework-compiler-ts` and `compiler-syntax-rust-ts` are symlinks to `🔨️modules/📚️compiler/📦️packages/🟦️typescript` and `🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript`. Both targets exist. VERIFIED-READ.
- Scope note: `🧰️framework/🔨️modules/📚️library` does not exist. The build library is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

## Live references to check (VERIFIED-READ)

- Compiler import specifiers across `🧰️framework` (git-tracked, relative, from/import/require): 58. All resolve. 55 target `🔨️modules/📚️compiler`, 3 target `📦️packages/🟦️typescript/📚️compiler`. Zero target `🧰️framework/📚️compiler`.
- Repo-root string paths containing `🧰️framework/…📚️compiler` (non-glob, 54): all exist. Targets are under `🔨️modules/📚️compiler` only.
- `retainedCompilerStores` in tracked files: live store file lines 55 (use), 61 (definition), 64 (use in `withRetainedRepositoryCompilerStoreV1`). Ticket-local copies exist in `ingress-perf-tools/patches.py` and `integrate-tools/final2/D/0014.ts`; they are not live.
- Name-based extractors of `withRepositoryCompilerStoreV1`: only the birth test (`📥️caller/🧪️tests/📥️birth/🟦️.ts`, line 54). Other hits are test titles or import lines.
- Suites named in result-conc-deps.md (regress.sh list, `$D/🧾️generation/🧪️tests/🟦️.ts`, `$D/🧾️generation/🧪️tests/🚪️ingress/🟦️.ts`, `$L/🏗️build/🚀️bootstrap/📦️parsers/🧪️tests/🟦️.ts`): no `📚️compiler` or `framework/📚️compiler` string. Their relative import closure (depth 5, 251 files) has no missing compiler target. The two "missing" items there are not real: `./actual-source` is a string passed to `typescript.preProcessFile`, and `./.semio-captured-module` is an esbuild virtual specifier.

## Exact edits

E1 (required, birth test). File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🧪️tests/📥️birth/🟦️.ts`
- Add an import at the top: `import {AsyncLocalStorage} from "node:async_hooks";` (not present today).
- Line 57, inside the ports object of the test at line 53: replace `compilerStoreOwners:owners};` with `compilerStoreOwners:owners,retainedCompilerStores:new AsyncLocalStorage(),retainedCompilerKeyV1:(workspace:string,mode:string)=>workspace+"\0"+mode};`
- Result: the extracted function sees the same two module-scope names the live file defines (line 61). With `getStore()` undefined, the retained branch is skipped and the test takes the fresh path it already asserts.
- Alternative if the test should avoid Node imports: `retainedCompilerStores:{getStore:()=>undefined}`.

E2 (no edit). `🏗️compiler/🟦️.ts` lines 55, 61, 64 are correct as written. Do not move the symbol unless the birth test is also reworked, because any module-scope dependency of an extracted function must be supplied through `ports`.

E3 (no edit to live files). If any consumer uses the old spelling, repoint it as follows:
- file-level: `🧰️framework/📚️compiler/` becomes `🧰️framework/🔨️modules/📚️compiler/`.
- package-level specifiers: keep `@semio-tech/framework/compiler/syntax/{module-tree,rust}`. They resolve through `📦️packages/🟦️typescript/package.json` lines 62-63 to `📦️packages/🟦️typescript/📚️compiler/` shims.
- relative imports from the live tree already use the correct depth.

Commit note: E1 and the staged store change must go in together. At HEAD the birth test passes its ports without the retained names, because HEAD has no retained branch.

## Open (INFERRED, not reproduced)

- The "module `🧰️framework/📚️compiler` missing" cause in result-conc-deps.md line 37 is unverified. Static evidence says no live file requests that path. A single live run of the generation suite (`bun test` on `$D/🧾️generation/🧪️tests/🟦️.ts`, run by the lead, not by this audit) would show the real resolution error. Check the mirror tree `/private/tmp/conc-deps/m` first, because a symlink mirror that omits `📦️packages/🟦️typescript/📚️compiler` would produce that error without any live reference.

## Read-only commands used

`git log --all/--oneline/--name-status/--diff-filter=D -- <path>`, `git grep -n/-F/-l`, `git show HEAD:<path>` and `git show :<path>`, `git diff --cached`, `git ls-files`, `readlink`, `ls`, and python resolvers over `git grep` output. Two temporary lists were written to the session scratchpad, not the ticket.
