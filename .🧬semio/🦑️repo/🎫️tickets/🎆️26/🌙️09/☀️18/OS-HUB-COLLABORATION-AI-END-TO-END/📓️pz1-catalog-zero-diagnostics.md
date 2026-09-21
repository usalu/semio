# PZ1 — the semio MCP capability catalog at zero diagnostics, `client-e2e` 36/36

Slice owner: PZ1, session 7 (2026-09-21 03:39 →). Spec: `📓️ce1-client-e2e-pinning-and-puzzle-bound.md`
§1/§4/§7, `📓️a3b-descriptor-sweep.md` §3/§5, `📓️a3-descriptor-regeneration.md` §3,
`📓️ds1-stdio-descriptor-bound.md` §10, `📓️ex1-extension-exports-and-describe-cliff.md` §3.

Inherited state (CE1's close, 2026-09-21 00:19): catalog diagnostics **2** (`🧩️puzzle` 4 803 294 B
over the 4 MiB bound and undescribable; `🗄️stdio` no committed descriptor), `client-e2e` **34/36**.

Session note: this slice ran 03:39–04:10 (source work; cut by the fleet-7 outage, preamble rule 29)
and resumed 10:28. Between 04:03 and 10:28 a peer's `Mutation::label` → `LocalizedLabel` sweep left
`semio-framework-os-kernel`/`-os`/`-plugin` uncompilable, so **no wasm build was possible in the
first window** — every measurement below is from the second one. The source work of the first window
(§2) survived the outage intact and was re-verified green at 10:28 (`cargo check -p
semio-framework-plugin`, 41 warnings; `-p semio-s-artifact-puzzle-{2d,3d,5d}`, 1 warning).

## 0. Headline — measured only

1. **`🗄️stdio` has a committed descriptor for the first time** (§1): `describe` rc=0 in **2 816 s**,
   `🔣️.json` **1 504 747 B**, pack **446 903 B**, both produced by the plugin's own verb, no
   hand-edit. Catalog diagnostics **2 → 1**; descriptors the audit inspects **58 → 59**; the
   registry's own `✅️catalog-complete` goes **17/18 → 18/18** with its `descriptor-pair-missing`
   list now `[]`.
2. **The deferrable example body is landed end to end** (§2) — `ExampleSourceBody::Deferred`, a
   producer the leaf owns, and a descriptor `AssetDeclaration` computed from the AUTHORED bytes with
   no parse. It is landed WITHOUT changing `ExampleDefinition`'s wire shape, and §2.1 is the measured
   argument for why changing it would have taken diagnostics from 2 to ~59.
   Proven general by re-describing `🖨️raster` in the same hold: `🔣️.json` **158 884 → 158 884 B,
   byte-identical**.
3. **The accepted cause of `🧩️puzzle`'s undescribability is wrong, and the real one is located**
   (§2.3). With the bodies deferred the guest got FURTHER and still died on the 1 800 s epoch
   (5 872 116 097 fuel vs A3b's 4 081 418 370, at a 33 % higher fuel rate). Measuring the descriptor
   instead of guessing showed the package's real declarations are only ~375 KB — a quarter of
   `🗄️stdio`'s, which describes in 1 764 s. The remaining cliff is one line:
   `puzzle5d_part_kind_options()` (`🖐️5d/…/✏️editor/🦀️.rs:9651`) builds the "Add Part" select by
   dereferencing **all three** example documents, `capsule-dream` included — re-parsing the very body
   the deferral had just removed, **plus** a `serde_json` parse of the derived 3.5 MB into a typed
   document, on the `AppDefinition` path, which IS the describe path. Fixed, pinned by a law, and
   the option set gets better as a side effect: `capsule-dream` was contributing raw catalog UUIDs.
4. **`🀄️wfc`'s inference refusal is not a `🀄️wfc` defect** (§3). `job kind "semio.infer" has no
   admitted explicit bounded state machine` is a FRAMEWORK gap: `spawn_job`
   (`⚛️reactor/💼️jobs/🦀️.rs:368-372`) drops every builtin `JobFn` on the floor under
   `#[cfg(not(test))]`, so all five builtin kinds — `semio.infer`, `semio.io-run`,
   `semio.io-sniff`, `semio.mutation-plan`, `semio.migrate` — are refused in every production build,
   for every plugin. No plugin in the tree registers a bounded builtin kind and none could have.
   Two concrete designs, neither started speculatively.
5. **`client-e2e` 30/32** (§4.3), reds: the `puzzle` catalog diagnostic, and a NEW one this slice did
   not cause — `🗒️note`'s 09-20 component answering `instantiate: wasmtime: no exported instance
   name`, i.e. CE1 §8 gap 5 (the freshness oracle catches age, not ABI) arriving after today's
   tree-wide `Mutation::label` → `LocalizedLabel` sweep.
6. **A source sweep the `document_json() -> String` change owed**: 12 call sites across `🧩️puzzle`,
   `🖍️draw`, `🪵️sourcing`, `🀄️wfc` and `🌍️gis`, all re-checked natively `--all-targets` (§5).

## 1. `🗄️stdio` — the missing descriptor

`🗄️stdio` is the only registry plugin in the tree with no committed `🔣️.json`/`🛂️.descriptor.semio`
pair, so the MCP registry answers `NotFound` for it and the catalog health line counts it twice
(catalog-load + routing). DS1 §10 established that the descriptor FITS the 4 MiB bound at runtime;
nobody had ever run its `describe` to completion and committed the result.

Method: the producer's own verb, never a hand-edit — `bun ./📜️script.ts describe` in
`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust`, run through the fleet wasm mutex inside this slice's
batched hold (`📜️pz1-describe-batch.sh`, new — A3's per-owner recipe with ONE mutex acquisition for
the whole batch instead of one per owner). Capture `🗑️generated/pz1-describe-🗄️stdio.txt`, ledger
row in `🗑️generated/pz1-describe-ledger.txt`.

**Landed, rc=0** (11:19, ledger row `🗄️stdio rc=0 2816s json 0 -> 1504747 pack 0 -> 446903`):

| | |
| --- | ---: |
| wall | **2 816 s** (component build + jco core extraction + guest `describe`) |
| guest `describe` alone | **1 764 s**, ending at fuel **4 528 416 303** |
| `🔣️.json` | 0 → **1 504 747 B** (36 % of the 4 MiB bound) |
| `🛂️.descriptor.semio` | 0 → **446 903 B** |
| `wasm_sha256` | `bf3455822cf7a724…` |
| `descriptor_sha256` | `43f73b25628072c1…` |

**It finished 35 s inside the 1 800 s guest epoch** (`DESCRIBE_DEADLINE_MS`), at 57 % of the 8 G
fuel budget. That margin is the honest headline of this item: `🗄️stdio` is 36 artifacts in one
package and its `wasm-dev` `describe` is the second-most expensive in the tree. Nothing was tuned to
get it there — the deadline and the fuel cap are untouched — but a single new artifact under
`🗄️stdio` will push it over, and the next owner should expect to re-describe it on a quieter
machine. (DS1 §10.11 proved the same descriptor fits the bound built as `wasm-release`, which is what
the trusted bootstrap uses; `describePluginComponent` hard-codes `wasm-dev` on purpose,
`📓️a3-descriptor-regeneration.md` §3.)

## 2. `🧩️puzzle` — the deferrable example body

### 2.1 What the wire shape already was, and why it is NOT changed here

CE1 §4 framed this as a WIRE-SHAPE change to `ExampleDefinition` and concluded it needed a tree-wide
WIT/jco regeneration. Reading the whole path first changed that conclusion twice, and both readings
are measurements rather than opinions:

1. **`PluginManifest` is not in the WIT.** The only `artifact-json` in
   `🔌️plugin/🧬️schema/📜️.wit` (`:406`) belongs to `spawn-plugin-instance-params`. The manifest
   crosses the ABI as a *pack value* (`dsl::to_dsl_value` → `store::pack_rt::encode_wire_value`), so
   no per-component jco binding carries `ExampleDefinition`. Its only generated twin is
   `🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts:469`, written by `@semio-tech/framework:generate` from
   the Rust type — types only, no functions.
2. **The "inline string OR declared asset reference" body already exists** — EX1 landed it as
   `describe::externalize_oversized_example_bodies` (`🛂️describe/🦀️.rs:52`): the row keeps its
   identity, `artifact_json` goes empty, and the body leaves as an `AssetDeclaration` carrying its
   name, media type, exact `size_bytes` and `sha256`. `📇️registry/✅️catalog-verification/🟦️.ts` and
   `🎠️kernel/🟦️.ts:427` both already read a row that way.

So the missing half was never the descriptor's shape — it was that the guest **materialised the body
while the bundle was assembled**, before `describe()` built anything. Replacing
`ExampleDefinition.artifact_json: String` with a tagged enum would have made all 59 committed
descriptors undecodable at once (the `FromValue` derive has no default for a missing field), i.e. it
would have taken catalog diagnostics from 2 to ~59 and cost a full-tree re-describe — ~3 h of
serialized fleet-mutex time — to get back to where this slice started. **That is the opposite of
this slice's own goal, so the deferral is expressed at the SOURCE, and the descriptor keeps the
declaration channel it already had.** Named honestly as a deviation from the brief, not silently.

### 2.2 The deferrable body

`ExampleSource` (`🔌️plugin/🦀️.rs:8081 📚️ExampleSource`) now carries an `ExampleSourceBody`:

| variant | what it holds | when the body exists |
| --- | --- | --- |
| `Inline(String)` | the body itself | at bundle assembly, as before |
| `Deferred { produce: fn() -> String, source_suffix, source: &'static [u8] }` | a non-capturing producer plus the AUTHORED bytes | only when someone asks for the document |

- `ExampleSource::deferred(id, label, icon, source_suffix, source, produce)` — the new constructor.
- `document_json()`/`document()`/`payload()` return `String` and are the ONLY place a deferred body
  is produced.
- `deferred_body_asset(dialect, media_type)` answers the descriptor declaration from `source` alone:
  `size_bytes = source.len()`, `sha256 = sha256_hex(source)`. **No parse.** That is EX1's own
  phrasing ("body stays bytes, with its size/hash declared at the source") taken literally: what is
  declared is the DSL the author wrote, not the JSON derived from it, and the asset name carries the
  authored suffix (`…/capsule-dream.dsl.semio`) so the declaration never claims to hash bytes it did
  not see.
- `into_example_definition` leaves `artifact_json` empty for a deferred body — byte-identical to
  what an externalized oversized body already produces.
- `Plugin::register_app_factory` collects each deferred row's declaration (the one place the leaf
  and its dialect are both in hand) into `Plugin::deferred_example_assets`, and
  `plugin_runtime::plugin_descriptor_extras` merges them into `PackageDescriptor.assets`.

Consumers updated: `manifest::{asset_name_segment, example_body_asset_prefix, example_body_asset_name}`
are now the single naming authority (describe calls them instead of its own copy); the TS twin gained
`exampleBodyAssetPrefix` and `auditNavbarExampleArtifactPayload` accepts any asset under a row's body
prefix rather than one fixed `.json` name; the wgpu shell's example-graph scan skips body-less rows
instead of pushing `Value::String("")`.

`🧩️puzzle`: all seven DSL example leaves (2d/3d/5d × nakagin + concrete-forest, and 5d
capsule-dream) are deferred. `capsule-dream` is the measured one — 3 035 200 B of DSL parsed and
re-serialised to ~3.5 MB of JSON inside a `LazyLock` the subset's `examples()` dereferenced.

### 2.3 The measurement that overturns the accepted cause

`🧩️puzzle` was re-described with the deferral landed (11:19 → 11:53, capture
`🗑️generated/pz1-describe-🧩️puzzle.txt`). **It still died on the 1 800 s epoch**, and the two runs
side by side say why that is NOT a failure of the deferral:

| run | fuel at the wall | fuel rate | finished? |
| --- | ---: | ---: | --- |
| A3b, 09-20, eager example bodies | 4 081 418 370 @ 1 667 s | 2 448 fuel/ms | no |
| **PZ1, 09-21, deferred example bodies** | **5 872 116 097 @ 1 800 s** | **3 262 fuel/ms** | no |
| `🗄️stdio`, same hold, for scale | 4 528 416 303 @ 1 764 s | 2 568 fuel/ms | **yes** |

The rate went UP by a third — the deferral removed exactly the allocation-heavy work A3 §3 located
— but the run still did not finish. So **the `capsule-dream` DSL parse was never the whole cliff**,
and CE1 §4's open question ("if it is a minute, laziness is the whole answer") is answered: it is
not.

Then the descriptor itself was measured rather than guessed. Breaking the committed 4 803 294 B
`🔣️.json` down by key:

| key | bytes | share |
| --- | ---: | ---: |
| `manifest.examples` (7 rows) | **4 436 313** | **92.2 %** |
| `manifest.apps` (6) | 369 961 | 7.7 % |
| everything else | ~5 300 | 0.1 % |

i.e. the package's real declarations are **~375 KB — a quarter of `🗄️stdio`'s 1.5 MB**, and
`🗄️stdio` describes in 1 764 s. A package a quarter that size cannot honestly need more than 1 800 s
to emit. Something on the describe path was doing work the descriptor never shows — and it was:

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/…/✏️editor/🦀️.rs:9651` `puzzle5d_part_kind_options()`
builds the `partKind` select of the "Add Part" dialog by dereferencing **all three** shipped example
documents, `CAPSULE_DREAM_EXAMPLE_DOCUMENT` among them. That static is
`document_from_json(CAPSULE_DREAM_EXAMPLE_JSON)` — the DSL parse the deferral just removed, **plus**
a `serde_json` parse of the derived ~3.5 MB JSON into a typed `Puzzle5dDocument`. It runs inside
`create_puzzle5d_app()`, which the subset calls to build its `AppDefinition`, which is bundle
assembly — the describe path. **The example body came back in through a second door, and that door
costs more than the first one did.**

Two independent reasons to close it, both measured on the fixtures:

1. `capsule-dream`'s own `kindCatalogs.parts` table is **0 rows** (its catalog is a composed child,
   `kind-catalogs=child_id=kind-catalogs-ce8f60fe…`, absent from the standalone document), so
   `puzzle5d_part_kind_rows` falls through to inference over its **2 880** parts — whose `part-kind`
   column holds that child's raw UUIDs (`"0e240cd2-7f98-42b6-af39-34e7ee4fad35"`, …).
   `nakagin`'s 180 parts and `concrete-forest`'s hold names (`Base`, `Bridge`, `Capital`,
   `Tambour`, `Capsule With Balcony J`, …). The select was offering unpickable ids, inside a 64-row
   cap that they crowd real kinds out of.
2. It is the remaining describe cliff.

**Fix**: `puzzle5d_part_kind_options()` unions `concrete-forest` + `nakagin` only, with the reasoning
in its docstring, pinned by a new law
(`🖐️5d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` `part_kind_options_are_named_kinds_and_never_catalog_uuids`)
that refuses any option shaped like a UUID.

**Re-describe after the fix: NOT MEASURED — three mutex holds lost to peers, two of them to a
tree that did not compile.** The honest record, because it is the reason this item is open:

| attempt | outcome |
| --- | --- |
| 03:39–04:10 (session 7a) | never reached a build: a peer's `Mutation::label` → `LocalizedLabel` sweep left `semio-framework-os-kernel`/`-os`/`-plugin` uncompilable for the whole window |
| 12:01 → 13:39 (pid 14113) | waited **98 min** in the fleet queue behind `rb1`'s `build-s-react-release`, got the lock, and both owners died **4 s and 3 s** in: `cargo build -p semio-s-plugin-note --target wasm32-wasip2 … exited with status 101`, on a peer's in-flight `UiNodeRecord: Clone` break in `🖱️ui/🧬️contract/📃️document/🦀️.rs:310`. A whole hold spent on somebody else's compile error |
| 13:40 → (pid 65484, live) | relaunched as `📜️pz1-await-and-describe.sh` (new): it polls `cargo check -p semio-framework-plugin -p semio-framework-ui-contract` **outside** the lock and only takes the mutex once the tree is green — which it did at **13:47** (`🗑️generated/pz1-await.txt`). Queued since 13:47; `tc3c` has held the lock since 13:44 and was still holding it at 15:09 (holder alive, 9 `rustc` running — a legitimate long build, not the rule-27(b) deadlock shape) |

That script is the reusable part of this item: a wasm owner should never spend a fleet-mutex hold
discovering that the shared tree is red.

**Expected rows** (`🗑️generated/pz1-describe-ledger.txt`, written by the detached run):
`🗒️note rc=0` — clearing `client-e2e` red 2 (§4.3) — then `🧩️puzzle rc=0` with `🔣️.json` falling
from 4 803 294 B to roughly **375 KB**, since all seven example bodies now leave as declared assets
and §2.3's cliff is gone. Neither is claimed here: an unfinished `describe` discards its staging
directory, so both committed pairs are byte-for-byte what they were.

Whoever reads the ledger next: re-run `semio-os-mcp audit --folder <repo>` (expect **0**
`skipping plugin` lines) and `bun ./📜️script.ts client-e2e` from `🌉️mcp/📦️packages/🟦️typescript`.

## 3. `🀄️wfc` — the refusal is NOT a `🀄️wfc` declaration defect

The brief (following CE1 §8 gap 3) reads `job kind "semio.infer" has no admitted explicit bounded
state machine` as a declaration a plugin forgot to make, "the way other plugins with inference do".
**No plugin in the tree does it, and none can have done it — the refusal is a framework gap.**

Located, by reading `🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs` end to end:

| line | what it says |
| --- | --- |
| `:139` | there are TWO registries: `KIND_REGISTRY` (`JobFn`, async futures) and `BOUNDED_KIND_REGISTRY` (`BoundedJobFactory`) |
| `:145-152` | the five BUILTIN kinds — `semio.io-run`, `semio.io-sniff`, **`semio.infer`**, `semio.mutation-plan`, `semio.migrate` — are all registered in `KIND_REGISTRY`, i.e. as `JobFn`s |
| `:353-360` | `spawn_job` prefers `BOUNDED_KIND_REGISTRY`; only if that misses does it look at `KIND_REGISTRY` |
| `:368-372` | and there, under `#[cfg(not(test))]`, it **drops the `JobFn` on the floor** and parks the slot as `JobBody::ExplicitStateMachineRequired`. The opaque-future executor is `#[cfg(test)]` only. |
| `:464`, `:481` | `step_job` then answers exactly the fault the gate prints |

So in a PRODUCTION build every builtin job kind is refused, not just inference. A grep for
`register_bounded_job_kind` over the whole tree returns three registrations and none of them is a
builtin kind: `framework.reserved.tool` (`🔌️plugin/🦀️.rs:17596`), `🏗️fem`'s 2d and 3d mounted
visual jobs. `🀄️wfc` declares its inference correctly — five `ArtifactInferenceDescriptor`s and a
real `ToolJobFactory` per artifact (`🖼️bitmap/…/💡️inferences/🦀️.rs:685`
`BitmapInferenceJobFactory`, keyed `("semio.infer", "s.wfc.bitmap.solve")`) — and its guest is
refused before any of that is consulted.

**Not landed here, and deliberately not started.** The fix is one of two real pieces of work, both
outside what this slice could verify tonight (see §6):

1. *Framework*: give `semio.infer` a `BoundedJob` that drives the ActionBus route explicitly —
   essentially `run_interactive_inference` (`💡️infer/🦀️.rs:220-326`) with every `ctx.tick().await`
   turned into a `BoundedJob::step` boundary and the two inner close loops (`outcome.close_step`,
   `session.close_step`) lifted into their own phases. It closes the route for EVERY plugin at once.
2. *Plugin-local*: `🀄️wfc` registers its own bounded `semio.infer` that dispatches on
   `request.inference_schema` to the synchronous driver each artifact already ships
   (`🖼️bitmap/…/💡️inferences/🦀️.rs:735` `solve_with_job`, a `BatchJobSession` stepped to
   completion), sliced one `session.step()` per `BoundedJob::step`. That needs the framework's
   private `encode_result` (`💡️infer/🦀️.rs:347`) to become a public helper so a plugin can build a
   `WireArtifactInferenceResult`.

(1) is the root fix. Neither was attempted speculatively: an unverifiable half-landed change to the
job registry would break every plugin's job route, which is a worse outcome than a named red.

## 4. Gates

Instruments, all pre-existing so the rows compare with A3b's and CE1's:
`bun nx run @semio-tech/framework-os-mcp-rs:capability-audit-check` (the staged
`…/🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp`, restaged 10:31 by a peer — this slice changed
no gateway Rust, and the gateway reads descriptors from disk at run time, so no restage was owed),
`bun ./📜️script.ts client-e2e` from `🌉️mcp/📦️packages/🟦️typescript`, and the registry's own
`✅️catalog-complete` vitest.

### 4.1 Laws (run, not written)

| law | result |
| --- | --- |
| `semio-framework-plugin --lib example_source_tests` (3, one new) | **3 passed**, 0.01 s — incl. `a_deferred_example_declares_its_authored_bytes_without_producing_the_document`, whose producer `panic!`s, so the pass IS the proof nothing materialises a deferred body to answer the manifest or the descriptor |
| `semio-framework-plugin --lib example_asset_tests` (3, EX1's) | **3 passed**, 0.34 s — the describe-time split still holds after the naming helpers moved into `manifest` |
| `semio-s-artifact-puzzle-5d --features component-app-assembly part_kind_options_are_named…` (new) | **1 passed**, **0.07 s** — the wall clock is the second measurement: building the `partKind` select used to materialise a 3 MB DSL document and a 3.5 MB JSON parse |
| `@semio-tech/plugin-registry` `✅️catalog-complete` | **18 passed (was 17/18)** — the `descriptor-pair-missing` list is now `[]`; its stale 15-id expectation is replaced |
| repo `tsc --noEmit` | no diagnostics in any file this slice edited |
| `cargo check -p semio-framework-plugin` / `-p semio-s-artifact-puzzle-{2d,3d,5d}` | green, 41 / 1 warnings (warnings are the proof a type-check really ran) |

### 4.2 Catalog diagnostics — **2 → 1**

`semio-os-mcp audit --folder <repo>` (`🗑️generated/pz1-audit-after.txt`, 12:00):

| metric | CE1's close | PZ1 | delta |
| --- | ---: | ---: | ---: |
| `[mcp registry] skipping plugin …` (one registry load) | 2 | **1** | **−1** |
| descriptors the audit decodes and inspects | 58 | **59** | +1 |
| audit findings | 46 | 46 | 0 |

The one survivor is `puzzle` (§2.3). `stdio`'s `NotFound` is closed.

### 4.3 `client-e2e` — 30/32, and the new red is component AGE, not this slice

`🗑️generated/pz1-client-e2e.txt`, 12:04, from `🌉️mcp/📦️packages/🟦️typescript`:

```
FAIL os: capability catalog health — 2 diagnostic(s), first: skipping plugin `puzzle` …
FAIL os: artifact_create (a real plugin artifact kind) — kind=s.note.note:
     `note` refused ReadArtifact (channel.not-wired): instantiate: wasmtime: no exported instance name…
```

30 PASS / 2 FAIL. The denominator fell 36 → 32 because the journey `return`s at `artifact_create`,
exactly as it used to at `action_prepare` before CE1's pin.

Red 2 is **CE1 §8 gap 5 arriving**: `🗒️note`'s component is CE1's 22:25 build, its committed
descriptor was cut from that same build, so `verifyStagedPluginComponent` passes it — the oracle is
"the descriptor describes this build", not "this build matches today's host ABI" — while the host
ABI moved under it during today's `Mutation::label` → `LocalizedLabel` sweep. `no exported instance
name` is the host asking a 09-20 guest for an instance it does not export. The remedy is the one
CE1 measured for `🀄️wfc`: re-describe the plugin on today's SDK.

## 5. Files changed

| file | change |
| --- | --- |
| `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` | `asset_name_segment`, `example_body_asset_prefix`, `example_body_asset_name` — one naming authority for example-body assets |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `ExampleSourceBody` + `ExampleSource::deferred`/`body`/`deferred_body_asset`; `document_json`/`document`/`payload` return `String`; `into_example_definition` empties a deferred body; `Plugin::deferred_example_assets` collected in `register_app_factory` and merged by `plugin_runtime::plugin_descriptor_extras` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs` | `externalize_oversized_example_bodies` uses the shared name helper (`:54`); its local `asset_name_segment` copy is gone |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-example-source/🦀️.rs` | a law that a deferred leaf declares its authored bytes and never runs its producer (the producer `panic!`s) |
| `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` | `exampleBodyAssetPrefix` next to `exampleBodyAssetName` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts` | `auditNavbarExampleArtifactPayload` accepts any declared asset under the row's body prefix |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | the example-graph scan skips a body-less row instead of pushing an empty string |
| `✏️s/🔌️plugins/🧩️puzzle/…/📚️examples/{🌙️capsule-dream,🏗️nakagin-capsule-tower,🌲️concrete-forest}/🦀️.rs` (7 leaves) | `ExampleSource::deferred(…, DSL_TEXT.as_bytes(), document_json)` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/…/✏️editor/🦀️.rs:9651` | `puzzle5d_part_kind_options()` unions `concrete-forest` + `nakagin` only — §2.3, with the two reasons in its docstring |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | `part_kind_options_are_named_kinds_and_never_catalog_uuids` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts` | the `descriptor-pair-missing` expectation is `[]` (was a 15-id list), and the case is renamed to say so |
| 12 call sites across `🧩️puzzle`, `🖍️draw`, `🪵️sourcing`, `🀄️wfc`, `🌍️gis` | `document_json()`/`document()`/`payload()` now return an owned `String`, so `parse_dsl(&…)`; `🪵️sourcing`'s `set-active-example` match arms unified on `String` |


## 6. Honest gaps

1. **`🧩️puzzle` is still the one catalog diagnostic, and `client-e2e` is 30/32.** The §2.3 fix is
   landed and its law passes in 0.07 s (the wall clock IS the proof the 3 MB document is no longer
   materialised), but the re-describe that would prove it end to end never got a working mutex hold
   — §4.4 records all three attempts. The run is detached (pid 65484) and queued, so the ledger may
   answer after this report. Until it does, `🔣️.json` is still the 09-19 4 803 294 B file and `client-e2e` red 1 stands.
   Nothing is half-landed: a failed `describe` discards its staging directory, so the committed pair
   is untouched.
2. **`🗄️stdio` finished 35 s inside a 1 800 s wall.** That is a real result, not a comfortable one
   (§1). The next artifact added under `🗄️stdio` will push it over.
3. **`🀄️wfc`/`semio.infer` is not fixed** (§3), deliberately: the root fix is a framework packet
   (a `BoundedJob` for the builtin kinds, with the retained-payload close discipline
   `run_interactive_inference` spells out), and a speculative half-landing would break every
   plugin's job route. `client-e2e` never reached the inference rows this run anyway — it returns at
   `artifact_create`.
4. **A deferred body loses one thing, and it is named**: `kernel::exampleArtifactSources` scopes
   operator kinds by scanning inline example bodies, and a deferred row has none, so
   `capsule-dream` no longer contributes to that scan. It never contributed to the picker — a guest
   loads `setActiveExample` from its own `LazyLock`s, not from the manifest — but the scope scan is
   a real, if small, loss. The wgpu shell's twin scan was changed to SKIP a body-less row rather
   than push an empty string, which it used to do.
5. **The descriptor declares the authored bytes, not the derived document.** A deferred row's
   `sha256`/`size_bytes` are over `DSL_TEXT`; an externalized inline body's are over the JSON. The
   asset name carries the suffix of whichever it is (`.dsl.semio` vs `.json`) precisely so nothing
   claims to hash bytes it did not see, and the TS audit matches the row's body PREFIX for that
   reason. Nothing fetches either asset yet — no consumer resolves an example body from `assets`.
6. **Nothing here was proven against a live browser shell.** The instruments are the committed
   descriptors, the staged `semio-os-mcp audit`, the two-server `client-e2e` journey over real
   stdio, the registry vitest and five native `cargo check --all-targets` runs.
7. **`🖨️raster`'s pack moved by one byte** (43 938 → 43 937) while its `🔣️.json` stayed
   byte-identical. Not investigated; it decodes and the JSON projection is unchanged.

## 7. Ticket files

`📜️pz1-describe-batch.sh` (A3's per-owner describe recipe with ONE mutex acquisition for a whole
batch), `📜️pz1-await-and-describe.sh` (waits for a green shared tree outside the lock, then takes
it — §4.4), this report. Captures: `🗑️generated/pz1-{describe-ledger,describe-🗄️stdio,
describe-🧩️puzzle,describe-🖨️raster,describe-🗒️note,describe-batch,describe-batch2,describe-batch3,
await,audit-after,client-e2e,part-kind-law,example-source-laws,example-assets-laws,sweep-check}.txt`.

Regenerated, never hand-edited: `✏️s/🔌️plugins/🗄️stdio/{🔣️.json,🛂️.descriptor.semio}` (new) and
`✏️s/🔌️plugins/🖨️raster/{🔣️.json,🛂️.descriptor.semio}`, each written by `bun ./📜️script.ts describe`
in its owner's own rust package.
