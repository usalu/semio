# Generation Form Component Audit

The Generation3d generate form (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs:45-49`) derives a spec through `flow_host_snapshot_to_form_spec` and `PlaybookFlowFormProducer`, then uses the shared `generation_form`.

## Reachable findings

1. The bridge (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs:85-93`) admits only InputSlider, InputNote, InputImage, and Variable. The producer (`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧩️extensions/🌊️flow/🦀️.rs:30-48,129-154`) maps Variables with enum/single/select/choice schema to single and every other schema to text. Therefore numeric, boolean, or vector schema declarations are presented as free text, despite existing shared number, toggle, and vector controls. The producer also unconditionally uses the variable name as a string default, which does not provide a typed default. Correcting this requires defining the actual schema vocabulary and its typed defaults at the producer/Flow contract boundary, then wiring typed controls through the mutation and evaluation path.
2. Choice variables receive exactly one option whose value is the schema token itself, while their default is the variable name. The select can start with a value outside its options and does not expose genuine choices; the current Flow producer interface only loans `id`, `name`, and `schema`. A solution needs actual option data rather than inventing options from the token.
3. Slider, note, image, text, and single are the only emitted kinds. Shared rendering already uses framework slider, text, select and image nodes; the image branch is being handled separately. The generic JSON-text fallback is not reachable from this real producer, so the Playbook kinds `multi`, `date`, `color`, `file`, and extension blocks should not be described as current Generation3d form interactions.

## Shared form issues relevant if new kinds become reachable

The shared form (`✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/🦀️.rs:237-292`) handles number, boolean and vector directly. Vector child controls submit a scalar and `fieldIndex`; the real command uses `generation_operations` (`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs:515-519`), which ignores `fieldIndex` and replaces the entire stored question value with that scalar. Enabling vector production without extending that mutation route would corrupt the array. Playbook defaults identify `multi`, `date`, `color`, `file`, and extension kinds (`📖️playbook/🦀️.rs:254-268`), but these use the generic string fallback in the shared form and require typed components and typed commits before exposing them.

The 3D command (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️generation/🦀️.rs:22-46`) delegates to the Playbook mutation vocabulary, so typed values need to be preserved through that route rather than merely rendered with a different control.

Read-only code audit; no production edits or builds were performed.
