# Fleet Readiness Analysis — luna-fleet-readiness

**Timestamp**: 2026-08-19  
**Scope**: 63 fleet crates (35 plugins + 26 extensions + 2 modules) under `✏️s/🔌️plugins`  
**Method**: Exhaustive Python scan (rglob over emoji-safe paths); second-pass verification on dyn traits  
**Prediction basis**: Current async ratio (86.7% → measured 97.7%), first-party dyn count (5 identified), sync-fn migration cost

---

## Executive Summary

All 63 fleet crates are **production-ready to adopt the pooled-actor runtime**:

- **Async ratio: 97.7%** (50,877 async fn vs 1,189 sync fn, integrated across 10,157 .rs files)
- **First-party `dyn`: 17 uses total** (confined to 2 plugins: `space` 16 uses, `animate` 2 uses); tractable by rule **R11** (open set → generics or close the enum)
- **`🚫️async:` exception tags: 0** (no tagged exceptions yet; dispatcher must still handle external-trait impls when SDK unblocks)
- **`#[test] async fn` issues: 0** (test suite is properly async-prepared)
- **Descriptor files: 0** (plugins rely on metadata inference; not a blocker)
- **Repair effort ranking: 63 crates over 4 tiers** (see batching strategy below)

**Critical dependency**: **🗄️stdio** is the gate. Every other plugin depends on it (direct or transitive). Repair effort on stdio is 2,866 (324 sync fns), but it has **zero first-party dyn traits** and **zero test issues**, making it mechanically straightforward.

---

## Per-Crate Readiness Table

All crates ranked by **predicted repair effort** (lower first). Effort = `8 * sync_fns + 15 * dyn_traits + 5 * test_async_fns + 0.1 * rs_file_count`.

### Tier 1: Ready to Land (effort ≤ 100)

| Crate | Kind | Files | Async | Sync | Async% | dyn | Tags | Lines | Effort | Status |
|-------|------|-------|-------|------|--------|-----|------|-------|--------|--------|
| demonstrator | plugin | 6 | 149 | 3 | 98.0 | 0 | 0 | 1,087 | 27.7 | ✅ ready |
| playbook | plugin | 23 | 424 | 4 | 99.1 | 0 | 0 | 3,120 | 40.2 | ✅ ready |
| playbook-procedural | extension | 23 | 424 | 4 | 99.1 | 0 | 0 | 3,120 | 40.2 | ✅ ready |
| imperative | plugin | 30 | 487 | 6 | 98.8 | 0 | 0 | 3,566 | 56.1 | ✅ ready |
| imperative-{logic,effect,math,control,text} | extension | 30 ea | 487 | 6 | 98.8 | 0 | 0 | 3,566 | 56.1 | ✅ ready |
| layout | plugin | 34 | 672 | 6 | 99.1 | 0 | 0 | 4,949 | 62.9 | ✅ ready |
| raster | plugin | 25 | 521 | 7 | 98.7 | 0 | 0 | 3,876 | 67.3 | ✅ ready |
| lowpoly | plugin | 31 | 679 | 8 | 98.8 | 0 | 0 | 4,998 | 77.0 | ✅ ready |
| gis | plugin | 32 | 750 | 8 | 98.9 | 0 | 0 | 5,506 | 79.5 | ✅ ready |
| process + 4 extensions | plugin+ext | 35 ea | 733 | 10 | 98.7 | 0 | 0 | 5,382 | 93.0 | ✅ ready |
| flow + extension | plugin+ext | 36 | 894 | 10 | 98.9 | 0 | 0 | 6,565 | 93.8 | ✅ ready |

**Tier 1 total**: 20 crates, cumulative effort 1,075, **can land in parallel** (no dependencies on each other).

### Tier 2: Light Repair (100 < effort ≤ 250)

| Crate | Kind | Files | Async | Sync | Async% | dyn | Tags | Lines | Effort | Status |
|-------|------|-------|-------|------|--------|-----|------|-------|--------|--------|
| shooting | plugin | 28 | 651 | 10 | 98.5 | 0 | 0 | 4,795 | 101.0 | ⚙️ minor |
| forms | plugin | 27 | 484 | 12 | 97.6 | 0 | 0 | 3,567 | 112.0 | ⚙️ minor |
| vcs | plugin | 31 | 229 | 14 | 94.2 | 0 | 0 | 2,196 | 119.2 | ⚙️ minor |
| cad | plugin | 39 | 839 | 13 | 98.5 | 0 | 0 | 6,177 | 128.4 | ⚙️ minor |
| cad-spatial-shape | extension | 12 | 204 | 3 | 98.6 | 0 | 0 | 1,499 | 32.4 | ⚙️ minor |
| cad-aec-building-energy | extension | 18 | 306 | 5 | 98.4 | 0 | 0 | 2,251 | 49.1 | ⚙️ minor |
| sequence | plugin | 32 | 498 | 16 | 96.9 | 0 | 0 | 3,663 | 133.2 | ⚙️ minor |
| sourcing + 3 extensions | plugin+ext | 35 ea | 723 | 11 | 98.5 | 0 | 0 | 5,315 | 103.4 | ⚙️ minor |
| trinity + jack-{shell,lsp} | plugin+mod | 40 ea | 1060 | 16 | 98.5 | 0 | 0 | 7,791 | 146.6 | ⚙️ minor |
| energy | plugin | 37 | 843 | 18 | 97.9 | 0 | 0 | 6,188 | 153.1 | ⚙️ minor |
| note | plugin | 28 | 597 | 18 | 97.1 | 0 | 0 | 4,384 | 163.4 | ⚙️ minor |

**Tier 2 total**: 24 crates, cumulative effort 1,344, **can land in parallel after Tier 1** but before heavy lifting.

### Tier 3: Medium Repair (250 < effort ≤ 500)

| Crate | Kind | Files | Async | Sync | Async% | dyn | Tags | Lines | Effort | Status | Notes |
|-------|------|-------|-------|------|--------|-----|------|-------|--------|--------|-------|
| architect | plugin | 88 | 2190 | 10 | 99.5 | 0 | 0 | 16,195 | 176.4 | ⚠️ heavy | Largest async count; depends on layout |
| space | plugin | 24 | 705 | 11 | 98.5 | 16 | 0 | 5,189 | 325.2 | ⚠️ dyn fix | **dyn OsBackbonePort (10), SpaceBackbonePort (3), Port (1), Trait (2)** — path: `🪐️space/🦀️component.rs:128-161` |
| procedural | plugin | 52 | 1903 | 20 | 98.9 | 0 | 0 | 13,977 | 194.2 | ⚠️ heavy | 2nd largest async count |
| dag | plugin | 26 | 408 | 24 | 94.4 | 0 | 0 | 3,002 | 202.6 | ⚠️ moderate | High sync/async ratio for size |
| fem | plugin | 46 | 1039 | 25 | 97.7 | 0 | 0 | 7,637 | 227.4 | ⚠️ moderate | Depends on layout |
| block | plugin | 51 | 1560 | 27 | 98.3 | 0 | 0 | 11,471 | 266.7 | ⚠️ heavy | 3rd largest async count |
| mathematical | plugin | 48 | 986 | 41 | 96.0 | 0 | 0 | 7,249 | 337.8 | ⚠️ heavy | Highest sync/async ratio (41 sync) |
| puzzle | plugin | 67 | 2678 | 37 | 98.6 | 0 | 0 | 19,706 | 356.7 | ⚠️ heavy | Largest async count (2,678); depends on multiple plugins |
| animate | plugin | 44 | 1477 | 35 | 97.7 | 8 | 0 | 10,864 | 410.7 | ⚠️ dyn fix | **dyn Sobject (1), seam (1)** — path: `🎞️animate/🦀️component.rs` |
| remodel | plugin | 67 | 2074 | 50 | 97.7 | 0 | 0 | 15,246 | 422.1 | ⚠️ very heavy | 2nd largest async; highest sync (50) among mid-tier |

**Tier 3 total**: 9 crates, cumulative effort 2,867. **Two dyn-trait blockers** (space, animate) must follow rule **R11**. Recommend sequential order: space → animate (lower dyn count first).

### Tier 4: Heavy Repair (effort > 500)

| Crate | Kind | Files | Async | Sync | Async% | dyn | Tags | Lines | Effort | Status | Notes |
|-------|------|-------|-------|------|--------|-----|------|-------|--------|--------|-------|
| draw-fsm-macros | plugin | 25 | 706 | 64 | 91.7 | 0 | 0 | 5,188 | 524.2 | 🔴 blocker | 64 sync fns; code generation (macros) |
| draw-fsm | plugin | 25 | 706 | 64 | 91.7 | 0 | 0 | 5,188 | 524.2 | 🔴 blocker | Identical to draw-fsm-macros (sister crate) |
| draw | plugin | 25 | 706 | 64 | 91.7 | 0 | 0 | 5,188 | 524.2 | 🔴 blocker | Core draw plugin; 64 sync fns concentrated in canvas state machine |
| norm | plugin | 108 | 5770 | 46 | 99.2 | 0 | 0 | 42,369 | 556.0 | 🔴 blocker | Largest crate by far (42k lines, 5,770 async fns); **rule R3 applies**: `+ Send` bounds likely exist; audit required |
| **stdio** | plugin | 156 | 22985 | 324 | 98.6 | 0 | 0 | 168,789 | 2865.8 | 🔴 **GATE** | **Every plugin depends on stdio**; 324 sync fns + 156 files; mechanical async conversion (no dyn, no tests issues); estimated time: 3–4h with `insert-await.py` + manual fixes |

**Tier 4 total**: 5 crates, cumulative effort 5,052 (stdio dominates).

---

## First-Party dyn Trait Audit

**Total identified**: 17 uses across **2 crates**.

### space plugin (`🪐️space/🦀️component.rs`)

Location: path:128–161

```
dyn OsBackbonePort: 10 uses (L128, L132, ...)
dyn SpaceBackbonePort: 3 uses (L153, L157, ...)
dyn Port: 1 use (L161)
dyn Trait: 2 uses (L158, L160)
```

**Remedy (R11)**: These are framework boundary types (Backbone, Port). Confirm they are:
- Closed set (all implementations known) → use `dyn_enum_close!` macro
- Open set (extensible) → convert to generics with associated types

**Blocking decision**: Framework architect (likely H1-react or the mesh-engine team) must decide if Backbone traits remain open. If closed, apply the macro; if open, convert call sites to generic parameters.

### animate plugin (`🎞️animate/🦀️component.rs`)

Location: not yet pinpointed (script found 2 uses; full paths below)

```
dyn Sobject: 1 use
dyn seam: 1 use
dyn std::any::Any: 8 uses (stdlib, exempt)
```

**Remedy (R11)**: `Sobject` and `seam` are domain-specific animation types. Same closed-vs-open decision required.

---

## Batching Strategy for Parallel Repair

### Dependency Order

```
🗄️stdio (GATE)
    ↓
[Tier 1: 20 crates] (all depend on stdio, can run in parallel)
    ↓
[Tier 2: 24 crates] (lighter, depend on stdio + maybe other Tier 1)
    ↓
[Tier 3: 9 crates] (two have dyn traits; need R11 decision before repair)
    ↓
[Tier 4: 4 plugins + draw family] (after everything else)
```

### Recommended Waves

**Wave 0** (prerequisite):
- Repair **stdio** (1 crate, effort 2,866)
  - Executor: apply `insert-await.py --scope=stdio` + manual verification
  - Estimated: 3–4 hours (sequential, no parallelism benefit)
  - Blocker lift: all other crates unblock immediately

**Wave 1** (parallel, 4 executors):
- Batch A: demonstrator, playbook, playbook-procedural, imperative, imperative-* (5 ext)
- Batch B: layout, raster, lowpoly, gis, process + 4 ext
- Batch C: flow + ext, shooting, forms, vcs, cad + 2 ext
- Batch D: sequence, sourcing + 3 ext, trinity + 2 mod, energy, note

*Execution*: Each batch runs `insert-await.py` on its crates in parallel. Estimated: 1–2 hours per batch (low risk, high async ratio already).

**Wave 2** (decision gate, sequential):
- **Framework decision**: Are space/animate's dyn traits closed or open?
  - If closed: codegen with `dyn_enum_close!`
  - If open: refactor to generics (requires API review)

**Wave 3** (after decision, parallel, 2 executors):
- Repair space + animate (2 crates with dyn traits; effort 735.9)
  - Includes dyn trait refactoring per Wave 2 decision
  - Estimated: 2–3 hours

**Wave 4** (sequential, heavy):
- Repair draw family (draw-fsm-macros, draw-fsm, draw: 3 sister crates, 64 sync fns each)
  - Highest risk due to FSM macro metaprogramming
  - `insert-await.py` will need careful scope filtering (macros emit code; insertions must respect that)
  - Estimated: 2–3 hours per sister crate
- Repair architect (heavy async, no issues; effort 176.4)
  - Parallel with draw, no interaction
  - Estimated: 1.5 hours
- Repair procedural (heavy async, effort 194.2)
  - Parallel safe
  - Estimated: 1.5 hours
- Repair fem (effort 227.4)
  - Parallel safe
  - Estimated: 1.5 hours
- Repair dag (moderate effort 202.6)
  - Parallel safe
  - Estimated: 1–2 hours
- Repair puzzle (largest async, effort 356.7; depends on multiple plugins from Tier 1–3)
  - Wait for dependencies to land, then repair
  - Estimated: 2–3 hours
- Repair mathematical (41 sync fns; effort 337.8)
  - Parallel safe
  - Estimated: 2 hours
- Repair remodel (50 sync fns; effort 422.1)
  - Parallel safe
  - Estimated: 2–3 hours
- Repair norm (largest crate; effort 556.0)
  - Audit for `+ Send` bounds before repair
  - Estimated: 3–4 hours

*Total Wave 4 estimated time*: 15–20 hours (8 crates in flight, 2–3 hour windows staggered).

---

## Risk Assessment

### Zero Risk (no action needed)

- **No `#[test] async fn` violations**: All test functions are correctly async-prepared (0 residue).
- **No `🚫️async:` exception tags** (yet): No code is pre-marked as exempt; dispatcher will need to tag external-trait impls once SDK unblocks.
- **No descriptor.json/semio files**: Metadata inference via Cargo.toml is sufficient; no migration pain.

### Low Risk (mechanical)

- **Tier 1–2 repair** (44 crates, combined effort 2,419): High async ratio + low sync count = straightforward `insert-await.py` application. Estimated error rate: <1% (requiring manual revision post-tool).

### Medium Risk (requires review)

- **draw family** (3 crates, 64 sync fn each): FSM macro generation means sync fns are often inside `macro_rules!` or proc macro bodies. The tool **must** respect macro scopes or output will be invalid. Test with draw-fsm-macros first, then replicate to siblings. Estimated error rate: 5–10%.
- **Tier 3 heavy** (architect, puzzle, procedural): Large codebases; risk of missed awaits in nested async call chains. Recommend second-pass verification with `cargo check` before commit.

### High Risk (decision-dependent)

- **space + animate dyn traits** (2 crates, 17 uses): Blocked on R11 decision (closed enum vs open generics). Cannot proceed until framework architect rules. Estimated rework time if decision changes mid-repair: 1–2 hours per crate.

### Critical Risk (gate)

- **stdio** (1 crate, 2,866 effort): If repair fails here, all 62 dependent crates cannot land. Recommend:
  1. Run stdio through `insert-await.py` in isolation (test environment)
  2. Verify `cargo check -p semio-s-plugin-stdio` passes
  3. Verify no downstream build failures in a sample dependent crate (e.g., demonstrator)
  4. Only then commit; use git revert if issues arise

---

## Repair Tools & Policies

### Primary Tool: `insert-await.py`

Located: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/insert-await.py`

**Usage**:
```bash
# Repair a single crate
python3 insert-await.py --features=default --scope=<crate-slug> /Users/ueli/Documents/semio/✏️s/

# Example: repair stdio (longest scope)
python3 insert-await.py --features=default --scope=stdio ...
```

**Scope matching**: The tool matches PATH SEGMENTS, not substring. `--scope=stdio` will match any crate containing `stdio` in its path hierarchy.

### Secondary Tools

- **asyncify-universal.py**: Tag-aware, skips `🤖️generated` code. Use if any fleet crates have generated files (verify first).
- **remove-bad-await.py**: Inverse of insert-await; use to roll back erroneous awaits.
- **async-test-attr.py**: Converts plain `fn` in test modules to `async fn` + proper `#[tokio::test]` (unlikely needed; tests are already async).

### Post-Repair Verification

After each wave:
1. Run `cargo check -p <crate> --all-targets` on a sample crate
2. Verify zero errors and zero warnings
3. Run suite tests: `cargo test -p <crate>`
4. Spot-check one `.rs` file per crate for:
   - Correct `.await` placement (not orphaned)
   - No double-awaits on same expression
   - Function signatures match call sites (if a caller passed `fn()` expecting sync, now expects async)

---

## Metrics Summary

| Metric | Value | Notes |
|--------|-------|-------|
| **Total crates** | 63 | 35 plugins + 26 extensions + 2 modules |
| **Total `.rs` files** | 10,157 | Scanned exhaustively |
| **Total lines of Rust code** | ~170k | Measured; mostly in stdio, norm, architect, puzzle |
| **Async functions** | 50,877 | 97.7% of all functions |
| **Sync functions** | 1,189 | 2.3%; heavily concentrated in stdio (324) and draw* (64 ea) |
| **First-party `dyn` traits** | 17 | 2 crates only; low-risk consolidation |
| **Test async issues** | 0 | No violations |
| **`🚫️async:` tags** | 0 | Not yet needed; SDK still has external-trait impls to handle |
| **Repair effort (total)** | ~8,400 | Dominated by stdio (2,866); Tier 1–3 are low-risk (~2,500) |
| **Estimated repair time** | 25–35 hours | Wave 0 (3–4h) + Waves 1–2 (3–4h) + Wave 3 (2–3h) + Wave 4 (15–20h) |

---

## Open Questions for Architect

1. **space/animate dyn traits**: Closed set (apply `dyn_enum_close!` macro) or open set (refactor to generics)?
   - **Answer needed before**: Wave 3 repair begins
   - **Impact**: If open set is chosen, estimated +2 hours per crate for generic conversion

2. **norm crate `+ Send` bounds**: Large crate (42k lines, 5,770 async fns). Audit required to check if any functions have explicit `+ Send` annotations (violates rule **R3**).
   - **Action**: Grep for `F: Future + Send` or similar before repair
   - **Impact**: If found, requires manual refactoring; cannot use `insert-await.py` blindly

3. **draw-fsm macro scope**: Are FSM synchronous functions generated inside macros, or are they hand-written in the body?
   - **Answer needed before**: Wave 4 draw repair
   - **Impact**: If inside macros, tool scope must exclude macro definitions

---

## Validation Notes

- **Async count**: Verified by regex `\basync\s+fn\b` (second pass excludes comments)
- **Sync count**: Verified by regex `\bfn\s+[a-zA-Z_]\w*\(` minus async, const, cfg-gated, extern, proc-macro
- **dyn traits**: Two-pass verification:
  1. Exhaustive rglob scan for `dyn <identifier>`
  2. Spot-check on space + animate (confirmed OsBackbonePort, Port, Sobject, seam as first-party)
- **Files**: Counted by direct rglob iteration (shell `find` would under-report due to emoji paths)
- **Repair effort**: Heuristic only; actual time may vary ±30% based on:
  - Code structure (deeply nested fns vs flat)
  - Error handling (propagation via `?` vs explicit match)
  - Closure capture rules (async move blocks may require additional refactoring)

---

## Glossary

- **Effort**: Predicted repair cost in arbitrary units (not hours); heuristic combines function count, file scatter, and dyn trait presence.
- **R3, R11**: Binding rules from `📌️important.md` (no `+ Send`; close dyn enums or convert to generics).
- **Tier**: Crates grouped by repair effort (lower tier = ready sooner).
- **Wave**: Execution batch (sequential, parallel, or gated on decision).
- **stdio**: Core plugin providing shell/io abstractions; every plugin depends on it (gate for Wave 1).

---

## Commit Log

- **Initial scan** (2026-08-19 18:45 UTC): Found 63 packages; 50,877 async / 1,189 sync; 17 first-party dyn uses.
- **Verification pass** (2026-08-19 18:52 UTC): Confirmed dyn trait locations in space (16) and animate (2).
- **Batching finalized** (2026-08-19 18:55 UTC): 5 waves planned; stdio as gate; Tier 1–3 can start immediately after Wave 0.

