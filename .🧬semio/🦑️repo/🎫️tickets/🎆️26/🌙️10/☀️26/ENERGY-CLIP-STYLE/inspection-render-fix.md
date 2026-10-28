# Inspection Render Fix

Canonical source 13 (Beat2_Luftwechselrate) failed in the first house-to-air-change-marker morph. Its immutable attempt log is under generated/i/de/spectator/s/13/attempt-1793093899074368100.log. Manim reported incompatible point-array shapes (320, 3) and (176, 3).

The independent test reproduced the same exception using the actual house geometry and the third-party Manim ReplacementTransform lifecycle. The house retained one live descendant geometry updater. During transform setup, Manim aligns source and destination topology; the copied live updater subsequently rebuilt its geometry, undoing that alignment before interpolation. Freezing the recursive updater tree before morphing preserved the aligned topology and passed the identical third-party transform. The extracted production show_use function also reproduced the failure before the fix.

The authored fix adds glyph.clear_updaters() immediately before the existing readout updater clearing and the two ReplacementTransform calls. It freezes the completed room glyph once its live illustration is finished, while preserving the preceding tracker animations and all narration timing. Shared rendering runtime and helper code are unchanged.

The affected owner contains three canonical scenes: Beat1_Innenvolumen, Beat2_Luftwechselrate, and Beat3_Volumenstrom (ordinals 12–14). A repository source search found no other authored scene importing this owner; its speech wrapper and an existing visual consumer test are the only direct references. Production provenance must refresh these three sources and rebuild their chapter after the source edit. Existing unrelated work remains reusable.

Independent regression evidence is retained in inspection-render-fix-test.md and inspection-render-fix-test.py. The coordinated recovery uses the normal production ventilation-only compositor with three jobs after the original global source batch, then checks all 117 canonical sources and 38 chapters before staged publication. No success certificate is copied, edited, or forged.

The independent green check passed: the original reproducer still raises the exact shape error, its frozen oracle passes, and the extracted patched production boundary passes. Both permanent regression tests passed (51.343 seconds). See inspection-render-fix-test.md for the detailed evidence.
