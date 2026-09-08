# Legacy JavaScript and TypeScript Test Attribution

## Scope

This is a read-only recovery against pre-goal commit `6152f9ca6a0fbb55aa61992077837a230996b51d`. Its Git tree has 65 JavaScript/TypeScript `*.test.*` or `*.spec.*` paths (49 TypeScript, 8 TSX, and 8 JavaScript). The separate tracked baseline's 69 total also includes four Go `_test.go` files, which are intentionally outside this map.

The evidence mapping below records each baseline path. A `matched` row identifies the current direct canonical implementation that carries the same normalized source/test identity; `contentDice` is the token-set Dice score after removing language boilerplate, and `sharedTokens` is its numerator evidence. The old and new path names provide the semantic-name evidence. A score of 0.65 was the automatic proof threshold.

```json
[
  {
    "old": ".storybook/animate-presentation-deck.spec.ts",
    "status": "matched",
    "new": "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/🧪️tests/🎞️storybook-deck/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 83
  },
  {
    "old": ".storybook/cad-renderer.spec.ts",
    "status": "matched",
    "new": "✏️s/🔌️plugins/📐️cad/🧪️tests/🎨️storybook-renderer/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 111
  },
  {
    "old": ".storybook/framework-hosts-no-wasm.spec.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📚️storybook-hosts-no-wasm/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 240
  },
  {
    "old": ".storybook/framework-hosts-wasm.spec.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📚️storybook-hosts-wasm/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 148
  },
  {
    "old": ".storybook/os-plugins.spec.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/📚️storybook-plugins/🟦️.ts",
    "contentDice": 0.9935,
    "sharedTokens": 154
  },
  {
    "old": ".storybook/puzzle-2d.spec.ts",
    "status": "matched",
    "new": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/◻️storybook-2d/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 183
  },
  {
    "old": ".storybook/puzzle-3d-5d-infinite.spec.ts",
    "status": "matched",
    "new": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️storybook-spatial/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 231
  },
  {
    "old": ".storybook/s-end-to-end.spec.ts",
    "status": "matched",
    "new": "✏️s/🧪️tests/🎭️storybook-end-to-end/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 223
  },
  {
    "old": ".storybook/styling.spec.ts",
    "status": "matched",
    "new": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🎨️storybook/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 192
  },
  {
    "old": ".storybook/ui-new-stories.spec.ts",
    "status": "matched",
    "new": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-new-stories/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 230
  },
  {
    "old": ".storybook/ui-uncovered-components-stories.spec.ts",
    "status": "matched",
    "new": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-uncovered-components/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 142
  },
  {
    "old": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️02/☀️02/FIX-D3-FORCE-DIAGRAM-NODE-EDGE-ALIGNMENT/node-edge-alignment.test.ts",
    "status": "unmatched",
    "new": null,
    "contentDice": 0.1165,
    "sharedTokens": 6
  },
  {
    "old": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️02/☀️02/FIX-D3-FORCE-DIAGRAM-NODE-EDGE-ALIGNMENT/test-node-edge-alignment.spec.ts",
    "status": "unmatched",
    "new": null,
    "contentDice": 0.1442,
    "sharedTokens": 23
  },
  {
    "old": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️02/☀️11/FIX-ALL-SIX-FAILING-SKETCHPAD-PLAYWRIGHT-TESTS/debug.test.ts",
    "status": "unmatched",
    "new": null,
    "contentDice": 0.1921,
    "sharedTokens": 17
  },
  {
    "old": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️03/☀️20/FIX-ALGORITHM-STORYBOOK-INFINITE-HANG-ON-LOAD/seed.spec.ts",
    "status": "unmatched",
    "new": null,
    "contentDice": 0.1584,
    "sharedTokens": 8
  },
  {
    "old": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️01/PLAYGROUND-WINDOW-MODE-COMPLETENESS-PASS/audit-playground-completeness.test.ts",
    "status": "unmatched",
    "new": null,
    "contentDice": 0.127,
    "sharedTokens": 4
  },
  {
    "old": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/STALE-CONFIG-FIXES-AND-CAPABILITY-LINT-REVIVAL/deep/nested/probe.test.ts",
    "status": "unmatched",
    "new": null,
    "contentDice": 0.1818,
    "sharedTokens": 2
  },
  {
    "old": "♻️mit-bestand/🧺️demonstrator/🧪️demonstrator.acceptance.spec.ts",
    "status": "matched",
    "new": "♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 547
  },
  {
    "old": "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/⚡️implementations/🟦️typescript/🧪️index.test.ts",
    "status": "matched",
    "new": "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/🧪️tests/🧩️index/🟦️.ts",
    "contentDice": 0.9808,
    "sharedTokens": 51
  },
  {
    "old": "✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/🧪️tests/🌐️sequence-browser-consumer.test.js",
    "status": "matched",
    "new": "✏️s/🔌️plugins/🎬️sequence/🧪️tests/🌐️browser-consumer/🟨️.js",
    "contentDice": 1,
    "sharedTokens": 97
  },
  {
    "old": "✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/🧪️tests/🔮️sequence-protocol-oracle.test.js",
    "status": "matched",
    "new": "✏️s/🔌️plugins/🎬️sequence/🧪️tests/🔮️protocol-oracle/🟨️.js",
    "contentDice": 0.9606,
    "sharedTokens": 122
  },
  {
    "old": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🧪️tests/🧪️sequence-host.test.js",
    "status": "matched",
    "new": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🖥️host/🟨️.js",
    "contentDice": 1,
    "sharedTokens": 301
  },
  {
    "old": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🧪️tests/🧬️sequence-schema.test.js",
    "status": "matched",
    "new": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🧬️schema/🟨️.js",
    "contentDice": 0.6809,
    "sharedTokens": 64
  },
  {
    "old": "✏️s/🔌️plugins/🗒️note/🧪️action-cohort/🧪️component.test.ts",
    "status": "matched",
    "new": "✏️s/🔌️plugins/🗒️note/🧪️tests/🧭️action-cohort/🟦️.ts",
    "contentDice": 0.9408,
    "sharedTokens": 167
  },
  {
    "old": "🌎️hub/📦️packages/🟦️typescript/🤝️index.test.ts",
    "status": "matched",
    "new": "🌎️hub/🧪️tests/🤝️integration/🟦️.ts",
    "contentDice": 0.9864,
    "sharedTokens": 691
  },
  {
    "old": "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🛡️admin.test.tsx",
    "status": "matched",
    "new": "🌎️hub/🔨️modules/🛡️admin/🧪️tests/🛡️admin/🟦️.tsx",
    "contentDice": 1,
    "sharedTokens": 407
  },
  {
    "old": "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🧩️slot.test.tsx",
    "status": "matched",
    "new": "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🧪️tests/🧩️slot/🟦️.tsx",
    "contentDice": 1,
    "sharedTokens": 85
  },
  {
    "old": "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🟨️javascript/🧪️webgpu-surface.test.js",
    "status": "matched",
    "new": "🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🖼️webgpu-surface/🟨️.js",
    "contentDice": 0.9907,
    "sharedTokens": 212
  },
  {
    "old": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🟨️javascript/🧪️browser-host.test.js",
    "status": "matched",
    "new": "🧰️framework/🔨️modules/🖱️ui/🖥️host/🧪️tests/🌐️browser-host/🟨️.js",
    "contentDice": 0.9954,
    "sharedTokens": 218
  },
  {
    "old": "🧰️framework/🔨️modules/🧬️schema/✅️draft07-oracle.test.ts",
    "status": "matched",
    "new": "🧰️framework/🔨️modules/🧬️schema/🧪️tests/✅️draft07-oracle/🟦️.ts",
    "contentDice": 0.9162,
    "sharedTokens": 82
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🌅️modern-era.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🌅️modern-era/🟦️.ts",
    "contentDice": 0.9937,
    "sharedTokens": 158
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🏛️legacy-conformance.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🏛️legacy-conformance/🟦️.ts",
    "contentDice": 0.9965,
    "sharedTokens": 283
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/💡️inference-bridge.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/💡️inference-bridge/🟦️.ts",
    "contentDice": 0.9619,
    "sharedTokens": 290
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🔄️end-to-end.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔄️end-to-end/🟦️.ts",
    "contentDice": 0.9974,
    "sharedTokens": 378
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🔐️authenticated-hub-workspace.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔐️authenticated-hub-workspace/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 101
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧹️hygiene.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🧹️hygiene/🟦️.ts",
    "contentDice": 0.9947,
    "sharedTokens": 187
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🧪️flow-host.test.js",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🖥️host/🟨️.js",
    "contentDice": 0.9742,
    "sharedTokens": 283
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🧪️flow-open-ownership.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🔓️open-ownership/🟦️.ts",
    "contentDice": 0.9861,
    "sharedTokens": 71
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🧬️flow-schema-oracle.test.js",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🧬️schema-oracle/🟨️.js",
    "contentDice": 0.915,
    "sharedTokens": 70
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/⚡️quick.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚡️quick/🟦️.ts",
    "contentDice": 0.9121,
    "sharedTokens": 109
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🏛️space-administration.test.tsx",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🏛️space-administration/🟦️.tsx",
    "contentDice": 1,
    "sharedTokens": 274
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/👥️scoped-presence.test.tsx",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/👥️scoped-presence/🟦️.tsx",
    "contentDice": 0.9574,
    "sharedTokens": 180
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📇️directory-home-bootstrap.test.tsx",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📇️directory-home-bootstrap/🟦️.tsx",
    "contentDice": 0.9644,
    "sharedTokens": 244
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️document-opening.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️document-opening/🟦️.ts",
    "contentDice": 0.4925,
    "sharedTokens": 66
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️index.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts",
    "contentDice": 0.9889,
    "sharedTokens": 3919
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🚪️opening.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🚪️opening/🟦️.ts",
    "contentDice": 0.8942,
    "sharedTokens": 93
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧯️router-plugin-faults.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧯️router-plugin-faults/🟦️.ts",
    "contentDice": 0.9762,
    "sharedTokens": 82
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🩺️window-fault.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🩺️window-fault/🟦️.ts",
    "contentDice": 0.9781,
    "sharedTokens": 156
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️component.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 242
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🚦️AgentPresence/🧪️component.test.tsx",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🚦️AgentPresence/🧪️tests/🧩️component/🟦️.tsx",
    "contentDice": 1,
    "sharedTokens": 76
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️component.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 120
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🤖️AgentApprovals/🧪️component.test.tsx",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🤖️AgentApprovals/🧪️tests/🧩️component/🟦️.tsx",
    "contentDice": 1,
    "sharedTokens": 124
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧭️TiledMapHost/🧪️component.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧭️TiledMapHost/🧪️tests/🧩️component/🟦️.ts",
    "contentDice": 0.994,
    "sharedTokens": 167
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧵️TaskManager/🧪️component.test.tsx",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧵️TaskManager/🧪️tests/🧩️component/🟦️.tsx",
    "contentDice": 1,
    "sharedTokens": 259
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-complete.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts",
    "contentDice": 0.9905,
    "sharedTokens": 524
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️generated-projection.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/📖️generated-projection/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 124
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts",
    "contentDice": 0.9878,
    "sharedTokens": 202
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🪪️plugin-identity.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🪪️plugin-identity/🟦️.ts",
    "contentDice": 0.9563,
    "sharedTokens": 197
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧹️config.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 182
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🧪️extension.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/🧩️extension/🟦️.ts",
    "contentDice": 0.9993,
    "sharedTokens": 714
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
    "contentDice": 0.9989,
    "sharedTokens": 3648
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🔬️schema.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧪️tests/🔬️schema/🟦️.ts",
    "contentDice": 1,
    "sharedTokens": 103
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🔬️server-persistence.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧪️tests/🔬️server-persistence/🟦️.ts",
    "contentDice": 0.9598,
    "sharedTokens": 155
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🧪️index.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts",
    "contentDice": 0.9866,
    "sharedTokens": 1252
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🧬️invariants.test.ts",
    "status": "matched",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️schema-invariants/🟦️.ts",
    "contentDice": 0.6904,
    "sharedTokens": 408
  }
]
```
## Exact Flat Path Manifest

This JSON array contains repository-relative strings only: every attributed active baseline legacy path, every proven canonical destination, all eleven changed caller/configuration paths, and this report. The six ticket-private scratch paths remain as historical baseline evidence but are excluded from the active authored-test coverage count and have no asserted deletion or migration attribution to this goal.

```json
[
  ".storybook/animate-presentation-deck.spec.ts",
  ".storybook/cad-renderer.spec.ts",
  ".storybook/framework-hosts-no-wasm.spec.ts",
  ".storybook/framework-hosts-wasm.spec.ts",
  ".storybook/os-plugins.spec.ts",
  ".storybook/playwright.config.ts",
  ".storybook/puzzle-2d.spec.ts",
  ".storybook/puzzle-3d-5d-infinite.spec.ts",
  ".storybook/s-end-to-end.spec.ts",
  ".storybook/styling.spec.ts",
  ".storybook/ui-new-stories.spec.ts",
  ".storybook/ui-uncovered-components-stories.spec.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️test-layout-legacy-js-ts-attribution-2026-09-08.md",
  "♻️mit-bestand/🧺️demonstrator/🧪️demonstrator.acceptance.spec.ts",
  "♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts",
  "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/⚡️implementations/🟦️typescript/🧪️index.test.ts",
  "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/🧪️tests/🎞️storybook-deck/🟦️.ts",
  "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/🧪️tests/🧩️index/🟦️.ts",
  "✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/🧪️tests/🌐️sequence-browser-consumer.test.js",
  "✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/🧪️tests/🔮️sequence-protocol-oracle.test.js",
  "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🧪️tests/🧪️sequence-host.test.js",
  "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🧪️tests/🧬️sequence-schema.test.js",
  "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🖥️host/🟨️.js",
  "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🧬️schema/🟨️.js",
  "✏️s/🔌️plugins/🎬️sequence/🧪️tests/🌐️browser-consumer/🟨️.js",
  "✏️s/🔌️plugins/🎬️sequence/🧪️tests/🔮️protocol-oracle/🟨️.js",
  "✏️s/🔌️plugins/📐️cad/🧪️tests/🎨️storybook-renderer/🟦️.ts",
  "✏️s/🔌️plugins/🗒️note/🧪️action-cohort/🧪️component.test.ts",
  "✏️s/🔌️plugins/🗒️note/🧪️tests/🧭️action-cohort/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/◻️storybook-2d/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️storybook-spatial/🟦️.ts",
  "✏️s/🧪️tests/🎭️storybook-end-to-end/🟦️.ts",
  "🌎️hub/📦️packages/🟦️typescript/vitest.config.ts",
  "🌎️hub/📦️packages/🟦️typescript/🤝️index.test.ts",
  "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🛡️admin.test.tsx",
  "🌎️hub/🔨️modules/🛡️admin/🧪️tests/🛡️admin/🟦️.tsx",
  "🌎️hub/🧪️tests/🤝️integration/🟦️.ts",
  "📜️script.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🎨️storybook/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
  "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🧩️slot.test.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🧪️tests/🧩️slot/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🟨️javascript/🧪️webgpu-surface.test.js",
  "🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🖼️webgpu-surface/🟨️.js",
  "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🟨️javascript/🧪️browser-host.test.js",
  "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🔨️modules/🖱️ui/🖥️host/🧪️tests/🌐️browser-host/🟨️.js",
  "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-new-stories/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-uncovered-components/🟦️.ts",
  "🧰️framework/🔨️modules/🧬️schema/✅️draft07-oracle.test.ts",
  "🧰️framework/🔨️modules/🧬️schema/🧪️tests/✅️draft07-oracle/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🌅️modern-era.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🏛️legacy-conformance.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/💡️inference-bridge.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🔄️end-to-end.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🔐️authenticated-hub-workspace.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧹️hygiene.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🌅️modern-era/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🏛️legacy-conformance/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/💡️inference-bridge/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔄️end-to-end/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔐️authenticated-hub-workspace/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🧹️hygiene/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🧪️flow-host.test.js",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🧪️flow-open-ownership.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🧬️flow-schema-oracle.test.js",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🔓️open-ownership/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🖥️host/🟨️.js",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🧬️schema-oracle/🟨️.js",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/⚡️quick.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🏛️space-administration.test.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/👥️scoped-presence.test.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📇️directory-home-bootstrap.test.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️document-opening.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️index.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🚪️opening.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧯️router-plugin-faults.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🩺️window-fault.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚡️quick/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🏛️space-administration/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/👥️scoped-presence/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📇️directory-home-bootstrap/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📚️storybook-hosts-no-wasm/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📚️storybook-hosts-wasm/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️document-opening/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🚪️opening/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧯️router-plugin-faults/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🩺️window-fault/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️component.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🚦️AgentPresence/🧪️component.test.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🚦️AgentPresence/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️component.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🤖️AgentApprovals/🧪️component.test.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🤖️AgentApprovals/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧭️TiledMapHost/🧪️component.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧭️TiledMapHost/🧪️tests/🧩️component/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧵️TaskManager/🧪️component.test.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧵️TaskManager/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-complete.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️generated-projection.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/📖️generated-projection/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/📚️storybook-plugins/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🪪️plugin-identity/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🪪️plugin-identity.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧹️config.test.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📜️script.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🧪️extension.test.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/🧩️extension/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🔬️schema.test.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🔬️server-persistence.test.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧪️tests/🔬️schema/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧪️tests/🔬️server-persistence/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📜️script.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🧪️index.test.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🧬️invariants.test.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️schema-invariants/🟦️.ts"
]
```

## Result

59 paths have a proven current canonical replacement. The six `unmatched` entries are historical ticket-private scratch paths. They remain baseline evidence but are excluded from active authored-test coverage; no deletion or migration attribution to this goal is asserted for them.

The former `🔬️document-opening.test.ts` maps to `🔬️document-opening/🟦️.ts` by retained test identity: the exact test title, `documentOpeningFixture.cases`, `runDocumentOpeningAttemptV1`, `socket`, `attach`, and `vi.getTimerCount()` are present. Its 0.4925 token-set Dice score is below the automatic 0.65 threshold because the current canonical leaf incorporates additional document-opening cases; the retained assertion identity proves the move. The semantically similar `🚪️opening/🟦️.ts` does not retain those identities and is not used as the destination.

Excluding that documented assertion-retention match, the lowest automatic normalized-content score is 0.6809. This includes the moved `🏛️space-administration.test.tsx` case, whose destination is `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🏛️space-administration/🟦️.tsx` with a score of 1.0.

## Changed Explicit Callers and Configuration

The following current callers/configuration paths resolve to proven replacements and did not resolve that same canonical target in the base tree. They are recorded only as layout-routing evidence.

- `.storybook/playwright.config.ts` directly references all eleven migrated Storybook leaves.
- `🌎️hub/📦️packages/🟦️typescript/vitest.config.ts` references `🌎️hub/🧪️tests/🤝️integration/🟦️.ts`.
- `📜️script.ts` references the migrated renderer `🔬️engine-contract` case.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts` references `🔬️workspace-contract`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📜️script.ts` references `🧩️extension`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📜️script.ts` references `🧪️test-platform` and `🧬️schema-invariants`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts` references the migrated `🚪️opening`, `🔬️engine-contract`, `📇️directory-home-bootstrap`, `🏛️space-administration`, and `👥️scoped-presence` cases.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts` references the migrated wasm `🖥️host`, `🔓️open-ownership`, and `🧬️schema-oracle` cases.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts` references `🧩️slot`.
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📜️script.ts` references `🖼️webgpu-surface`; `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📜️script.ts` references `🌐️browser-host`.

No runtime, Nx, source, or Git-state operation was performed.

Closure attribution correction: the six unmatched historical ticket-private scratch paths remain in the evidence rows, but are excluded from the flat authored-file array because this lane has no evidence it changed them. The flat closure array contains 130 paths.
