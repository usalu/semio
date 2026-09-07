# Explore: producing a stdio descriptor from the existing served module

Read-only investigation, 2026-09-06. Builds on `📓️explore-stdio-wasm-link.md` (stdio's own standalone
component cannot currently be linked — wasmparser's 1,000,000-defined-function ceiling) and
`📓️explore-boot-with-stale-stdio.md` (stdio has never had a descriptor anywhere in git history; every
`dependsOn:["stdio"]` plugin 404s on `🔣️.json` at boot). This packet answers: is there a way to
produce a valid stdio descriptor pair from the already-materialized Aug-18 artifacts without a full
rebuild, and — separately — does raster genuinely need the stdio **actor** loaded at runtime at all.

## TL;DR

**There are two entirely separate, independently-coded descriptor-emission mechanisms in this repo**,
not one:

1. The **dev-script inline probe** (`describeBuiltPlugin`, dev `📜️script.ts:354-372`) — runs the
   **jco-transpiled JS** for a component under `node --experimental-wasm-jspi`, and calls the guest's
   own `describe.describe()` export. This is the ONLY mechanism that could theoretically run against
   the existing Aug-18 served stdio artifacts (`.js` + `.core.wasm` + `interfaces/`) with **no rebuild**,
   because jco's transpiled JS is self-sufficient to execute a component. But it still needs a
   never-retained **raw** (pre-transpile) `component.wasm` to compute `hashes.wasmSha256` — and that
   file no longer exists anywhere for stdio (deleted with every isolated diagnostic target dir).
2. The **canonical native pipeline** (`describePluginComponent` → `describe_component`,
   `🖨️describe/📦️packages/🦀️rust/🦀️.rs:429`) — used by every plugin crate's own `bun ./📜️script.ts
   describe` (raster's included) and by stdio's own `catalog-root` script. In production this executes
   the **raw** component via semio's **own** wasm interpreter (`execute_describe_owned` →
   `OwnedRuntime`, `🦀️.rs:406-421`) — **not** wasmtime, **not** jco/node. `wasmtime` exists in the same
   file only behind `#[cfg(test)]` as a third-party cross-check oracle (`execute_describe_wasmtime`,
   `🦀️.rs:388-403`, exercised by `configured_real_component_suite_owned_descriptors_match_wasmtime`) —
   exactly CLAUDE.md's "use third-party only to validate our own implementation" rule. This path always
   calls `buildPluginComponent` (`cargo rustc`, `🖨️describe/…/📜️script.ts:115-121`) first — there is no
   "describe an already-built artifact" *subcommand*, though a lower-level function
   (`emitOwnerDescriptorPairV1`, same file `:214-257`) does accept pre-built paths directly. Either way
   the **raw** component `.wasm` must exist and must be a real, executable stdio component — and it
   does not, for stdio, anywhere in this repo.

**Conclusion: neither mechanism can produce a stdio descriptor from what's on disk today.** The
raw/un-transpiled component `.wasm` is the one artifact `materializePlugin` deliberately never retains
(dev `📜️script.ts` comment, quoted in §1) and it is also the one artifact every isolated stdio build
attempt explicitly deleted on cleanup (`explore-stdio-wasm-link.md` §1). A descriptor requires either a
fresh raw component (i.e. a successful build) or nothing.

**A second, independent finding changes the shape of the problem**: raster's own crate never calls
stdio *as a separate actor* — every real codec path goes `raster's composite → stdio's in-repo
encode/decode fn`, called **in-process** as a linked Rust library (`📓️w3-raster-io.md`, quoted in §4).
`dependsOn`, however, is derived by a blind Cargo-dependency-line regex with zero distinction between
"linked library" and "needed as a live actor" (registry `📜️script.ts:406-416`), and the runtime graph
(`loadPluginModulesInDependencyOrder`, PluginRuntime `🟦️.tsx:2013-2045`) then genuinely **cascade-fails
raster itself** — not just stdio — the moment stdio's own `loadPluginModule` rejects. This is very
likely a false coupling: raster's registry `dependsOn` should probably not include stdio at all, since
nothing at runtime actually calls across to a separate stdio actor.

---

## 1. How is the descriptor pair produced? (mechanism, exact inputs)

### 1a. Dev-script path (`🧰️framework/…/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`)

- `stagePluginDescriptor` (`:226-238`) — pure file copy: `<ownerRoot>/🔣️.json` (+`🛂️.descriptor.semio`
  if present) → the served `🔌️plugin-modules/<id>/` dir. Deletes stale copies if the owner-root file is
  absent (`:229-231`). Builds nothing.
- `PLUGIN_DESCRIPTOR_PROBE_SOURCE` (`:326-333`) — a Node script string, `eval`'d via
  `node --experimental-wasm-jspi --input-type=module --eval "<source>" <componentModule>`
  (`describeBuiltPlugin`, `:354-364`). The probe: `import(pathToFileURL(componentModule))` (the
  **jco-transpiled `.js`**, which internally loads its sibling `.core.wasm` + `interfaces/`), asserts
  every required actor export exists (`assertActorComponentExports`, `:314-321`, against the fixture at
  `🧫️fixtures/🔣️.json`), then calls `component.describe.describe()` and writes the returned bytes as
  base64 to stdout.
- `describeBuiltPlugin(target, artifact, componentModule)` (`:354-364`): runs the probe above against
  `componentModule` (the transpiled `.js`), then **separately** computes two SHA-256 digests —
  `pluginFileDigest(artifact)` (the **raw pre-transpile** component `.wasm` from cargo's target dir) and
  `pluginFileDigest(componentModule.replace(/\.js$/, ".core.wasm"))` (the jco-extracted core). Only
  `finalizePluginDescriptor` (`:339-350`) patches these two hashes into the decoded descriptor and
  self-hashes `descriptorSha256`; it does **not** check `wasmSha256 !== coreWasmSha256` (no distinctness
  guard, unlike the native path — see §1b).
- Called from `materializePlugin` (`:387-402`) right after `transpilePluginComponentAsync` — i.e. this
  only ever runs immediately after a **fresh** `cargo rustc` in the same dev-boot invocation
  (`buildPluginCargo`, `:372-378`, is materializePlugin's only caller of the Cargo build). There is no
  "describe an existing served module" entry point at this layer.
- **Exact inputs needed to execute** `describe()` itself: only the jco-transpiled `.js` + its sibling
  `.core.wasm` + `interfaces/` — self-contained, since jco's JS glue reimplements the canonical ABI in
  JS around the raw core module. **Exact inputs needed for the full descriptor (hashes included)**: the
  above, **plus** the raw pre-transpile component `.wasm` (only ever used for its SHA-256, never
  executed by this path).

### 1b. Native pipeline (`🧰️framework/…/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts` +
`🦀️.rs`)

- `describePluginComponent(repoRoot, packageName, ownerRoot, rootCdylib, control)` (`📜️script.ts:566-581`)
  — the shared two-line-wrapper every plugin crate's own `describe` command calls (raster's
  `DescribeScript`, `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📜️script.ts:16-19`; stdio's own
  `DescribeScript`, `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:799-804`, passing
  `rootCdylib=true`). Always: `buildPluginComponent` (`cargo rustc`/`cargo build`, `:115-121`) →
  `extractPluginCore` (jco transpile, only to obtain the core module — `:124-135`) →
  `emitOwnerDescriptorPairV1` (`:214-257`).
- `emitOwnerDescriptorPairV1` (`:214-257`) is the lower-level function that takes **pre-built** paths
  directly (`rawComponentPath`, `extractedCorePath`, `ownerRoot`, optional `artifactRoot`) — it does
  *not* invoke Cargo itself. It hashes both files in-process (`emissionArtifact`, `:180-203`), throws if
  `raw.path === core.path` or `raw.sha256 === core.sha256` (`:222-223`, a **hard distinctness guard**
  absent from the dev-script path), then execs the native `semio-framework-plugin-describe` binary
  (`ensureBuiltBin`/`runCmdStatus`, `:227-230`) as `describe <raw> --core <core> --out <staging>`,
  verifies the emitted pair (`verifyDescriptorPairBytesV1`, imported from the registry script), and
  atomically publishes into `ownerRoot`.
- `describe_component` (Rust, `🦀️.rs:420-449`): reads+hashes both `wasm_path` (raw) and
  `core_wasm_path` (core, hash only), then `execute_describe_owned(&wasm_bytes, wasm_path)`
  (`:406-421`) — **this is what actually runs the component**: `OwnedRuntime::new()` (semio's own
  in-repo wasm interpreter, not wasmtime, not V8), `.compile(&package, wasm_bytes)`, then
  `.describe_observed(&compiled, Budget{fuel:…, deadline_ms:…}, …)`. `core_wasm_path`'s bytes are
  **read only for their SHA-256** (`read_artifact`, called at `:424` for wasm, `:425` for core) — the
  core module is **never executed** on this path.
- `execute_describe_wasmtime` (`🦀️.rs:388-403`) is `#[cfg(test)]`-gated (compiled out of the production
  binary entirely) — a wasmtime-component-based instantiation used **exclusively** as a third-party
  cross-check oracle inside the test suite, confirming the owned interpreter's `describe()` output
  matches wasmtime's. This is the concrete instance of CLAUDE.md's "use a third-party library only to
  validate our own implementation" rule for this feature.
- **Exact inputs**: the raw component `.wasm` (executed, and hashed), the extracted core `.wasm`
  (hashed only, never executed), an empty `out_dir`. **No jco `.js`, no `interfaces/`, no Node
  involved anywhere in this path** — it is a pure native Rust CLI tool.

### 1c. Registry-side `dependsOn` derivation (answers part of Q4 too)

`parseCargoPluginDependencyIds` (registry `📜️script.ts:406-416`) is a **plain regex over the Cargo.toml
text** — two patterns: `package = "semio-s-plugin-<id>"` (renamed deps) and bare
`semio-s-plugin-<id> = ` (direct deps) — with zero AST parsing and zero distinction between "this crate
links `<id>` as an `rlib`/library" and "this crate needs `<id>`'s own actor to be loaded at runtime".
`dependsOn` (`parsePluginCargo`, `:280-281`) is exactly this list (plus the extension `extends` host
prepended). `dependencies` on the rendered registry row (`:934`, `:1893`) is
`entry.dependsOn.map(id => ({pluginId: id, version: "*"}))` — i.e. the runtime graph edge is a direct,
unfiltered copy of the Cargo dependency-line scan.

---

## 2. Can describe be run against the EXISTING served stdio module without rebuilding?

**No — not with a valid result, for either mechanism, because the one artifact both mechanisms need to
*execute* (the raw pre-transpile component `.wasm`) does not exist anywhere for stdio.**

- The dev-script path (§1a) is the only one that could in principle skip a rebuild, since its `describe()`
  *execution* only needs the already-served `.js`+`.core.wasm`+`interfaces/` (Aug-18, still on disk at
  five locations per `explore-stdio-wasm-link.md` §4). The exact bun/node invocation a coordinator could
  attempt (spelled out from `describeBuiltPlugin`'s own construction, dev `📜️script.ts:354-372`):
  ```
  node --experimental-wasm-jspi --input-type=module --eval "<PLUGIN_DESCRIPTOR_PROBE_SOURCE>" \
    "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🗄️stdio/semio_s_plugin_stdio_component.js"
  ```
  (`PLUGIN_DESCRIPTOR_PROBE_SOURCE` itself would need to be reconstructed from `:326-333`, or the whole
  function called directly from a bun script that imports `describeBuiltPlugin` — it is not exported,
  so a coordinator would need to either export it locally or duplicate the ~15-line probe). This WOULD
  produce real `manifest.pluginId`/`capabilities`/etc from the genuine guest `describe()` call (the
  Aug-18 build is a real, working stdio build — nothing about *running* it is broken). **But**
  `describeBuiltPlugin`'s second half needs `artifact` — the **raw** component `.wasm` cargo produced on
  2026-08-18 — to compute `hashes.wasmSha256`, and that file was never retained
  (`materializePlugin`'s own doc comment, dev `📜️script.ts:387-393`: *"plugin-modules never receives a
  copy of the full component `.wasm`... shipping the untranspiled component alongside it was pure
  duplicate ~60MB-class weight"*) and no isolated target dir survives either
  (`explore-stdio-wasm-link.md` §1's `find` came up empty for any `target-stdio*` dir). **There is
  nothing to hash.**
- A workaround a coordinator might consider: pass the served `.core.wasm` itself as both `artifact` and
  the core path (reusing one file for both digests). The dev-script path's `finalizePluginDescriptor`
  (`:339-350`) has **no distinctness check**, so this would not throw there — but it produces a
  descriptor whose `hashes.wasmSha256` is fabricated (equal to `coreWasmSha256`, not a real raw-component
  hash), which is exactly the kind of self-consistent-but-untrue state `emitOwnerDescriptorPairV1`'s
  explicit `raw.sha256 === core.sha256` guard (native path, `🦀️.rs`-adjacent `📜️script.ts:222-223`) and
  stdio's own `verifyIndependentOracles` (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:249`:
  *"stdio descriptor substituted raw component bytes for independently extracted core bytes"*) are
  specifically designed to reject. The dev-script path lacks this guard, so it is technically possible
  to produce a syntactically-valid but semantically-fraudulent descriptor this way — **not
  recommended**, flagged here only because the ticket asked whether it's possible.
- The native path (§1b) cannot be aimed at the existing artifacts at all: `describePluginComponent`
  always calls `buildPluginComponent` (a fresh `cargo rustc`) first, and even the lower-level
  `emitOwnerDescriptorPairV1` requires a real raw component that isn't merely present but also
  **executable and distinct from the core** — the served core.wasm is not itself a valid component (jco
  strips the component wrapping to inline it into JS), so pointing the native emitter at it would fail
  at `wasmtime::component::Component::from_binary`/`OwnedRuntime::compile` (whichever produces the
  eventual error) long before reaching `describe()`.

**Registry hash gate, if a descriptor were produced anyway**: `validateDescriptors` (registry
`📜️script.ts:2085-2135`, the `check`/`check-generated` gate) only checks `entry.hashes.wasmSha256`
against a **real file on disk** if one exists at the canonical publication path,
`publicationWasmPath()` = `target/wasm32-wasip2/wasm-release/<wasmOut>` (`:2008-2019`, hardcoded —
does **not** honor a `CARGO_TARGET_DIR` override, so an isolated build wouldn't even be checked against
this path). For stdio, this path currently does not exist at all (no `target-stdio*` dir survives). So:
- **No canonical wasm-release artifact present** (today's actual state) → **WARNING only**:
  *"has hashes.wasmSha256 but no canonical wasm-release publication artifact at … — publication identity
  remains unverified"* (`:2127`) — **not fail-closed**.
- **A canonical artifact IS later built and its real hash differs** from whatever was recorded (e.g. the
  fabricated hash from the workaround above, or a genuinely stale one) → **hard ERROR**: *"hashes.wasmSha256
  is X but … actually hashes to Y — re-run `describe` after the latest build"* (`:2130-2131`).

So the answer to "stale-schema warning vs fail-closed error" is: **it's a warning today** (because
nothing at the canonical path exists to contradict the recorded hash), and would only become a hard
error the moment someone actually produces a real wasm-release stdio artifact whose hash disagrees.

**377 MB instantiation risk (unverified, not run — task forbids running `describe`)**: the dev-script
probe uses `node --experimental-wasm-jspi`, not bun, and does not instantiate the raw 377 MB component —
it loads the **transpiled JS + core.wasm**, whose size should be comparable (jco extraction doesn't
meaningfully shrink the core). `--max-memory=536870912` (`.cargo/config.toml`, per
`explore-stdio-wasm-link.md` §3) is a **link-time rustflag baked into the module's own declared memory
maximum** (512 MiB), not a Node/V8 runtime flag — V8's default per-instance wasm memory ceiling is well
above that, so the module's own 512 MiB cap should be well within Node's default limits. The bigger
unverified risk is **compile time/JS-heap pressure from parsing a ~377 MB `.core.wasm` module** inside a
single `node --eval` process — no ticket note anyworehere records anyone actually having tried this, and
this exploration did not run it (explicitly out of scope: "do NOT run describe").

---

## 3. Kernel behavior after `plugin.descriptor-unavailable` for a dependency

- `fetchDescriptorManifest` (`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:107-125`): `GET
  <dirname(moduleUrl)>/🔣️.json`; any non-OK response throws `SemioFaultError` code
  `plugin.descriptor-unavailable` (`:118`, `retryable: true`). Confirmed live at that exact line.
- `expandPluginRegistry` (`:300-320`) computes the browser session's full load set: primary + consumes/
  contributes matches + the **transitive `dependencies` closure** (`:311-320`, walking `entry.dependencies`
  — populated straight from `dependsOn`, §1c). Doc comment at `:307-309`, verbatim: *"consumes/
  contributes alone never pulls Cargo/`dependsOn` plugins (stdio, flow, cad, …)... an empty usable load
  order"* — i.e. stdio is unconditionally pulled into the load set for any `dependsOn:["stdio"]`
  primary, confirming the earlier ticket's finding.
- `validatePluginDependencyGraph` (`:3108-3123`) only reports `transaction.dependency-missing` if a
  dependency **id has no matching node in the graph at all** — stdio DOES have a registry node (role
  plugin, `dependsOn: []`, no `hashes`/`executionMode`), so this specific static check does **not** fire
  for raster→stdio. The failure is purely at the runtime **load** stage, not the static graph-shape
  stage.
- **The actual fail-soft mechanism, confirmed live**: `loadPluginModulesInDependencyOrder`
  (`🧱️elements/🔌️PluginRuntime/🟦️.tsx:2013-2045`, doc `:1978-2011`) — levels entries by dependency depth
  (`computeDependencyLevels`, `:1953-1963`), loads each level with bounded concurrency
  (`runBounded`/`poolConcurrency`), and on **any** `loadModule` rejection (network/activation failure,
  which includes a 404'd descriptor) adds that pluginId to `failedPluginIds` and records a
  `PluginLoadFailure` (`:2032-2037`). **Critically**, at the START of each subsequent level, every entry
  whose `dependencies` includes an already-failed id is itself **cascade-skipped** — added to
  `failedPluginIds`/`loadFailures` **without ever being attempted** (`:2020-2026`: *"a blockedDependency
  ... `${entry.pluginId} skipped — dependency ${blockedDependency.pluginId} failed to load`"*). This
  never throws / never aborts the whole `Promise.all` — the function's own doc (`:1997-1999`) states
  explicitly: *"Two independent failure classes, BOTH fail-soft (never abort the whole boot)"*.
- **Consequence for raster specifically**: raster's own registry row has `stdio` in its
  `dependencies` (Cargo-derived, §1c/§4). If stdio's `loadPluginModule` call rejects (404 on its
  `🔣️.json`), **raster itself gets cascade-skipped** — it is not merely "stdio's own actor fails while
  raster still renders." Raster would never receive a `PluginWasmHandle` at all in this call, matching
  procedural's first-hand boot-8 observation quoted in `explore-boot-with-stale-stdio.md` §1
  (*"stdio / flow-extension-draw / flow-extension-bim fail to load... `PluginRuntime: turn failed for
  actor procedural#1` → 'No plugins loaded'"*). The earlier framing in this ticket's CONTEXT ("the
  primary plugin still becomes a live actor... fail-soft") describes a **different** scenario — lowpoly,
  whose Cargo dependency on stdio produced the SAME `dependsOn` edge, but whose own build happened to
  reach a fresh descriptor for **itself** and never had stdio's OWN separate actor load fail on the same
  run in a way that mattered — not a general guarantee that a primary with a failed dependency still
  renders. **Not independently re-run against raster in this exploration** (read-only scope, no `dev`
  boot performed) — this is inference from the exact code path plus one directly-matching sibling
  observation, not a fresh reproduction.
- **`negotiate_wire_format`/`s.stdio.*` io-router routes**: `negotiate_wire_format` only exists in the
  native host Rust (`🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs`) — no browser/kernel TS call site found.
  The `s.stdio.*` dialect/`ownerOf`/`ioIdentify` machinery in `🎠️kernel/🟦️.ts` (lines 835-836, 3495-3608,
  test-only fixtures) models a **generic cross-actor artifact-conversion routing graph** where a route
  can be "owned" by a specific plugin actor (e.g. stdio owns `s.stdio.binary@raw → s.stdio.gif@87a`).
  **This is a different mechanism from raster's own IO**: per §4 below, raster's actual codec calls are
  all in-process Rust function calls into the linked-in stdio library, never a cross-actor `ioIdentify`/
  router hop. Whether *some other* runtime path (unrelated to raster) unconditionally invokes this router
  in a way that needs stdio's actor specifically was not exhaustively traced — flagged as unverified.

---

## 4. Is `dependsOn` really just Cargo path dependencies? Does raster need the stdio actor at runtime?

**Yes, confirmed** (§1c): `parseCargoPluginDependencyIds` (registry `📜️script.ts:406-416`) is a bare
regex over Cargo.toml text with no distinction between a linked-library dependency and an
actor-runtime dependency. Any plugin whose Cargo.toml names `semio-s-plugin-stdio` (however it uses it)
gets `stdio` in `dependsOn` → `dependencies` → the runtime graph edge in §3.

**Does raster need stdio's actor at runtime, or only its library code?** Per `📓️w3-raster-io.md`
(same ticket, §1, quoted verbatim): every one of raster's real format hops is **an in-process Rust call
into stdio's own codec functions**, e.g. *"composite → stdio's `SemioImageToPng` → stdio's
`encode_png`"*, *"stdio's `decode_bmp` → stdio's `s.stdio.bmp` → `semio/v1/image` deserializer"*. This is
raster's own compiled component calling functions from the `semio-s-plugin-stdio` **rlib** it links at
build time (Cargo.toml `path = "..."`, `package = "semio-s-plugin-stdio"`,
`✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml:28` per `explore-boot-with-stale-stdio.md` §3) —
**not** a cross-actor RPC to a separately-running stdio `PluginWasmHandle`. Nothing in raster's own real
codec paths documented in `w3-raster-io.md` names an `ioIdentify`/router hop or a cross-actor message —
every conversion bottoms out in a function call compiled directly into raster's own linked core module.

**Conclusion: raster's genuine runtime need for stdio is exactly zero at the actor level.** The
`dependsOn: ["stdio"]` edge that (per §3) can cascade-fail raster's own boot the moment stdio's actor
fails to load is very likely a **false coupling** introduced purely because the registry generator
cannot distinguish "I link your rlib" from "I need you running as a peer actor". This is the single
most actionable finding in this packet: if raster's own registry `dependsOn` did not include `stdio` (or
if `expandPluginRegistry`/`loadPluginModulesInDependencyOrder` only pulled in Cargo-derived dependencies
that are ALSO declared as genuine runtime actor needs, e.g. via `consumes`/`contributes`/activation
events, rather than every raw Cargo dependency line), raster would very plausibly boot and render clean
today, independent of whether stdio ever gets a descriptor. **This exploration did not verify this by
actually booting raster with a hand-edited registry row** (read-only scope) — it is a strong, well-cited
inference, not a confirmed fix.

One nuance not fully chased down (flagged unverified): whether `svg` import's *"requires the native
semio-framework-os host"* path (`w3-raster-io.md` §1) or any other raster IO route uses the generic
`s.stdio.*` `ioIdentify`/router graph (§3) rather than a direct in-process call — if any single raster
route did route through that graph specifically naming `stdio` as the owning actor, that one route (not
raster's boot as a whole) would still need a live stdio actor. Not exhaustively grepped for time reasons.

---

## 5. Cheapest path to a valid stdio descriptor pair (or to raster booting without one)

**Path A — remove the false dependency (cheapest, but requires a source edit, out of this agent's
read-only scope; the highest-leverage recommendation from this whole packet)**: per §4, raster's actual
runtime need for stdio is zero at the actor level. If raster's registry entry did not carry `stdio` in
`dependsOn`/`dependencies` (e.g. because the registry generator learned to only propagate Cargo
dependencies that also declare a genuine runtime relationship, or because raster's Cargo.toml dependency
were expressed in a way the current blind regex doesn't pick up as a `dependsOn` edge), raster would stop
being cascade-blocked by stdio's absent descriptor entirely — `expandPluginRegistry`/
`loadPluginModulesInDependencyOrder` would simply never try to load stdio as a peer for raster's session.
**This does not touch stdio's own problem at all** — it only unblocks raster. **Unverified**: this
exploration did not attempt this edit or a subsequent boot; it is a design conclusion from the code paths
cited in §3/§4, not a tested fix.

**Path B — produce a real stdio descriptor (addresses the root problem, not proven to succeed)**:
stdio already has a purpose-built, already-coded pipeline for exactly this
(`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts`, `CatalogRootScript`, `:851-931`):
```
cd ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust
bun ./📜️script.ts catalog-root --build-root <absolute empty fresh directory, not target/, not the dev cache>
```
This does, in order: `cargo rustc -p semio-s-plugin-stdio --profile wasm-release --lib --crate-type
cdylib --target wasm32-wasip2` (an isolated `CARGO_TARGET_DIR`, `RUSTC_WRAPPER=""`) → jco
transpile+extract-core → jco WIT-export inspection → build+run the native
`semio-framework-plugin-describe` emitter (§1b's `describe_component`/`OwnedRuntime` path) →
`verifyIndependentOracles` (WebCrypto+Pack self-checks, including the same `assertComponentizableCore`
1,000,000-function pre-check, `:238`, that would surface the real linker-time function count if the
build gets that far) → publish the descriptor pair atomically to stdio's owner root → re-run
`plugin-registry:generate` + `:check-generated` → a strict `createFreshCatalogBuildVerifier` re-check.
On any failure it rolls the owner-root descriptor back to its prior (currently absent) state.

**This script is already what every prior stdio diagnostic effectively (re)implemented by hand** — and
it has its own hardcoded `CATALOG_DEADLINE_MS = 1_200_000` (20 minutes, `:20`). Per
`.../🌙️09/☀️05/S-END-TO-END/📓️stdio-check-census.md` (cited in `explore-boot-with-stale-stdio.md` §1),
a plain `cargo check` (type-check only, cheaper than `cargo rustc`+link) already took **86m52s** on the
current source tree — more than 4× this script's own budget. **Running `catalog-root` as it exists
today should be expected to self-cancel** (*"stdio catalog-root exceeded 1200000ms deadline"*) well
before `cargo rustc` even finishes compiling, let alone before reaching the linker step that has never
once succeeded since 2026-08-18 per `explore-stdio-wasm-link.md`. Raising `CATALOG_DEADLINE_MS` would be
a source edit (out of this read-only agent's scope) and is not itself claimed sufficient — the deeper,
still-unresolved question (per `explore-stdio-wasm-link.md`'s conclusion) is whether stdio's linked core
can be componentized under the 1,000,000-function ceiling **at all** on the current 8.2 GB/4,104-file
`🗿️artifacts` tree, independent of how much time is allowed.

**Recommended order for a coordinator**: attempt Path A first (cheap, no build, directly informed by
§4's finding that raster doesn't need stdio's actor) to get raster itself rendering; treat Path B as the
separate, expensive, still-unproven stdio-specific effort that the already-open
`COMPLETE-SEMIO-END-TO-END`/`SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS...` tickets are the
proper home for, not something to re-attempt ad hoc inside a raster-focused ticket.

**Stated plainly, what is unverified in this report**:
- Neither Path A nor Path B was actually executed (read-only agent scope: no builds, no `describe`, no
  boot). Both are code-cited design conclusions, not reproduced outcomes.
- Whether `PLUGIN_DESCRIPTOR_PROBE_SOURCE`'s `node --experimental-wasm-jspi` process can actually
  instantiate a 377 MB jco-transpiled module without hitting a Node/V8 resource limit was not tested.
- Whether any raster IO route (beyond the ones enumerated in `w3-raster-io.md`) routes through the
  generic `s.stdio.*` `ioIdentify` cross-actor graph rather than an in-process call was not exhaustively
  checked.
- `catalog-root`'s 20-minute self-cancellation is inferred from the S-END-TO-END census's `cargo check`
  timing on a *possibly slightly different* tree state (same day, but not the identical commit) — not a
  fresh timed rerun of `catalog-root` itself.

## Files referenced

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:226-238,314-402`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:73-121,137-257`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/🦀️.rs:388-449`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:280-330,406-416,934,1893,2008-2135,3190`
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:107-125,300-320,3095-3135,3235-3305,835-836,3495-3608`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1933-2045`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📜️script.ts:1-22`
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:20-22,190-238,780-954`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/📓️w3-raster-io.md` (§1, format-by-format table)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/📓️explore-stdio-wasm-link.md` (this ticket, read first)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/📓️explore-boot-with-stale-stdio.md` (this ticket, read first)
