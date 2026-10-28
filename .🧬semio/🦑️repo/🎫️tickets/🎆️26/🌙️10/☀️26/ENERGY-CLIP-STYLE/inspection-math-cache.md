# Inspection Math Readout Cache

Status: GREEN. The implementation was written after the original owned render tree and capacity grant were stopped. Five focused tests failed first for missing text-context/cache ports, then all five passed in 22.742 seconds after implementation and correction of test-harness assumptions. No broad unrelated suite was run.

Only math_readout uses a 32-entry LRU of unplaced formula templates. Every request obtains a fresh Manim deep copy and then uses existing place_math; normal math_text and math_label remain the independent uncached oracle. RGBA normalization retains alpha. Font-owned text_context returns primitive values for the cached process asset/font/theme digest, active face, size/line/fallback profile, kerning settings, minimum text canvas, actual canvas/frame/renderer and stable Text init/hash descriptor identities. Calling the cached font digest does not rehash files per frame. Source edits still require matching normal production provenance and fresh source renders.

The language-neutral fixture and focused Manim/NumPy/Cairo suite live below visuals/🧪️tests/📟️readout and are registered in the existing authoring test entry. Coverage includes geometry/styles/alpha/baseline/anchors/exact pixels, mutation isolation, callable placement and live object identity, source/size/color keys, bounded LRU eviction and font/theme/kerning/canvas/frame/renderer/native-default namespace changes.

Before-change visuals SHA-256: 6ec450b3ea3f972bf1c6128767b546eb5649b81900616869d4d3e6090dffba9c

Current production source identities:

```json
{
  "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py": "90899ef0b7e7b9110f9cd29b0b53ff22307f79a8fd070791d0855a924c702387",
  "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py": "23e2e4c79406b2613dc1333b2c778289fe6b9811655acecfa9059cf7bfc362e0"
}
```

The bounded performance experiment and raw samples are retained in inspection-math-bench.md. It measured 16.5–29.7x warmed label and 23.7–30.6x unchanged readout update wall-time gains under the prior render contention; current-path references and cached prototype were pixel-identical.

## Focused Validation

- Text-context/font/theme/kerning/minimum canvas/actual canvas/frame/renderer/native Text defaults change template keys.
- Unchanged formulas reuse templates; callable positions and changed values still update; the live object retains its identity and matches the original always_redraw update history.
- Source, resolved size, normalized RGB/RGBA and alpha vary keys correctly; equivalent white representations share a template.
- The 32-entry LRU evicts old entries deterministically and fresh copies isolate point arrays, baseline markers, styles, opacity, stroke width, custom attributes and updaters.
- Ordinary uncached math_label/math_text remain independent references. Real indices and fractions match every point/style/baseline/anchor and every Cairo output pixel, including alpha colors.

Reviewed with the scout; no blocking code issue was found. The asset digest is captured once per Python process and existing math metrics are process-local, so this cache does not introduce a hot-font-file swapping contract. Normal source hashes and fresh subprocess rendering continue to bound the render lifetime.

## Changed Paths

- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/📟️readout/🔣️.cases.json
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/📟️readout/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts (existing teaching test registration)
