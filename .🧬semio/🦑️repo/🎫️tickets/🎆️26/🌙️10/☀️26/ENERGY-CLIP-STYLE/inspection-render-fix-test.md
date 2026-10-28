# Inspection Render Fix Test

Status: passed.

The real source13 house geometry reproduces a Manim interpolation failure without full scene rendering. The driver invokes real ReplacementTransform.begin, update_mobjects and interpolate at five alpha values; NumPy performs the third-party interpolation. Freezing all descendant updaters before creating the transform passes the same boundary. The regression then extracts and executes the actual source nested show_use function with real Manim objects, independently checking the authored boundary rather than mirroring a proposed fix.

- Original dynamic geometry failure: operands could not be broadcast together with shapes (320,3) (176,3) 
- Descendant updater count before freeze: 1
- Frozen geometry oracle: passed
- Production show_use result: passed

This test changes no production source. Generated fixtures/results belong to generated/iv/fix. Full source13 rendering and the final full-catalog verifier remain required after the narrow fix.

## Permanent Regression And Test Workflow

The same focused regression is retained under ventilation/🎬️clip/📦️volume/🧪️tests as a language-neutral 🔣️.cases.json geometry/transition fixture and a 🐍️.py unittest implementation. Both permanent tests passed in 51.343s: the original topology failure plus frozen Manim/NumPy oracle, and the actual AST-extracted production show_use boundary after the one-line updater freeze.

The existing authoring/📜️script.ts `test` command now registers the new dotted test module next to its existing ventilation architecture tests. This is the documented teaching workflow entry, so the focused regression participates in future normal teaching runs. No broad suite was run for this fix. Registration is outside source dependencies: segment_state follows Python literal imports and enumerates asset suffixes that exclude TypeScript, while the permanent fixture lives below the explicitly excluded 🧪️tests directory.

The fresh full production render subsequently completed Beat2_Luftwechselrate successfully and published its current source certificate. All three volume sources (ordinals 12–14) were current at the 13/117 source milestone. This exercises the entire scene beyond the independent focused transform oracle, including the later use-case morphs. Final chapter/body/envelope verification remains part of the complete catalog acceptance.
