# Linux Cold Setup: Next Owned Execution

Read-only plan from current helper inputs and Source, 2026-10-29. No container/native/toolchain/probe/test/lease/process action was executed. Existing canonical and semantic gate owners must settle before launching another resource-heavy operation.

## Current Helper and Source Proof Limits

The retained rootless container is semio-linux-readiness-source. `run-container-command.cjs` runs host repo tools exec docker (normal funded admission), selects /usr/bin/podman through SEMIO_TOOL_DOCKER, and execs the existing container at /workspaces/semio with explicit language/workspace/ticket artifact environment. It writes both output channels to ticket 🗑️generated/<name>.log. **Current implementation buffers progress into the file, not live terminal streaming.** The outer registered light Node is only a logging wrapper; never add light to inner setup/build/test or compiler tools.

The wrapper reports status/error but only sets process.exitCode when status is truthy. Therefore null spawn status/error must be rejected from its printed result even if the outer wrapper exits0. Acceptance is a real inner terminal status0 with no spawn error and a complete log, not wrapper exit alone.

capture-container-source.cjs selects every present nonignored Go file and Go module/workspace manifest, plus changed/untracked files with a finite extension filter. It excludes ticket payloads and records path/size/mode/SHA256, checking dev/ino/size/times/mode before/after each capture. **It does not select unchanged non-Go inputs or .rs files, even changed ones**, and is not a full setup Source closure snapshot. It overwrites container-current-source.json. Preserve a build-time frozen manifest separately before another capture.

verify-container-source.cjs checks only rows already in the supplied manifest for regular file/size/mode/hash and reports installed frontend ELF header. It does not recensus missing/new members, compare identity again after hashing, prove dependency endpoint custody, or certify all monorepo Source. For the new browser policy, confirm its actual Source/helper/schema/fixture, native branch, root target and lock inputs are selected. For whole setup, retain normal canonical graph Source admission and owner freeze; a selected metadata match alone is insufficient.

An existing private container with retained cache/dependency generations can establish **current-Source revalidation and warm setup**, not fresh cold clone/cache installation. Do not relabel it cold merely because the frontend was rebuilt. Genuine cold requires an owner-created fresh private Source view with new empty isolated stores and actual configured creation hooks; do not erase protected existing stores to simulate it. Native Linux host, rootless container and full devcontainer lifecycle are separate acceptance contexts.

## Exact Owner Commands

Use ticket-relative variable P below only as notation (substitute its literal path when invoking tools):
`P=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/LINUX-READINESS`.

After explicit Source freeze and existing owners settle:

~~~sh
SEMIO_LANGUAGE=en /home/deck/.local/bin/repo tools exec --light node -- $P/capture-container-source.cjs
SEMIO_LANGUAGE=en /home/deck/.local/bin/repo tools exec --light node -- $P/run-container-command.cjs setup-source-before /home/vscode/.local/bin/repo tools exec --light node -- $P/verify-container-source.cjs $P/🗑️generated/container-current-source.json
SEMIO_LANGUAGE=en /home/deck/.local/bin/repo tools exec --light node -- $P/run-container-command.cjs setup-current /home/vscode/.local/bin/repo run workspace:setup
SEMIO_LANGUAGE=en /home/deck/.local/bin/repo tools exec --light node -- $P/run-container-command.cjs setup-source-after /home/vscode/.local/bin/repo tools exec --light node -- $P/verify-container-source.cjs $P/🗑️generated/container-current-source.json
~~~

**Host verifier caveat:** current verify-container-source.cjs hardcodes /home/vscode/.local/bin/repo, so it is not included as a host command above. For native host metadata recheck use the already owned host-side census/check procedure rather than running this container-specific verifier unchanged. The container before/after commands are exact valid wrapper shape. This is a concrete helper portability limitation, not a request to edit it in this read-only task.

Do not overwrite frozen metadata between before/after checks. Require both views match the same frozen rows; recapture after owner changes and restart provenance only when the owner deliberately begins a new freeze. All command output stays under generated; helper inputs and reports remain at ticket root.

## Actual Setup Closure

workspace:setup depends on:
deps-build-tools; deps-js-all; deps-python; deps-cargo; deps-go; deps-dotnet; deps-cpp; deps-browsers; deps-wasm; deps-tools; setup-git; prepare; repo-mcp:install; @semio-tech/framework-os-mcp-rs:build.

This remains the required full target rather than only testing setup's leaf. prepare builds/generates framework schema, UI tokens, assets, framework graph and plugin registry. Python adds UI styling/energy oracle project deps; .NET adds repo-test-dotnet deps; C++ adds cpp-setup; WASM adds Binaryen and trunk prerequisites. Setup mutates package/tool stores and repo-local git settings; the current existing instruction authorizes owner execution, not this audit.

New browsers branch requires online authority on every platform and awaits noninteractive Linux system libraries before Chromium download. A previous Chromium binary or old native Bash test cannot certify the new canonical setup route; require the actual deps-browsers node's terminal success in setup log.

## Remaining Language and Toolchain Acceptance

| Family | Setup's current required work | Additional actual acceptance |
|---|---|---|
| JS/TS | registered build tools plus all locked JS environments and generated Source | workspace:verify-typescript and workspace:build/test; engine availability alone is insufficient |
| Rust | nightly-2026-07-20, rust-src/llvm-tools-preview, Cargo dependency preparation for discovered workspaces | normal native builds/tests; reviewed WGPU build-script contract is already fixed, historical93180 no longer justifies a Source patch |
| Go | pinned Go environment and go.work synchronization; repo-mcp install depends on its build | actual installed ELF/receipt/current search/MCP/cancellation gates after native owner freeze |
| Python | pinned uv environment/interpreter series and project synchronization | actual Python/oracle tests under canonical workspace target closure |
| .NET | global.json exact SDK8.0.421 with rollForward disable, ICU support, test deps | actual .NET tests; additional image SDKs9/10 do not satisfy exact8 pin by themselves |
| C++ | deps-cpp only declares cpp-setup complete | **workspace:cpp-test-toolchain and workspace:cpp-test** remain required: latter → cpp-build → cpp-configure → cpp-setup. A prior tiny C system-library smoke does not replace this CMake/Ninja configure/build/CTest closure |
| WASM | deps-wasm→deps-wasm-opt + deps-trunk; pinned wasm-pack/trunk/wasm-bindgen; target checks wasm32-wasip2 and wasm32-unknown-unknown | actual authored WASM/product builds/browser activation; successful rustup target installation is not compilation proof |
| Browsers/graphics | new system policy + pinned Chromium | all three owned test-e2e targets with actual server readiness, product maps/3D and cancellation; package presence/ICD enumeration alone insufficient |

For additional owned container commands use the same wrapper shape with unique log names and suffixes `run workspace:cpp-test-toolchain`, `run workspace:cpp-test`, `run workspace:verify-typescript`, then root build/test/contract and three browser gates as scheduled by their owners. Do not start duplicate existing contract/browser gates.

## Cold/Warm Completion Criteria

Record initial cache/dependency state without resetting it. One full setup terminal0 establishes only that observed state. Repeat the same full target with unchanged Source and retained private stores for warm idempotency; require no error suppression, native frontend usable, correct dependency endpoint ownership and no unrelated Source drift. A separate fresh-image/create execution must run actual onCreate ownership, Bash postCreate bootstrap and every start/attach hook; current retained container exec does not exercise those hooks. Repeat build/test/typecheck/contract/C++/WASM/browser acceptance in the actual devcontainer after native Linux acceptance, preserving explicit platform skips. No such success is claimed in this plan.

Root corrected the retained container wrapper null/spawn-error exit normalization: only actual status0 succeeds; null/error/signaled outcomes exit1. Log names now use a closed slug grammar and generated log root is created explicitly. No container/install command was run for this correction. The existing Source capture remains explicitly partial and is not a cold all-language Source proof.

The retained partial Source capture input now routes every Git inventory command through repo tools exec --light git, normalizes spawn failure, and includes changed Rust/C#/C/C++/Objective-C/Swift/Kotlin/Java/Python/SQL/GraphQL/shader sources in its selected extension census. It remains explicitly partial because unchanged non-Go Source/assets are not a complete setup closure proof; this correction does not justify a cold all-language claim. No capture/container action executed.
