# F65 result: `repo typecheck` and the `.d.mts` drift law

## What exists now
- `repo typecheck [--json] [--dry-run] [--no-importers] [--all-errors] [--declarations] [--] <path>...` (operations catalogue en/de, MCP tool `typecheck` generated from the catalogue + surface policy, `--lang`). Member: `B/🔬️typecheck/` (B = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`).
- How: paths -> touched sources (a `.mjs` stands for its `.d.mts`); direct importers from an import index (relative specifiers, kept in the machine cache, re-read only for files whose size/mtime changed); one `tsc -p` per project (library incl. process module, else workspace) over exactly those files through a generated tsconfig that `extends` the project's own, with `incremental` + a per-checkout `.tsbuildinfo` in the machine cache (`~/Library/Caches/semio/typecheck/<hash>`, `XDG_CACHE_HOME`, `LOCALAPPDATA`, override `SEMIO_TYPECHECK_CACHE`). Nothing is written in the checkout, no graph read, no new runtime dependency (the compiler is the repo's installed one, run by the current runtime). Importer cap 400 (else only the touched files, said in a notice).
- Failure rule: exit 1 on any error in a named file or importer, or any declaration drift; errors in other files the program reads are counted "elsewhere" and do not fail (`--all-errors` counts them); exit 2 refused (bad path, compiler missing, timeout 10 min, cancel).
- Drift law (`B/🔬️typecheck/🧾️declarations/🟦️.ts`): every `.mjs` export is declared in its `.d.mts`; no function/constant/class declared that the script lacks; same parameter count (rest counted). Runs on every touched pair inside `repo typecheck` and over all 222 pairs with `--declarations` (1.7 s, no compiler). Law test `DECLARATION DRIFT LAW` in the member's tests walks the whole library. One source of truth: `declarationStubsV1` derives declarations from the `.mjs` (barrel re-exports become `export {..} from "./x.mjs"`, so they carry the real sibling declaration; functions keep names and counts; values `any`). Generating the full declarations from JSDoc was not feasible: the scripts carry no JSDoc types and the 220 hand-written files hold precise types worth keeping.
- Cross-check against a third-party implementation: the scanner agrees with the TypeScript parser API (names, kinds, parameter counts) on every `.mjs`/`.d.mts` of the library and process module (551 files) - test "reads every script and declaration ... exactly as the TypeScript parser does".

## Drift fixed
193 findings in 36 files: 186 undeclared exports (stubs/re-exports appended under a marked docstring), 4 arity drifts (`resolveRowDoubtfulV1`, `rebaseRowsV1`, `projectWithDefaults`, `createFactSinkV1`), 3 phantom declarations removed (nx-plugin `📤️arguments`: the functions moved to `B/📤️arguments`). `repo typecheck --declarations` exits 0. Stubs typed `any` are placeholders to refine; no product file reports a type error through them.

## Timings (coordinator Mac, load 17-45 during the work)
- 1 file + 2 importers, warm: 1.4-2.4 s; first call: 3-11 s.
- A set whose closure is new (the tests of the typecheck member reach the whole process module): 86 s once, then 10 s when a second call adds files; 36 d.mts files with 63-196 importers: 9-38 s.
- `--declarations`: 1.3-1.7 s. `--dry-run`: 1.1 s (import index warm; cold index of 5.8k files 10-19 s under load).
- Whole library `tsc` (F63): about 3 min; `repo run @semio-tech/repo-lib:typecheck`: over 5 min waiting for the graph.

## Tests (repo test-file)
typecheck 18/0 (policy schema, texts en/de, args, path resolution, import keys, importer index, plan, config, cache dir per platform, diagnostics, drift cases, stubs, TS-API cross-check on 551 files, drift law, run semantics with injected compiler, command exit codes, one real compiler run in a scratch checkout); mcp build-surface 6/0; operations requires 3/0, help-offline 1/0, door 9/0; operations tests 15 pass, 1 red `cache gc --reclaim` (scoping fixture of another lane: `scopedSubcommands` cache gc lacks `--reclaim`). `⌨️command/🧪️tests` has 5 red tests about "Authenticated captured" texts (peer change, unrelated).

## Not verified
- The Rust MCP implementation against the new golden vectors (argv order `typecheck --json [flags] -- files`); only the TS oracle ran. Windows/Linux paths of the cache directory are unit-tested, the compiler call itself only ran on macOS.
- The 36 appended d.mts stubs were typechecked through the importers of 24 files; part of the 36 files exceeded the importer cap when checked together.
- `repo doctor`/readiness integration of the gate (F63's proposal) is not wired; the brief rule is in `W/brief-addendum-typecheck.md`.

## Others must know
- Test importers of the d.mts show about 640 pre-existing test-file type errors (F66's list); they are in tests, so `repo typecheck` on product paths still passes unless you also name a test.
- The progress line says "reading imports of 0 files" for `--declarations` without paths (cosmetic).
