# Cargo Typing Edit Resolution

## Actual First Failure

Root73042 is terminal1. Its authenticated actual Cargo lock preparation refused missing `semio-framework-tool-typing-edit`, before the shell exporter feature request was evaluated. The real target critical path was45.6seconds after graph planning. The shell nonexistent-feature issue remains a prospective separate defect; it must not be labeled the cause of this run.

The only current tracked Source Cargo reference found by whole-Source read-only Git grep is `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/⌨️typing/✍️edit/📦️packages/🦀️rust/Cargo.toml:25`, which points to `🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/✍️edit/📦️packages/🦀️rust`. The expected Cargo.toml and entire package directory are absent. Root Cargo members explicitly include the plugin consumer, so workspace lock resolution encounters this unrelated crate before running selected shell tests. A framework filesystem Cargo scan independently found that same sole dependency declaration.

## Existing Neutral Domain Implementation

The real domain-neutral implementation already exists and is tracked at `🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/✍️edit/🦀️.rs`, together with its three authored Rust/JSON test files. It defines OriginalTypingEdit, OriginalTypingEditRequest, OriginalTypingEditResult and OriginalTypingFold. It depends on existing parent tool-machine TypingLedger/TypingState/ToolStep, job Original control, value owned retirement and replication identities. The plugin consumer explicitly imports those four edit types from semio_framework_tool_typing_edit. This is a real implemented domain boundary with missing package wiring, not a missing algorithm.

Parent `semio-framework-tool-machine` has the actual canonical parent manifest under tool-machine/📦️packages/🦀️rust, but mounts only typing/🦀️.rs, not typing/edit/🦀️.rs. Replacing the consumer dependency by the parent crate alone would not provide the requested API. Mounting the child into that parent unchanged would also retain its self-crate import assumptions, so it is not a proven path-only repair. Do not introduce an alias crate, shim, empty crate or compatibility reexport.

## Git Evidence and Ownership Limits

Read-only Git status restricted to both neutral tool-machine and plugin typing-edit trees showed only one unrelated modified node-graph-edit schema. No typing-edit deletion, addition or rename appeared. Git ls-files confirms the neutral implementation/tests and plugin manifest/implementation are tracked, but no neutral package manifest. Git log restricted to that exact missing manifest returned no history in the currently reachable history. This supports an authored unmaterialized package boundary, not an evidenced ongoing filesystem move. It does not establish another agent intent; coordinate the owning domain before edits.

## Clean Repair Boundary

Restore coherent actual package ownership around the existing neutral implementation: a real package manifest should bind the existing implementation, exact real dependencies and existing authored tests, with matching root workspace/build owner registration where required. The natural existing consumer crate name and domain tree already identify the intended standalone boundary. Do not duplicate the implementation or add a replacement stub. Independently prove path resolution and metadata/owner registration before the single actual Cargo retry; preserve root lock/admission and Original guards.

This audit changed only this Markdown report. No Cargo, tests, compiler, Native, process, lease or modifying Git operation was performed.
