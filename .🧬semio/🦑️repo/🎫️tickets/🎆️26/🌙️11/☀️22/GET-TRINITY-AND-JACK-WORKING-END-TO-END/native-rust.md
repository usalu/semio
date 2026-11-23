# Native Rust side of Trinity and Jack on Windows

Status: partial. The dev graph test binary builds, links and runs on Windows; the rest of the scope is not run yet (see "Not run").
The repo fleet (`repo run`, `repo show project`) never admitted anything (queue on `build:source-admission`), so everything ran with native cargo.

## How the native cargo runs are done

- Env: `CARGO_BUILD_RUSTC=rustc`, empty `CARGO_BUILD_RUSTC_WRAPPER` (the repo `.cargo/config.toml` forces rustc through the repo launcher, which refuses plain cargo), `CARGO_INCREMENTAL=0`, `CARGO_PROFILE_DEV_DEBUG=0`, `-j 8`, target dir on `G:` (C: was repeatedly full; incremental caches reached 57 GB), `CARGO_ENCODED_RUSTFLAGS=-Zthreads=0 -Zshare-generics=n`.
- Target and build dir must be ASCII: `link.exe` fails with LNK1104 on emoji paths (the ticket `🗑️generated` dir cannot hold a target dir).
- `-Zshare-generics=n` is required for the dev graph test exe: with the default every upstream generic instantiation (`drop_glue`, `Result` impls) is expected from an rlib that does not export it and `link.exe` ends with LNK1120 (4355 unresolved externals).
- The Jack workspace needs `🤖️generated` (git-ignored). `repo ./📜️script.ts graph-generate` needs the repo process envelope, so the ticket helper `generate-graph.ts` calls the framework graph renderer directly: `repo generate-graph.ts C:/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack`.

## Findings and fixes

1. Missing manifests. `🛂️manifest.jsonnakagin.manifest.json` (jack) and `🛂️manifest.jsonrewrite-lhs.manifest.json` (rewriting) were deleted by the TS agent; the graph generator discovers `*manifest.json`, so nothing generated and `jack-model` failed with "couldn't find file 🤖️generated/🏢️nakagin/🦀️.rs". I restored the content from HEAD as `🛂️manifest.json` in both artifacts (the repo convention, see `wires`, `writer`) and generated both `🤖️generated` trees.
2. Jack graph window config owner lacked `store_disposer_birth` (the framework default refuses with "Window owner must declare its original disposer birth"). Added it (same as the Rewriting owner). Without it the first mounted app boot of every test failed.
3. Dev graph test fixture `ActionIngressSource` lacked the new `original_press_selected` field.
4. Framework `semio-plugin` re-export `history_edit_acceptance` pointed at helpers that were being rewritten, and the acceptance crate had a duplicated `assert_mounted_history_overwrite` (an accidental double paste in `🧪️history-edit-acceptance/🎛️acceptance-policy-pump-and-drive/🦀️.rs`); removed the duplicate, kept the original re-export.
5. Fixture drift `from_json_rejects_wrong_schema`: the document fixture JSON had no `manifestId`, so the decoder refused earlier than the schema check. Added `"manifestId":"nakagin"`.
6. Original operation wires without a conflict path issuer: the framework now requires `with_original_target` ("original operation has no authored conflict path issuer"). Added `whole_document_target` to the Jack document wire (`🕹️play-app`), to the Semio graph member wire (stdio semio editor) and to the Rewriting binary mutation wire.
7. Reserved commit metadata was never paid: `ReservedCommitMetadata` asserts in `Drop` that its `ActionMeta` retired, but neither `step_framework_reserved_commit` nor the close ladder ever called `close_step`. Both now pay it before the commit or its lease retires (`🧷️command-routes/📥️staging/🧊️reserved-routes/🦀️.rs`, `📥️staging/🔒️close-ladder/🦀️.rs`). Another agent landed the same change in the reserved-routes file at the same time; the file has one block.

## Stack overflow: root cause (not fixed yet)

`-Zprint-type-sizes` on the dev graph test crate shows the cause is size, not recursion:

- `VcsArtifactApp<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, SemioMembers>` is 927,376 bytes inline (`JackTestApp` 927,392). Largest fields: `ToolOperations` 271 KB, `AgentLanePreview` 232 KB, `PendingReserved` 191 KB, `TimeTravelLedger` 168 KB, `ConfigurationWorker` 128 KB, `TimeTravelStoreState` 112 KB, `ToolMachineRuntime` 81 KB.
- Every test future that owns an app is about 2.1 MB (`async block` at 2,164,000 to 2,206,000 bytes; `probe_agent_lane` and `probe_declared_verbs_where` 2,174,000). A 2 MiB test thread cannot even hold one such future in a debug build, so every mounted test overflows on any default stack, independent of recursion depth. Stack sweep: overflows at 16 and 32 MiB, runs at 64 MiB and above.
- The construction path builds the app in place on the heap (`🧱️construction/🧺️sources`, `initialize_field_at(addr_of_mut!((*target).field))`) and then returns it by value (`frame.assume_init()`), which is where the 927 KB lands on the stack again.
- Fix direction (framework): box the five or six largest owners inside `VcsArtifactApp` (`pending`, `tools`, `time_travel`, `tool_machines`, the configuration worker) and extend the in-place initializer to allocate the boxes, or return `Box<VcsArtifactApp>` from the registered-app constructors, so the app handle stays a few hundred bytes. Boxing only in the trinity test helper does not help because `new_app_with_registry_and_members` still moves the 927 KB through its own future.

## Dev graph test runs (`semio-s-dev-trinity-jack`, 79 tests, 256 MiB stack, 4 threads)

- Before the fixes above: 55 passed, 24 failed (same count as the macOS record).
- After items 2 to 7 (rebuilt on G:, layout tests skipped because the three `reorganize` tool-run laws run past 10 minutes instead of failing at their 120 s deadline): 56 passed, 16 failed of 72, plus 7 layout tests not counted (4 ran and passed before the hang, 3 hung).
- Fixed by the changes: wrong-schema fixture, `query_action_publishes_one_live_child_edit...`, `jack_query_is_document_content_shared...`, inspection panel, node graph select, conflict history edit, the app boot refusal.
- Still failing, all raised inside the framework child-edit publication pipeline rather than in the test code: `original lifecycle child receipt differs from its whole producer receipt` (7), `existing child edit requires its retained original command authority` (7, `🧩️composition/📬️publication/🤝️group/📦️owner`), `operation output payload extent exhausted` (4), `terminal ticket has no explicit native header lifecycle` (2), `whole envelope reached Drop before every original ... owner closed` (3 live envelope and archive tests), `mounted tool metadata lease must return through its original close` (1), `typed operation 'textEdit' advanced 4096 publication units without moving its ladder` (1). These are the "fixture routing drift" of the closure audit: every failing test publishes a child (Semio graph) edit through Jack, and the framework publication group now demands an original command authority and matching receipts that the Jack and Semio child adapters (`🪆️content/🧿️semio`, `🕹️play-app`) do not supply yet. Not fixed.

## Not run

`@semio-tech/trinity-jack-rs`, `-lsp` (incl. wasm), `-shell`, `-rewriting-rs`, `-plugin` and the verify-* targets: the machine was saturated (a single dev graph build took 3.5 hours under load) and C: ran out of space several times. Static checks only: all 14 members of the Jack workspace resolve to real directories (the `🔨️modules` move is fine), `JackLspSession` is exported from `🔨️modules/🧠️lsp/🧩️component/🦀️.rs`, the TS worker imports it.
