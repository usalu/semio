# Puzzle 5D launch audit

The generated playground catalog (`✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json`, `puzzle5d` record) maps aliases `5d` and `puzzle 5d` to the `s.puzzle.puzzle5d@1/*#editor` app in the Hub puzzle component. React uses port 6014 and WGPU port 6114. Its development contribution (`✏️s/🧑‍💻dev/🎭️variants/🧩️puzzle/🧬️schema/🔣️.json`) selects the puzzle Vite config and browser entry. This is the editor playground, distinct from the Storybook scope on port 6010.

The timeline story at `✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/🎭️5d-timeline/🧪️.story.tsx` imports all three real DSL assets, parses them through `@semio-tech/puzzle-wasm`, and composes the 3D world and timeline. Its history is synthesized from part order and its selection and checkpoint actions update a story-local reducer. Thus its previously reported four passing browser checks verify parsing and rendering but do not establish editor commands, archive persistence, or history replay.

The existing `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🖐️5d-lifecycle/🟦️.ts` drives the React editor: duplicate and move a board part, select and transform it in the world, write a folder archive, load it in a fresh peer, reload and reattach, then undo and redo. The non-React branch explicitly reports the plan failure and returns; the scenario does not verify WGPU. The registered `@semio-tech/framework-os-dev:time-travel` target forwards arguments to the shared time-travel harness, which starts the admitted playground if necessary and defaults to English and German. Suggested execution, after refreshing the WASM package:

```sh
repo run @semio-tech/framework-os-dev:time-travel --lang=en -- --scenario '✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🖐️5d-lifecycle/🟦️.ts' --renderer react --out '<ticket>/🗑️generated/lifecycle'
```

The harness prepares the browser, starts or reuses the declared serve, emits an acceptance record and reports failures, uncaught page errors, and guest faults. The audit did not find a current lifecycle pass in the ticket. The separate large-example audit describes missing actual Capsule Dream dispatch, undo, cancellation, and teardown coverage; these are coverage gaps, not reproduced failures.

The earlier `closinv-sgp-5` report says it was stopped while repeating graph reads and does not report an exit. The later `closure-final.md` documents 96 Linux cases passed across split native runs, four Darwin-only skips, and explicitly leaves the standard project-graph entry point unverified. These results must not be treated as a successful original project-graph invocation.

## Baseline launch attempt

The registered target entered project-graph planning and was cancelled after about 20 seconds for the native investigation; this is not an observed target failure. Direct invocation of the script refused its missing original process envelope. The retained `editor-lifecycle.ts` instead issues a real bootstrap invocation and calls the same harness. With the generated specific catalog supplied through `SEMIO_PLUGIN_CATALOG`, browser pin verification passed, but the harness's activation subprocess refused `Finite original repository child authority required` under that bootstrap. Without the catalog it refused `A uniquely declared playground is required: puzzle5d`; the loader deliberately returns an empty catalog without the environment variable.

The actual puzzle Vite config, launched with `SEMIO_PLUGIN=puzzle5d` and `--mode prototype`, refused before listening because `.cache/b/p/m/browser/prototype/runtime/react/puzzle5d/activation/🔣️receipt.json` does not exist. The activation owner declares the full component, materialization, web support, fonts, WASM, session, preparation and activation closure. The Storybook WASM stage alone does not constitute that closure. No editor lifecycle verdict or failure of an editor interaction was observed.

The registered activation target was then started and remained in graph-slice reading. `repo leases status --lang=en` identified an exclusive `.cache/b/g` graph-image lease held by a child of the graph service, marked progressing and longer than usual; another graph reader was waiting for it. The status reported zero deadlock cycles and did not expose a project or tranche. The concurrent cache collector held a different image, so its completion alone does not release this graph lease. Nx and Bazel share the repository graph planning boundary, so selecting another registered backend does not by itself establish a route around this admission.

On November 21 at 21:06 CET, the repository lease census showed the resident service still holding its first lease for 35,112 seconds. Its provisioning database descriptor remained open and the kernel lock table identified the originating process. The canonical editor activation receipt is still missing. The separate Storybook verification completed with all four browser tests passing against the private interim WASM package.

The exact-owner Bazel dry run also remained at graph-service admission for more than four minutes. It was cancelled after confirming the wait. The resident service stayed active.

The Semio backend with `--graph-policy=current` and the exact-owner dry run also waited on the graph service. It was cancelled after more than three minutes. There was still no process listening on either registered editor port.

The subsequent live attempt used `repo --tool=semio run @semio-tech/framework-os-dev:activate-puzzle5d-react-prototype --lang=en --exclude-dependencies --graph-policy=current`. The current service was initially absent. Automatic startup waited until the four-minute command budget expired and released the owned command's leases on interrupt. The earlier resident process still held the exclusive SQLite lock on `nx-tooling-provision` at 21:51 CET. The attempt yielded no React editor activation receipt or browser lifecycle verdict.
