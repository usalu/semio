# Artifact verification audit

`🗑️generated/fix5d2-4.log` records three passing standalone 5D tests: two command-envelope round trips and SQLite interior cancellation across all four copy phases. `🗑️generated/hubown-9.log` records nine passing Hub surface tests. Both commands exited zero. The exact cached test executables recorded in those logs no longer exist, so this is an audit of prior results, not a test of the latest sources. Rebuilding would contend with active integration builds.

The earlier native WASM package parsed all three 5D examples and passed four Playwright tests as documented in `native-storybook.md` and `results.md`. After refreshing `semio_puzzle.js`, `semio_puzzle.d.ts`, `semio_puzzle_bg.wasm`, and `semio_puzzle_bg.wasm.d.ts` in `.cache/b/p/m/browser/prototype/wasm-packages/d30e56a055ea2d5bde08382c/`, start Storybook on port 6010 as documented in `native-storybook.md`, then run from repository root:

```sh
bun node_modules/playwright/cli.js test -c '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END/native-pw.config.ts' -g puzzle5d
```

The staged directory is not content-addressed. Ensure it contains the refreshed binary and bindings before treating the browser run as current.

After refresh, check the staged package directly from the repository root with:

```sh
node '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END/wasm-staged-node.mjs'
```

The script checks all three fixture counts. It has not been run against the refreshed package.

The existing published Hub package passed the current Node smoke after the cache cleanup. Node checked Concrete Forest, Nakagin and Capsule Dream at 1/0, 180/179 and 2,880/2,864 parts/fasteners respectively. A refreshed binary remains in progress.
