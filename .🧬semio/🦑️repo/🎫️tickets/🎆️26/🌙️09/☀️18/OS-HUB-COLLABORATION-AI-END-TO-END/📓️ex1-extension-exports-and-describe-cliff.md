# EX1 — `extension_exports!` owned ABI + the describe cliff (example bodies as assets)

Slice owner: EX1, session 5c (2026-09-20 16:18 →).
Spec: `📓️a3-descriptor-regeneration.md` §3 / §3b, `📓️ds1-stdio-descriptor-bound.md` §9 item 5 + §10.10.

---

## 0. Headline — measured only

1. **`extension_exports!`'s single-argument arm now emits the nine `semio_owned_*_v1` core exports,
   by construction.** The nine were factored into ONE shared inner macro
   `__semio_owned_core_exports!` (`🔌️plugin/🦀️.rs:38559`) invoked by both owners — no copy. A
   native law asserts the three owner entry points reach the *same* symbol set and that the set is
   exactly `OwnedSemioExport::ALL`, and refuses any second definition of them.
   **Law run: `1 passed`** (`🗑️generated/ex1-owned-export-law.txt`).
2. **The law is not vacuous.** Against `HEAD` (pre-fix) the single-argument arm contains **zero**
   `semio_owned_` mentions and the repo has **zero** `__semio_owned_core_exports!` invocations, so
   the arm assertion would have fired (`🗑️generated/ex1-prefix-arm-proof.txt`).
3. **The describe cliff is now bounded by construction.** A descriptor no longer inlines an example
   document body over 256 KiB: it keeps the row and declares the body as an `AssetDeclaration`
   (name + mediaType + sizeBytes + sha256). Landed in the **guest's own** describe path
   (`🛂️describe/🦀️.rs`) — not the emitter — because `descriptor_is_fresh` byte-compares the
   committed pack against exactly what `describe_plugin()` returns. **Three laws run: `3 passed`**
   (`🗑️generated/ex1-example-asset-law.txt`).
4. **The threshold is measured, not guessed, and its blast radius is one package.** Census over all
   46 committed descriptors (`🗑️generated/ex1-example-body-census.txt`): exactly **one** example row
   in the whole tree is over 256 KiB — `🧩️puzzle`'s `capsule-dream`, **3 577 295 B = 84.9 %** of a
   4 803 294 B descriptor. The next largest body anywhere is `🌍️gis`'s 118 153 B. So the split
   takes `🧩️puzzle` from 4 803 294 B to **≈ 1 226 000 B**, under the 4 MiB bound, and changes **no
   other descriptor** and **no plugin in `DESCRIPTOR_MIGRATED_PLUGINS`**.
5. **One pre-existing red fixed en route**: `semio-framework-plugin-describe`'s test target did not
   compile at HEAD — `actor_bindings::…::effects::DocumentWriteParams` was renamed
   `artifact-write-params` in the WIT and the `#[cfg(test)]` host binding was not followed
   (`🛂️descriptor-emission/🦀️.rs:163`). One token. Without it no law in that crate could run at all.
6. **What is NOT proven yet: the `wasm32-wasip2` build.** The fleet wasm mutex (preamble rule 27)
   has been held by `b3f` since 16:22 with `b3c` and `tc1` also ahead of this slice; EX1's queued
   `cargo check --target wasm32-wasip2` over two extensions + one plugin had not started when this
   report was written. **No extension is claimed to describe yet** (§4).

---

## 1. Inherited state

No EX1 captures existed (`🗑️generated/ex1-*` empty at 16:18) — this slice starts from A3's report, not
from a predecessor. Machine at 16:18: load 34, 1 `rustc`, 5 cargos, 51 GiB free on
`/System/Volumes/Data`, wasm mutex held by `b3f` since 16:22 with a 4-deep queue.

A3 §3b located the defect and deliberately did not land it ("it changes the exported ABI surface of
every extension component in the tree"). This slice lands it.

---

## 2. Item 1 — `extension_exports!` owned ABI

### 2.1 What was wrong, restated from the source

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` has two owner entry points:

| entry point | routed to | emitted the nine at HEAD? |
| --- | --- | --- |
| `plugin_exports!` | `__semio_plugin_actor_exports!` | yes |
| `extension_exports!($bundle_fn, $plugin_fn, $app)` | `__semio_plugin_actor_exports!` | yes |
| `extension_exports!($bundle_fn)` | `__semio_actor_exports!` **directly** | **no** |

`__semio_actor_exports!` emits only the four WIT guest impls (`reactor`, `jobs`, `checkpoint`,
`describe`) plus the component export anchor. The nine `semio_owned_*_v1` `extern "C"` symbols lived
**inside `__semio_plugin_actor_exports!`**, so an extension component carried none of them, and
`OwnedSemioArtifact::from_component` (`🧠️interpreter/🦀️.rs:624-631`) — which requires all nine with
exact core types — rejected every extension with `component has no core module implementing the
owned Semio actor ABI`.

### 2.2 The fix — one shared inner macro, no copy

New `__semio_owned_core_exports!($runtime, $ensure, $describe_bytes)` at `🔌️plugin/🦀️.rs:38559`
holds the nine bodies once, parameterised by exactly the three things that differ between the two
owners: the `component_persistent_local!` runtime slot, the idempotent install hook, and the
zero-argument descriptor encoder.

| call site | line | arguments |
| --- | ---: | --- |
| `__semio_plugin_actor_exports!` (serves `plugin_exports!` and the 3-arg extension arm) | `:38687` | `__SEMIO_PLUGIN_RUNTIME, __semio_ensure_plugin_runtime, __semio_describe_component` |
| `extension_exports!`'s single-argument arm | `:39088` | `__SEMIO_EXTENSION_RUNTIME, __semio_ensure_extension_runtime, __semio_describe_component` |

Two behavioural notes, both deliberate and both visible in the diff:
- `semio_owned_describe_v1` now calls the owner's own `__semio_describe_component()` rather than
  re-deriving `$describe(runtime)` inline. For a plugin these are the same expression by
  construction (`:38683-38686`); for an extension it is `describe::describe_extension()`, which
  takes no runtime — which is exactly why the shared macro is parameterised on the encoder and not
  on a `$describe:path` + runtime pair.
- `$ensure()` replaced the hard-coded `__semio_ensure_plugin_runtime()`; the extension's
  `__semio_ensure_extension_runtime` additionally runs `extension_activate()`, which is what an
  extension needs before its manifest is readable.

The one function the extension owner had never instantiated before is `reactor::poll_kernel` (its
WIT arm calls `reactor::poll`). Checked: `poll_kernel<PA: PluginApp + 'static>`
(`⚛️reactor/🔄️turn/🦀️.rs:462`) is generic exactly as `poll<PA: PluginApp + 'static>` is
(`⚛️reactor/🦀️.rs:1250`), and the extension runtime is a concrete `PluginRuntime<NoPluginApp>`, so
the substitution is type-identical to the one the WIT arm already performs. `checkpoint_now` /
`restore_now` / `jobs::{start,step,cancel}_job` were already called with that same runtime by
`__semio_actor_exports!`. This is reasoning about signatures, not a compile — §4 is the compile.

### 2.3 The law

`owned_core_exports_are_defined_once_and_invoked_by_both_owners`
(`🖨️describe/🧪️tests/🔬️unit/🦀️.rs`, run in `semio-framework-plugin-describe` because that is the
crate that mounts `interpreter::OwnedSemioExport`). It `include_str!`s the SDK source and asserts:

1. `__semio_owned_core_exports!`'s body defines exactly the nine names in
   `OwnedSemioExport::ALL.map(core_name)` — the list the interpreter actually demands, not a
   restatement of it;
2. the **whole SDK file** defines those nine `extern "C" fn`s exactly once — a copy into the
   extension arm would satisfy a symbol-set assertion while leaving two bodies free to drift, so the
   law refuses one;
3. `plugin_exports!`, `extension_exports!`'s 3-argument arm and `extension_exports!`'s
   single-argument arm each *reach* a `__semio_owned_core_exports!` invocation (directly or through
   `__semio_plugin_actor_exports!`), and the export set all three reach is **identical**.

```
running 1 test
test tests::owned_core_exports_are_defined_once_and_invoked_by_both_owners ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out
```

Non-vacuity, measured against `git show HEAD:` (`🗑️generated/ex1-prefix-arm-proof.txt`):

| at HEAD | count |
| --- | ---: |
| `extern "C" fn semio_owned_` definitions in the SDK file | 9 (all inside `__semio_plugin_actor_exports!`) |
| `__semio_owned_core_exports!` invocations anywhere | **0** |
| `semio_owned_` mentions inside the single-argument `extension_exports!` arm (line 38898 →) | **0** |

### 2.4 The 25 extension crates whose ABI this changes — REBUILD LIST

Measured by `grep -rn "extension_exports!(bundle);" ✏️s` (2026-09-20): **exactly 25** crates use the
single-argument arm and therefore gain nine exported symbols. **A component built before this change
is ABI-stale and must be rebuilt before it is loaded again.**

| host plugin | extension crates |
| --- | --- |
| `🌊️flow` (9) | `semio-s-plugin-flow-extension-{bim,brep,dictionary,draw,list,logic,math,primitive,text}` |
| `📜️imperative` (5) | `semio-s-plugin-imperative-{control,effect,logic,math,text}` |
| `🏭️process` (4) | `semio-s-plugin-process-{concrete,metal,robotic,wood}` |
| `📐️cad` (4) | `semio-s-plugin-cad-{aec-building,aec-building-energy,aec-building-structure,spatial-shape}` |
| `🪵️sourcing` (3) | `semio-s-plugin-sourcing-{beams,slabs,windows}` |

**Correction to A3 §3b's blast-radius table**: `semio-s-plugin-playbook-procedural` is the ONE
extension in the tree on the **three-argument** arm
(`📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs:752`,
`extension_exports!(module_extension_bundle, module_plugin_bundle, ProceduralModuleApps)`), which
already routed through `__semio_plugin_actor_exports!` and already emitted the nine. Its `NotFound`
descriptor therefore has a **different cause**, still unattributed, and this fix does not close it.
A3's "16 of the 29 skips" is **15** on this defect. `🌊️flow` has 9 extension crates, not 10
(`🧫️fixtures` is a fixture directory, not a crate).

Exact rebuild command, one crate at a time, through the fleet mutex (preamble rule 27):

```sh
CARGO_PROFILE_WASM_DEV_DEBUG=false \
zsh ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/📜️wasm-build-mutex.sh" <slice> -- \
  cargo build -p <crate> --target wasm32-wasip2 --profile wasm-dev
```

and the re-describe that follows is `📜️a3-describe.sh <owner …>` (A3's driver, unchanged).

Plugin crates are **not** ABI-affected: `__semio_plugin_actor_exports!` emitted the nine before and
emits the same nine now, from the shared body — the law's clause 3 is what pins that.

---

## 3. Item 2 — the describe cliff: example bodies as referenced assets

### 3.1 The measurement that chose the design

`🐍️ex1-example-body-census.ts` (ticket folder) over every committed `🔣️.json`, 46 owners
(`🗑️generated/ex1-example-body-census.txt`, threshold 256 KiB):

| owner | json bytes | examples | inlined body bytes | body share | largest single body | rows over 256 KiB |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `🧩️puzzle` | 4 803 294 | 7 | 4 077 796 | **84.9 %** | **3 577 295** | **1** |
| `🔋️energy` | 471 401 | 15 | 123 776 | 26.3 % | 8 672 | 0 |
| `🌍️gis` | 1 077 748 | 2 | 118 574 | 11.0 % | 118 153 | 0 |
| `💠️lowpoly` | 333 382 | 1 | 60 498 | 18.1 % | 60 498 | 0 |
| `🖍️draw` | 222 599 | 1 | 31 188 | 14.0 % | 31 188 | 0 |
| every other owner | — | — | ≤ 30 046 | ≤ 16 % | ≤ 28 556 | 0 |
| **total** | | | | | | **1** |

Two populations, not a spectrum: the largest legitimate inline body is 118 153 B, and the one that
breaks the bound is 30× larger. **256 KiB sits 2.2× above the first and 14× below the second.**
Consequences, all arithmetic off the census:

- `🧩️puzzle`'s `🔣️.json` goes **4 803 294 → ≈ 1 226 000 B**, comfortably under the 4 MiB bound.
- **No other descriptor in the tree changes at all**, so nothing that A3 re-described goes stale.
- No plugin in `DESCRIPTOR_MIGRATED_PLUGINS` (`note, sequence, vcs, forms, sourcing, dag,
  mathematical, writer, reasoning, animate, draw, energy, layout`) has a body over the ceiling —
  its largest is `draw`'s 31 188 B — so **no `descriptor_is_fresh` test turns red.**

### 3.2 Where the fix had to go, and why it is NOT the emitter

DS1 §10.10 and the brief both suggest the emitter. It cannot be the emitter:
`__semio_plugin_descriptor_fresh_test!` (`🧪️tests/🧬️generated-test-contracts/🦀️.rs:15`) byte-compares
the committed `🛂️.descriptor.semio` against exactly what `describe_plugin()` returns, with hashes
blanked. An emitter-side split would make every committed descriptor differ from its own guest
output, i.e. turn all 13 migrated plugins red.

So the split lives in the guest describe path:
`🔌️plugin/🛂️describe/🦀️.rs` — `DESCRIPTOR_INLINE_EXAMPLE_MAX_BYTES` (256 KiB),
`asset_name_segment`, `externalize_oversized_example_bodies`, called from `plugin_descriptor()`
before the `PackageDescriptor` is assembled. The row keeps its id, label, icon and dialect, so every
picker predicate (`examples_for_dialect` / `examplesForDialect`) still resolves it; only
`artifact_json` is emptied, and one `AssetDeclaration` takes its place:

```
name      📚️examples/<artifactKind>.<standard>.<subset>/<id>.json   (non `[0-9A-Za-z._-]` → '-')
mediaType the owning app's own AppIo.artifact_media_type            (the example IS a document of it)
sizeBytes body.len()
sha256    sha256(body)
```

`AssetDeclaration` had **no producer anywhere in the tree** before this (`grep "AssetDeclaration {"`
returns only its own definition) — it was a modelled-but-unused shape, exactly as DS1 §9 item 5
described it.

### 3.3 The laws

`🛂️describe/🧪️tests/🔬️example-assets/🦀️.rs`, three of them, run in `semio-framework-plugin`:

```
running 3 tests
test component::describe::example_asset_tests::the_descriptor_bound_this_law_measures_against_is_the_declared_one ... ok
test component::describe::example_asset_tests::a_body_at_the_inline_ceiling_is_left_alone ... ok
test component::describe::example_asset_tests::a_multi_megabyte_example_body_leaves_the_descriptor_as_a_referenced_asset ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 812 filtered out
```

- the bound law `include_str!`s `📇️directory/🧬️schema/🦀️.rs` and asserts
  `DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES` is still `4 * 1024 * 1024`, so the other two can
  never measure against a stale ceiling;
- the split law uses `🧩️puzzle`'s **measured** 3 577 295 B body: inline it consumes more than three
  quarters of the whole descriptor budget on its own (which is how puzzle went over); externalized,
  the same manifest is under an eighth of it, the row survives, `artifact_json` is empty, and the
  declaration carries the exact `size_bytes` and `sha256` of the body it replaced;
- the ceiling law pins that a body *at* 256 KiB is left completely alone.

### 3.4 The one reader that had to move

`auditNavbarExampleArtifactPayload` (`📇️registry/✅️catalog-verification/🟦️.ts:33`) demanded
non-empty `artifactJson` on every example row of every committed descriptor, so it would have
flagged `🧩️puzzle`'s row the moment puzzle is re-described. It now admits a row that declares its
body as an asset instead, via a new TS twin `exampleBodyAssetName` in `🛂️manifest/🟦️.ts` (next to
`examplesForDialect`, the same twin convention), and its caller
(`📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts:532`) now passes `descriptor.assets`.

Exercised directly (`🐍️ex1-example-asset-audit.ts` → `🗑️generated/ex1-example-asset-audit.txt`):

```
committed descriptors audited: 32; problems: 0
asset name: 📚️examples/s.puzzle.5d.1.-/capsule-dream.json
externalized row WITH the declaration -> []
externalized row WITHOUT any declaration -> ["puzzle: example capsule-dream has neither inline artifactJson nor a declared example-body asset"]
```

i.e. the relaxed law is silent on every descriptor in the tree today (no regression), accepts an
asset-referenced row, and still rejects a row with no body at all.

**No product runtime reader was touched, because none reads example bodies out of a descriptor**:
the React `ShellHost` (`exampleArtifactSources`) and the wgpu shell (`plugin.manifest.examples`)
both read the **live** plugin handle's manifest, where the body is still inline; the MCP capability
catalog (`🌉️mcp/🗂️catalog/🦀️.rs:899`) reads only `example.id`.

### 3.5 The `wasm-release` profile question — decided by measurement, answered NO

DS1 §10.10 item 5 offers building the describe guest at `wasm-release` as the alternative fix.
Rejected, for a reason that is in the source rather than in a preference:
`DESCRIBE_ARTIFACT_MAX_BYTES`'s own docstring
(`🛂️descriptor-emission/🦀️.rs:252-257`) states the emitter reads the **unoptimised** artifact
deliberately, and its 256 MiB bound is sized for it. Switching `describePluginComponent`
(`🏗️component-build/🟦️.ts:56`) to `wasm-release` would trade a 30-minute guest run for a 60-minute
release build **per plugin** and change what that bound is sized against. The brief's own
instruction — "if a release component already exists on disk for the plugin, prefer it; do not force
60-min release builds" — is the right shape, but it is a `📜️script.ts` change whose verification is
a describe sweep, and the §3 split removes the reason to make it: with `capsule-dream` out of the
descriptor, the guest no longer serialises 3.5 MB through the DSL value encoder and the pack writer.
**Not landed, and now not needed for the bound.** See §7 gap 3 for what it does *not* fix.

The brief's narrower variant — "if a release component already exists on disk for the plugin, prefer
it" — was measured rather than assumed (`🗑️generated/ex1-release-component-census.txt`, 16:50):

| `wasm32-wasip2/wasm-release` component on disk | bytes | built |
| --- | ---: | --- |
| `semio_s_plugin_demonstrator.wasm` | 44 513 865 | 09-17 02:38 |
| **`semio_s_plugin_puzzle.wasm`** | **24 424 679** | **09-17 19:49** |
| `semio_s_plugin_stdio.wasm` (DS1's private target dir) | 47 496 132 | 09-20 07:30 |
| `semio_s_plugin_gis.wasm` (DS1's private target dir) | 47 272 589 | 09-20 08:32 |

Four of 30-odd plugins, and exactly the one that matters — `🧩️puzzle` — is there. But it is **three
days old**, and there is no `wasm-dev` puzzle component in the shared tree at all (32 `wasm-dev`
plugin components, puzzle not among them). So "prefer the release component if it exists" as written
would describe **stale source** silently: the rule the describe path would actually need is "prefer
it when it is newer than the crate's own inputs", which is a freshness question `describe` does not
model today and which `descriptor_is_fresh` cannot catch (it compares the descriptor to the guest,
not the guest to the source). Recorded as measured, not landed.

---

## 4. Item 3 — runtime proof through the fleet mutex

**Not reached. Nothing here is claimed.**

`📜️ex1-wasm-check.sh` (ticket folder) was queued on the fleet wasm mutex at **16:26:46** and had not
acquired it by the end of this slice: `b3f` held the lock from 16:22:21 with `b3c` (15:37) and `tc1`
(16:20) also ahead of EX1 in the FIFO.

The holder was diagnosed rather than assumed (preamble rules 14 / 27b, read-only — nothing of a
peer's was touched): holder pid 69925 is `📜️wasm-build-mutex.sh b3f -- bun nx run
@semio-tech/framework-os-dev:activate-demonstrator-react-dev`, elapsed 1 h 16 m, with a live child
(pid 2240, 29 m elapsed) — i.e. **progressing, not deadlocked**, with only 2 `rustc` and 9 cargos on
the machine at load 38. There was nothing to kill and nothing to escalate; EX1 stayed in the FIFO
and spent the wait on the non-cargo half of the slice.

The capture is `🗑️generated/ex1-wasm-check.txt`. The job was detached (pid 5406, started by this
slice) and, since a detached child cannot outlive the slice's turn, was **stopped by pid and
dequeued at 17:04** rather than left as a phantom ticket at the head of a FIFO other slices sleep
on. Nothing of a peer's was touched. It checks, in order and one cargo at a time:

```
cargo check -p semio-s-plugin-cad-spatial-shape --target wasm32-wasip2 --profile wasm-dev
cargo check -p semio-s-plugin-imperative-text   --target wasm32-wasip2 --profile wasm-dev
cargo check -p semio-s-plugin-mathematical      --target wasm32-wasip2 --profile wasm-dev
```

— the two cheapest extensions (the two owners A1 and A3 measured failing) plus one plugin as the
regression side. **Rerun, and it is the next owner's first command:**
`zsh ".🧬semio/…/📜️ex1-wasm-check.sh"` (optionally with an explicit crate list).

What IS verified natively, with warnings as proof of real expansion (memory: "Require Warnings As
Proof Of Type-Check"):

| command | result |
| --- | --- |
| `cargo check -p semio-framework-plugin --lib` | `Finished dev in 53.08s`, 0 errors, **38 warnings** |
| `cargo check -p semio-framework-plugin --lib --tests` | 0 errors, **332 warnings** |
| `cargo test -p semio-framework-plugin --lib example_asset_tests` | **3 passed** |
| `cargo test -p semio-framework-plugin-describe --lib owned_core_exports` | **1 passed** |
| `bun 🐍️ex1-example-asset-audit.ts` (the changed TS law over all committed descriptors) | 32 audited, **0 problems** |

Native builds never expand the `#[cfg(all(target_arch = "wasm32", target_env = "p2"))]` items, so
**the macro's expansion is proven by the source law, not by a compiler** until the queued wasm check
lands. That is the honest state.

---

## 5. Catalog health, before → after

**Not re-measured.** `🐍️m5a-descriptor-census.ts` and the `semio-os-mcp audit` baseline (A3 §4.3: 22
diagnostics per registry load, 38 decodable descriptors) read **committed** descriptors, and this
slice regenerated none — a regeneration needs the wasm mutex (§4). Re-running either script now
would reproduce A3's 15:48 numbers byte for byte and would be a measurement of A3's work, not of
this slice's. The numbers move only after the 25 extensions of §2.4 are rebuilt and re-described.

---

## 6. Files changed

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | new `__semio_owned_core_exports!` (`:38539-38672`) holding the nine owned core exports once; `__semio_plugin_actor_exports!` now invokes it (`:38687`) instead of defining them inline; `extension_exports!`'s single-argument arm invokes it too (`:39088`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs` | `DESCRIPTOR_INLINE_EXAMPLE_MAX_BYTES`, `asset_name_segment`, `externalize_oversized_example_bodies`; `plugin_descriptor()` runs the split and passes the grown `assets` into the `PackageDescriptor`; new `#[cfg(test)] mod example_asset_tests` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🧪️tests/🔬️example-assets/🦀️.rs` | **new** — the three descriptor-bound laws |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🔬️unit/🦀️.rs` | **new law** `owned_core_exports_are_defined_once_and_invoked_by_both_owners` + its three source helpers |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs` | pre-existing red: `DocumentWriteParams` → `ArtifactWriteParams` (`:163`), the WIT's committed rename |
| `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` | new `exampleBodyAssetName` (+ private `assetNameSegment`), TS twin of the Rust asset-name rule |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts` | `auditNavbarExampleArtifactPayload` takes the descriptor's `assets` and admits an asset-referenced example row |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts` | passes `descriptor.assets`; test title follows |

New files in the ticket folder: `📜️ex1-wasm-check.sh`, `🐍️ex1-example-body-census.ts`,
`🐍️ex1-example-asset-audit.ts`, this report.
Captures: `🗑️generated/ex1-{example-body-census,owned-export-law,example-asset-law,example-asset-audit,prefix-arm-proof,release-component-census,wasm-check}.txt`.

---

## 7. Honest gaps

1. **No `wasm32-wasip2` compile of the changed macro yet** (§4). The queued mutex job had three
   slices ahead of it. Until it lands, "every extension is describable" is proven *by construction
   and by a source law*, not by a build — and certainly not by a `describe`. Outcome 4 (zero catalog
   skips) is **not** met by this slice; it is unblocked by it.
2. **No extension was described and no descriptor was regenerated**, so the catalog numbers are
   still A3's (§5). The 25 rebuilds in §2.4 are the next owner's first job, and they are what turns
   15 of the 22 remaining skips — A3 said 16; `playbook-module-procedural` is not one of them
   (§2.4's correction) and needs its own diagnosis.
3. **The split does not fix `🧩️puzzle`'s `describe` wall clock, only its size.** A3 §3 located a
   second cost: `capsule-dream`'s `ExampleSource` *parses* 3 035 200 B of DSL and re-serialises it to
   JSON inside the guest at **bundle install** (`🖐️5d/…/🌙️capsule-dream/🦀️.rs:18-39`,
   `LazyLock`), which `describe` triggers before it ever builds a descriptor. Externalizing the body
   removes the DSL-value encode and the pack write of 3.5 MB, not the parse. Whether that is enough
   to bring puzzle inside `DESCRIBE_DEADLINE_MS` is **unmeasured** — it needs one run through the
   mutex. If it is not, the remaining fix is a lazy `ExampleSource` (the body stays bytes until a
   consumer asks), not a raised deadline.
4. **The body does not yet travel as a file.** The descriptor now *references* it with an exact
   size and sha256, which is what bounds the descriptor, but nothing writes
   `📚️examples/…/capsule-dream.json` to disk next to it: the guest cannot write, and the emitter
   never sees the body (by §3.2 it must not). Materialising it needs either a tenth owned export or
   an emitter-side asset channel — both ABI-scale, both out of this slice. Until then an
   asset-referenced example is resolvable at runtime (the live manifest still carries the body) but
   not from the descriptor alone.
5. **The TS twin can drift.** `exampleBodyAssetName` restates the Rust naming rule in TypeScript
   with no shared fixture pinning them (`examplesForDialect` has one:
   `🧫️fixtures/📚️example-picker.json`). One fixture over an externalized example would close it.
6. **Nothing was proven against a live MCP client or a running shell.** Same gap A3 §6.5 records;
   this slice is framework source, two laws and a census.
