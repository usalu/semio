# Container Entry Points

## Build Context

The development image currently contains only environment provisioning instructions and no COPY/ADD or source-consuming build mount. Its Compose configuration nevertheless used the complete monorepo as its filesystem context. That context is now the .devcontainer directory, with Dockerfile resolved locally, and its existing .dockerignore denylist was replaced with ** to exclude every ordinary context file. The former ignore file was below the previous root context and therefore was not its root ignore policy. Docker still receives the Dockerfile and ignore control file through its standard build protocol. Source and user-data mounts remain separate from image build input.

RED: the language-neutral fixture rejected the former parent-directory context. GREEN: the YAML/minimatch fixture passed after the configuration change. Native docker compose config --format json also exited 0 and resolved the build context to /Users/ueli/Documents/semio/.devcontainer and dockerfile to Dockerfile. No Docker daemon was used by that configuration check. No image build or runtime was qualified; the Docker daemon remains unavailable.

The permanent fixture rejects new COPY/ADD or mount instructions until an explicit context contract accompanies them. The check is registered in repo:test after the preceding complete CI suite; its focused execution passed.

[Docker’s build-context documentation](https://docs.docker.com/build/concepts/context/) defines recursive filesystem contexts and Dockerfile/ignore handling.

## Remaining Lifecycle Work

Post-create still performs a direct application install, Go build and broad workspace setup. Post-attach still installs tools and rebuilds/installs the extension using timestamp checks. Post-start still changes user/config state and includes destructive Git-stash/database cleanup. None of those hooks was executed or changed during this context-only correction. They require an owned lifecycle refactor, minimal runtime acquisition, explicit Nx environment/extension leaves, cheap attachment and preservation of user state. The global Nx feature is still present.

No BuildKit builder or garbage-collection policy has yet been configured, and no container volume was modified or removed.

## Consolidated Validation at 23:53 UTC

The complete repo:test suite containing CI, container-context and isolated WASM-fingerprint contracts exited 0 in 2m30s (2m28s test task). Native repo:toolchain exited 0 in 891 ms and emitted valid installed-tool/pinned-Binaryen identity JSON. This remains local macOS arm64 validation.

## Pinned Runtime and Nx Creation Hook

The image now provisions Bun 1.3.14 and Node 24.15.0 directly from official Linux x64/arm64 archives. Each archive has a literal SHA-256 pin verified before extraction; downloads are bounded, only the runtime binaries are installed, and their versions are checked during image build. The global Nx feature was removed. Container creation is the argument-array invocation `bun nx run workspace:deps-javascript`; its existing uncached leaf synchronizes the frozen lockfile through the independently acquired repository Nx toolset. The former post-create shell script was deleted, removing its direct install, Go client build and broad workspace setup. Other dependency environments remain selectable Nx targets. No shared application install was executed during this change.

RED: the focused runtime test rejected the global Nx feature, and the creation-hook extension subsequently rejected the shell hook. GREEN: JSONC/lodash contract checks and actual POSIX shell selection passed for both supported architectures and rejected four invalid selectors. All four official archives were downloaded in ticket-generated storage, matched both Node crypto and native shasum digests, and contained the expected executable members according to native unzip/tar. The byte counts were Bun x64 35,595,658; Bun arm64 35,700,603; Node x64 31,164,460; Node arm64 30,108,656. Native Compose config exited 0 after the changes. No Linux runtime or Docker image was executed because the Docker daemon is unavailable.

Changed: `.devcontainer/Dockerfile`, `.devcontainer/devcontainer.json`, `.devcontainer/README.md`; removed `.devcontainer/post-create.sh`; added container `🧪️tests/🚀️runtime-bootstrap/{🔣️.json,🟦️.ts}` and registered it in the caching contract suite; added ticket probes under `🔬️container-context/🚀️runtime`. The complete suite is active. Post-start/post-attach service ownership and installation behavior still require refactoring.

Sources: [Bun 1.3.14 runtime image source](https://raw.githubusercontent.com/oven-sh/bun/bun-v1.3.14/dockerhub/debian/Dockerfile), [Bun official checksums](https://github.com/oven-sh/bun/releases/download/bun-v1.3.14/SHASUMS256.txt), [Node official checksums](https://nodejs.org/dist/v24.15.0/SHASUMS256.txt).

The optional Docker Hub manifest probe produced no output for nine minutes. Its exact command/PID pair (64352 and child 64367) was revalidated with native ps and sent SIGTERM. Subsequent inspection showed both processes still alive, so this was not a successful shutdown. It supplied no digest or validation evidence; the implementation instead uses the official checksum-verified release archives above. No other process was signalled.

The manifest probe later required SIGKILL after revalidating those exact PIDs/commands. Its native descendant tree identified credential helper PID 64372. The probe exited 137; its owned credential helper had exited by the subsequent check, so no signal was sent to that helper. No daemon or unrelated process was signalled.

The complete suite containing the image-runtime and creation-hook contract passed in 3m49s after the independent command-import and artifact-router regressions were corrected. The image itself, Linux execution and mounted-volume permissions remain unqualified; post-start/post-attach ownership and behavior are still unfinished.

The [official Dev Container schema](https://raw.githubusercontent.com/devcontainers/spec/main/schemas/devContainer.base.schema.json) confirms that the creation-hook string array executes one command without a shell. Native container/image and mounted-volume execution remains pending.

## Preserve Persisted State on Restart

The existing post-start script cleared Git stashes, deleted the Neo4j store when its graph name differed, then wiped and replayed the live graph and ran a direct legacy-pruning command. Those restart-time mutations and their unused helper functions were removed. A named workspace neo4j-data volume now owns the live store across container recreation; Cypher exports remain explicit artifacts. README statements about automatic deletion/replay were corrected.

RED: the persistent-state fixture found six prohibited restart operations. Its first attempt could not start the Python Nx plugin worker within the native timeout; the normal retry reached the expected assertion without resetting the daemon. GREEN: the source contract matched the lodash oracle, Bun/jsonc-parser agreed on the retained-data mount, and native Bash syntax validation exited 0. No startup hook, Neo4j service or Docker container was executed, and no Git/database state was modified. The fixture is registered for the next full suite. Full conversion of the remaining startup/attach shell behavior to Nx still remains open.

## Extension Attach Uses the Nx Artifact Owner

The attach block now invokes repo-vscode:build-vsix exactly once on every enabled editor attach. Its existing dependsOn build edge owns compilation. The four-file timestamp shortcut and separate build invocation were removed. Installation runs only after the packaging target succeeds, so a failed build cannot install a stale existing archive. Documentation now describes the actual Nx/CLI behavior.

RED: native Bash execution of the original attach block skipped Nx for the current-archive fixture. GREEN: native Bash with isolated Nx/editor doubles passed current, stale and missing archive cases, plus build failure with no installation. The first green attempt failed before execution because the Python Nx plugin worker did not connect within 30 seconds; the normal retry passed without daemon reset. This proves attach control flow, not installation into a real editor. The actual VSIX build is tracked separately in the build-contract report.
