# PNG Picker and Text Layer Regression Repairs

## Picker callback

The registered picker test failed with `stdio.window-kit.argument-required` for `chunk`. The fixture supplies `DslValue::uint(0)` and `DslValue::uint(1)`, and the shared window-kit parser already accepts unsigned numbers. The framework's `ImportStaging::admit_args` validates the transport envelope and strips both chunk fields before returning the assembled argument object. The plugin action dispatch deliberately uses that assembled object for the typed application action.

The PDF PNG action now requires only its application fields. Its import parser retains the existing format, encoded-length, revision, and target checks. Both chunk fields remain part of the shell protocol, and the registered picker regression continues sending the original unsigned values. The direct browser TypeScript helper produced numeric JSON fields. At the maximum PDF PNG file size, its encoded callback consumed 32,765 of the 32,768 allowed bytes in one transport chunk.

## Text layer fixture

The registered PDF text-layer unit test calls the page editor renderer directly. The real builder admits collection-backed selection arguments, which require a current UI arena lifetime. The test now holds `artifact_app_laws::FixtureUiArena::admit()` from immediately before render through the selection assertions. That helper opens the actual Source-funded lifetime and closes only after the built panel leaves scope. Both localized label checks, page ordering and the actual accessible selection binding remain covered.

The two fixes were applied against exact, freshly read file anchors. The browser transport helper was exercised directly. The governed registered suite is left to the active native run.
