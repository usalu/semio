# Final summary

Three capabilities were asked for: **mutations must be able to call other mutations**, **plugins can depend on
other plugins**, and **plugins can register mutations and inferences on artifacts defined by other plugins** —
end to end, with the two-tier extensibility (plugin tier + extension tier) preserved and unified underneath.

## The mechanisms, and why they are shaped this way

**A composite mutation is a pure plan.** `CompositeMutationKind::plan(&self, base, planner)` calls other
mutations through a shared `Planner`; `diff` and `inverse` are *folded* from that plan by `fold_plan_diff` /
`fold_plan_inverse` rather than handwritten. A composite therefore cannot drift from the steps it composes,
and replay re-plans deterministically exactly as `diff` always did — the event log and the mutation laws are
untouched. The delegation is emitted by `#[derive(CompositeMutation)]`, not a blanket impl, because a blanket
impl is rejected by coherence against the ~200 concrete `MutationKind` impls already in the tree.

**Anything touching a second artifact is host-driven.** A guest never blocks inside a mutation waiting on
another plugin: it returns a *proposal*, and the host runs a two-phase transaction over the existing
`exchange` channel — prepare every member, then commit in reverse discovery order, compensating on failure.
This is the only design that works identically in both hosts, since WIT imports are synchronous, the browser
host runs one worker per plugin, and the wasmtime host already holds the caller's `Store` mutex.

**Contribution is routing, not ownership.** An artifact's mutation enum stays closed at the type level;
openness lives in a host `ArtifactMutationRouter`, mirroring the `ArtifactInferenceRouter` that already
existed. Contributed steps are recorded in the owner's log as owner ops carrying
`MutationOrigin::Contributed{…}`, so replay and undo never need the contributor loaded.

**Both extensibility tiers got the same machinery.** `.depends_on` / `.contributes` exist on `PluginBuilder`
*and* `ExtensionBundle`; both worlds export the same new `contributor` WIT interface. The pilot deliberately
proves the *extension* tier, since that was the weaker half.

## What is demonstrably true

- os-kernel **909/0**, framework **137/0**, plugin host **40/0**, TypeScript **292/2**.
- A shipped artifact has a real composite: flow's `👯️duplicate-widget` plans `create-widget` then
  `connect-widgets` over one `Planner`, the second step reading the base the first advanced.
- A shipped extension really contributes across a plugin boundary: `aec-building` declares
  `.depends_on("cad", "^0.1.0")` and contributes one composite mutation
  (`cad.scene#cad-extension-aec-building:create-building-storey`, planned from cad's own leaf kinds) plus one
  inference — both refused by the gates if the dependency is removed.
- Both hosts implement the same five components (graph, mutation router, contributor-aware inference router
  with a `depends_on` DAG, instance directory, transaction coordinator) against one frozen protocol.
- A composite mutation is first-class taxonomy — `🧬️mutations/<kind>/{🦠️mutation,🧩️plan}`, with owning a
  handwritten `🔺️diff`/`↩️inverse` beside a plan rejected as two competing sources of one semantics — enforced
  identically by the taxonomy validator, the root policy gate and the plugin registry.

## What is not

**The end-to-end composite transaction over real wasm.** Every `semio-s-plugin-*` crate depends on
`semio-s-plugin-stdio`, which was mid-restructure by ticket `26/08/16/FULL-STDIO-…` for this entire session and
still fails to compile. The pilots' code is landed and reviewed but their crate gates never ran, and the
transaction cycle is proven against a pure-Rust wire harness rather than a rebuilt guest component. This is
the one headline claim the ticket does not get to make; it needs a rerun of
`cargo test -p semio-s-plugin-flow` and `-p semio-s-plugin-cad-aec-building` plus a guest rebuild once stdio
is green.

Also open: the repo's `wasm` target for `semio-framework-os` fails on `getrandom` for
`wasm32-unknown-unknown`, which keeps one TypeScript workflow-parity test unrunnable.

## Defects found and fixed along the way

Several were pre-existing and unrelated to the feature, surfaced because barriers actually ran the gates:

1. **Rust and TypeScript disagreed on the app-channel wire version** (10 vs 8) with nothing in the tree
   reporting it. The `Hello`/`Welcome` goldens encoded the *live constant*, so every version bump silently
   rewrote its own expected bytes — two separate tickets had already broken and "fixed" them that way. Split
   into a codec golden (literal version) plus a shared cross-language version pin.
2. **A cross-language byte-identity guard had been asserting nothing** since a directory restructure, and
   fixing its path exposed the real defect behind it: the TypeScript presence codec never implemented the
   `interaction` section (bit 7) that Rust encodes — even though the Rust fixture's own comment says it exists
   so the TS twin would exercise it. Implemented in both directions; the test now round-trips a real Rust blob
   byte-for-byte.
3. **The demonstrator's dev server never loaded the plugins it depends on** — the registry filter only unioned
   topic-based `contributes`/`consumes`. It now closes the transitive `dependsOn` graph.
4. **Two rejection codes in the frozen taxonomy existed nowhere in code**, so "contributor not loaded" was
   reported as a permission error, pointing operators at a correct declaration.
5. **This ticket's own dependency-parity gate double-counted nested extensions**, making a plugin inherit its
   extension's dependencies.

## Files

Framework: `📡️spr/🎮️command`, `📡️spr/🧵️channel`, `📡️spr/📜️history`, `🗣️dsl/✨️derive`, `🏪️store` (+`🔄️sync`),
`🌿️vcs`, `🛂️manifest`, `🚪️io`-adjacent call sites, `🧩️extension`, `🔌️plugin` (+`🏗️builder`, `🖥️host`,
`📇️registry`, WIT world), `🎠️kernel`, `💻️os/🟦️component.ts`, `💻️os/🟦️backbone-worker.ts`, renderer boot and
`PluginRuntime`. Plugins: `🌊️flow` (composite + command), `📐️cad/🧩️extensions/🏢️aec-building` (dependency +
contributions). Infrastructure: `🔣️taxonomy.json`, `📚️library/🔍️discovery`, root `📜️script.ts`,
`.vscode/launch.json` + seed, new `💻️os/📦️packages/🟦️typescript` test target, new channel fixtures.
