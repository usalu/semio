# Writer and Equation Ownership Handoff

## Writer complete checkpoint

Writer's main editor state is owned by each exact `writer-main` window instance:

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config` owns `WriterMainWindowConfig`, camera/settings mutations, and `WriterMainWindowConfigOwner`.
- The adjacent `🫧️transient` facet owns selection, engagement input, lint generation, their mutations, and `WriterMainWindowTransientOwner`.
- Retained commands publish addressed window mutations from trusted `ViewModel`; rendering and measures read `ConfigView.window`/`TransientView.window`.
- `WriterPlayApp` uses explicit `NoConfig`, `NoDraft`, `NoPresence`, and `NoTransient` app lifecycle providers.
- Artifact/snapshot/diff projections contain durable fields only and use the canonical `ArtifactChild` contract.

Validation:

- `writer-window-state-oracle`: GREEN with Ajv and independent `fast-json-patch`; `🗑️generated/writer-window-state-oracle-3.log`.
- `artifact-field-parity-test`: GREEN after Writer artifact/snapshot/diff correction.
- `writer-window-state-native`: GREEN, 2/2 tests; `🗑️generated/writer-window-state-native-agent-8.log`. The runtime law proves two exact-window config/transient partitions, independent generations, unchanged document/app config, isolated rendering, config pack reload, and transient reset. The test runs in an 8 MiB thread because the assembled fixture exceeds the default test stack.

## Equation validation checkpoint

The sole Equation camera is being moved from app config to exact `math-graph` window config:

- New owner facet: `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config`.
- `EquationGraphWindowConfig`, `EquationCamera`, `EquationGraphWindowConfigMutation::SetCamera`, the registered owner, typed addressing helper, Rust codec laws, and TS/JSON/GraphQL/Protobuf schemas are present.
- `EquationPlayApp` now uses `NoConfig`; app presence is removed; direct and retained `NodeGraphViewport` paths require trusted window context and emit `WindowConfig` mutations; graph rendering reads the exact window config.
- Six command generic boundaries now use `NoConfig`/`NoConfigMutation`.
- Ticket validation gained the permanent `equation-window-config-check` target through `validation/📜️script.ts` and `validation/project.json`.

Validation completed before the native runtime rerun:

- `equation-window-config-check`: GREEN for `cargo check --tests`; `🗑️generated/equation-window-config-check-4.log`.
- `equation-window-config-oracle`: GREEN. The language-neutral two-window fixture is checked by Ajv and an independent `fast-json-patch` fold; `🗑️generated/equation-window-config-oracle-2.log`.
- `artifact-field-parity-report`: Equation has zero artifact/snapshot/diff representation mismatches after the twelve JSON Schema, TypeScript, GraphQL, and Protobuf facets were corrected to canonical `ArtifactChild`/`DslValue` ownership. The repository total fell from 62 to 59; `🗑️generated/artifact-field-parity-report-equation-6.log`.

The first native run failed at compile time because its test tried to serialize `NoConfig`; that invalid assertion was removed. The second native run passed its mutation/codec law and exposed missing document-store close ownership. `EquationPlayApp` now binds both `bounded_document_store_owners()` and `ArtifactDocumentStoreDisposer` explicitly. The third run was canceled at root's request during disk recovery. Rerun 4 proved the mutation/codec law and isolated the missing internal `MemberStoreOwners` catalog. Rerun 5 is GREEN, 2/2 tests, in `🗑️generated/equation-window-config-native-5.log`: the neutral mutation/codec trace passes, and the assembled runtime publishes two exact graph-window configurations, renders their independent cameras, packs/reloads them, preserves document/app config, and closes terminal-empty. The wrapper supplied `CARGO_INCREMENTAL=0` throughout recovery-safe validation.

## Subsequent source-only parity checkpoint

Shooting and Sequence artifact/snapshot/diff contracts now describe their native composed-child fields in all four non-Rust representations:

- Shooting adds optional `emblem: ArtifactChild` for `s.stdio.semio.image`, with nullable replacement semantics in the diff.
- Sequence replaces the stale inline `steps`/`edges` aggregate fields with the exact `content: ArtifactChild` owner for `s.stdio.semio.flow`.

The permanent `artifact-field-parity-report` target is GREEN as a report and contains no Shooting or Sequence findings in `🗑️generated/artifact-field-parity-report-shooting-sequence-7.log`. The concurrent repository total is 44; the report does not attribute every concurrent reduction to this lane. These representation edits have not yet received family-native runtime validation.

Equation's window ownership, language-neutral oracle, native runtime law, and artifact/snapshot/diff parity checkpoints are complete.
