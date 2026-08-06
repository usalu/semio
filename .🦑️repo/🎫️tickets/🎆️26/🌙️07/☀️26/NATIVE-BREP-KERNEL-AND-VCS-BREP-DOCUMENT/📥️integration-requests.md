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
