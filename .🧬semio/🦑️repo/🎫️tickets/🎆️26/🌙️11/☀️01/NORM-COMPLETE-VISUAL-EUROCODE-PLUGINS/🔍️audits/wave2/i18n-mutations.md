# 🌍️ Wave 2 i18n mutations

- Source: DE labels came from the leaf generators (`🧰️scripts/leaf-kinds/leaf.py` `label_de`, `index_selector.py`): `"<entity de>: <field de> ändern"` where field de silently falls back to the raw field name (`Leaf.label` in `model.py`) when the schema had no DE label at generation time. i18n-labels later added DE to the schema JSON, but the Rust `LocalizedLabel::native` strings were never regenerated (the schema twins carry no labels).
- New tool: `🧰️scripts/i18n-mutations/relabel.py [--write]` recomposes the DE label from the leaf's own `🧬️schema/🔣️.json` (`x-semio-ui.label.de` of the `new*` field; entity from the existing DE prefix, else the `role: target` property) for every pair whose DE leaks English. Applied: 65 labels in en1990, en1991, en1992, en1993, en1997, en1999 (e.g. `Stützwand: height ändern` -> `Stützwand: Höhe ändern`; `Anschluss: kind ändern` -> `Anschluss: Art ändern`; `Bauteil: member id` -> `Bauteilkennung`). Verb stays localised (`ändern`). EN left unchanged (curated "Change X of the Y").
- Schema JSON and twins untouched (no label lives in them), so no regeneration or drift run was needed; heavy.sh not used.
- Test: `⚖️compliance/🧪️tests/🌍️i18n-mutations/{🐍️.py,🥒️.feature}` scans 3043 mutation leaf sources; fails on a lowercase ASCII DE word equal to a kind/field-name word or in the English list (capitalised cognates pass). Result: ok; negative probe flags `height`.
- Accepted: en1999 `change-vertex-ring` "Ring (outer oder Lochname)" (`outer` is a literal data value, also in field-meta).
- Rebuild requests appended to build-en1990/1991/1992/1993/1997/1999 `latest.md`.
- Not changed: generator fallback in `model.py` (ticket scripts, historic); the test guards regressions.
