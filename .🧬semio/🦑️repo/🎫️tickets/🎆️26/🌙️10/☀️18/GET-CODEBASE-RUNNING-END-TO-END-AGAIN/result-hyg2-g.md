# result-hyg2-g (wave 1026g, executor hyg2-g: mechanical hygiene F1-F3)

Scope: `C/audit-leftovers-1026g.md` F1 (docstrings), F2 (duplicate leading emoji), F3 (English-only user texts). F4 and F5 not touched. Scripts: `C/hyg2-g-tools/`, output: `C/🗑️generated/hyg2-g/`.

## FOR lines (not edited, owner must act)

- FOR gate-g and gram2-g (13:xx): 12 graph test/oracle files under `🕸️graph`, `🕰️generations` and `📥️inference` still lack a leading emoji docstring (F1). Live graph lane gram2-g (result 13:05) and the graph-module rule need an ack before I add them. Files: `🕸️graph/⏱️phases/🧪️tests/🐍️model.py`; `🕸️graph/🎯️test-owners/🧪️tests/🐍️oracle/🐍️.py`; `🕸️graph/📥️inference/♻️revision/🧪️tests/🐍️oracle/🐍️.py`; `🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🏪️store/🐍️.py`; `🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🐍️oracle/🐍️.py`; `🕸️graph/🕰️generations/🌿️frontier-walk/🧪️tests/🐍️oracle/🐍️.py`; `🕸️graph/🕰️generations/🧩️claims/🧪️tests/🏃️processes/📥️holder/🟦️.ts`; `🕸️graph/🕰️generations/🧩️claims/🧪️tests/🏃️processes/📥️worker/🟦️.ts`; `🕸️graph/🕰️generations/🧩️claims/🧪️tests/🏃️processes/🟦️.ts`; `🕸️graph/🕰️generations/🧩️claims/🧪️tests/🐍️oracle/🐍️.py`; `🕸️graph/🕰️generations/🧩️claims/🧪️tests/🟦️.ts`; `🕸️graph/🧪️tests/♻️tagged-retention/🐍️oracle/🐍️.py`.
- FOR nodeffi-g (result 13:07, live): 2 files in `⚙️runtime/🧲️native/📚️libc/🧪️tests` (`🟦️.ts`, `🐍️.py`) lack a leading docstring (F1). Not edited.
- FOR the writer of the leases/windows and workers files (mtime under 20 min, another writer active at 13:03-13:10): `🔒️leases/📁️physical/🧪️tests/🪟️windows/🐍️oracle/🐍️.py`, `…/🪟️windows/🟦️.ts`, `🔒️leases/🪟️windows/🧪️tests/🐍️oracle/🐍️.py`, `…/🪟️windows/🧪️tests/🟦️.ts`, `🧪️testing/👷️workers/🧪️tests/🟦️.ts` (F1). Not edited.
- FOR the writer of 3 files that changed under 20 min ago at 13:1x: `🛑️termination/🟦️.ts`, `🔒️busy/🟦️.ts` (churn), `⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🪟️windows/🟦️.ts` (F1). Not edited.
- FOR lane2-g (result 13:05, live on pooled lanes): F2 pair `⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧪️tests/🏊️pool-model/` (`🐍️.py` and `🟦️.ts`) both start with 🏊. Not edited, one emoji change needed.
- FOR rust owner (rust-g, not live): `🎟️admission/🦀️.rs` lines 380, 391, 461, 471 are English-only `eprintln!` strings (F3). The crate has no language selection (no env var, no catalog), so per the brief I write this FOR instead of inventing one. Decision needed: which language source the scheduler uses (for example `SEMIO_LANGUAGE` passed from the TS side).
- FOR owner of `🧪️test/🟨️.mjs` (mtime 7 min, live): 4 English-only thrown errors (F3, audit lines 59, 68, 236, 261). Not edited.
- FOR owner of `🔧️tools/🏗️bazel/📜️script.ts` (mtime 6 min, live): 2 English-only texts (F3, audit lines 220, 267, "Bazel source inventory exceeds 64 MiB"). Not edited.
- FOR route2-g (suites owner): `🗂️suites/🧪️tests/🟦️.ts` "every test file belongs to exactly one suite" fails with `unrouted: 🔭️witness/🧪️tests/🟦️.ts`. That file is not mine and not in my list; the suites table needs the entry. Not touched.

## Status (newest first)

- run 2 (after all edits, see table below): 47 of 51 touched files pass their check.
- F3 sealed promotion done (5 texts EN+DE, see below). F2 pairs fixed except pool-model (FOR lane2-g). F1: 50 of 72 test/oracle files done, 22 left as FOR.

## F1: docstrings (leading, unique emoji, native format)

Scope count: the audit listed 41 files. The current tree has 78 new (added or untracked) test/oracle/fixture paths without a leading docstring. 6 of the 78 are production modules (`📥️inputs/🔭️read-oracle/…`, false positive of the `oracle` name), not test files: not edited, see the last section. So 72 test/oracle files are in scope: 50 edited (below), 22 FOR (above).

Format: TS/MJS `/** <emoji> <text> */`, Python `"""<emoji> <text>"""` (after a shebang if present). Emoji = the nearest subject directory emoji (not `tests`/`fixtures`/`oracle`). Existing docstrings without an emoji got the same emoji prefixed (`prefix`). Text is `Tests for …` / `Fixture for …` / `Independent oracle for …` built from the path, and hand-fixed for 3 fixtures. Unique emoji within each file.

| action | kind | emoji | file |
|---|---|---|---|
| insert | NODOC | 📐️ | `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/🟦️.ts` |
| insert | NODOC | 🪟️ | `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🪟️windows/🟦️.ts` |
| insert | NODOC | 🪟️ | `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🪟️windows/🧫️fixtures/🐍️tree.py` |
| insert | NODOC | 🔁️ | `🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🧪️tests/🔁️incremental/🟦️.ts` |
| insert | NODOC | 🪶️ | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪶️sparse/🧪️tests/🐍️.py` |
| insert | NODOC | 🪶️ | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪶️sparse/🧪️tests/🟦️.ts` |
| insert | NODOC | 🔀️ | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🔀️exchange/🧪️tests/🟦️.ts` |
| insert | NODOC | 🪞️ | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🧪️tests/🪞️fold/🟦️.ts` |
| insert | NODOC | ✍️ | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/🍎️mounted-image/✍️sealing/🧪️tests/🟦️.ts` |
| insert | NODOC | 🧰️ | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧰️programs/🧪️tests/🐍️.py` |
| insert | NODOC | 🧰️ | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧰️programs/🧪️tests/🟦️.ts` |
| insert | NODOC | 🪶️ | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🪶️extent/🧪️tests/🟦️.ts` |
| insert | NODOC | 📈️ | `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🐍️model.py` |
| insert | NODOC | 📥️ | `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/📥️worker/🟦️.ts` |
| insert | NODOC | 🚦️ | `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🚦️live/🟦️.ts` |
| insert | NODOC | 📈️ | `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🟦️.ts` |
| insert | NODOC | 📐️ | `🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🧪️tests/📐️learning/🟦️.ts` |
| insert | NODOC | 🧱️ | `🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🧪️tests/🧱️capped-run/🟦️.ts` |
| insert | NODOC | 🏠️ | `🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🧪️tests/🏠️runtime-store/🟦️.ts` |
| prefix | NOEMOJI | 🎲️ | `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🎲️fuzz/🐍️.py` |
| insert | NODOC | 🎲️ | `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🎲️fuzz/🟨️.mjs` |
| prefix | NOEMOJI | 🎚️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🧪️tests/🐍️.py` |
| insert | NODOC | 🎚️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🧪️tests/🟦️.ts` |
| insert | NODOC | 🧊️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧊️memo/🟦️.ts` |
| insert | NODOC | 🪟️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🪟️windows/🧫️fixtures/🐍️echo.py` |
| insert | NODOC | 🪟️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🪟️windows/🧫️fixtures/🟦️echo.ts` |
| insert | NODOC | 🪝️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🪝️hooks/🧪️tests/🟦️.ts` |
| insert | NODOC | 🔣️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/📋️context/🔣️policy/🧪️tests/🟦️.ts` |
| insert | NODOC | 🌱️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/🧪️tests/🌱️plan-child/🟦️.ts` |
| insert | NODOC | 📈️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/📈️memory-fit/🟦️.ts` |
| insert | NODOC | 🧪️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🧪️workers/🟦️.ts` |
| insert | NODOC | 🪟️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🪟️partial/🟦️.ts` |
| insert | NODOC | 👥️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/👥️processes/📜️child.ts` |
| insert | NODOC | 🏛️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🧫️fixtures/📜️trampoline.ts` |
| insert | NODOC | 🏛️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🧫️fixtures/🟦️.ts` |
| insert | NODOC | 🏝️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🏝️reach/🟦️.ts` |
| insert | NODOC | 🌳️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🟦️.ts` |
| insert | NODOC | 🧬️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🧬️inference/🟦️.ts` |
| insert | NODOC | 🛤️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧪️tests/🛤️router/🟦️.ts` |
| insert | NODOC | 🤝️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🤝️shared-writer/🟦️.ts` |
| insert | NODOC | 🔭️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🧪️tests/🟦️.ts` |
| insert | NODOC | 🪟️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🧪️tests/🪟️windows/🟦️.ts` |
| insert | NODOC | 🪛️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪛️toolchain-activation/🧪️tests/🟦️.ts` |
| insert | NODOC | 🪟️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🪟️toolchain-link/🟦️.ts` |
| insert | NODOC | 🗂️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🧪️tests/🟦️.ts` |
| insert | NODOC | 📝️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/📝️plan-file/🟦️.ts` |
| insert | NODOC | 🔗️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🔗️process-chain/🧫️fixtures/🪆️relaunch.ts` |
| prefix | NOEMOJI | 👯️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/👯️dedupe/🧪️tests/🐍️.py` |
| insert | NODOC | 👯️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/👯️dedupe/🧪️tests/🟦️.ts` |
| insert | NODOC | 💰️ | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🧪️tests/💰️budget/🟦️.ts` |

Re-emoji inside a file (13 files): the inserted leading docstring clashed with an existing docstring emoji in the same file, so the leading one got 🧪 (or the next free candidate). Same-directory clashes (py oracle vs ts test with the same subject emoji, 8 dirs): the python oracle got 🐍 (trampoline fixture: 📜). Affected: `🪶️sparse/🧪️tests/🐍️.py`, `🧰️programs/🧪️tests/🐍️.py`, `📈️standing/🧪️tests/🐍️model.py`, `🎲️fuzz/🐍️.py`, `🎚️machine/🧪️tests/🐍️.py`, `🪟️windows/🧫️fixtures/🐍️echo.py`, `🏛️sealed/🧪️tests/🧫️fixtures/📜️trampoline.ts`, `👯️dedupe/🧪️tests/🐍️.py`.

Fixture texts hand-fixed: `🪟️windows/🧫️fixtures/🐍️tree.py` ("Fixture reading a process table and a pid"), `🪟️windows/🧫️fixtures/🐍️echo.py` ("Fixture echoing its arguments as JSON"). `📥️worker/🟦️.ts` (leases standing) text fixed to "Worker script that the leases standing tests start as a child process." (it is a child script, not a test).

Not edited, out of scope (production modules, no leading docstring, not test/oracle): `📥️inputs/🔭️read-oracle/🍎️seatbelt/🟦️.ts`, `🏃️run/🟦️.ts`, `🐧️linux/🟦️.ts`, `🚪️cli/🟦️.ts`, `🟦️.ts`, `🪞️mirror/🟦️.ts`. Owner keys2-g (not live). Candidate for a later pass.

## F2: duplicate leading emoji

- `🔒️lease/🧪️tests/🔓️unit-lock-model/🐍️.py`: 🔓 -> 🔐 (ts keeps 🔓).
- `🎟️admission/🧪️tests/🧵️frontend-threads/🐍️.py`: 🧵 -> 🔢 (ts keeps 🧵).
- `🏛️sealed/🧪️tests/👥️processes/📜️child.ts`: child docstring 👥 -> 📜 and moved to the top (it had my inserted 👥 line too). `🟦️.ts` keeps 👥.
- `📥️inputs/🔭️read-oracle/🐧️linux/🐍️.py` (production Landlock launcher, pair with `🟦️.ts` 🐧): 🐧 -> 🔒.
- `⚡️caching/…/🏊️pool-model/`: FOR lane2-g.

## F3: English-only user texts

- DONE `🏛️sealed/📤️promotion/🟦️.ts`: 5 English row messages now go through `sealedTextV1(sealedLanguagesV1(environment,policy), key, values, policy)` (same pattern as the module): `candidateLoader`, `candidateNotLoaded`, `candidateNotLoadedWithin`, `selfCheckNotEnded`, `selfCheckEnded`. Texts added to `🏛️sealed/🔣️.json` (`texts`, en + de) and to the schema `🏛️sealed/🧬️schema/🔣️.json` (`required` + `properties`). The sealed store test (`🧪️tests/🗂️store/🟦️.ts`, 12 tests) passes after the change.
- Still English in the same file: the fallback `"the candidate did not load"` in `rowOfLoadFailureV1` (about line 107). It has no environment argument. Left as is, to be decided with the rows owner (self-g).
- DECISION (developer-only, dev to confirm): `🔁️incremental/🟦️.ts:44` "Retained TypeScript programs must be an integer from 1 to 8" is a thrown configuration error for a programmatic argument. No end user sees it, so I did not translate it.
- FOR: Rust scheduler (4 lines), `🧪️test/🟨️.mjs` (4), `bazel/📜️script.ts` (2). See the FOR section.

## Tests (single file runs, after all edits)

Run order: `bun test --timeout 120000 ./<file>` for `*.ts` tests, `python3 -I -c "ast.parse(...)"` for Python (the oracles need arguments, so a plain `python3 -I <file>` only prints usage; the AST parse checks syntax of the edited docstring), `node --check` for the `.mjs`, `bun build --no-bundle` for TS fixtures that are not tests. Log per file: `C/🗑️generated/hyg2-g/f1-tests-2.tsv`.

| rc | kind | file |
|---|---|---|
| 0 | bun-test | `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🪟️windows/🟦️.ts` |
| 0 | pysyntax | `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🪟️windows/🧫️fixtures/🐍️tree.py` |
| 1 | bun-test | `🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🧪️tests/🔁️incremental/🟦️.ts` |
| 0 | pysyntax | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪶️sparse/🧪️tests/🐍️.py` |
| 0 | bun-test | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪶️sparse/🧪️tests/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🔀️exchange/🧪️tests/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🧪️tests/🪞️fold/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/🍎️mounted-image/✍️sealing/🧪️tests/🟦️.ts` |
| 0 | pysyntax | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧰️programs/🧪️tests/🐍️.py` |
| 0 | bun-test | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧰️programs/🧪️tests/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🪶️extent/🧪️tests/🟦️.ts` |
| 0 | pysyntax | `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🐍️model.py` |
| 1 | bun-test | `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/📥️worker/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🚦️live/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🧪️tests/📐️learning/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🧪️tests/🧱️capped-run/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🧪️tests/🏠️runtime-store/🟦️.ts` |
| 0 | pysyntax | `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🎲️fuzz/🐍️.py` |
| 0 | mjssyntax | `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🎲️fuzz/🟨️.mjs` |
| 0 | pysyntax | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🧪️tests/🐍️.py` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🧪️tests/🟦️.ts` |
| 1 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧊️memo/🟦️.ts` |
| 0 | pysyntax | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🪟️windows/🧫️fixtures/🐍️echo.py` |
| 0 | tssyntax | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🪟️windows/🧫️fixtures/🟦️echo.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🪝️hooks/🧪️tests/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/📋️context/🔣️policy/🧪️tests/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/🧪️tests/🌱️plan-child/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/📈️memory-fit/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🧪️workers/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🪟️partial/🟦️.ts` |
| 0 | tssyntax | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/👥️processes/📜️child.ts` |
| 0 | tssyntax | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🧫️fixtures/📜️trampoline.ts` |
| 0 | tssyntax | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🧫️fixtures/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🏝️reach/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🧬️inference/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧪️tests/🛤️router/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🤝️shared-writer/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🧪️tests/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🧪️tests/🪟️windows/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪛️toolchain-activation/🧪️tests/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🪟️toolchain-link/🟦️.ts` |
| 1 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🧪️tests/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/📝️plan-file/🟦️.ts` |
| 0 | tssyntax | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🔗️process-chain/🧫️fixtures/🪆️relaunch.ts` |
| 0 | pysyntax | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/👯️dedupe/🧪️tests/🐍️.py` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/👯️dedupe/🧪️tests/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🧪️tests/💰️budget/🟦️.ts` |
| 0 | bun-test | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/📤️promotion/🟦️.ts` |

Known failures, none caused by the docstrings or the texts (checked individually):

- `🔁️incremental/🟦️.ts` (11 pass with env): needs `SEMIO_TEST_ARTIFACT_DIR`. Passed when run with a short artifact dir.
- `🚦️live/🟦️.ts` (4 pass with env): needs `SEMIO_TEST_ARTIFACT_DIR`. With a path under `🗑️generated/hyg2-g` it fails with "Materialized absolute path exceeds 256 characters". Ticket-folder artifact dirs break the 256-char rule in this test. Passed with a short scratchpad dir. Finding for the leases owner.
- `📥️worker/🟦️.ts`: a child script, not a test. Run directly it fails on `JSON.parse(process.argv[2])`. Pre-existing.
- `🧊️memo/🟦️.ts` (authorizers): "Original parent Script process invocation required" (from `receiveScriptProcessInvocation`). Harness precondition, not a docstring effect. Not fixed.
- `🗂️suites/🧪️tests/🟦️.ts`: `unrouted: 🔭️witness/🧪️tests/🟦️.ts` (not mine). FOR route2-g.
- `🏛️sealed/📤️promotion/🟦️.ts` is a module, not a test file: `bun build --no-bundle` syntax check only.

## Notes

- Lock-free: no graph run, no product build, no modifying git command. Live-lane check by file mtime (under 20 min = skipped) plus owner result mtimes.
- First attempt of this wave used zsh `read ... path`, which overwrote the `PATH` tied variable; no file was written by that attempt (the stat check failed before any edit). Fixed by renaming the variable.
- Artifact dirs for the env-dependent tests were put in the session scratchpad (short path), not in the ticket: see the 256-char finding above.
