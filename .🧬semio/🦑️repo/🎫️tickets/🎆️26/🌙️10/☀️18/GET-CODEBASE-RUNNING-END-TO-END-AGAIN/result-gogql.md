# Result gogql: Go split of `repo/graphql` and `repo/model`

Both packages are hand-written (no gqlgen or other generator header or config); split lossless by top-level declaration.
Method: AST (go/parser) declaration extents + a throwaway splitter in `🗑️generated/gogql/tool/` (`split.py`, `spec_graphql*.py`, `spec_model*.py`); originals kept as `🗑️generated/gogql/orig-*.go`. Import blocks recomputed per file; everything else verbatim (docstrings included). Original region markers that wrapped decl groups across the new files (`🚚️Split`, mislabelled/unbalanced regions in model) are dropped; each new file gets its own header region and one wrapping region (whole original regions kept as-is where they map 1:1 to a file).

## graphql (`🔗️graphql/📦️packages/🐹️go`)
- `🐹️.go` 7,262 -> 173 lines (header, imports, Schema type system).
- New (lines incl. header/imports): `🔤️lexer.go` 90, `🌳️ast.go` 130, `🧩️parser.go` 233, `⚡️execution.go` 333, `🧱️executor.go` 155, `🩻️context.go` 213, `🕳️default_context.go` 239, `🥅️goals.go` 481, `🎫️tickets.go` 210, `🗒️drafts.go` 35, `📰️todos.go` 165, `🛡️policy.go` 124, `🩹️autofix.go` 1,071, `🔧️autofix_system.go` 40, `📁️filesystem.go` 147, `🧑️contributors.go` 66, `🔄️management.go` 246, `🏗️schema_code.go` 204, `🏛️schema_governance.go` 249, `🌐️schema_roots.go` 163, `📃️sdl.go` 182, `🖼️source_helpers.go` 404, `🗃️sources.go` 311, `🗂️query_resolvers.go` 291, `🧪️entity_resolvers.go` 142, `💻️mutation_resolvers.go` 489, `🎞️recording.go` 179, `🎞️recording_queries.go` 99, `🎞️recording_mutations.go` 466 (approx, +~20 header/import lines each).
- `🔬️_test.go` 2,016 -> 327; new `🩹️autofix_test.go` 501, `🔍️autofix_scan_test.go` 325, `📑️autofix_docstring_test.go` 179, `🧾️autofix_layout_test.go` 354, `🛡️policy_test.go` 183, `🎫️tickets_test.go` 234.
- OPEN: `🩹️autofix.go` is 1,071 lines because `applyAutofixes` is ONE 1,047-line function (a 13-case `switch` over breach kinds sharing `content/lines/linesToRemove/fixed`; biggest cases 290/232/181 lines). Splitting it needs a behaviour-preserving refactor (per-breach-kind fixers over a shared state struct), which the brief forbids (no new abstractions). Left whole in its own file.

## model (`📐️model/📦️packages/🐹️go`)
- `🐹️.go` 7,010 -> 182 (header, package doc, allowed-values + slug resolution).
- New (all 79..450 lines): `🎁️templates`, `🧊️ansi`, `🖥️entity_emojis`, `📈️metrics`, `🏗️technology_bundle`, `📁️folder_file`, `🔖️section_definition`, `🧑️contributor`, `🗒️draft_todo`, `🎫️ticket`, `📼️ticket_data`, `🎟️ticket_stream`, `🥅️goal`, `🛡️policy`, `⚖️statutes`, `📊️statute_info_table` (440, one 437-line map literal), `🎡️inputs`, `🔗️codebase`, `🎥️repo_context`, `🪪️scope_ids`, `🦀️hooks`, `🪵️hook_log`, `🔣️emoji`, `🔡️kind_codes`, `🆔️artifact_id`, `🧮️artifact_ref`, `🧷️artifact_uri`, `🧭️artifact_path`, `🪨️entity_rendering` (all `.go`). Max 450 lines.
- `🔬️_test.go` 2,909 -> 252; new `🏷️kinds_test.go`, `⚖️statutes_test.go`, `🆔️artifact_id_test.go`, `🔢️artifact_id_work_test.go`, `🧬️artifact_spec_test.go` (595), `🪨️entity_rendering_test.go`, `🧷️artifact_uri_test.go`, `🦀️hooks_test.go`, `📋️allowed_test.go` (all `_test.go`). Max 595.

## Text readers re-pointed
Four Rust doc comments referenced `🔗️graphql/📦️packages/🐹️go/🐹️.go`: `🏗️repo-schema/🦀️.rs` (-> `📃️sdl.go` + `🏗️schema_*.go`), `📦️packages/🦀️rust/🦀️.rs` (-> lexer/ast/parser), `🗄️fs-context/🦀️.rs` and `🗄️fs-context/🏠️context-core/🦀️.rs` (-> `🩻️context.go`). No `go:embed`, project.json, sha pin or taxonomy fixture reads these files by path. Both `project.json`/`📜️script.ts` run `go test ...` (package-level), unchanged. No taxonomy schema lists Go filenames (siblings use the same free emoji stem naming).

## Verification (foreground, in each package dir)
- Before and after: `go build ./`, `go vet ./`, `go test -count=1 -v ./`; additionally after: `go vet -tags exhaustive ./` (graphql/model), `go build ./` in `⌨️cli/📦️packages/🐹️go` (downstream consumer), `gofmt -l .` empty in both.
- model: 93 top-level results before == after (diff of `--- PASS/FAIL` lines empty), 0 failures; 93 `Test*` names identical.
- graphql: 35 top-level results before == after (diff empty); 56 `Test*` names (incl. parse + exhaustive files) identical. One PRE-EXISTING failure, same before and after: `TestSystemPolicy/the_checked-in_devcontainer_block_equals_its_derivation` (workspace settings drift: "run bun repo run workspace:generate-config"), unrelated to the split.
- Losslessness: non-blank-line multiset of all new non-test files equals the original except region marker lines (and header/import lines); same for test files (0 missing).
- Logs and before/after result lists: `🗑️generated/gogql/{before,after}-{graphql,model}*.txt`.
