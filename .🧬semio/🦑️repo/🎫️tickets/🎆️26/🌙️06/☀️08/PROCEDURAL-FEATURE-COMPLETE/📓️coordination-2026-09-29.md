# Procedural 3D Completion Coordination

The requested outcome is an end-user BRep and mesh workbench for constructing, editing and analysing arbitrary shapes through typed procedural graphs.

The existing feature-completion ticket was reopened through the local repository MCP stdio server after reading `repo://goals`. MCP rejects the current model name because its bookkeeping catalogue is stale; reopening succeeded with the model field omitted. Execution agents use the explicitly requested GPT 6.1 Sol model. The coordinator cannot change its own model through the available tools. High execution and Low read-only exploration/audits use all three subordinate slots, with this coordinator as the fourth slot.

## Fleet

- `kernel_explore`, GPT 6.1 Sol Low: read-only kernel/widget capability and correctness audit.
- `ux_explore`, GPT 6.1 Sol Low: read-only user workflow, accessibility, localisation and verification audit.
- `widget_execute`, GPT 6.1 Sol High: schema-first widget completeness, catalogue and executable fixture work.
- Coordinator: integration plan, unowned end-user gaps, validation, ticket closure.

## Working Rules

Preserve other sessions' edits, use no modifying Git commands or worktrees, and leave AGENTS.md unchanged. Permanent executable commands belong to the existing `📜️script.ts` and Nx targets, with launch registrations. Keep research and summaries in this ticket; put transient outputs under `🗑️generated` and remove only this run's generated outputs at completion. Add language-neutral fixtures before feature changes and use existing third-party libraries as validation oracles.

## Acceptance Plan

Validate the complete operation catalogue against executable kernel support. Verify typed inputs, deterministic parameter validation, useful errors, shape preview, editable downstream graph insertion, analysis, history, progress and cancellation. Exercise both arbitrary mesh and BRep workflows and document geometry fidelity explicitly. Run targeted Nx checks and runtime probes; never infer passing checks or runtime behaviour from source alone.

Implementation slices will be recorded after the parallel audits establish the current state; existing README claims are evidence of intended behaviour, not verification.
