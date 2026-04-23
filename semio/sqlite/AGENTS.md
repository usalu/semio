---
technology: semio
bundle:
 name: sqlite
 emoji: 💾
 description: The sqlite bundle for semio.
 kind: library
---

# 🧾 Specification

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

## 📛 Entities

Normalized kit persistence (`schema.sql`) includes:

- **`family`** — kit-scoped family metadata; nested ports are rows in **`port`** with `parent_family_id` set.
- **`type_family`** / **`design_family`** — ordered many-to-many links for `TypeFullDto.families` and `DesignFullDto.families`.
- **`port`** — `kit_id` required; `parent_family_id` optional (kit-level ports use NULL). No `type_id` column; connectors still reference `type_id` and optional `port_id`.
- **Removed columns** (vs older kits): `kit.version`, `type.variant`, `design.variant`, `design.view`.
- **`attribute.family_id`** — optional scope for attributes on a family.

`SCHEMA_VERSION` in `semio/rs` sqlite I/O must move forward when this file changes.
