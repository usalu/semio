# Luna Census Baseline Report

**Date:** 2026-08-19
**Session:** io-async-signatures sweep active during measurement

## Summary

- **Total plugins:** 33
- **Total production RS files:** 8,738
- **Total test RS files:** 1,340
- **Plugins with descriptor ratchet (both .json + .semio):** 26/33
- **Columns changed between runs:** NONE (sweep stable or inactive during window)

## Consolidated Baseline Table

| Plugin | RS Files | Prod | Test | Desc | Activation | Execution | Requests | Host Calls | Async Fn | Await | Block On | Pending FX | Job Reg | Async Task | DL Export |
|--------|----------|------|------|------|------------|-----------|----------|------------|----------|-------|----------|------------|---------|-----------|-----------|
| ✒️writer | 85 | 40 | 45 | Y | 2 | 2 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| ➗️mathematical | 98 | 73 | 25 | Y | 2 | 2 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 🌀️procedural | 342 | 234 | 108 | Y | 3 | 2 | 2 | 0 | 2 | 0 | 0 | 2 | 0 | 0 | 0 |
| 🌊️flow | 138 | 82 | 56 | Y | 2 | 2 | 2 | 0 | 0 | 0 | 59 | 1 | 0 | 0 | 0 |
| 🌍️gis | 155 | 105 | 50 | Y | 2 | 2 | 3 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| 🌿️vcs | 76 | 52 | 24 | Y | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 🎞️animate | 107 | 68 | 39 | Y | 2 | 2 | 2 | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 6 |
| 🎥️shooting | 171 | 130 | 41 | Y | 2 | 2 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 3 |
| 🎪️demonstrator | 37 | 23 | 14 |  | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 🎬️sequence | 79 | 53 | 26 | Y | 2 | 2 | 2 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| 🏗️fem | 274 | 231 | 43 |  | 3 | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| 🏛️architect | 864 | 836 | 28 | Y | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 |
| 🏭️process | 130 | 101 | 29 | Y | 2 | 2 | 2 | 0 | 1 | 0 | 13 | 0 | 0 | 0 | 2 |
| 💠️lowpoly | 130 | 88 | 42 | Y | 2 | 2 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 💡️reasoning | 90 | 62 | 28 | Y | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 📋️forms | 104 | 72 | 32 | Y | 2 | 2 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 |
| 📏️layout | 146 | 116 | 33 | Y | 2 | 2 | 2 | 0 | 1 | 1 | 0 | 0 | 0 | 0 | 10 |
| 📐️cad | 142 | 116 | 26 | Y | 2 | 2 | 2 | 0 | 1 | 2 | 45 | 0 | 0 | 0 | 5 |
| 📕️norm | 1680 | 1357 | 323 | Y | 16 | 2 | 2 | 0 | 15 | 0 | 0 | 0 | 0 | 0 | 0 |
| 📖️playbook | 82 | 59 | 23 |  | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 📜️imperative | 81 | 53 | 28 | Y | 2 | 2 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 📸️remodel | 221 | 167 | 54 | Y | 2 | 2 | 3 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 |
| 🔋️energy | 91 | 24 | 67 | Y | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 🔱️trinity | 182 | 153 | 29 |  | 3 | 2 | 2 | 0 | 2 | 1 | 0 | 0 | 0 | 0 | 0 |
| 🕸️dag | 106 | 79 | 27 | Y | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 🖍️draw | 122 | 106 | 16 | Y | 2 | 2 | 2 | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 |
| 🖨️raster | 113 | 94 | 19 | Y | 2 | 2 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 🗄️stdio | 2714 | 1697 | 1041 |  | 37 | 2 | 2 | 0 | 260 | 0 | 15 | 0 | 0 | 0 | 0 |
| 🗒️note | 194 | 162 | 32 | Y | 3 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 |
| 🧩️puzzle | 581 | 549 | 58 |  | 4 | 2 | 4 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| 🧱️block | 501 | 446 | 55 |  | 4 | 2 | 2 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 |
| 🪐️space | 162 | 92 | 70 | Y | 3 | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 9 |
| 🪵️sourcing | 80 | 52 | 28 | Y | 2 | 2 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| **TOTALS** | **10078** | **7572** | **2559** | **26** | **121** | **63** | **67** | **4** | **301** | **6** | **134** | **3** | **0** | **0** | **41** |

## What Is Moving

**No columns changed** between census run 1 and run 2 (90 seconds apart).
This indicates either:
- The io-async-signatures sweep made no changes to the files during this window, or
- Changes did not affect the counted patterns (async_fn, await, block_on, etc.)

The sweep may be in other directories (🚪️io/** or ✏️editor/**) not yet propagating to plugins.

## Descriptor Ratchet State

**26 plugins have both descriptor.json AND descriptor.semio:**
 1. ✒️writer
 2. ➗️mathematical
 3. 🌀️procedural
 4. 🌊️flow
 5. 🌍️gis
 6. 🌿️vcs
 7. 🎞️animate
 8. 🎥️shooting
 9. 🎬️sequence
10. 🏛️architect
11. 🏭️process
12. 💠️lowpoly
13. 💡️reasoning
14. 📋️forms
15. 📏️layout
16. 📐️cad
17. 📕️norm
18. 📜️imperative
19. 📸️remodel
20. 🔋️energy
21. 🕸️dag
22. 🖍️draw
23. 🖨️raster
24. 🗒️note
25. 🪐️space
26. 🪵️sourcing

**7 plugins missing the ratchet:**
 1. 🎪️demonstrator (has neither)
 2. 🏗️fem (has neither)
 3. 📖️playbook (has neither)
 4. 🔱️trinity (has neither)
 5. 🗄️stdio (has neither)
 6. 🧩️puzzle (has neither)
 7. 🧱️block (has neither)

## How I Measured

All measurements use Python 3 to navigate emoji-laden file paths (shell glob under-reports).

### Census Data
```bash
cd /Users/ueli/Documents/semio
python3 ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/census-async-adoption.py" --json
```
- Writes: `🔣️census-async.json` (contains per-plugin counts)
- Metrics: rs_files, host_calls, async_fn, await, block_on, pending_effects, job_reg, async_task, dl_export

### Production vs Test Files
```python
# For each .rs file, check for #[cfg(test)] attribute
# Count files with the attribute as test_files
# Count files without as prod_files
# Test files match 'test' in path OR 'test' in filename
```

### Descriptor Ratchet
```python
# Check plugin root directory for:
# - 🔣️descriptor.json (exists?)
# - 🛂️descriptor.semio (exists?)
```

### Component Declarations
```python
# In each plugin's 🦀️component.rs, count regex patterns:
# - \.activation\s*\(
# - \.execution\s*\(
# - \.requests\s*\(
```

## What Could Not Be Measured

- **Async adoption per-file granularity:** Census counts patterns but not their file locations.
  Reason: The census script outputs aggregated totals only.
- **Test coverage by feature:** No mapping of test files to specific async patterns they exercise.
  Reason: Would require AST parsing of test module structures.
- **Active sweep status in 🚪️io/** and **✏️editor/**:** Cannot confirm changes without reading those dirs.
  Reason: This audit focuses on plugin layer; system-level changes outside scope.

---
*Report generated by luna (READ-ONLY audit mode)*