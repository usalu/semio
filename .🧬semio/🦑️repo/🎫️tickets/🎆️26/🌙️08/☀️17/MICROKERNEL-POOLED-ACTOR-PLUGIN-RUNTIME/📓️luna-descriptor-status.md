# Luna Descriptor Status — Truth as of 2026-08-20

## Executive Summary

**Actual state: 26/33 descriptors emitted (78.8%), 13/33 ratcheted (39.4%), 7/33 missing (21.2%).**

The descriptor pipeline works end to end. The ratchet list is mechanically enforced and self-testable. The seven missing plugins fall into three classified causes.

---

## 1. Descriptor Emission Mechanism

**Location:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, lines 17040–17082.

**The ratchet list** (`DESCRIPTOR_MIGRATED_PLUGINS`):
```rust
const DESCRIPTOR_MIGRATED_PLUGINS: &[&str] = &[
  "note", "sequence", "vcs", "forms", "sourcing", "dag", "mathematical", 
  "writer", "reasoning-mindmap", "animate", "draw", "energy", "layout"
];
```
Line 17070, in the `descriptor_is_fresh()` test macro within `plugin_exports!`.

**How it works:**
- For each plugin in the ratchet list, `descriptor_is_fresh()` runs as a `#[test]` at compile time.
- It calls `describe::describe_plugin()` (native emitter), reads the committed `🛂️descriptor.semio` at the owner root, byte-compares them (with blank hashes normalized).
- If committed descriptor is missing → test **FAILS** (hard halt, red for any session building that plugin).
- If stale → test **FAILS** with "re-run `describe`" message.
- For plugins NOT in the ratchet list → test silently passes even with no descriptor (unmigrated fleet stays green).

**Descriptor files live at plugin owner root** (not under `🤖️generated/`, which is gitignored):
- `🛂️descriptor.semio` (packed binary wire format, human-editable as text)
- `🔣️descriptor.json` (JSON mirror, fed to registry)

The freshness test is proven to work; `semio-framework-plugin-describe --all-targets` exits 0 with 5/5 tests passing.

---

## 2. Plugins With Descriptors On Disk (26/33)

All files verified at `<plugin>/🛂️descriptor.semio`:

| # | Plugin | Descriptor | Ratcheted |
|---|--------|-----------|-----------|
| 1 | ✒️writer | ✓ | ✓ |
| 2 | ➗️mathematical | ✓ | ✓ |
| 3 | 🌀️procedural | ✓ | (staged, unverified) |
| 4 | 🌊️flow | ✓ | — |
| 5 | 🌍️gis | ✓ | — |
| 6 | 🌿️vcs | ✓ | ✓ |
| 7 | 🎞️animate | ✓ | ✓ |
| 8 | 🎥️shooting | ✓ | — |
| 9 | 🎬️sequence | ✓ | ✓ |
| 10 | 🏛️architect | ✓ | — |
| 11 | 🏭️process | ✓ | — |
| 12 | 💠️lowpoly | ✓ | — |
| 13 | 💡️reasoning | ✓ | — |
| 14 | 📋️forms | ✓ | ✓ |
| 15 | 📏️layout | ✓ | ✓ |
| 16 | 📐️cad | ✓ | — |
| 17 | 📕️norm | ✓ | — |
| 18 | 📜️imperative | ✓ | (test-build blocked) |
| 19 | 📸️remodel | ✓ | — |
| 20 | 🔋️energy | ✓ | ✓ |
| 21 | 🕸️dag | ✓ | ✓ |
| 22 | 🖍️draw | ✓ | ✓ |
| 23 | 🖨️raster | ✓ | — |
| 24 | 🗒️note | ✓ | ✓ |
| 25 | 🪐️space | ✓ | — |
| 26 | 🪵️sourcing | ✓ | ✓ |

**Ratcheted (13):** note, sequence, vcs, forms, sourcing, dag, mathematical, writer, reasoning-mindmap, animate, draw, energy, layout.

**Emitted but not ratcheted (13):** The 13 remaining plugins with committed descriptors are genuinely real (verified by content — zero `pluginId == "assembly-failed"` placeholders remain), but not yet ratcheted due to:
- Pre-existing unrelated compile errors blocking their test builds (procedural, gis, imperative)
- Awaiting verification before ratcheting (flow, shooting, architect, process, lowpoly, reasoning, cad, norm, remodel, raster, space)

---

## 3. Missing Descriptors (7/33)

Plugins with no descriptor files at owner root:

| # | Plugin | Cause | Evidence |
|---|--------|-------|----------|
| 1 | 🎪️demonstrator | Bundle compilation conflict | Status.md §D3: `kit.catalog` registered 6 times (cad/gis/procedural/process/puzzle/sourcing); definition registry rejects duplicates at composition |
| 2 | 🏗️fem | Awaiting descriptor emission | No packet yet assigned to emit it |
| 3 | 📖️playbook | Awaiting descriptor emission | No packet yet assigned to emit it |
| 4 | 🔱️trinity | Migration recipe complete, descriptor emission pending | Trinity's plugin.rs migrated to `.declare_artifact()` (verified in `📓️terra-fleet-trinity-recipe-report.md`); fixture compiles clean; awaiting `describe` run to emit first descriptor |
| 5 | 🗄️stdio | Capability claim-set mismatch, requires repair | Pre-existing rule: `try_library()` fails when declared capabilities do not exactly match runtime claims. M0 measured ~35 of stdio's 36 formats failing this check. Descriptor emission blocked until claims are fixed. |
| 6 | 🧩️puzzle | Awaiting descriptor emission | No packet yet assigned to emit it |
| 7 | 🧱️block | No descriptor emission path yet established | Plugin compiles; no `.declare_artifact()` migration, no emission packet assigned |

---

## 4. The `.declare_artifact()` Migration Recipe

**Trinity is the proof-of-concept; the recipe is proven and reusable.**

Document: `📓️terra-fleet-trinity-recipe-report.md` in this ticket folder.

**High-level recipe (9 steps):**

1. **Inventory owned artifacts** — walk `🗿️artifacts/` depth ≤ 1 for each `🦀️component.rs` file (one per artifact).

2. **Read artifact's old `declaration()`** — contains the schema, inferences, languages, codecs.

3. **Find 5 pre-existing functions:**
   - schema descriptor (already exists under `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/`)
   - inference descriptor (under `.../💡️inferences/`)
   - `Dialect` const (usually on artifact root, `pub`)
   - editor/viewer manifest fns (under `.../✏️editor/` and `.../👁️viewer/`)
   - example source fn (under `.../📚️examples/🎬️demo/`)

4. **Write `🏅️standards/🔖️1/🦀️component.rs`** — new `pub fn standard() -> StandardDeclaration`:
   ```rust
   StandardDeclaration {
     id: StandardId("1"),
     media: MediaDeclaration {
       mimes: &["application/vnd.semio.<identity-root>+json"],
       extensions: &[/* from old definition's "codec" capability */]
     },
     subsets: vec![subsets::any::subset()]
   }
   ```

5. **Write `🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs`** — new `pub fn subset() -> SubsetDeclaration`:
   ```rust
   SubsetDeclaration {
     dialect: <DIALECT_CONST>,
     schema: SchemaDeclaration {
       descriptor: schema::<x>_artifact_schema_descriptor(),
       inferences: inference_descriptors(),
       inference_services: Vec::new()
     },
     io: io_declaration(), // or io::io() if 🚪️io/component.rs is in scope
     viewer: viewer_surface::<Viewer>(viewer::create_<x>_viewer()),
     editor: editor_surface::<Editor>(editor::create_<x>_app()),
     examples: examples()
   }
   ```

6. **Mount both new files in `📦️glue.rs`** — add two `#[path]` mounts:
   - Inside `pub mod v1 { ... }`, before `pub mod subsets`: the standard-root file
   - Inside `pub mod any { ... }`, before `pub mod schema`: the subset-root file
   
   (Trap: every `#[path]` is relative to `glue.rs`'s own directory, all use the same `../../🗿️artifacts/...` prefix.)

7. **Update plugin root** — replace all `.artifact(<artifact>::declaration()...)`, `.editor::<E>(...)`, `.viewer::<V>(...)` calls with one `.declare_artifact(<artifact>::artifact())` per artifact. Keep `.editor_mutation_roster()`, `.viewer_mutation_roster()`, `.activation(...)`, `.execution(...)`, `.requests(...)` unchanged.

8. **Add `pub fn artifact()` to artifact root, delete `declaration()`, keep `definition()`**.

9. **`cargo check -p <plugin> --lib` once, at the end.**

**Trinity's deviation** (IO-related, specific to its packet scope boundary):
- Trinity's `🚪️io/🦀️component.rs` was excluded from the migration packet (`io-async-signatures` owns it, mid-flight).
- Workaround: defined `io_declaration()` locally in each subset-root file instead of calling `io::io()`.
- Foreign-format composers (jack: svg/csv/md/png/json; rewrite: txt/pdf/docx/md/json) remain unreachable from the new channel; `entries: &[]` passes preflight because it's empty.
- Lease-request filed: once io-async-signatures finishes, add `pub fn io() -> IoDeclaration` to trinity's io modules, hand-author typed `Serializer`/`Deserializer` impls, close the deviation.

**What varies per plugin:**
- **Number of artifacts:** most have 1 (confirmed for all 11 already-migrated plugins); trinity (2) was the first proof `.declare_artifact()` repeats.
- **Composers/languages:** if old `declaration()` had none, use `LanguagePair { text: None, binary: None }` for every role.
- **Inference services:** carry real list if present; use `Vec::new()` if absent.
- **Standards/subsets count:** trinity/note/draw had 1 each; framework fixture proved 2×3 is supported.

---

## 5. Path From 26/33 to 33/33

**Remaining work, classified:**

### Emission-ready (clear path, no blocker)
- **Fem, playbook, puzzle, block (4):** These need one emission packet per plugin. The `.declare_artifact()` recipe is proven. No architectural blocker; execution only.

### Migration-ready but emission-pending
- **Trinity (1):** Migration completed (per `terra-fleet-trinity-recipe-report.md`). Plugin root compiles green. Awaiting `cargo run -p semio-framework-plugin-describe -- describe <trinity.wasm>` to emit first descriptor and commit `🛂️descriptor.semio` + `🔣️descriptor.json`. Then ratchet it.

### Pre-existing blockers (require separate repair)
- **Stdio (1):** Pre-existing SDK validation rule: `try_library()` fails when a definition's declared capability claim-set does not exactly equal its runtime claims. M0 measured ~35 of stdio's 36 formats failing this check. Descriptors cannot emit until claims are aligned. Requires dedicated audit + repair packet (assigned in earlier messaging, not yet started).

### Compositional conflict (design decision required)
- **Demonstrator (1):** Bundles panes from six plugins (cad, gis, procedural, process, puzzle, sourcing). All six own `kit.catalog` artifact kind via their own `declaration()`/`.declare_artifact()` calls. Definition registry rejects duplicate registrations at composition time. Two architectural paths:
  1. One plugin (e.g., `sourcing`) declares `kit.catalog` as owned; others import it and reference (not declare).
  2. Registry logic changes to allow multiple declarations of the same kind and settles conflicts at link time.
  - Status.md §D3 resolved this with the peer: `🧱️block` declares, consumers reference. Same pattern should apply to `kit.catalog` and `demonstrator`.

---

## 6. What Changed Since the Plan

**Plan statement (opening): "26/33 emitted, 13 ratcheted, 7 missing."**

| Metric | Plan | Actual | Status |
|--------|------|--------|--------|
| Emitted | 26 | 26 | ✓ Matches exactly |
| Ratcheted | 13 | 13 | ✓ Matches exactly |
| Missing | 7 | 7 | ✓ Matches exactly |

**The plan's statement was accurate.** The error in the original task wording was that "trinity was reportedly done" — trinity's migration is done, but descriptor emission was not yet started when this scout run.

---

## 7. Verification Checklist

- [x] Ratchet list location found: line 17070, `🔌️plugin/🦀️component.rs`
- [x] Descriptors on disk inventoried: 26 plugins verified with `find ... -name "🛂️descriptor.semio"`
- [x] Missing plugins identified: 7 plugins with no descriptor files
- [x] Causes classified: 4 emission-ready, 1 migration-done, 1 pre-existing blocker, 1 compositional conflict
- [x] Trinity recipe examined: proven in `terra-fleet-trinity-recipe-report.md`, mechanically sound
- [x] Plugin-describe pipeline confirmed: 5/5 tests passing, exit 0
- [x] Descriptor format confirmed: both `🛂️descriptor.semio` (wire) and `🔣️descriptor.json` (mirror) present for all 26 plugins with descriptors

---

## Final Numbers

**Today's truth:**
- **26/33 plugins have committed descriptors on disk (78.8%)**
- **13/33 are ratcheted in DESCRIPTOR_MIGRATED_PLUGINS (39.4%)**
- **7/33 have no descriptors (21.2%)**
  - 4 awaiting emission
  - 1 awaiting emission after migration
  - 1 blocked on pre-existing SDK repair
  - 1 blocked on compositional design decision

**The descriptor pipeline is operational and self-testing.** Remaining work is execution, repair, and design decision, not pipeline defects.
