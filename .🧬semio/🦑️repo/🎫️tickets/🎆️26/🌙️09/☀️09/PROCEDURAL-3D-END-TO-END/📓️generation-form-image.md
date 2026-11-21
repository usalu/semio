# Semantic generation-form images

The shared Generation2d and Generation3d form renderer produces a framework Image for an authored image source. Image accessibility uses the authored description when present and falls back to the field label. Missing and whitespace-only sources show the calling app's localized placeholder.

The neutral fixture covers English with an authored description and German with its field label, plus both missing-source forms. The actual image branch built against semio-ui-contract passed both locale cases and verified the Image payload, alt label and accessibility label. The independent PNG decoder validated the embedded raster in both cases. The registered Generation3d composition test remains pending.
