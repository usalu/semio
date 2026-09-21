# Window Options Tree Implementation

Current integration: Native141 and Native142 stopped at compilation, so neither provides behavioral results. Native141 encountered mutable borrowing errors in the shared plugin runtime; these have been fixed in source. Native142 found one missing argument after carrying inline flow into retained painting. Sol is correcting that call site before Native143.

The schema now carries `TreePresentation` (`standard`, `compact`) through Rust builders, typed ownership, TypeScript wire decoding, and retained nodes. Sol has integrated per-item metrics, nearest-owner scope, mirrored control/hit placement, and layout flow. Root has integrated bounded right-aligned labels and the compact Window Measures projection. The painter census includes measurement-byte progress.

The new mounted React Interpreter law consumes the same neutral expected overlay as the native projection test. It failed at the missing compact presentation attribute, with the other eight tests passing. The React Tree consumer now receives presentation and establishes local row, font, gutter, and value-column variables; a nested standard Tree resets those variables. Checkbox disabled state is forwarded to its native input. The green rerun passed all 9 tests in 11.44 seconds (Nx 14.1 seconds). This mounted DOM test does not replace physical browser geometry acceptance.

The neutral fixture now has a mounted React oracle: 8/8 tests passed. Native139 reproduced both targeted visual defects: the old root was a generic Container and the body began at the chip rather than beneath it. The replacement projects a Tree, an unlabelled section, and recursive TreeItem rows with distinct `.row` keys. Existing control keys and group header-slider bindings are preserved. Group disclosure defaults match React.

The shared binary control contract now has `ToggleAppearance` (`button`, `checkbox`), typed ownership operations, builder selection, TypeScript wire decoding, and role projection. Window Options selects checkbox; the shared React Interpreter renders TreeCheckbox, and WGPU paints a fixed-size outlined check. These changes still require compilation and accepted-pointer validation.

Native140 stopped at compilation because root's new production checkbox helper referenced a test-only imported type. The helper now names the WGPU component type explicitly. Native141 reruns Map/Canvas/Display/Find/provenance repairs and the new Settings accessibility/overflow RED laws. Native140 is not behavioral evidence.

The next implementation step is a neutral `TreePresentation::Compact` resolved per owning Tree, including nested scope boundaries. Physical value-column placement must consume the document's existing inline flow. A document-wide theme switch is inappropriate because the UI engine concurrently owns unrelated trees.

React uses varying compact control heights (`min-h-tiny` rows, `h-small` Select, and a medium checkbox wrapper). The fixture's 20px bound currently measures native control hits, not every outer row; final dimensions must be validated from the actual React DOM before treating one fixed row pitch as parity.

Actual fresh React browser measurement at 1600×1000 now confirms: group-only row 14.390625px, Select row 16px, Checkbox and Slider rows 22.390625px; font size 9.6px. The RTL value column is exactly 104px and physically left of labels. Its checkbox square is 9.59375px at the column's right edge, not centered. Evidence: `🗑️generated/astra-runtime/checkpoint20-iab/react-window-options-metrics.json`. Reload restored the two default windows; the prior empty dock was not a persistent reload state. The compact layout must derive per-control row heights from existing typography and control tokens. A fixed 20px outer row or new arbitrary token would not match React.

UI22 passed the production clipping-piece sequence oracle, but overlap refusal failed because counter overflow bypassed grant faulting. Sol repaired that path; another UI run is required. UI21's full 667-test run hit its 15-second fundamental budget without a summary.

The compact generic Tree now scopes Input and Select height with an inherited variable reset by nested standard Trees. Terra identified that the existing virtual Tree window protocol assumes a uniform standard row pitch; the correct contract treatment for variable compact row heights remains under audit. No virtual compact parity claim is made.
