# Final Neutral Public Interface Audit

Bounded read-only Source audit while Root public workspace 32266 runs. No Source edits, runtime tests, SDK/tool store use, graph writer or cleanup operation was started. Windows and Linux physical execution remain unperformed.

## Existing Coverage and Current Read

The retained `neutral-public-surfaces-current.md` records the earlier actual Source/oracle suite: 12 tests / 96,425 assertions / 5.55 s, including 683 then-current script wrappers, launch/seed, CI, native host, task environments, MCP argv, editor and orchestration. That prior test is not rerun or relabeled as a new current runtime result.

Current structured JSON/Source inspection found 278 package manifest files and 688 script rows; zero direct Nx executable commands. Current launch.json has 12,284 configuration-array entries and the seed has 9,316; their actual command objects have zero direct Nx executable commands. The scanner recognizes direct nx at command start/shell boundaries, bun/bunx/npx/pnpm/yarn Nx commands and direct node_modules Nx binaries. It does not mistake the legitimate `bun repo tools setup nx` selection for a direct Nx executable invocation. Actual project descriptors outside test/fixture/ticket paths had no `nx:` or `@nx/` executor matches.

The read also searched current CI/devcontainer, agent instruction/skill documents, root/framework README files and dashboard Source for literal direct Nx invocations. No genuine new user-facing direct command path was found within this scope. This is bounded structured/lexical Source inspection, not a proof over arbitrary computed shell code.

## Remaining Matches Classified

- Root package dependencies and patchedDependencies retain pinned Nx23.2.0/@nx packages and the owned patch. They are private SDK inputs, not executable public task wrappers.
- Build `📜️script.ts` lines150/156/184–187 calls private `runOwnedNxInvocationV1` after neutral tools/reset/build selection. The actual private bootstrap line595 checks projected owned Nx arguments from the parent's genuine invocation and runs NxScript with that parent. This is the selected backend boundary.
- NX_WORKSPACE_ROOT_PATH in the bootstrap, native graph storage/startup, publication/transaction and use modules binds private SDK namespace/custody. It does not introduce a public command. The verification orchestration removes NX_TASK metadata before an isolated independent actual Nx cache oracle; its class explicitly owns a ticket fixture and tests reuse/restoration/invalidation. No oracle execution was performed here.
- The dashboard Rust entry lines52–53 uses SEMIO_TASK_PROJECT/TARGET and `runOwnedBuildChildInvocationV1`; its compile route uses neutral declared Rust dispatch. Shared Rust routing lines10/17 uses neutral identity and lines29–30 launches the owned neutral child. No runOwnedNx or NX_TASK guard remains in these public routes.
- Dashboard README line322 describes an actual Nx-native regression. Root README line883 describes Nx as the selected default and shows neutral `bun repo` flags/Bazel selection. Devcontainer README describes the locked SDK/cache, while postCreateCommand is exactly `["bun","repo","run","workspace:setup"]`. CI task invocations use neutral bun repo run commands. These are truthful backend descriptions, not bypasses.
- Editor matches such as nx-promotion, next-nx-inputs, test-nx-native-child-issuance and old ticket fixture names are named backend-specific test/oracle targets routed through bun repo or authored ticket test scripts. The two EN/DE Set Up Nx rows use the neutral tools setup route. No direct Nx executable appeared.
- Root AGENTS.md line7 still instructs package manifests to call Nx, but no current package script does so. This is an existing repository instruction, not an executable bypass. AGENTS files were not edited, as explicitly required.
- The clean skill lists nx among process-name detectors; it is not an Nx build launcher. No clean command was run.

No concrete new public-interface defect requiring a Source patch was identified. Current public default-backend Bazel aggregate, current ALL/plan/Stdio and cross-platform physical claims remain gated on actual Root-authorized execution.

## Current Read Fingerprints

- `package.json`: `1733c1d2780613d03620fd947e04177efc77d9c01036326767bd32c09ccb6018`
- `.vscode/launch.json`: `bca3c2740615841b232f8ecfd54d63b8315e0057240ca50eb302d4d1f7764e17`
- `.vscode/🧩️launch.seed.jsonc`: `23be6c371f7d311de23461e1e93521150d3a696b4611cb74b9e32149d325ed31`
- `README.md`: `3afcbd2ca4db03e67ea7028eec5439a930d8c65c861b1cdd407aee9e3aedf089`
- `.devcontainer/devcontainer.json`: `775d0049adf4b386408cc10e35c095d4d0de64e20a5c4b89a7f0f4fdacc24fa9`
- `AGENTS.md`: `1e063f58902f857b1c2e801813862484fd0a1aaa4ad58721fcd5ac663e2a61bc`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/README.md`: `30bddc72d569d901aef4b8bcc4842db43b24abff4c1a6c4c66ec724530934eda`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📜️script.ts`: `f2c77300fcf55544892d9ea7d2ec8ee9db3d4d971a4f43a143693fdc64e78d7a`
