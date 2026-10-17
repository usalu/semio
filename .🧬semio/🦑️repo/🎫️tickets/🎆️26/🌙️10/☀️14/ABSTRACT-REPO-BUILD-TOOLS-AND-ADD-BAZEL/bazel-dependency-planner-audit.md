# Bazel Dependency Planner Qualification

Nx remains primary. Root MODULE.bazel/.bazelrc remain unchanged: the existing backend deliberately creates owned task workspaces. The current SDK/native/public-runtime blockers in final-report.md remain outside this narrow planner correction.

## Change And Evidence

The actual planner now walks dependency projects without the requested target until reaching the nearest project with that target, retains ordering edges among explicitly selected tasks under excludeDependencies, and ignores direct self edges. Per-target traversal tracks visited project identities; adjacency and wildcard target matches are indexed once per plan. Cross-task dependency cycles still fail before executable actions are published. No new runtime dependency or authority bypass was added.

The strict language-neutral corpus contains eight cases: targetless bridge/diamond, nearest target stop, retained selected ordering, excluded unselected task, direct self edge, targetless project cycle, and retained selected dependency-project ordering. Each expected graph is computed independently by installed Nx and compared with the actual imported Bazel planner. The corpus is validated with independent Ajv.

RED corrected-corpus run returned 1: 3 pass, 5 fail, 23 assertions, 532 ms; installed Nx accepted all expected graphs, while the original actual Bazel planner failed the five applicable cases. Initial direct Nx import correctly refused the repository's protected fixed namespace; isolated-oracle runs use NX_WORKSPACE_ROOT_PATH pointing only to this ticket's Generated fixture. No protected namespace environment or state was fabricated.

Final full-file filtered regression returned 0: 11 pass, 0 fail, 46 filtered, 76 assertions, 2.20 seconds. This includes all eight new laws and existing portable graph, forwarded arguments/configuration, cross-task cycle, and explicit selection/project-exclusion laws. All new tests emit DEBUG observations after comparing actual planner output. Final strict TypeScript with root strict options and two roots returned 0 with zero diagnostics; an earlier strict RED found and corrected the fixture's unknown Object.entries inference.

The source-only oracle does not execute a Bazel action, SDK publication, native transition or project command. It does not qualify the entire public Nx/Bazel runtime matrix or physical Windows/Linux. Prior backend closure digests are historical because the planner Source changed; file-count bounds are unchanged. The suite is imported by the existing public Bazel source/parity dispatcher, so existing launch commands cover these tests.

## Oracle Scope

Exploratory vectors revealed installed Nx removes some dummy-task edges for a targetless project cycle with an eventual target and for an explicitly selected endpoint through a targetless bridge. Those exploratory vectors were corrected to the independent eight-case supported parity corpus before production edits. No universal graph-parity claim is made, and this work does not reproduce those Nx edge-loss behaviors.

## Files And Current SHA-256

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts` — 76691 bytes; `9177abcab6cd9f15261a90e228cb83a94a0fbf26b5746ab78616d894c73665fe`; absolute path length 113.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🟦️.ts` — 53544 bytes; `950d3f12e7d1a79fa6d83a1e122ca9e89f8fc9a293c17f30e0d42edd05f6817a`; absolute path length 115.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🕸️dependencies/🧬️schema.json` — 1660 bytes; `a4d3e03d83074094185ef4a86fedd91287bebdc998de085ed5834233822bef05`; absolute path length 138.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🕸️dependencies/🔣️.json` — 4101 bytes; `de4e37866e151dd1e928bffb52ee31c401acae2da5356b0b4a9a21315e594907`; absolute path length 132.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🕸️dependencies/🟦️.ts` — 2168 bytes; `600157bdc811d46385162c60711fe5553ee326d1f6a643b7d660cb14e24a9a7f`; absolute path length 130.

## Generated Evidence

- `red.log` — 33147 bytes; `4ee45d54c6219842f8f94b2494f5a817a8ccabe31c14bacaa044c2d852daf759`.
- `red-isolated-oracle.log` — 10907 bytes; `627f583f69152ba57e63fff664caf19a0e1d5d8055de9baa839a20a21c7871a9`.
- `red-corrected-corpus.log` — 11242 bytes; `9cb6d38861c8eaf1ac08e9a832dab48eccb96187826a7284c21402b2ec7d5416`.
- `green.log` — 1810 bytes; `736fbe543cd2cfdd14bf9bb7921740d1fc0ad66075c193a1c97584efb37036b9`.
- `regression.log` — 2117 bytes; `d4c8122c9fc57d7e45687bbf419ef6169d4a40b1327c9d2a7625dde407cf3f2a`.
- `strict.log` — 3304 bytes; `43690f08ee00a832bea99a3da33ebcfcb6414023d70e4736267c29287538d86f`.
- `regression-final.log` — 2113 bytes; `5599ddccb49a702f46817bf4666713248017901d23da3a47d12a311c996b2385`.
- `strict-final.log` — 52 bytes; `ed2bacaf98673945772e8893f1a869a0b91ac8bc363e76c6688901b8694c4932`.

Generated evidence remains only in this agent's ticket subtree for parent review and ticket-finalization cleanup. Ticket inputs and this audit remain. The ticket must remain open while the wider objective is incomplete.

## Genuine Full Source Qualification Before Configuration Work

The exact supported const-fold mode is --backend-tests; unsupported --source-tests correctly refused. The first supported full run returned1:39existing Source laws passed, while all8new independent Nx oracle imports refused the original protected namespace. A first test-only child integration attempt used the wrong existing routing export surface and failed module admission; the canonical package routing surface corrected that import without aliases or compatibility changes.

The independent Nx oracle now runs as an actual same-wire owned child under each test's genuine receiver. Its ordinary temporary model workspace is beneath the test artifact tree, so installed Nx computes the independent graph without opening the repository's protected fixed namespace or mutating global Nx module state in the suite. The child receives the exact Original; no new policy clock is issued. It uses only one additional permanent test helper named 📜️script.ts.

Full genuine --backend-tests then returned0:47pass,9runtime skip,0fail,1472assertions,13.50seconds under the unchanged60second authored Original. The installed-SDK/public runtime parity case was excluded by the existing helper pattern. The actual production backend closure law accepted511files/3646251bytes/esbuildInputs440/digest45d6eb68395bca5a6512ab273f59f1d54c2f43d00b894b8168d4dd6da9f01562. Strict three-root TypeScript returned0/zero diagnostics.

Existing aggregate --prepare PID80796 then returned0, performing two actual captures with exactly the same511files/3646251bytes/digest above. Pair diff identityChangedfalse; added/removed/changed all empty. No captured test paths. Pair receipt: Generated/bazel-source-capture-pair-80796-e607b944-be5b-40a4-923b-42cef58b3653.json. The actual same-wire receiver preserved defaultNx/repository:build/maximumElapsed0/null deadline; aggregate completed13684.363292ms. Readiness and prepare receipts carry PID80796. Source-only; no permanent runtime dispatcher or public package alias executed.

Configuration schema-first work now adds independent Nx vectors for requested configuration fallback and original requested-mode propagation. The above receipts become historical if backend Source is edited. Canonical native pending state remains untouched.

## Configuration Fallback And Inheritance

Four strict language-neutral vectors were added before production changes. The actual owned installed-Nx oracle accepted the first fallback vector while the original Bazel planner chose app:build rather than app:build:prototype; subsequent test receiver admission refusals were downstream of that first actual test failure and are not separate feature evidence. The RED receipt remains configuration-red.log.

The narrow taskIdentity fix chooses a target's declared default if the requested configuration is unavailable. include separately carries the original requested configuration through any dependency fallback, while a root's default becomes the inherited configuration when no caller configuration exists. Explicit invalid configuration identities still refuse. Independent audit reviewed this formula.

Full genuine configuration-green.log returned0:51pass/9runtime skip/0fail/1484assertions/32.95seconds under the unchanged60second Original. All12independent owned Nx planner vectors passed. Its actual production capture law accepted511files/3646345bytes/esbuildInputs440/digest233492e8d3d61d916e5ea19c0f7a146fab2729b2aa9e174817ec494a66c7b3e7. No broader runtime acceptance follows.

The audit lane separately confirmed a per-command forwardAllArgs override defect in Bazel's actual runTask initializer. Parent assigned its correction next; current configuration capture becomes historical if that production Source changes. Full prepare/double-capture is intentionally deferred until those fixes settle.

## Per-Command Literal Forwarding Runtime

The strict language-neutral five-case corpus and Source law were added before the one-line production correction. RED forwarding-red.log reached actual owned commands: Nx literal adapter and independent SQLite expected7literal arguments, while actual extracted production runTask invoked its receiver with0when target forwarding was false and command forwarding true. Downstream receiver admission refusals followed that first test failure; no additional failures are claimed as independent evidence.

Production runTask now assembles declared defaults plus caller argv before applying each command's own flag/inherited target flag. Native owner environment projection still sees the original execution object and is unchanged. The actual initializer is extracted without rewriting its body, then executes actual owned Bun commands against the permanent 📜️script.ts argv fixture. The independent Nx literal adapter also emits and executes real command sources. SQLite independently selects the contract's resolved forwarding flags and literal arrays. Both outputs match the same language-neutral expected JSON arrays across per-command enable, disable, inherit, mixed commands, and omitted flags. Empty strings, Unicode, spaces, dollar syntax and a single quote remain literal.

Full genuine forwarding-green.log returned0:52pass/9runtime skip/0fail/1501assertions/36.00seconds under the unchanged60second authored Original. This includes all12owned installed-Nx planner/configuration vectors and the five real-command forwarding vectors. No Bazel action store, SDK publication, native transition or public package alias executed. Strict five-root and final doublecapture remain pending at this paragraph's writing.

## Remaining Public Option Audit

Read actual selector and adapter plus installed Nx shared-options middleware. Bazel public request bail defaults true because selectBuildTasksV1 uses !head.includes('--bail=false'). Installed Nx defaults nxBail to whether NX_BAIL equals true (normally false). Explicit false maps to --keep_going correctly; the default difference is assigned to the parent neutral-selector owner for review. outputStyle is admitted as stream/stream-without-prefixes/static and returned by the selector, but the Bazel backend never consumes that request field. Every owned Bazel action declares local/no-sandbox/no-remote; skipRemoteCache currently has no extra effect because remote execution is already unavailable. No unsupported output-style implementation is claimed.

## Observable Output Styles And Correct Default Bail

The previously identified outputStyle omission is now corrected. The domain-neutral owned-command output schema declares stream, stream-without-prefixes and static. The existing default keeps raw output. Explicit stream uses StringDecoder and prefixes each actual output line with the owned command label, including split UTF-8 and trailing partial lines. Explicit static buffers only admitted bounded actual output, while the existing raw onLine observer remains live for service readiness. Static delivery uses the original actual issued control; signal, stdin, progress, finite byte/line limits and consumer/publication ordering remain active. It does not replace service execution with separated capture.

The Bazel outer build and continuous service calls project the selected style, original finite native transport limits and original phase control into the existing owned-command ports. Individual action commands and their action identities remain unchanged. The main public selector independently changed bail to default false; this lane changed the backend direct-programmatic fallback so both omitted and false map to --keep_going, while true does not. A Source law extracts the actual rendering initializer, actual build/service call ports and actual bail conditional, then compares the neutral JSON cases with SQLite. No remote cache behavior was fabricated: the existing actions remain explicitly local/no-remote.

The language-neutral output corpus now has eight cases. It is independently validated by Ajv; SQLite produces the same expected raw/prefixed Unicode output. Actual same-wire owned worker subprocesses execute actual commands, preserve separate stdout/stderr, record raw live observations, and distinguish status7, successful output-sink failure and static child-failure precedence. The independent audit found asynchronous EventEmitter sink errors were not yet covered. The new schema-first stream sink-error case reproduced an actual unhandled output-sink error and missing terminal fixture receipt in stream-sink-red.log. Subsequent Original admission-closed failures in that RED run were consequences of the first failed owned scope, not separate parity defects.

Actual stream destinations, admitted streams and rendering transforms now have owned delivery-error listeners. A sink refusal terminates the actual owned child, retains the delivery failure when termination caused a signal, and preserves an actual nonzero child exit as primary. The listeners are removed after settlement. Existing consumer/progress/cancellation primaries retain their precedence. The new real asynchronous sink-error case returns its actual terminal receipt and output-sink outcome in the final GREEN run. No third-party runtime dependency or extra production import path was introduced.

## Literal Program And Authored Templates

The separate materialization audit found a concrete callback defect: a caller literal such as {projectName} or {options.missing} was shell-quoted by the selector, then reinterpreted by the Bazel authored-template interpolator. Shell quoting does not prevent that application-level interpolation. Schema and neutral fixtures were extended first with a distinct nonempty program plus literal args execution form; command/program and commands/program forms are mutually exclusive. Configuration cannot redefine the program identity. Both the repository validator and independent Ajv2020 reject malformed or mixed forms.

The selector synthetic exec target now stores the actual program and literal tail. Nx projection quotes only the literal program and uses its existing literal argument adapter; Bazel quotes the literal program/cwd and appends literal args without passing them through authored-template interpolation. Existing authored command and commands interpolation remains active. The Source regression executes the actual selector, validator, planner, runTask initializer, extracted original interpolate function, and actual Nx projection/literal adapter against permanent owned argv fixtures. Literal project/workspace/options placeholders, a cwd containing {projectName}, empty strings, Unicode, dollar syntax, spaces and quotes survive exactly. A retained authored-template command separately proves authored {projectName}/{options.literal} still interpolate while a separately appended literal {projectName} remains literal. The selector owner's corresponding exec assertion compares program plus args with the independent SQLite tail.

The initial actual exec RED failed with 'Build task requires option missing' while the independently executed Nx projection retained the literal callback tokens. After the narrow production change, the actual exec literal and authored-template laws pass. This is Source-extracted real command/materialization evidence; it does not execute a Bazel action or qualify installed SDK/public task runtime.

## Final Source And Process Checks On Current Production Bytes

The exact existing genuine --backend-tests helper returned0 in stream-sink-green.log:55pass,9runtime skips,0fail,1611assertions,38.07seconds under its unchanged60second authored Original. It includes all12installed-Nx planner/configuration vectors, real per-command forwarding, exec materialization, output rendering and default bail laws. The helper's existing exclusion of installed-SDK parity remained, and the9actual Bazel runtime phases skipped because the integration phase was not requested. No public package alias, protected SDK read, native writer or native transition ran.

A preexisting purpose Source trace assumed its10ms phase would reach the environment hook before expiring. Actual scheduling instead produced genuine 'Original Script process deadline refused' before that hook. The test-only schema now admits a null pre-hook sample or the original fractional sample below1; both branches still require zero command executions. The null branch independently checks the actual issued control is expired. Both genuine canonical expiry messages are accepted, while the original frozen child failure identity and the separate genuine fractional helper branch remain unchanged. No numeric clock, command budget, grant, source-count limit or production deadline check changed. This refinement is Source-trace evidence only.

Strict TypeScript against root strict options, bun/node types and nine owned production/test/worker roots returned0 with zero diagnostics (strict-current.log). The final existing process regression was split into12pass/2filtered/95assertions/8.93seconds (process-output-current.log) and3pass/42assertions/13.58seconds (process-routing-current.log). Together the15actual laws cover descendant termination, exact nonzero/signal outcomes, byte/line bounds, Unicode, ignored stdout, stdout routing, separate observer provenance and genuine10second progress. The earlier complete17law probe had two independent infrastructure failures: the existing schema-validator package facade was omitted by a preexisting source-isolation assertion, and the independent tree-kill package was not installed. Neither was reported as passing or worked around with a runtime dependency.

## Final Double Capture And Independent Current Reread

Existing aggregate --prepare PID6195 returned0 after two genuine full production captures. Both contain511files/3653085bytes, digest d4a58735fe79f50b716025976049ec2ceda33fac1ae7ddc16938d14a61c56687. The512file bound is unchanged. Complete pair receipt is Generated/bazel-source-capture-pair-6195-2b4c1f0f-009b-470c-83ee-b889b8a39a48.json; identityChangedfalse and added/removed/changed arrays empty. No test paths were captured. An independent Python reread of every current captured source verified all511path/byte/SHA records without a mismatch (pair-current-python.json).

The actual same-wire receiver retained defaultNx, policyowner repository:build, maximumElapsedMilliseconds0 and null deadline. Aggregate settlement completed20612.078625ms. These are current production Source/double-capture and receiver facts only. All earlier production digests in this audit are historical. Canonical native tooling publication, full installed-SDK task/runtime parity and physical Windows/Linux qualification remain separate pending gates; Nx remains primary. Parent owns wider ticket finalization and Generated cleanup.

## Final Owned File Snapshot

The selection files also contain the selector owner's concurrent changes. This lane edited only the synthetic exec target and its corresponding exec assertion there; the library projection changed only the literal program form. The following snapshot supersedes earlier source hashes at this audit checkpoint.

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts` — 77253bytes; SHA256`3533b43b1d8867ebe5058180aa5f4352a3dc38c077a73e2557505b479ae423cc`; absolute path length113.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🟦️.ts` — 53708bytes; SHA256`2f7740e460db30d2feccaa97c56ce9e56161673c9f60db98d9c70ce62b7682c7`; absolute path length115.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🔣️.json` — 14529bytes; SHA256`da990fd07c5188f887df8113c7014d4bd4ec7c0323e9afc3cb64acc6253f8665`; absolute path length102.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts` — 52882bytes; SHA256`2d82e4a278b16736a6c66526e43f140fb83780b7aec2706941bd8ba1c36b4a6a`; absolute path length99.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection/🟦️.ts` — 16129bytes; SHA256`7bd65d56ed23a98b1d2b1afda6d12c5518b8a33d1a01fd5b948a9f7d97869e6d`; absolute path length103.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection/🧪️tests/🟦️.ts` — 8499bytes; SHA256`4f06f9c0772dbb9924fba0dd18ca2af6ec7d5bbaea0cb641646f0a4b415c05d8`; absolute path length111.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` — 12858bytes; SHA256`085b7ce505a9d01bbf1671bfe99380f905e1ec9ccc06ef260c59e732576e2b60`; absolute path length84.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts` — 13184bytes; SHA256`4a35d8a018486de8ef00a115dadbfa5f6a8463931c6862689c9a6c938e633940`; absolute path length83.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧬️schema/📬️output.json` — 615bytes; SHA256`537c837d6828ee38e53fab114d9d3e7a53b77154b62c2ef442b378776a88c06f`; absolute path length100.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️purpose/🧬️schema/📤️output.json` — 3436bytes; SHA256`bcd8e3977c57ebe03420d1057a28326f9e1b606137bae8858529423591f353b4`; absolute path length142.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️purpose/🧪️tests/🟦️.ts` — 12600bytes; SHA256`8bd9131313109b5565269f9ee2595060b97d4733b065759cba1b43fbf114bd1c`; absolute path length133.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🕸️dependencies/📜️script.ts` — 1985bytes; SHA256`7964bff3997822cdaff07546d0c13592ae88199127738a96a4a233af5ac95c4a`; absolute path length136.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🕸️dependencies/🔣️.json` — 7559bytes; SHA256`90e0d31ce94fc57966cad78e6f8dd62f3d3f1225b06ee3f1c3e38d49448eb8cb`; absolute path length132.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🕸️dependencies/🟦️.ts` — 2905bytes; SHA256`8a93c1129b846a6821e4880d790003fd8b50f260db8cd0307d9ca069a27ba1af`; absolute path length130.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🕸️dependencies/🧬️schema.json` — 2385bytes; SHA256`99da5c3e56ad7eb2b714227573391953165af6441072da38a34323239083b3c0`; absolute path length138.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📤️arguments/📜️script.ts` — 238bytes; SHA256`366c56b85fa7e972a232b75ccfc1972f6dc2405f7f636c51074babc8f615fe45`; absolute path length133.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📤️arguments/🔣️.json` — 2517bytes; SHA256`c8176a3512f40b8cba0508b6b721fefa87fef588d735bde91a3b6a98fb2441a6`; absolute path length129.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📤️arguments/🟦️.ts` — 13083bytes; SHA256`837a70e043b64ecbab230253f60a8ddadb14bab503cf4abe6db4341a1d3a77b7`; absolute path length127.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📤️arguments/🧬️schema.json` — 2349bytes; SHA256`8a5f4ab417053a4e5cc97707f3b68beb8a25e7bbbbc2b89256c8e214851ab640`; absolute path length135.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧪️tests/📤️style/📜️script.ts` — 2264bytes; SHA256`f14e17a91895dfcd8e8acdd6093fb2bbe6121c5c8b932f2d5bce201db93eb976`; absolute path length105.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧪️tests/📤️style/🔣️.json` — 1542bytes; SHA256`cab0e23698ffe06e47de04ab0d691c3aa23d601398aae5b2c484c10a3fd66647`; absolute path length101.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧪️tests/📤️style/🟦️.ts` — 3045bytes; SHA256`c8181506e0c4768d5509889ef5638ddff37a3c2b3d391c939643e5dcb93c7e27`; absolute path length99.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧪️tests/📤️style/🧬️schema.json` — 2125bytes; SHA256`e16e30cd433c3b4336840f70a9c309c9b194435772a41c308e20d108b39ce3f0`; absolute path length107.
