# Devcontainer Lifecycle Owner Pre-Audit

Date: 2026-09-13  
Status: **read-only preaudit complete; two active-consumer gaps require a decision before lifecycle extraction acceptance.**

## Current Entry Selectors

`devcontainer.json` currently selects `.devcontainer/post-start.sh` and `.devcontainer/post-attach.sh`; the README currently selects `.devcontainer/gitkraken-launch.sh` for a user command. Those are configurable command strings, not fixed-name tool contracts. All three bodies require semantic owners and ordinary Bun/Nx/`📜️script.ts` route rebinding; no shell shim or conventional basename exemption is justified.

`post-start.sh` currently combines these distinct concerns:

- persisted-volume ownership and emoji font configuration;
- Neo4j environment, server configuration, start and Bolt readiness;
- Claude auth-storage normalization;
- workspace and submodule Git ownership/safe-directory configuration;
- SSH signing-agent lifecycle; and
- optional Python virtual-environment activation.

`post-attach.sh` combines GitKraken desktop/CLI provisioning, F3D provisioning, editor CLI discovery, GitKraken workspace registration, repo-client configuration, and VSIX build/install/verification. The VSIX block has a precise existing boundary: it delegates exactly once to `@semio-tech/repo-vscode:build-vsix`, then works against fixture editor/package doubles and a short lock. It should remain separate from tool provisioning and workspace registration.

`gitkraken-launch.sh` is a distinct interactive desktop-launch concern: process gating, temporary Xvfb display setup, and GitKraken invocation for one workspace. It should not be absorbed into post-attach’s provision/workspace logic.

## Existing Consumer and Fixture Closure

The source-as-data cache controls currently read the start and attach paths:

- `🔒️persistent-state` reads `post-start.sh`, asserts no destructive persistence commands and the single Neo4j volume, parses current JSONC through Bun and `jsonc-parser`, and can perform only `bash --noprofile --norc -n` as its native mode.
- `🧩️extension-attach` extracts just the marked VSIX block from `post-attach.sh`; its four isolated current/stale/missing/failed cases shadow `bun`, the editor CLI and `flock`, impose a 10-second Bash timeout, and never call a real installer, editor, Nx target or network endpoint.
- `🚀️runtime-bootstrap` reads `devcontainer.json` and Dockerfile. Its `retiredScripts: [".devcontainer/post-create.sh"]` assertion intentionally verifies absence. Root independently ran it successfully: Bun JSONC and `jsonc-parser` agree, lodash comparison holds, and two native architecture positives plus four rejections pass.

The broader `⚡️cache-contracts` aggregate invokes all three controls alongside unrelated tooling/build checks, so it is not a focused lifecycle test. `TestExhaustiveDevcontainerPostAttachGitKrakenWorkspaceBootstrap` is **not safe to run as an isolated fixture**: although it supplies private `WORKSPACE`, `HOME`, tool, and extension-skip values, `post-attach.sh` derives `REPO_ROOT` from its actual source location and its RepoConfigure block can invoke the real canonical repo-client binary with `configure --repo` against the checkout. Future coverage must inject that operation or stage a wholly owned context. Root separately repaired and ran the distinct `TestMcpBootstrapAssetsStayRepoRelative`, which does not execute a hook: 9 subcases passed in 0.455 s through the isolated `repo-client:test-long` Nx route in 15.0 s with cache skipped.

## Findings

1. `SEMIO_GITKRAKEN_AUTO_START` is declared in `.devcontainer/devcontainer.json` and documented in `.devcontainer/README.md`, but no active lifecycle source reads it. It currently cannot control auto-start behavior.
2. The README says a `🐧️gitkraken` VS Code task is available and offers `bash .devcontainer/gitkraken-launch.sh` as an alternative. Current `.vscode` sources contain no GitKraken task or launcher reference. The actual launcher consumer is therefore the manual documented shell command only.

3. Existing full-hook expectations still demand the old `go run ./repo/client/mcp/go/config` route, which is absent from the current attach body. Treat this as a current source-consumer repair, separate from the safe bootstrap-source test.

These are active-consumer closure defects. They need an explicit product decision: either register/consume the declared behavior or remove the stale declaration/documentation. This audit requires route rebinding, not a compatibility facade, and did not invoke the launcher.

## Preserved Input Boundary

The three retired, unreferenced helpers remain as ticket-owned authored inputs under `📋️devcontainer-intake`, verified 3/3 by an Ajv/JSONC/YAML/Bash-syntax closure. They are not active lifecycle alternatives. Their preservation does not establish runtime behavior.

## Limits

No lifecycle hook, installer, network request, IDE, service, container build, Neo4j connection, GitKraken process, or full-hook Go fixture was executed. This audit inspected current sources and fixture mechanisms only. It leaves root’s completed `TestMcpBootstrapAssetsStayRepoRelative` repair untouched.
