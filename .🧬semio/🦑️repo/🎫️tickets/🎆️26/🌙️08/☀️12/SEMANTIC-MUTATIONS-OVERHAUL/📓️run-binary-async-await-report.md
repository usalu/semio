# Run Binary Async Await Repair

Changed only `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs`: `persist_run` is async and awaits the async workflow seed, `ArtifactStore::new`, dispatch, and snapshot pack. Both parse calls, `RunSink` initialization, and both persist callers are awaited from `run_async`. `run` remains the sole `block_on` boundary. `SpaceBundle` reads and writes remain synchronous.

The ticket controller is [`🧪️run-binary-async/📜️script.ts`](./🧪️run-binary-async/📜️script.ts). It was run with `bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️run-binary-async/📜️script.ts'` and emitted `[DEBUG] run binary await contract callerCallees=9 syncSpaceBundle=3 sourceOnly=true`. There is no separate vector file and no retained command-output artifact: this repository invocation's output is the reported one-line result.

No pre-change failing regression was executed. The controller was authored and run only after the source correction, so it is a source-only regression guard—not compile/runtime readiness. No binary build or runtime test was run; runner build remains pending root scheduling.
