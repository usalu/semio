# A3b — catalog to zero diagnostics (EX1 wasm proof, extension describe, puzzle bound, the sweep)

Slice owner: A3b, session 5d (2026-09-20 19:34 →). Appended to A3's work; spec:
`📓️ex1-extension-exports-and-describe-cliff.md` (§2.4 rebuild list, §3 example-asset split),
`📓️a3-descriptor-regeneration.md` (per-plugin table, `📜️a3-describe.sh`),
`📓️wr4-typed-command-dispatch-and-gates.md` §5.3/§5.4 (`client-e2e` 15/17).

---

## 0. Headline — measured only

1. **EX1's ABI fix compiles and WORKS.** `📜️ex1-wasm-check.sh` (never run before) is green — both
   extension crates and the plugin regression crate check for `wasm32-wasip2`, 199 warnings (§1) —
   and an extension then **described for the first time in this tree** (§2).
2. **Catalog diagnostics 22 → 2**, decodable descriptors **38 → 58**, and the only two survivors are
   `🧩️puzzle` (§3) and `🗄️stdio` (GM1's, untouched). **20 of A3's 22 skips are closed** (§5).
3. **Two more declaration defects sat behind the ABI one**, each found by fixing the previous:
   every extension emitted an **empty `packageId`**, and none declared its **host as a dependency**.
   One SDK default + 24 one-line source declarations (§2).
4. **33 descriptors regenerated** through their own `describe` verb (§4, §5b), 14 of them created
   from nothing; **60 committed descriptors now, 42 of them regenerated today**, and every one
   except `🧩️puzzle` is under the 4 MiB bound.
5. **`🎞️animate`'s `setFrame` is declared and callable** (WR4 §5.4's red) — its `frame` block was
   *unexpressible* until `ActionArgDef::object` existed (§2b).
6. **NOT met: `🧩️puzzle` under the bound.** Re-describe hit the 1 800 s epoch deadline again, at
   **4.08 G fuel vs DS1's 1.64 G** — 2.5× more work in the same budget, still unfinished (§3).
7. **`client-e2e` 13/17** (WR4: 15/17). Both rows this slice owned are green; the gate's unpinned
   target moved to `energy`, whose component is missing — and building it makes the gate time out
   instead (§6, measured twice, artifact removed again).

---

## 1. Item 1 — `📜️ex1-wasm-check.sh`: EX1's ABI change compiles for `wasm32-wasip2`

EX1 §4 left this unproven (its queued job never acquired the fleet mutex). It ran here as the
slice's first command and is **green with nothing to fix in EX1's files**.

Machine at 19:34: load 20, **0 `rustc`**, mutex free (`/tmp/semio-wasm-build.lock` absent),
40 GiB free on `/System/Volumes/Data`.

| crate (arm) | rc | wall |
| --- | ---: | ---: |
| `semio-s-plugin-cad-spatial-shape` (single-arg `extension_exports!`) | **0** | 17 s |
| `semio-s-plugin-imperative-text` (single-arg `extension_exports!`) | **0** | 0 s (warm) |
| `semio-s-plugin-mathematical` (plugin, regression side) | **0** | 22 s |

Whole run 19:37:58 → 19:38:37, **199 warnings** across the three — i.e. the expansion really
happened (memory law "Require Warnings As Proof Of Type-Check"), not an aborted check.
Capture: `🗑️generated/a3b-wasm-check.txt` (= `ex1-wasm-check.txt`, the script's own path).

Two things this settles that EX1 could not:

1. `__semio_owned_core_exports!` **type-checks inside the single-argument arm** — `poll_kernel`,
   `checkpoint_now`/`restore_now`, `jobs::{start,step,cancel}_job` and the extension's
   zero-argument `__semio_describe_component` all instantiate against
   `PluginRuntime<NoPluginApp>`. EX1 §2.2 reasoned about the signatures; this compiled them.
2. **A3 §2.4's blocker is gone.** `semio-framework-ui` compiled for `wasm32-wasip2` as a
   dependency of all three crates — the peer gated `📥️input`'s retained-tree helpers behind
   `#[cfg(feature = "wgpu-engine")]` at **16:21** (10 gates in
   `🎯️targets/🧊️wgpu/📥️input/🦀️.rs`). Descriptor regeneration is unblocked for the whole tree.

## 2. Item 2 — can an extension be described at all now? **Yes — and two more defects sat behind it**

`📐️cad/🧩️extensions/📐️spatial-shape` (cheapest extension, 1 598 B descriptor) was run through its own
`describe` three times. The first run is the one that settles EX1's open question:

```
described …/semio_s_plugin_cad_spatial_shape.wasm with core …/…core.wasm
  ("cad-extension-spatial-shape", role=Extension) -> …/🛂️.descriptor.semio + 🔣️.json
```

**The owned interpreter accepted an extension component.** A3 §3b's
`component has no core module implementing the owned Semio actor ABI` is gone — EX1's
`__semio_owned_core_exports!` is what closed it, and the guest ran, emitted a manifest and staged a
descriptor pair. That was never true for any extension in this tree before 2026-09-20.

The run then failed **after** the descriptor was produced, in `verifyDescriptorPairBytesV1`
(`📇️registry/✅️catalog-verification/🟦️.ts:501`) — the emitter's own identity gate, which no
extension had ever reached. Two defects, both of them *declaration* defects of the same family A3
and M5a describe, each found by fixing the previous one:

| # | refusal | cause, located | fix |
| --- | --- | --- | --- |
| 1 | `descriptor packageId does not match the complete Cargo component identity` | `ExtensionBundle::new` left `package_id` **empty** (`🔌️plugin/🦀️.rs:39184`) and **no extension in the tree ever calls `.package_id(…)`**, while all 37 plugins declare `semio:<id>`. Every committed extension descriptor carries `packageId: ""` — including the 14 that still decode. | `ExtensionBundle::new` now defaults it to `format!("semio:{extension_id}")`, still overridable. Audited: for **25 of 26** extension crates the bundle id already equals the `[package.metadata.component] package` id minus `semio:`; the 26th difference is a test-only bundle inside `🏢️aec-building`. |
| 2 | `extension host must be its first dependency` | `.extends("cad")` records the host but pushes **no dependency**; the emitter derives the host from `manifest.dependencies[0].pluginId`. Only `🏢️aec-building` and `🌀️procedural` ever declared `.depends_on(host, …)` — which is exactly why `🏢️aec-building`'s 09-16 descriptor is one of the 14 that decode. | One line per extension at the extension's own source: `.depends_on("<host>", semio_framework::VersionReq::Any)` right after `.extends("<host>")`, matching `🏢️aec-building`'s precedent. **24 crates, 24 insertions, 24 deletions** — `git diff --stat` per file is `2 +-`. |

Defect 2 was deliberately **not** fixed in `ExtensionBundle::extends()` (which could push the host
itself): `🏢️aec-building`'s own law `contribution_onto_cad_requires_a_declared_dependency`
(`🧪️tests/🔬️unit/🦀️.rs:53`) asserts a bundle that extends `cad` **without** `.depends_on` is
rejected, and an auto-push would silently make that law vacuous.

Native proof of both edits (rule 27 does not cover native checks; no wasm slot burned):
`cargo check -p semio-framework-plugin --lib` → **0 errors, 59 warnings**
(`🗑️generated/a3b-framework-plugin-check.txt`).

## 2b. `🎞️animate` — `setFrame` is declarable and declared (WR4 §5.4)

`animate`'s manifest declared `setFrame` with `"args": []` while its reducer decodes a `frame`
block, so the catalog published an empty `inputSchema` and **no agent could call it**. Fixed at
animate's source and re-described.

The blocker was that a `#[dsl(block)]` payload was **unexpressible**: `ArgSchema::Object { fields }`
exists in `🛂️manifest/🦀️.rs:203` and projects a full nested JSON Schema (`:558`), but **had no
constructor** — modelled and unused, the same shape EX1 found in `AssetDeclaration`. So:

| file | change |
| --- | --- |
| `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` | new `ActionArgDef::object(id, label, fields)` next to `text_list`/`json_text` |
| `✏️s/🔌️plugins/🎞️animate/…/✏️editor/🦀️.rs` | `figure_tile_frame_arg_fields()` (the four `FigureTileFrame` fields, one source for both declarations); `.action_args("setFrame", …)` new; `.action_args("setSource", …)` corrected — it declared a flat `src` **text** arg while the reducer decodes a `source` **record** (`decode(action, args, "source")`), so that capability was uncallable for the same reason |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts` | `minimalInputForSchema` recurses into a required object instead of emitting `{}` — its own docstring already promised "every required property filled with a value of its declared type", and an empty `{}` is refused by the guest that decodes it |

Measured: `cargo check -p semio-s-plugin-animate` → **0 errors, 76 warnings**; re-described at
**20:33:53 in 289 s**, `🔣️.json` **123 841 → 131 777 B**, pack **34 525 → 36 058 B**, and the
committed descriptor now carries

```
{"id":"setFrame", …, "args":[{"id":"frame", …, "schema":{"kind":"object","fields":[
  {"id":"x", …,"required":true,"default":0.0}, {"id":"y", …}, …]}}]}
```

## 3. Item 3 — `🧩️puzzle` under the 4 MiB bound: **NOT met, and now measured rather than predicted**

`🧩️puzzle` was run through `describe` once (20:33 → 21:08, **2 071 s**). It died the same way DS1's
run did, **after** EX1's example-asset split:

```
[describe] owned phase=execute fuel=4081418370 elapsed_ms=1667081
… calling owned describe() on …/semio_s_plugin_puzzle.wasm: epoch deadline exceeded
describe semio-s-plugin-puzzle failed: descriptor emitter exited with 1
```

`🔣️.json` stays **4 803 294 B**. What is new is the comparison, and it settles EX1 §7 gap 3:

| run | fuel at the 1 800 s wall | finished? |
| --- | ---: | --- |
| DS1, 09-19 | 1 640 000 000 | no |
| **A3b, 09-20, with the split landed** | **4 081 418 370 (at 1 667 s, still climbing)** | **no** |

The guest got **2.5× further** in the same budget and still did not finish. So the split removed
real work (the DSL-value encode and pack write of the 3.5 MB body) but **the remaining cost is the
`capsule-dream` DSL parse itself**, which A3 §3 located at
`🖐️5d/…/🌙️capsule-dream/🦀️.rs:18-39` — a `LazyLock` that parses 3 035 200 B of DSL and
re-serialises it to JSON at **bundle install**, i.e. before `describe` ever builds a descriptor.
EX1's own asset split cannot avoid it either: it needs the body's `size_bytes` and `sha256`, so it
must materialise it. The remaining fix is the lazy `ExampleSource` EX1 named (body stays bytes, with
its size/hash declared at the source) — a puzzle+SDK change, not a re-describe, and **not landed
here**. Raising `DESCRIBE_DEADLINE_MS` remains the wrong fix (A3 §3).

## 4. Item 4 — the sweep: **24 owners landed, 2 refused, 0 hand-edits**

Driver: A3's `📜️a3-describe.sh`, one owner at a time through the fleet mutex, appending to
`🗑️generated/a3-describe-ledger.txt`. All `🔣️.json`/`🛂️.descriptor.semio` pairs are producer
output; nothing was hand-edited. `🗄️stdio` and `🌍️gis` were **never touched** (GM1's).

| owner | rc | wall | `🔣️.json` before → after | pack before → after |
| --- | --- | ---: | ---: | ---: |
| `🎞️animate` | 0 | 289 s | 123 841 → **131 777** | 34 525 → 36 058 |
| `🧩️puzzle` | **1** | 2 071 s | 4 803 294 → *unchanged* | 4 295 257 → *unchanged* |
| `📜️imperative/🧮️math` | 0 | 508 s | 0 → **30 208** | 0 → 12 680 |
| `📜️imperative/🧠️logic` | 0 | 45 s | 0 → **12 713** | 0 → 5 524 |
| `📜️imperative/🎮️control` | 0 | 38 s | 0 → **4 035** | 0 → 1 940 |
| `📜️imperative/📣️effect` | 0 | 24 s | 0 → **9 732** | 0 → 4 331 |
| `📜️imperative/📝️text` | 0 | 11 s | 0 → **9 420** | 0 → 4 210 |
| `🏭️process/🧱️concrete` | 0 | 93 s | 0 → **9 252** | 0 → 4 229 |
| `🏭️process/🔩️metal` | 0 | 44 s | 0 → **10 887** | 0 → 4 916 |
| `🏭️process/🤖️robotic` | 0 | 41 s | 0 → **10 949** | 0 → 4 922 |
| `🏭️process/🪵️wood` | 0 | 24 s | 0 → **12 564** | 0 → 5 603 |
| `🪵️sourcing/🪵️beams` | 0 | 19 s | 0 → **3 921** | 0 → 1 970 |
| `🪵️sourcing/🧱️slabs` | 0 | 11 s | 0 → **3 181** | 0 → 1 668 |
| `🪵️sourcing/🪟️windows` | 0 | 12 s | 0 → **3 389** | 0 → 1 769 |
| `📐️cad/📐️spatial-shape` | 0 | 9 s | 1 598 → **1 784** | 963 → 1 085 |
| `📐️cad/🔥️aec-building-energy` | 0 | 11 s | 2 656 → **2 848** | 1 446 → 1 574 |
| `📐️cad/🏛️aec-building-structure` | 0 | 11 s | 8 432 → **8 627** | 4 165 → 4 296 |
| `📐️cad/🏢️aec-building` | 0 | 81 s | 5 464 → **5 496** | 2 686 → 2 718 |
| `📖️playbook/🌀️procedural` | 0 | 44 s | 0 → **45 642** | 0 → 14 447 |
| `📖️playbook` | 0 | 56 s | 0 → **115 388** | 0 → 31 135 |
| `🪵️sourcing` | 0 | 79 s | 115 677 → **186 680** | 73 313 → **47 758** |
| `🔱️trinity` | 0 | 149 s | 800 388 → **475 698** | 183 832 → **119 608** |
| `🎪️demonstrator` | 0 | 835 s | 1 769 947 → **1 299 539** | 402 801 → **313 286** |

Notes worth keeping:

- **`📖️playbook/🌀️procedural` was NOT a separate cause.** EX1 §2.4 flagged it as the one extension
  on the three-argument arm whose `NotFound` "has a different cause, still unattributed". It is the
  same cause as the other 15 — the missing `packageId` and the undeclared host dependency — and it
  described in 44 s once both were fixed.
- **The first extension pays the chain, the rest are seconds.** `imperative-math` 508 s, then
  45/38/24/11 s. Same economics A3 §2.1 measured for plugins.
- **`🔱️trinity`, `🎪️demonstrator` and `🪵️sourcing` all SHRANK** (trinity −40 % json / −35 % pack,
  demonstrator −27 % / −22 %) — the 09-15/09-17 generations they replaced predate DS1's roster fix.
- Every extension's `packageId` is now the real `semio:<id>`; the previous generation of all 14
  decodable extension descriptors carried `""`.

## 5. Catalog health, before → after — **22 diagnostics → 2**

Same staged binary as A3 (`🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp audit --folder <repo>`,
161 462 672 B, 18:04) and A3's fixed census (`🐍️m5a-descriptor-census.ts`), before at 19:36 and
after at 21:45. Captures `🗑️generated/a3b-audit-{before,after}.txt`,
`🗑️generated/a3b-census-after.txt`.

| metric | A3's close (= this slice's before) | after | delta |
| --- | ---: | ---: | ---: |
| `[mcp registry] skipping plugin …` (one registry load) | 22 | **2** | **−20** |
| the same over the catalog-load + routing pair (the ticket's doubled figure) | 44 | **4** | **−40** |
| descriptors the audit decodes and inspects | 38 | **58** | **+20** |
| registry rows the census decodes | 45 of 46 | **59 of 60** | +14 |
| capabilities in decodable descriptors | 1 872 | **1 943** | +71 |
| with a description | 155 | **219** | **+64** |
| of those, EN+DE | 155 | **219** | +64 |
| with a declared `audience` | 47 | **66** | +19 |
| with `effects.destructive` | 25 | **44** | +19 |
| gesture routes published to agents undeclared | 35 | **26** | −9 |
| audit findings | 103 | 111 | +8 (20 more descriptors are now inspectable) |

**The two survivors are both out of this slice's reach and both named:**

1. `puzzle`: `🔣️.json did not decode as a PackageDescriptor` — §3, the describe deadline.
2. `stdio`: `NotFound: no committed descriptor` — **GM1's**, deliberately not touched
   (`🗄️stdio` and `🌍️gis` were excluded from every command this slice ran).

So of the 22 skips A3 handed over, **20 are closed**, and neither survivor is an extension.

## 5b. The `🌊️flow` extensions, so this slice leaves nothing stale

The two source fixes in §2 change what **every** extension emits, so the nine `🌊️flow` extension
descriptors (09-15, `packageId: ""`) became stale the moment they landed even though they still
decode. All nine were re-described (22:01–22:06, 14–139 s each, every `rc=0`): `📝️text`
9 098 → 9 219, `🧠️logic` 9 052 → 9 174, `📃️list` 33 514 → 33 635, `🔤️primitive` 16 303 → 16 429,
`📖️dictionary` 32 990 → 33 117, `🧮️math` 125 098 → 125 219, `🖍️draw` 105 690 → 105 811,
`🏗️bim` 126 068 → 126 188, `📐️brep` 423 042 → 423 163. The uniform +121 B is the `packageId` the
old generation did not carry.

**Descriptor inventory after this slice: 60 committed** (A3 inherited 46 — the 14 new ones are the
extensions/plugins that had never been described). **42 of 60 were regenerated today**, 31 of them
by this slice. The 18 that were not are plugins nobody re-described today (`🌍️gis` and `🗄️stdio`
deliberately excluded; `🧩️puzzle` cannot be); they all decode. Largest descriptor in the tree:
`🧩️puzzle` 4 803 294 B, then `🏛️architect` 1 302 865 B — **every descriptor except puzzle is under
the 4 MiB bound.**

## 6. `client-e2e` — **13/17**, and why the number went DOWN while the catalog got better

Command (WR4's, from `🌉️mcp/📦️packages/🟦️typescript`): `bun ./📜️script.ts client-e2e`.
Capture `🗑️generated/a3b-client-e2e.txt`.

| | WR4 (18:00) | A3b (22:0x) |
| --- | --- | --- |
| tally | **15/17** | **13/17** |
| `capability catalog health` | FAIL, **44** diagnostic(s) | FAIL, **4** diagnostic(s) (puzzle + stdio, doubled) |
| the dispatch rows | `animate.…#setFrame` refused by animate's reducer | 3 rows red on **`energy`**: `plugin \`energy\`'s compiled wasm is missing` |

The two reds this slice owned are both gone: catalog health dropped 44 → 4 (and its remaining 4 are
the two survivors of §5), and `animate.setFrame` is no longer the target *nor* undeclarable (§2b).
What replaced them is WR4 gap 5's own prediction: the journey takes **hit 0 of
`capabilities_search({query:"set", kind:["mutation"]})`, deliberately unpinned**, and 20 newly
decodable descriptors moved that hit from `animate` to
`energy.s.energy.model@1/*#editor.set-cell`. `capabilities_describe` on it PASSES — the catalog
types it correctly; the three reds are all one missing build artifact.

That was tested rather than assumed. `semio-s-plugin-energy` was built for `wasm32-wasip2` through
the fleet mutex (`📜️a3b-stage-energy.sh`, **140 s**, 112 705 979 B) and the gate re-run **twice**:
both runs then died at the **240 000 ms** per-request wall inside
`artifact_create(kind=s.energy.model)` — the component's first instantiate does not finish inside
the gate's budget, and no compile cache makes the second run cheaper. That is strictly worse than a
named red (WR4 §5.4 recorded the identical trap for `gis` at 202 MB), so the artifact was removed
again and the tree left exactly as it was found; **13/17 with four named rows is the honest
number.** The rebuild is one command if a future owner wants to chase the instantiate budget:

```sh
zsh ".🧬semio/…/OS-HUB-COLLABORATION-AI-END-TO-END/📜️a3b-stage-energy.sh"
```

## 7. Files changed

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `ExtensionBundle::new` defaults `package_id` to `semio:{extension_id}` (`:39181-39186`) |
| `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` | new `ActionArgDef::object(id, label, fields)` — the first constructor for the `ArgSchema::Object` variant |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts` | `minimalInputForSchema` recurses into a required object |
| `✏️s/🔌️plugins/🎞️animate/…/✏️editor/🦀️.rs` | `figure_tile_frame_arg_fields()`; `setFrame` args declared; `setSource` args corrected to the record its reducer decodes |
| 24 × `✏️s/🔌️plugins/*/🧩️extensions/*/🦀️.rs` | `.depends_on("<host>", semio_framework::VersionReq::Any)` after `.extends("<host>")` — one line each (`flow` ×9, `process` ×4, `imperative` ×5, `sourcing` ×3, `cad` ×3) |

**Generated, never hand-edited** — 33 `🔣️.json` + `🛂️.descriptor.semio` pairs written by
`bun ./📜️script.ts describe` in each owner's own rust package (§4 and §5b; 14 of them are new
files). Ticket files: `📜️a3b-stage-energy.sh`, this report. Captures:
`🗑️generated/a3b-{wasm-check,audit-before,audit-after,census-after,client-e2e,client-e2e-2,framework-plugin-check,animate-native-check,stage-energy,describe-batch,describe-batch2,describe-flow}.txt`
plus A3's shared `a3-describe-ledger.txt` / `a3-describe-<owner>.txt`.

## 8. Honest gaps

1. **`🧩️puzzle` is still over the 4 MiB bound and still cannot be described** (§3). Measured, not
   fixed. The lazy `ExampleSource` is the remaining fix; the deadline must not be raised.
2. **`🗄️stdio` has no descriptor** and is the second of the two surviving diagnostics. It is GM1's
   live work (`trusted-stdio-gis-bootstrap` held the fleet mutex 19:38→20:21 during this slice);
   nothing under `.🧬semio/🌐hub/`, `🗄️stdio` or `🌍️gis` was read, built or written here.
3. **`client-e2e` is 13/17, not 15/17** (§6) — the two rows this slice owned are green, the three
   that replaced them are one missing/oversized build artifact on a plugin this slice does not own,
   and the fourth is gap 1 + gap 2. The gate's own target keeps moving because its search is
   deliberately unpinned.
4. **Nothing here was proven against a live shell or a live MCP client.** The measurements are the
   staged `semio-os-mcp audit` binary, the census over committed descriptors, and the two-server
   `client-e2e` journey over stdio. `live-agent-loop-check` was not re-run.
5. **18 of 60 descriptors were not regenerated today** (§5b). They decode and none is near the
   bound, but they predate today's declaration sweeps, so their agent-facing
   `description`/`audience`/`destructive` numbers are the old ones — the 219/66/44 totals would
   grow again if they were swept.
6. **`ActionArgDef::object` has no law of its own** and no fixture pins the TS/Rust twin of a nested
   object arg. Its only exercise is `animate`'s two declarations and the descriptor they produce.
7. **The 24 `.depends_on` lines all declare `VersionReq::Any`.** That is the weakest true statement
   (an extension does depend on its host); `🏢️aec-building`'s hand-written `^0.1.0` is stricter. If
   the tree later wants real ranges, they are 24 one-line edits, not a framework change.
