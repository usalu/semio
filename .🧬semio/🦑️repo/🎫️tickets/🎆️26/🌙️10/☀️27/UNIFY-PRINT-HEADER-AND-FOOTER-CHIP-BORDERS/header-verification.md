# Three Assignment Header Chips

The German assignment now declares exactly these header labels:

| Position | Content |
|---|---|
| Left | Semio emblem + Architektur und Technologie |
| Middle | Fingerübung |
| Right | Parametrisierung |

The English assignment uses Architecture and Technology, Finger Exercise, and Parametrization. Course metadata matches the left-chip title.

Added the configurable SemioHeaderSetup command with left, center and right keys. Explicit labels stay independent of section marks and retain their physical left/center/right order. The left chip keeps the emblem inside its existing shared border. The existing footer is unchanged.

## Verification

- Added language-neutral English/German three-zone cases.
- Confirmed the new check fails before implementation.
- Focused source, layout, geometry and logo checks pass through the retained verify.ts.
- Independently validated each parsed header declaration with AJV against its exact expected labels.
- Existing combined-emblem and shared-border regressions still pass.
- Whitespace diff checks pass.
- Full quick pipeline still cannot start because @semio-tech/framework-transient is unresolved in the UI theme module. Rendered-PDF verification remains unavailable.

The new setup command is documented in the print README in English and German. Concurrent assignment content edits were preserved. No generated ticket outputs remain.
