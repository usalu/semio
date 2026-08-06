# Integration Requests (append-only)

**Audience:** Integrator agent (owns root `Cargo.toml`, `Cargo.lock`, `📜️script.ts`, nx/eslint/dependency-cruiser, `launch.json`).

**Format:** Append new sections at the **bottom**. Never edit or delete prior entries.

```markdown
## YYYY-MM-DD — <wave/agent id> — <short title>

**Why:** one sentence

**Files / globs:**
- `path/or/glob` — what to change

**Exact ask:**
- [ ] bullet list of concrete edits (members, deps, scripts, launch config)

**Depends on:** ticket/wave ids or "none"

**Status:** open | applied | rejected — <note>
```

---

<!-- entries below this line -->

## 2026-08-06 — Wave 1a M2 Engine — ArtifactKind::Engine

**Why:** Engine derive/read WIT imports need a capability gate; `ArtifactKind` in framework core has no `Engine` variant yet.

**Files / globs:**
- `🧰️framework/🔨️modules/🧩core/🧩️ui/🧠️kernel/🦀️component.rs` — `ArtifactKind` enum

**Exact ask:**
- [ ] Append `Engine` variant to `ArtifactKind` (after `Backbone`), so host can grant `CapabilityRequirement { artifact: ArtifactKind::Engine, rights: Invoke|Read, … }` for `engine-derive` / `engine-read`.
- [ ] Do **not** change OS kernel `Cargo.toml` for blake3/thiserror — already present and used by engine via glue path-include.

**Depends on:** Wave 1b WIT imports (`engine-derive` / `engine-read`)

**Status:** open — M2 engine module shipped without editing framework core (Wave 2 / integrator ownership).
