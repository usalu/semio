# Semio MCP suite verification

The repository test-file runner ran every MCP TypeScript test file importing Bun's test API. All 33 files passed, with 215 tests and zero skips. The scoped-session process received the original parent invocation through that runner. The staging test verified the generated mounts and build prerequisites in the actual workspace.

The external MCP SDK called `ui_focus` and `ui_reveal` against the native binary in an isolated folder with no attached shell. Both tools returned retryable `PLUGIN_UNAVAILABLE` for their valid, distinct inputs. Giving each tool the other's extra fields returned nonretryable `INPUT_INVALID`. The Vitest end-to-end fixture now sends the declared arguments for each tool. The publication revalidation tests run under Bun in the 33-file command.

The native inference-job regression passed its paid-slice execution and exact-wire checks. That test exercises the job machinery. A successful original WFC Wasm solve through MCP requires separate verification.

With the rebuilt native executable selected, the full Vitest MCP configuration passed all 63 tests across eight files in 7.94 seconds. The separate `repo run` build target reached a protected host-capacity directory and was unavailable inside this session.

Artifact creation, export, and shell snapshot reads previously collapsed channel refusals into `INTERNAL` and command refusals into `SIDE_EFFECT_REJECTED`. Both paths now use the same typed mapping as action dispatch. The SQLite oracle matched ten fixture combinations. The native regression on these changes is queued separately from the 63-test run.

The 01:25 native relink passed all 63 Vitest checks. An independent SDK with only workspace-read scope listed 28 tools and five prompts, resolved English context, answered ping, and verified that the denied write left no artifact. A fresh SDK requested export on a persisted WFC artifact before opening it. The gateway selected the guest and returned retryable `BUDGET_EXCEEDED` after 219.390 seconds and two completed guest turns. A subsequent open returned the artifact. On a separate Note export the SDK cancelled during guest opening, pinged the same process, and verified the original 306-byte Pack and 216-byte journal.

The Note creation and seeded Add Block calls both returned the same concrete original cleanup refusal after two guest turns. The SDK creation attempt returned in 191.245 seconds, left the requested artifact absent and answered ping. Complete Note mutations and archive bytes remain unverified.

The same native relink drove the one-pixel WFC input through two real inference requests on the original component. The standard request returned retryable `BUDGET_EXCEEDED` at 26.175 seconds when the next header required 93 items and 54 remained. The larger request reached its semantic deadline and returned at 321.362 seconds. Independent snapshot checks recovered the original 306-byte Pack and 204-byte journal.

Replaying the original Note failure exports exposed the WASI monotonic clock import on the cleanup path. The host previously accepted only its operation clock on that path. The interpreter and import changes compiled against the pinned native metadata. Verification of the revised path through the SDK awaits the next full link.

The 02:05 native gateway contains the clock change. Against the original Note component the SDK reached its wall budget before completing the initial guest turn, so this run did not exercise the corrected cleanup path.

The genuine WFC paid bitmap route passed its one-pixel regression in 0.11 seconds on a coherent native dependency snapshot. It drove the real worker, decoded the original payload, checked pixels `AA==`, contradiction false and positive-zero entropy, and closed the worker, document pair, decoder and source. Its first run exposed an input cursor accidentally reused for copying the output. That cursor now resets before output copying.

The same one-pixel document also passed through the registered native `semio.infer` entry. The test starts the original job through the reactor ABI, verifies the response envelope and requires the job registry and source to finish empty. Both direct and registered cases passed together in 0.47 seconds. These tests run through the native producer. The separate SDK test uses its published Wasm component.

The rebuilt Note Wasm has SHA-256 `22d2393505844cd972e914a40d1efe0adad7bd98545b086d41800b05ca0b7a38`. Its descriptor matched the authored descriptor and its prepared actor passed validation. The Wasm guest captured and acknowledged sequence one, then recovered on the same instance in a separate forced-failure test. Independent native replay matched all 19 memory reads and both input receipts. The input required 616,106,101 fuel and 95.37 seconds of interpreter time, excluding preparation.

Two fresh SDK processes reopened the seeded Note against the rebuilt actor and recovered its exact Pack and SPR. The full protocol journey and seeded SDK Add Block reached one completed turn and then returned the same `INTERNAL` error from the failure receiver. A tag-order mismatch in the native host was identified at both failure fetch and producer capture. Client-side mutation and its history remain pending.

The linked MCP port matched all ten refusal cases against the shared fixture. Direct faults and error frames each preserved the producer’s diagnostic, public error code and retryability for budget exhaustion, cancellation, plugin unavailability, command rejection and invalid input. The independent SQLite check matched the ten cases as well.
