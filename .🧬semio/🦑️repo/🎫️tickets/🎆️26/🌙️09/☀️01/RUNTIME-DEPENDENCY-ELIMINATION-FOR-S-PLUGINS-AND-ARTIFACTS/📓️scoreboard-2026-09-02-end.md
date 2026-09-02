# 📊️ Plugin production-serde scoreboard — measured end of 2026-09-02

Method (the only one that gives true numbers here): strip `//` comments → strip `#[cfg(test)] mod …`
blocks by brace matching → match `use serde|serde::|serde_json|#[serde(|derive(… Serialize|Deserialize …)`
→ exclude `_serde::`, `Error::(Serialize|Deserialize)`, `VcsError::`, `cfg_attr(test`.

**TOTAL 10,712 → 3,937 (‑63%)**

| plugin | before | now |  | plugin | before | now |
|---|---|---|---|---|---|---|
| 🏛️architect | 1636 | **1** |  | 🕸️dag | 148 | 82 |
| 📕️norm | 1595 | **2** |  | 🌿️vcs | 132 | 82 |
| 🖍️draw | 240 | **2** |  | 📜️imperative | 131 | 84 |
| 📐️cad | 422 | **14** |  | 📖️playbook | 113 | 89 |
| 🪵️sourcing | 105 | **22** |  | 💡️reasoning | 133 | 99 |
| 🧱️block | 695 | **29** |  | 💠️lowpoly | 269 | 105 |
| 🎪️demonstrator | 51 | 37 |  | 📏️layout | 280 | 105 |
| 🎞️animate | 142 | 37 |  | ✒️writer | 162 | 110 |
| 🎥️shooting | 199 | 38 |  | 🖨️raster | 169 | 118 |
| 📋️forms | 95 | 59 |  | 🌀️procedural | 458 | 119 |
| 🌊️flow | 153 | 78 |  | 🎬️sequence | 157 | 121 |
| | | |  | 🌍️gis | 283 | 152 |
| | | |  | 🗒️note | 246 | 246 |
| | | |  | 📸️remodel | 293 | 292 |
| | | |  | 🔋️energy | 364 | 350 |
| | | |  | 🗄️stdio | 582 | 369 |
| | | |  | 🧩️puzzle | 1716 | 1095 |

## ⚠️ Compiling clean ≠ serde-free
Verified at **0 errors, 0 serde-family**: `semio-framework-os-flow`, `semio-s-plugin-sourcing`,
`semio-s-plugin-stdio`. Yet sourcing still had 42 refs and BOTH manifest lines at that moment, and
stdio still has 369. Zero errors proves the conversion broke nothing — it does NOT prove serde left.
The goal metric is production refs + `[dependencies]` entries, never the error count.

## ✅️ Framework seam layer — verified clean (this is what forced serde onto the plugins)
    semio-framework-os-kernel   0      semio-framework-plugin  0
    semio-framework-os-flow     0      semio-framework-mesh-engine 0 (35/35 tests, serde → dev-deps)
    semio-framework-value-derive 23/23 tests, incl. byte-identical `flatten` vs serde_json

## ▶️ Next, in priority order
1. FINISH the near-zero plugins and REMOVE their manifest lines: 🏛️architect(1), 📕️norm(2), 🖍️draw(2),
   📐️cad(14), 🪵️sourcing(22), 🧱️block(29). Each becomes a proof-of-recipe.
2. Untouched/barely-moved: 🗒️note(246), 📸️remodel(292), 🔋️energy(350) — their agents converted derives
   but re-added serde for io bridges; convert the bridges first.
3. 🧩️puzzle(1095) is the largest single remaining body of work.
4. 🗄️stdio(369) — but its 🧪️oracle/ and 🏭️generator/ third-party crates are the DELIBERATE evidence
   base and must stay.
