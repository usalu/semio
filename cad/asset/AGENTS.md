---
technology: cad
bundle:
  name: asset
  emoji: 📦
  description: CAD builtin data — model definitions and play sample models.
  kind: asset
---

# 🧾 Specification

- `modelDefinition/` — declarative spatial schemas consumed by `@cad/js/core`.
- `play/` — sample model JSON for CAD play and integration tests that exercise real play payloads.

Test-only synthetic JSON MUST NOT live here; use package-local test folders or `@semio/fixture` for semio kit data.
