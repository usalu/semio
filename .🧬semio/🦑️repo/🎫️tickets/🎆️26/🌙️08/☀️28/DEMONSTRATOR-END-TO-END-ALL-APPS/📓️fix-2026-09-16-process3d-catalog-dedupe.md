# 🧩️ process3d catalog dedupe — the Werkstatt renders each `catalog_id` once (2026-09-16)

Closes the process-owner follow-up left open by
[`📓️fix-2026-09-16-kernel-capability-contributions-and-close-cliff.md`](./📓️fix-2026-09-16-kernel-capability-contributions-and-close-cliff.md)
§4.2 / §5 bullet 2. Repo root `/Users/ueli/Documents/semio`.

---

## 1. Finding

`installed_catalogs` concatenated the built-ins with every `process.machines` contribution and deduped
nothing:

```rust
let mut catalogs = builtin_installed_catalogs();
catalogs.extend(contributed_machine_catalogs(contributions_json).into_iter().map(MachineCatalogs::from));
```

`builtin_installed_catalogs()` is the fixed five — `geometry`, `wood`, `concrete`, `metal`, `robotic`
(`✏️editor/🦀️.rs:2139`). The four shipped extensions contribute those SAME ids: each
`🧩️extensions/*/🦀️.rs` `bundle()` sends `moduleId = catalog.catalog_id()`
(`🧩️extensions/🪵️wood/🦀️.rs:~184`, `HOST_APP_ID = "process3d-play"`). So once the host actually
forwarded capability packs, the workshop panel — which opens one section per entry of
`installed_catalogs` (`📌️panels/🛠️workshop/🦀️.rs:60`) — rendered **nine** sections: the built-in five
plus a second Wood/Metal/Concrete/Robotic.

Sourcing had already fixed the same shape at its app boundary: `schema::installable_contributions`
(`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:747-779`)
seeds `installed` from `sourcing_modules("[]")` and skips any contributed `module_id` already served.

## 2. Design

**A `catalog_id` names exactly one catalog. The installed build wins; a contribution repeating an
installed id is dropped; only a genuinely new id appends, in host order.**

Applied in `installed_catalogs`, not in `installable_contributions`: the distillation lane stays the
"what may this app retain" law (and is owned right now by the concurrent admission-ceiling work), while
the dedupe is the "what does this app install" law that the panel reads. Retention is therefore
unchanged — the four entries are still admitted, still retained, still provable on the wire — and only
the rendered roster is deduped.

## 3. Edits

| file:line | change |
|---|---|
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2143-2163` | `installed_catalogs` now walks `contributed_machine_catalogs` and skips a `catalog_id` already present; docstring states the law and names the sourcing precedent. |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1127` | `demonstrator_contributions_pack` → `pub(crate)` so the workshop panel test can push the REAL pack. |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1190-1193` | `the_real_demonstrator_pack_is_admitted_distilled_and_retained` now asserts the installed ids **equal** the built-in ids (was `builtin.len() + 4`). |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1197-1209` | new `a_pack_repeating_the_builtin_ids_installs_exactly_the_builtin_catalogs`. |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1211-1236` | new `a_pack_with_a_new_catalog_id_appends_one_section_with_its_machines`. |
| `…/✏️editor/📌️panels/🛠️workshop/🧪️tests/🔬️unit/🦀️.rs:60-85` | new `the_real_extension_pack_renders_each_catalog_section_once`. |

No other file touched. `installable_contributions`, the admission ceilings and the `setContributions`
wire lane were left exactly as the concurrent worker has them.

## 4. Tests

- **(a) repeated ids ⇒ built-ins only** — `a_pack_repeating_the_builtin_ids_installs_exactly_the_builtin_catalogs`
  builds the pack from the real extension descriptors (`demonstrator_contributions_pack`, itself derived
  from the same `MachineCatalog` impls the extension crates emit) and asserts `(id, label)` of the
  installed roster equals the built-in roster, and that each of `wood`/`metal`/`concrete`/`robotic`
  names exactly one section.
- **(b) a new id appends** — `a_pack_with_a_new_catalog_id_appends_one_section_with_its_machines`
  adds a `glass` entry to the real pack; the installed ids are the built-ins followed by `glass`, and
  `catalog_machine(pack, "glass", "glassCutter")` resolves with `catalog_id = Some("glass")`. Dedupe
  drops repeats, never the contribution channel.
- **(c) the real pack, admitted and rendered** — `the_real_demonstrator_pack_is_admitted_distilled_and_retained`
  still proves admission (wire fits `CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES`, four entries distilled,
  retained config accepts them) and now proves the roster is deduped.
  `the_real_extension_pack_renders_each_catalog_section_once` dispatches that pack's distillation
  through `setContributions` on a registry-backed app, renders `process.play.workshop`, extracts every
  `"key":"process3d-play-workshop.catalog.<id>"` and asserts no id opens two sections and no
  non-built-in id appears. (`wood` and `geometry` render no section in that fixture — the default
  workshop already has all of their machines installed, which is the panel's existing
  "nothing left to add" rule; `metal` is asserted present.)

## 5. Commands and counts

All prefixed with `DEVELOPER_DIR=/Library/Developer/CommandLineTools RUSTFLAGS=-Awarnings CARGO_TERM_QUIET=true
CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_BUILD_JOBS=4 NX_TUI=false NX_TASKS_RUNNER_DYNAMIC_OUTPUT=false`.
Logs under `🗑️generated/`.

| command | result | log |
|---|---|---|
| `cargo test -p semio-s-artifact-process-process3d --lib -- --skip vcs_artifact_app_production_maintenance_swap` (before) | **304 passed, 34 failed**, 3 ignored | `process3d-dedupe-test-before.txt` |
| same (after) | **308 passed, 33 failed**, 3 ignored | `process3d-dedupe-test-after.txt` |
| `cargo test … -- <the four test names>` | **4 passed, 0 failed** | `process3d-dedupe-focused.txt` |
| `cargo check --target wasm32-wasip2 -p semio-s-artifact-process-process3d -p semio-s-plugin-process` | **exit 0** | `process3d-dedupe-wasm-check.txt` |

`--features component-app-assembly` does **not** exist on `semio-s-artifact-process-process3d` (cargo
lists the 54 crates that do have it; this artifact is not one). The wasm check was therefore run
featureless on the artifact crate and additionally on `semio-s-plugin-process`, the crate that actually
builds the process component.

Failure-set diff (`before` minus `after`) is exactly one line:

```
< editor::process3d::component::unit_tests::the_real_demonstrator_pack_is_admitted_distilled_and_retained
```

i.e. **no new failure, and one baseline failure now passes** — that test failed at baseline with
`the real pack's command wire (26537 B) must fit the registered admission`, which the concurrent
`setContributions` admission-ceiling work fixed in the tree between the two runs. The remaining 33
failures are the pre-existing harness set (world-pointer/undo/sun/wasm/mounted-publication/retained-laws
families) and are unchanged, name for name.

## 6. Not done here — restage required

**The staged React bundle was NOT restaged.** The concurrent worker restages after its admission fix;
this change only lands in the running demonstrator at `:6029` once that restage happens. Until then the
browser still serves the pre-dedupe component and will still show nine Werkstatt sections.

Expected after the restage: the Werkstatt shows **five** sections —
`Geometry`, `Wood`, `Concrete`, `Metal`, `Robotic` (minus any whose machines are all installed already)
— with the `[DEBUG] contributions publish … status installed` line still the proof the push landed,
since the duplication that used to serve as that proof is now gone by design.
