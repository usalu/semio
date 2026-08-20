# 📓 Luna Block-On Census (Re-Measured 2026-08-20)

Re-measurement of all 1,028 block_on(  call sites in the first-party codebase.

## Quick Summary

| Status | Count |
|---|---|
| ✅ Sanctioned (a-e) | 737 |
| ❌ NOT SANCTIONED (f-*) | 291 |
| **Total** | **1,028** |

### Sanctioned Breakdown

- **(a)** Binary/main executor entry: **11 sites** (e.g., 🏃️run/📦️bin.rs)
- **(b)** Dedicated-thread actor bridge: **492 sites** (actor::block_on, db bridges)
- **(c)** Spawn_blocking wrapper: **0 sites** (not used)
- **(d)** E5-tagged executor bridge: **51 sites** (🚫️async: E5 annotation)
- **(e)** Test code (#[test]): **183 sites** (in #[cfg(test)] blocks)

### NOT SANCTIONED Breakdown

- **Out-of-scope (compose root, R3)**: 48 sites
- **Plugin code (needs real await)**: 129 sites — **HIGH RISK**
- **Unknown/needs review**: 88 sites
- **UI/render thread**: 16 sites — **HIGH RISK**
- **Network I/O path**: 10 sites

## Distribution by Crate

| Category | Count | Examples |
|---|---|---|
| compose/client (out of scope) | 48 | Root compose tree, R3 says ignore |
| dispatch tests | 43 | scale.rs: test code, NOT in final count |
| Plugin code | 129 | flow/brep, cad, stdio, process — all need real await |
| Framework test modules | ~183 | pack, async, kernel, run modules |
| Services/DB bridges | ~492 | actor::block_on, runtime.block_on (sanctioned) |

## Metrics

- **pending_effects**: 43 references (target: 0)
- **register_job_kind**: 13 references (target: >0 per CPU-heavy plugin)

## High-Risk NOT SANCTIONED Sites

### 1. Plugin Code (129 sites) — HIGHEST PRIORITY

Patterns like blocking inside async function bodies:

```
async fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
    with_kernel(|k| {
        let handle = block_on($expr)?;
        Ok(...)
    })
}
```

**Affected plugins**:
- flow/brep (39 sites)
- cad (>30 sites)
- stdio (13 sites)
- process (13 sites)

**Issue**: block_on() inside async function bodies. The with_kernel() macro pattern suggests these should be refactored to use kernel-provided async entry points.

### 2. UI/Render Thread (16 sites) — CRITICAL

Example: 2D engine, drawing/render modules using pollster::block_on().

**Issue**: Never sanctioned. Winit event loop cannot block.

## Next Steps

1. **Compose client (48)**: OUT OF SCOPE per R3. Ignore.
2. **Plugin code (129)**: Refactor macros and entry points to support real async.
3. **UI thread (16)**: Move I/O off render thread, use proper async patterns.
4. **Unknown (88)**: Requires per-site review.

---

**Generated**: 2026-08-20
**Classification method**: Grep-based context analysis (no cargo builds, read-only)
**Review status**: Scout luna-blockon, read-only pass complete
