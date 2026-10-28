# audit-leftovers-1026g (wave 1026g, read-only auditor aud-leftover-g, 2026-10-26)

Scope: the 693 changed or untracked paths outside ticket folders (`git status --porcelain -uall`, 226 added, 359 modified, 37 MM, 11 AM, 34 untracked, 26 M). Method: `git diff HEAD -U0` added lines (streamed through awk), file reads of the 34 untracked files, `rg` over names, `git ls-files`, `git check-ignore`, line and byte counts, a docstring check in Python (streaming, no bulk reads). No build, no source edit, no modifying git command.

Owner lanes are the best match from the wave result files (`result-*-g.md`). Where the match is weak, it says so.

## Clean (checked, nothing found)

- Backup, orig, old, bak, rej, tmp and swap copies: none among the 693 paths.
- Scratch scripts or probes at the repo root or in source dirs: none. The root `CMakeLists.txt` (new, 18 lines, 10:22) is intentional. `🛠️cmake/🧬️schema` pins its digest; route2-g restored it (`result-route2-g.md:63`). Do not delete it.
- `[DEBUG]` markers: none in added lines or untracked files.
- `console.log`, `debugger`, `dbg!`, `println!`, `breakpoint`: only intentional CLI stdout in production code. `🦀️cargo/🗃️unit-cache/📜️script.ts:146`, `📥️inputs/🔭️read-oracle/🚪️cli/🟦️.ts:41`. All other hits are in tests, fixtures or oracles.
- TODO, FIXME, XXX, HACK added today: none.
- Commented-out code added today: none. One false positive: `🐹️.go` line with a docstring emoji.
- Generated or runtime-state dirs: `🧰️framework/🤖️generated/process` (193 MB) is NOT tracked. `.gitignore:94` (`**/🤖️generated/`) ignores it. No `🤖️generated/` or `🗑️generated/` path is tracked outside tickets.
- Path length over 256 characters: none (checked in characters and bytes).
- `.d.mts` next to `.mjs` pairs (e.g. `🔭️witness/🟨️.d.mts`) are hand-written sources, not leftovers.

## Findings

### F1. Docstrings (new source files) — 42 files, all under `🧪️tests`, fixtures or oracles

Files with no doc comment at all (29) or a doc comment without a leading emoji (13). Detector false positive: `🧭️invocation/🧪️tests/📝️plan-file/🟦️.ts` (string literal, no docstring), so the real count is 41.

- No docstring: `🔨️modules/⚙️runtime/🧲️native/📚️libc/🧪️tests/🐍️.py`, `…/🟦️.ts`; `🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/🟦️.ts`; `…/🧪️tests/🪟️windows/🧫️fixtures/🐍️tree.py`; `…/📦️artifacts/🏗️native-build/💾️storage/` (`🍎️image/🪶️sparse/🧪️tests/🐍️.py`, `…/🟦️.ts`; `📁️directory/🔀️exchange/🧪️tests/🟦️.ts`; `📁️directory/🧪️tests/🪞️fold/🟦️.ts`; `🔌️backends/🍎️mounted-image/✍️sealing/🧪️tests/🟦️.ts`; `🧰️programs/🧪️tests/🐍️.py`, `…/🟦️.ts`; `🪶️extent/🧪️tests/🟦️.ts`); `🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🐍️model.py`; `🔨️modules/🏃️process/🧪️testing/👷️workers/🧪️tests/📐️learning/🟦️.ts` and `…/🧪️tests/🟦️.ts`; `🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🔒️busy/🟦️.ts`; `🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🧪️tests/🟦️.ts`; `…/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧊️memo/🟦️.ts`; `…/🏗️build/⌨️command/🧪️tests/🌱️plan-child/🟦️.ts`; `…/🏗️build/🌡️host-capacity/🧪️tests/🧪️workers/🟦️.ts`; `…/🕸️graph/🕰️generations/🧩️claims/🧪️tests/🏃️processes/📥️holder/🟦️.ts`, `📥️worker/🟦️.ts`, `🟦️.ts`, `🟦️.ts` (claims tests); `…/🗂️suites/🧪️tests/🟦️.ts`; `…/🧹️cache-gc/👯️dedupe/🧪️tests/🟦️.ts`; `…/🧹️cache-gc/🧪️tests/💰️budget/🟦️.ts`; `🧾️observation/🔁️churn/🧪️tests/…`; `🦀️cargo/🗃️unit-cache/🧪️tests/🪟️windows/🧫️fixtures/🐍️echo.py`, `🟦️echo.ts`.
- Docstring without a leading emoji: `🔁️churn/🧪️tests/🎲️fuzz/🐍️.py` and `…/🟨️.mjs`; `🧾️ledger/🎚️machine/🧪️tests/🐍️.py`; `🕸️graph/⏱️phases/🧪️tests/🐍️model.py`; `🕸️graph/🎯️test-owners/🧪️tests/🐍️oracle/🐍️.py`; `📥️inference/♻️revision/🧪️tests/🐍️oracle/🐍️.py`; `🕰️generations/✂️cutoff/🧪️tests/🏪️store/🐍️.py` and `🐍️oracle/🐍️.py`; `🌿️frontier-walk/🧪️tests/🐍️oracle/🐍️.py`; `🧩️claims/🧪️tests/🐍️oracle/🐍️.py`; `🧪️tests/♻️tagged-retention/🐍️oracle/🐍️.py`; `🧹️cache-gc/👯️dedupe/🧪️tests/🐍️.py`.
- Owner (best match): gram-g / cutoff-g / hold-g for `🕸️graph`; store-g for `💾️storage`; churn-g for `🔁️churn`; disk-g for `🧾️ledger`, `🧹️cache-gc`; phase-g for host-capacity estimates; testmem-g for `👷️workers`; xplat-g for `🪟️windows` fixtures; route2-g for `⌨️command`, `🗂️suites`; noop-g for `🔏️authorizers`.
- Fix: one line per file, `🟦️`/`🐍️` docstring starting with a unique emoji in the file's own language (`/** 🧪️ … */` for TS, `"""🧪️ …"""` for Python).

### F2. Duplicate leading docstring emoji — 5 TS/Python test pairs in the same directory

- `🎟️admission/🧪️tests/🧵️frontend-threads/` (`🐍️.py`, `🟦️.ts`), owner ram-g / rust-g (weak match).
- `🔒️lease/🧪️tests/🔓️unit-lock-model/` (`🐍️.py`, `🟦️.ts`), owner lease-g.
- `⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧪️tests/🏊️pool-model/` (`🐍️.py`, `🟦️.ts`), owner lane-g.
- `🏛️sealed/🧪️tests/👥️processes/` (`📜️child.ts`, `🟦️.ts`), owner self-g.
- `📥️inputs/🔭️read-oracle/🐧️linux/` (`🐍️.py`, `🟦️.ts`), owner keys2-g (weak match).
- Probably deliberate pairs (same subject in two languages). Fix: keep the pair only if intended and document it; otherwise pick a distinct emoji per file.

### F3. English-only user-facing text (no `de` text)

The repo rule is EN and DE text. Each item below needs a `messages.json` key with `en` and `de`, or an explicit decision that it is developer-only.

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs:380, 391, 461, 471` — `eprintln!` strings "semio-build-scheduler: cannot run {}", "expected <rust…", "{error}". User-visible on stderr. Owner rust-g (result-rust-g mentions the scheduler). Fix: route through the runtime message catalog (en + de).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs:59, 68, 236, 261` — thrown English-only errors ("Test discovery requires real no-follow … authority"). Owner: test-runner lanes (buildlib-reds, zt-g; weak match). Fix: catalog key, or mark as developer-only.
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🔁️incremental/🟦️.ts:44` — "Retained TypeScript programs must be an integer from 1 to 8". Owner ts2-g. Fix: catalog key or developer-only decision.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/📤️promotion/🟦️.ts:71, 95` — report rows "the candidate did not load", timeout finish text. Owner self-g. Fix: catalog keys (en + de).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts:220, 267` — "Bazel source inventory exceeds 64 MiB". Owner: bazel lanes (bzsrv-g; weak match). Fix: catalog key.
- Checked OK: `🧲️libc/🟦️.ts` (runtime message keys with en + de), host-capacity `📡️sampling/🟦️.ts` (both languages), `🔒️leases/🟦️.ts` (both languages). `🧰️framework/🔨️modules/⚙️runtime/🧬️schema/🗣️messages.json`: the 3 new libc keys have en + de.

### F4. Tracked runtime state (not changed today, pre-existing)

- `holder/853a7dc2….sqlite`, `holder/b7d6d227….sqlite`, `holder/claims/d92d5a25….sqlite`, `holder/claims/f466e067….sqlite`, `holder/d92d5a25….sqlite` at repo root: tracked since 697 (`git log -- holder`), not in today's status. Owner idle-c (`result-idle-c.md` mentions holder). Fix: `git rm --cached` and ignore them. The coordinator must run that, not the auditor.
- `🌎️hub/📦️packages/🦀️rust/.semio/hub/directory.db` (98 KB): tracked since 525. Owner tax lanes (result-tax*.md). Fix: same as above.
- `🧰️framework/🛍️products/💻️os/…/fixtures/…/g3-event-log.jsonl` (`🖥️server/🎛️coordinator/🧪️tests/🧫️fixtures/📜️g3-event-log.jsonl`): a fixture, OK.

### F5. Oversized files (changed today, outside tickets)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json` — 1,388,082 bytes, +9 lines today. A generated index that is tracked. Owner: the dsl-derive generator (no 1026g result file). Fix: confirm it is generated, and either regenerate it in the build or move it out of the tracked tree.
- `🧰️framework/🛍️products/🦑️repo/README.md` — 2,345 lines, 611 KB, longest line 6,311 chars (line 1748). Changed today (+37/-9 from docs-g, `result-docs-g.md:5`). Fix: split into topic docs, keep the EN and DE pointer.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🗂️catalog/🐹️.go` — 985 lines, changed today. Owner: repo MCP lanes (mcp-b; weak). Fix: split by tool group.
- `…/📚️library/🏗️build/👁️observation/🟨️.mjs` — 955 lines. Owner gram-g / observation lanes. Fix: split.
- `…/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts` — 933 lines. Owner bazel lanes. Fix: split.
- `…/📚️library/🏗️build/📏️incremental/🏎️bench/🧪️tests/🟦️.ts` — 898 lines. Owner bench-g. Fix: split.
- `…/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts` — 890 lines. Fix: split by case group.
- `…/📚️library/🏗️build/🕸️graph/🟦️.ts` — 768 lines. Owner gram-g / cutoff-g. Fix: split.
- JSON fixtures over 300 KB (changed today): `🏛️sealed/🧪️tests/🔀️interleavings/🔣️.json` (313 KB), `🔧️tools/🏗️bazel/🎬️scopes/🔣️.json` (387 KB), `⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json` (585 KB), `🏗️build/🌡️host-capacity/🧪️tests/🧫️fixtures/🔣️.json` (642 KB). Low priority; fixtures.

## Summary counts

Tracked-changed or new outside tickets: 693. Debug markers: 0. TODO/FIXME: 0. Commented-out code: 0. Backup or scratch copies: 0. Generated or runtime-state files newly tracked today: 0. Docstring findings: 41. English-only user texts: 9 sites in 6 files. Oversized: 1 generated JSON, 1 README, 8 source files, 4 JSON fixtures.
