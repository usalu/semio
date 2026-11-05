# Native Cargo Ingress Audit

Read-only audit of current sources, 2026-11-04. No compiler run, infrastructure edits, environment alteration or bypass.

## Current Public Route

repo cargo --store=shared --configuration=prototype --parallelism=1 -- test ... is the advertised canonical route, confirmed by repo cargo --help.

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts:49-52 handles command cargo by calling readRepositoryCargoLaunchV1 and launchRepositoryCargoV1 with the actual invocation and an issued cargo:build-entry resource control, then returns. This branch precedes dependencies.ensure, JavaScript lock synchronization and later graph.regrow. Thus direct public cargo currently avoids the general JavaScript dependency/build-graph gate. It still requires original command admission/diagnostics and native storage/toolchain/dependency/source admission. This audit establishes current ordering, not when it changed.

## Authentic Native Admission

The launcher owner is library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts. launchRepositoryCargoV1 checks the invocation/control, opens repository compiler runtime entries and enforcing store, prepares the Cargo scheduler for compile requests, admits native Cargo inputs, captures the Cargo compilation graph and source boundaries, acquires profile/build and capacity leases, supplies the prepared scheduler as RUSTC_WRAPPER, and executes the owned Cargo process. This native Cargo graph is distinct from the general repository JS build graph. Shared store does not mean unguarded execution.

library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts:42 also routes registered runRepositoryCargoCommandV1 callers through the same canonical launcher. Current other-chat native leases are consistent with this route but do not by themselves prove a given invocation can pass every gate.

## Earlier Refusal

.cargo/config.toml intentionally points rustc at library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/📜️script.ts and rustc-wrapper at repo. That ingress file unconditionally throws policy.refusal: Original compiler ingress requires the repository launcher and authentic enforcing store. It is the default unissued compiler guard, not the native launcher.

The earlier message therefore proves a Cargo probe/compiler reached that default refusal path instead of an admitted compiler environment. It does not prove the current direct repo cargo command requires JavaScript graph admission. Exact historical cause cannot be inferred from that text alone. The documented runtime-storage negative test intentionally reproduces it by removing the issued compiler/storage environment; build-guide also documents scratch workspaces inheriting repository Cargo configuration as another trigger.

## Recommendation

Attempt one genuine public repo cargo invocation with explicit shared store, prototype mode and parallelism 1, without changing environment or calling compiler tools directly. Root owns that attempt. This audit has no execution/native success claim.
