# VIS EN 1991 (WP-14) - Phase 1 and Phase 2 written, build pending (norm agent builds)

Scene crate `semio-s-artifact-norm-en1991-scene` at `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene`, registered in the artifact `Cargo.toml` (member, workspace deps incl. structural-scene) and `📦️packages/🦀️rust/📜️script.ts`. No build ran: my queued check was cancelled under the "one build per artifact" rule; "ready for rebuild" is in `build-en1991/latest.md`. Golden fixture is empty: run tests once with `SEMIO_UPDATE_GOLDEN=1`.

## Delivered
- kit (Spec/Out, owner resolution, bindings by subject or check id, bilingual number formatting, hidden alternative layers), layout (Pose + Placement for building, bridge, crane, silo, self-weight stack, extra roofs).
- envelope: translucent building prism, flat/mono/duo/hip roof bodies, bound dimensions.
- wind: c_pe zones A-J as Pressure glyphs on walls and roof (value q_p c_pe,10), net arrows for c_pi +0.2 and -0.3, q_p(z) profile.
- snow: mu1 per slope, duo alternative arrangements (hidden layer), parapet wall, obstruction, drift area plus linear profile, detached roof planes for further roofs.
- gravity: floors by category colour (Okabe-Ito) with reduced q_k load; self-weight stack coloured by unit weight.
- thermal/fire/accidental: uniform and gradient temperature glyphs, fire compartment with gas temperature, impact and explosion glyphs.
- traffic/crane/silo: deck, lanes, LM1 tandems and UDL, footways, LM2/LM3/LM4 and footbridge crowd (hidden layer), crane runway wheel loads, silo/tank pressure, friction, patch.
- diagrams (table twins, draggable markers): qp-profile, cpe-area, snow-mu, snow-mu3, fire-curves, bridge-influence, silo-pressure.
- handles: building height/width/depth/storeys/altitude/wind zone/terrain, roof pitch/parapet/obstruction, floor area, layer thickness, face z/loaded area, bridge span/lane width/lane, hoisting speed, silo height/radius, fire compartment height.
- tests `🧪️tests/🔬️unit`, `🧪️tests/🥒️.feature`, oracle `🔮️oracles/🐍️.py` (closed forms, scipy ODE for Janssen).

## Limits / requests
- Not in the model, so not drawn: draggable tandem positions, crane position, rail LM71/SW, wind-face zone letter handle, drift obstacle plan position, nested accidental-case handles, hipped-roof zones (Table 7.5 absent from checks). Crane runway geometry uses schematic constants.
- Missing EDIT_RULES (schema owner): roofs[].pitchDeg/parapetHeight/driftObstructionHeight, floors[].area, selfWeightElements[].thickness, windFaces[].z/loadedArea; and a `resolve` as in EN 1993. Top-level handle paths already have rules.
- Phase 2 applied: inputs/results/report windows deleted; viewport, diagrams, properties, checks copied from EN 1993 for editor and viewer, mode layouts, editor root arms and registrations, Cargo deps, TS twins trimmed. Requests for edit rules and missing fields are in requests.md.
