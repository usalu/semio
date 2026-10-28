import sys, os
C, G = sys.argv[1], sys.argv[2]
def rows(p):
    return [l.rstrip('\n').split('\t') for l in open(p, encoding='utf-8') if l.strip()]
applied = rows(os.path.join(G, 'f1-applied.tsv'))
kind = rows(os.path.join(G, 'f1-kind.txt'))
live_rc = [r[1] for r in rows(os.path.join(G, 'f1-live-recheck.tsv'))]
tests = rows(os.path.join(G, 'f1-tests-2.tsv'))
tests = [t for t in tests if len(t) >= 4 and t[0] != 'FINISHED']
graph = [r[1] for r in kind if '🕸️graph' in r[1] or '🕰️generations' in r[1] or '📥️inference' in r[1]]
libc = [r[1] for r in kind if '🧲️native/📚️libc' in r[1]]
live_mtime = []
for fp in ['🧰️framework/🔨️modules/🏃️process/🔒️leases/📁️physical/🧪️tests/🪟️windows/🐍️oracle/🐍️.py',
           '🧰️framework/🔨️modules/🏃️process/🔒️leases/📁️physical/🧪️tests/🪟️windows/🟦️.ts',
           '🧰️framework/🔨️modules/🏃️process/🔒️leases/🪟️windows/🧪️tests/🐍️oracle/🐍️.py',
           '🧰️framework/🔨️modules/🏃️process/🔒️leases/🪟️windows/🧪️tests/🟦️.ts',
           '🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🧪️tests/🟦️.ts']:
    live_mtime.append(fp)
ok = [t for t in tests if t[0] == '0']
bad = [t for t in tests if t[0] != '0']
L = []
L.append('# result-hyg2-g (wave 1026g, executor hyg2-g: mechanical hygiene F1-F3)')
L.append('')
L.append('Scope: `C/audit-leftovers-1026g.md` F1 (docstrings), F2 (duplicate leading emoji), F3 (English-only user texts). F4 and F5 not touched. Scripts: `C/hyg2-g-tools/`, output: `C/🗑️generated/hyg2-g/`.')
L.append('')
L.append('## FOR lines (not edited, owner must act)')
L.append('')
L.append('- FOR gate-g and gram2-g (13:xx): 12 graph test/oracle files under `🕸️graph`, `🕰️generations` and `📥️inference` still lack a leading emoji docstring (F1). Live graph lane gram2-g (result 13:05) and the graph-module rule need an ack before I add them. Files: ' + '; '.join('`' + p.replace('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/', '') + '`' for p in graph) + '.')
L.append('- FOR nodeffi-g (result 13:07, live): 2 files in `⚙️runtime/🧲️native/📚️libc/🧪️tests` (`🟦️.ts`, `🐍️.py`) lack a leading docstring (F1). Not edited.')
L.append('- FOR the writer of the leases/windows and workers files (mtime under 20 min, another writer active at 13:03-13:10): `🔒️leases/📁️physical/🧪️tests/🪟️windows/🐍️oracle/🐍️.py`, `…/🪟️windows/🟦️.ts`, `🔒️leases/🪟️windows/🧪️tests/🐍️oracle/🐍️.py`, `…/🪟️windows/🧪️tests/🟦️.ts`, `🧪️testing/👷️workers/🧪️tests/🟦️.ts` (F1). Not edited.')
L.append('- FOR the writer of 3 files that changed under 20 min ago at 13:1x: `🛑️termination/🟦️.ts`, `🔒️busy/🟦️.ts` (churn), `⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🪟️windows/🟦️.ts` (F1). Not edited.')
L.append('- FOR lane2-g (result 13:05, live on pooled lanes): F2 pair `⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧪️tests/🏊️pool-model/` (`🐍️.py` and `🟦️.ts`) both start with 🏊. Not edited, one emoji change needed.')
L.append('- FOR rust owner (rust-g, not live): `🎟️admission/🦀️.rs` lines 380, 391, 461, 471 are English-only `eprintln!` strings (F3). The crate has no language selection (no env var, no catalog), so per the brief I write this FOR instead of inventing one. Decision needed: which language source the scheduler uses (for example `SEMIO_LANGUAGE` passed from the TS side).')
L.append('- FOR owner of `🧪️test/🟨️.mjs` (mtime 7 min, live): 4 English-only thrown errors (F3, audit lines 59, 68, 236, 261). Not edited.')
L.append('- FOR owner of `🔧️tools/🏗️bazel/📜️script.ts` (mtime 6 min, live): 2 English-only texts (F3, audit lines 220, 267, "Bazel source inventory exceeds 64 MiB"). Not edited.')
L.append('- FOR route2-g (suites owner): `🗂️suites/🧪️tests/🟦️.ts` "every test file belongs to exactly one suite" fails with `unrouted: 🔭️witness/🧪️tests/🟦️.ts`. That file is not mine and not in my list; the suites table needs the entry. Not touched.')
L.append('')
L.append('## Status (newest first)')
L.append('')
L.append('- ' + 'run 2 (after all edits, see table below): ' + str(len(ok)) + ' of ' + str(len(tests)) + ' touched files pass their check.')
L.append('- F3 sealed promotion done (5 texts EN+DE, see below). F2 pairs fixed except pool-model (FOR lane2-g). F1: 50 of 72 test/oracle files done, 22 left as FOR.')
L.append('')
L.append('## F1: docstrings (leading, unique emoji, native format)')
L.append('')
L.append('Scope count: the audit listed 41 files. The current tree has 78 new (added or untracked) test/oracle/fixture paths without a leading docstring. 6 of the 78 are production modules (`📥️inputs/🔭️read-oracle/…`, false positive of the `oracle` name), not test files: not edited, see the last section. So 72 test/oracle files are in scope: 50 edited (below), 22 FOR (above).')
L.append('')
L.append('Format: TS/MJS `/** <emoji> <text> */`, Python `"""<emoji> <text>"""` (after a shebang if present). Emoji = the nearest subject directory emoji (not `tests`/`fixtures`/`oracle`). Existing docstrings without an emoji got the same emoji prefixed (`prefix`). Text is `Tests for …` / `Fixture for …` / `Independent oracle for …` built from the path, and hand-fixed for 3 fixtures. Unique emoji within each file.')
L.append('')
L.append('| action | kind | emoji | file |')
L.append('|---|---|---|---|')
for r in applied:
    a, k, e, t, p = r
    L.append('| %s | %s | %s | `%s` |' % (a, k, e, p))
L.append('')
L.append('Re-emoji inside a file (13 files): the inserted leading docstring clashed with an existing docstring emoji in the same file, so the leading one got 🧪 (or the next free candidate). Same-directory clashes (py oracle vs ts test with the same subject emoji, 8 dirs): the python oracle got 🐍 (trampoline fixture: 📜). Affected: `🪶️sparse/🧪️tests/🐍️.py`, `🧰️programs/🧪️tests/🐍️.py`, `📈️standing/🧪️tests/🐍️model.py`, `🎲️fuzz/🐍️.py`, `🎚️machine/🧪️tests/🐍️.py`, `🪟️windows/🧫️fixtures/🐍️echo.py`, `🏛️sealed/🧪️tests/🧫️fixtures/📜️trampoline.ts`, `👯️dedupe/🧪️tests/🐍️.py`.')
L.append('')
L.append('Fixture texts hand-fixed: `🪟️windows/🧫️fixtures/🐍️tree.py` ("Fixture reading a process table and a pid"), `🪟️windows/🧫️fixtures/🐍️echo.py` ("Fixture echoing its arguments as JSON"). `📥️worker/🟦️.ts` (leases standing) text fixed to "Worker script that the leases standing tests start as a child process." (it is a child script, not a test).')
L.append('')
L.append('Not edited, out of scope (production modules, no leading docstring, not test/oracle): `📥️inputs/🔭️read-oracle/🍎️seatbelt/🟦️.ts`, `🏃️run/🟦️.ts`, `🐧️linux/🟦️.ts`, `🚪️cli/🟦️.ts`, `🟦️.ts`, `🪞️mirror/🟦️.ts`. Owner keys2-g (not live). Candidate for a later pass.')
L.append('')
L.append('## F2: duplicate leading emoji')
L.append('')
L.append('- `🔒️lease/🧪️tests/🔓️unit-lock-model/🐍️.py`: 🔓 -> 🔐 (ts keeps 🔓).')
L.append('- `🎟️admission/🧪️tests/🧵️frontend-threads/🐍️.py`: 🧵 -> 🔢 (ts keeps 🧵).')
L.append('- `🏛️sealed/🧪️tests/👥️processes/📜️child.ts`: child docstring 👥 -> 📜 and moved to the top (it had my inserted 👥 line too). `🟦️.ts` keeps 👥.')
L.append('- `📥️inputs/🔭️read-oracle/🐧️linux/🐍️.py` (production Landlock launcher, pair with `🟦️.ts` 🐧): 🐧 -> 🔒.')
L.append('- `⚡️caching/…/🏊️pool-model/`: FOR lane2-g.')
L.append('')
L.append('## F3: English-only user texts')
L.append('')
L.append('- DONE `🏛️sealed/📤️promotion/🟦️.ts`: 5 English row messages now go through `sealedTextV1(sealedLanguagesV1(environment,policy), key, values, policy)` (same pattern as the module): `candidateLoader`, `candidateNotLoaded`, `candidateNotLoadedWithin`, `selfCheckNotEnded`, `selfCheckEnded`. Texts added to `🏛️sealed/🔣️.json` (`texts`, en + de) and to the schema `🏛️sealed/🧬️schema/🔣️.json` (`required` + `properties`). The sealed store test (`🧪️tests/🗂️store/🟦️.ts`, 12 tests) passes after the change.')
L.append('- Still English in the same file: the fallback `"the candidate did not load"` in `rowOfLoadFailureV1` (about line 107). It has no environment argument. Left as is, to be decided with the rows owner (self-g).')
L.append('- DECISION (developer-only, dev to confirm): `🔁️incremental/🟦️.ts:44` "Retained TypeScript programs must be an integer from 1 to 8" is a thrown configuration error for a programmatic argument. No end user sees it, so I did not translate it.')
L.append('- FOR: Rust scheduler (4 lines), `🧪️test/🟨️.mjs` (4), `bazel/📜️script.ts` (2). See the FOR section.')
L.append('')
L.append('## Tests (single file runs, after all edits)')
L.append('')
L.append('Run order: `bun test --timeout 120000 ./<file>` for `*.ts` tests, `python3 -I -c "ast.parse(...)"` for Python (the oracles need arguments, so a plain `python3 -I <file>` only prints usage; the AST parse checks syntax of the edited docstring), `node --check` for the `.mjs`, `bun build --no-bundle` for TS fixtures that are not tests. Log per file: `C/🗑️generated/hyg2-g/f1-tests-2.tsv`.')
L.append('')
L.append('| rc | kind | file |')
L.append('|---|---|---|')
for t in tests:
    L.append('| %s | %s | `%s` |' % (t[0], t[1], t[2]))
L.append('')
L.append('Known failures, none caused by the docstrings or the texts (checked individually):')
L.append('')
L.append('- `🔁️incremental/🟦️.ts` (11 pass with env): needs `SEMIO_TEST_ARTIFACT_DIR`. Passed when run with a short artifact dir.')
L.append('- `🚦️live/🟦️.ts` (4 pass with env): needs `SEMIO_TEST_ARTIFACT_DIR`. With a path under `🗑️generated/hyg2-g` it fails with "Materialized absolute path exceeds 256 characters". Ticket-folder artifact dirs break the 256-char rule in this test. Passed with a short scratchpad dir. Finding for the leases owner.')
L.append('- `📥️worker/🟦️.ts`: a child script, not a test. Run directly it fails on `JSON.parse(process.argv[2])`. Pre-existing.')
L.append('- `🧊️memo/🟦️.ts` (authorizers): "Original parent Script process invocation required" (from `receiveScriptProcessInvocation`). Harness precondition, not a docstring effect. Not fixed.')
L.append('- `🗂️suites/🧪️tests/🟦️.ts`: `unrouted: 🔭️witness/🧪️tests/🟦️.ts` (not mine). FOR route2-g.')
L.append('- `🏛️sealed/📤️promotion/🟦️.ts` is a module, not a test file: `bun build --no-bundle` syntax check only.')
L.append('')
L.append('## Notes')
L.append('')
L.append('- Lock-free: no graph run, no product build, no modifying git command. Live-lane check by file mtime (under 20 min = skipped) plus owner result mtimes.')
L.append('- First attempt of this wave used zsh `read ... path`, which overwrote the `PATH` tied variable; no file was written by that attempt (the stat check failed before any edit). Fixed by renaming the variable.')
L.append('- Artifact dirs for the env-dependent tests were put in the session scratchpad (short path), not in the ticket: see the 256-char finding above.')
open(os.path.join(C, 'result-hyg2-g.md'), 'w', encoding='utf-8').write('\n'.join(L) + '\n')
print('written', len(L), 'lines; ok', len(ok), 'bad', len(bad))
