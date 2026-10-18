# Audit: canonical Go build and taxonomy gate vs. split Go files

Auditor: read-only. No git modifying commands, no builds, no edits outside this report. Method: code reading of the current `canonicalGoModulePlan` and gate definitions, plus structural checks (go.mod presence, member-name membership, git status). Not runtime-verified: no `go test` or taxonomy gate was executed.

Note: the library was being refactored concurrently. `canonicalGoModulePlan` is in `📚️library/🟦️.ts` (also copied to `📚️library/🧪️testing/🐹️go-plan/🟦️.ts`, same logic). The gate moved to `📚️library/🔍️discovery/🗂️file-kind-resolution/🟦️.ts`. Line numbers are from the state read at audit time.

## Rules: what the canonical build compiles

- `📚️library/🟦️.ts:983-988` `canonicalGoLayout()`: `implementationFilename` = `canonicalFilenameForKind(testAdapterFileKinds["🐹️go"] = go-source)` = `🐹️.go`; `sourceDirectoryNames` = `semanticDirectoryMemberKinds["members-of-members-of-members-of-modules"].memberNames` (234 names); `opaqueDirectoryNames` = `[🧫️fixtures, 📦️packages]`; tests dir `🧪️tests`.
- `🟦️.ts:1005-1036` `walk(directory)`:
  - `:1006` a non-root dir containing `go.mod` is not descended (separate module).
  - `:1008-1011` every directory that directly contains any `*.go` file becomes a package (`packages.add`). The Go toolchain then compiles all its `*.go` files natively, regardless of basename.
  - `:1015-1021` a child dir whose name is in `sourceDirectoryNames` contributes ONLY `<child>/🐹️.go`, mapped to a virtual `zz_semio_<hash>.go` inside the PARENT package (`map(directory, …)`, `:996-1003`). The child dir is not recursed, so its siblings are never compiled.
  - `:1023-1033` `🧪️tests/<case>/🐹️.go` is mapped as `zz_semio_<hash>_test.go` into the owner package. Other files in case dirs are ignored.
  - `:1034-1035` `.git`, `node_modules`, `target`, `vendor`, `🧫️fixtures`, `📦️packages` are skipped. So `📦️packages/🐹️go` is compiled only as its own module (it has `go.mod`) or as a `replace` target.
- `:1079-1094` `canonicalGoPlan`: for `replace`d local modules only `sourceReplacements` of the dependency are taken. Its `packages` list is discarded, but Go still compiles the dependency's package-dir files natively when imported.
- `:1102-1106` `runCanonicalGoTests`: `go test -overlay=… <packages>`; packages must be in `plan.packages`.
- Net: the canonical build compiles (a) every `.go` in a package directory (any basename; `_test.go` only under `go test`), and (b) only `🐹️.go` leaves of member dirs and `🧪️tests/<case>/🐹️.go` leaves. Sibling files inside member dirs are never compiled.

## Rules: taxonomy gate

- `📚️library/🔣️taxonomy.json:1716-1720` `fileKinds.go-source`: emoji `🐹️`, extension `.go`, role `source`.
- `🔣️taxonomy.json:21-24` `implementationLeafPolicy.roles = ["source"]`, so `go-source` is an implementation leaf. `ignoredPathPatterns` (from `:29`) includes `**/.🧬semio`.
- `🔍️discovery/🗂️file-kind-resolution/🟦️.ts:61-88` `implementationLeafBasenameFindingResolved`: `:87` returns `taxonomy/kind-only-basename` unless `actualBasename === expectedBasename`. `expectedBasename` = `🐹️.go` (`canonicalLeafFilenameForSourcePath`, `:42`). Only exemptions: fixed filename contracts (`go-module` = `go.mod` at `:23502`, `go-sum` = `go.sum` at `:23514`) and scoped kinds (none for `.go`).
- `🔍️discovery/🗂️file-kind-resolution/🟦️.ts:98-105` `implementationLeafBasenameFinding` is the single-path entry point used by the taxonomy check.
- `testInPackageImplementations` (`🔣️taxonomy.json:30688-30693`, `go-package-test`, suffix `_test.go`) is validated by schema code (`🔐️taxonomy-authority`, `📑️taxonomy-document`) but NOT consulted by the gate. So `_test.go` siblings are rejected too.
- Consequence: any `.go` leaf whose basename is not exactly `🐹️.go` is rejected, including in `📦️packages/🐹️go/`. Pre-existing HEAD siblings are rejected the same way: `⌨️cli/📦️packages/🐹️go/🔌️mcp.go`, `🔬️_test.go`, `🔭️exhaustive_test.go`, `🖨️render.go`. The gate's own fixture `🧪️tests/🌳️kind-only-basename/🟦️.ts:127` expects `📦️packages/🦀️rust/build.rs` to be flagged, so packages dirs get no exemption.

## Classification of new and modified `.go` files (git status, 426 entries)

| Class | Count | Compiled by canonical build | Taxonomy-admitted | Mechanism |
|---|---|---|---|---|
| registered member leaf | 142 | yes | yes | `🐹️.go` in member dir (`:1015-1021`); owner has `go.mod` |
| test-case leaf | 11 | yes | yes | `🧪️tests/<case>/🐹️.go` (`:1023-1033`) |
| package-root leaf | 14 | yes | yes | `📦️packages/🐹️go/🐹️.go`, native package `.` |
| sibling in package-root module dir | 250 | yes (native; `_test.go` under `go test`) | no, taxonomy/kind-only-basename | `📦️packages/🐹️go/<other>.go`; `:1008-1011` plus gate `:87` |
| stale staging copy | 4 | no (not on disk) | no (not on disk) | `💻️client/⌨️cli/zz-gocomp-stage/*.go`, AD = staged in index, deleted on disk |
| tool-file | 5 | no | ignored (`**/.🧬semio`) | `.🧬semio/…/gocomp*`, `gocli-tools`, `gocomptest` tools |

## Per-executor verdict

| Executor | Report | Leaves (`🐹️.go`) | Sibling `.go` files | Verdict |
|---|---|---|---|---|
| gocomp | result-gocomp.md | 133 new member leaves under `💻️client/⌨️cli/<member>/🐹️.go` plus `🧩️component/🐹️.go` (M): compiled yes, admitted yes | none (member dirs contain only `🐹️.go`) | Canonical. Defect: `zz-gocomp-stage/*.go` staged in index (AD) but deleted on disk; the user must run `git rm --cached`. |
| gotree | result-gotree.md | `🌳️tree`, `🏃️test-runner`, `🪝️hooks`, `🗣️languages` package roots: yes/yes | siblings (`⏲️mermaid.go` … `🧬️input_test.go`): compiled yes, admitted NO | NOT canonical. The claimed `⌨️cli` precedent is itself rejected by the same gate. |
| gogql | result-gogql.md | `🔗️graphql` and `📐️model` package roots: yes/yes | siblings (`🔤️lexer.go` … `🎞️recording_mutations.go`, model `🎁️templates.go` …): compiled yes, admitted NO | NOT canonical, same as gotree. |
| gocli | result-gocli.md | `⌨️cli` and `🚚️move` package roots: yes/yes | 54 cli + 3 move siblings, including pre-existing `🔌️mcp.go`, `🔬️_test.go`, `🔭️exhaustive_test.go`: compiled yes, admitted NO | NOT canonical. Its "no fileKinds change needed" holds for kind matching only; the basename rule still rejects. |
| gocoord | result-gocoord.md | coordinator member leaves (`🪝️webhook`, `🧾️diff`, `📡️event`, `🎯️scope`, `⚠️warnings`, `🚨️breaches`, `🎫️ticket`, `🧩️component`) and test leaves `🧪️tests/<case>/🐹️.go` in coordinator and `💻️client/🔌️mcp`: yes/yes | siblings in `🎛️coordinator`, `🧩️providers`, `🏠️workspace`, `🗂️codebase`, `📊️metrics`, `🧑️contributors` `📦️packages/🐹️go`: compiled yes, admitted NO | Member and test leaves canonical. Package siblings NOT canonical. |
| gocomptest | (no result file) | test leaves listed above: yes/yes | none | Canonical. |
| gofix | result-gofix.md | no split files of its own in git status | n/a | Not a split executor. |

All 426 `.go` entries of git status are classified above; no entry is unclassified. `📐️model` siblings are attributed to gogql (result-gogql.md, section "model").

## Files that would be ignored or rejected

### Not compiled, and stale or ignored

- `💻️client/⌨️cli/zz-gocomp-stage/a_component.go` (AD, stale index entry)
- `💻️client/⌨️cli/zz-gocomp-stage/b_executor.go` (AD, stale index entry)
- `💻️client/⌨️cli/zz-gocomp-stage/c_export.go` (AD, stale index entry)
- `💻️client/⌨️cli/zz-gocomp-stage/d_kinds.go` (AD, stale index entry)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gocli-tools/main.go` (tool, ignored by `**/.🧬semio`)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gocomp/prune.go` (tool, ignored by `**/.🧬semio`)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gocomp/scan.go` (tool, ignored by `**/.🧬semio`)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gocomptest/prune/main.go` (tool, ignored by `**/.🧬semio`)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gocomptest/scan/main.go` (tool, ignored by `**/.🧬semio`)

### Compiled but taxonomy-rejected: package-root siblings (250 files, all `kind-only-basename`)

- `⌨️cli/📦️packages/🐹️go/` (57): `🔌️mcp.go` (tracked, M), `🔬️_test.go` (tracked, M), `🔭️exhaustive_test.go` (tracked, M), `⚖️exhaustive_parity_test.go`, `🌊️stream.go`, `🌲️tree_test.go`, `🌳️tree.go`, `🎟️tool_ticket.go`, `🎫️ticket.go`, `🎯️goal.go`, `🎲️g1_test.go`, `🎼️mcp_prompt.go`, `🏁️benchmark.go`, `🏬️mcp_resource.go`, `🏭️mcp_factory.go`, `🏷️ticket_test.go`, `🏹️tool_goal.go`, `💬️interaction.go`, `📃️exhaustive_list_test.go`, `📋️list_test.go`, `📏️loc.go`, `📑️markdown_test.go`, `📜️statute.go`, `📝️todo.go`, `📡️engine.go`, `📤️export.go`, `🔁️exhaustive_lifecycle_test.go`, `🔃️sync_test.go`, `🔄️update.go`, `🔍️exhaustive_query_test.go`, `🔎️search.go`, `🔐️auth_sync.go`, `🔗️mcp_test.go`, `🕹️command.go`, `🖥️mcp_adapter.go`, `🖼️format_test.go`, `🗂️codebase.go`, `🗣️mcp_description.go`, `🙅️wrong_args_test.go`, `🚚️move.go`, `🚩️flag.go`, `🛎️mcp_tool_repo.go`, `🛠️tool_test.go`, `🥅️goal_test.go`, `🥾️bootstrap_test.go`, `🧑️contributor.go`, `🧩️mcp_protocol.go`, `🧪️test_command.go`, `🧬️mcp_tool_codebase.go`, `🧭️command_test.go`, `🧮️exhaustive_tool_test.go`, `🧱️tool_codebase.go`, `🧷️hook_test.go`, `🧾️export_test.go`, `🩺️analyze.go`, `🪝️hook.go`, `🪞️projection.go`
- `🌳️tree/📦️packages/🐹️go/` (17): `🔬️_test.go` (tracked, M), `⏲️mermaid.go`, `🌿️nodes.go`, `🎊️query-cache.go`, `🎯️goals.go`, `🏩️goal-render.go`, `💿️records.go`, `📌️cache.go`, `📜️statutes.go`, `🔌️ports.go`, `🔍️search.go`, `🔦️search_test.go`, `🔨️build.go`, `🖨️render.go`, `🖼️render_test.go`, `🧫️filter_test.go`, `🪞️identity_test.go`
- `📐️model/📦️packages/🐹️go/` (39): `🔬️_test.go` (tracked, M), `⚖️statutes.go`, `⚖️statutes_test.go`, `🆔️artifact_id.go`, `🆔️artifact_id_test.go`, `🎁️templates.go`, `🎟️ticket_stream.go`, `🎡️inputs.go`, `🎥️repo_context.go`, `🎫️ticket.go`, `🏗️technology_bundle.go`, `🏷️kinds_test.go`, `📁️folder_file.go`, `📈️metrics.go`, `📊️statute_info_table.go`, `📋️allowed_test.go`, `📼️ticket_data.go`, `🔖️section_definition.go`, `🔗️codebase.go`, `🔡️kind_codes.go`, `🔢️artifact_id_work_test.go`, `🔣️emoji.go`, `🖥️entity_emojis.go`, `🗒️draft_todo.go`, `🛡️policy.go`, `🥅️goal.go`, `🦀️hooks.go`, `🦀️hooks_test.go`, `🧊️ansi.go`, `🧑️contributor.go`, `🧬️artifact_spec_test.go`, `🧭️artifact_path.go`, `🧮️artifact_ref.go`, `🧷️artifact_uri.go`, `🧷️artifact_uri_test.go`, `🪨️entity_rendering.go`, `🪨️entity_rendering_test.go`, `🪪️scope_ids.go`, `🪵️hook_log.go`
- `🔗️graphql/📦️packages/🐹️go/` (42): `🔬️_test.go` (tracked, M), `⚡️execution.go`, `🌐️schema_roots.go`, `🌳️ast.go`, `🎞️recording.go`, `🎞️recording_mutations.go`, `🎞️recording_queries.go`, `🎫️tickets.go`, `🎫️tickets_test.go`, `🏗️schema_code.go`, `🏛️schema_governance.go`, `💬️autofix_comments.go`, `💻️mutation_resolvers.go`, `📁️filesystem.go`, `📃️sdl.go`, `📋️autofix_requirements.go`, `📑️autofix_docstring.go`, `📑️autofix_docstring_test.go`, `📝️autofix_summary.go`, `📰️todos.go`, `🔄️management.go`, `🔍️autofix_scan_test.go`, `🔤️lexer.go`, `🔧️autofix_system.go`, `🕳️default_context.go`, `🖼️source_helpers.go`, `🗂️query_resolvers.go`, `🗃️sources.go`, `🗒️drafts.go`, `🛡️policy.go`, `🛡️policy_test.go`, `🥅️goals.go`, `🧑️contributors.go`, `🧩️parser.go`, `🧪️entity_resolvers.go`, `🧱️autofix_sections.go`, `🧱️executor.go`, `🧾️autofix_layout_test.go`, `🩹️autofix.go`, `🩹️autofix_test.go`, `🩻️context.go`, `🪪️autofix_file_policy.go`
- `🪝️hooks/📦️packages/🐹️go/` (19): `🔬️_test.go` (tracked, M), `🎫️checkpoint.go`, `🎫️track_test.go`, `📓️logging_test.go`, `📓️session.go`, `🔖️checkpoint_test.go`, `🔖️micro-commit.go`, `🔶️dispatch.go`, `🔶️run_test.go`, `🚦️run.go`, `🛡️blocking.go`, `🛡️blocking_test.go`, `🧪️test-input.go`, `🧪️test-input_test.go`, `🧬️input.go`, `🧬️input_test.go`, `🧭️payload.go`, `🧮️resolve_test.go`, `🧷️populate_test.go`
- `🏃️test-runner/📦️packages/🐹️go/` (9): `⚙️execution.go`, `🌲️snapshot.go`, `📊️parsing.go`, `📖️extract.go`, `📰️generate.go`, `🔎️resolution.go`, `🕸️test-command.go`, `🖲️command-files.go`, `🗺️planning.go`
- `🏠️workspace/📦️packages/🐹️go/` (8): `⚙️config.go`, `🌿️git.go`, `🎨️formatter.go`, `📄️files.go`, `🔤️text.go`, `🖍️pattern.go`, `🙈️ignore.go`, `🧱️results.go`
- `📊️metrics/📦️packages/🐹️go/` (8): `⏱️benchmark.go`, `📊️report.go`, `📤️render.go`, `🔌️gitsource.go`, `🕰️time.go`, `🗂️classify.go`, `🧩️numstat.go`, `🧮️pipeline.go`
- `🖥️server/🎛️coordinator/📦️packages/🐹️go/` (14): `🌐️server.go`, `🎫️ticket_handlers.go`, `📋️model.go`, `📚️repository.go`, `📡️eventbus.go`, `🔎️diff_handlers.go`, `🔏️envelope.go`, `🗂️scopes.go`, `🗄️eventstore.go`, `🛡️durability.go`, `🧠️projection.go`, `🧾️diff.go`, `🩹️recovery.go`, `🪝️webhooks.go`
- `🗂️codebase/📦️packages/🐹️go/` (7): `🌳️tree.go`, `🎫️links.go`, `🎯️test_definitions.go`, `📦️bundles.go`, `🗃️files.go`, `🛠️technologies.go`, `🧩️sections.go`
- `🗣️languages/📦️packages/🐹️go/` (14): `⏳️rust.go`, `🎚️go.go`, `🎶️typescript.go`, `🎽️base.go`, `🐍️python.go`, `📇️registry.go`, `📜️csharp.go`, `📝️sections.go`, `📭️data-languages.go`, `🗣️parsing.go`, `🧩️json-sections.go`, `🧾️analysis.go`, `🪅️ruby.go`, `🪶️comments.go`
- `🚚️move/📦️packages/🐹️go/` (3): `🔤️rename.go`, `🧩️section.go`, `🧵️integrate.go`
- `🧑️contributors/📦️packages/🐹️go/` (6): `✍️authors.go`, `🎁️interactions.go`, `🏁️checkpoints.go`, `🔭️session_source.go`, `🗄️store.go`, `🪅️sessions.go`
- `🧩️providers/📦️packages/🐹️go/` (7): `🎄️github.go`, `🎆️editors.go`, `🎯️github_goals.go`, `🏃️process.go`, `🔒️git.go`, `🕳️null.go`, `🪝️hooks.go`

### Pre-existing HEAD siblings, also rejected by the gate

Tracked at HEAD and modified in the working tree, so they predate this split: `⌨️cli/📦️packages/🐹️go/🔌️mcp.go`, `⌨️cli/📦️packages/🐹️go/🔬️_test.go`, `⌨️cli/📦️packages/🐹️go/🔭️exhaustive_test.go`, `🌳️tree/📦️packages/🐹️go/🔬️_test.go`, `📐️model/📦️packages/🐹️go/🔬️_test.go`, `🔗️graphql/📦️packages/🐹️go/🔬️_test.go`, `🪝️hooks/📦️packages/🐹️go/🔬️_test.go`. `⌨️cli/📦️packages/🐹️go/🔌️mcp.go`, `🔬️_test.go`, `🔭️exhaustive_test.go` and `🖨️render.go` are tracked at HEAD and get the same rejection.

### Compiled and taxonomy-admitted (reference)

- Member leaves: 142; test-case leaves: 11; package-root leaves: 14.

## Follow-ups (not performed; read-only audit)

1. Rename package-root siblings to a shape both the compiler plan and the gate accept, or move each into a registered member dir as `<member>/🐹️.go`. Only `🐹️.go` leaves pass the gate.
2. Remove the `zz-gocomp-stage` index entries (`git rm --cached`, user action).
3. Fix the pre-existing HEAD siblings (`🔌️mcp.go`, `🔬️_test.go`, `🔭️exhaustive_test.go`, `🖨️render.go`) under the same rule.
4. Run the taxonomy check (`implementationLeafBasenameFinding`) over `📦️packages/🐹️go/**` to confirm the rejection count. This audit did not execute it.

