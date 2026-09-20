# A3 — Descriptor regeneration (every shipped plugin `🔣️.json` through its own `describe`)

Slice owner: A3, session 5c (2026-09-20 12:45 →).
Spec: `📓️a1-mcp-end-to-end.md` §3, `📓️m5a-mcp-catalog-agent-usability.md` §5/§8, `📓️ds1-stdio-descriptor-bound.md` §10, `📓️wr2-headless-command-response-wire.md` §8.

Three outcomes this slice owns:
1. the semio MCP capability catalog compiles with **zero** skips/diagnostics (58 today = 29 plugins × 2 reads),
2. agents see `description` / `audience` / `destructive` (M5a's census over committed descriptors: 1866 capabilities, **0** described, **0** destructive),
3. every descriptor is under the 4 MiB `DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES` bound (DS1's roster fix only lands on re-describe).

**Never hand-edit a generated `🔣️.json`** — `hashes.descriptorSha256` is a self-hash over the descriptor's own encoded pack (`🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:434-442`). The only repair is the producer verb.

---

## 0. Headline — five lines, measured only

1. **7 descriptors regenerated** through their own `describe` verb (`➗️mathematical`, `🎞️animate`,
   `🌿️vcs`, `📋️forms`, `🎬️sequence`, `💡️reasoning`, `🕸️dag`); with `📕️norm` and `🗒️note`, which
   landed before this slice, **9 of 46** committed descriptors are current. 3 more were still queued
   behind a peer on the fleet mutex when this slice ended (§6.3) — no result is claimed for them.
2. **Catalog diagnostics 29 → 22** on one registry load (the ticket's doubled figure: **58 → 44**),
   decodable descriptors **31 → 38**, audit findings **110 → 103**. Outcome (a), zero skips, is **not
   met**.
3. **Agents now see the M5a sweep**: descriptions **40 → 155** (all EN+DE), declared `audience`
   **14 → 47**, `destructive` **4 → 25**, undeclared gesture routes **54 → 35**. Outcome (b) works
   exactly as M5a §8.8 predicted — regeneration alone turns it on, no emitter change.
4. **The brief's "0 described" baseline was partly a measurement bug, now fixed** (§4.2):
   `🐍️m5a-descriptor-census.ts` did not unwrap the `{"native":{"en","de"}}` envelope, so it reported
   0 for `🗒️note`, which already carried 40 bilingual descriptions.
5. **The biggest finding is not a descriptor, it is one macro arm** (§3b): `extension_exports!`'s
   single-argument arm never emits the nine `semio_owned_*_v1` core exports, so **no extension can be
   described at all** — 16 of the 29 skips, including A1's unattributed
   `imperative-extension-text` failure. **Largest descriptor: `🧩️puzzle`, 4 803 294 B
   (pack 4 295 257 B), still over the 4 MiB bound** and still the one plugin whose `describe` cannot
   finish (§3). Outcome (c) is **not met**.

---

## 1. Inherited state (measured at 12:45 on 2026-09-20)

### 1.1 The 29 skipping plugins, by cause (WR2 §8, re-read off `🗑️generated/m5a-s5-capability-audit.txt`)

| cause | count | plugins |
| --- | --- | --- |
| `missing field \`artifactSchema\`` (rename drift: `documentSchema`→`artifactSchema`) | 10 | `dag`, `demonstrator`, `forms`, `mathematical`, `puzzle`, `reasoning`, `sequence`, `sourcing`, `trinity`, `vcs` |
| `missing field \`executionProtocol\`` (new required field) | 3 | `cad-extension-aec-building-{energy,structure}`, `cad-extension-spatial-shape` |
| `missing field \`windowKindId\`` | 1 | `animate` |
| `NotFound: no committed descriptor` (never described) | 15 | `stdio`, `playbook`, `playbook-module-procedural`, `sourcing-module-{beams,slabs,windows}`, `process-extension-{concrete,metal,robotic,wood}`, `imperative-extension-{control,effect,logic,math,text}` |

### 1.2 Descriptor inventory — before (`✏️s/🔌️plugins/*/🔣️.json`, `stat` at 12:52)

34 plugin owner dirs; **32 carry a `🔣️.json`**, `📖️playbook` and `🗄️stdio` do not.
27 extension owner dirs; **14 carry one** (10 × `🌊️flow`, 4 × `📐️cad`), 13 do not
(`🏭️process` ×4, `📜️imperative` ×5, `🪵️sourcing` ×3, `📖️playbook/🌀️procedural`).
So **46 committed descriptors / 61 owners** — the 15 gap is exactly WR2's `NotFound` set.

| plugin | json bytes | mtime | plugin | json bytes | mtime |
| --- | ---: | --- | --- | ---: | --- |
| `🧩️puzzle` | 4 803 294 | 09-19 03:23 | `📜️imperative` | 205 066 | 09-17 12:32 |
| `🎪️demonstrator` | 1 769 947 | 09-17 00:24 | `📏️layout` | 199 861 | 09-18 17:02 |
| `🏛️architect` | 1 302 865 | 09-17 12:09 | `🗒️note` | 197 877 | **09-20 11:51** |
| `🌀️procedural` | 1 241 651 | 09-18 13:10 | `🏭️process` | 188 300 | 09-17 12:12 |
| `📕️norm` | 1 213 835 | **09-20 07:21** | `🖍️draw` | 168 066 | 09-18 16:39 |
| `🌍️gis` | 1 077 748 | 09-17 13:33 | `💡️reasoning` | 135 720 | 09-15 15:01 |
| `🀄️wfc` | 964 457 | 09-18 18:09 | `✒️writer` | 121 580 | 09-20 09:18 |
| `🔱️trinity` | 800 388 | 09-15 15:01 | `🪵️sourcing` | 115 677 | 09-17 10:33 |
| `🏗️fem` | 739 708 | 09-17 10:29 | `🎬️sequence` | 93 037 | 09-17 10:33 |
| `📸️remodel` | 648 137 | 09-19 12:22 | `📋️forms` | 82 978 | 09-17 10:33 |
| `🌊️flow` | 505 087 | 09-18 22:49 | `🌿️vcs` | 56 583 | 09-17 10:33 |
| `📐️cad` | 483 290 | 09-19 17:53 | `🎞️animate` | 44 008 | 09-17 10:33 |
| `🔋️energy` | 471 401 | 09-17 00:24 | `➗️mathematical` | 42 783 | 09-17 10:33 |
| `🪐️space` | 454 019 | 09-19 02:16 | `🕸️dag` | 234 496 | 09-15 15:01 |
| `🧱️block` | 399 498 | 09-20 00:08 | `🎥️shooting` | 276 323 | 09-17 12:11 |
| `💠️lowpoly` | 333 382 | 09-17 23:25 | `🖨️raster` | 229 357 | 09-17 12:13 |

`📕️norm` (DS1, 07:21) and `🗒️note` (M5a, 11:51) were already re-described before this slice started;
both are counted as landed and are not re-run here.

### 1.3 Machine state this slice ran under (rule 14/23 context)

Load average **35 → 61** for the whole session, 13–24 concurrent `rustc`, with a peer holding a
workspace-wide `cargo check --target wasm32-wasip2` over all 30 plugin crates (pid 49966) and further
peers holding 26–56-minute `wasm-dev` component builds of `animate`, `sourcing`, `space`,
`playbook-procedural` and `flow-extension-bim`. Every wall time below is measured under that load and
is **not** the "≈6 min calm" figure the brief assumed.

---

## 2. Per-plugin regeneration table

Columns: stale? → described at → `🔣️.json` bytes before/after → pack bytes before/after → catalog findings before/after.

Driver: `📜️a3-describe.sh` (ticket folder) — one `bun ./📜️script.ts describe` per owner, through the
coordinator's `📜️wasm-build-mutex.sh` (preamble rule 27) from 14:06 on, appending one row per landed
plugin to `🗑️generated/a3-describe-ledger.txt` and one full capture to
`🗑️generated/a3-describe-<owner>.txt`. `CARGO_PROFILE_WASM_DEV_DEBUG=false`, `NX_DAEMON=false`,
**shared** artifact dir on purpose (see §2.1).

| plugin | stale? | described at | json before | json after | pack before | pack after | wall | notes |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| `📕️norm` | no | 09-20 07:21 (DS1) | — | 1 213 835 | 338 948 | 281 469 | — | landed before this slice; −17 % pack |
| `🗒️note` | no | 09-20 11:51 (M5a) | — | 197 877 | — | — | — | landed before this slice; 30 described EN+DE, 14 audience, 4 destructive |
| `➗️mathematical` | **yes** (`artifactSchema`) | **09-20 14:05** | 42 783 | **88 529** | 28 799 | **25 244** | 444 s | first cold attempt SIGKILLed at 4 003 s (§2.2); warm retry clean |
| `🎞️animate` | **yes** (`windowKindId`) | **09-20 14:17** | 44 008 | **123 841** | 31 057 | 34 525 | 713 s | |
| `🌿️vcs` | **yes** (`artifactSchema`) | **09-20 14:47** | 56 583 | **866 206** | 37 228 | 283 450 | 1 797 s | guest ran **1 561 s** of the 1 800 s epoch — 4 min from puzzle's failure mode (§2.3) |
| `📋️forms` | **yes** (`artifactSchema`) | **09-20 14:48** | 82 978 | **176 836** | 55 088 | **49 359** | 63 s | pack shrank — DS1's roster fix |
| `🎬️sequence` | **yes** (`artifactSchema`) | **09-20 14:49** | 93 037 | **169 651** | 60 696 | **42 894** | 55 s | pack −29 % |
| `💡️reasoning` | **yes** (`artifactSchema`) | **09-20 14:50** | 135 720 | **142 306** | 35 329 | 39 168 | 50 s | |
| `🕸️dag` | **yes** (`artifactSchema`) | **09-20 14:51** | 234 496 | **192 722** | 57 518 | **51 597** | 63 s | both json and pack shrank |
| `📐️cad/🧩️extensions/📐️spatial-shape` | yes (`executionProtocol`) | — | 1 598 | *unchanged* | 963 | *unchanged* | 9 s | **rc=1**, §3b |
| `📐️cad/🧩️extensions/🔥️aec-building-energy` | yes (`executionProtocol`) | — | 2 656 | *unchanged* | 1 446 | *unchanged* | 9 s | **rc=1**, §3b |
| `📐️cad/🧩️extensions/🏛️aec-building-structure` | yes (`executionProtocol`) | — | 8 432 | *unchanged* | 4 165 | *unchanged* | 187 s | **rc=1**, §3b |
| `🪵️sourcing` | yes (`artifactSchema`) | — | 115 677 | *unchanged* | 73 313 | *unchanged* | 2 426 s | **rc=1 — peer's broken tree**, §2.4 |
| `🔱️trinity`, `🎪️demonstrator` | yes (`artifactSchema`) | — | — | — | — | — | — | not attempted: batch stopped, §2.4 |
| `🧩️puzzle` | yes (`artifactSchema`) | — | 4 803 294 | — | 4 295 257 | — | — | **cannot be described at all**, §3 |

**Descriptors grow, they do not shrink.** DS1 measured `📕️norm` at −17 % and predicted the roster fix
would shrink every descriptor. That is true of the *roster term only*. Measured over the four
plugins re-described so far, the `🔣️.json` is **2–15× larger** than the stale generation it replaces,
because those generations date from 09-15/09-17 and the tree has since added the whole M5a
declaration sweep (descriptions EN+DE, `useWhen`, `audience`, `destructive`), the `artifactSchema`
rename, `executionProtocol`, and the verbs each plugin has gained. `🌿️vcs` 56 583 → 866 206 B is the
extreme. The pack is the number the 4 MiB contract bound actually reads, and it grows too
(37 228 → 283 450 for vcs). **Nothing regenerated so far is near the bound**; `🧩️puzzle` remains the
only descriptor over it, and it is the one that cannot be regenerated at all (§3).

### 2.4 What ended the sweep: `semio-framework-ui` is broken in the working tree by a peer

`🪵️sourcing`'s `describe` spent 2 426 s and failed — **not** on anything in sourcing:

```
error[E0433]: cannot find `tree` in `wgpu`
   --> 🧰️framework/🔨️modules/🖱️ui/…/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:717:25
    | 717 |     tree: &crate::wgpu::tree::UiTree,
note: found an item that was configured out
   --> 🧰️framework/🔨️modules/🖱️ui/…/🎯️targets/🧊️wgpu/🦀️.rs:67:9
error: could not compile `semio-framework-ui` (lib) due to 18 previous errors
```

Attributed, not guessed: `git diff` shows `🎯️targets/🧊️wgpu/🦀️.rs` **uncommitted, +47/−32**, with
`📥️input` newly mounted *unconditionally* ("target-neutral pointer and keyboard input, with retained
engine registrations gated by `wgpu-engine`", `:131-133`) while `wgpu::{tree, arena, layout, chrome,
flex}` remain behind `#[cfg(feature = "wgpu-engine")]` (`:60-70`). A peer is mid-refactor; HEAD is
clean; nothing in this slice touched that module.

Every plugin `describe` compiles `semio-framework-ui` for `wasm32-wasip2` **without** `wgpu-engine`,
so from ~15:35 no descriptor in the tree could be regenerated by anyone. **This slice stopped its own
batch at that point** (`kill` on pids 5134 / 69553, the two processes it started) rather than keep
the shared fleet wasm mutex occupied with builds that could not succeed. `🔱️trinity` and
`🎪️demonstrator` were never attempted. Per preamble rule 3 the peer's refactor is not this slice's
to fix; the remaining describes are a rerun of `📜️a3-describe.sh` once that module compiles again.

### 2.3 The 30-minute guest epoch is a general cliff, not a puzzle-specific one

`🌿️vcs` — a plugin whose *old* descriptor was 56 583 B, the second smallest in the tree — ran its
guest for **1 561 s** and finished 239 s inside `DESCRIBE_DEADLINE_MS`. Measured fuel traces:

| plugin | fuel at completion | guest wall | fuel rate |
| --- | ---: | ---: | ---: |
| `🗒️note` (M5a, 11:51, calm) | 120 846 464 | 58 s | 2.07 M/s |
| `🌿️vcs` (A3, 14:47, load 25–35) | 4 175 701 164 | 1 561 s | 2.68 M/s |
| `🧩️puzzle` (DS1, killed) | 1 640 000 000 at the deadline | **1 800 s, killed** | 0.91 M/s |

The fuel *rate* is the same order on a calm and a loaded machine, so the wall deadline is not a load
artifact — it is a fixed budget over a workload that varies by **35×** between plugins. Two
consequences worth recording:

1. **`DESCRIBE_FUEL_BUDGET = 8_000_000_000` is unreachable on this hardware.** At ~2.5 M fuel/s the
   1 800 s wall deadline caps the guest at ≈ 4.5 G fuel, so the deterministic runaway bound the
   constant's docstring describes as "the independent fuel cap" can never fire here — the wall clock
   always wins first. The two bounds are not independent in practice; they are one bound, and it is
   the wall clock.
2. **`🧩️puzzle` is not alone, it is just first.** `🌿️vcs` came within 4 minutes of the same failure.
   Any plugin that grows its declaration surface meaningfully will cross it next, and the failure
   mode is a `describe` that cannot run at all — i.e. a descriptor that can never again be
   regenerated, which is exactly the state `🧩️puzzle` is in today.

### 2.1 Why the shared artifact dir, not rule 25's private one

Rule 25's private `CARGO_TARGET_DIR` cures *writer starvation on the artifact directory*. A plugin
`describe` is not artifact-bound, it is **build-dir bound**: it compiles the whole
`wasm32-wasip2 / wasm-dev` framework dependency chain, and those units are shared. A private artifact
dir would not have saved a single one of them, while pointing `describe` at a cold tree would have
paid that chain again per plugin. Measured both sides of exactly this on `➗️mathematical`:

| | wall |
| --- | ---: |
| cold — framework wasm32 chain not yet built (`semio-framework-ui`, `-schema`, `-tool-run`, …) | **4 003 s**, then SIGKILL |
| warm — same command, same dir, immediately after | **444 s**, exit 0 |

That 9× is the whole economics of this slice: the first `describe` pays the framework chain, every
later one pays only its own crate plus the guest run. It is also why the per-plugin estimate in the
brief ("≈ 6 min calm") is right *only* from the second plugin onward.

### 2.2 The first attempt was OOM-killed, not budget-killed

`🗑️generated/a3-describe-➗️mathematical.txt:151`:

```
error: cargo build -p semio-s-plugin-mathematical --target wasm32-wasip2 --profile wasm-dev killed by signal SIGKILL
  at buildPluginComponent (…/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts:58:3)
```

No `[budget] … exceeded <n>ms — killed` line was printed, so this is **not** `runCmd`'s budget
timeout — it is the kernel. Measured at the time of the kill: `vm.swapusage` **25 653 MB used of
26 624 MB**, 233 233 pageouts, 6–9 concurrent `wasm-dev` component builds from peers. This is the
swap-kill regime the repo has recorded before, and it is what preamble rule 27 (the fleet wasm build
mutex, added by the coordinator at 13:58) exists to prevent. Every `describe` from 14:06 on runs
through that mutex.

---

## 3. `🧩️puzzle` — why its `describe` cannot finish

DS1 §10.10 named the symptom (30-minute guest epoch, `phase=execute`, fuel 1.64 G and climbing, i.e.
progress not a hang) and guessed the cause was "the 3.5 MB inlined example". **This slice read the
declaration and the cause is one line more specific than that, and it is not a walk over an
already-materialised string — it is a DSL parse plus a JSON re-serialisation executed inside the
unoptimised guest on every `describe`:**

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream/🦀️.rs:18-39`

```rust
pub const DSL_TEXT: &str = include_str!("🖼️assets/🌙️dream/🗣️.dsl.semio");

fn document_json() -> String {
    let projection = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(DSL_TEXT)…;
    dsl::json::to_json_string(&projection)
}

pub static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| ExampleSource::new(ID, label(), document_json(), ICON));
```

Measured fixture sizes under `🧩️puzzle` (`stat`, this session):

| fixture | bytes |
| --- | ---: |
| `🖐️5d/…/🌙️capsule-dream/🖼️assets/🌙️dream/🗣️.dsl.semio` | **3 035 200** |
| `🖐️5d/…/🌙️capsule-dream/🖼️assets/🔣️.json` (golden poses) | 860 636 |
| `🖐️5d/…/🏗️nakagin-capsule-tower/…/🗣️.dsl.semio` | 168 355 |
| `🧊️3d/…/🏗️nakagin-capsule-tower/…/🗣️.dsl.semio` | 128 755 |
| `◻️2d/…/🏗️nakagin-capsule-tower/…/🗣️.dsl.semio` | 93 779 |
| every other puzzle example fixture | < 8 KB |

So `capsule-dream` alone is **3.03 MB of DSL text that the guest parses and re-emits as JSON** every
time the descriptor is built — and the resulting 3 560 143 B JSON string is then carried verbatim in
`ExampleDefinition.artifact_json`, which is the 74 % of the descriptor DS1 measured. The example is
also `include_str!`-ed, so it is in the component binary as well.

That makes the two costs one cost with two fixes of very different price:

- **`DESCRIBE_DEADLINE_MS` is not the bug and must not be raised.** It is a documented constant
  (`🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:250`, `= 1_800_000`) with its own law
  (`🖨️describe/🧪️tests/🔬️unit/🦀️.rs:20` asserts the exact value). The independent fuel cap is
  `DESCRIBE_FUEL_BUDGET = 8_000_000_000` and puzzle was only at 1.64 G, so raising the wall clock
  would only buy a longer run of the same quadratic-ish work, and would slow every other plugin's
  failure mode down with it.
- **`describePluginComponent` hard-codes `--profile wasm-dev`**
  (`🖨️describe/🏗️component-build/🟦️.ts:56`, `buildPluginComponent`), and
  `DESCRIBE_ARTIFACT_MAX_BYTES`'s own docstring (`:252-258`) says that is deliberate — the emitter
  reads the *unoptimised* artifact on purpose, and the bound it carries (256 MiB) is sized for it.
  The trusted bootstrap never hits the deadline precisely because it builds `wasm-release`.
- **The structural fix — examples travel as referenced assets, not inlined strings** — is DS1
  §10.6(b), and DS1 already scoped it as a slice of its own, not a cleanup: the host, the React
  shell's example picker (`examplesForDialect` / `exampleArtifactSources`) and the hub all read
  `artifact_json` as an *immediately available* string, so `AssetDeclaration` cannot replace it
  without an asset-fetch path in front of all three.

**Decision, stated honestly: this slice did not land either fix.** Raising the deadline is the wrong
fix; the profile change and the asset-reference change are both framework-source changes whose
verification cost (a re-describe of every plugin to prove nothing else regressed) exceeded what was
reachable on a machine at load 35–61. `🧩️puzzle` therefore stays over the 4 MiB bound and outcome (3)
of this slice is **not** met. What this section adds over DS1 is the exact declaration and the
measured fixture table, so the next owner starts from a located line rather than a hypothesis.

---

## 3b. Root cause: **no extension can be described at all** — one macro arm, 16 of the 29 skips

This is the largest finding of the slice, and it is a single located line rather than per-plugin debt.

### 3b.1 What was measured

All three stale `📐️cad` extensions were run through their own `describe` verb and all three failed
identically (`🗑️generated/a3-describe-📐️cad_🧩️extensions_*.txt`, ledger rows at 14:51/14:51/14:54):

```
[describe] owned phase=compile bytes=17873039 elapsed_ms=0
semio-framework-plugin-describe describe: compiling …/semio_s_plugin_cad_spatial_shape.wasm with the
  owned interpreter: plugin: wasm validation: component has no core module implementing the owned
  Semio actor ABI
describe semio-s-plugin-cad-spatial-shape failed: descriptor emitter exited with 1
```

The wasm builds fine — it is the **validation** that rejects it. A1 §3 measured the same
`descriptor emitter exited with 1` on `imperative-extension-text` on 09-19 and could not attribute
it ("a per-plugin emitter defect of unknown breadth"). It is not per-plugin and it is not unknown.

Symbol-level proof, `strings` over the two components in the shared wasm-dev tree:

| symbol (`OwnedSemioExport::ALL`, `🔌️plugin/🧠️interpreter/🦀️.rs:522-530`) | `semio_s_plugin_mathematical.wasm` (plugin) | `semio_s_plugin_cad_spatial_shape.wasm` (extension) |
| --- | ---: | ---: |
| `semio_owned_alloc_v1` | 2 | **0** |
| `semio_owned_describe_v1` | 8 | **0** |
| `semio_owned_poll_v1` | 10 | **0** |
| `semio_owned_step_job_v1` | 2 | **0** |

### 3b.2 The line

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `extension_exports!` (`:38981`) has **two
arms**, and only one of them emits the owned ABI:

| arm | routes to | emits `semio_owned_*_v1`? |
| --- | --- | --- |
| `extension_exports!($bundle_fn, $plugin_fn, $app)` | `__semio_plugin_actor_exports!` (`:38509`) | **yes** — all nine (`:38522-38618`) |
| `extension_exports!($bundle_fn)` | `__semio_actor_exports!` (`:38412`) **directly** | **no** |

`plugin_exports!` (`:38502`) always routes through `__semio_plugin_actor_exports!`, which is why
every *plugin* describes. Every extension in the tree uses the **single-argument** arm — verified on
`📐️spatial-shape/🦀️.rs:49` and `🏢️aec-building/🦀️.rs:113`, both literally
`semio_framework_plugin::extension_exports!(bundle);` — so every extension component carries the WIT
guest exports and none of the nine core exports. `describeExtensionComponent`
(`🖨️describe/🏭️fresh-component/🟦️.ts`) forwards straight to `describePluginComponent`, which runs the
owned interpreter, whose `OwnedSemioArtifact::from_component`
(`🧠️interpreter/🦀️.rs:625-631`) requires **all nine** exports with exact core types.

### 3b.3 Blast radius — this is 16 of the 29 catalog skips

| owner | descriptor state | explained by 3b.2 |
| --- | --- | --- |
| `cad-extension-{spatial-shape, aec-building-energy, aec-building-structure}` | stale 09-15, **re-describe fails today** (measured) | yes |
| `imperative-extension-{control,effect,logic,math,text}` | `NotFound` | yes (A1 measured `text` failing identically) |
| `process-extension-{concrete,metal,robotic,wood}` | `NotFound` | yes |
| `sourcing-module-{beams,slabs,windows}` | `NotFound` | yes |
| `playbook-module-procedural` | `NotFound` | yes |
| **total** | | **16** |

The 14 committed extension descriptors that *do* decode (`🏢️aec-building` 09-16, the ten `🌊️flow`
extensions 09-15) are **legacy artifacts from before the emitter moved onto the owned interpreter**.
They are not evidence that the path works; they are evidence of when it stopped working. Any attempt
to refresh one of them fails the same way — `🏛️aec-building-structure`, whose sibling
`🏢️aec-building` decodes, is the controlled case and it failed at 14:54.

### 3b.4 The fix, and why this slice did not land it

The fix is to make the single-argument arm emit the owned ABI too — i.e. factor the nine
`semio_owned_*_v1` exports out of `__semio_plugin_actor_exports!` into a macro parameterised by
runtime + describe fn, and invoke it from both arms. The extension arm already has both pieces in
hand (`__SEMIO_EXTENSION_RUNTIME` and `__semio_describe_component`, `:39012-39022`), so the change is
mechanical.

**Not landed here, deliberately.** It changes the exported ABI surface of every extension component
in the tree, which means rebuilding 23 extension crates plus re-describing all 16 owners to prove it,
on a machine that was at load 24–61 for this entire slice and that OOM-killed a single component
build at 67 minutes (§2.2). Landing an ABI change I could not verify would be worse than leaving it
located. It is the single highest-value follow-up on this ticket: **one macro arm closes 16 of the
29 catalog skips**, and no amount of per-plugin `describe` work can close any of them.

---

## 4. Catalog health, before → after

### 4.1 Baseline, measured with the staged binary (no cargo) at 13:23

`🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp audit --folder <repo>` — WR2's staged 161 319 600 B
binary, 12:32 today. Capture `🗑️generated/a3-audit-before.txt`.

| | before |
| --- | --- |
| `[mcp registry] skipping plugin …` diagnostics (one registry load) | **29** |
| the same over the catalog-load + channel-routing pair (WR2 §8) | **58** |
| `semio-os-mcp audit: N finding(s) over M descriptor(s)` | **110 findings over 31 descriptors** |
| audit exit code | 1 |

### 4.2 A measurement bug found and fixed in `🐍️m5a-descriptor-census.ts`

The brief's baseline — "1866 capabilities, **0 described / 0 destructive**" — is **two different
facts, and the first one was wrong**. `🗒️note` had already been re-described by M5a at 11:51 and its
`🔣️.json` carries **40 descriptions, all of them bilingual EN+DE**, 14 declared audiences and 4
`destructive` marks (verified directly against the JSON, independent of the census). The census still
printed `described = 0 / en+de = 0` for it.

Cause, at `🐍️m5a-descriptor-census.ts:82-95`: a described capability serializes its description as
the `LocalizedLabel` envelope `{"native": {"en": "…", "de": "…"}}`, so the locale map is one level
down. `hasText` looked for string values at the **top** level and found an object; `localeKeys`
likewise returned `[]`. Both helpers therefore reported zero for exactly the descriptors a
regeneration has already fixed — the metric would have stayed at 0 no matter how many plugins this
slice landed.

Fixed at the root with one shared `localeMap()` helper that unwraps `native` (the only envelope the
emitter produces) and falls back to the bare map. **Re-measured, same tree, same minute:**

| metric (45 decoded descriptors, 1866 capabilities) | as printed before the fix | true value |
| --- | ---: | ---: |
| capabilities with a description | 0 | **40** |
| of those, EN+DE | 0 | **40** |
| capabilities with a declared `audience` | 14 | 14 (unaffected) |
| capabilities with `effects.destructive` | 4 | 4 (unaffected) |

All 40 come from the two plugins already re-described before this slice: `🗒️note` 30 (of 52
agent-facing) and `📕️norm` 10 (of 17). Every other plugin is genuinely 0. So outcome (b) is real and
**already demonstrated end to end on `🗒️note`** — M5a's source declarations do reach an agent the
moment the descriptor is regenerated, with no emitter change, exactly as M5a §5/§8.8 predicted. What
this slice was asked to do is repeat that for the other 44.

### 4.3 After — measured with the same binary and the same two scripts

Captured twice, 15:12 and 15:48 (identical — nothing landed after `🕸️dag`, see §2.4).
Captures: `🗑️generated/a3-audit-{mid,after}.txt`, `🗑️generated/a3-census-{mid,after}.txt`.
Largest descriptor after the sweep, by `ls -S`: **`🧩️puzzle/🔣️.json`, 4 803 294 B** — unchanged, and
still the only one over the bound.

| metric | before (13:23) | after (15:12) | delta |
| --- | ---: | ---: | ---: |
| `[mcp registry] skipping plugin …` (one registry load) | 29 | **22** | **−7** |
| the same over the catalog-load + routing pair (the ticket's "58") | 58 | **44** | **−14** |
| descriptors the audit could decode and inspect | 31 | **38** | **+7** |
| audit findings | 110 | **103** | −7 |
| capabilities in decodable descriptors | 1866 | 1872 | +6 |
| **with a description** | 40 | **155** | **+115** |
| **of those, EN+DE** | 40 | **155** | **+115** |
| **with a declared `audience`** | 14 | **47** | **+33** |
| **with `effects.destructive`** | 4 | **25** | **+21** |
| gesture routes published to agents undeclared | 54 | **35** | −19 |

Every one of those deltas is one thing: seven plugins re-described. Outcome (b) of the slice is
**demonstrated and moving** — M5a's source declarations reach an agent with no emitter change, as
predicted. Outcomes (a) and (c) are **not** met: 22 skips remain, 16 of them blocked by §3b's macro
arm and 6 by plugins not yet re-described, and `🧩️puzzle` is still over the 4 MiB bound.

---

## 5. Files changed

### 5.1 Generated descriptors regenerated through their own `describe` verb (never hand-edited)

Seven owners, each a `🔣️.json` + `🛂️.descriptor.semio` pair rewritten by
`bun ./📜️script.ts describe` in that plugin's own rust package:
`➗️mathematical`, `🎞️animate`, `🌿️vcs`, `📋️forms`, `🎬️sequence`, `💡️reasoning`, `🕸️dag`.

### 5.2 Source files changed

| file | change |
| --- | --- |
| `🎫️…/🐍️m5a-descriptor-census.ts` | §4.2 — new `localeMap()` helper; `hasText`/`localeKeys` now unwrap the `LocalizedLabel` `{"native":{…}}` envelope. Without it the census reports `described = 0` for every descriptor this slice regenerates. |

**No production source file was changed by this slice.** The two framework fixes this slice located
(§3 the describe profile / inlined example, §3b the `extension_exports!` arm) are both ABI- or
build-shape changes whose verification is a full re-describe sweep; both are written up with the
exact line so the next owner does not re-derive them.

### 5.3 New files in the ticket folder

| file | purpose |
| --- | --- |
| `📜️a3-describe.sh` | the describe driver — one owner at a time, through the fleet wasm mutex, bytes + wall time per run into `🗑️generated/a3-describe-ledger.txt` |
| `📓️a3-descriptor-regeneration.md` | this report |

Captures: `🗑️generated/a3-describe-ledger.txt`, `a3-describe-<owner>.txt` (11), `a3-batch1.txt`,
`a3-audit-before.txt`, `a3-audit-mid.txt`, `a3-census-before.txt`, `a3-census-mid.txt`.

---

## 6. Honest gaps

1. **7 of 46 committed descriptors regenerated; the catalog is at 22 skips, not 0.** Outcome (a) is
   not met. 16 of the 22 cannot be closed by any amount of `describe` work — they are §3b's macro
   arm — and 6 are plugins this slice did not reach.
2. **`🧩️puzzle` is still 4 803 294 B / 4 295 257 B pack, still over the 4 MiB bound.** Outcome (c)
   is not met. §3 names the declaration and both candidate fixes; neither was landed, and raising
   `DESCRIBE_DEADLINE_MS` is explicitly the wrong one.
3. **The sweep was ended by a peer's uncommitted refactor, not by scope** (§2.4):
   `semio-framework-ui` does not compile for `wasm32-wasip2` in the working tree, so **no plugin in
   the repo can be re-described right now**. `🪵️sourcing` burned 2 426 s discovering that;
   `🔱️trinity` and `🎪️demonstrator` were never attempted and this slice stopped its own batch by
   pid rather than hold the fleet mutex on doomed builds. `📜️a3-describe.sh` appends to
   `🗑️generated/a3-describe-ledger.txt` per plugin, so the rerun is `📜️a3-describe.sh <remaining …>`
   once that module builds again — read the ledger first rather than redoing the nine that landed.
4. **The 24 plugins never reached at all**: `🏛️architect`, `🧱️block`, `📐️cad`, `🖍️draw`, `🔋️energy`,
   `🏗️fem`, `🌊️flow` (+10 extensions), `🌍️gis`, `📏️layout`, `💠️lowpoly`, `🏭️process`, `🌀️procedural`,
   `🖨️raster`, `📸️remodel`, `🎥️shooting`, `🪐️space`, `🀄️wfc`, `✒️writer`. `🖍️draw` matters most — it is
   the plugin M5a annotated verb-by-verb with typed arguments, and its descriptor is still 09-18.
5. **Nothing here was proven against a live MCP client.** The measurements are the staged
   `semio-os-mcp audit` binary (WR2's, 12:32) and the two census scripts over the committed
   descriptors. M5a §8.7 step 4 (`live-agent-loop-check` with a live shell) was not run by this
   slice, so "an agent sees the descriptions" is proven at the catalog, not at the wire.
6. **The machine, not the work, set the budget.** One cold component build took 4 003 s and was then
   OOM-killed with swap at 25.6/26.6 GB; the same build warm took 444 s. Two of eleven `describe`
   runs spent their whole time queued. Any future estimate for this slice should assume the fleet
   mutex is the scheduler and budget one warm describe at ~60–700 s plus queueing.

---

## A3b — continuation, session 5d (2026-09-20 19:34 → 22:10). Full report: `📓️a3b-descriptor-sweep.md`

| A3's gap | state after A3b |
| --- | --- |
| §6.1 — 22 skips, 16 blocked by `extension_exports!` | **2 skips.** EX1's macro fix compiles (`📜️ex1-wasm-check.sh`, 3 crates `rc=0`) and extensions describe. Two further defects sat behind it: every extension emitted `packageId: ""` (`ExtensionBundle::new` never defaulted it) and none declared its host as `dependencies[0]` (`.extends` records the host but pushes no dependency). Fixed as one SDK default + 24 one-line `.depends_on(host, VersionReq::Any)` declarations. `playbook-module-procedural` was the same cause, not a separate one. |
| §6.2 — `🧩️puzzle` over the 4 MiB bound | **still over**, and now measured: with EX1's example-asset split landed the guest reached **4 081 418 370 fuel** (DS1: 1.64 G) and still hit the 1 800 s epoch deadline. The remaining cost is `capsule-dream`'s DSL parse at bundle install, not the descriptor write. |
| §6.3 — `semio-framework-ui` broken for `wasm32-wasip2` | **gone** — the peer gated `📥️input`'s retained-tree helpers at 16:21; every describe below compiled through it. |
| §6.4 — 24 plugins never reached | 33 owners re-described (24 in the main sweep, 2 stragglers, 9 `🌊️flow` extensions). **60 committed descriptors, 42 regenerated today.** `🗄️stdio`/`🌍️gis` deliberately untouched (GM1's). |
| §4.3 — catalog after | diagnostics **22 → 2**, decodable **38 → 58**, census **59/60 rows**, described **155 → 219**, audience **47 → 66**, destructive **25 → 44**, undeclared gesture routes **35 → 26**. |

Also landed: `🎞️animate`'s `setFrame`/`setSource` argument declarations (WR4 §5.4's red — a
`#[dsl(block)]` payload was unexpressible until `ActionArgDef::object` existed). `client-e2e`
**13/17**: both rows this slice owned are green, the three that replaced them are `energy`'s missing
component (building it makes the gate exceed its 240 s request wall — measured twice, artifact
removed again).
