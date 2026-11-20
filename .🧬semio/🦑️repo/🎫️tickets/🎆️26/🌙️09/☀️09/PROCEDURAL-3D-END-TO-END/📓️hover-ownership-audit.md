# Hover ownership audit

Read-only source audit, 2026-11-20. No runtime test performed.

WGPU row leave in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🏗️router-construction-and-intents/🦀️.rs:372` emits empty targets. World3d leave in `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/📜️plan-and-publish/🦀️.rs:29` suppresses duplicate leaves locally but publishes the same unconditional clear. React row leave has the same form in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧱️node-tree/🟦️.tsx:179`.

The interaction handler `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📥️staging/🧭️interaction/🦀️.rs:335` unconditionally clears the domain. `DomainHover` in `🧰️framework/🔨️modules/📡️replication/📡️wire/🕹️interaction/🦀️.rs:371` stores only channel and expanded IDs; `next_hover` at line 716 has no owner or previous-state comparison. ActionMeta has no surface or sequence identity. A delayed leave from an old surface can therefore clear a newer publisher; this ordering needs a live integration test. Comparing IDs alone cannot distinguish two surfaces publishing the same target. A complete fix needs a source token transported and stored for each producer and compare-and-clear on admission, including React and WGPU graph, tree and world paths.

Suggested test: enter surface A, enter B, deliver A leave after B enter, verify B remains; repeat with both surfaces publishing the same target and with graph/component domains.

## Implementation and verification

The shared interaction reducer now associates the current local hover with its projected window or the panel role. A leave with another window, panel role or channel keeps the published hover intact, including when both surfaces point at the same target. Document revalidation retires the ownership entry when its hover disappears. A headless clear remains available for its requested channel. Browser and WGPU event integration tests remain pending. The neutral seven-case law was expanded to ten cases to cover both orderings between the inspector and a preview window.
