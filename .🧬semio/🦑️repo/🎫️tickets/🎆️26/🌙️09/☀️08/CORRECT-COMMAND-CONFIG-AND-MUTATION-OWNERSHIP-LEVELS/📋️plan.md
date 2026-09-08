# Correct Command, Config, and Mutation Ownership Levels

## Objective

Place each command, configuration, mutation, schema, and associated implementation at the narrowest owner that fully describes its semantics. OS preferences belong to the shared OS; reusable surface behavior belongs to a shared surface module; artifact data belongs to its artifact; domain behavior belongs to its plugin or artifact. Consumers reference the owner instead of duplicating contracts or maintaining compatibility APIs.

## Execution

The requested fleet uses GPT 6 Astra with extra-high reasoning for coordination, GPT 5.6 Sol with extra-high reasoning for implementation, and GPT 5.6 Terra with extra-high reasoning for independent read-only audits. Four total concurrent slots are available, including the coordinator. Begin with two Terra auditors and one Sol executor, then rotate completed audits into further Sol execution and return to independent audits for verification.

The repository MCP was reached through its existing local stdio executable. `repo://goals` was read before opening this ticket and the ticket was associated with `AI-OPTIMIZED-REPO`. The MCP model allowlist does not accept the requested model names, so its attribution uses generic `codex`; this does not change the requested agent models. Management integration and remote issue creation were disabled.

1. Audit Trinity/Jack/Rewriting ownership and the wider OS/plugin boundaries.
2. Remove plugin-owned UI locale contracts and their artifact/config storage, and implement the OS-owned setting with schema-first regression coverage and runtime propagation.
3. Refactor additional evidenced ownership violations in bounded parallel workstreams.
4. Add reusable ownership verification through the existing Bun/Nx script and launch infrastructure.
5. Run focused behavior tests and contract checks, perform independent final audits, preserve reports, remove generated ticket outputs, and close the ticket through MCP with the exact touched files.

## Constraints

Preserve concurrent work. No modifying Git commands, worktrees, AGENTS.md edits, legacy APIs, adapters, or migration scripts. Permanent scripts belong in `📜️script.ts`. Shared fixtures must express language-independent behavior and use an existing third-party implementation for parity where relevant. Runtime observations and failures must be reported accurately.

## Completion Criteria

- UI locale is owned once by the OS and affects all mounted surfaces through the shared context.
- Plugin/artifact schemas, commands, mutations, fixtures, and consumers have no obsolete ownership definitions.
- Other concrete violations found by the scoped audits are resolved at their proper level.
- Regression checks prevent recurrence and focused runtime tests verify the changed behavior.
- Reports distinguish completed verification from unrelated environment or concurrent-change failures.

## Execution Decisions

- The initial repository ownership report identified 339 declarations in 28 plugin families: 210 locale fields, 39 locale commands/mutations, 24 terminology fields, 2 terminology commands, 54 active-utility fields, and 10 active-utility commands/mutations. Counts include separate language representations; they are not 339 independent behaviors.
- The existing `semio_framework::ViewModel` is the canonical render-time context. Its immutable reference now passes through the shared renderer/editor/viewer/context-menu seams. No duplicate `RenderContext` type or compatibility method is needed.
- Canonical OS preferences use the existing `os.setLocale` and other OS command identities, with one shared preference mutation/state contract. Plugin cleanup removes ownership of those choices while preserving label declarations and genuine document-language data.
- The ownership policy schema and fixtures are colocated under the repository library's `📏️ownership` module. The root script registers both standalone verification and its normal app-schema gate integration; launch entries expose the checks.
- The isolated ticket Nx workspace runs the actual repository verification implementation and avoids the shared monorepo project-graph lock. Its nine ownership vectors passed against strict Ajv after fixing the schema's missing explicit array types.
- Camera, LOD, and selection cannot be classified by their names alone. Document cameras may express reproducible domain content. Live view state must use a concrete window **instance** identity: the host supports split/spawned windows of the same kind, so fixed per-kind fields would merge otherwise independent users' views. This supersedes the initial Trinity audit's suggestion to use fixed fields for the declared window kinds.
- Host utility selection must retain gesture cancellation and preview cleanup while removing duplicate app config storage. Moving a field without carrying these runtime transitions is insufficient.

## Remaining Workstreams

1. Complete OS preference schema/mutations and host persistence/routing; validate reload, shell isolation, locale propagation, and context-menu propagation.
2. Complete all plugin preference removals and direct signature updates; validate each affected implementation and schema/fixture parity.
3. Remove plugin-owned active-utility state through the shared per-window host context, preserving pointer/gesture behavior.
4. Resolve evidenced Jack/Rewriting config/presence/transient duplication and public schema ownership leakage; extend shared surface/window ownership where necessary.
5. Run final Terra audits, resolve findings, rerun relevant checks, clean only generated ticket outputs, and close the ticket and active goal only after all required work is complete.

## Latest Verified Progress

- The canonical OS config now has a dedicated Rust crate; shell and plugin host consume its types. The owner no longer depends on a shell-owned preference snapshot.
- Shared window projections pass five TypeScript cases including split windows, absent utilities, unknown windows, and inherited-object-key isolation. Panel projections clear window-local context. Serialized Rust views now require explicit locale and terminology; the updated native test remains queued behind the SDK build.
- Browser actor host-context envelopes pass six neutral schema and wire-round-trip cases; the resolved view schema passes eight cases. The real worker dispatch with a mocked child passes the complete cold-pair render test and emits runtime logs for live preference refresh, stale opening refusal, exact patch receipts, and unchanged artifact config/frontier.
- The latest declaration scan reports 46 remaining misplaced utility declarations, assigned to the plugin cleanup lane. This is a report, not a passing enforcement gate.
- Generic manifest icon lookup still knows domain-specific action IDs and example names. `presentation-metadata-ownership.md` records the remaining bounded abstraction fix.
- The shared SDK surface-context native test is still compiling. No native SDK or full application launch success is claimed.
# Additional Review Requirements

The initial UI preference adapter folded a typed mutation and replaced an `OsShellConfig` snapshot. Inspection of `OsShellConfig.update` and `setPreference` confirmed that this path has no event log. The host lane must use an OS-owned local event stream with replay and projection, preserving the repository's CQRS/event-sourcing requirement. Its verification must cover reload replay and updates observed by separate mounted shells; a snapshot round trip alone does not establish either behavior.

The shared framework now exposes concrete-window projections in Rust and TypeScript and carries host view context in `ActionMeta`. The utility cleanup must consume that context and retain gesture cancellation, including the renderer-driven intent path, which currently lacks an explicit host view argument.
