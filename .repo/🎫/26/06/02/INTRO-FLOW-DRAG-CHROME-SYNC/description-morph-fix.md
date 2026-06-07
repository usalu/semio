# Description morph + auto-animate

## Symptom
Long description doubled on goal slide and did not morph into short label.

## Causes
1. Flow selection chrome used section-space `measuredNatural` as % inside the disposition wrapper (wrong ink frame → layout churn during reveal measurement).
2. Ink measurement (`--measuring` class) and selection chrome ran during reveal `pending`/`running`, disturbing FLIP.
3. `relaxHiddenPreflight()` ran only on mount/slidechanged, not `beforeslidechange` (Tailwind `[hidden]` could collapse auto-animate measurement).

## Fix
- Chrome ink frame: container-relative `inkInWrapper` only (no `measuredNatural` fallback).
- Skip ink measure, registry measure, and selection chrome while `isRevealSlideAutoAnimating`.
- Call `relaxHiddenPreflight()` in `onBeforeSlideChange` before `prepareArrangementBeforeAutoAnimate`.
