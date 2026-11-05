# Actual Linux Release Execution Environment

## Initial Read-Only Inventory

The actual host is native Windows. `wsl.exe` is present on PATH and at the Windows System32 location. This establishes the Windows launcher only, not a working WSL kernel or installed distribution. The current tool registry has no WSL, PowerShell, SSH or devcontainer CLI tool entry, so no arbitrary direct subprocess was used to infer their readiness. A supported existing system/remote execution interface must be established before querying a WSL distribution or starting Linux work.

The registered read-only Docker command `repo tools exec --light docker -- --version` refused `tool-missing`. The normal Docker Desktop CLI path is absent, and neither the user's Docker configuration file nor WSL configuration file exists. No daemon/context credential values were read. No Docker engine, Linux container, or usable Docker context is currently established by this evidence.

The checked-in devcontainer configuration exists, but its actual base is Ubuntu 24.04, while the course release runbook explicitly requires a dedicated Debian 12 x64 runner. Its Docker-in-Docker feature and named native caches are declared; the current Windows host has no Docker CLI to materialize that stack. Automatic devcontainer lifecycle setup also includes Git safe-directory/signing preparation, so it must not be launched wholesale under the user's prohibition on Git mutations. No editor or IDE was started.

The native remote-execution implementation exists under `library/🏗️build/🌐️remote-execution`, with registered command operations `serve`, `status`, and `workers`. It supports explicit worker platform properties, sandbox custody, content digests and cancellation. The checked environment contains no configured remote executor/worker, Docker host/context, devcontainer or course-runner variable. This is a presence-only observation; no token, credential, endpoint secret or configuration value was printed. A declared implementation is not evidence of an actual Linux worker.

## Source-Declared Release Route

The existing aggregate is `repo run @teaching/architecture-course:deploy-check --configuration=release`. The image producer requires the actual process to be Linux x64. The current manually dispatched workflow selects a Debian 12 x64 runner through the repository variable `ARCHITECTURE_COURSE_RUNNER`; its baseline includes Docker Engine 25+, Compose 2.24+, generic preflight and owned compiler/dependency capacity. A Windows CLI cannot satisfy the Linux guard by supplying a flag or platform string.

The workflow checks out a committed Git ref. Dispatching it now would not validate this shared checkout's uncommitted source repairs. No workflow dispatch, registry push, Pages deployment, production service action or billable service was performed. Existing local publish/image-check/stack/bundle gates prepare and validate artifacts; their names must not be confused with public delivery authorization.

## Next Narrow Step

Inspect the existing repository OS/native interface and remote/devcontainer bridge contracts for a supported bounded WSL inventory or current-source snapshot route. If absent, add only the necessary schema-first tool-independent local bridge and test it before use. Provision reversible local prerequisites only through their owned setup route. OS feature consent/elevation and reboot are not bypassed or automated. Keep the current single JavaScript dependency writer intact and do not introduce a second setup/dependency writer.

## Actual WSL Terminal Facts

The retained `linux-inventory.mjs` input uses the existing first-party owned capture/process-tree cancellation interface and native executable file probe. It privately resolves the Windows system launcher and allows only the two authored read-only argument vectors. It bounds the complete operation to twenty seconds and output to 32768 bytes/128 lines; cancellation joins the owned child. The task itself still enters through `repo tools exec --light bun`.

An initial retained PowerShell input was refused because local script execution is disabled. No execution-policy override or policy change was applied. The native executable query then avoided that script requirement entirely and made no OS policy change.

The actual `--list --quiet` query ended with status **1**; `--status` ended with status **50**. Both reported that Windows Subsystem for Linux is **not installed**, pointing to the system's `wsl.exe --install` route. Thus the available System32 launcher is an installation stub, not an installed Linux environment. No distribution was launched, installed, registered, modified or removed. No consent dialog or reboot was initiated. The bounded raw facts are generated at `🗑️generated/linux-environment.json`; the retained inputs are `linux-inventory.mjs` and `linux-inventory.ps1`.

There is presently no proven local Linux release execution route: WSL is uninstalled and Docker is missing. The existing Ubuntu devcontainer declaration alone cannot satisfy the documented dedicated Debian 12 x64 baseline. A future authorized environment must receive this current uncommitted checkout through an existing owned snapshot/mount/input path; a workflow at an older Git ref is insufficient. Remote publication remains outside this local readiness inventory.

## Existing Selective and Remote Contracts

Independent source review confirms that full devcontainer postCreate calls native Bash setup, which calls `workspace:setup`; its dependencies include `setup-git`, local Git configuration writes and microcommit hook installation. Therefore the full lifecycle is outside the allowed scope. The existing selective native Bash `rust` route exits before full setup and only prepares the pinned Rust/compiler link. A Git-safe Linux preparation would combine that selective route with explicit owned tool/dependency/course targets and the documented system prerequisites. It would not invoke `workspace:setup`, nor assume that the selective Rust route installs Docker, pkg-config or Chromium system libraries.

Checked the actual source-declared remote environment names rather than guessed prefixes: execution endpoint, platform, instance, properties, jobs, scheduler and worker token are all absent; remote cache endpoint and related selection variables are also absent. Only presence booleans were observed. Repository-side Actions variable/runner configuration has not been queried, and no claim is made that a remote runner is absent. The declared remote scheduler/worker interface alone does not establish a reachable actual Linux worker or a current-source deployment-task bridge.
