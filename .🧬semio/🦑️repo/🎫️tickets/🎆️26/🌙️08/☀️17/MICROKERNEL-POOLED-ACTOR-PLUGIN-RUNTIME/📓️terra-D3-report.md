# 📓️ terra — D3-collisions-and-linkage — report

CARGO_TARGET_DIR used throughout: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎯️target-d3`

## Classification correction — read this first

D1 classified the `s.stdio.dwg@ac1018/*` collision as **cross-plugin**, between `procedural` and `gis`, and the coordinator's own brief to me repeated that framing ("Two plugins cannot own the same dialect coordinate"). **That framing is wrong.** Reading the actual code (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`'s `ArtifactDefinitionRegistry`/`PluginRuntimeRegistry`) shows the registry is instantiated **fresh per plugin build** (`let mut definitions = ArtifactDefinitionRegistry::new();` in the builder, one per `Plugin::builder(...)` call) — there is no merged, cross-plugin registry at build time. `process3d` (a different, single-artifact plugin) independently claims the *identical* literal `s.stdio.dwg@ac1018/*` string with zero conflict, which is only possible if the registry never sees `procedural`'s and `gis`'s claims at the same time.

The real defect is **intra-plugin**: `procedural` bundles two top-level artifacts (`procedural2d`, `procedural3d`) into one crate/registry, and `gis` bundles two (`gismap`, `gisterrain`) into one crate/registry. Within each plugin, **both** artifacts independently declared identical literal composer claims for `s.stdio.dwg@ac1018/*` — and, once I looked past just dwg, also for `s.stdio.json@rfc8259/*` and `s.stdio.png@1.2/*`. The registry rejects the second registration of an already-claimed literal value; which one wins is just declaration order (alphabetical by capability identity, since `ArtifactDefinition.capabilities` is a `BTreeMap`) — coincidentally `composer.dwg` sorts first among the overlapping names, which is why only dwg ever surfaced as the reported error even though json/png were equally broken underneath.

This is a materially different defect from "two plugins fighting over one dialect": it's two *artifacts inside one plugin* both declaring themselves the (sole) writer of a format neither of them writes for real. The fix targets artifact-level capability declarations inside each plugin, not any cross-plugin arbitration.

## (a) dwg/json/png collision — procedural, gis

**Root cause, confirmed from real build output** (before any fix, `.🧬semio/…/terra-D3-procedural-describe1.txt`, real command below, exit 0 but a placeholder descriptor):
```
$ CARGO_TARGET_DIR=…/🎯️target-d3 bun ✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📜️script.ts describe
...
described …/semio_s_plugin_procedural.wasm ("assembly-failed", role=Plugin) -> …/🛂️descriptor.semio + 🔣️descriptor.json
EXIT:0
```
`🔣️descriptor.json`'s `manifest.label` (the assembly error is smuggled through this field): `"dialect:s.stdio.dwg@ac1018/* is already registered by s.procedural2d.composer.dwg"`. D1's report cites the mirror error for gis: `"…already registered by s.gismap.composer.dwg"`.

**Investigation.** Both `procedural2d`/`procedural3d` (and `gismap`/`gisterrain`) declare a `composer.<format>` `ArtifactCapability` per stdio format in `export_stdio_kinds`, each carrying an identical literal `ArtifactIdentityClaim` (e.g. `dialect: "s.stdio.dwg@ac1018/*"`) — the claim value carries no per-artifact scoping, it's the bare interchange-format coordinate. `procedural2d` and `procedural3d` both list `stdio.dwg` in their `export_stdio_kinds`/`import_stdio_kinds` (DWG legitimately spans both 2D and 3D CAD content), and both also independently list `stdio.json`/`stdio.png` (generic bridge/raster formats). Same shape for `gismap`/`gisterrain`.

Read both plugins' actual DWG codec code (`🚪️io/📤️export/🧵️serializers/…/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs` and the matching `📥️import/🧩️deserializers` file) in all four artifacts: **every one of them is an equally fake stub** — `serialize_bytes` just calls `print_dsl` on the artifact's own snapshot and mislabels the result as DWG bytes; `deserialize`/`deserialize_bytes` discard the input and return `Snapshot::default()`. Same for the PNG export files I checked (also `print_dsl` stubs). None of the four artifacts has a real DWG or PNG codec — this rules out "read the fidelity of the implementation" as an ownership signal for those two formats.

**What does distinguish them, for DWG specifically:** `procedural`'s own plugin root (`✏️s/🔌️plugins/🌀️procedural/🦀️component.rs`) declares `.host_media_handler(HostMediaHandlerDeclaration::mesh_dwg_bridge("s.procedural.host-media.mesh-dwg", …procedural3d::artifact_kind()…, …procedural3d_document_from_mesh…))` — a real, independently-wired DWG↔mesh bridge, scoped explicitly to `procedural3d`. `procedural2d` has no such handler. That is genuine evidence, not a coin flip.

For `gis`: `gisterrain`'s own plugin-root doc comment (`✏️s/🔌️plugins/🌍️gis/🦀️component.rs`) states outright that `gisterrain` is "a composed CHILD artifact … never a standalone `ArtifactKindSpec`" and confirms via grep that `gisterrain`'s own `component.rs` defines no `artifact_kind()` function. Only `gismap` gets `.activation(ActivationEvent::OnArtifactKind{...})`, and `gismap` is the plugin's only artifact with a `host_media_handler` (SVG export). `gismap` is unambiguously the plugin's real, independently-exposed top-level artifact; `gisterrain` is not.

**Ownership rule applied, and where it is/isn't evidence-backed:**
- **DWG — evidence-backed.** `procedural3d` keeps the `composer.dwg` EXPORT claim (real `mesh_dwg_bridge` host-media handler). `procedural2d`'s duplicate `composer.dwg` capability + matching `EXPORT_DWG_DIALECT`/`compose_export_dwg` io-registry entry removed.
- **json/png — NOT evidence-backed, a documented tie-break.** Neither artifact has any real distinguishing signal for these two generic bridge formats (both codecs are equally `print_dsl` stubs). I kept `procedural2d`'s claims (the plugin's first-declared artifact) and removed `procedural3d`'s duplicates. This is explicitly called out in-source as a tie-break, not a justification — do not let this read as evidence-backed later.
- **gis (all three: dwg, json, png) — evidence-backed.** `gismap` is the plugin's real, independently-activated top-level artifact with the plugin's only host-media handler; `gisterrain` is a composed child, never independently activated. `gismap` keeps every shared EXPORT claim; `gisterrain`'s three duplicate capabilities + matching io-registry EXPORT entries removed.

**Import capability is untouched in every case** — only the EXPORT/composer claim was removed from the non-owning artifact. The `derived_composition::…::reads()` list (which drives *importing* that dialect into the artifact's own native snapshot) still includes the removed dialect on both sides; nothing about reading DWG/JSON/PNG changed. This keeps the change minimal and reversible.

**Files edited** (all within `path_scope`):
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🦀️component.rs` — removed `composer.dwg` capability; adjusted `export_stdio_kinds` (kept in `import_stdio_kinds`).
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — removed `EXPORT_DWG_DIALECT`/`compose_export_dwg` + its `entries()` row; adjusted free `export_stdio_kinds()`.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🦀️component.rs` — removed `composer.png`/`composer.json` capabilities; adjusted `export_stdio_kinds`.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — removed `EXPORT_PNG_DIALECT`/`compose_export_png` and `EXPORT_JSON_DIALECT`/`compose_export_json` + their `entries()` rows; adjusted free `export_stdio_kinds()`.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🦀️component.rs` — removed `composer.png`/`composer.json`/`composer.dwg` capabilities (gisterrain has no `export_stdio_kinds` on an `ArtifactKindSpec` — it isn't one).
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — removed the matching three `EXPORT_*_DIALECT`/`compose_export_*` + `entries()` rows; adjusted free `export_stdio_kinds()`.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/**` — **untouched**, keeps every claim.

## (b) weak-linkage duplicate symbol — draw, imperative

**draw — root cause confirmed from real linker output:**
```
$ CARGO_TARGET_DIR=…/🎯️target-d3 bun ✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📜️script.ts describe
...
error: symbol `semio_plugin_bundle_installer_link_shim` is already defined
  --> ✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📦️glue.rs:601:1
601 | semio_framework_plugin::plugin_exports!(plugin::plugin);
EXIT:101
```
Three definitions exist in the link graph: (1) the framework's own `#[cfg(feature="component-guest")] #[linkage="weak"]` default in `🔌️plugin/🦀️component.rs`; (2) the STRONG definition generated by `plugin_exports!` itself (also in the framework crate, unconditional `#[unsafe(no_mangle)]`, no `weak` linkage) via draw's own `glue.rs:601` call; (3) a hand-written duplicate directly in `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/…/✏️editor/🦀️component.rs`'s `//#region 🔗️StandaloneLinkage`, marked `#[linkage="weak"]`, doc-commented "satisfies the plugin runtime when this app is linked as its own WASM module."

That third one is dead weight: unlike `procedural`/`gis` (which declare a `plugin-entry` Cargo feature, default-on, and gate their own `plugin_exports!` call behind it specifically so an *embedding* crate like `demonstrator` can turn it off), `draw` has no `plugin-entry` feature at all and its `glue.rs` calls `plugin_exports!` **unconditionally**. Draw is never embedded (not in demonstrator's embed list). So draw's own crate already provides the strong symbol every time it's built, standalone or not — the "standalone" fallback shim in editor/component.rs was solving a problem draw doesn't have, and its own presence is what breaks the link.

**Fix:** deleted the whole `//#region 🔗️StandaloneLinkage` block (editor/component.rs) and the now-orphaned `#![cfg_attr(target_arch="wasm32", feature(linkage))]` crate attribute + its explanatory comment in `glue.rs` (the only remaining use of the unstable `linkage` feature was that block).

**Verified, real:**
```
$ CARGO_TARGET_DIR=…/🎯️target-d3 bun ✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📜️script.ts describe
described …/semio_s_plugin_draw.wasm ("draw", role=Plugin) -> …/🛂️descriptor.semio + 🔣️descriptor.json
EXIT:0
```
```
$ CARGO_TARGET_DIR=…/🎯️target-d3 cargo test -p semio-s-plugin-draw --lib
test result: ok. 105 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
(includes: test descriptor_is_fresh ... ok)
EXIT:0
```
Ratcheted `draw` into `DESCRIPTOR_MIGRATED_PLUGINS` (below). **This verification is real and pre-dates the peer churn described in Peer-coexistence** — it was clean before the environment became volatile, and stands on its own.

**imperative — root cause, from D1's citation plus my own reading (not independently re-reproduced pre-fix — see caveat below):** D1 located the same symbol class, `semio_extension_bundle_installer_link_shim`, first surfacing at `…/🧩️extensions/🧮️math/🦀️component.rs:157`. Reading the structure: `imperative`'s own `📦️glue.rs` (`✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/📦️glue.rs:410-419`) `#[path]`-mounts **all five** extension `🦀️component.rs` files (`effect`, `math`, `text`, `logic`, `control`) directly as inline submodules of the `imperative` crate — and each of those five files independently calls `semio_framework_plugin::extension_exports!(bundle)` **unconditionally** (only gated `#[cfg(target_arch = "wasm32")]`, confirmed identical across all five). Each of those five crates *also* exists as its own standalone crate (own `Cargo.toml`, own `cdylib`, e.g. `semio-s-plugin-imperative-math`) with a thin `glue.rs` that `#[path]`-mounts the exact same `🦀️component.rs`. So compiling `semio-s-plugin-imperative` for `wasm32-wasip2` compiles the SAME unconditional `extension_exports!` invocation five times into one crate — five strong definitions of `semio_extension_bundle_installer_link_shim`.

**Fix, mirroring procedural's existing `plugin-entry` pattern:** added a new Cargo feature `extension-entry` (`default = ["extension-entry"]`, `extension-entry = []`) to each of the five extension crates' own `Cargo.toml`; changed each `component.rs`'s export line from `#[cfg(target_arch = "wasm32")] semio_framework_plugin::extension_exports!(bundle);` to `#[cfg(all(target_arch = "wasm32", feature = "extension-entry"))] …`. Each extension's own standalone build still has the feature on by default (unaffected). `imperative`'s own `Cargo.toml` declares no such feature, so when these files are mounted inline into `imperative`'s crate the cfg check is simply false there — added `#![allow(unexpected_cfgs)]` to `imperative`'s `glue.rs` (same pattern draw already carries for its own now-removed `linkage` feature) so that doesn't become a hard error under `-D warnings`.

**Verification caveat — be honest about this:** I never got a clean, dedicated pre-fix reproduction of imperative's duplicate-symbol error myself (D1's citation is the primary source for the exact symbol/location). My own first `describe` run against `imperative` was started *before* editing its source, but ran long enough (10m17s to reach the framework/description-tool compile step) that my source edits landed on disk before cargo actually reached `semio-s-plugin-imperative`'s own compilation — the log shows `Compiling semio-s-plugin-imperative …` with **zero** subsequent `error:` lines, i.e. it appears to have compiled *cleanly with the fix already applied*, not as a pre-fix baseline. The run's overall exit code was still 1, but that traces to a concurrent `rm -rf …/incremental` I ran (see Peer-coexistence/self-inflicted section) deleting the freshly-built `.wasm` before the describe tool could read it — not to a compile failure. I could not get a second, clean, dedicated run (pre- or post-fix) before the environment-wide blocker below made every `wasm32-wasip2` build across all five of my plugins fail identically. **Net honest position: the fix is structurally sound and reuses an established, working pattern (procedural's `plugin-entry`), and the one build that did reach imperative's own compilation shows no linker error — but I do not have a clean, dedicated, pasted pre/post-fix pair for imperative the way I do for draw. Re-verify once the environment settles.**

## (c) energy — no wasm artifact

`✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/Cargo.toml`'s `[lib]` section had `path = "📦️glue.rs"` and **no `crate-type` key at all** — default is `["lib"]` (rlib only), so no `cdylib`/`.wasm` is ever produced, matching D0's and D1's prior finding. Compared against `🗒️note` (a working plugin): `[lib] crate-type = ["cdylib", "rlib"] path = "📦️glue.rs"`. Added the identical `crate-type` line to energy's `Cargo.toml`. **Not yet build-verified** — every attempt (one dedicated run) hit the same environment-wide blocker described below before reaching a real pass/fail signal for energy specifically.

## Per-plugin table

| plugin | class | root cause found | fix applied | describe exit code | `cargo test --lib descriptor_is_fresh` | committed? | ratcheted? |
|---|---|---|---|---|---|---|---|
| 🖍️draw | (b) weak-linkage duplicate symbol | yes, real linker output pasted above | yes — removed dead duplicate weak shim + orphaned `linkage` feature attr | **0** (real, pasted) | **ok**, 105/105 (real, pasted) | working-tree (auto-commit bot), not manually | **yes** |
| 🌀️procedural | (a) intra-plugin dwg/json/png collision, reclassified from D1's cross-plugin framing | yes, real descriptor label pasted above | yes — removed procedural2d's `composer.dwg`, procedural3d's `composer.png`/`composer.json` (+ matching io-registry entries) | **101**, environment-blocked (pasted below, not a code failure) | not run — blocked before reaching test compile | not this session | **no** |
| 🌍️gis | (a) same, gismap/gisterrain pair | yes, cited from D1 + same code pattern independently confirmed | yes — removed gisterrain's `composer.dwg`/`composer.json`/`composer.png` (+ matching io-registry entries) | **101**, environment-blocked (pasted below) | not run — blocked | not this session | **no** |
| 📜️imperative | (b) weak-linkage duplicate symbol, 5× `extension_exports!` in one crate | yes, D1 citation + independent structural analysis | yes — `extension-entry` feature gate on all 5 extensions, mirroring procedural's `plugin-entry` | ambiguous run only (see caveat above) | not run — blocked | not this session | **no** |
| 🔋️energy | (c) no `crate-type` | yes, confirmed by reading `Cargo.toml` | yes — added `crate-type = ["cdylib", "rlib"]` | **101**, environment-blocked (pasted below) | not run — blocked | not this session | **no** |

**Every "101, environment-blocked" row above is pasted verbatim in the section below — none of them are my dwg/json/png/crate-type code failing; every one of them is the same repo-wide, unrelated `🎒️pack` blocker.**

## Foreground verification — real commands and exit codes (as demanded)

All run in the foreground, `CARGO_TARGET_DIR=…/🎯️target-d3`, sequentially, after killing stray duplicate background processes I had incorrectly left running from an earlier (corrected) attempt at this packet — see Peer-coexistence for the self-inflicted mess and the fix.

```
$ cargo test -p semio-s-plugin-procedural --lib
… 44 compile errors, e.g.:
error[E0599]: no method named `dispatch` found for enum `Result<T, E>` in the current scope
  --> …procedural3d/…/🧬️mutations/💾️binary/🦀️component.rs:256:15
REAL EXIT: 101
```
This is **not caused by my fix**. `store::ArtifactStore::<P,Mutation>::new(envelope)` returns `Result<Self, VcsError>` (confirmed at `🧰️framework/…/🏪️store/🦀️component.rs:4350`); procedural2d's and procedural3d's own native `document_text_round_trip_with_operation_applied` tests (and, per the full error list, several other framework files — `📡️replication`/`📡️wire`, more of `🏪️store` itself) never adapted. This is the same class of bug D1 already fixed once in `animate` ("Native Cargo Misses Wasm-Gated Code" — nobody had run native `cargo test` on this file before). It's real, it's in `procedural`'s own path_scope for the two obvious call sites, but the full error list also names framework files outside any single plugin's path_scope, so it's **larger than a two-line fix and outside this packet's charter**. Flagged as a separate task (`task_08edf1ba`) rather than patched blind.

Because the whole `--lib` test binary must compile before any single test (including `descriptor_is_fresh`) can run, this blocks `descriptor_is_fresh` verification for `procedural` entirely, independent of anything in this packet.

```
$ CARGO_TARGET_DIR=…/🎯️target-d3 bun ✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📜️script.ts describe
   Compiling semio-framework-os-kernel v0.1.0 (…)
error: couldn't read `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🎒️pack/⏳️async/🦀️component.rs`: No such file or directory (os error 2)
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs:102:3
error: could not compile `semio-framework-os-kernel` (lib) due to 1 previous error
REAL EXIT: 101
```
Reproduced **identically** across 5 separate procedural retries, 1 energy retry, and 1 gis retry (all pasted to `terra-D3-procedural-describe-fg{3,4,5}.txt`, `terra-D3-energy-describe-fg.txt`, `terra-D3-gis-describe-fg.txt`). Confirmed via `git status --porcelain` this is a live peer's in-progress relocation of the `🎒️pack` module (files newly present under the top-level `🧰️framework/🔨️modules/🎒️pack/`, deleted-but-unstaged under the os-product-local `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/`, and `…/os/📦️packages/🦀️rust/📦️glue.rs`'s `#[path]` mounts never updated to the new location) — entirely outside this packet's `path_scope`, and per my brief's own instruction ("only fix it if it is dead… never if it is moving") I did not touch it. `semio-framework-os-kernel` is a near-universal dependency, so this blocks essentially every plugin's `wasm32-wasip2` build repo-wide right now, not just mine. Flagged as `task_020dd98c`.

**This blocker is why I cannot currently paste a clean post-fix `describe` exit-0 for procedural, gis, or energy, and why the `imperative` verification above is ambiguous rather than clean.** The registry-claim code changes for (a) and the `crate-type` line for (c) are unreachable by any build right now — not rejected by them.

## Peer-coexistence

- **Liveness-checked all 5 owned plugins before touching them** (`git log --date=iso --oneline -3` + mtime): all showed only the stale `🌙️06☀️04` history, no plugin directory had any file with mtime in the last 30 minutes at the time I started. Proceeded on all 5.
- **`CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`**: never touched a declaration channel (`.artifact(declaration())` / `.declare_artifact(artifact())`), never converted any plugin between channels. All edits were pruning already-declared `.capability(...)` entries and their matching io-registry rows within the existing channel shape.
- **Transient, self-resolved churn**: mid-session, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📦️glue.rs` briefly referenced `semio_framework::ASSEMBLY_FAILED_PLUGIN_ID` before that constant existed in `semio_framework` (a peer adding a "refuse to write assembly-failed placeholder descriptors" hardening, mid-edit, ~30–60s window). Caused one describe run to fail with `E0425: cannot find value ASSEMBLY_FAILED_PLUGIN_ID`. Did not touch the file (outside path_scope); retried once the constant appeared (`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:4562`) and the error cleared.
- **Persistent, unresolved churn**: the `🎒️pack` module relocation described above. Confirmed present at the start of this section and still present as of the last retry. Flagged as `task_020dd98c`, not fixed.
- **A third, unrelated, out-of-scope discovery**: `ArtifactStore::new()`'s `Result`-unwrap fallout in procedural's own native tests (+ some framework files). Flagged as `task_08edf1ba`, not fixed — too large for this packet's charter and touches files outside `path_scope`.

## Self-inflicted mess, corrected mid-session (full honesty per coordinator's note)

I initially launched `describe` for procedural/gis/energy/imperative as backgrounded commands and moved on, which the coordinator correctly called out — those results were at risk of never being consulted. Two direct consequences, both now cleaned up:
1. A background `gis describe` run was corrupted when I ran `rm -rf …/incremental` **while it was still writing to that directory** ("failed to move dependency graph… No such file or directory") — pure self-inflicted race, not a code issue. Discarded that output, do not cite it.
2. I ended up with duplicate overlapping `describe` invocations for `procedural` and `energy` (one stale background one from my first pass, one fresh foreground one) competing for the same `cargo` target-dir lock. Identified via `ps aux`/`lstart`, killed the stale ones (`kill -9`), confirmed via `ps` that only my intended foreground process remained before continuing.

All results cited in this report from that point forward are from single, foreground, sequentially-run commands with `; echo REAL EXIT: $?` captured directly (never through a pipe).

## Ratchet — `DESCRIPTOR_MIGRATED_PLUGINS`

Liveness-checked `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` before editing (mtime 3+ hours old, no staged changes at edit time). Edited **only** the array literal:
```rust
const DESCRIPTOR_MIGRATED_PLUGINS: &[&str] = &["note", "sequence", "vcs", "forms", "sourcing", "dag", "mathematical", "writer", "reasoning-mindmap", "animate", "draw"];
```
Only `draw` added — the only one of my five with a real, passing, freshly-verified `descriptor_is_fresh`. `procedural`, `gis`, `energy`, `imperative` are explicitly **not** ratcheted; their fixes are applied and code-reviewed for internal consistency but not build/test-verified due to the environment-wide blocker above.

## Disk stewardship

Pruned `🎯️target-d3/{debug,wasm32-wasip2/debug}/incremental` and stale top-level `.wasm` once mid-session (after procedural's first successful build, before the churn started) — went from ~90 GB free back up to ~119 GB. Monitored `df -g /System/Volumes/Data` throughout (dropped as low as ~73 GB during peak concurrent peer activity, ~40-50 other `rustc` processes running repo-wide); never approached the 60 GB stop-line.

## Files touched

- **Edited** (all within `path_scope`): the 6 procedural/gis files and 5+1 imperative files listed under (a)/(b)/(c) above; `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (`DESCRIPTOR_MIGRATED_PLUGINS` line only).
- **Not touched, explicitly out of scope**: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` and `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/**` (live peer relocation, `task_020dd98c`); the framework/replication files implicated in the `ArtifactStore::new()` fallout (`task_08edf1ba`); `gismap`'s own files (kept fully intact, it is the evidence-backed owner).
- **Scratch/logs in ticket folder**: `terra-D3-procedural-describe{1,-fg,-fg2,-fg3,-fg4,-fg5}.txt`, `terra-D3-gis-describe{1,-fg}.txt`, `terra-D3-energy-describe{1,-fg}.txt`, `terra-D3-imperative-describe1.txt`, `terra-D3-draw-describe{1,2}.txt`, `terra-D3-draw-test1.txt`, `terra-D3-procedural-libtest-fg.txt`. Two files are known-garbage from the self-inflicted mess above and should not be cited: `terra-D3-procedural-describe2.txt` (killed mid-build, no real exit line), `terra-D3-energy-describe1.txt` (`EXIT:137` = SIGKILL, killed mid-build by me).

## Lease-requests

None. All edits stayed within `path_scope`. The two out-of-scope discoveries (pack-module relocation, `ArtifactStore::new()` fallout) were flagged as background tasks rather than requested as leases, since neither is something this packet should absorb — the first needs its owning peer session, the second needs its own scoped packet.

## What's left, honestly

- Re-run `describe` + `cargo test --lib descriptor_is_fresh` for `procedural`, `gis`, `energy`, `imperative` once `task_020dd98c` (pack relocation) lands — the fixes in this report should then produce clean exit-0s, but that is a prediction, not a verified fact, and must be re-checked before anyone ratchets these four.
- `procedural`'s `cargo test --lib` additionally needs `task_08edf1ba` resolved before `descriptor_is_fresh` can even compile, independent of the pack blocker.
- `imperative`'s duplicate-symbol fix has the weakest verification of the four (see caveat in section (b)) — prioritize re-testing it first once unblocked.
