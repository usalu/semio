# Equilibrium Label Overlap (Heizen / Wärmeleitung Beat1)

## Symptom

Above the masonry, `θ₁` / `θ₂` readouts and `Gleichgewicht: θ = 20°C` sit on
screen together for ~1.5 s as a morph — glyphs look stacked and unreadable.

## Cause

`heating.conduction.equilibrium.readouts-to-label` uses recipe `replace`
(`Move` / Transform) from `VGroup(left_read, right_read)` into `eq_label`.
Different formula strings reshaping into one label produce the mid-morph mess.
A simultaneous cross-fade (`fade` / Dissolve) would still show two strings at
once — the same class of bug `swap_caption` already documents.

## Fix

Sequential handoff (fade readouts out, then fade the equilibrium label in),
and retarget the declarative transition to `appear` for the label only so the
morph registry still records the beat without a reshape morph.
