# Workstream A — packet manifest (the burn-down of 95)

Baseline established by F1: `policyArtifactEngineFacetForbiddenBreaches` censuses **95** artifact-tree `⚙️engine/` directories at `low`. This file assigns every one of them to a packet. The rule rises to `"high"` only when the on-disk count is **0** — counted with `find`, never from `compose.json` (whose top-level key is `breachs`, so a query on `breaches` returns `[]` and reads as success).

## Distribution — the reason this is not 95 equal packets

| plugin | dirs | LOC | tier |
|---|---:|---:|---|
| `🗄️stdio` | 41 | 32,360 | **RELEASED** — mapped, 41 packets, 38 parallel (see below) |
| `📸️remodel` | 1 | 22,138 | C (one 22k file) |
| `📕️norm` | 15 | 12,570 | C (15 dirs, one plugin) |
| `🔋️energy` | 1 | 11,775 | C |
| `🏗️fem` | 2 | 9,587 | B |
| `🧩️puzzle` | 3 | 9,553 | B |
| `🎞️animate` | 1 | 9,288 | **HELD** by #2545 |
| `🏛️architect` | 1 | 3,599 | **HELD** by #2545 |
| `📐️cad` | 1 | 3,486 | B |
| `🔱️trinity` | 2 | 2,712 | B |
| `🖍️draw` | 1 | 2,133 | B |
| `🏭️process` | 1 | 1,915 | **HELD** by #2545 |
| `📏️layout` | 1 | 1,654 | B |
| `🌀️procedural` | 2 | 1,473 | A |
| `🌍️gis` | 2 | 1,390 | A |
| `💠️lowpoly` | 1 | 1,303 | A |
| `🎬️sequence` | 1 | 1,288 | A |
| `🗒️note` | 1 | 1,186 | A |
| `✒️writer` | 1 | 1,066 | A |
| `🪵️sourcing` | 1 | 1,033 | A |
| `🧱️block` | 3 | 947 | **A — EXEMPLAR** |
| `🖨️raster` | 1 | 868 | A |
| `🎥️shooting` | 1 | 688 | A |
| `💡️reasoning` | 1 | 556 | **HELD** by #2545 |
| `📜️imperative` | 1 | 540 | A |
| `📋️forms` | 1 | 439 | A |
| `🕸️dag` | 1 | 422 | A |
| `🌊️flow` | 1 | 413 | A |
| `➗️mathematical` | 1 | 395 | A |
| `📖️playbook` | 1 | 299 | A |
| `🪐️space` | 1 | 224 | A |
| `🎪️demonstrator` | 1 | 216 | A |
| `🌿️vcs` | 1 | 183 | **A — but see below** |

Two plugins carry **36%** of the total in a single directory each (`📸️remodel` 22k, `🔋️energy` 11.7k). `🗄️stdio` alone is 43% of the directories. An even fan-out would be badly wrong.

## Dependencies on other sessions — measured, not assumed

- **`🗄️stdio` (41 dirs) is RELEASED as of the freeze broadcast.** UCAS (#2548) froze the roster — *directory structure final*, not merely compiling — and verified it directly: `cargo nextest --profile long -p semio-s-plugin-stdio` → **2174 run, 2168 passed, 6 failed, 5 skipped**; `cargo nextest -p semio-framework-plugin` → **150/150**. `🧿️semio` v1 = **18 subsets + `✳️any`**: `animation audio brep cad document drawing flow graph image kit mesh model object presentation table text value video`.

  ⚠️ **Two renames where the name survived but the meaning did not — the second is the dangerous one because a mechanical search finds the wrong directory *and still compiles*:**
  - `workflow` → **`flow`**
  - old value-tree `object` → **`value`**; **`object` now means a *spatial* thing** (transform + owned brep/mesh/value children)

  Every stdio packet must name which of the 19 subsets it touched, and must diff against **6 expected failures, not 0**: `dwg`/`ifc` `fixture_honesty_law` (unowned) and `html`/`json`/`pdf` `inference_default_law` + `md` outline (IIF's; `csv` passes). Attribution independently confirmed against this ticket's own ledger — those dirs' last commit is flag 490/491, every commit of this ticket is 492+.
- **4 plugins are HELD by #2545** (`🏛️architect`, `🎞️animate`, `🏭️process`, `💡️reasoning` — 4 dirs, ~15.4k LOC). APA (#2549) is separately blocked on the same four: `PluginBuilder::setup()` cannot be deleted until all 33 plugins convert, and these four keep the escape hatch alive. Do not touch; coordinate.
- **APA (#2549) IS writing `declaration()` into artifact `⚙️engine/` — measured, 39 of 95 already.**

  ```
  grep -rln "fn declaration\|ArtifactDeclaration" ✏️s/🔌️plugins --include="*.rs" | grep "🗿️artifacts.*⚙️engine"   →  39
  ```

  Attributed via `git log`, not mtime: those files' only commit is today's HEAD (flag 495), i.e. APA's live conversion. Affected: `🔱️trinity` (×2), `📸️remodel`, `🖨️raster`, `📕️norm` (×6), and others.

  **Resolution — APA confirmed and is clearing it themselves.** They independently verified the taxonomy change (both vocabularies now `["🧬️schema","🚪️io"]`, the forbidding rule live at `📜️script.ts:5640` wired at `:5801`, `find` → 95, matching) and agreed the artifact root is `declaration()`'s correct home: it is data describing the artifact, sitting next to `artifact_kind()` where identity already lives.

  They are **letting the three in-flight batches finish rather than killing them** — a half-converted plugin (plugin root pointing at a `declaration()` never written) is far more expensive to diagnose than a file in a known-wrong directory. Then **one mechanical relocation pass**: `crate::artifacts::<x>::engine::declaration()` → `crate::artifacts::<x>::declaration()`, one function and one call site per plugin.

  ⚠️ **The 39 will tick UP before it comes down.** Do not read a rising count as regression.

  **Ordering rule agreed, because both sides would otherwise edit the same `⚙️engine/🦀️component.rs`:**
  > APA's relocation pass goes **first** on every plugin they touched. Packets for those plugins **wait** until APA reports the pass done. Plugins APA never touched proceed now.

  The exemplar `🧱️block` has **no** `declaration()` in any of its three engine dirs, so it is clear of APA either way. After their pass, **re-census rather than trusting either count**, and treat any disagreement as information to bring back to them.
- **`🌿️vcs` is the most interesting packet, not the smallest.** APA reports its demo app cannot port to the pure `genesis() -> Vec<Mutation>` that replaced `seed(&mut ArtifactStore)`, because it needs multi-command history, checkpoints and alternatives that a flat mutation list cannot express. That is not a `genesis()` shortcoming — **it is machine state misfiled as document state**, and it is the single best demonstration of why the engine had to leave the artifact. Claimed for this ticket; APA told to leave `🌿️vcs` on the escape hatch rather than bend `genesis()`.

## Workstream B — `🔄️machine` TypeScript twin: DONE

`🧰️framework/🔨️modules/🔄️machine/🟦️component.ts`, **1132 LOC**, wired into the shared `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts` at `:8` and `:70`. **30 new machine tests pass in both runs** (146 total across 2 runs: 142 pass, 4 fail).

**The executor corrected my instruction, and was right to.** I told it to mirror `🧬️schema`/`🎠️kernel`/`🎯️action-bus` and assume a per-module `📦️packages/🟦️typescript/`. No such thing exists. Verified independently across all seven modules: every framework TS twin is a bare `🟦️component.ts` wired into **one shared glue**, with tests centralized there. It followed the observed convention over my stated assumption — the correct precedence.

**The 4 failures are pre-existing and unrelated**, independently confirmed rather than accepted: `ephemeralBox` appears **0** times in the machine twin, lives in `🎠️kernel` and other modules, and is present at `HEAD`. Flagged for separate follow-up rather than fixed in-scope.

**Rust constructs that did not map cleanly** (decisions recorded in `📓️b-machine-ts-twin-report.md`): static `Machine::definition()` → explicit `Machine<M>` param; associated types → `MachineSpec["Context"]` bundling; `BitSet<const W>` → dynamic `Set`-backed; `u64` fingerprint → `bigint`; `Result`/`thiserror` → discriminated-union returns; `M::Context: Clone` → runtime `structuredClone` (**a narrower guarantee, documented**); `ActorLogic`/`MachineLogic` markers omitted (redundant under structural typing); `StatechartSchema` omitted (nothing to generate in TS). **WasmBridge deliberately not ported** — the TS side is the *consumer* via kernel's existing plugin-wasm bridge, not a reimplementation.

## Packet results — burn-down 95 → 91, all four cleared by DIRECT evidence

| packet | engine dir | attribution of remaining crate errors | basis |
|---|---|---|---|
| `🧱️block/◻2d` (exemplar) | deleted | not ours | errors in `🗄️stdio/📦️glue.rs` + framework `🏪️store`/`📡️spr`/`🗣️dsl`; **0** touching `block2d` |
| `🌊️flow` | deleted | not ours | `could not compile semio-s-plugin-stdio` |
| `✒️writer` | deleted | not ours | `could not compile semio-s-plugin-stdio` |
| `🎬️sequence` | deleted | not ours | `could not compile semio-s-plugin-stdio` |

The last three fail with an **identical single upstream error**, and the compiler *names the failing crate*. Our crates are never compiled at all. That is direct evidence, not inference from file paths — which is the standard adopted after agent reports twice attributed errors by guesswork.

`📕️norm` + `🧱️block` relocation (17 sites): `semio-s-plugin-norm --all-targets` → **green, exit 0**. Reconciled with APA: 45 declarations at artifact root, **0** in `⚙️engine`, **0** `pub fn pilot_languages`, **0** real `engine::declaration` call sites.

## ⚠️ stdio REGRESSED — and this ticket supplied the stale "green"

Declared green (both forms, `Finished`, exit 0) and broadcast to four sessions. **Now red again:**

```
error: couldn't read `…/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`
       No such file or directory (os error 2)
```

Verified on disk: `✳️brep/…/📄set-snapshot` is gone; that directory now holds a **new vocabulary** (`🔗create-edge ✂️delete-edge 🏗️create-vertex 📍move-vertex 🐚create-shell 💥delete-shell ➰replace-curve`). The rename landed on directories and left the `#[path]` mount behind — **third instance of this pattern today**, after `✳️drawing` twice. Another session's lane; **not to be fixed here**, despite looking like a two-minute edit. Two sessions already talked themselves into that edit and were wrong both times.

> **A verification is a timestamp, not a property.** This ticket broadcast a green that was true when measured and false an hour later, and four sessions may have acted on it. The same decay hit the `🖨️raster` baseline row (absent from the 27 because it was *fixed*, not because it was *unbroken*) and APA's 40-minute-stale line numbers.

## The four instruments that return a confident, well-formed, WRONG answer

| instrument | blind to |
|---|---|
| `cargo check` without `--all-targets` | tests/benches/examples — exactly where a vocabulary rename lands |
| `cargo check` without `RUSTC_WRAPPER=""` | anything sccache serves from a stale cache (`.cargo/config.toml:2` sets it repo-wide) |
| `cargo check --workspace` without `--keep-going` | **every crate after the first failure** — reported **3** failing crates where the truth was **27 of 96 / 804 errors** |
| **all of the above**, for a relocated unqualified path | **a silent rebind to a different function** |

The last defeats the whole stack. Across two sessions, **44 of 45** artifact roots contain a shadowing `io_registry` whose `entries()` returns a differently-typed view (`&[&ComposerEntry]` vs `&[ComposerEntry]`). A bare `io_registry::entries()` in a moved body binds to the wrapper: no error, green build, wrong function. **All 17 sites here were qualified** — verified, not assumed.

> **Rule: when relocating code, every unqualified path in the moved body is a hazard until proven otherwise.** Qualify it, or prove no shadow exists in the destination scope. Corollary, learned the hard way by APA: **a stopped pass does not leave nothing behind** — halting one mid-flight still stranded two `pub fn pilot_languages`.

## Two verification rules that override convenience

**1. `RUSTC_WRAPPER=""` on every cargo command, in addition to `--all-targets`.** This repo sets `rustc-wrapper = "sccache"` at `.cargo/config.toml:2`, so **the default state of the repo is sccache-on**. You are not forgetting to unset something; you are failing to set something the repo silently sets for you. APA (#2549) retracted three "compiler-verified at 0 errors" claims (`🔱️trinity`, `🌍️gis`, `🏗️fem`) that came from this configuration — and their proper re-run *died before reaching trinity at all*, so the false green had been reporting success for a crate the compiler never compiled.

> **The override must be written into every subagent's instructions, not merely used by the orchestrator.** An orchestrator who is careful themselves and dispatches ten agents without it has ten unreliable verifications. This ticket had exactly that exposure: the exemplar executor was already running when the rule was learned, and the override had to be sent mid-flight.

Applied to this ticket's own prior claim before repeating it: `🔄️machine` re-run under `RUSTC_WRAPPER=""` with `--all-targets` → **31 passed, 0 failed**. It stands. A standard that only ever catches other people's errors is not a standard.

**2. Nothing plugin-side is verifiable while `semio-s-plugin-stdio` is red.** Measured: `🧱️block` depends on stdio (`📦️packages/🦀️rust/Cargo.toml:57`), and this holds across the plugin graph. Current stdio state: the `#[path]`/`os error 2` dangling mounts are **fixed**, replaced by **14 × `E0432` unresolved import, all confined to `subsets::brep`** — another session's mutation-vocabulary refactor mid-flight, not ours, not to be touched. Packets completed in this window are marked **complete but UNVERIFIED**, with upstream errors quoted, and re-verified in one pass when stdio goes green. A green claim resting on a build that never ran is worse than no claim.

## APA's relocation pass does NOT reduce the 95 — measured

APA reported their pass would get the burn-down "42 directories closer to empty." It will not. Measuring the declaration-bearing engine dirs:

```
dirs = 43 (still growing as batches land)
total LOC = 31,710     avg = 737 LOC/dir
regions = 271          top-level items = 978
dirs under 60 LOC (plausibly declaration-only) = 0
```

**Not one contains only `declaration()`.** The pass lifts one function out of each — correct and worth doing — but empties **zero** directories. The count goes 95 → 95, not 95 → 53. Every one still needs real dissolution. Planning Tier A around 53 would have silently under-scoped it by 42 packets.

## The `*Engine` structs are dead — verified, against a subagent report that said otherwise

A read-only exploration of stdio reported 26 zero-reference `*Engine` structs and concluded: *"This is NOT suspicious. These are entry-point structs that the plugin system loads by resolving the module path at runtime."* **That is speculation stated as fact, and it is wrong.** Taking it would have preserved dead code in ~45 packets.

Refuted on four independent counts, in shipped source only (`✏️s` + `🧰️framework`, excluding ticket generator folders):

| test | result |
|---|---|
| string-literal registry naming any `*Engine` (`"[A-Za-z]*Engine"`) | **0** — no name-based runtime resolution exists |
| `trait ArtifactEngine` in source | **0** |
| `impl … ArtifactEngine for …` in source | **0** |
| every reference to `ZipEngine` / `JsonEngine` | all **3 apiece, all inside their own file**: the `//!` doc line, the `pub struct`, the `impl` block. **Zero construction sites.** |

The only external hits are in `.🦑️repo/🎫️tickets/…/generators/codecs/w4b_png_engine.rs` — the generator that *emitted* the struct, not a consumer. Revealingly, that generator contains `impl protocol::ArtifactEngine for PngEngine` against a trait that was never created, and the shipped file dropped the impl while keeping the struct. **These structs are the fossil of a trait that never shipped** — the same shape as `Block2dEngine` (0 refs, no trait) found in the exemplar. They are the placeholder that named the entire directory class.

**Ruling for all packets: delete the `*Engine` struct outright.** Do not rehome it. If any packet finds one that *is* constructed somewhere, that is a genuine exception — report it rather than assuming this ruling.

**Process note:** subagent reports are evidence, not conclusions. This one was accurate and valuable on inventory, region taxonomy, coupling and test counts — and wrong on the single most consequential judgement in it. Verify the load-bearing claim of any report before acting on it.

## Region → destination map (from the exemplar analysis)

Derived by reading `🧱️block/◻2d`'s engine (314 lines) and counting every external reference. Later packets follow this and report deviations.

| region | destination | note |
|---|---|---|
| `🔖️ArtifactEngine` (the `*Engine` struct) | **DELETE** | `Block2dEngine` had **0** external refs and implements no trait. This is the placeholder that named the whole directory class. |
| derived compute from a snapshot | `🧬️schema/💡️inferences/` | e.g. `puzzle2d_manifest_fragment` (3 refs) |
| pure document helpers | `🧬️schema/` (snapshot side) | `empty_block2d_snapshot` (14 refs), `next_id` (9 refs) — main callers are `🧬️schema/🧬️mutations/` |
| `*_io() -> AppIo` | `🎛️apps/<app>/` | its own docstring already said "`…PlayApp`'s typed media I/O surface (`AppDefinition.io`)" — app surface all along (6 refs) |
| `io_registry` / `ComposerEntry` / serializers | `🚪️io/` | ~90 lines in block2d |
| `register*()` wiring | plugin-root setup | collision risk with APA's conversion — rehome minimally, report, don't fight |
| `🧪️Tests` | split to follow each symbol | every assertion must survive |

The app-side `⚙️engine` directories (`🎛️apps/<app>/⚙️engine`) are **empty stubs — 0 LOC** in `🧱️block`. They are required by `appComponentDirs` and are the intended destination for behaviour. They stay.

## `🗄️stdio` — mapped in detail

41 engines, ~29,885 LOC, 424 region declarations across **187 unique region names**.

- **Not templatable, but the skeleton is uniform.** 38 distinct region *patterns* across 41 engines; only 2 patterns are shared. Every engine has the same 2–3 core regions (`Register`, a codec region, `DerivedIoRegistry`) and 39/41 have `ConformanceLaws`. The wiring is identical; the meat (`Huffman`, `Adam7`, `ColorTableConv`, `Cp437`) is bespoke per format. ⇒ **one packet per engine, pattern as reference not template.**
- **Highly parallelizable: 38 of 41 are fully independent.** Exactly 3 dependency chains, all version pairs: `gif 87a → 89a`, `pdf 1.4 → 1.7`, `dwg ac1018 → ac1024`. Serialize each pair; everything else fans out.
- **Zero `declaration()` contamination** — APA's conversion has not touched stdio engines, so stdio packets are clear of the ordering rule.
- **Region classification matches the exemplar map**, with one addition: the `ConformanceLaws` / `CodecRetentionLaw` / `FieldSweep` / `MutationDiffLaw` / `InverseLaw` / `AbsorbLaw` family (30 engines) is **derived compute → `🧬️schema/💡️inferences`**. Notably **no** engine references `AppIo` or any app type — stdio engines are already decoupled from the app layer, so the `→ 🎛️apps/` destination is empty here.
- **~1,430 assertions across 39/41** engines, co-located with their regions; all must survive relocation.
- 8 engines make cross-plugin calls (`zip`/`txt`/`csv`/`png`/`json` → `🗒️note`; `mp4`/`avi` → `📸️remodel`; `dwg ac1024` → `🏛️architect`) — calls to shared utilities, not struct instantiations, so safe during dissolution.

## ⚠️ `🗄️stdio`'s `⚙️engine` is the repo's de-facto codec library — measure before dispatching

Discovered while clearing `🔋️energy`'s last references. Energy's own dissolution was clean (0 dirs, 0 own-engine refs), but one surviving line was **code**, not a comment:

```rust
semio_s_plugin_stdio::artifacts::epw::standards::energyplus::engine::decode_epw(content)
```

Enumerated repo-wide (excluding stdio itself):

| consumers | files | plugins |
|---|---|---|
| **15 plugins** | **19 files** | `🖨️raster`(14) `📸️remodel`(7) `🗒️note`(6) `📐️cad`(6) `🏗️fem`(6) `🔱️trinity`(4) `📏️layout`(3) `🌍️gis`(3) `🔋️energy`(2) `📜️imperative`(2) `🎥️shooting`(2) `🎞️animate`(2) `🧩️puzzle`(1) `🖍️draw`(1) `✒️writer`(1) |

Symbols consumed: `encode_png`/`decode_png` (6 each), `parse_markdown_blocks`/`render_markdown_blocks` (3 each), `encode_stl_ascii`, `decode_epw`, `encode_jpg`/`decode_jpg`, `encode_gif`, `decode_pdf`, `encode_stl_binary`, `register`.

**Consequences:**

1. **Dissolving stdio's 41 engines is NOT a stdio-local change.** It breaks 15 other plugins at ~60 call sites. Any packet scoped to "stdio only" produces a green stdio and a red everything-else.
2. **Destination is clear and the taxonomy already allows it.** These are codecs — (de)serialization — so they belong in each artifact's `🚪️io/{📥️import/🧩️deserializers,📤️export/🧵️serializers}`. Pure algorithms with no artifact of their own may instead go one level up into a module's `⚙️engine` (`🧰️framework/🔨️modules/<domain>/⚙️engine/`), which `taxonomyLeafParentDirs` keeps globally legal. Nothing here needs a new vocabulary word.
3. **`🗄️stdio` must run LAST**, after every other packet lands — its call-site updates land in 15 plugins that other agents are editing right now. Running it concurrently guarantees collisions.
4. It also confirms the earlier exploration's finding from the other direction: that report noted 8 stdio engines calling *out* to other plugins; this is 15 plugins calling *in*. The coupling is bidirectional and denser than either measurement alone suggested.

**This is the single largest remaining risk in the ticket**, and it was invisible until a packet completed and left three references behind — two of which were comments. Enumerating rather than counting the grep is what separated them.

## Cross-plugin engine consumers — a packet can break crates its own `cargo check` never builds

Generalised from the `🔋️energy` discovery. Artifact engines are consumed **across plugin boundaries and by the framework itself**, so `cargo check -p <the plugin>` comes back **green while breaking someone else**.

| provider | consumers | note |
|---|---|---|
| `🗄️stdio` (all) | **15 plugins, 19 files** | the de-facto codec library — see section above |
| **`puzzle2d`** | **the OS renderer**, 5 refs in `📺️renderer/…/EngineCanvas/🧊️component.rs` (`:71`, `:1478`, `:1562`, `:1578`, `:1587`) | `BoardHost` + `board_host::puzzle_board_host()` — a **stateful host**, so rule 7: app-side |
| `📐️cad` | `🎪️demonstrator` (1), `💠️lowpoly` (2) | `cad_document_from_dwg` / `cad_document_from_mesh` — deserialisation, so rule 5: `🚪️io/📥️import/🧩️deserializers/` |

**The `puzzle2d` case is the sharpest**: a *framework* crate depends on a *plugin's artifact engine*. Deleting that directory breaks the OS renderer, and nothing in the puzzle plugin's own build would report it.

> **RULE for every packet: before deleting an engine directory, grep the whole repo — not just your plugin — for consumers.**
> ```
> grep -rn "::artifacts::[a-z0-9_]*::engine::" ✏️s/🔌️plugins 🧰️framework --include="*.rs" \
>   | grep -v "crate::artifacts::" | grep -v "semio_s_plugin_stdio::"
> ```
> Then `cargo check -p` **each consumer crate**, not only your own. "Checked, none" is a required result, not an optional one.

**A false alarm worth recording, because it nearly became a report of breakage.** A repo-wide sweep found 22 code references to engines in plugins whose directories were already gone — apparently 22 breakages across 7 plugins. Enumerating them showed **21 were `semio_s_plugin_stdio::artifacts::<x>::engine::…`**, i.e. legitimate references into stdio's still-standing engines, and only **1** was a genuine cross-plugin case (`🎪️demonstrator` → `cad`). Counting the grep would have produced a 22× overstatement and sent several agents chasing nothing. **Grep to find, enumerate to count** — third application today, first one where it prevented a false alarm rather than catching a real defect.

## Execution plan

1. **P-A2 `🧱️block` — exemplar, in flight.** Its verified diff is the pattern. Nothing else dispatches until it is green and reviewed.
2. **Tier A** (~18 plugins, ≤1.5k LOC each, one packet per plugin) — parallel Sonnet executors, batched.
3. **Tier B** (`🏗️fem`, `🧩️puzzle`, `📐️cad`, `🔱️trinity`, `🖍️draw`, `📏️layout`) — one packet per plugin.
4. **Tier C** (`📸️remodel` 22k, `🔋️energy` 11.7k, `📕️norm` 15 dirs) — split per directory/region, not per plugin.
5. **`🌿️vcs`** — taken deliberately with the machine-state design, not as a rote dissolution.
6. **HELD 4** — only after #2545 releases.
7. **`🗄️stdio` 41** — only after UCAS freezes the roster, and authored against the *new* names.
8. Raise the forbidding rule to `"high"` when `find … -name "⚙️engine" | grep 🗿️artifacts | wc -l` is 0.
