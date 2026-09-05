# Graph, Value, and UI Current-Byte Repair

## Outcome

Three framework frontiers exposed by the native-provider and public-directory builds are repaired in current bytes.

- Graph manifest discovery admits every permission-prefixed manifest filename by the canonical `🛂️…manifest.json` boundary. The migrated flow DAG is generated into both Rust and TypeScript catalogs and resolves through a declared Rust law.
- The first-party value derive now gives a missing bare `Option<T>` the same `None` behavior as serde for ordinary structs and named tagged-enum variants.
- The WGPU UI package mounts its canonical component source, uses exact layout-kind dispatch for both serde and the first-party value codec, faults render-credit overruns before packet publication, and exercises incremental retained ownership using the actual state-machine grants.
- The Unix PTY smoke continues draining the PTY after child exit until buffered output is observed or the fixed deadline expires.

## Ownership Corrections

- `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📜️script.ts`
  - discovery and generated `build.rs` watching use the same suffix-based permission-manifest rule.
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs`
  - declares the flow-DAG source-resolution law.
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs`
  - missing bare options default only when the final type-path segment is `Option`.
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/tests/🌾flatten-with-skip.rs`
  - compares plain-struct and named-variant behavior with serde JSON.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️🧩️component.rs`
  - root/child decoding accepts only `stack`, `row`, `column`, `horizontal`, or `vertical`; empty stacks remain stacks and unknown kinds fail closed in both codec paths.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️🟥️🍋️prepared.rs`
  - fixed owner exhaustion reports the process-permit boundary;
  - terminal tests drive one governed step at a time until a bounded terminal result;
  - tests generation-bind retained raster producers and preserve the independent page/source retirement grants;
  - completed packets and jobs are explicitly retired in the test law.
- `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🦀️.rs`
  - PTY output draining no longer treats child exit as proof that the PTY buffer is empty.

## Registered Evidence

- Graph generation: session 71763, exit 0, nine manifests.
- Graph generated freshness: exit 0.
- Graph quick: session 44901, 184/184 passed.
- Value derive full suite: session 54744, 32/32 passed.
- UI axes generation: session 22533, exit 0.
- UI axes freshness: session 74671, exit 0.
- UI quick, uncached, no fail-fast: session 71694 / Nextest `8d303ac9-9f01-420a-865d-b5f45956596b`, 235/235 passed.

## Honest Residuals

- The option detector deliberately has no alias resolution; a foreign final segment named `Option` remains a separate schema-taxonomy concern.
- The public window-layout structs still store terminal and axis kinds as strings. Decoder publication is fail-closed, but a typed in-memory discriminator/factory boundary remains a clean-up packet.
- This framework gate does not establish real hub boot, linked native catalog activation, checkpoint-to-tail composition, or AI inference over the hub.
