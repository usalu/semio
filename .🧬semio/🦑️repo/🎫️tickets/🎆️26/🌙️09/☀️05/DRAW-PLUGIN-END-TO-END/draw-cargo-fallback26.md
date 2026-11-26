# Draw Cargo Component Diagnostic Fallback

The Draw component is the `semio-hub-draw-component` cdylib. On the pinned toolchain, `wasm32-wasip2` emits the linked WebAssembly component directly. The repository Cargo config supplies its guest stack and memory limits. The previously completed direct Cargo build produced the component header, and the installed emitter described those bytes successfully. This fallback requires no separate adapter or componentization command.

From the repository root, run the ticket capture after the current component pipeline has stopped and the build owner authorizes this fallback.

```sh
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"
G="$T/🗑️generated"
repo tools exec --light python -- "$T/draw-cargo-component26.py" build cargo-draw26
```

The helper's `plan` action prints the exact subprocess arguments without compiling. The `build` action supplies `SEMIO_OFFLOAD=never`, `SEMIO_BUILD_MODE=prototype` and `SEMIO_COMPILER_PARALLELISM=1` and invokes:

```sh
repo capacity run --memory=6144 --cpu=1 --jobs=1 -- \
  repo cargo --store=shared --configuration=prototype --parallelism=1 --lang=en -- \
  build --manifest-path "🌎️hub/🧩️compositions/🖍️draw/🧩️component/📦️packages/🦀️rust/Cargo.toml" \
  -p semio-hub-draw-component --lib --target wasm32-wasip2 --profile prototype \
  --locked --offline --color never --message-format=json-render-diagnostics
```

The Cargo ingress acquires the repository compiler store, selects the workspace and profile configuration, admits the Cargo graph and its sources, and starts the managed scheduler. Its one-token compiler rewrites the front-end thread count and adds serial back-end policy. This avoids a manual rustc process or rebuilding the prerequisite descriptor emitter. The `shared` path borrows a private lane from the same pool as the component driver and shares the content-addressed unit cache. An existing component in matching lane `paefc3c` has the correct component header and dates to November 25 at 21:08. That file can serve only if a new successful Cargo invocation actually reports it as its exact selected artifact, including an admitted `fresh: true` hit. The lane file alone cannot establish current inputs.

The capture writes `cargo.jsonl`, `cargo.log`, `component.wasm` and `artifact.json` to a new ticket output directory. It requires a zero process exit and the final successful Cargo completion following one exact compiler-artifact row for the owned manifest, target and non-test profile. It opens only the declared managed prototype WASI output, checks that its component header, file identity and byte extent hold through the copy, and records the unmodified compiler-artifact row and observed SHA-256. A failed attempt, ambiguous row or core Wasm file produces no receipt. The streaming CLI releases the lane before the ticket helper opens the output, so this capture depends on the single compiler pipeline and is diagnostic. The repository launcher's retained `output: "capture"` consumer can preserve lane custody through result extraction if a future owner needs artifact publication authority.

## Diagnostic descriptor

Run the registered standalone describe owner after the capture completes. A normal product publication still uses the component driver.

```sh
B="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
SEMIO_OFFLOAD=never repo tools exec --light bun -- \
  "$T/draw-native-dev.ts" "$G" describe-cargo26 \
  "$B/🖨️describe/📦️packages/🦀️rust/📜️script.ts" \
  describe "$PWD/$G/cargo-draw26/component.wasm" --out "$PWD/$G/cargo-draw26/descriptor"
repo tools exec --light bun -- "$T/verify-cargo-draw26.ts" "$G/cargo-draw26"
```

Standalone `describe` reads the installed same-mode emitter through the existing managed reader, verifies the real executable and complete original compiler evidence, and uses it to describe the capture. Its verified historical emitter is suitable for this diagnostic. The follow-up rehashes the retained component against the compiler-artifact receipt, verifies the canonical JSON and Pack pair and checks the Draw scenario oracle. The ticket receipt describes what Cargo emitted. It creates no repository artifact publication or attestation of current emitter sources.

The command was checked with the registered CLI help and its plan-only action. Capture checks passed for an exact artifact, a retry ending in success, missing or failed completion, ambiguous rows, out-of-lane output, and core Wasm refusal. Bun independently matched the captured fixture SHA-256. The verifier passed the receipt check against a small component fixture and validated the retained historical Draw descriptor pair with the scenario oracle. Changing one component byte made the verifier refuse the receipt. The current warm lane file's header is `0061736d0d000100`. The fallback did not run.

## Live fallback

The workspace-current driver completed four consecutive 584-unit descriptor-emitter Cargo invocations. During the last receipt capture it detected a changed framework value-retirement collection input and began a fifth emitter pass. It had yet to enter the Draw component stage. Its ticket-owned child was stopped through its signal handler and the driver recorded that it released its leases. The diagnostic compile was submitted separately with the same 6,144 MB ceiling and one CPU. The broker initially queued it to preserve the 2,048 MB host floor alongside an independently admitted graph plan.

## Registered publication selected

The diagnostic was cancelled while waiting for capacity, before Cargo emitted any output. The registered prototype driver now accepts an explicitly identified published descriptor emitter. Its 13 focused checks passed, including retained tool selection and independent schema verification. The canonical Draw build was submitted with the authenticated binary, compiler receipt and file index under the same single-CPU 6,144 MB reservation. The registered native transpiler resolved its installed Darwin executable and reported no rebuild required. Product publication still requires an actual successful driver completion.
