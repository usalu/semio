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
- Jack query source and its current execution/result form one app query workspace consumed by Editor, Runner/menu actions, and Results. Additional windows view that same workspace. The authored source therefore stays in app config and the execution/result stays in app transient; concrete-window caret, camera, and LOD remain independent. This supersedes the initial audit's inference that the source must move solely because the Editor displays it. Artifact/diff contracts expose none of these app/window fields.

## Remaining Workstreams

1. Complete compile-stable typed window transient/config partitions and their close/replay lifecycle, then move the evidenced live window fields out of app config.
2. Finish retained Jack query preparation, cancellation, replay, output publication and interleaved window behavior checks.
3. Run native host routing tests, all affected artifact test compilation, and existing exact-diff generation/application laws; resolve source integration errors.
4. Reclassify the computed Procedural generation previews and Block brush preview currently held in config.
5. Run independent final Terra audits, resolve their findings, refresh exact file ledgers, delete only generated ticket outputs, and close the ticket/goal only after completion.
6. Trace the remaining 36 app-config schemas (270 listed fields) beyond the initial camera families. The independent Terra audit now covers Puzzle camera/LOD partitions, brush results, job lifecycle state, and other named runtime/config candidates. The initial claim that Puzzle2D camera was already window-local did not trace its reconstruction from app config and must not be treated as verified.

## Latest Verified Progress

- Ownership enforcement now passes with zero misplaced declarations. The actual Nx enforcement run completed in 10.4 seconds, checking 10 direct vectors, 4 nested-schema vectors, 3 Rust command-source vectors against strict Ajv, and 102 artifact/diff contracts across Rust, TypeScript, GraphQL, JSON Schema and Protobuf. TypeScript syntax is also checked. This proves the encoded ownership rules; semantic window-state review remains separate.
- Artifact contracts and sparse diffs no longer duplicate explicitly classified app/presence/transient fields. Traced hover, gesture and computed-preview duplicates were also removed from the document contracts. The exact additional 49-owner change ledger is `artifact-contract-changes.md`. Compilation and runtime diff laws are pending.
- The canonical OS config has its own Rust crate; the shell and plugin host consume its types. The host lane reports seven passing shell TypeScript tests, one passing native shell schema law covering 29 definitions, and one passing event replay/cross-shell preference fixture.
- Shared window projections passed five TypeScript cases including split windows, absent utilities, unknown windows and inherited-key isolation. Surface-context native tests passed five cases. Native context-menu wire and concrete-window projection tests passed.
- Browser host-context envelopes passed six neutral schema/wire cases and resolved views passed eight cases. The real worker dispatch with a mocked child passed the cold-pair render test, with logs for live preference refresh, stale opening refusal, exact patch receipts and unchanged artifact config/frontier.
- A later host audit found that native action routing lost concrete-window identity after hit testing. Command, drop and recursive menu routing now stamp the host-selected window while preserving domain arguments. The three neutral JSON Patch parity cases pass; the new native runtime tests remain pending shared compilation.
- Jack query workspace output uses app transient while each concrete editor caret uses its own window transient partition and generation. WindowConfig is being integrated with the matching typed ownership boundary. A keyed map inside one app store would not provide independent publication authority.
- The Jack query runtime is being made incremental across preparation, graph changes, return construction, replay and cancellation. SQLite passed six behavior cases and the neutral cancellation/source-live oracle. New native tests must run once the shared SDK compiles.
- The SDK WindowConfig implementation compiles and now has Rust/TypeScript host pack/load messages. Rewriting adds an actual two-window retained publication/render/reload regression; its initial 37 compiler errors reduced to one missing crate-private body export and are corrected for the third run. Jack's native query run compiled and executed focused cases; final assembled editor validation awaits its explicit binary tool catalog correction.
- Native host compilation reached WGPU and exposed obsolete access to the shared Board engine through the Puzzle plugin. Those 44 references now target the existing OS Infinite owner; old queue/recovery fixtures are updated. A newly introduced DocumentBackbone transport variant still requires its native shell delivery route. The fourth all-artifact check follows seven corrected Puzzle2D test schema imports. None of these pending/failed attempts is counted as a passing runtime gate.
- Generic manifest domain inference and renderer graph-domain action rewriting were removed by the plugin lane; their final independent audit is pending.

# Additional Review Requirements

The initial UI preference adapter folded a typed mutation and replaced an `OsShellConfig` snapshot. Inspection of `OsShellConfig.update` and `setPreference` confirmed that this path has no event log. The host lane must use an OS-owned local event stream with replay and projection, preserving the repository's CQRS/event-sourcing requirement. Its verification must cover reload replay and updates observed by separate mounted shells; a snapshot round trip alone does not establish either behavior.

The shared framework now exposes concrete-window projections in Rust and TypeScript and carries host view context in `ActionMeta`. The utility cleanup must consume that context and retain gesture cancellation, including the renderer-driven intent path, which currently lacks an explicit host view argument.
