# Result gocli: Go split of repo cli and move packages

Packages: `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/` (cli) and `…/🚚️move/📦️packages/🐹️go/` (move). Method: AST-driven splitter (`gocli-tools/main.go`, plans `plan-*.tsv`, losslessness check `lossless.py`); declarations moved verbatim by source line range, imports regenerated per file (alias form preserved), `gofmt` applied, `#region` markers re-balanced per file (spanning regions repeated).

## Line counts (before -> after)
| File | Before | After |
|---|---|---|
| cli `🐹️.go` | 6,892 | 27 files, max 562 (`🪞️projection.go`), `🐹️.go` itself 278 |
| cli `🔌️mcp.go` | 2,450 | 9 files, max 564 (`🏬️mcp_resource.go`), `🔌️mcp.go` 208 |
| cli `🔬️_test.go` | 4,013 | 16 files, max 497 (`🧷️hook_test.go`), `🔬️_test.go` 244 |
| cli `🔭️exhaustive_test.go` | 1,978 | 6 files, max 496, `🔭️exhaustive_test.go` 284 |
| move `🐹️.go` | 1,073 | 4 files, max 333 |

Largest file in either package after the split: 724 lines (`🖨️render.go`, untouched, not in scope). Everything else <= 564.

## New files
cli source: `📡️engine.go 📤️export.go 🔐️auth_sync.go 🔎️search.go 🧪️test_command.go 📝️todo.go 🎫️ticket.go 🎯️goal.go 💬️interaction.go 📜️statute.go 🧑️contributor.go 🌊️stream.go 🗂️codebase.go 🚚️move.go 📏️loc.go 🎟️tool_ticket.go 🏹️tool_goal.go 🧱️tool_codebase.go 🩺️analyze.go 🏁️benchmark.go 🪝️hook.go 🔄️update.go 🚩️flag.go 🕹️command.go 🪞️projection.go 🌳️tree.go`
cli mcp: `🎼️mcp_prompt.go 🛎️mcp_tool_repo.go 🧬️mcp_tool_codebase.go 🏬️mcp_resource.go 🗣️mcp_description.go 🏭️mcp_factory.go 🧩️mcp_protocol.go 🖥️mcp_adapter.go`
cli tests: `🥾️bootstrap_test.go 🔃️sync_test.go 🧾️export_test.go 📋️list_test.go 🏷️ticket_test.go 🥅️goal_test.go 🙅️wrong_args_test.go 🖼️format_test.go 📑️markdown_test.go 🧷️hook_test.go 🔗️mcp_test.go 🛠️tool_test.go 🌲️tree_test.go 🧭️command_test.go 🎲️g1_test.go`
cli exhaustive tests: `🔁️exhaustive_lifecycle_test.go 📃️exhaustive_list_test.go 🔍️exhaustive_query_test.go 🧮️exhaustive_tool_test.go ⚖️exhaustive_parity_test.go` (each keeps `//go:build exhaustive`)
move: `🧩️section.go 🧵️integrate.go 🔤️rename.go`
Kept names (rewritten, smaller): cli `🐹️.go 🔌️mcp.go 🔬️_test.go 🔭️exhaustive_test.go`, move `🐹️.go`.

## Verification
- Losslessness: multiset of all trimmed non-blank, non-region, non-import lines of every original equals that of its new files (missing 0, extra 0) for all five files.
- Test counts: `Test*` funcs 105 + 36 (exhaustive) = 141 before and after; `go test -list '.*'` and `go test -tags exhaustive -list '.*'` outputs identical to baseline (106 / 142 entries).
- cli: `go build ./`, `go vet ./`, `go vet -tags exhaustive ./`, `gofmt -l .` clean. `go test -count=1 -v ./` top-level result list (105 entries) is byte-identical before and after: 7 failures both times, all pre-existing (TestMcpBootstrapAssetsStayRepoRelative and six TestG1* tests need `.kiro/.windsurf/.devcontainer` files, native bootstrap strings, and the missing `📚️library/🧪️tests/1️⃣g1-contract/🧫️fixtures/🔣️.json`; not caused by the split).
- move: `go build`, `go vet`, `go test -count=1 -v ./` PASS (16 tests, same as before).
- Regions: `#region`/`#endregion` counts balanced in every file. The original had broken nesting (duplicate/crossed regions around lines 4378-4380, 4988-4992, 750-754); the new files are repaired balanced.
- Text readers: none read these files by path. `📜️script.ts` `policyFile = "🐹️.go"` (10,000-line budget) and `…/⌨️cli/🦀️.rs` `@see …/🐹️.go` still resolve (file kept). Taxonomy `go-source` kind matches by extension only; no fileKinds change needed. Exhaustive tests were compiled/listed, not executed (they were not in baseline either).

## Open
- Exhaustive suite not run (long, needs `-tags exhaustive`).
- Baseline failures above remain.
- Temp splitter sources kept in `gocli-tools/` (input scripts); generated logs removed.
