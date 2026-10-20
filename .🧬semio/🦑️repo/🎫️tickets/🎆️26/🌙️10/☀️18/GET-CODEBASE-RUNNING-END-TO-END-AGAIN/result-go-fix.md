# Result go-fix (wave 1021a): gopls "undefined" symbols are canonical-layout false positives, no code defect

Root `M` = `🧰️framework/🛍️products/🦑️repo/🔨️modules`. Static analysis only: the coordinator HOLD (swap 19.7/20.5 GB) forbade any go build/vet/test, so nothing was compiled in this task; the verdict rests on the last green runs (result-go-all/-go-all2: all 32 modules build + vet + test under the canonical overlay, 0 failures) plus the checks below.

## Where every reported symbol is defined (all exist, all in tracked `🐹️.go`)
- `Server`, `NewServer`: `M/🖥️server/🎛️coordinator/🧩️component` (owner module `repo/server`, `package main`) and `.../📦️packages/🐹️go/🌐️server` (`package coordinator`).
- `EventBus`, `NewEventBus`: `.../📡️event` (main) and `.../📦️packages/🐹️go/📡️eventbus`.
- `EventRepository`, `OpenDatabase`: `.../📚️repository` (main) and `.../📦️packages/🐹️go/📚️repository`.
- `languages.ParseGitAuthor`: `M/🗣️languages/📦️packages/🐹️go/🧾️analysis`; `hostContributors`: `M/🧑️contributors/.../🗄️store`.
- `model.Contributor/CodebaseContributor/ContributorIcons`: `M/📐️model/.../🧑️contributor`; `model.Statute`, `InferEntityKindFromStatute`: `.../⚖️statutes`; `model.Breach/Territory`: `.../🛡️policy`; `model.Folder/File`: `.../🎛️folder-file`; `model.Goal/GoalDates`: `.../🥅️goal`.
- `workspace.ExecCommand`: `M/🏠️workspace/.../🌿️git`.

## Root cause
Canonical layout by design: each registered member directory holds exactly one `🐹️.go`, and all member dirs of a module share one package clause (`package model`, `package contributors`, `package main`/`coordinator`, ...). The repo's canonical Go plan (`canonicalGoPlan`) merges them into one package in an overlay. A stock language server (gopls) opens the real checkout, sees every member directory as its own package, and reports every sibling-member symbol (and every cross-module symbol that lives in another member dir, e.g. `workspace.ExecCommand`) as undefined. `go build` in the real checkout would show the same; the canonical overlay does not. Not a split/move leftover, not a peer in-flight edit: no `🐹️.go` under `M` was modified since 00:35 today, `git status` for those dirs shows only staged `📋️project.json` changes, last Go-touching commits are 10-18..10-20, and `git log` for the coordinator dir shows no symbol removal.

## Other checks
- Non-`🐹️.go` files under `M`: only `⌨️cli/📦️packages/🐹️go/🚪️entrypoint_unix.go` and `🚪️entrypoint_windows.go` (tracked since 10-20 15:04, platform-gated, registered in the taxonomy) and the fixture `🗣️languages/🧪️tests/🧫️fixtures/🐹️sample.go` (exempt fixture). No stray siblings.
- Version drift (audit-langs-1020e): `go.work` `go 1.25` and every `go.mod` 1.25 vs pinned 1.26.9; provisioned `~/.local/go` is still go1.25.0 (VERSION). Both older than the pin; not related to the undefined symbols; not changed here (build-system area, needs a bootstrap decision, and a toolchain bump would need a full-module re-verification which the HOLD forbids).
- `.claude/settings.json` is the only file mentioning gopls; there is no repo-side gopls overlay configuration.

## Actions
- No live edits, nothing staged (no defect to fix). READY: nothing to apply.
- Not run (HOLD): overlay `go build`/`go vet`/`go test`. Re-run after RESUME if desired: `bun go.ts --label fix --steps build,vet --jobs 1` in `go-all-tools/`.

## Recommendation (not done)
A tool-independent `repo` command that emits a gopls overlay (`-overlay` JSON built from `canonicalGoPlan`) would make editors/LSP agree with the canonical build; today the only authority is the overlay run.
