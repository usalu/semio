# U3b — `LocalizedLabel` migration: whole tree type-checks (native + wasm32)

Slice U3b of ticket 26/09/18. Continues `📓️u3-mutation-label-localisation.md` (§5 gap 1: "107 crates
carry rewritten sites; 4 crates were checked").

Status: **landed**. 2026-09-21.

## 0. Inherited state

* U3's codemod rewrote 2795 sites in 2791 files. Crate mapping recomputed here from
  `🗑️generated/u3-codemod-per-file.txt` by longest-prefix match against every `[package]`
  `Cargo.toml` owner dir: **106 crates** (U3's note says 107; the list is
  `🗑️generated/u3b-crate-list.txt`).
* TC3b §6(h): `cargo check -p semio-framework-plugin --all-targets` RED, 23 `dsl::LocalizedLabel`
  errors in two test files.
* HT13 §6.2: kernel was briefly red 04:03–04:05 on `fn label(&self) -> String` leaves; reported
  landed by 10:28.

## 1. Framework root (`semio-framework-plugin --all-targets`)

`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-u3b cargo check -p semio-framework-plugin
--all-targets --keep-going`

| run | result | capture |
|---|---|---|
| before | **24 errors** (23 `LocalizedLabel` + 1 downstream), 363 warnings | `🗑️generated/u3b-check1-framework-plugin.txt` (overwritten by the after-run) |
| after | **exit 0**, 368 warnings (`semio-framework-plugin (lib test) generated 339 warnings (35 duplicates)`) | `🗑️generated/u3b-check1-framework-plugin.txt` |

### 1.1 Root cause A — the codemod wrapped a *delegating* `label()` in `native(&x, &x)`

Three `CompositeMutationKind` fixtures delegated to their own `MutationKind::label`. The codemod
treated the delegation as if it were an English literal and emitted
`LocalizedLabel::native(&<Self as MutationKind<..>>::label(self), &<Self as ..>::label(self))` — a
type error (`expected &str, found &LocalizedLabel`) **and**, had it compiled, unbounded recursion.
Fixed by delegating the carrier straight through:

* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations/➕️add-value/🦀️.rs:44`
* `…/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧫️fixtures/🧬️job-test-mutations/🧬️mutations/➕️add-value/🦀️.rs:36`
* `…/🔌️plugin/🧫️fixtures/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🦀️.rs:51`

A tree-wide grep for `LocalizedLabel::native(&<Self as` finds exactly these three, so the shape is
now extinct.

### 1.2 Root cause B — tests still read the label as a `&str`

`LocalizedLabel` deliberately has no `as_str`, no `contains`, no `From<&str>` (that is the gate).
The assertions were re-authored to name the locale they read instead of being cast back:

| file:line | fix |
|---|---|
| `…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:10` | new module helper `label_in(&LocalizedLabel, Locale) -> &str` (`resolve(Terminology::Native, locale)`) |
| same, 4669/4786/4787/4864/4917 | `entry.label.as_str()` / `.contains(..)` → `label_in(&entry.label, Locale::En)` |
| same, 4961 | `increment` row asserted in **both** locales — proves the no-definition fallback is locale-invariant `data`, not untranslated English |
| same, 5058/5073/5129 | `CommandView.label: "Increment"/"Undo"/"Fill".into()` → `LocalizedLabel::native(en, de)` |
| same, 5160/5194 | oversized/windowed fixture rows → `LocalizedLabel::data(..)` (generated filler, no translation to claim) |
| same, 5346 | `entry.label.contains("dark")` → `label_in(.., Locale::En).contains("dark")` |
| same, +5357 | **new test** `a_shell_noted_row_carries_the_same_text_in_every_locale` — asserts all 4 cells |
| `…/🔌️plugin/🧪️tests/🛰️declaration-channels-unit/🦀️.rs:25` | `== "Set value to -1"` → `== LocalizedLabel::native("Set value to -1", "Wert auf -1 setzen")` |
| `…/🔌️plugin/🧪️tests/🔬️plugin-runtime-contributed-mutation-wire/🦀️.rs:49` | wire result asserted as the full matrix — proves the German cell crosses the wire |
| `…/🔌️plugin/🧪️tests/📡️contributed-mutation-wire-unit/🦀️.rs:95` | same, composite-trait call site |
| `…/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution-unit/🦀️.rs:106` | same |
| `…/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🔬️unit/🦀️.rs:51` | job-wire result asserted as the full matrix |
| `…/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations-mutations-add-value-unit/🦀️.rs:98` | same |
| `…/🔌️plugin/🧪️tests/🔬️app-panel-kit/🦀️.rs:218,628` | `label: format!("Move {seq}")` → `LocalizedLabel::data(..)` |

This closes TC3b `📓️tc3b-catalog-genesis-landed.md` §6(h).

## 2. 106-crate native sweep

Eleven batches of ≤ 10 crates, ONE cargo at a time, foreground, private
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-u3b` (shared build-dir, rule 25). The
script is `📜️u3b-sweep.sh <first> <last>`, the batch plan `🗑️generated/u3b-batches.json`.

`--features component-app-assembly` is passed only for the 57 crates that declare it: with several
`-p` and no selected package carrying the feature, cargo hard-errors
(`none of the selected packages contains this feature`) instead of ignoring it, so the 49 that do
not declare it are checked in their own batches without it.

**Result: 102 of 106 green, 1 red-then-fixed by me, 3 red in peers' in-flight files (§7.1). Zero
`LocalizedLabel` errors remain anywhere in the sweep.**

Warnings are quoted as proof the type-checker reached the code: batch totals 236 / 142 / 138 / 128 /
65 / 124 / 98 / 123 / 130 / 169 / 137 warnings.

### 2.1 The one real label defect the sweep found

`semio-s-artifact-sequence-sequence` priced its fixed retained envelope with `label.len()`:

* `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1180`
  — `admit(label.len())?` over `ChildEmit.labels`, which U3 turned into `Vec<LocalizedLabel>`.

Fixed at the root rather than at the call site: `LocalizedLabel` now answers what it really costs.

* `🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale/🏷️label/🦀️.rs` — new
  `LocalizedLabel::retained_bytes()`, the sum of every cell. A caller that used to admit
  `label.len()` admits this, so the envelope is never under-priced by counting one locale.
* the sequence editor line above now calls it.

This is not cosmetic: the old code, had it compiled via a cast, would have admitted roughly a
quarter of the bytes the carrier actually retains.

### 2.2 Second relocation defect (found by §3's `--all-targets` run)

U3 moved `🌐️locale/🧾️value` down into the kernel but left its `#[path]` test mount spelled for the
old depth, so `semio-framework-os-kernel`'s **lib test** target could not even be read
(`couldn't read …/🧪️tests/🔬️targets-wgpu-locale-terminology-value-locale-terminology-value-round-trip/🦀️.rs`).
A plain `cargo check` never touches it, which is why U3's four checks were green.

* `🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale/🧾️value/🦀️.rs:48` — `#[path]` retargeted at
  `../../../../../🔨️modules/🖱️ui/🧪️tests/…`, matching the sibling `🏷️label` module's already-correct
  spelling.

### 2.3 Crate table

| crate | sites | `component-app-assembly` | batch | check | capture |
|---|---|---|---|---|---|
| `semio-s-artifact-stdio-semio` | 233 | yes | 1 | **green** | `🗑️generated/u3b-sweep-batch1.txt` |
| `semio-s-artifact-stdio-pdf` | 148 | yes | 1 | **green** | `🗑️generated/u3b-sweep-batch1.txt` |
| `semio-s-artifact-stdio-gltf` | 120 | yes | 1 | **green** | `🗑️generated/u3b-sweep-batch1.txt` |
| `semio-s-artifact-block-5d` | 41 | yes | 1 | **green** | `🗑️generated/u3b-sweep-batch1.txt` |
| `semio-s-artifact-stdio-step` | 38 | yes | 1 | **green** | `🗑️generated/u3b-sweep-batch1.txt` |
| `semio-s-artifact-block-3d` | 38 | yes | 1 | **green** | `🗑️generated/u3b-sweep-batch1.txt` |
| `semio-s-artifact-puzzle-5d` | 36 | yes | 1 | **green** | `🗑️generated/u3b-sweep-batch1.txt` |
| `semio-s-artifact-puzzle-3d` | 36 | yes | 1 | **green** | `🗑️generated/u3b-sweep-batch1.txt` |
| `semio-s-artifact-puzzle-2d` | 34 | yes | 1 | **green** | `🗑️generated/u3b-sweep-batch1.txt` |
| `semio-s-artifact-stdio-gif` | 31 | yes | 1 | **green** | `🗑️generated/u3b-sweep-batch1.txt` |
| `semio-s-artifact-procedural-generation3d` | 30 | yes | 2 | **green** | `🗑️generated/u3b-sweep-batch2.txt` |
| `semio-s-artifact-fem-3d` | 30 | yes | 2 | **green** | `🗑️generated/u3b-sweep-batch2.txt` |
| `semio-s-artifact-stdio-ifc` | 30 | yes | 2 | **green** | `🗑️generated/u3b-sweep-batch2.txt` |
| `semio-s-artifact-fem-2d` | 29 | yes | 2 | **green** | `🗑️generated/u3b-sweep-batch2.txt` |
| `semio-s-artifact-stdio-svg` | 28 | yes | 2 | **green** | `🗑️generated/u3b-sweep-batch2.txt` |
| `semio-s-artifact-stdio-docx` | 26 | yes | 2 | **green** | `🗑️generated/u3b-sweep-batch2.txt` |
| `semio-s-artifact-block-2d` | 26 | yes | 2 | **green** | `🗑️generated/u3b-sweep-batch2.txt` |
| `semio-s-artifact-stdio-pptx` | 24 | yes | 2 | **green** | `🗑️generated/u3b-sweep-batch2.txt` |
| `semio-s-artifact-stdio-xlsx` | 23 | yes | 2 | **green** | `🗑️generated/u3b-sweep-batch2.txt` |
| `semio-s-artifact-stdio-obj` | 21 | yes | 2 | **green** | `🗑️generated/u3b-sweep-batch2.txt` |
| `semio-s-artifact-wfc-2d` | 19 | yes | 3 | **green** | `🗑️generated/u3b-sweep-batch3.txt` |
| `semio-s-artifact-stdio-jpg` | 19 | yes | 3 | **green** | `🗑️generated/u3b-sweep-batch3.txt` |
| `semio-s-artifact-wfc-3d` | 18 | yes | 3 | **green** | `🗑️generated/u3b-sweep-batch3.txt` |
| `semio-s-artifact-gis-gismap` | 18 | yes | 3 | **green** | `🗑️generated/u3b-sweep-batch3.txt` |
| `semio-s-artifact-stdio-dxf` | 18 | yes | 3 | **green** | `🗑️generated/u3b-sweep-batch3.txt` |
| `semio-s-artifact-procedural-generation2d` | 15 | yes | 3 | **green** | `🗑️generated/u3b-sweep-batch3.txt` |
| `semio-s-artifact-stdio-png` | 15 | yes | 3 | **green** | `🗑️generated/u3b-sweep-batch3.txt` |
| `semio-s-artifact-wfc-grid2d` | 14 | yes | 3 | **green** | `🗑️generated/u3b-sweep-batch3.txt` |
| `semio-s-artifact-wfc-grid3d` | 14 | yes | 3 | **green** | `🗑️generated/u3b-sweep-batch3.txt` |
| `semio-s-artifact-stdio-las` | 14 | yes | 3 | **green** | `🗑️generated/u3b-sweep-batch3.txt` |
| `semio-s-artifact-stdio-xml` | 14 | yes | 4 | **green** | `🗑️generated/u3b-sweep-batch4.txt` |
| `semio-s-artifact-stdio-tiff` | 14 | yes | 4 | **green** | `🗑️generated/u3b-sweep-batch4.txt` |
| `semio-s-artifact-stdio-json` | 14 | yes | 4 | **green** | `🗑️generated/u3b-sweep-batch4.txt` |
| `semio-s-artifact-trinity-jack` | 13 | yes | 4 | **green** | `🗑️generated/u3b-sweep-batch4.txt` |
| `semio-s-artifact-stdio-zip` | 13 | yes | 4 | **green** | `🗑️generated/u3b-sweep-batch4.txt` |
| `semio-s-artifact-stdio-bcf` | 13 | yes | 4 | **green** | `🗑️generated/u3b-sweep-batch4.txt` |
| `semio-s-artifact-stdio-epw` | 12 | yes | 4 | **green** | `🗑️generated/u3b-sweep-batch4.txt` |
| `semio-s-artifact-stdio-avi` | 12 | yes | 4 | **green** | `🗑️generated/u3b-sweep-batch4.txt` |
| `semio-s-artifact-wfc-bitmap` | 11 | yes | 4 | **green** | `🗑️generated/u3b-sweep-batch4.txt` |
| `semio-s-artifact-trinity-rewriting` | 9 | yes | 4 | **green** | `🗑️generated/u3b-sweep-batch4.txt` |
| `semio-s-artifact-stdio-html` | 9 | yes | 5 | **green** | `🗑️generated/u3b-sweep-batch5.txt` |
| `semio-s-artifact-stdio-mp4` | 9 | yes | 5 | **green** | `🗑️generated/u3b-sweep-batch5.txt` |
| `semio-s-artifact-stdio-ply` | 9 | yes | 5 | **green** | `🗑️generated/u3b-sweep-batch5.txt` |
| `semio-s-artifact-stdio-tsv` | 6 | yes | 5 | **green** | `🗑️generated/u3b-sweep-batch5.txt` |
| `semio-s-artifact-stdio-stl` | 6 | yes | 5 | **green** | `🗑️generated/u3b-sweep-batch5.txt` |
| `semio-s-artifact-stdio-csv` | 5 | yes | 5 | **green** | `🗑️generated/u3b-sweep-batch5.txt` |
| `semio-s-artifact-stdio-md` | 5 | yes | 5 | **green** | `🗑️generated/u3b-sweep-batch5.txt` |
| `semio-s-artifact-stdio-txt` | 5 | yes | 5 | **green** | `🗑️generated/u3b-sweep-batch5.txt` |
| `semio-s-artifact-stdio-bmp` | 5 | yes | 5 | **green** | `🗑️generated/u3b-sweep-batch5.txt` |
| `semio-s-artifact-stdio-mp3` | 4 | yes | 5 | **green** | `🗑️generated/u3b-sweep-batch5.txt` |
| `semio-s-artifact-stdio-binary` | 4 | yes | 6 | **green** | `🗑️generated/u3b-sweep-batch6.txt` |
| `semio-s-artifact-stdio-wav` | 4 | yes | 6 | **green** | `🗑️generated/u3b-sweep-batch6.txt` |
| `semio-s-artifact-stdio-deflate` | 4 | yes | 6 | **green** | `🗑️generated/u3b-sweep-batch6.txt` |
| `semio-s-artifact-space-space` | 4 | yes | 6 | **green** | `🗑️generated/u3b-sweep-batch6.txt` |
| `semio-s-artifact-gis-gisterrain` | 3 | yes | 6 | **green** | `🗑️generated/u3b-sweep-batch6.txt` |
| `semio-s-artifact-stdio-dwg` | 2 | yes | 6 | **green** | `🗑️generated/u3b-sweep-batch6.txt` |
| `semio-s-artifact-space-home` | 1 | yes | 6 | **green** | `🗑️generated/u3b-sweep-batch6.txt` |
| `semio-s-artifact-energy-model` | 291 | — | 7 | **green** | `🗑️generated/u3b-sweep-batch7.txt` |
| `semio-s-artifact-architect-program` | 268 | — | 7 | **green** | `🗑️generated/u3b-sweep-batch7.txt` |
| `semio-s-artifact-norm-din16798` | 62 | — | 7 | **green** | `🗑️generated/u3b-sweep-batch7.txt` |
| `semio-s-artifact-norm-en1998` | 49 | — | 7 | **green** | `🗑️generated/u3b-sweep-batch7.txt` |
| `semio-s-artifact-shooting-shooting` | 39 | — | 7 | **green** | `🗑️generated/u3b-sweep-batch7.txt` |
| `semio-s-artifact-remodel-remodeling` | 36 | — | 7 | **green** | `🗑️generated/u3b-sweep-batch7.txt` |
| `semio-s-artifact-norm-en1992` | 35 | — | 7 | **green** | `🗑️generated/u3b-sweep-batch7.txt` |
| `semio-s-artifact-note-note` | 34 | — | 7 | **green** | `🗑️generated/u3b-sweep-batch7.txt` |
| `semio-s-artifact-norm-en1991` | 32 | — | 7 | **green** | `🗑️generated/u3b-sweep-batch7.txt` |
| `semio-framework-os-kernel` | 32 | — | 7 | **green** (`--all-targets`) | `🗑️generated/u3b-check2-framework-alltargets.txt` |
| `semio-s-artifact-norm-en1999` | 26 | — | 8 | **green** | `🗑️generated/u3b-sweep-batch8.txt` |
| `semio-s-artifact-layout-layout` | 25 | — | 8 | **green** | `🗑️generated/u3b-sweep-batch8.txt` |
| `semio-s-artifact-cad-cad` | 24 | — | 8 | **green** | `🗑️generated/u3b-sweep-batch8.txt` |
| `semio-s-artifact-norm-en1997` | 22 | — | 8 | **green** | `🗑️generated/u3b-sweep-batch8.txt` |
| `semio-s-artifact-norm-en1994` | 22 | — | 8 | **green** | `🗑️generated/u3b-sweep-batch8.txt` |
| `semio-s-artifact-norm-din4108` | 22 | — | 8 | **green** | `🗑️generated/u3b-sweep-batch8.txt` |
| `semio-s-artifact-norm-en1996` | 22 | — | 8 | **green** | `🗑️generated/u3b-sweep-batch8.txt` |
| `semio-s-artifact-norm-iso16757` | 21 | — | 8 | **green** | `🗑️generated/u3b-sweep-batch8.txt` |
| `semio-s-artifact-norm-en1995` | 20 | — | 8 | **green** | `🗑️generated/u3b-sweep-batch8.txt` |
| `semio-s-artifact-norm-vdi3805` | 19 | — | 8 | **green** | `🗑️generated/u3b-sweep-batch8.txt` |
| `semio-framework-artifact-workflow-workflow` | 18 | — | 9 | **green** (`--all-targets`) | `🗑️generated/u3b-check2-framework-alltargets.txt` |
| `semio-s-artifact-lowpoly-lowpoly` | 17 | — | 9 | **green** | `🗑️generated/u3b-sweep-batch9.txt` |
| `semio-s-artifact-norm-en1993` | 17 | — | 9 | **green** | `🗑️generated/u3b-sweep-batch9.txt` |
| `semio-s-artifact-dag-dag` | 17 | — | 9 | **green** | `🗑️generated/u3b-sweep-batch9.txt` |
| `semio-s-artifact-mathematical-equation` | 16 | — | 9 | **green** | `🗑️generated/u3b-sweep-batch9.txt` |
| `semio-s-artifact-process-process3d` | 16 | — | 9 | **green** | `🗑️generated/u3b-sweep-batch9.txt` |
| `semio-framework-plugin` | 16 | — | 9 | **green** (`--all-targets`) | `🗑️generated/u3b-check1-framework-plugin.txt` |
| `semio-s-artifact-draw-drawing` | 14 | — | 9 | **green** | `🗑️generated/u3b-sweep-batch9.txt` |
| `semio-framework-artifact-infinite-dag` | 14 | — | 9 | **green** (`--all-targets`) | `🗑️generated/u3b-check2-framework-alltargets.txt` |
| `semio-s-artifact-norm-din18599` | 13 | — | 9 | **RED — peer's file, not the label migration (§7.1)** | `🗑️generated/u3b-sweep-batch9.txt` |
| `semio-s-artifact-reasoning-wires` | 12 | — | 10 | **green** | `🗑️generated/u3b-sweep-batch10.txt` |
| `semio-s-artifact-forms-forms` | 12 | — | 10 | **green** | `🗑️generated/u3b-sweep-batch10.txt` |
| `semio-s-artifact-raster-raster` | 12 | — | 10 | **green** | `🗑️generated/u3b-sweep-batch10.txt` |
| `semio-s-artifact-animate-presentation` | 11 | — | 10 | **green** | `🗑️generated/u3b-sweep-batch10.txt` |
| `semio-s-artifact-playbook-playbook` | 11 | — | 10 | **green** | `🗑️generated/u3b-sweep-batch10.txt` |
| `semio-s-artifact-flow-flow` | 10 | — | 10 | **green** | `🗑️generated/u3b-sweep-batch10.txt` |
| `semio-s-artifact-norm-en1990` | 10 | — | 10 | **RED — peer's file, not the label migration (§7.1)** | `🗑️generated/u3b-sweep-batch10.txt` |
| `semio-framework-artifact-flow-flow` | 10 | — | 10 | lib **green**; `--all-targets` RED in a peer's test (§7.1) | `🗑️generated/u3b-check2-framework-alltargets.txt` |
| `semio-s-artifact-writer-writer` | 9 | — | 10 | **green** | `🗑️generated/u3b-sweep-batch10.txt` |
| `semio-s-artifact-sequence-sequence` | 8 | — | 10 | **green** after §2.1 fix | `🗑️generated/u3b-sweep-recheck-reds.txt` |
| `semio-s-artifact-imperative-procedure` | 7 | — | 11 | **green** | `🗑️generated/u3b-sweep-batch11.txt` |
| `semio-s-artifact-vcs-vcs` | 6 | — | 11 | **green** | `🗑️generated/u3b-sweep-batch11.txt` |
| `semio-framework-os-config` | 6 | — | 11 | **green** (`--all-targets`) | `🗑️generated/u3b-check2-framework-alltargets.txt` |
| `semio-framework-artifact-workflow-run` | 5 | — | 11 | **green** (`--all-targets`) | `🗑️generated/u3b-check2-framework-alltargets.txt` |
| `semio-s-artifact-sourcing-curation` | 3 | — | 11 | **RED — peer's file, not the label migration (§7.1)** | `🗑️generated/u3b-sweep-batch11.txt` |
| `semio-s-artifact-demonstrator-playground` | 1 | — | 11 | **green** | `🗑️generated/u3b-sweep-batch11.txt` |
| `semio-s-plugin-cad-aec-building` | 1 | — | 11 | **green** | `🗑️generated/u3b-sweep-batch11.txt` |
| `semio-s-plugin-norm` | 1 | — | 11 | **green** | `🗑️generated/u3b-sweep-batch11.txt` |
| `semio-s-plugin-playbook-procedural` | 1 | — | 11 | **green** | `🗑️generated/u3b-sweep-batch11.txt` |

## 3. wasm32 checks

### 3.1 Framework `--all-targets` (native)

`cargo check -p semio-framework-os-kernel -p semio-framework-artifact-workflow-workflow
-p semio-framework-artifact-infinite-dag -p semio-framework-artifact-flow-flow
-p semio-framework-os-config -p semio-framework-artifact-workflow-run --all-targets --keep-going`
→ `🗑️generated/u3b-check2-framework-alltargets.txt`.

Five of six **green** (27 warnings). Label errors found and fixed here:

| file:line | fix |
|---|---|
| `…/🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🧪️tests/🔬️unit/🦀️.rs:7` | assertion → full `LocalizedLabel::native` matrix |
| `…/🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🧪️tests/🔬️unit/🦀️.rs:7` | same |
| `…/🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🧪️tests/🔬️unit/🦀️.rs:11` | same |
| `…/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs:369` | same (`RenameMini`) |

…plus a **German defect** the assertions exposed: `set-default-app` / `clear-default-app` piped the
ENGLISH `role_name()` ("editor"/"viewer") into the German cell ("Standard editor für …"). Both leaves
now carry a `role_name_de()` twin (Editor / Betrachter) and read
`Standard-Editor für "…" festlegen` / `Standard-Betrachter für "…" entfernen`.

`semio-framework-artifact-flow-flow` lib is green; its **lib test** is red on a peer's incomplete
rename (§7.1).

### 3.2 wasm32-wasip2

`CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-note -p semio-s-plugin-draw
-p semio-s-plugin-puzzle --target wasm32-wasip2 --keep-going` → **exit 0, 213 warnings**,
`🗑️generated/u3b-check3-wasm32.txt`.

`--features component-app-assembly` is **not** passed here: none of the three plugin crates declares
it (`semio-s-plugin-puzzle` declares only `plugin-entry`; note and draw declare no features at all),
and cargo hard-errors on a feature no selected package has. The guest-gated path is still covered —
all three depend on `semio-framework-plugin` with `features = ["component-guest"]`, so the
`cfg(target_arch = "wasm32")` code compiled.

## 4. TypeScript

### 4.1 `🛠️dev💻️os🪁️typecheck` (`bun nx run @semio-tech/framework-os:typecheck`)

Run directly as `bun ./📜️script.ts typecheck` from `💻️os/📦️packages/🟦️typescript` (rule 16).
**63 errors tree-wide — identical to U3's baseline, and 0 of them in any file U3 or U3b touched.**
Capture `🗑️generated/u3b-typecheck-os.txt`.

The four hits that land *in* U3-touched files are the same pre-existing ones U3 named, and none is
about the carrier:

| file:line | error | why it is not the label migration |
|---|---|---|
| `🔬️engine-contract/🟦️.ts:1523` | `pointerCancelScreen` missing on `Board2dWasmSession` | a board2d session-surface field |
| `🔬️engine-contract/🟦️.ts:8539` | `ResolvedActionDefinition[]` | bottoms out at `options: string[]` vs `{value,label}[]` — an ARG-option shape, not `LocalizedLabel` |
| `🏛️ShellHost/🟦️.tsx:5123` | `consumes` missing | plugin module descriptor |
| `🏛️ShellHost/🟦️.tsx:8120` | `PluginWasmHandle` | ephemeral-snapshot handle shape |

### 4.2 vitest

| suite | selection | result | capture |
|---|---|---|---|
| `@semio-tech/framework-kernel` | `-t "LocalizedLabel shared fixture"` | **4 passed**, 76 skipped | `🗑️generated/u3b-vitest-localized-label.txt` |
| `@semio-tech/framework-kernel` | `-t "historyEntryLabelText"` | **5 passed**, 75 skipped | same |
| `@semio-tech/framework-kernel` | whole suite | 79 passed, **1 failed — not mine** (§7.2) | `🗑️generated/u3b-vitest-kernel.txt` |
| `@semio-tech/framework-renderer-react` (level `long`) | `-t "history cursor"` | **1 passed** — "the shell publishes the framework history cursor undo and redo are decided by, **in the locale it is showing**" | `🗑️generated/u3b-vitest-react-history.txt` |
| `@semio-tech/framework-renderer-react` (level `long`) | `-t "Set Active Example"` | **1 passed** — "navbar example id follows Set Active Example revertible, **on the action id alone**" | same |
| `bun test 🖱️ui/🧪️tests/🎚️axes/🟦️.ts` | — | **4 passed**, 37 assertions | `🗑️generated/u3b-bun-axes-parity.txt` |

> The two react rows need the **`long`** level: the react engine project's `include` is the quick
> suite alone at the default level, and `🔬️engine-contract` is only in `engineTestSuites`. A bare
> `bun ./📜️script.ts test -t "history cursor"` reports `5 skipped` and looks like a pass.

### 4.3 The axes parity test was red and is now green

`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎚️axes/🟦️.ts:59` asserted that the generated axes' Rust consumer —
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs` — contains `#[path = "🤖️generated/🦀️.rs"]`. U3's
relocation replaced that mount with a re-export, so the string is **absent from that file**
(verified by grep) and the assertion could not hold. Retargeted at the real consumer,
`🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale/🦀️.rs`, plus a new assertion that the wgpu façade still
re-exports all six names — the two halves of what the relocation actually promises.

## 5. Language-agnostic fixture + JSON schema twin

One fixture, one schema, two readers. Nothing in either reader names a language: the axes come from
the generated `Locale`/`Terminology` (Rust) and `SHELL_LOCALES`/`SHELL_TERMINOLOGIES` (TS), and every
expectation is keyed `<terminology>.<locale>`, so **adding a locale to `🖱️ui/🎚️axes/🔣️.json` fails both
readers until every row carries a translated cell.**

| file | role |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale/🧫️fixtures/🏷️localized-label/🔣️.json` | 3 complete rows + 3 refusal rows, each with its full `resolve` table |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale/🧬️schema/🔣️.json` | the JSON-schema twin: every terminology required, every locale required, `additionalProperties: false` at both levels |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale/🧪️tests/🏷️localized-label-fixture/🦀️.rs` | **Rust decode** through the crate's own `FromValue`, with `ToValue` asserted as its exact inverse |
| `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🏷️localized-label-fixture/🟦️.ts` | **AJV** oracle + `historyEntryLabelText` resolution |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale/🏷️label/🦀️.rs:200` | mounts the Rust test |
| `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🎚️config/🟦️.ts:22` | adds the TS test to the kernel vitest `include` (a suite in no include list is a gate that measures nothing) |

**Rows.** `add-widget` (authored copy, four distinct cells), `rename-layer` (runtime data quoted
inside authored copy — the datum identical in every cell, the prose not), `locale-invariant-data`
(`LocalizedLabel::data`, every cell byte-identical, declared as a shape so it is not misread as an
untranslated leak).

**Refusals.** `missing-locale` (only `de` filled), `missing-terminology` (the pre-`LocalizedLabel`
`{en, de}` shape that took the whole shell down on 2026-09-21), `unknown-locale` (an `fr` cell the
axes do not declare). Each carries the `schemaError` AJV must raise and the `resolve` table both
readers must produce — **empty**, never another locale's text.

| run | result | capture |
|---|---|---|
| `cargo test -p semio-framework-os-kernel --lib localized_label` | **8 passed, 0 failed** (4 new + the 4 pre-existing round-trip laws) | `🗑️generated/u3b-cargo-test-localized-label.txt` |
| kernel vitest `-t "LocalizedLabel shared fixture"` | **4 passed** | `🗑️generated/u3b-vitest-localized-label.txt` |

## 6. German proof-read of the 40 most-used labels

### 6.1 A mojibake defect, repaired in 14 files

`✏️s/🔌️plugins/📕️norm/🗿️artifacts/*/…/✏️editor/🦀️.rs` (14 norm editors, the `setSelectedCheckIndex`
Actions row) carried **`"AusgewÃ¤hlte PrÃ¼fung setzen"`** — UTF-8 bytes of the Latin-1 reading of
`Ausgewählte Prüfung setzen`, i.e. text that had been round-tripped through the wrong encoding. It
compiles, so no check could find it; only reading the German does. All 14 repaired. The only `Ã`
left in `✏️s` is a docstring in `📖️pdf/🚪️io/🦀️.rs:498` that *describes* this phenomenon.

### 6.2 The glossary is now the single source of truth — `--retranslate`

U3's codemod only matches `fn label(&self) -> String`, so after the migration landed it was a no-op
and **a wrong translation could only be fixed by hand, in the leaf**. That makes
`🐍️u3-de-glossary.json` a historical record rather than a source of truth.

`🐍️u3-localise-mutation-labels.py --retranslate` (new) closes that: for every already-migrated,
trait-anchored `fn label`, it re-derives the German argument's **string literals** from the English
literals in the same positions and the current glossary, and rewrites only where they disagree.
Same region anchor, same span-keyed edits, same abort-on-missing-entry rule.

It replaces literals **only**, never the surrounding expressions, so a German side that reaches for
its own helper survives a glossary correction — `set-default-app`'s `role_name_de(self.role)` (§3.1)
is the case that forced this design. A site whose two sides carry different literal counts is
skipped and named on stderr rather than guessed at.

* baseline before any edit: `would retranslate 2 of 2792` (both my own §3.1 edits)
* after the 44 glossary corrections: **`retranslated 52 of 2792 migrated sites in 52 files`**
  (`🗑️generated/u3b-retranslate.txt`)
* **idempotence proven**: an immediate second run reports `would retranslate 0 of 2792 migrated sites in 0 files`

### 6.3 The 44 corrections

Ranked by use across the ten named plugins (`🗒️note 🖍️draw 🧩️puzzle 📏️layout 📋️forms 🗺️gis 🏗️fem 🔋️energy
🏛️architect 📕️norm`): 1124 distinct templates over 1194 mutation-label sites. The most-used 45 read
correctly except for the defects below; the review then widened to the whole glossary for the defect
CLASSES those revealed, because a class is cheaper to fix once than to meet again.

**Class A — hyphenated English modifier carried into German** (the compounder split on the hyphen and
left the second part lowercase). 39 entries had the shape `[a-zäöüß]-[A-ZÄÖÜ]`; all repaired:

| before | after |
|---|---|
| `Fall "{}" eigen-Gewicht auf {} setzen` | `Fall "{}" Eigengewicht auf {} setzen` |
| `angenommen eigen-Gewichtlast auf {:?} ändern` | `Angenommene Eigengewichtslast auf {:?} ändern` |
| `eigen-Gewichtmaterial` / `eigen-Gewichtdicke` | `Eigengewichtsmaterial` / `Eigengewichtsdicke` |
| `Quer-Abschnittfläche [mm2]` | `Querschnittsfläche [mm2]` |
| `Kühlungssatz-Punkttemperatur` | `Kühlsolltemperatur` |
| `Zone {} Geschoss-Flächenbeteiligung` | `Zone {} Geschossflächenanteil` |
| `Mörteldruck-Festigkeitsklasse` | `Mörteldruckfestigkeitsklasse` |
| `Mauerwerkfertigung-Regelungsklasse` | `Mauerwerksfertigungskontrollklasse` |
| `Ermüdungss-Nneigungsm auf {} ändern` | `Ermüdungs-S-N-Steigung m auf {} ändern` |
| `EN 1993-1-12 hoch-Festigkeitsstahleingaben aktualisieren` | `EN 1993-1-12 Eingaben für hochfesten Stahl aktualisieren` |
| `Bauteil-Nummereingabe` / `Bauteil-Nummerregel` | `Bauteilnummer-Eingabe` / `Bauteilnummer-Regel` |
| `Gebäude-/Energie-/Form-/Struktur-klassisch-Modellkind` | `Gebäudemodell-/Energiemodell-/Formmodell-/Klassisches-Strukturmodell-Kind` |
| `Re-Anheftungsrepräsentation an #{}` | `Repräsentation bei #{} neu anheften` |
| `geo-Produkte ersetzen` | `Geo-Produkte ersetzen` |
| `Farbe-Raum`, `eingebettet-Datei`, `ext-g-Zustand`, `Marke-Info`, `offen-Aktion`, `optional-Inhalt`, `Seite-Layout`, `Seite-Modus`, `acro-Formular`, `Betrachter-Präferenzen` (PDF catalogue) | `Farbraum`, `Eingebettete Datei`, `ExtGState`, `Markierungsinfo`, `Öffnen-Aktion`, `Optionalen Inhalt`, `Seitenlayout`, `Seitenmodus`, `AcroForm`, `Betrachtereinstellungen` |
| `Maximal-Schritte=`, `Zeitlimit-ms=`, `Maximal-Datei-Bytes=` | `Max-Schritte=`, `Timeout-ms=`, `Max-Datei-Bytes=` |

**Class B — wrong Fugenelement.** `Analyseneinstellungen` → `Analyseeinstellungen` (Analyse + Einstellungen takes no `-n`).

**Class C — case after `auf … ändern`.** `Nationaler Anhang auf {} ändern` → `Nationalen Anhang auf {} ändern` (both the `{}` and `{:?}` rows; the pattern governs the accusative and the adjective was left nominative).

**Class D — §3.1's own two rows**, now spelled in the glossary so `--retranslate` keeps rather than reverts them.

### 6.4 Re-check after the pass

`cargo check` over the ten crates the pass touched most (`norm-en1991/-en1993/-en1995/-en1996/
-en1999/-din16798/-iso16757`, `remodel-remodeling`, `stdio-pdf`, `framework-os-config`):
**all green, 94 warnings**; the only error in the run is the peer's `norm-en1990`, pulled in as a
dependency (§7.1). `🗑️generated/u3b-check5-retranslated.txt`.

## 7. Honest gaps

### 7.1 Four crates are red in peers' in-flight files, none of it the label migration

Re-checked one last time at the end of the slice (`🗑️generated/u3b-final-peer-reds.txt`,
`🗑️generated/u3b-final-flow.txt`) — still red, still not mine. I did not touch them (rule 3): each
repair belongs to the slice mid-edit in it.

1. **`semio-s-artifact-norm-en1990`, `semio-s-artifact-norm-din18599`** — `E0560: struct
   ReplaceSnapshot has no field named snapshot`, raised by the **uncommitted** macro
   `semio_s_artifact_norm_contract::norm_command_from_action!`
   (`✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs:450-475`, added in the working tree, `git diff` shows +46).
   The macro builds `ReplaceSnapshot { snapshot }`, but these two artifacts' payload is
   `ReplaceSnapshot { text: String }` (en1990 line 32, din18599 likewise) because their snapshot is a
   composed `ArtifactChild` that carries its own DSL text. The other thirteen norm editors take the
   macro unchanged. Zero label errors in either crate.
2. **`semio-s-artifact-sourcing-curation`** — `E0308` at
   `…/🗂️curation/…/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs:24`: `ExampleSource::document()`
   returns `String` (the `ExampleSourceBody::Deferred` design) while the sibling match arm is
   `&'static str`. The compiler names the one-character repair (`&…`). Not label-related.
3. **`semio-framework-artifact-flow-flow` (lib TEST only; the lib is green)** — an incomplete rename:
   `🧵️retained/🧪️tests/🧵️retained/🦀️.rs:35,44,158` and `🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs:34` still
   read `host_snapshot` where line 33 now binds `fixture`. Four `E0425`s, no label involvement.

Everything else in the sweep is green: **102 of 106 crates, plus `semio-framework-plugin`
`--all-targets`, plus the wasm32 lane.**

### 7.2 One pre-existing vitest failure in the kernel project

`📤️return/📦️content/🟦️.ts > KernelReturnContentFraming …` fails with
`can't resolve reference https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/NonZeroU64
from id …/framework/actor/lifetime/schema.json`. An AJV `$ref` registration problem in another
slice's schema wiring; I changed no TS source in that module, only added a test file and one
`include` entry. The other 79 tests in that project pass.

### 7.3 The JSON schema twin is authored, not generated

`🌐️locale/🧬️schema/🔣️.json` is hand-written with a **drift guard on both sides** (§5: the Rust test
and the TS test each assert its axis key sets equal the generated enums, and that
`additionalProperties` is false at both levels), so adding a locale fails until the schema carries
it. It would be stronger still as a third emit of the `ui-axes` generator
(`🖱️ui/🎚️axes/📋️plan/🟦️.ts`), which would make it impossible to forget rather than merely impossible
to ignore. I did not do that: it changes a registered generator contract (`uiAxesTargets`,
`uiAxesPreview`, the taxonomy row and the parity test's exact two-file plan) and would need a
generator publish run to land the file — a gate-shaped change that wants its own slice while
peers are mid-edit.

### 7.4 What is verified by tests, not observed at runtime

Everything in this report is type-checking and unit/contract tests. **No shell was booted and no
wasm component was rebuilt**, so the History panel has still not been *seen* rendering German at
runtime — U3's §5 gap 3 stands unchanged. The vitest row in §4.2 proves the projection re-renders
the ledger from the carrier for a given `uiLocale`; it does not prove the live shell does.

### 7.5 Two axis-round-trip test mods are mounted twice

`🔬️targets-wgpu-label-localized-label-value-round-trip` and
`🔬️targets-wgpu-locale-terminology-value-locale-terminology-value-round-trip` still live under
`🔨️modules/🖱️ui/🧪️tests/` and are `#[path]`-mounted from BOTH the kernel's `🌐️locale` modules and the
wgpu target root, so they run in two crates. In the wgpu crate they do test the re-export façade,
which is worth something; the `targets-wgpu-` names are nonetheless stale for code that now lives in
the kernel. §2.2 fixed the broken one of the two paths rather than moving the directories, which
would churn files peers are in.

### 7.6 English prose and `Locale::En`'s `#[default]`

Unchanged from U3 §5.5–5.6: ~324 kebab-case op ids are still the English side's text, and the
generated `Locale` enum still carries `#[default]` on `En`. Nothing on the label path reads that
default; removing the attribute is a separate slice.

## 8. Files changed

**Framework — carrier and its contract (Rust)**
* `🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale/🏷️label/🦀️.rs` — `LocalizedLabel::retained_bytes()`; mounts the new fixture test.
* `🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale/🧾️value/🦀️.rs` — `#[path]` repaired after U3's relocation.
* `🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale/🧫️fixtures/🏷️localized-label/🔣️.json` — **new**, the shared fixture.
* `🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale/🧬️schema/🔣️.json` — **new**, the JSON-schema twin.
* `🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale/🧪️tests/🏷️localized-label-fixture/🦀️.rs` — **new**, the Rust decoder half.

**Framework — call sites and tests (Rust)**
* `…/🔌️plugin/🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations/➕️add-value/🦀️.rs`, `…/⚛️reactor/💼️jobs/🧬️mutation-plan/🧫️fixtures/🧬️job-test-mutations/🧬️mutations/➕️add-value/🦀️.rs`, `…/🔌️plugin/🧫️fixtures/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🦀️.rs` — the three `native(&x, &x)` self-delegations.
* `…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` — `label_in` helper, 11 assertions/constructions, 1 new test.
* `…/🔌️plugin/🧪️tests/🛰️declaration-channels-unit/🦀️.rs`, `…/🧪️tests/🔬️plugin-runtime-contributed-mutation-wire/🦀️.rs`, `…/🧪️tests/📡️contributed-mutation-wire-unit/🦀️.rs`, `…/🏗️builder/🧪️tests/🔗️dependency-contribution-unit/🦀️.rs`, `…/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🔬️unit/🦀️.rs`, `…/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations-mutations-add-value-unit/🦀️.rs`, `…/🔌️plugin/🧪️tests/🔬️app-panel-kit/🦀️.rs` — label assertions re-authored against the full matrix.
* `…/💻️os/🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🦀️.rs` + `🧹clear-default-app/🦀️.rs` — `role_name_de()` and the corrected German; their `🧪️tests/🔬️unit/🦀️.rs`, and `🛡️change-merge-policy/🧪️tests/🔬️unit/🦀️.rs`.
* `…/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs` — `RenameMini` label assertion.

**Framework (TypeScript)**
* `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🏷️localized-label-fixture/🟦️.ts` — **new**, the AJV half.
* `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🎚️config/🟦️.ts` — includes it.
* `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎚️axes/🟦️.ts` — consumer assertion retargeted + façade assertion added.

**Plugins**
* `✏️s/🔌️plugins/🎬️sequence/…/✏️editor/🦀️.rs:1180` — `retained_bytes()`.
* 14 × `✏️s/🔌️plugins/📕️norm/🗿️artifacts/*/…/✏️editor/🦀️.rs` — mojibake repaired.
* 52 × `🧬️mutations/**/🦀️.rs` across `📕️norm`, `📸️remodel`, `🗄️stdio` (`📖️pdf`) and `🧰️framework/🛍️products/💻️os/🎚️config` — the `--retranslate` pass (`🗑️generated/u3b-retranslate.txt` lists them).

**Ticket folder**
* `🐍️u3-localise-mutation-labels.py` — `--retranslate` pass (`split_native_arguments`, `migrated_sites`, `retranslated`, `retranslate`) + docstring.
* `🐍️u3-de-glossary.json` — 44 corrected entries.
* `📜️u3b-sweep.sh` — **new**, the batched sweep runner.
* `📓️u3b-localized-label-tree-green.md` — this report.
* `🗑️generated/`: `u3b-crate-list.txt`, `u3b-crate-table.md`, `u3b-batches.json`, `u3b-check1-framework-plugin.txt`, `u3b-check2-framework-alltargets.txt`, `u3b-check3-wasm32.txt`, `u3b-check4-kernel-fixture.txt`, `u3b-check5-retranslated.txt`, `u3b-sweep-batch1..11.txt`, `u3b-sweep-recheck-reds.txt`, `u3b-final-peer-reds.txt`, `u3b-final-flow.txt`, `u3b-typecheck-os.txt`, `u3b-vitest-kernel.txt`, `u3b-vitest-localized-label.txt`, `u3b-vitest-react-history.txt`, `u3b-bun-axes-parity.txt`, `u3b-cargo-test-localized-label.txt`, `u3b-retranslate.txt`.
