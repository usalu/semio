# 🌐️ Booting `s` in a browser — what the runtime actually rejected

Compiling `s` was never the whole job. Once the storybook could build and the story could load, the
plugin's own runtime rejected it three times in a row, each for a different reason, and each only
visible by actually running it. Recording them because none is discoverable from a compile.

## 1. Unclassified interactive commands (a panic, not a diagnostic)

```
panicked at 🔌️plugin/🦀️.rs:5750:
app-definition.interactive-job-classification: unclassified interactive command
  'framework.window.table:foldDirectoryEvents'; … 's.space.space@1/*#editor:presenceHeartbeat'
```

`validate_interactive_job_classification` refuses a catalog containing any `Unclassified` action, and
`InteractiveJobClassification`'s **default IS `Unclassified`** — so every action needs an explicit
disposition. The space-index editor declared none. Two non-obvious details cost a build cycle each:

- **Ordering matters.** `.action_interactive_job(id, …)` only rewrites actions **already registered**
  on the builder (`self.actions.iter_mut().find(…)`). Placing the block before the `.view_action(…)`
  calls silently classified nothing for those two. The sibling `🏠️home` editor puts its block last
  for exactly this reason.
- **It does not reach window kinds.** The same commands are re-exposed as
  `framework.window.table:<id>`, which live in `self.window_kinds`, and `action_interactive_job`
  never touches those. `.interactive_jobs(classification)` is the blanket that walks
  `self.actions.iter_mut().chain(self.window_kinds…)`. The fix is the `🌍️gis` editors' idiom: a
  blanket `.interactive_jobs(Migrated)` first, then per-action calls to refine.

## 2. Guest stack size — `memory access out of bounds`

With the classifications fixed the plugin still died, now silently:

```
RuntimeError: unreachable            (first turn)
RuntimeError: memory access out of bounds   (every turn after)
```

Cause: I had been building with `cargo build --lib`, but the dev pipeline builds plugins with
`cargo rustc … -- -C link-arg=-zstack-size=8388608` (`pluginCargoArgs`). Without it the component
links with the default **1 MB** guest stack; `s`'s boot overruns it. Rebuilding with the 8 MB stack
cleared both errors — the console went completely clean.

This is worth remembering generally: a plugin built with plain `cargo build` **loads and then faults
at runtime**. It is not interchangeable with what the pipeline produces.

## 3. A materialized module is more than the transpile

`materializePlugin` does five things, and doing only the first produces a module that loads the
*previous* component with no error at all:

1. `transpilePluginComponentAsync` — jco transpile + async-result lifting + asset-URL rewriting +
   core-module optimization + preview2 shim imports.
2. `hostShimSource()` → `🟨️.js`.
3. `describeBuiltPlugin` — **executes** the component's `describe` export under Node's JSPI engine.
4. `stagePluginDescriptor` — copies the owner-root descriptor into the module dir.
5. `pluginComponentBridgeSource(...)` → `<wasmOut>.js`, the entry the browser actually imports.

`🏗️transpile-s-from-artifact.ts` in this folder does 1, 2 and 5 by reusing the pipeline's own
exported functions, which is enough to iterate without a rebuild. Steps 3–4 still need the pipeline,
because the descriptor is produced by running the component and is hash-bound to the artifact.

## 4. The 20-minute build budget is the real obstacle, and it has a knob

`buildPluginCargo` runs under `buildBudgetMs()`. The shared `target/` is permanently contended by the
other sessions, so cargo spends the whole budget `Blocking waiting for file lock on build directory`
and is killed with `spawnSync cargo ETIMEDOUT` — the build never even starts, and the transpile never
runs. `SEMIO_BUILD_BUDGET_MS` overrides it (`🦑️repo/…/🟦️.ts:1233`); `SEMIO_BUILD_BUDGET_MS=5400000`
lets the pipeline outlast a peer's build instead of dying at 20 minutes. Warming the shared target dir
first with the identical `cargo rustc` command also makes the pipeline's own cargo step a no-op.
