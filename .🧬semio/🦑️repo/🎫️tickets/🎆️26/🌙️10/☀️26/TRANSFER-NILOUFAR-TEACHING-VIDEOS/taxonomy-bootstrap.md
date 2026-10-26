# Taxonomy Correction Bootstrap

The ticket-local repo launcher is restored. No product/source files, names, AGENTS files or index entries were changed. No tests or source renames were run. All owned build and diagnostic processes have finished.

## Command Interface

The cached `.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo.exe --help` exposes the older domain CLI, without the current script/tool routes. Its attempted `tools exec --light bun -- --version` failed with `unknown flag: --light`. The retained `bootstrap.md` documents this same limitation.

As explicitly delegated, ran the retained `bootstrap-launcher.ps1` to rebuild the real launcher. It compiled14 Go overlay source mappings referencing the original repository source files; no tracked source files were copied. The generated module, overlay, compiler cache and executable all remain in this ticket's generated folder. The build ended0. Launcher validation reports1.0.0, contract1, implementation0.1.0. First script use provisioned and verified pinned Bun1.4.2 into generated/runtimes. The generic existing `run.ts` interface successfully executed `git --version` and reported `git version2.53.0.windows.2`.

Use PowerShell from `C:/git/semio`:

```powershell
$env:SEMIO_RUNTIME_CACHE = 'C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️26\TRANSFER-NILOUFAR-TEACHING-VIDEOS\🗑️generated\runtimes'
$env:SEMIO_LANGUAGE = 'en'
$env:PYTHONUTF8 = '1'
$env:PYTHONIOENCODING = 'utf-8'
$env:PYTHONDONTWRITEBYTECODE = '1'
& 'C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️26\TRANSFER-NILOUFAR-TEACHING-VIDEOS\🗑️generated\launcher\repo.exe' 'C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️26\TRANSFER-NILOUFAR-TEACHING-VIDEOS\run.ts' <executable> <arguments>
```

Alternatively pass a ticket-local TypeScript script path directly after `repo.exe`. Keep `SEMIO_RUNTIME_CACHE` set for every invocation so no launcher runtime appears outside this ticket.

## Preserved Index Baseline

The retained `taxonomy-baseline.ts` ran through repo and used read-only Git commands with `GIT_OPTIONAL_LOCKS=0`. It captured the exact index bytes before listing entries and verified the original and final index bytes remained identical during capture. Files under `🗑️generated/taxonomy-baseline/`:

- `index.bin`: exact index snapshot for byte comparisons; do not restore it over concurrent work.
- `index-entries.z`: NUL-separated staged mode/object/stage/path entries.
- `staged-name-status.z`: NUL-separated staged name/status differences.
- `staged-stat.txt`: current staged diff summary.
- `status.z`: tracked porcelain status, NUL-separated, without untracked traversal.
- `baseline.json`: capture timestamp, SHA256, byte length, entry count and HEAD.

Captured at `2026-10-26T07:52:29.480Z`; index contains198074 entries and51471558 bytes. Index SHA256: `6ecfa4f97ec670fea41b71e7895dd29527c81df02f541aef6b90e2ef1c731313`. HEAD: `7b2365ac08c46994aa8ac098d5014f72a453f043`. Existing staged changes were preserved.

## Applicable Instructions and Paths

Read root `AGENTS.md`. There is no `AGENTS.md` at the teaching root or anywhere below the current teaching subtree, and none in the ticket ancestry. The root rules therefore govern this correction: do not edit AGENTS; no modifying Git commands or worktrees; commands use repo; source stays outside the ticket; generated outputs stay within this ticket; filenames/paths must remain within256UTF16units; preserve concurrent work; follow domain-driven multi-implementation taxonomy and schema-first/TDD requirements for subsequent changes.

Also inspected the launcher ancestor instructions in `🧰️framework/🛍️products/AGENTS.md` and repo product `AGENTS.md`; the former describes the platform/playground/OS products, while the latter defines repository taxonomy and artifact identity. No launcher source edits were made. Generated paths after bootstrap and baseline capture max198UTF16units, below256.
