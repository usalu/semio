# Configured semio MCP client results

On 25 November 2026, the installed Model Context Protocol SDK launched the semio entry from the checkout's .mcp.json with its original arguments, scopes, operation policy, bridge and folder. The check explicitly removed SEMIO_OS_MCP_BIN. The client discovered 28 tools, 5 prompts and 15 resources in the existing workspace, read semio://workspace, resolved both en and de contexts, and verified that an invalid search limit returned INPUT_INVALID. The SDK reported no client errors. The SDK run exited successfully. Its transport does not expose the server's final exit status.

A separate raw client used an isolated ticket workspace, disabled the bridge and enabled approval for that test process. It initialized in 0.6 s and reached the native catalog by 12.9 s. The native replies included a created artifact revision, a snapshot and diagnostics for the probe document. Its missing-token undo request returned INPUT_INVALID. The launcher initially exited with status 1 on client EOF. The dependency provisioning child had reported its own typed cancellation error while the gateway expected the identity of its staging signal's reason. The provisioning boundary now preserves that reason for this specific cancellation. The regression failed before the change and passed after it. All 69 command registry cases passed. The 70 gateway and client journey test cases also passed.

The published Rust server and committed schema mirrors differed, including the inference progress cursor ceiling. Both schema mirrors were regenerated from the published native executable using the repository's TypeScript renderer. Both match byte for byte. AJV resolved all 73 exports, and all four schema contract tests passed. The ticket's verify-published-schemas.ts checks the mirrors with the renderer from the production source.

The SDK and probe checks do not establish that the typed Note, undo/redo, transaction or WFC inference journeys pass.

The direct published executable passed the strict probe journey in 1.0 s, including a zero exit on EOF. A separate sandboxed launcher-disconnect check reproduced exit 1 with the macOS boot-session query denied during publication admission. This is a different exception from the background provisioning cancellation repaired above. A second host-level EOF exit code after the repair has not been measured.

The guest Poll loop contained a temporary once-per-second diagnostic that dispatched an additional metered operation to read state. Its execution frequency depended on the system clock. The diagnostic and its unused state formatter were removed. Both Poll session contract tests passed. This source change has not yet been verified in a rebuilt executable.

The installed SDK also connected to two successive instances of the published executable in one isolated workspace. The first created and snapshotted a probe document. The second opened it by identifier, and the snapshot returned byte-identical pack and journal content. The test removed its temporary workspace after the second disconnect.

## Codex desktop connection

This chat's attached semio connector fails before it receives an initialize response. The repo connector in this chat also resolves repository resources under the filesystem root. Starting the semio command from that directory independently reproduces an immediate exit with “workspace root not found from /”. The SDK tests started in the repository. This establishes a reproducible directory-dependent failure. The desktop log reports the failed handshake without giving the child's directory or stderr.

The repository's Codex config generator now omits the explicit relative working directory. Codex's local stdio launcher uses the caller’s runtime working directory when this field is absent. With a configured value it passes that value directly to current_dir. The project config supplied to this session still contains the relative override. The managed workspace marks .codex read-only. The ticket's codex-project-cwd.patch removes only the two cwd lines and preserves both servers' existing approval settings. It was prepared but not applied. No permissions change was requested.

The config generator now matches its updated language-neutral fixture under the TOML readers. Of 25 configuration tests, 22 passed; the three failures compare the live protected config, or a copy of it, against generated text. The protected file also contained approval settings omitted by the pre-existing generator. The two-line patch preserves those settings.

[OpenAI's local stdio launcher source](https://github.com/openai/codex/blob/main/codex-rs/rmcp-client/src/stdio_server_launcher.rs)

A separate invocation of the installed Codex app-server was attempted with a test-only workspace override. It exited before MCP initialization because the managed sandbox denied opening Codex's local state database. No sandbox exceptions or changes to that database were requested.

## Cold publication

The recorded cold build produced a 609,440,192-byte semio executable and then failed to admit its completed Cargo metadata. A 512 MiB per-file ceiling in that schema disagreed with the native inventory's existing 8 GiB total budget. The completed-capture schema now represents the same budget and the aggregate check still rejects combined files over the limit. The language-neutral test failed on the original schema and passed with the fix, including 12 assertions against AJV and SQLite. A new cold build through publication has not been verified.

The published native executable also answered a stateless server/discover request directly over stdio. The reply included protocolVersion, serverInfo, capabilities and instructions, and the process exited with status zero on EOF. This separate check used the configured host policies and exercised discovery only.

The ticket's direct-client-journey.ts runs the repository's production client journey and owned stdio transport against a supplied executable and explicit model roots. A missing-model control passed the first 14 native protocol and probe-artifact checks, including capability ranking and paging. The next step returned the expected PLUGIN_UNAVAILABLE for the absent Note model. The control run deliberately supplied no model. Its result is independent of the installed-model tests.

## Editor resource funding

The old native host offered an implicit editor guest both the full work budget and all retained resources of the physical host operation. The guest's outstanding reservation then left too little funding for the host to execute the guest and acknowledge its progress. Controlled Note calls reproduced shortages in copy, items and work. Changing only the copy allowance moved the failure to the retained-item dimension. These trials did not produce a document.

The implicit editor Poll and initial JSON import now request a separate finite budget, declared by implicit-editor-request.json beside the channel's budget code. The authored initial caps are 100 million work units, 1,048,576 items, 256 MiB copy, and 128 MiB each of capacity and release. The issuer intersects the request with the operator's physical caps. An explicit inference request continues to supply its own budget. Existing launch policy data is unchanged, so the currently published server can still start. Eight host policy cases passed with 81 assertions. Native verification of this change is pending the isolated build.

The focused native Poll regression ran with the pinned offline toolchain. Under a one-byte-short acknowledgment budget, the before-Input, Input-progress and scalar-progress frontiers now return WorkLimit, preserve the original loan, and reach terminal cleanup. Cleanup required a second repair so a cancelled infrastructure loan does not reserve ordinary transit again. This verifies the refusal path independently of a successful Note operation.

## Rebuilt host

The offline native build succeeded from the current sources with the repository's selected hot-path optimization settings. Both checked-in schema mirrors match the rebuilt executable byte for byte, and AJV resolved all 73 definitions. The official SDK also created and snapshotted a probe on this executable, closed the process, opened the artifact in a second process, and compared identical Pack and journal bytes. All 18 tests in the native Poll test binary passed, including the corrected returned-frame capacity case.

The production client journey with the verified installed Note model passed the first 14 checks. The stock driver then refused typed creation because the remaining physical work could not cover guest admission, cleanup and transport. In an isolated experiment with only the physical work ceiling raised from one to sixteen billion, the guest completed a 745-byte output after 920,157,956 Wasm execution fuel units. The host failed in ParserClose while decoding that output. A second run with larger per-turn allowances hit the same receiver fault. Neither attempt created a Note, and the SDK confirmed the server remained responsive. These physical budgets were diagnostic inputs.

The kernel encoder emits a successful Poll output inside a single Ok member. MCP was trying to decode a bare TurnResult. The kernel now owns a typed success envelope and the native MCP channel receives it before borrowing its result. AJV, SQLite and the neutral cases validated the envelope and rejected bare, error and mixed objects. The rebuilt host progressed beyond the earlier ParserClose failure. With stock infrastructure it then refused the fixed Core wire grant. A diagnostic increase in infrastructure funding moved that run to the 150-second activation wall without producing a Note. The probe lifecycle and discovery remain verified separately.

## Cold activation and lifecycle

A settled SDK refusal now identifies the exact missing stock. The infrastructure guest had 63 retained items left and needed 93 for its 152-byte crossing. Copy, capacity, release and work all covered that crossing. The physical work trial with four billion units also progressed past the one-billion-unit refusal. The default policies now use four billion physical work units, 262,144 work units and 16,384 retained items per drive turn, and 131,072 infrastructure items. The measured-stock regression failed with 63 available before the change and the eight policy tests then passed with 83 assertions.

The owned interpreter and Bun's independent Wasm engine reproduced the Note opening input. Opening and acknowledgment input decoding alone cost about 907 million instructions. The independent engine needed 40 empty follow-up Polls before the guest emitted a genuine Captured receipt. A successful acknowledgment commits the matching lifecycle transition before publishing its turn result, even when that result reports MoreWork for other tasks. The native channel now awaits the exact captured identity and settles its acknowledgment. The neutral lifecycle test covers 15 transitions. A real SDK call with extended time and larger stock still timed out before this lifecycle change, while waiting for idle. No successful typed Note creation is implied by the fixture.

The native server also reports a bounded ownership refusal as BUDGET_EXCEEDED. The rebuilt host successfully verified its 73 schemas and cross-process probe persistence again. A separate SDK run with read-only scope confirmed that attempting to create a probe left no artifact and kept the server available.

The captured receipt appeared after 40 genuine empty continuations in the Bun Wasm replay. ACK then produced a fully settled MoreWork response with no retained input or new receipt. The retained output encodes requestSequence as a quoted decimal, whereas the acknowledgment input encodes it as an integer. The native direction-specific receiver was corrected and the independent codec checks reject crossed forms.

On the installed pre-repair guest, the next measured physical shortage was copy credit. Of the original 16 MiB infrastructure allowance, 23,339 bytes remained when 29,922 were required. The current default uses 32 MiB. The language-neutral policy test failed before the change and passed afterward. All eight host budget cases passed with 84 assertions.

Two wire-compatible guest changes address the measured CPU cost. Canonical empty Poll input can be received directly into its typed owner, respecting the original limits, work and retirement. One native fixture took five control steps and 174 work units with no retained allocation, versus 81 steps, 2,389 work units and 269,163 retained bytes through generic JSON. All ten ActorInput tests passed. Opening work now advances within the existing bounded quiet slice while the same instance is making paid progress. The guest yields on a parked instance, cancellation or deadline. Guest component linkage and the real typed SDK journey are separate verification steps.
