# Nx Executor Graph Consistency

## Reproduction

Normal root `bun nx` generation intermittently failed in Nx 21.6.11 run-executor with `readCachedProjectGraph: No cached ProjectGraph is available`. The cache file existed. Installed Nx source shows this reader also returns no graph when persisted errors are present. A Node fork preload and filesystem observer recorded the same canonical graph file alternating between 307 projects with no errors and 222 projects with WorkspaceValidityError. The missing projects include the custom Cargo and emoji-project declarations introduced during this refactor.

The Nx Console 18.101.1 language server (PID 12891 at observation) and seven isolated plugin workers had remained alive for eighteen hours, predating the plugin hot-reload callback. Its bundled implementation computes project graphs independently of the CLI daemon. A bounded diagnostic paused only this language server, executed three ordinary registry generation commands, and resumed it in a finally handler. All three completed with exit 0, 307 projects, and no persisted graph errors. Evidence: `🗑️generated/nx-console-pause-probe.log`. No editor extension host, Cargo process, task result or source was removed.

## Configuration

The repository discovery plugin now explicitly declares its supported emoji-project and Cargo manifest include patterns. This both documents discovery ownership and changes the Nx plugin configuration, whose installed loader invalidates its worker pool when the configuration hash changes. Whether the already-running editor reloads that configuration is being verified. No permanent process pause or kill is part of the solution.

Editor terminal environments, native bootstrap environments and the devcontainer now use the same `.nx/workspace-data` directory as ordinary Nx and the root coordinator. The former terminal-specific override was inconsistent with Nx’s workspace-root-derived daemon socket. These configuration edits require normal new-terminal/container/bootstrap startup to affect inherited environment variables; root invocation already pins the canonical directory.

## Source References and Limits

Nx 21.6.11’s installed socket consumer decodes each buffer independently, which cannot preserve UTF-8 characters split across buffers. The current [upstream socket consumer](https://raw.githubusercontent.com/nrwl/nx/master/packages/nx/src/utils/consume-messages-from-socket.ts) has a different byte-framed protocol. The current [upstream plugin loader](https://raw.githubusercontent.com/nrwl/nx/master/packages/nx/src/project-graph/plugins/get-plugins.ts) also includes additional concurrent-load freshness handling. No upstream package upgrade or patch has been applied; the current root invocation continues to disable isolated plugin workers. An editor UI refresh was attempted, but computer-control permission remained unavailable, so no UI refresh is claimed.

The full Note preparation and the component/font/WGPU restoration gates remain unqualified until the shared graph remains valid during ordinary editor and task activity.

## Worker Reload Result and Toolchain Qualification

The include-pattern change did reload Nx Console’s four configured plugin workers; its three default workers remained active. The registry generation and full `repo:test` contract run passed. The full Note preparation still failed: graph samples alternated between 307 valid projects and 282 projects with MultipleProjectsWithSameNameError and ProcessDependenciesError. Consequently, a worker refresh alone is insufficient. The sample report was reduced to bounded counts and error kinds after an error object included a complete duplicate project graph.

A new language-neutral socket fixture reproduced Unicode corruption under Nx 21.6.11 with one-byte socket chunks (`nx-runtime-red-corrected.log`). Nx 23.2.0, installed in an isolated ticket workspace, passed the same UTF-8 payloads across fragmented and coalesced frames at all six chunk sizes (`nx-runtime-23-framing.log`). Its [released socket implementation](https://raw.githubusercontent.com/nrwl/nx/23.2.0/packages/nx/src/utils/consume-messages-from-socket.ts) keeps complete byte-framed payloads before decoding. The package is the current stable release reported by the npm registry; root dependencies have not yet changed at this point in the evidence.

Nx 23’s isolated graph and source-change callback passed in the combined coordinator fixture, but the first cancellation check found a server in its own detached process group. That process and its ignoring child were explicitly stopped, and no production server was involved. The root coordinator now snapshots owned descendants before signalling so it also terminates detached descendants and waits for their shutdown. The public Nx executable is resolved through package exports rather than an assumed on-disk layout. The callback invokes the installed Nx binary through Bun without embedding an absolute shell path. Both the existing and proposed runtimes are undergoing the complete coordinator regression again.

## Workspace Upgrade Result

Bun installed exact Nx/@nx/devkit/@nx/js 23.2.0 and @nxlv/python 23.0.2 pins and updated the lockfile. Both forced plugin-isolation overrides were removed. The existing library test now verifies that an explicit isolated-worker preference survives the tooling environment. Editor launch commands and their seed were changed from the old physical Nx bin path to `bun nx`; root execution resolves the published package export.

The root repository contracts passed with the new Unicode frame regression (`nx-23-workspace-contracts.log`), followed by another passing run with isolated plugins enabled (`nx-23-isolated-contracts.log`). The full real graph check passed with 307 projects and 1,225 edges. Thirty-one consecutive graph samples retained 307 projects with no persisted errors. Nx Console loaded the new package’s dist modules without an editor restart; the old CLI daemon was replaced through normal Nx startup.

The focused library Unicode suite passed two tests, including an actual isolated-worker describe graph. The isolated daemon regression passed warm cache reuse, native socket interoperability, caller-directory normalization, live source/policy reload, Unicode source watching, and callback cancellation under Nx 23 (`nx-23-daemon.log`). The fifteen cache scenarios passed with byte-identical restored output, executable permission preservation, actual consumer execution, relevant invalidation and failure rejection (`nx-23-cache-verification.log`, `📓️cache-verification.md`). The combined watcher/server regression passed against both the previous and new runtimes after detached descendant handling was added.

The full Note preparation has now started its actual compiler prerequisites rather than failing in executor graph reads. It remains running; no complete preparation or product cache-restoration claim is made yet. Shared-daemon preservation during cancellation is receiving an additional targeted check.

## Shared Daemon Cancellation Boundary

The added regression initially failed: snapshotting every descendant also terminated the shared Nx daemon when the watch CLI had originally started it. The coordinator now excludes the resolved daemon entrypoint and its subtree while retaining detached task descendants. POSIX snapshots use ps; Windows uses Win32_Process through PowerShell and terminates the captured PID set with taskkill rather than recursively including a shared daemon. Microsoft documents [multiple explicit PID arguments](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/taskkill); no Windows runtime claim is made.

Language-neutral POSIX and Windows process-tree cases passed against a Lodash grouping oracle (`nx-owned-processes-green.log`). The real Nx 23 watcher/server fixture then passed source rebuilding, HTTP consumption, server/ignoring-child shutdown, released port, exit 143, and continued liveness of the same shared daemon (`nx-23-shared-daemon-preserved.log`). The fixture’s own final cleanup stops that isolated daemon afterward.

## Nx 23 Restoration and Startup Cancellation

The font tool and packed asset, browser support directories, and variant session restoration probes all completed with exit 0 under Nx 23.2.0. Each deleted only its owned output, restored identical bytes/modes from Nx, and exercised an independent consumer. OpenType parsed all 17 restored fonts; Node imported the Preview2 IO module; Node imported both Note and Studio sessions and verified their independent membership. Evidence: `nx-23-font-restoration.log`, `nx-23-support-restoration.log`, `nx-23-session-restoration.log`; retained details are in `📓️font-artifacts.md`, `📓️browser-support-artifacts.md`, and `📓️playground-sessions.md`.

Early watcher startup cancellation also passed with exit 143 and no surviving launched child (`nx-23-startup-cancellation.log`). Full Note preparation continues through its real WASI component prerequisite; the complete warm graph and product runtime are still unqualified.
