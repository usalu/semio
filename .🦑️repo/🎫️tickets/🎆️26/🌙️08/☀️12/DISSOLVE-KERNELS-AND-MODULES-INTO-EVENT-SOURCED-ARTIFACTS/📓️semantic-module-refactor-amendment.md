# Semantic Module Refactor Amendment

## Supersession

This amendment implements the developer's repository-wide semantic component
and module directive. It supersedes this ticket's earlier placement rule that
put reusable computational internals under `💡️inferences` merely because they
contributed to a derived result.

## Binding Placement Rule

- A semantic component has one maximally specific identity at
  `<collection>/<specific>/component.<language>`.
- A list root contains only generated or mechanical assembly, mounting,
  registration, and exports.
- Implementation used by one production semantic component stays private to
  that component.
- A module is allowed only when the same responsibility is used by at least two
  independent terminal production semantic components. Tests, examples, glue,
  re-exports, generated mirrors, language mirrors, multiple files, and multiple
  call sites in one component do not qualify.
- A qualifying module lives at the lowest common semantic owner of its terminal
  consumers. Domain-neutral cross-product reuse may live in framework; all
  other reuse remains at its subset, artifact, app, plugin, product, or `✏️s`
  owner.
- Existing `geometry`, `bounds`, `engine`, and `kernel` names are reviewed by
  responsibility rather than by spelling. Generic umbrellas are split,
  relocated, inlined, or removed.
- No compatibility aliases, forwarding re-exports, adapters, migrations, or
  legacy support are added.

## Scope

The implementation covers authored active code in `🧰️framework`, active
products, `✏️s`, and root tooling. It excludes `compose`, `🌎️hub`,
`♻️mit-bestand`, other legacy/exempt taxonomy areas, repository history,
vendored code, caches, build output, and direct generated-output edits.
Generated output changes through its source schema or generator and is then
regenerated.

## Immediate Control-Plane Result

The desktop MCP connector currently fails its own startup handshake, but the
repo stdio server itself initialized successfully and served `repo://goals` and
`repo://tickets` directly on 2026-08-15. Ticket #2550 is open under
`🎯aioptimizedrepo`; `🎯aioptimizedrepo/🎯singlefilerepo` is the governing
subgoal. The direct server session is the approved control-plane path until the
connector is repaired without tracked source changes.

## Current Protected Paths

Do not edit the user/other-agent-owned prompt, framework kernel/machine/platform
TypeScript components, OS renderer boot/Canvas2dHost/WorldTerrainLayer, or the
repo-library TypeScript index until their owner releases them and a fresh dirty
snapshot confirms no overlap.

## Next Milestone

Implement the schema-first semantic census and report-mode taxonomy enforcement
before semantic moves. The glTF `💡️inferences/📐️geometry` umbrella is the
first migration pilot: retain only genuinely specific derived inferences,
move proven shared capabilities to lowest-owner modules, and remove its bounds
alias atomically with every consumer and mount.
