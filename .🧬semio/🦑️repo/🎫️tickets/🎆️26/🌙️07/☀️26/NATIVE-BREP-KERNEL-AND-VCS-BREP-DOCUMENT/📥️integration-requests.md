# Integration Requests (append-only)

**Audience:** Integrator agent (owns glue.rs, Cargo.toml, project.json, script.ts, launch.json, engine trait, kernel flip).

**Format:** Append new sections at the **bottom**. Never edit or delete prior entries.

```markdown
## YYYY-MM-DD — <wave/agent id> — <short title>

**Why:** one sentence

**Files / globs:**
- `path/or/glob` — what to change

**Exact ask:**
- [ ] bullet list of concrete edits

**Depends on:** lane ids or "none"

**Status:** open | applied | rejected — <note>
```

---

<!-- entries below this line -->

## 1-tessellate → 1-int-cc (2026-08-06)
- Observed duplicated/corrupted closing brace block after `intersect_circle_circle` that broke `cargo test -p semio-s-3d` for all lanes.
- Tessellate lane removed only the duplicated garbage so Wave-1 verification could finish; please re-check `✂️int-cc/🦀️component.rs` ownership integrity.

## 2026-08-06 — 3-heal — Wire native heal into BrepkitKernel

**Why:** `🩹heal/🦀️component.rs` is FROZEN with `heal_solid`, `defeature`, and `convert_to_nurbs` on `crate::brep::topo::Body`.

**Files / globs:**
- `📐️brep/🧰️kernel/🦀️component.rs` — replace `brepkit_operations::heal::*` and `brepkit_operations::defeature::defeature`

**Exact ask:**
- [ ] `heal_solid_sync` → `crate::brep::heal::heal_solid(&mut self.topo, solid, tolerance)`
- [ ] `convert_to_nurbs_sync` → `crate::brep::heal::convert_to_nurbs(&mut self.topo, solid)`
- [ ] `defeature_sync` → `crate::brep::heal::defeature(&mut self.topo, solid, &face_ids)`

**Depends on:** 3-heal FROZEN

**Status:** open

## 2026-08-06 — 3-classify — measure delegates to classify

**Why:** `classify` lane now owns solid/UV point classification; `measure::classify_point_on_solid` still duplicates ray parity.

**Files / globs:**
- `📏measure/🦀️component.rs` — replace `classify_point_on_solid` body

**Exact ask:**
- [ ] Map `classify::point_in_solid` → `PointSolidClassification` (same variants as today)
- [ ] Remove private ray parity helpers if unused

**Depends on:** 3-classify FROZEN

**Status:** open
