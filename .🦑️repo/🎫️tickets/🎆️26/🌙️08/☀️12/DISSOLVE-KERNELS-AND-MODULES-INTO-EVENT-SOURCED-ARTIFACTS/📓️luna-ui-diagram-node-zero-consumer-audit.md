# Luna UI Diagram Node Zero-Consumer Audit

## Baseline and Scope

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Active scope excludes `compose`, `🌎️hub`, `♻️mit-bestand`, legacy/exempt taxonomy areas, tickets/history, generated outputs, dependencies, and build caches.
- This record transcribes the read-only Luna result into the active ticket; Luna modified no file.

## Definition and Hashes

| Path | SHA-256 | State |
|---|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️DiagramNode/🟦️component.tsx` | `3f0fd02b9a2236f72a631e783dca9ebd1e63f261635a12d1cae7306b139106f4` | clean |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️DiagramNode/🧪️story.tsx` | `aafd1ffbf1730ac5e7a1133daef362b144b9d6f077c0f074165feaba8378a85c` | clean |
| React package index | `01005e76dbc844cbaa2e9c8b2e6b7727bfd3d575f7ef887e62c3f1ce249c4a52` | dirty only from accepted serialized removals; target region untouched |

Exports are `DiagramNodeProps`, `DiagramNode`, and `PlaceholderDiagramNode`.

## Consumer Closure

- Mechanical barrel: imports/re-exports all three symbols. This is glue and not a production consumer.
- Exclusive `DiagramNode` story: example/test only.
- Canvas story: imports `DiagramNode` and renders five visual examples; example/test only.
- Diagram story: imports `DiagramNode` and renders four visual examples; example/test only.
- No authored test imports either component.
- `PlaceholderDiagramNode` has no consumer outside its definition and barrel.
- The OS renderer `NodeGraph` defines an unrelated local `WorkflowDiagramNode`; it does not import or render the framework component and therefore is not a consumer or writable path.

There are zero independent production terminal consumers. Story call sites and package glue do not qualify under the production-consumer rule.

## Decision

Delete the component and exclusive story, remove only its examples/imports from Canvas and Diagram stories while retaining their other coverage, and remove the exact shared React registrar region. Do not inline, retain, extract a module, touch the protected renderer, or add compatibility surface.
