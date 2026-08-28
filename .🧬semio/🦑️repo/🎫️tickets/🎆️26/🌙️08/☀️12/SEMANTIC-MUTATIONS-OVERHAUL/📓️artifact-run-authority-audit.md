# Retained Test-Artifact Authority Audit

## Scope

Read-only review for the metadata-source-provider registered launcher. No Cargo, compose access, production change, or ticket discovery heuristic was used. Environment inspection recorded variable names only, never their values.

## Existing Authorities

`nextestArtifactLocation` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts` is intentionally caller-owned. A non-empty `SEMIO_TEST_ARTIFACT_DIR` resolves against the caller working directory and sets `retain: true`; omission selects the system temporary directory with `retain: false`. It therefore cannot itself choose the active ticket and its fallback is unsuitable for retained proof artifacts.

The root `taxonomyTicketDirectory(repoRoot, ticketId)` validates a supplied `YYYY/MM/DD/SLUG` id and resolves every Unicode ticket segment from the canonical ticket root using actual directory entries and `realpathSync`. It is the correct physical ticket-directory resolver after an authoritative ticket id is supplied. It does not associate a process or Codex thread with a ticket, and its existing CLI callers receive an explicit `--ticket` option; that option must not be reinterpreted as a current-ticket source.

The current command environment exposes `CODEX_THREAD_ID` and `CODEX_SESSION_ID` names, but no `SEMIO_*`, `TICKET_*`, or task-to-ticket variable. Their values were not read. A thread id alone is opaque to repository tooling: no repository-local thread/session-to-ticket mapping was found under `.🧬semio/🦑️repo`, and this execution context exposes no callable repo-ticket MCP operation to resolve one. The cache directories observed are tool caches, not ticket association records.

## Proposed Reuse

Add a root/Nx launcher boundary, not a library fallback:

1. The Codex/repo-ticket host resolves the *current thread's* active ticket through its existing ticket metadata and supplies one canonical ticket id to the root launcher. The host, rather than the target, owns the thread-to-ticket association.
2. The root launcher passes that id to `taxonomyTicketDirectory`; only its resolved canonical directory may become the evidence authority.
3. The launcher derives `SEMIO_TEST_ARTIFACT_DIR` below that directory, for example `🧪️runs/metadata-source-provider/`, and passes it as the child environment of the registered Nx target. The test continues to allocate its unique retained child run below that authority.
4. `nextestArtifactLocation` remains unchanged: it consumes the injected directory as its existing explicit caller-owned authority.

This reuses the proven physical resolver and explicit artifact interface. It adds only the missing host-owned current-ticket association bridge. It must not scan ticket directories, choose the newest/open ticket, infer from the working tree, or use `CODEX_THREAD_ID` as a directory name.

## Fail-Closed Conditions

The launcher must refuse to start the retained target if the host provides no current ticket, more than one ticket, a malformed id, a missing directory segment, a non-directory or symlink boundary, or a resolved directory outside the canonical ticket root. It must reject a caller-provided artifact directory that differs from the derived one, and it must not silently fall back to `tmpdir()` for this retained proof target. A bare Nx invocation without the host bridge consequently errors with an actionable missing-current-ticket diagnostic rather than emitting unretained evidence.

The existing target currently has the final condition: it requires `SEMIO_TEST_ARTIFACT_DIR`, but does not establish it. The successful registered replay supplied that value externally. This audit does not claim a bare launch is zero-touch until the root launcher implements the authoritative bridge and an isolated test covers all acceptance and rejection cases.
