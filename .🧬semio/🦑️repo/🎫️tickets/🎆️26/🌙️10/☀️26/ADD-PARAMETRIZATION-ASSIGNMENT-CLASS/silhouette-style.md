# Native Diagram and Transition Silhouettes

The shared native window engine now registers `Diagram` on its visual tier and `Transitions` on its logical tier. Both use the existing native frame, top baseline, localized automatic right chip, register sequences, and shared hierarchy slot. No new frame/chip drawing implementation was introduced.

`semio-core.sty` registers German Diagramm/Übergänge and English Diagram/Transitions labels, including plural labels for native registers. `semio-window.sty` registers the two kinds and distinguishes an omitted title from an explicitly empty title. Omitted titles retain the existing localized left-title default; `title={}` suppresses the left chip; a supplied title retains its content.

## Content API

Wrap graph calls with `Diagram[title={},break=false]` and transition ledger calls with `Transitions[title={},break=false]`. The existing shared per-section slot sequence yields worked table 1.1.a, diagram 1.1.b, transitions 1.1.c; task tables 1.2.a/1.2.b, diagram 1.2.c, transitions 1.2.d. Number strings are computed by the shared hierarchy engine rather than embedded in content.

## Geometry

The native body inset is 5.5 pt on every side, plus its native frame hairline. AssignmentLedger already derives each column from its current linewidth and each writing rule from 76% of that column. Framing it therefore naturally adapts the former 64.6 mm rules to approximately 63 mm while retaining the 9 pt font and 6.3 mm row pitch. The longest German named change is below 60 mm. No assignment geometry or body font change was needed.

## Test First

`silhouette-style.case.json` provides language-neutral kind, hierarchy, and omitted/empty/named-title cases. The source contract runner `silhouette-style.verify.cjs` first failed because Diagram was unregistered, then passed registration, singular/plural locales, and explicit-title state after the shared implementation. Independent native PDF path/text tests belong to the audit agent; root owns the native compiler. All source mutations were atomic with concurrent-content checks. Only semio-core.sty and semio-window.sty changed in this styling task; sources are frozen for native verification.
