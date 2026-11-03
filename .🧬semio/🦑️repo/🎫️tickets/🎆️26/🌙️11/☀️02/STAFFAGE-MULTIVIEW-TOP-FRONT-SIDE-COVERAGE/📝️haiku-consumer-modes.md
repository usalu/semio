# 📝️ haiku-consumer-audit: 7 Modes × 3 Views (relayed by the coordinator)

This is a static trace of the code; no tests were run. `D` = `🔨️modules/🎨️depiction`, `T` = `🧪️tests`, `REACT` = the React target.

## Per mode

| Mode | Elevation | Front | Plan |
|---|---|---|---|
| dark-line | Line style (`D/🟦️.ts:121-126`), ground line from the rig bounds (`:43`), section markers drawn only here (`:180`) | Ground line from the front shapes (`:44`); dimension from the front shapes (`:49-55`) | No ground line; the dimension is depth (`:55`) |
| dark-solid and light-solid | Fill by tone; the outline is gated by scale (`:127,178`). Occlusion uses the 3D rest frames (`D/🧊️depth:47,59`). Tested only at elevation (`T/🎨️seven-modes:21-29`) | Occlusion does not use the rest frames, so it is approximate under spread or twist | Same as front |
| thermal | Diverging ramp (`D/🌡️thermal-view:97-111`). The hatch is off by default (`D/🟦️.ts:100`, `REACT/🗺️routing:159`) | The mapping is view-independent. Regions hidden from this view are named, in en and de (`:143-144`) | Same as front |
| forces | Kind is shown by dash, not only by colour (`D/🏗️structural-view:236`). Supports drawn side-on when the tilt is under 45° (`:214`) | Projected arrows (`:232`). The free-body labels use inline ternaries instead of the bundle (`:493,514`) | Out-of-plane components drawn as dot and cross (`:245`) |
| moments | Curls and double-headed arrows (`:557-606`). Sampled at uniform, mid and transition instants | Same | Same |

## Defects and gaps, with owners

1. Hatching is off by default, so thermal and forces read by colour only. The README claims a colour-independent reading. Owners: viewer (defaults), contracts (option).
2. Member hatching applies only to axial force and stress. Shear, moment and contact pressure are colour-only, and temperature parcels in transport too (`D/💨️transport-view:73-77`). Owners: structural, thermal.
3. Transport numbers use `.toFixed` (`D/💨️transport-view:131`), so German gets a decimal point. Owner: thermal.
4. The note "N further values in the inspector" (`D/🟦️.ts:191-193`) varies by view, so legends differ between views. Owner: contracts.
5. Front and plan occlusion under spread or twist is approximate (`D/🧊️depth:47,59`). Owner: rigs.
6. Test gaps. Owner: validation.
   - Dark-line, dark-solid, light-solid and forces are not rendered at uniform or mid instants or at transitions (`T/🔭️multiview-coverage/🧰️.ts:74-75,250`).
   - Only English is rendered.
   - Seven-modes, view-synchrony and transport are tested only at elevation.
   - No test checks that a colour-independent cue is present.
