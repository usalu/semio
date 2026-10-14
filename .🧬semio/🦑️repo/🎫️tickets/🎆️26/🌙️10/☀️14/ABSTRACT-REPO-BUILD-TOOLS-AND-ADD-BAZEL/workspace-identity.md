# Portable Workspace Identity

## Implemented Contract

A workspace has regular, colocated `📋️project.json` and `package.json` files, and its package explicitly declares `workspaces` as an array or object. A nested ordinary package/project does not become the repository root. Runtime discovery and persistence guards no longer depend on `nx.json`. Malformed paired package JSON refuses discovery instead of silently climbing to a different owner.

The shared Rust filesystem admission function lives in `🧰️framework/🔨️modules/🏃️process/🧭️routing/🏠️workspace/🦀️.rs`. Its caller owns JSON decoding, keeping the dependency-free repository host independent of framework serialization and allowing OS runtime consumers to use the existing owned JSON implementation. Python, Go, and C# use their system JSON APIs.

## Updated Surfaces

- OS MCP repository discovery and repository-root workspace binding guard.
- OS run bootstrap discovery.
- OS DSL mutation-source authority root pairing, preserving no-follow checks and strict missing-pair refusal.
- OS store folder event-log root-write guard and its existing regression fixture.
- Rust, Python, C#, and Go repository test-host discovery. Missing workspace roots now refuse instead of returning the invocation working directory.
- Stdio deletion-proof bootstrap copies package identity instead of an Nx marker.
- DSL leaf and aggregate authority fixtures use package workspace declarations and neutral nested workspace anchors.
- Framework process script and project manifest expose `test-workspace`; the integrating agent owns corresponding editor launch entries.

## Validation

Executed on macOS, 2026-10-14:

1. Existing process routing Bun suite: **41 passed**, 206 assertions.
2. New workspace identity suite: **2 passed**, 117 assertions. Twelve schema-validated language-neutral vectors agree across TypeScript, the existing `find-up` third-party oracle, and the actual Rust, Python, Go, and C# test hosts. Rust and Go hosts were compiled from their authored sources; C# linked the authored host. Runtime output confirmed all twelve vectors with a `[DEBUG]` line. Vectors include array/object declarations, nested ordinary projects, nearest nested workspaces, missing marker pairs, absent/null/scalar workspace declarations, Nx-only identity refusal, malformed JSON, and malformed nested JSON that must not resolve an outer workspace.
3. Strict TypeScript check of the new test source: **passed**.
4. Focused DSL native Cargo test through the owned public repository target: the Nx backend failed before Cargo because its dependency graph referenced absent ignored `.lake/packages/proofwidgets/widget/package.json`. An initial fresh graph discovery was cancelled by its owning process after a reusable graph became available. A retry through the Bazel backend is in progress; no DSL assertion result is claimed.

The native fixture workspace paths are padded above absent-root fixtures to exercise the 32-ancestor limit without accidentally discovering the real repository. Each asserted start path is at most 256 characters. Generated compiler outputs and fixture files live beneath this ticket's `🗑️generated/wi-*` and will be removed during ticket cleanup.
