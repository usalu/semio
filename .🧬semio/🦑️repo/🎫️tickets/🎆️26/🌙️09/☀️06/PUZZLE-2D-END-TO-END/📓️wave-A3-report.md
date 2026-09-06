# ✅️ Wave A3 — leaf schema alignment (report written by main; the agent was cut by the session limit after finishing the work)

What landed (all committed in `2d2b39eb7f`, 26 leaf files in `🧬️schema/🧬️mutations/<slug>/🧬️.schema.json`):
- every leaf carries the `mutation` const discriminator matching the committed camelCase payload value;
- `Option<T>` members admit `null` (`index`, `newRadius`, `newCatalogs`, `edgeKind`, `sourceTip`, `targetTip`, …);
- `Puzzle2dNodeAnchor` values are the wire casing `fixed`/`derived`, `abstract` replaces `isAbstract`;
- the aggregate union `🧬️mutations/🔣️.json` (wave A2) and the leaves agree — the leaves were regenerated from one source (`🗑️generated/a3/gen_leaves.py` over `rust_schema.py`, a transcription of the Rust `#[value]` attributes) and the union re-derived.

Validation (`🗑️generated/a3/validate_a3.py`, jsonschema Draft 2020-12, run 2026-09-06 20:46 by main):

| what | count | failures |
|---|---|---|
| payloads vs their leaf schema | 75 | 0 |
| payloads vs the union (exactly one `oneOf` branch each; 26/26 branches hit) | 75 | 0 |
| before/after snapshots vs `📸️snapshot/🔣️.json` | 150 | 0 |
| diffs vs `🔺️diff/🔣️.json` | 53 | 0 |

Stale prose: `🐍️.py` line 19 was updated; the `🔮️oracle/🔣️.json` rationale sentence still describes the pre-A2 snapshot-shaped aggregate (historical record inside a rationale — left as the D1-dated history it is).
