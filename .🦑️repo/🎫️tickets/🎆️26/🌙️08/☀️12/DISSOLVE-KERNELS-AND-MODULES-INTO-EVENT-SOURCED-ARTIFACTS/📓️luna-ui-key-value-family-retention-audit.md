# UI Key Value Family Retention Audit

## Snapshot

- Specific Rust/wgpu element: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔑️KeyValue/🧊️component.rs`.
- SHA-256: `570d5766ac0c8052cad6c06b357470e4be6a3c4d035bcdc2929616d79b9cd3e5`.
- The definition was clean and has no story.
- The shared React barrel has no KeyValue export or implementation.

## Active Semantic Closure

- The wgpu immediate renderer mounts `render_key_value` through the package glue and calls it for both widget and control nodes.
- The wgpu retained renderer independently dispatches `UiNode::KeyValue` to its retained paint facet.
- The OS React interpreter independently renders KeyValue nodes as a definition list.
- Manifest Rust and TypeScript schemas own `UiKeyValueEntry` and `UiKeyValueNode`; platform and shell helpers provide registry/routing glue.
- Active plugin producers emit KeyValue nodes but are not rendering terminals.

## Disposition

Retain `KeyValue` as one maximally specific cross-language UI component. It is a UI component, not a `modules/<specific>` capability, so language mirrors and renderer facets are not used to manufacture a module consumer count. A future coordinated family lease should co-locate the retained wgpu `paint_key_value` behavior from the package `paint.rs` with the existing KeyValue Rust facet while preserving the schema and renderer dispatches. The central wgpu paint/schema/platform/renderer files are protected from an isolated cleanup lease, so no zero/one-consumer Terra packet is safe now.
