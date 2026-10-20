# Native Execution Route For Retained Builder Observation

## Result

Use the existing documented native repo exec interface. The workspace owner is declared by root 📋️project.json with name workspace and root .; exec is the intended generic execution operation of that owner. No new CLI, ContainerTool class, ticket manifest, adapter or compatibility wrapper is needed. Earlier notes treating a missing named target/🎮️commands.json as an execution blocker were too restrictive: the documented exec operation itself owns this invocation.

This review inspected Source and filesystem metadata only. No command, process observation, builder operation, Source edit or test ran. Only this report was written. Root remains sole owner of course22 reconciliation, Source decisions and managed scheduling. No course/native/browser/deploy pass or artifact is claimed.

## Exact Candidate

Working directory: /Users/ueli/Documents/semio

Native executable: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/🗃️bin/prototype/repo/repo

Dry-run argv:

~~~json
["exec","--projects=workspace","--dry-run","--","bun",".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/📊️process/📜️script.ts","inventory","third"]
~~~

Real read-only argv after Root accepts the dry-run and retained identity:

~~~json
["exec","--projects=workspace","--","bun",".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/📊️process/📜️script.ts","inventory","third"]
~~~

The backend defaults to Nx; an explicit --tool=nx before exec selects the reviewed implementation even if SEMIO_BUILD_TOOL differs. The internal bun is the language tool behind the documented native exec interface. There is no raw outer Bun/Nx/Docker invocation in these argv.

Expected dry-run task plan: roots [workspace:repo-exec], one target {project:workspace,target:repo-exec}, no dependencies. Dry-run itself still performs normal dependency/bootstrap, graph and store preparation; it is not a zero-cost metadata route. It does not run the observer.

Local helper identity: SHA256 b75c20edce76e9fbb4bfeb717b09752c9bee709a5ec03bfd2dfc994a282bb61a, 28,372 bytes. Both inventory-third and inventory-fourth receipt paths were absent when read. Recheck absence immediately before real execution because observations are immutable. The helper writes only its bounded new observation receipt in ticket 🗑️generated; read-only here describes builder/Source/process state. It emits [DEBUG] console data.

## Ownership And Envelope Evidence

The current native Rust entrypoint at 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧭️repo-cli/🚪️entrypoint/🦀️.rs recognizes exec and enters the authored build script through NativeScriptTool. Bun is internal to that port. Its Unix implementation execs the script process with unchanged argv and inherited environment/streams; Windows status invocation also inherits those fields.

The build entry 📚️library/🏗️build/📜️script.ts validates exec through the same admitted build caller and graph/store boundaries. In its Nx branch, --dry-run prints buildExecPlanV1 without launching Nx exec. buildExecPlanV1 in 🏗️build/⚙️operations/🟦️.ts:62 emits the neutral workspace:repo-exec task described above.

The portable selector selectBuildTasksV1 in 🏗️build/🧭️selection/🟦️.ts:102 creates one noncached repo-exec command target per selected owner with outputs [], exact forwarded program/args, and that owner's root as cwd. This confirms exec is a first-class operation, not an invented target name.

For Nx, nxOperationArgumentsV1 and nxExecWrapperV1 in 📚️library/⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🟦️.ts add a Bun implementation process that launches the exact declared command with env {...process.env, SEMIO_PROJECT_NAME:process.env.NX_PROJECT_NAME, SEMIO_PROJECT_ROOT:process.env.NX_PROJECT_ROOT_PATH}. This preserves SEMIO_SCRIPT_PROCESS_INVOCATION and caller environment rather than replacing or reconstructing the original envelope. The frozen observer does not consume, forge or replace that envelope. It retains its own reviewed bounded internal tool port, identity guards and neutral DTO.

No intrinsic ownership or envelope refusal was found for this exec candidate. Actual stale dependency generation, physical Source changes, graph input checks, store custody/capacity, missing tools or backend invocation checks can still refuse at runtime; this Source review does not claim admission success.

## Meaning Of The Observation

The retained observer loads the exact reviewed runner identity functions, verifies builder ID/image/created time/ticket label/run token/sole RW source-volume mount, then projects current state through its internal tool port. It never starts, stops, removes, attaches a tracer or changes Source.

processObservationV1 authenticates inspection then explicitly requires an already-running builder before its fixed bounded guest proc reader. If authenticated stopped, it writes accepted:false, failure Retained builder must already be running and exits1; no guest exec occurs. That refusal establishes the authenticated stopped state at that observation point only. It is not a managed course22 terminal receipt. If running, the bounded inventory returns executable basenames, comm, PID/PPID/state/start ticks/CPU/RSS plus race information; no argv/environment/Source is returned. Root must compare current process birth identities and retained course ownership before classifying old course22. Missing handle22832, unchanged logs or a missing terminal receipt alone prove neither live nor terminal.

The fresh-child TransactionProcessObserver in repo 🔩️native/👁️observe is not needed for this route and cannot reacquire arbitrary old process tokens. The repo SandboxProvider kind-only interface and OS MCP semantic document tools are likewise irrelevant to this documented exec route. Do not start OS MCP merely to observe the builder: native-release's source inspection found no host process/container tool, and its startup can stage/build the OS MCP backend.

## Product Commands After Reconciliation

The existing product owner targets remain native repo run <project>:<target> --configuration=release. Retained sequence is nine suites, three product typechecks, explicit architecture browser preparation, Root proctor check barrier, deploy-check and same-tag docker-stack-bundle/export. Their commands are retained in 2026-10-19-continuation.md lines172–178 and build-input.json plannedDeploymentCommands. None should run until Root reconciles course22 and hands off the sole lane.

The old 🚀️run helper command mode still launches inner bun repo. That is a separate future managed-run native-ingress change; it does not block the fixed read-only observation helper through native repo exec. Do not change that frozen runner or Source during current reconciliation.

Root reported prototype native --help actual0 with the unified command interface and release --help terminal1 with repo MCP accepts no command arguments. Use the tested prototype identity, not the old release file. This reviewer only observed file presence and did not execute either frontend.
