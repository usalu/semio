# terra-descriptors — close the descriptor gap to 33/33 and ratchet

Packet: `terra-descriptors`, ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`.

## Executive summary

**Code migration work is DONE for 6 of the 7 genuinely-missing plugins** (playbook, fem, puzzle
directly by this packet; block and trinity discovered ALREADY migrated by a prior packet — the
scout's audit was stale for both, same pattern the scout itself flagged for trinity). **Emission and
ratcheting could NOT be run this session**: `semio-framework-plugin` — the crate every plugin depends
on, and the crate `descriptor_is_fresh`'s own macro lives in — is currently RED due to a large, live,
uncommitted, unrelated rewrite by another session (`ui_contract`/`ui_runtime`/wgpu draw backends).
Evidence and full detail below. Per this ticket's own rule 3 ("a descriptor is only ratcheted after
its `descriptor_is_fresh` test passes... a false green is the worst possible outcome"), nothing was
ratcheted on an unverified build.

**Numbers, before and after this packet:**

| metric | before | after |
|---|---|---|
| descriptors emitted on disk | 26/33 | **26/33 (unchanged — emission blocked, see below)** |
| `DESCRIPTOR_MIGRATED_PLUGINS` ratchet list | 13 | **13 (unchanged — ratcheting blocked, see below)** |
| plugins CODE-COMPLETE for `.declare_artifact()`, awaiting only `describe` + ratchet | 2 (trinity, block — both from prior packets) | **6** (+ playbook, fem2d, fem3d, puzzle2d, puzzle3d, puzzle5d, this packet) |
| plugins genuinely blocked (design/repair, out of scope) | 2 (stdio, demonstrator) | 2 (unchanged, confirmed still blocked) |

## 1. Fresh audit re-verification (matches the scout exactly)

Re-derived independently at session start, not trusted from the scout's numbers:

```
$ for d in plugins/*/; do [ -f "$d🛂️descriptor.semio" ] && echo HAS || echo MISS; done
26 HAS, 7 MISS: demonstrator, fem, playbook, puzzle, stdio, trinity, block
$ grep DESCRIPTOR_MIGRATED_PLUGINS 🔌️plugin/🦀️component.rs
["note","sequence","vcs","forms","sourcing","dag","mathematical","writer","reasoning-mindmap","animate","draw","energy","layout"]  (13 entries)
```
Both match the scout's `📓️luna-descriptor-status.md` exactly (26/13/7). All 33 plugin dirs enumerated
and cross-checked against each plugin's `Plugin::builder("...")` id string to build the full N/33
table in §5.

## 2. Code migration performed this packet (`.declare_artifact()` recipe, per
`📓️terra-fleet-trinity-recipe-report.md`)

### 📖️playbook (1 artifact)
- New: `🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🦀️component.rs` (`standard()`)
- New: `🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs` (`subset()`)
- Modified: artifact root `🦀️component.rs` — `pilot_languages()` made `pub`, `declaration()` replaced
  with `artifact()` (kind `s.playbook.playbook`), `EditorApp` import dropped (now unused)
- Modified: plugin root `🦀️component.rs` — `.artifact(...)`/`.editor::<>()`/`.viewer::<>()` replaced
  with `.declare_artifact(crate::artifacts::playbook::artifact())` +
  `.editor_mutation_roster()`/`.viewer_mutation_roster()`
- Modified: `📦️packages/🦀️rust/📦️glue.rs` — mounted both new files (`standards::v1::standard`,
  `standards::v1::subsets::any::subset`)
- Confirmed no other caller of `playbook::declaration` repo-wide.

### 🏗️fem (2 artifacts: fem2d, fem3d)
Same 4-file pattern per artifact (standard-root, subset-root, artifact-root edit, plugin-root edit),
plus one shared glue.rs edit covering both mounts. Kinds `s.fem.fem2d` / `s.fem.fem3d`.

### 🧩️puzzle (3 artifacts: puzzle2d, puzzle3d, puzzle5d)
Same pattern ×3. Kinds `s.puzzle.puzzle2d` / `s.puzzle.puzzle3d` / `s.puzzle.puzzle5d`. Notable
deviation found and handled: puzzle's `examples` are NOT reachable via the artifact-root shim path
(unlike note/trinity) — puzzle's own `📦️glue.rs` mounts them at the CRATE ROOT
(`crate::examples::puzzle2d::{nakagin_capsule_tower,concrete_forest}::SOURCE`, a
`LazyLock<ExampleSource>` static, not a `source()` fn) — a third shape beyond the two the trinity
report already named. Handled by cloning the statics directly in each `examples()` fn.

**Shared deviation across all 6 new artifacts (documented in each subset-root file's own doc
comment, exactly matching trinity's own precedent):** `io: io::io()` was NOT used. Each artifact's
`🚪️io/🦀️component.rs` is still on the OLD `ComposerEntry`/`io_registry` channel; hand-authoring typed
`Deserializer`/`Serializer` impls for the foreign formats (5-6 per artifact) is real, non-trivial
migration work outside this packet's descriptor-emission scope — same boundary trinity's own packet
drew for the identical reason. `io_declaration()` is defined locally per subset-root file instead:
`native` codecs are REAL (reuse each artifact's own `pilot_languages()`, made `pub` for this),
`entries: &[]` (foreign-format composers unreachable from the new channel — an honest, documented gap,
not an oversight; `try_build()` still succeeds since an empty batch passes `preflight_io_entries`).
A lease-request is written into each new subset-root file's doc comment for the follow-up.

**Files changed this packet, complete list:**
- `✏️s/🔌️plugins/📖️playbook/🦀️component.rs`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🦀️component.rs`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🦀️component.rs` (new)
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs` (new)
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🏗️fem/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs`

Not touched: any `🚪️io/` path, any `✏️editor/`/`👁️viewer/` `🦀️component.rs` (only referenced, never
edited).

**Verification performed on this code (given the compile blocker below):**
- Brace/paren balance checked programmatically on every new/modified file — all balanced (rules out
  gross truncation/corruption, the class of damage the ticket's "no whole-file rewriting scripts"
  incident produced).
- Every referenced symbol (`pilot_languages()`, `create_<x>_app`/`create_<x>_viewer`,
  `<x>_artifact_schema_descriptor`, `<x>_artifact_inference_descriptor`, `Snapshot`/`Mutation` type
  names, `DIALECT`/`SCHEMA` consts) was grepped and confirmed to exist at the exact module path used,
  cross-checked against each plugin's own `📦️glue.rs` re-export map — not assumed from the trinity
  template.
- **NOT confirmed: `cargo check`.** See §3.

## 3. Already-migrated-by-others, discovered this session (scout audit was stale)

- **🧱️block (3 artifacts: block2d/3d/5d):** fully migrated already — `artifact()`/`standard()`/
  `subset()` all present, `📦️glue.rs` mounts present, plugin root already calls
  `.declare_artifact()` ×3, all **committed** (`git log` shows `cb9bcce7a4`/`5e7b8046be`, `git status`
  clean for the whole plugin dir). The plugin root's own doc comment names the packet
  (`descriptor-prep`, following `fleet-trinity-recipe`) — confirms this was real, deliberate,
  finished work, not accidental. The scout's classification ("no `.declare_artifact()` migration, no
  emission packet assigned") was **stale** — exactly the failure mode the scout's own report already
  flagged for trinity ("trinity reportedly done" turning out true for migration, not emission). No
  code changes made; block only needs `describe` + ratchet once the SDK is green.
- **🔱️trinity:** confirmed already migrated (per the scout's own finding, re-verified: `artifact()`
  present, `declaration()` gone). Also just needs `describe` + ratchet.

## 4. Genuinely blocked, confirmed still blocked, NOT touched (correctly out of scope)

- **🗄️stdio:** re-confirmed the pre-existing `try_library()` capability claim-set mismatch (declared
  capabilities must EXACTLY equal runtime claims; ~35/36 formats fail). This is an SDK validation
  defect, not a descriptor-emission gap — descriptors cannot emit until the claim-set repair lands.
  Needs its own dedicated packet, exactly as the scout assessed. My path_scope brief explicitly
  named "coordinate by NOT editing anything packet `terra-fleet-wasm` is building" and this repair is
  plausibly inside that packet's territory (or a sibling stdio-repair packet) — deferred, not
  attempted.
- **🎪️demonstrator:** confirmed it has **no plugin-root `🦀️component.rs` at all** — there is nothing
  to migrate yet. The blocker is the `kit.catalog` multi-declaration conflict across six owning
  plugins (cad/gis/procedural/process/puzzle/sourcing), a registry-level design decision
  (`status.md` §D3 already proposes the "one plugin declares, others reference" pattern — the exact
  shape block's own migration now demonstrates works). Resolving it means editing SIX plugin crates'
  own artifact declarations plus (possibly) the definition registry's duplicate-registration rule —
  a cross-plugin design change well outside a single packet's `path_scope`, and outside what "close
  the descriptor gap" mechanically means. Flagged for the coordinator; not attempted.

## 5. 🚨 Blocker: `semio-framework-plugin` is RED from a live, unrelated, in-progress peer rewrite

This is why emission and ratcheting could not run this session, for ANY plugin — not just the ones
this packet touched.

**Evidence, in order collected:**

1. `git status --porcelain` at session start showed `🔌️plugin/🦀️component.rs` **modified,
   uncommitted** (this file is NOT in this packet's `path_scope`; never edited by this packet).
2. First `cargo check -p semio-framework-plugin --lib` (13:20): **6 errors**, all
   `Sized`/`clone()`-on-`str` at `🔌️plugin/🦀️component.rs:11973-11977`
   (`tree.interaction_domain.clone()` — a field whose type had just been changed, mid-edit, by the
   peer session; this shape matches the ticket's own documented "hover/selection" live-ticket
   warning in `📌️important.md`'s registrar-files table).
3. `git diff --stat` on that one file: **162 lines** changed (90 insertions / 72 deletions) at that
   moment.
4. Re-checked ~8 minutes later (13:28): **same file, same region, error text UNCHANGED in shape but
   shifted 2 lines** (11975/11977 vs 11973/11975) — proof the peer was still actively typing in that
   exact function, not stale damage from earlier.
5. `git diff --stat` at that point: **1029 lines** changed on the SAME file — grew 6× in 8 minutes.
6. Re-checked again (~13:32): **175 errors**, an entirely different shape — `semio-framework-ui`
   itself now fails with **854 errors** (`active_foreground_of()` missing `.await` in wgpu
   `draw.rs`), and `semio-framework-plugin` now fails on `cannot find module ui_contract`,
   `Label::data` missing, and `Serialize`-bound failures in `ui_refresh_section`. `git status`
   confirmed **15 files** under `🧰️framework/🔨️modules/🖱️ui/` also uncommitted (wgpu draw/render
   backends across metal/d3d12/vulkan targets, `ui_contract`, `event.rs`).
7. Final check before writing this report (13:38): `git diff --stat` on `🔌️plugin/🦀️component.rs`
   alone: **1069 lines** changed — still growing, 6 minutes after the previous measurement.

**Conclusion:** this is a large, live, multi-file, cross-crate rewrite (plugin SDK + ui_contract +
ui_runtime + wgpu backends across 3 native targets) by another session, actively in progress, not
converging within this packet's observation window. This matches this ticket's own documented
pattern (concurrent workspace churn can run 30-90+ minutes) and its own precedent (`terra-actor-green`
hit an identical "live peer edit, RED, do not touch" situation in the same `🔌️plugin`-adjacent
`🗣️dsl/🧬️schema` file earlier this same day, which self-resolved — this one has not yet, as of
13:38).

**Consequence, explicit:**
- Could not run `cargo build -p <plugin> --target wasm32-wasip2` for ANY plugin (needed by
  `describePluginComponent`'s emitter step) — so **no new `🛂️descriptor.semio`/`🔣️descriptor.json`
  were committed**, for playbook/fem/puzzle/trinity/block alike.
- Could not run `descriptor_is_fresh` for ANY plugin — so **nothing was added to
  `DESCRIPTOR_MIGRATED_PLUGINS`**, per this ticket's own rule 3 ("ratcheting a plugin whose
  declarations may still move turns the tree red for every session").
- Could not re-verify the **1,328-test regression baseline** (`os-kernel` 779, `os-kernel-db` 424,
  `plugin-host` 125/0/1) — it is also gated on `semio-framework-plugin` compiling. **Baseline status:
  UNKNOWN this session, not claimed unmoved, not claimed broken** — the crate it depends on cannot
  currently compile for reasons unrelated to this packet's edits (none of this packet's changes are
  in the dependency path of `os-kernel`/`os-kernel-db`, which do not depend on `semio-framework-plugin`
  — but `plugin-host`'s baseline does transit `semio-framework-plugin`, and cannot be re-measured
  right now).

## 6. Full 33-plugin table (fresh audit, this session)

| plugin | emitted | ratcheted | this-session status |
|---|:-:|:-:|---|
| note | yes | yes | unchanged |
| sequence | yes | yes | unchanged |
| vcs | yes | yes | unchanged |
| forms | yes | yes | unchanged |
| sourcing | yes | yes | unchanged |
| dag | yes | yes | unchanged |
| mathematical | yes | yes | unchanged |
| writer | yes | yes | unchanged |
| reasoning (`reasoning-mindmap`) | yes | yes | unchanged |
| animate | yes | yes | unchanged |
| draw | yes | yes | unchanged |
| energy | yes | yes | unchanged |
| layout | yes | yes | unchanged |
| procedural | yes | no | unchanged (pre-existing compile blocker, per scout) |
| flow | yes | no | unchanged |
| gis | yes | no | unchanged (pre-existing compile blocker, per scout) |
| shooting | yes | no | unchanged |
| architect | yes | no | unchanged |
| process | yes | no | unchanged |
| lowpoly | yes | no | unchanged |
| cad | yes | no | unchanged |
| norm | yes | no | unchanged |
| imperative | yes | no | unchanged (pre-existing compile blocker, per scout) |
| remodel | yes | no | unchanged |
| raster | yes | no | unchanged |
| space | yes | no | unchanged |
| **playbook** | **no** | no | **code-complete this packet, awaiting emission** |
| **fem** | **no** | no | **code-complete this packet, awaiting emission** |
| **puzzle** | **no** | no | **code-complete this packet, awaiting emission** |
| **trinity** | **no** | no | **already code-complete (prior packet), awaiting emission** |
| **block** | **no** | no | **already code-complete (prior packet), awaiting emission** |
| stdio | no | no | blocked — pre-existing capability claim-set repair needed, out of scope |
| demonstrator | no | no | blocked — cross-plugin `kit.catalog` design decision needed, out of scope |

**Totals: 26/33 emitted (unchanged), 13/33 ratcheted (unchanged). 31/33 are now either already
ratcheted, already emitted-pending-ratchet, or code-complete-pending-emission. Only stdio and
demonstrator remain genuinely blocked on work outside this packet's scope.**

## 7. Recommended next step (for the coordinator or a follow-up packet)

1. Wait for `🔌️plugin/🦀️component.rs` / `🧰️framework/🔨️modules/🖱️ui/**` to reach a committed, stable
   state (poll `git status`/`git log`, do not chase).
2. Forced-rebuild census `semio-framework-plugin --lib` (R12/R17: a red crate cannot report anything
   meaningful) to confirm green.
3. Run `describePluginComponent` (or each plugin's own `📜️script.ts describe`) for: playbook, fem,
   puzzle, trinity, block — five plugins, eight artifacts, all code-complete as of this report.
4. For each, run the crate's own `descriptor_is_fresh` test; on pass, commit the emitted
   `🛂️descriptor.semio` + `🔣️descriptor.json` and add the plugin id to `DESCRIPTOR_MIGRATED_PLUGINS`
   (playbook, fem2d's id is `"fem"` at the plugin level — one id per PLUGIN not per artifact, confirm
   against `plugin_manifest().plugin_id` at test time — likely `"playbook-play"`, `"fem"`,
   `"puzzle"`, `"trinity"`, `"block"`).
5. Re-measure `📇️registry:check`'s "descriptor gate: N/33" census line.
6. stdio and demonstrator remain separate, dedicated packets per the scout's own classification.
