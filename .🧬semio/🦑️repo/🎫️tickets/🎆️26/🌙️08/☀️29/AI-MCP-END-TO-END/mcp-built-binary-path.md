# MCP Cargo binary path resolution

The focused Vitest run reproduced the failure with a valid closed provider configuration that contained no repository-tooling provider. An explicit Cargo target directory still tried to obtain that provider. The call also ignored the requested platform, so the host path API could interpret a synthetic Windows target incorrectly.

The MCP resolver now uses the requested platform’s path API when `CARGO_TARGET_DIR` or `CARGO_BUILD_TARGET_DIR` is provided, in Cargo’s precedence order. Without either variable it asks the repository tooling provider for the configured target directory. The stdio module shares this resolver. The staged MCP artifact still resolves independently from the Cargo compiler directory.

The language-neutral fixture covers a relative Linux target, an absolute macOS target, and relative and absolute Windows targets. It checks both profile directories, the Windows executable suffix, and environment precedence. The focused Vitest suite passed 19 of 19 tests. Python’s standard-library `posixpath` and `ntpath` independently produced all four expected fixture paths.
