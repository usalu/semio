# P4 pilot status (2026-08-07)

Pilots: `lowpoly`, `en1992`, `dag`, `cad` under `✏️s/🔌️plugins/.../🗿️artifacts/...`.

Legend: ✅ done · ⚠️ placeholder bytes (SEM + domain stub, pending Rust encoder) · — n/a

## Per-pilot facet checklist

| Facet | lowpoly | en1992 | dag | cad |
| --- | --- | --- | --- | --- |
| **dsl** grammar | ✅ typed half-edge mesh, `family-scene`, no catch-all / no `mesh-json` | ✅ typed `en1992` fields (`annex`, `fire-rating`, `tightness-class`, quantities) | ✅ graph tables + schema | ✅ pane/brep/reference typed scene |
| **op** grammar | ✅ 9 typed `LowpolyOperation` keywords | ✅ `set-document` + sheet refs | ✅ nodes/edges CRUD + `set-document` | ✅ 14 typed CAD ops (panes, brep, references) |
| **diff** grammar | ✅ mirrors op surface | ✅ sheet-oriented diff | ✅ graph patch vocabulary | ✅ object/node/reference patches |
| **pack** protocol | ✅ magic `0x894C57504C0D0A1A`, segments Objects/PaintLayers/Projection | ✅ magic `0x894E19920E0A1A0A`, clause/quantity segments | ✅ magic `0x894441470E0A1A0A`, node-graph/edge-graph | ✅ magic `0x894341443E0A1A0A`, pane + brep segments |
| **spr** protocol | ✅ record tags 1–9 per op variant | ✅ `set-document` tag 1 | ✅ tags 1–11 (graph ops + `set-document`) | ✅ tags 1–14 scene ops |
| **`COMPONENT_GRAMMAR_SEMIO`** | ✅ dsl/op/diff `include_str!` | ✅ dsl/op/diff | ✅ dsl/op/diff | ✅ dsl/op/diff |
| **`COMPONENT_PROTOCOL_SEMIO`** | ✅ pack/spr `include_str!` + `_PATH` | ✅ pack/spr | ✅ pack/spr | ✅ pack/spr |
| **`register_language`** (engine) | ✅ 5 roles via `register_pilot_languages` | ✅ 5 roles | ✅ 5 roles | ✅ 5 roles |
| **pack example > envelope** | ✅ 141 B, LWPL magic after SEM | ✅ 168 B (`norm.en1992` anchor + reuse) | ✅ 172 B DAG magic after SEM | ✅ 180 B CAD magic after SEM |
| **spr example > envelope** | ✅ 147 B, tag-1 stub | ✅ 151 B, `set-document` stub | ✅ 139 B, tag-11 stub | ✅ 147 B, tag-14 stub |

## Notes

- Binary examples for `en1992`, `dag`, and `cad` were expanded with ticket script `🔧️pad-p4-pilot-binary-examples.mjs` (domain framing bytes matching each `📡️component.protocol.semio`, not SEM-only tokens).
- `lowpoly` pack/spr were already above the 64 B empty-envelope floor; spr was refreshed to stay aligned with stub layout.
- Text examples: lowpoly DSL ~1 KB structured mesh; en1992 liquid-retaining fixture; dag/cad reuse/default DSL fixtures unchanged and non-trivial.
- Next hardening (out of scope here): replace ⚠️ stubs with `encode_pack` / `encode_op` round-trips once handcrafted codecs land.

## Verification run

- `bun seed-lowpoly-examples.mjs` — pack/spr/dsl sizes OK, no `mesh-json` in text fixtures.
- Repo scan: zero `*.pack.semio` / `*.spr.semio` ≤ 64 B under `✏️s/🔌️plugins`.
