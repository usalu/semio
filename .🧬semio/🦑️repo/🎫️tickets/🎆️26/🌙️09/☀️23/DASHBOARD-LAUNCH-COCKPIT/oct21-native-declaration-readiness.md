# Native Declaration Readiness

Read-only audit on 2026-10-21. No command, compiler, Source producer, installation/receipt/private-cache read, or cold-state inspection was performed. The sole product-independent authored update is the parent-authorized cold input prerequisite: current root-package lock whole 0 followed by sync whole 0; historical 37397 cannot satisfy the latest package mutation. Lock 24430 remains pending at this observation. All 47 inspected files were reread and their SHA-256 values remained identical through the audit.

## Findings

No disappeared explicit target or literal runtime entry was found. The plan contains 19 repo-run rows: 18 explicit targets and one inferred integration target. The integration project intentionally has name/tags only; current `cargoTargets` creates build/check/test for a real Cargo package, and project assembly merges these before authored targets. Its absence from the local targets object is not a registration gap. Runtime graph publication, admission, compilation, and execution remain authoritative and unrun by this audit.

| Plan Row | Current Reference | Declaration Result |
|---|---|---|
| inferred-native-integration-owner | `@semio-tech/value-retained-clone-integration-rs:test` | Inferred Cargo target; manifest package present |
| canonical-cold-lazy-launch | `@semio-tech/repo-dashboard-rs:test-cold-start` | Explicit target present |
| canonical-build-and-install | `@semio-tech/repo-dashboard-rs:build` | Explicit target present |
| developer-tools-live | `@semio-tech/repo-developer-tools:test` | Explicit target present |
| repo-cli-mcp-root-scope-native | `@semio-tech/repo-cli-rs:test` | Explicit target present |
| os-mcp-native-protocol | `@semio-tech/framework-os-mcp:test` | Explicit target present |
| canonical-os-mcp-stdio-probe | `@semio-tech/framework-os-mcp:test-canonical-stdio` | Explicit target present |
| installed-authority | `@semio-tech/repo-dashboard-rs:test-authority` | Explicit target present |
| installed-execution | `@semio-tech/repo-dashboard-rs:test-execution` | Explicit target present |
| installed-journeys | `@semio-tech/repo-dashboard-rs:test-journeys` | Explicit target present |
| installed-load | `@semio-tech/repo-dashboard-rs:test-load` | Explicit target present |
| installed-smoke | `@semio-tech/repo-dashboard-rs:test-smoke` | Explicit target present |
| cross-platform-tui-window-laws | `@semio-tech/repo-dashboard-rs:test-windows` | Explicit target present |
| process-deadline | `@semio-tech/repo-dashboard-rs:test-process-deadline` | Explicit target present |
| rooted-ticket-service-rust | `@semio-tech/repo-tickets-rs:test` | Explicit target present |
| repo-cli-go-root-scope | `@semio-tech/repo-cli-go:test` | Explicit target present |
| repo-workspace-go-root-scope | `@semio-tech/repo-workspace-go:test` | Explicit target present |
| repo-graphql-go-root-scope | `@semio-tech/repo-graphql-go:test` | Explicit target present |
| repo-tickets-go-root-scope | `@semio-tech/repo-tickets-go:test` | Explicit target present |

## Cold Ordering and Conditional Children

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json#test-cold-start` remains cache false, dependsOn [], outputs [], forwardAllArgs false. Its exact conditional child matches the cold input: dashboard build. Build in turn declares dashboard compile, which remains present. The script routes only `test cold-start` to the exact runtime entry; that entry contains the single required body `canonical cold launch rebuilds real absent or stale installation then warm replay retains publication`. The cold row precedes dashboard build and CLI/OS installer prerequisites in the Native plan. Its natural cold precondition, immutable publication checks, and warm replay remain actual runtime obligations; no present installation state is inferred.

CLI test still has the declared repo-mcp:install prerequisite. The actual repo-mcp project and install/build script registrations exist. OS test still requires framework-os-mcp-rs:build; canonical stdio additionally requires repo-mcp:install. Both child owners/targets are present. Dashboard authority/execution/journeys/load/smoke conditional compile children remain declared. Journeys/load have no eager dependencies. The explicit window route remains a composite Source-plus-Native route, not just the ten libraries.

## Actual Entry and Body Ownership

All seven runtimeSourceEntries referenced by these pending targets resolve to regular source files (including shared authority entries counted separately per target). Nine unique explicit script entry files were read through their target cwd; the four Go scripts call canonical Go testing with the actual invocation. This establishes source wiring only.

Integration Cargo lib mounts four literal Rust law/helper files. Nine #[test] functions are present: four scalar lifecycle laws, one normal-demand law, and four standard-collection laws; all four named original requirements remain present.

CLI test script selects seven native crates, then its actual installed scope runner typechecks and runs the declared SDK entry. That entry and its four literal test leaves yield eight Bun law registrations, matching the plan. Two profiles × three fixture roots yield six protocol conversations. No compiler or installed protocol result is inferred. Other advertised body counts were not independently recounted in this bounded audit.

The registered windows dispatcher still runs three Bun source entries, a separate filtered terminal selection, and the exact ten TUI libraries/native feature selection. Accepted direct TUI 72467 whole 0 /395 bodies proves only the ten-library portion; the registered windows target remains pending. Accepted rooted ticket 91975 whole 0 /8 bodies with 9 filtered is behavior-only; owning repo-run dispatch remains pending. Existing accepted FS53, original15/238 with all69 triples, and other settled gates are unchanged and were not rerun.

## Limits and Next Boundary

The only actual input drift found was the cold prerequisite historical lock attribution; it is now corrected in e2e-cold-first-dispatch-input.json. No production or declaration file was changed. Current lock→sync terminal acceptance, Source admissions, cold runtime, installed protocols, Go laws, and registered composite windows acceptance are still required. No new test or substitute target was invented.

## Exact Read Set

| Path Relative to Workspace | Bytes | SHA-256 |
|---|---:|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📜️script.ts` | 14715 | `4217a15bcf335c890f2d649e2a208450a8b428d0e9b57c76f11d8fe75bff3b22` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/📜️script.ts` | 17768 | `f538d4b94e67583f8869e9b357c635965e4de634f20c902172858dbfa90259a8` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts` | 2945 | `f1da392f8cc2052e6aeb7115504fa7c9099310d7971cc772759663359e56afa6` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📜️script.ts` | 3953 | `c3e1be0856400d4b53d6dc62799aa8d46a7778fd8bad96778a1c2947b6a49d71` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🦀️rust/📜️script.ts` | 1636 | `e7209ab3d37cf982e04fd622773edae81d47d9ed752e7b46ed6e67d4a4e3ec70` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/📜️script.ts` | 2607 | `21991d014aa0bb5c65ba4282eed6d07b280dde03218db729df79527582b7d4c5` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🐹️go/📜️script.ts` | 1563 | `5a792af046079aff6b171059e71dac1e685d7c5eda005abff7755fda1dfdf721` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🐹️go/📜️script.ts` | 1438 | `7a5652674ec933f3a4f5b14b1548542c4ecc851d1fff10c0a0e27b0bf695643f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🐹️go/📜️script.ts` | 1438 | `737a390baa661875f7e482b042fef215cb5051369e0ee21494ccf44eb5ae5b52` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧊️execution/❄️cold-start/🟦️.ts` | 5790 | `ecaf4d5d72413d3be65d8c52d001c7d95b976d8d5f185752c9b439babc56f9ea` |
| `nx.json` | 2735 | `f93010d1574ba0e3299c82bf44aa194eb5ac6eb97fc8c3a85959ff7588df3f76` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` | 9893 | `5534c7b44e7626ec921d26db7bf882dc8ee1c2653225a05f246ad5739e766d95` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📜️script.ts` | 12576 | `a1602694244743641c1e992e0c5e162c16908e5f1316d59b6b159025486b6931` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs` | 44383 | `85f73389453e2b7c9c3e49f6d449e074ccad4d3c4639fdd7090d19dbd4548a29` |
| `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧪️tests/🌐️integration/📦️packages/🦀️rust/Cargo.toml` | 1117 | `55cbd85eb3c87a9fdb85e25b19c3563162585a5e3db03f3422033cdcf53514c9` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📋️project.json` | 10538 | `699f9374888b8aa5b3fb586457298f148cd264c65264a37a45c5da048ffccd43` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts` | 50427 | `4f565f94b5de0b9098d53e831f8671e18659cf003240897cb7d82e5b90e671f9` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs` | 17472 | `fd32f638c8a50b91fe59cce0b9a499535bf64a8e3b7378d8870fc6cc9c69b1ba` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs` | 38001 | `d877ec2614292beaafce64629da43dfc534bcc87b64cb0ef3f7bf7279f07f4ff` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs` | 13175 | `59a4895bcb699cbb5b5862f38385b94b788e3b2d80fadfe5a0446b2b5352a9e5` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts` | 14587 | `814117d237d4e767d04bbaf3767069ec40827b97e0bd04f7a29682331f7bf5cd` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔌️mcp-verb/📁️root-scope/🧪️installed/🟦️.ts` | 15367 | `380d153deefae7ede9dd52850f1fa6a1bee96cefbb6db841c899462a20cdea8d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔌️mcp-verb/📁️root-scope/🧪️installed/🏃️.ts` | 2580 | `6b07e92fd835322160274a902e3106bc3d5803b03a2e177ee52ca264c6e62b73` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts` | 906 | `226d51f737b6f6eed02c2cadac4b6381464a68c19585e50a3d4709682e555f5d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔌️mcp-verb/📁️root-scope/📄️resources/🧪️tests/🟦️.ts` | 1764 | `65af567b2116a17e47f0a5396eaf0ac36c004348244a0a28ebd20468e35d4668` |
| `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧪️tests/🌐️integration/🦀️.rs` | 612 | `f6e0ff93500288c6d02520e171280d7edcde6b0bb867027eb31ac36fad91dc65` |
| `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧮️scalar/🧪️tests/🌐️lifecycle/🦀️.rs` | 16421 | `1f861df8ed724d11f69e14fb37b87e29c6d9ec44e3f74561f8f4f839c7fd7b8f` |
| `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔮️demands/🧪️tests/🦀️.rs` | 7465 | `54f7a82bbdf2248a57c879e4345367957d696d06268e1d359f84a7c7dbdd3e1a` |
| `🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/⏱️work/🦀️.rs` | 2148 | `eb4647344a3421203fbb26c8ec60c78c01ec0e99a19252d3d9378503feb54c0f` |
| `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧺️std-collections/🧪️tests/🌐️integration/🦀️.rs` | 8560 | `8d892c41bca2973bb318e1aef453bcd43de26671c33684c74a764718ac05a038` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔌️mcp-verb/📁️root-scope/🎫️tickets/🧪️tests/🟦️.ts` | 1740 | `5fc69ff311f7f51116157e9f6504540efb8e213fb72b143107e7900dc3f2cf8c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔌️mcp-verb/📁️root-scope/📄️resources/📥️bindings/🧪️tests/🟦️.ts` | 1148 | `e4564305844dbeb69c5861aaf5f5c698747445f5c6abb938337c16d77fff6680` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔌️mcp-verb/📁️root-scope/📄️resources/🧭️routes/🧪️tests/🟦️.ts` | 1014 | `d11b979b1cb9d15e0ed1e59cd6fdd55a5d628924a689ba2ae40abf582526e1af` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔌️mcp-verb/📁️root-scope/🧪️tests/🧫️fixtures/🔣️.json` | 1434 | `65041d02aa12cb651c907cd2dd2df960cb2710e8fd8008c9a0ee194b594492ec` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/e2e-cold-first-dispatch-input.json` | 2315 | `7896e040da60c8214cbf2c301610fdb63eb3bbf4e16fb1484bc013df4c77e27d` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/e2e-native-runtime-plan.json` | 39066 | `2d71d1ffa08c4cfc5086ee6a86bf270c63f8b793116da146dabdf4234c1efb71` |
| `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧪️tests/🌐️integration/📦️packages/🦀️rust/📋️project.json` | 124 | `e12c4c6de0c4fba33d7b5310134084667b0b2e671d3ef98b5f7835fda0a41e9a` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/📋️project.json` | 26016 | `b4c381f44bc41eabe062ea44ce8e5d0363011a1e823994fa2bfe55a90421308a` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📋️project.json` | 6010 | `9bd707aba3c829e2f5bcae430ca6a65aedf74fb167a09885876a20845d71fb14` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🦀️rust/📋️project.json` | 1756 | `789658089aa33d68c7a519b0f37457ee2acb9c05d9a3b13c867d15475a748fab` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/📋️project.json` | 3328 | `cc29e43036cddc4fb057c78569073fefa94bf8ab8ea2824aca37dbd2156a82a5` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🐹️go/📋️project.json` | 948 | `887c7616b49e940a742afc61a85df5312064c711fdfc8ac8884d02195ec3c6f9` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🐹️go/📋️project.json` | 942 | `ec3ad94c796e7671a6b9ecad1c7e3d6b4f482f15434857594a4386fca544998f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🐹️go/📋️project.json` | 1030 | `2a472433084b4e38a4aea48e4d7ff19b9909faee372182a61cfe883a6bb0c23e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json` | 26495 | `1581c1f09d4d5da5653bc716fddfd660538fb7dd8a05a31ed4419db7c560f2c7` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📋️project.json` | 6008 | `5889a40afe533b6b21126f94e9c3e2082726a0dc711ca5ba0f6ea555e55e362e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📋️project.json` | 3340 | `478895db95329d032fc536679a94add28d71317db73b86339a1ce82ae9bb8c87` |

## Follow-Up: Developer Tools Live Target

Parent-authorized bounded read of the already declared developer-tools target, its actual test leaf, fixture/schema/planner, and current process port. No PATH lookup, debugger launch, device/protocol probe, package setup, or runtime command was performed. These six additional files remained hash-stable on reread.

The target reaches TestScript at developer-tools/📜️script.ts:81–82 and Bun test at the declared test module under a 300000ms bound. Production request execution retains DeveloperProcess.run(plan,signal) behind its first-party interface; schema owns four debugger profiles and all platform executable names. The current process implementation does not require options.control for every ordinary child; only its auxiliary/static-output paths add those mandatory checks, so absence of that option in these older debugger test calls is not by itself an established API refusal.

| Actual Live Body | Source Admission/Prerequisite | What a Pass Establishes |
|---|---|---|
| native Git × three fixture rows | Unconditional; git must execute in the actual workspace and HEAD/AGENTS.md must supply authored blame expectations | Real read-only commit/blame/staged-diff commands; child status 0 is required. Current code uses bounded spawnSync, not the DeveloperProcess production port. |
| debug runtime node-inspect | skipIf schema-backed executable discovery has no installed Node profile | Real local inspector attaches/pauses, then cancellation rejects and output retains attach/break evidence. |
| debug runtime python-pdb | skipIf profile executable missing (python3 on macOS/Linux, python on Windows) | Actual Pdb evaluates 42 and preserves the authored literal/Unicode argv. |
| debug runtime gdb native breakpoint and arguments | Non-Windows: both gdb and gcc on PATH, otherwise skipped | Actual compiled C main breakpoint, argc/42 and continuation. |
| debug runtime gdb installed frontend | Windows: gdb on PATH, otherwise skipped | Frontend evaluates 42 only; it does not prove a Windows native target breakpoint. |
| debug runtime lldb native breakpoint and arguments | Entire body skipped on Windows; elsewhere both lldb and clang must be found by Bun.which | Actual compiler + debugger launch, main breakpoint, argc/argv/42 and process exit. Installed executables alone do not establish the protocol result. |

Concrete qualifications for final acceptance: the plan requests Node/Pdb/LLDB live evidence and an explicit GDB skip reason, but these source bodies permit availability skips. A successful suite with skipped debugger bodies cannot satisfy those requested protocol observations. The LLDB live body has no Windows implementation despite the product supporting a managed Windows LLDB profile; Windows must not be represented as covered by this body. The LLDB skip predicate only checks PATH while the production planner can select an existing schema-owned absolute LLDB candidate; this can skip an otherwise selectable product frontend. No current host availability is inferred.

Terminal vectors only inspect generated plans and recorded ports; there is no actual interactive shell body in this leaf. Native setup tests inject finite fixture hosts and do not install packages; they are not evidence of host privilege or actual debugger provisioning. The production setup has explicit privilege/manager/probe refusals. These distinctions preserve the original plan warning that terminal/debug plan vectors are not shell/debugger protocol proof.

| Additional Relative Read Path | Bytes | SHA-256 |
|---|---:|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧪️tests/🟦️.ts` | 23738 | `4732d106d1c0d1bd501560fde073a9448072397838b3f0973c35ea7a9eca01f4` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧫️fixtures/🔣️.json` | 25487 | `ebb68f7fc2f0408760127e47af62d4a6eb0dda5d61f5ce35db7976dff72aff7e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧬️schema/🔣️.json` | 12742 | `22d6b77ca5de9795fc65ae6852de006c169f45667a6912cafb5c7bf7f87e6844` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🟦️.ts` | 11373 | `f513b03bade0fe2eb2e8d5fd0eadb49e365f6a457f3d371e61a1a43aa34b4a8c` |
| `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` | 1981 | `c96b98b763e4c440bfd6d85868734011650d0d0547c647335ed6f2aff35ebcde` |
| `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts` | 16377 | `0455563d375770f72fc07a81ccda7c4d2df8f7543e04f629404aed68f6dc4833` |
