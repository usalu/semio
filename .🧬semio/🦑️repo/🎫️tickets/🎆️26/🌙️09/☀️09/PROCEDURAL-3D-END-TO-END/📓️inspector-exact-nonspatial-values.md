# Exact Text and Integer Values in the Procedural Inspector

## Reachable gaps

Flow’s live values include text dictionaries and native integer atoms. The Values tree already exposes long text through the shared retained Text Editor. The Generation3d Inspector used a separate reflection path. It showed evaluated scalar text through 256 UTF-8 bytes, then took the unavailable path without an exact expansion. The same limit affected the selected Variable’s evaluated output. Inspector numeric reflection routed integer atoms through a binary floating-point conversion, which changes the displayed value above JavaScript’s safe integer range.

The Inspector also stopped offering editors for note text, disconnected operator text inputs and text collection members over the 512-byte inline record limit. The command accepts text through 1 MiB. The existing Boolean editor uses the framework checkbox. Point and vector fields use the framework axis controls. The registered export format selector uses the artifact codec roster.

## Changes

Inspector outputs use the same exact windowed Text Editor pages as the evaluated Values tree. A long value can be opened without re-evaluating the graph. Numbered pages inherit the original output handle. The selected Variable uses the same reflection path. Integer and Decimal values retain their canonical Pack JSON spelling.

The Inspector reuses Generate’s semantic retained Input for text drafts. The input keeps its node ID, accessible name and widget, channel and optional item index across inline and body-backed values. The window ledger charges carrier records. When the editor body cannot fit, the exact text can still be read through the windowed Text Editor. Connected inputs retain their graph source-navigation affordance. The command still validates ownership and membership before issuing its existing absolute mutation. It reconstructs valid browser text pages under the existing bound, while native scalar text arrives as the same command value.

## Checks

- The [neutral fixture](inspector-text-contract.json) covers a multilingual note, a large output containing a leading BOM, CRLF and NUL, and the exact integer spelling beyond binary floating-point precision.
- The [compact Rust driver](inspector-text-native.ts) used the production reflection, page selector, retained Input presenter, carrier ledger charge and command decoder. Its control and tree stand-ins checked the 256-byte boundary, whole output reconstruction, inherited hover addresses, item index and operation, and exact read-only fallback when carrier records are unavailable. It accepted exactly 1 MiB, refused a larger value and rejected malformed pages. The framework browser encoder and iconv-lite independently produced the same UTF-8 bytes.
- [Mounted Chromium](inspector-text-browser.ts) checked a long saved note using the actual React tree, retained Input and action bridge with a fixture publisher. It verified both locale labels, exact widget and channel arguments, composition-owned Enter, commit, Escape, reopening, and editing from a short value past 512 bytes.
- The existing Generate retained-input Chromium regression passed following the shared presenter reuse. The whitespace check passed.
- The full native WGPU host and live actor application of the Inspector edit remain outside the compact checks.
