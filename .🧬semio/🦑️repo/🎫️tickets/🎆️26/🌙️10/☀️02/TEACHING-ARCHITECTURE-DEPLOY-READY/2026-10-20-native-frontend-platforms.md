# Existing Native Frontend Platform Publication

## Finding

The existing repo-mcp Go owner can build the unified native frontend on an actual Linux/amd64 owner. Its managed Go port also inherits GOOS/GOARCH/CGO_ENABLED and an explicit REPO_MCP_BIN destination, so the implementation can mechanically cross-compile a separately placed Go ELF without a new library. No inspected declaration, however, provides a Linux cross-publication configuration, validates that cross artifact, installs it remotely, or creates/executes a Linux container owner. Cross-building the frontend alone does not satisfy the course image-build requirement that its running process is Linux/x64. Copying a frontend into the guest is excluded by Root's instructions and is not proposed.

No build, command, compiler, container operation, installation, binary copy or Source edit ran. Only this report was written. Root63416 dry2 remains the sole live command.

## Exact Existing Owners

Native same-platform Go build:

~~~text
repo run repo-mcp:build --configuration=release --skip-cache
~~~

Prototype is the corresponding configuration=prototype. The build manifest has dependsOn [], owned repo:script entry, inputs production/^production/goToolchain and one mode output .🧬semio/🦑️repo/⚡️cache/🗃️bin/{SEMIO_BUILD_MODE}/repo. There is no linux/amd64 configuration or platform suffix. The existing native frontend recovery selector admits repo-mcp:build before normal JavaScript graph/image bootstrap; it preserves the genuine invocation and calls the same runBuildScript owner.

Native same-platform Go install:

~~~text
repo run repo-mcp:install --configuration=release
~~~

It depends on the owned build. Install accepts no forwarded arguments, verifies current-platform file magic, executes source --help under the incoming issued control, rechecks its digest, publishes into the user's native installation, persists managed PATH selection, executes installed repo --help and writes installed.json plus native-repo-path. This is a current-host installation, not a cross-platform deployment route. A Linux ELF cannot pass the macOS install probe. No install is requested now.

Existing Rust alternative:

~~~text
repo run @semio-tech/repo-cli-rs:build-cli --configuration=release
~~~

Its declared rustBuildIntent selects package semio-framework-repo-cli, scope binary, binary repo, with bounded Cargo fallback. The generic intent schema supports an authored target field, but this exact owner declares none. It has outputs [] and does not declare the unified repo-mcp installation destination. Its build router delegates into selected native graph consumption rather than allowing a separate private Cargo cache. Do not treat a passed --target flag or a generic host-platform token as an already-declared Linux frontend publication contract.

## Go Compiler Port And Source Closure

Owner path: 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/{📋️project.json,🟦️.ts,📜️script.ts}. runBuildScript resolves the selected binary, sets GOWORK to the actual root go.work and calls runCanonicalGoBuild(MODULE_DIR,[-o,binary,.]). The primary module declares Go1.25; go.work also declares Go1.25.

The canonical projection owner is repo 📚️library/🧪️testing/🐹️go-plan/🟦️.ts. It constructs virtual Go source overlays from the domain-neutral layout and recursively follows local module replacements. The conservative replacement closure inspected from repo-mcp contains23local Go modules and273Go source files. Every inspected module is Go1.25; their requirements are repository-local, with no registry module requirement found. No import C, #cgo or cgo-only build constraint was found in those273files. This is static module-closure evidence, not a compiled import graph or proof of successful CGO-disabled cross compilation. Standard-library networking/user lookup can choose different implementations when CGO is disabled.

The managed artifact port is 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🐹️go-build/🟦️.ts with its closed GoArtifactBuildPolicy schema. It validates mode/cwd/capture/lease/deadline, applies mode-owned flags (-trimpath and stripped linker output for release), captures compiler output in a private staging directory, holds whole-host compiler capacity, supplies bounded worker count/GOMAXPROCS/GOTMPDIR, and publishes through stageArtifacts after compiler capacity is released. It invokes the existing Go tool behind runOwnedCommand; no raw Go invocation is needed.

The port's environment spreads the incoming environment. Thus GOOS=linux, GOARCH=amd64 and CGO_ENABLED=0 would reach Go unchanged, as would a supported REPO_MCP_BIN override read by resolveMcpBin. These are environment inputs rather than positional build arguments. REPO_MCP_BIN can prevent overwriting the usable host prototype path; never cross-build into the default current-host mode path. The manifest does not declare those environment axes as hashed inputs or a platform-specific output. Any exploration would need skip-cache and a distinct permitted destination to avoid incorrect cache replay/clobber, plus ELF/architecture and Linux execution verification; none has been authorized or performed here. No exact cross-build execution argv is recommended as an already-verified deliverable route.

## Rust And Native Linux Constraints

The Rust frontend package has its native producer-crate closure plus repository domain/workspace/MCP owners and serde_json. rust-toolchain.toml pins nightly-2026-07-20, rust-src/llvm-tools-preview and only wasm32-unknown-unknown plus wasm32-wasip2 additional targets. .cargo/config.toml has Linux GNU LLD flags, but no macOS-to-Linux C compiler/sysroot declaration. LLD flags alone do not provide the Linux target standard library, glibc/sysroot or any native dependency build environment. No cross-Rust executable claim is supported by this inspection.

The existing native bootstrap shell supports actual Darwin/Linux x64/arm64, chooses native platform tool pins and distribution C/C++ prerequisites, then workspace setup. It is host setup, not a container executor or cross-publication selector. The updated build skill permits the owned repo-mcp build ingress as installer recovery only when no usable frontend exists; that exception is not authority to use raw tools for product gates. A legitimate fresh Linux owner can produce its own frontend through that existing owner and then use native repo, but this review establishes no currently running Linux owner or bootstrap transport.

## Relocation And Process Envelope

The unified Go main routes ordinary commands through cli.RunCLI; explicit repo mcp serves MCP. Build verbs use NativeScriptTool. RunNativeScript resolves the repository from current working directory via workspace.FindRepoRoot, selects that root's build script and preserves opaque argv. It does not embed the macOS Source root as its runtime build root. Release -trimpath removes compiler path details from the Go artifact, but runtime relocation still requires the complete intended repository Source at the destination, Bun on PATH and normal platform-native resources.

On Unix the Go port resolves Bun then syscall.Exec(binary,arguments,os.Environ()), retaining PID, streams and caller environment including the original process envelope. Windows runs Bun with inherited streams/environment, forwards interruption and waits for terminal status. Rust NativeScriptTool has the equivalent root-relative authored script and inherited-environment behavior. Relocation of the executable is therefore structurally supported; relocation of a live issued invocation is not. A fresh destination operation must use genuine destination authority and Source/store admission, rather than rewriting a host-root envelope to fit guest paths or replaying an old frozen26 Original.

The missing full-objective piece is execution/provisioning of a legitimate native Linux x64 owner, not frontend language capability. No generic sandbox feature, compatibility layer, guest binary copy, private Cargo cache or raw external command is proposed.
