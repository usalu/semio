---
slug: CLEAN-CODEBASE
summary: Cleanup diagnostic artifacts
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.903Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

- Sketchpad runtime components still emitted persistent `console.log` output for settings panels and model selection paths.
- Settings visibility logging in `Sketchpad.tsx` remained from earlier diagnostics.

# Plan

- Find non-test console logging that stayed in runtime code and determine what is safe to drop.
- Strip noisy logs while keeping warnings/errors that surface real data issues.
- Reflect the logging policy updates in the dev docs.

# Changes

- Removed runtime `console.log` calls from `Home` settings sections, `PieceMesh` and `TypeMesh` model selection paths, and the Sketchpad settings visibility effect.
- Left warnings/errors intact for missing models/files and blob URL failures to keep actionable signals without console noise.
- Documented the console cleanliness policy in `README.md` and `AGENTS.md`.
