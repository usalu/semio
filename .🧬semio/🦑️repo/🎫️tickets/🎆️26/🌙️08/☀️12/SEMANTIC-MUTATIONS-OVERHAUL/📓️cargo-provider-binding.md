# Cargo Provider Binding

## Design

FND-CARGO-PROVIDER-BINDING-25 binds one selected normal dependency only. Its input is the native workspace base, exact consumer manifest locator, and dependency key. Its future repository-owned output records exact consumer, governing workspace, provider manifest, explicit library source/name/proc-macro state, package identity, key, inheritance/path authority, and compiler extern binding.

The existing provider TOML projection now retains two missing unapproved facts: `package.workspaceLocator` and `workspaceDeclared`. The resolver will inspect only the selected consumer/provider ancestor `Cargo.toml` chains to establish the nearest declared workspace and reject explicit-pointer or nearest-workspace disagreement. It will not enumerate members or dependencies.

## Initial Red Verification

`SEMIO_TEST_ARTIFACT_DIR='<ticket>/🧪️cargo-provider-binding' bun '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts' test -t 'resolves one selected local normal dependency'`

The test is intentionally red before implementation: Bun reports the missing `resolveCargoProviderBinding` export (0 pass, 1 error). No Cargo command was run.

The schema-first neutral vectors cover direct local binding, explicit same-name package override, renamed hyphen override, workspace inheritance, and no inferred library fallback. Their expected binding-name rule is constrained by the ticket's six-case Cargo extern preflight: a missing `package` override binds the explicit library name; an explicitly present override binds the dependency key with hyphens changed to underscores.

## Current Implementation

`resolveCargoProviderBinding` is exported deliberately through the repository TypeScript facade. It reads only the root workspace manifest, selected consumer manifest, selected provider manifest, explicit library source, and bounded ancestor `Cargo.toml` chains needed to establish the nearest declared workspace. It rejects non-local, versioned, target-conditional, ambiguous, unsupported-source, package-identity-mismatched, implicit-library, escaping, excluded, or no-follow-invalid selected evidence.

The projection has gained exact `workspaceDeclared` and `package.workspaceLocator` facts. The projection fixture validates those facts against the existing independent TOML parser.

The selected consumer's own inheritance controls are preserved and validated before workspace authority replaces the edge: `optional`, package override, unknown fields, malformed `features`/`default-features`, and `workspace = false` cannot disappear through inheritance. Every non-parent dependency or library path segment is lstat-checked before a later parent segment can remove it from the lexical result, so a detour cannot hide a missing directory or symlink.

An explicit `package.workspace` locator is a workspace *directory*, never a manifest filename. Both consumer and provider pointers therefore append `Cargo.toml` only after proving the directory; a spelling such as `../Cargo.toml` fails closed. The permanent fixture also retains virtual excluded-root/consumer inputs plus materialized workspace-root and workspace-ancestor symlink cases.

## Current Verification

- Focused binding test passed: 1 group, 64 expectations, 296 filtered, with ticket artifact directory `🧪️cargo-provider-binding/🧪️workspace-root-boundaries`.
- Registered projection and binding run passed: 2 groups, 96 expectations, 295 filtered, with ticket artifact directory `🧪️cargo-provider-binding/🧪️registered-final`.

The root-owned 30-case virtual no-follow probe passed with zero failures (`run-w0bTq2`), including zero-I/O invalid inputs, inherited-control conflicts, detour symlinks, root symlink, foreign drive, workspace-directory pointers, and workspace disagreement checks.

The permanent isolated filesystem trace replays the same boundary class before dynamically importing the resolver: unsafe workspace root, unsafe consumer locator, and empty dependency key record zero `lstat`/read calls; a symlinked workspace root or ancestor records only the exact no-follow `lstat` ancestry and no manifest read. It passed with 11 assertions through `test cargo-provider-binding-trace`.

The focused trace is available through the registered Nx target `@semio-tech/repo-lib:test-cargo-provider-binding-trace` and the matching `🧹clean🧩️taxonomy🧪️cargo-provider-binding-trace` launch configuration.

Both runs used ticket-contained `SEMIO_TEST_ARTIFACT_DIR`; no Cargo command was run.
