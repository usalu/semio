# BlockList Palette Target

## Accepted contract

Palette activation resolves its current step from the decoded ordered step snapshot: a selected step wins, otherwise the step containing the selected block wins, otherwise the first current step. A stale selection therefore cannot escape the current snapshot. An empty snapshot exposes no palette action and no palette drag.

The exact accepted action is `addBlock { stepId, kind }`. `draggingId` and `domainId` are not target authorities. A palette transfer still carries its block-kind MIME and the explicit drop step supplies its destination.

## Neutral fixture

- `🧰️framework/🔨️modules/🖱️ui/🧪️fixtures/🧩️block-list-presentation/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🧩️block-list-presentation/🔣️.json`

The fixture covers selected-step, selected-block-parent, stale-selection, no-selection, and empty-list cases. Ajv validates it before the mounted React oracle consumes it.

## React receipt

Production `BlockListHost` resolves the target once from the current `stepsJson` projection and `selectedId`. Click, Enter, and Space dispatch the same exact action. Empty lists disable the native button, omit the transfer handle, and refuse drag MIME publication.

Command:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long bun nx run @semio-tech/framework-renderer-react:test --excludeTaskDependencies -- '../../../../🧪️tests/🧩️block-list-presentation/🟦️.tsx' --silent=false --reporter=verbose
```

Result: 1 file passed, 2/2 tests passed, Vitest 17.64 s, Nx 19.5 s. Output is retained temporarily at `🗑️generated/sol-block-list-target/react-focused.log`.

## WGPU implementation

The WGPU plan now resolves the target with the same ordered snapshot rule and carries the exact `stepId` in its physical action. An empty list still paints the palette affordance as disabled chrome but publishes no hit, transfer handle, drag surface, or virtual accessibility action. Accessibility activation revalidates both the accepted block kind and the current resolved target step before dispatch, so a selected-step change or retirement refuses the stale control.

The Scenes law consumes every fixture target case, checks the exact physical and accepted accessibility action, proves empty-list omission, and rejects both a stale controller and a changed target. The Interpreter wiring law now carries a current `basics` step and checks the exact `{ stepId: "basics", kind: "filter" }` descriptor. These Rust changes are source-coherent but have not executed yet: the shared native queue is draining and the existing focused Projection compile will be the first current-source compile receipt. No native pass is claimed.
