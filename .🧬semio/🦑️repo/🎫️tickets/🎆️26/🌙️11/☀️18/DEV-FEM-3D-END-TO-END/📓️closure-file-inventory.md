# Closure File Inventory

Conservative, non-exhaustive inventory from this ticket's explicit changed-file sections. Paths are repository-relative; no generated artifacts are included. A shared-worktree diff cannot establish ownership: unrelated peers also edit these paths, and some recorded edits may have since moved, been superseded or reverted. Validate the final file list against the current task state before passing it to `ticket_close`; do not treat a diff as exclusive attribution. Other ticket reports enumerate further infrastructure work.

## Command ingress and original return

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎮️command/🧬️op-binary/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/📥️ingress/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/🎟️typed/🏃️runtime/🦀️.rs`
- `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/📦️return/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/🪙️return-escrow/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🎟️control/📨️reply/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🎟️control/📨️reply/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs`

Evidence: `📓️analysis-queued-and-shell-stall.md`, `📓️ingress-slot-settle.md`, `📓️set-active-example-no-progress.md`. The latter also records temporary diagnostic edits; include a file only if its final task change remains.

## Host event batch delivery

- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🪪️exact-activation/🟦️.ts`
- `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🔖️turn-result/🟦️.ts`
- `🧰️framework/🔨️modules/🎠️kernel/🎛️activation-registry/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🪝️wire-codec/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🔁️turn-settle/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🧾️typed-operations-and-handle/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🔌️plugin-module-loader/🖼️render-and-turn-drivers/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🎟️activation-lease/🟦️.ts`

Evidence: `📓️event-batch-ui-patches.md`.

## Explicit infrastructure fixes

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/💾️storage/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/📤️file-serving/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/📦️asset-delivery/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🧪️tests/🟦️.ts`
- `✏️s/🧑‍💻dev/🏗️fem/📖️stories/🎭️3d-viewer/🧪️.story.tsx`

Evidence: the linked status reports for graph slice, cargo lock refresh, Windows scratch paths, cargo lock EPERM, publish-served-directory and status. More source paths for graph memo and source admission are listed in `📓️graph-admission-memo.md` and `📓️source-admission-throughput.md`; their grouped relative paths need expansion and current-state verification before closure.

## Ticket artifacts

The authored ticket reports and retained test inputs/scripts are inside `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️18/DEV-FEM-3D-END-TO-END/`. Preserve them; do not include files under `🗑️generated`. Recent FEM mesh, picker, preparation and scheduler reports describe changes but do not consistently enumerate exact file paths, so this inventory does not infer ownership of their shared source files from names alone.
