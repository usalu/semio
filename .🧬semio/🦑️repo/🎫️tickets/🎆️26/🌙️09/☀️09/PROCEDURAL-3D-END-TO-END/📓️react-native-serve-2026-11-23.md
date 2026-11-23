# React generation3d native serve (2026-11-23)

Goal: serve `http://127.0.0.1:6018/?plugin=generation3d` without the repo graph, then run the React battery.

## State at writing

- The native Vite host starts and resolves its full config without the graph (proved by the receipt read below). It stops at exactly one missing input: the activation receipt `.cache/b/p/m/browser/prototype/runtime/react/generation3d/activation/🔣️receipt.json`.
- That receipt needs the staged `procedural` plugin component (`.cache/b/p/m/browser/prototype/modules/🌀️procedural/`). Only `🖨️raster`, `🧵️shard` and `🪞️vendor` are staged today. The component is a `wasm32-wasip2` build (describe, transpile, optimize, publish stages) driven by `buildComponentClosureV1`, which requires the repo's issued `ScriptInvocation`. Running the script outside `repo` fails with `Original parent Script process invocation required` (log `🗑️generated/native-prepare-1.txt`). It cannot be driven from a plain bun script without forging the capability envelope, which I did not do.
- So the only legitimate build path is `repo run semio-hub-procedural-component:component -c prototype --exclude-dependencies`. Attempts and why they did not finish are under "Blockers".

## How to start (once the component is staged and activated)

```
# 1. stage + activate (repo-owned, needs the graph slice for ONE owner only)
repo run semio-hub-procedural-component:component -c prototype --exclude-dependencies
repo run @semio-tech/framework-os-dev:activate-generation3d-react-prototype --exclude-dependencies   # writes the receipt
# 2. serve (graph-free)
sh .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/serve-generation3d-native.sh   # log: 🗑️generated/serve-native-6018.txt
# or via the launch config "generation3d-react-native" in .claude/launch.json
# 3. battery
cd <ticket> && PATH=$(dirname $(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)):$PATH bun 🐍️react-battery.mjs
```

## What was fixed (source)

1. No dev owner existed for `generation3d` and `generation2d`. The generic `serve` path falls back to the neutral framework config, whose inventory is empty, so `requirePlaygroundVariant`/`resolvedPluginId` throws "Unknown playground module identity" and the entry mounts an empty catalog. Every other served variant (raster, process, pdf, gis, wfc, draw, fem, note) has an owner under `✏️s/🧑‍💻dev/🎭️variants/`. New owner `✏️s/🧑‍💻dev/🎭️variants/🌀️procedural/` (`🧬️schema/🔣️.json`, `🏗️builder/🌐️vite/🟦️.ts`, `🚀️entry/🟦️.ts`), same shape as raster, entry restricted to `generation2d`/`generation3d`.
2. Wired it: `devContribution` added to both `[[package.metadata.semio.playground]]` rows in `🌎️hub/🧩️compositions/🌀️procedural/🧩️component/📦️packages/🦀️rust/Cargo.toml`, and the same field in the generated catalog `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/{🚀️playgrounds.json,🎠️playgrounds.json,🎮️playgrounds/🟦️.ts}` (edited by hand to equal what the generator emits for raster; regenerate to confirm no drift).
3. Serve tooling: `serve-generation3d-native.ts` / `.sh` in this ticket, launch entry `generation3d-react-native` in `.claude/launch.json`.

## Blockers observed

- `repo run ...:dev-generation3d-react-prototype` / `...:component`: the graph slice read took 20 to 40 minutes even for a single owner (`--exclude-dependencies`), and one attempt died with `BUILD_SOURCE_CONFLICT: Conflicting inference observations` on `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🟦️.ts` because a peer was editing that file (mtime 01:20). The third attempt passed the slice (`graph: plan answered and verified`, `tasks: 1 stale`).
- After the plan, the component command waits for a shared lease on `native-fixed-image:.cache/b/d`. `repo leases status` shows it held exclusive for over 90 minutes by process 17436 (`tools setup all`, started by another lane); my process (25339) and a `🔌️plugin` process (58486) queue behind it. Not mine to kill.
- Logs: `🗑️generated/native-component-procedural-{1,2,3}.txt`, `🗑️generated/native-prepare-1.txt`, `🗑️generated/serve-native-6018.txt`.

## Battery results

Not run: no server can serve the editor without the staged `procedural` component. Before: all rows red (nothing serves). After: unchanged. The earlier audit's example-oracle path bug (`🐍️example-oracle.mjs`) is already fixed in this tree (`🐍️example-fixtures.mjs`).

## Open

- Stage the `procedural` component, activate, start the native serve, run the battery, fix reds.
- Confirm the generator reproduces the hand-edited catalog files byte for byte.

## Left running

`repo run semio-hub-procedural-component:component -c prototype --exclude-dependencies` (door pid 96971, log `🗑️generated/native-component-procedural-3.txt`) is still waiting for the shared `.cache/b/d` lease behind `tools setup all` (pid 17436). When it finishes, the component is staged; then activate and start the native serve as above.
