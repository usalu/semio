# STATUS — Puzzle Design Parity

## Wave 0 — DONE
Ticket + normative spec + flatten excerpt.

## Wave 1 — DONE
2d/3d/5d schema surgery (8 connection params, anchor, type-like kinds). Compose bridge deleted.

## Wave 2 — DONE
3d flatten port + 5d wrapper + 2d fastened layout. Diagram centers match Flat golden.

## Wave 3 — MOSTLY DONE
- Fastener commands module implemented and registered in glue + app dispatch/mutations.
- Inspection panel exposes anchor + 8 fastener params + part x/y.
- App twin structs gained `Puzzle5dPartAnchor` + fastener `x`/`y`.
- `patch_part` supports `anchor`.
- Remaining: locale/settings/terminology polish, pose-from-flatten wiring in edit windows if not already.

## Wave 4 — MOSTLY DONE
- Capsule Dream DSL generated (2880 parts / 2864 fasteners), empty kind-catalogs (DSL rejects nested catalog LIST payloads).
- Golden poses remapped Flat→Dream by unique piece name.
- Example unit wired in glue.rs + TS index + app example picker.
- DSL parse + round-trip green; diagram-center golden green.
- Open: 3d origin golden vs Flat still diverges under compose-identical matrix packing (~2793). Follow-up needed.

## Wave 5 — PARTIAL
Permanent center golden in example tests. Compose cross-check vs flatten.cases still pending.

## Wave 6–7 — PENDING
Sketchpad consumer, storybook, launch.json, verify gate, ticket_close.
