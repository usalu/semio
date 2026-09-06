# 🌀️ `artifact::quick` stack overflow — root cause, fix, and the state of `semio-framework-os-mcp --lib`

Lane `fable-mcp-artifact-quick-recursion`, 2026-09-05. Picks up blocker 3 of
`📓️fable-mcp-inference-bridge.md` ("A lane-independent unbounded recursion in the probe-workspace
path").

## 1. Reproduction

```
CARGO_TARGET_DIR=<scratchpad>/fable-mcp-artifact-quick-recursion-target RUSTC_WRAPPER="" CARGO_BUILD_JOBS=4 \
  cargo test -p semio-framework-os-mcp --lib artifact::quick --message-format=short -- --nocapture --test-threads=1
```

17 s (warm private target, APFS-cloned from the sibling lane's `fable-mcp-inference-bridge-target`
with `cp -c -R`, 15 s, no cargo held its lock). Verbatim:

```
running 9 tests
test artifact::quick::artifact_create_then_open_round_trips_for_real_with_exactly_one_resolvable_plugin ...
thread 'artifact::quick::artifact_create_then_open_round_trips_for_real_with_exactly_one_resolvable_plugin' (2906332) has overflowed its stack
fatal runtime error: stack overflow, aborting
```

`lldb` cannot be used to backtrace it in this environment (`attach failed (Not allowed to attach to
process.)`), so the cycle was bisected by running each of the nine tests alone against the prebuilt
test binary. Exactly the four that commit a real probe edit abort; the five that do not are green:

| test | before the fix |
| --- | --- |
| `every_artifact_tool_registers_under_its_declared_name` | ok |
| `every_top_level_schema_is_object_typed_2020_12` | ok |
| `no_workspace_bound_is_a_retryable_plugin_unavailable_for_every_artifact_tool` | ok |
| `missing_required_field_is_input_invalid_before_any_workspace_check` | ok |
| `workspace_bound_with_zero_resolvable_plugins_is_still_plugin_unavailable` | ok (opens a folder workspace, commits nothing) |
| `artifact_create_then_open_round_trips_for_real_with_exactly_one_resolvable_plugin` | **stack overflow** |
| `artifact_validate_is_a_real_typed_gap_never_a_fabricated_pass` | **stack overflow** |
| `artifact_snapshot_returns_real_bytes_for_the_current_revision_and_rejects_a_stale_one` | **stack overflow** |
| `artifact_export_never_fabricates_a_successful_export` | **stack overflow** |

The discriminator is `HeadlessWorkspace::ensure_probe_artifact` / `artifact_create` — i.e. anything
that dispatches a `ProbeMutation` through the probe `ArtifactStore`. `workspace::long::a_headless_
commit_propagates_to_a_second_host_on_the_same_folder` aborted the same way, which is why the whole
`--lib` suite died before printing a summary.

## 2. Root cause — a two-frame infinite recursion, exact file:line

`🧰️framework/🔨️modules/🌱️value/🦀️.rs:313`

```rust
pub fn to_dsl_value<T: ToValue>(value: &T) -> Result<DslValue, String> {
    Ok(value.to_value())
}
```

`🧰️framework/🔨️modules/🌱️value/🦀️.rs:319`

```rust
pub fn from_dsl_value<T: FromValue>(value: DslValue) -> Result<T, String> {
    T::from_value(value).map_err(|error| error.to_string())
}
```

and, at the other end of the cycle,
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` (line numbers as of the pre-fix tree):

- `:215` `impl ToValue for ProbeSnapshot::to_value` → `store::to_dsl_value(self)`
- `:220` `impl FromValue for ProbeSnapshot::from_value` → `store::from_dsl_value(value)`
- `:240` / `:245` — the same pair for `ProbeDiff`
- `:301` / `:306` — the same pair for `ProbeMutation`

`ProbeSnapshot::to_value` calls `to_dsl_value::<ProbeSnapshot>`, which calls `ProbeSnapshot::to_value`.
That is the whole cycle: two frames, no base case, so no stack size can survive it — consistent with
the sibling lane's observation that `RUST_MIN_STACK=1073741824` (1 GiB) changes nothing.

**Why it appeared without anyone editing these lines.** The three hand-written bridges are correct
against the *old* `to_dsl_value`, which was a `serde::Serialize`-bounded bridge — their docstrings
still say "Routes through the same `to_dsl_value`/`from_dsl_value` **serde** bridge". The
2026/09/01 `RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS` sweep redefined
`to_dsl_value`/`from_dsl_value` as thin `ToValue`/`FromValue` forwarders (see their own docstrings:
"first-party analog of the former `serde::Serialize`-bound bridge"). Every derived implementor was
migrated; these three hand-written ones silently became self-calls. It compiles cleanly and there is
no lint for it — the only symptom is a `SIGABRT` at first use.

## 3. Fix

Owner-side, in `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`. The three impls now
bridge through `DslValue`'s own total `From<&serde_json::Value>` / `From<DslValue> for
serde_json::Value` conversions (`🧰️framework/🔨️modules/🌱️value/🦀️.rs:218,236,247,268`), which are
the base cases the trap was supposed to bottom out on:

- `ProbeSnapshot` (`:220`–`:229`) and `ProbeDiff` (`:246`–`:255`) encode **transparently** as their
  inner `serde_json::Value` — exactly what a derived newtype struct emits.
- `ProbeMutation` (`:311`–`:327`) encodes **externally tagged** as `{"SetValue": <payload>}`, keyed
  off `PROBE_SET_VALUE_DESCRIPTOR.aggregate_variant` rather than a second copy of the string. That is
  byte-identical to both `serde`'s default and `#[derive(ToValue)]`'s default for a tagless
  single-unnamed-field variant (`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs:24`–`26`).
  `from_value` rejects a non-object, a wrong variant key, and an entry count other than one.

No stack size was raised, no test was skipped or ignored, no abort was caught. The docstrings now
carry an explicit `🪲️ NEVER route this through store::to_dsl_value/store::from_dsl_value` warning so
the trap is not re-laid.

### Regression law + language-agnostic fixture

`🧫️fixtures/🔣️first-party-codecs.json` gains a `probeCodec` section (`value`, `snapshotEncoding`,
`diffEncoding`, `mutationEncoding`, `rejectedMutationEncodings`) covering u64/i64/f64/string/bool/
null/array/nested-object leaves. `workspace::quick::probe_codec_encodes_the_fixture_shape_and_agrees_
with_the_third_party_serializer` reads that fixture and asserts:

1. the first-party `to_value` tree, projected to JSON, equals the fixture's pinned encodings;
2. **independent oracle** — `serde_json::to_value(&…)` (third-party) produces the same three
   encodings for the same values;
3. `from_value(to_value(x)) == x` for all three types;
4. every `rejectedMutationEncodings` entry is an `Err`.

Reaching any assertion at all is itself the proof the cycle is gone. The fixture is the
language-agnostic artifact; no TypeScript oracle was added because the probe codec has no TS
implementation to cross-check against (see nonclaims).

## 4. Commands and results

All runs: `CARGO_TARGET_DIR=<scratchpad>/fable-mcp-artifact-quick-recursion-target`,
`RUSTC_WRAPPER=""`, `CARGO_BUILD_JOBS=4`, `--message-format=short`, foreground, one cargo at a time.

| # | command | duration | result |
| --- | --- | --- | --- |
| 1 | `cargo test -p semio-framework-os-mcp --lib artifact::quick -- --nocapture --test-threads=1` (pre-fix) | 17 s | `fatal runtime error: stack overflow, aborting` (SIGABRT) |
| 2 | per-test bisection against the prebuilt binary (9 runs) | < 5 s total | 5 ok / 4 abort, table in §1 |
| 3 | `cargo test -p semio-framework-os-mcp --lib artifact::quick -- --test-threads=1` (post-fix) | 5 m 31 s (5 m 22 s of it rebuild) | **ok. 9 passed; 0 failed** |
| 4 | `cargo test -p semio-framework-os-mcp --lib probe_codec -- --test-threads=1` | 29 m 23 s (29 m 10 s rebuild — a peer had touched the upstream crates) | **ok. 1 passed; 0 failed** |
| 5 | `cargo test -p semio-framework-os-mcp --lib artifact::quick -- --test-threads=1` (re-confirm after restart) | 34 s | **ok. 9 passed; 0 failed** |
| 6 | `cargo test -p semio-framework-os-mcp --lib` | 53 s | aborts, see §5 |
| 7 | `cargo test -p semio-framework-os-mcp --lib -- --skip a_headless_commit_propagates_to_a_second_host_on_the_same_folder` | 42 s | **FAILED. 301 passed; 14 failed; 0 ignored; 1 filtered out** |

Raw stdout of runs 6 and 7 is kept at
`🗑️generated/fable-mcp-artifact-quick-recursion/full-lib-abort.txt` and
`…/full-lib-skip-one.txt`.

## 5. The whole `--lib` suite still cannot print a summary — a second, unrelated abort

Run 6 (`cargo test -p semio-framework-os-mcp --lib`, no filter) never reaches a `test result:` line.
Verbatim tail:

```
thread 'workspace::long::a_headless_commit_propagates_to_a_second_host_on_the_same_folder' (3481765) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/../../🏠️workspace/🦀️.rs:2470:26:
no RemoteMutations before the 20s deadline: Err(Elapsed(()))

thread 'workspace::long::a_headless_commit_propagates_to_a_second_host_on_the_same_folder' (3481765) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🏪️store/🦀️.rs:16810:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
…
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
```

Two separate defects, both outside this lane:

- **the surface failure** — a headless commit does not reach a second `ArtifactHost` bound to the
  same folder within 20 s (`🏠️workspace/🦀️.rs:2470`). This test was previously masked by the stack
  overflow, so it is newly *visible*, not newly broken. It is very likely the flake its own comment
  at `🏠️workspace/🦀️.rs:2456`–`2464` already documents: a peer measured 9 of 10 runs delivering
  `RemoteMutations` "in well under 1s", with the single timeout occurring on a box running "several
  DOZEN concurrent `cargo`/`rustc` processes from unrelated sibling tickets". That is exactly the
  state of this box during run 6 — a peer's `space-public-boundary-sol-target` build plus `sccache`
  were saturating it. Treat it as contention until someone reproduces it on a quiet machine;
  this lane did not re-run it in isolation and makes no propagation-gap claim;
- **the amplifier** — `ArtifactStore`'s `Drop` witness at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:16810` panics while the first panic is
  unwinding, which Rust escalates to a non-unwinding abort. Any assertion failure that leaves a
  live `ArtifactStore` therefore kills the whole test process and erases the summary for all 316
  tests. Worth an owner: a `Drop` witness should not panic during unwind (`std::thread::panicking()`
  guard).

Run 7 skips only that one test to recover a real count: **301 passed; 14 failed; 1 filtered out**
(315 of 316 run; `artifact::quick`'s nine are all in the passed set).

### The 14 remaining failures, for the coordinator (none touched by this lane)

| test | first diagnostic line |
| --- | --- |
| `root::quick::action_prepare_tool_call_returns_a_prepared_action_report_for_a_granted_scope` | `CallToolResult { … "unknown capability: cad.editor.translateSelection" … is_error: true }` |
| `root::quick::action_prepare_tool_call_is_permission_denied_for_a_scope_the_principal_lacks` | ``assertion `left == right` failed`` |
| `root::quick::context_resolve_tool_call_returns_a_context_summary_with_the_catalog_hash` | ``assertion `left == right` failed`` |
| `root::quick::action_invoke_tool_call_commits_a_prepared_capability_end_to_end` | ``called `Option::unwrap()` on a `None` value`` |
| `inference::quick::declared_inferences_for_workspace_finds_the_real_procedural_roster` | `procedural is the sole plugin owner: GatewayError { code: Internal, message: "…/🌀️procedural/🔣️.json did not decode as a PackageDescriptor: unknown variant \`kind\`, expected one of \`onCommand\`, … at line 4 column 12" … }` |
| `inference::quick::gis_inference_discovery_reads_committed_descriptor_through_registered_mcp_tool_without_execution_authority` | `committed GIS descriptor must load` |
| `conformance::quick::note_and_cad_fixtures_produce_zero_conformance_findings` | `unexpected findings: [Finding { … "Mutation-kind capability declares no writes" }, … "input schema failed to compile as JSON Schema 2020-12: … unsupported JSON Schema keyword \`pattern\`" …]` |
| `conformance::quick::bare_action_id_grammar_violation_is_flagged` | `assertion failed: findings.iter().any(\|finding\| finding.message.contains("must start with"))` |
| `prompts::quick::no_prompt_names_a_specific_plugin_or_artifact_kind` | `safe_mutation names the plugin \`note\`` |
| `bridge::quick::bounded_shell_decoder_and_materializer_advance_incrementally` | ``assertion `left == right` failed: one preflight grant may consume only one scalar token`` |
| `transport::quick::incremental_bridge_decode_cancellation_and_stale_generation_retain_exact_raw_owner` | `assertion failed: payload.len() <= 125` |
| `transport::quick::request_line_and_header_delimiter_search_faults_at_cap_without_scanning_late_crlf` | `assertion failed: matches!(state.drive_one(1), HttpTurn::MoreWork)` |
| `transport::quick::terminal_public_fifo_preserves_generation_aba_and_process_close_is_one_owner_per_grant` | ``called `Result::unwrap()` on an `Err` value: Os { code: 35, kind: WouldBlock, message: "Resource temporarily unavailable" }`` |
| `workspace::quick::mcp_probe_document_transport_binds_full_scope_and_exact_surface_authority` | `probe source: Os { code: 2, kind: NotFound, message: "No such file or directory" }` |

Notes the coordinator will want:

- The six `root::quick`/`inference::quick`/`conformance::quick` failures are blocker 2 of
  `📓️fable-mcp-inference-bridge.md` — ~forty peer-corrupted `🔣️.json` plugin descriptors. Sample
  from run 7's own stdout: ``[mcp registry] skipping plugin `animate`: Internal: …/🎞️animate/🔣️.json
  did not decode as a PackageDescriptor: missing field `windowKindId` at line 2579 column 9``.
  Nothing in the MCP crate can fix these.
- `workspace::quick::mcp_probe_document_transport_binds_full_scope_and_exact_surface_authority` is
  **broken by construction**, independently of the path problem. Its last line is
  `assert!(!std::fs::read_to_string(std::path::Path::new(file!())).expect("probe source").contains("probe_document_socket_surface"))`
  — (a) `file!()` is workspace-root-relative while a test's cwd is the package root, hence the
  `NotFound`; and (b) even with the path repaired the assertion reads its own source, which contains
  that very string literal in the assertion itself, so it can never hold. It needs a different
  authority witness, not a path fix. Deliberately left alone — it is not stale-test drift, and it is
  in a region other lanes are editing.

## 6. Same defect, still live, in a second crate

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` carries a
literal copy of the trap for `NativeSocketProbeSnapshot` (`:15906`, `:15913`),
`NativeSocketProbeDiff` (`:15935`, `:15942`) and `NativeSocketProbeMutation` (`:15992`, `:15999`) —
all six are `store::os_store::to_dsl_value(self)` / `store::os_store::from_dsl_value(value)` inside
the matching `ToValue`/`FromValue` impl, all `#[cfg(not(target_arch = "wasm32"))]`. Any native
socket-probe commit will abort exactly the same way. Left to the renderer's owner: it is a different
crate on a long build, and this lane holds no evidence from running it. A repo-wide sweep for
`to_dsl_value(self)` / `from_dsl_value(value)` inside a `ToValue`/`FromValue` impl found no other
instances — every other call site is a legitimate caller (`OpBinary::encode_op`,
`ArtifactPack::encode_pack_with`, host decode paths), and `SpaceHistorySnapshot`/`SpaceHistoryMutation`
derive their `ToValue`/`FromValue`, so those are safe.

## 7. Files changed by this lane

```
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧫️fixtures/🔣️first-party-codecs.json
```

No file was created, renamed or removed, so no registered target changed and
`@semio-tech/plugin-registry:check-generated` was **not** run — stated as a nonclaim below rather
than a green.

## 8. Nonclaims

- The `--lib` suite is **not** green. 301/315 pass with one test skipped; the unfiltered run still
  aborts. This lane fixed one abort, not the suite.
- No claim that any of the 14 remaining failures is caused by, or fixed by, this change. They live
  in `root`/`inference`/`conformance`/`prompts`/`bridge`/`transport` and in one workspace authority
  test, none of which construct a `ProbeSnapshot`/`ProbeDiff`/`ProbeMutation`. Their pre-fix status
  is unmeasurable because the process aborted before libtest could report — the fix is what makes
  them observable.
- `workspace::long::a_headless_commit_propagates_to_a_second_host_on_the_same_folder` is **not**
  fixed. It now fails on a real 20 s propagation deadline instead of overflowing the stack; whether
  folder-to-folder replication is genuinely broken, or this is the box-contention flake its own
  in-file comment documents, was not determined. It was observed failing exactly once, under heavy
  peer build load, and was not re-run in isolation.
- The `[DEBUG]` string at `🏠️workspace/🦀️.rs:2457` is a peer's prose reference inside a comment, not
  a live log, and not this lane's. It was left untouched.
- The renderer twin in §6 was found by reading, not by running. No renderer build or test was run by
  this lane.
- No TypeScript/Bun/AJV oracle was added or run for the probe codec; the third-party oracle is
  `serde_json` inside the Rust law, and the shared JSON fixture is the language-agnostic artifact.
- `lldb` could not attach in this environment, so the cycle was established by source reading plus
  per-test bisection and then confirmed by the fix turning all four aborting tests green — not by a
  captured backtrace.
- No `[DEBUG]` logging was added at any point; none to remove.
