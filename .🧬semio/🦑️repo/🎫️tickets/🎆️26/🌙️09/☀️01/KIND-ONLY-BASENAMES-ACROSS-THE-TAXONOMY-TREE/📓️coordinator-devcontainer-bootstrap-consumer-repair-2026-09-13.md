# Devcontainer Bootstrap Consumer Repair

The Go MCP/bootstrap source-contract test now reads the actual devcontainer.json creation declaration. Its former case tried to read the retired post-create.sh and expected an application build/setup command, while the current container creates dependencies through the existing Nx dependency-only route. This change updates that one stale consumer; no bootstrap implementation, lifecycle command or application behavior changed.

## Exact Product Ownership

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🔬️component/🐹️.go` — only the devcontainer case inside TestMcpBootstrapAssetsStayRepoRelative. Other shared Go hunks, including the entity catalog consumer, belong to their respective executor.

The case requires the explicit postCreateCommand argv bun,nx,run,workspace:deps-javascript and forbids the retired hook, workspace:setup and old repo-client paths in that container configuration. Native shell and Windows bootstrap cases retain their separate current setup/build contracts.

## Executed Red And Green

Before the correction, the actual package command `bun ./📜️script.ts test long -run ^TestMcpBootstrapAssetsStayRepoRelative$ -count=1 -v` exited1 after5.352s. Eight subcases passed; the devcontainer subcase failed with ENOENT for .devcontainer/post-create.sh. The selected Go package took0.858s. This was an observed current stale-path failure, not a hypothetical test.

After the correction, actual isolated `bun nx run @semio-tech/repo-client:test-long --skip-nx-cache --args='-run ^TestMcpBootstrapAssetsStayRepoRelative$ -count=1 -v'` exited0. The selected test and all9subcases passed; the selected native Go package took0.455s. Nx successfully completed this target and its one generation prerequisite:15.0s run,14.4s critical path,cache skipped. Total observed wrapper time was87.014s including bootstrap/graph preparation. The log confirms exact forwarding without a literal Go -- separator. Other Go package rows had no selected tests and establish compilation only.

Root also invoked the existing testContainerRuntimeBootstrap control directly. It passed against its existing language-neutral runtime-bootstrap fixture: Bun JSONC agrees with installed jsonc-parser, dependency-only creation has no output/cache/implicit build dependency, pins/checksum-before-extraction match, installed lodash agrees on the platform records, and native sh accepts both amd64/arm64 selectors while rejecting i386,riscv64,empty and injected syntax. It performs no download or installation. The fixture's retiredScripts entry correctly asserts post-create.sh absence; it was never the stale Go reader.

Private Nx workspace/cache/tmp/artifact directories were selected with NX_DAEMON=false and NX_ISOLATE_PLUGINS=false. This does not claim every underlying content-addressed repository compiler cache is ticket-private. Scoped git diff --check passes. No Windows bootstrap, real installer, container startup, network service or user tool was executed. The Go suite's cross-platform source checks are not host runtime certification.

Both owned command sessions completed. Authored report retention precedes deletion of only generated/coordinator/devcontainer-bootstrap-consumer. No new permanent test command or launch entry is needed because the existing registered target owns this existing case.

The exact owned generated/coordinator/devcontainer-bootstrap-consumer scratch directory was removed after retention. No process remains from this control.
