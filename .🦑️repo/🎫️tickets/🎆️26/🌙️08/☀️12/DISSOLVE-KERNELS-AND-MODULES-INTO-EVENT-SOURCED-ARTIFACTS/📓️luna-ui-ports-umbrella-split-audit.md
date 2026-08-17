# UI Ports Umbrella Split Audit

## Baseline

- HEAD verified by coordinator: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Source: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/🟦️component.tsx`
- Source SHA-256: `dbfe7968b1322633ba5db67f86a74e9f88b529d2558cae76044e68102fd347ad`
- Source state: clean, 165 lines

## Responsibilities

The file combines three unrelated external-host boundaries:

- React host interface, mutable binding, and setter;
- React Three Fiber, Drei, and Three scene-host interface, mutable binding, and setter;
- XYFlow host interface, mutable binding, setter, and two evaluation-time JSX aliases.

Importing any React-only consumer currently initializes React, Three, Drei, Fiber, and XYFlow adapters together.

## Consumer Graph

### React Host

Thirty-four active framework UI element files directly consume `reactHostPort`. Additional terminal production consumers through the framework package include CAD renderer, Infinite World R3F renderer, Infinite canvas React reconciler, and World3dHost. This responsibility has strong independent-consumer evidence.

### Scene Host

`Scene` is the direct framework consumer. Independent terminal production consumers through the package include CAD renderer and Infinite World R3F renderer. This responsibility also meets the two-consumer threshold.

### Flow Host

Only `Diagram` consumes `HostReactFlow` and `HostReactFlowProvider`. Two symbols in the same component are one consumer. There is no second independent production terminal.

The framework React barrel and renderer package barrel are assembly/glue and do not count. Storybook, tests, compose, generated, and legacy references are excluded.

## Runtime and Boundary Findings

- The mixed source is outside the local SCC because it imports only external adapters, but the framework React barrel remains in SCCs with elements it imports and that import authored helpers from it. Direct port imports prevent top-level `createContext`, `forwardRef`, `memo`, and scene reads from traversing the barrel cycle.
- `HostReactFlow` and `HostReactFlowProvider` snapshot the initial mutable port. Calling `setFlowHostPort` cannot update those aliases.
- Port interfaces directly expose React, Three, Drei, Fiber, and XYFlow library types. The implementation therefore does not yet satisfy the repository-owned external-library boundary rule.
- The three setters have no active production caller in the bounded graph. Current configuration use is test/Storybook-only, but mutation/configuration remains an inseparable facet if retained as the actual repository interface boundary.

## Disposition

- Delete the mixed `Ports` element identity.
- Move the React host boundary to a specific shared UI-owner module.
- Move the scene host boundary to a separate specific shared UI-owner module.
- Inline the Flow host adapter privately into Diagram and remove its mutable port/setter/snapshot aliases; it has one production consumer.
- Update all direct importers and explicit barrel registrations atomically, preserving the top-level cycle break.
- Replace external-library-shaped public contracts with the narrowest repository-owned interfaces required by consumers. Where JSX/runtime interoperability requires an external type, expose a repository-owned alias or explicit repository re-export rather than requiring consumers to import the external library contract.

This lease overlaps 34 UI sources, the protected React barrel, and external renderer boundary validation. It is graph-proven but must wait until the active ClassNames consumer rewrite releases the same paths.
