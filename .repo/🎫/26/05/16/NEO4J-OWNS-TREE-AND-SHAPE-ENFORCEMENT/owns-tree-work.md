# OWNS tree + shape enforcement

- Canonical `Module(operation)` parent is the **domain `Module` row** (same `name` as the kit `Class`/`Interface`), not the kit host.
- `Module`→`Command` uses **`HAS`**; `Module`→`Module` stays **`OWNS`**.
- `package.json` `migrate:neo4j` now points at the live ticket folder `NEO4J-MODULE-SHELL-REL-TYPE-HAS`.
- Post-migrate `AssertOwnsContainmentShape` requires APOC `apoc.util.validate`.
