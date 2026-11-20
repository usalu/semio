# Norm UI intake rejection

Source of the string: `OwnedUiSurface.beginPatch` (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts`) throws `Owned UI patch admission rejected` when `closing`, a patch is already open, `baseRevision !== surface.view.revision`, or `revision <= baseRevision`. It surfaces via `OwnedUiPatchIntake.advance` (phase `intake`) and `installUiPatchRound` in `PluginRuntime/handle-ui-intake` as `plugin-ui.intake-rejected:intake:...`. Related identity faults use other messages (`Foreign native instance patch owner`, `Foreign instance surface owner`, `Busy instance surface owner`).

Probe run (guest staged 18:39, bridge patch present, 600 s, `boot-probe-intake.mjs`): the rejection did NOT reproduce. A temporary `[DEBUG] ui.beginPatch rejected` log never fired (removed again). Screenshot `🗑️generated/intake1.png`: outliner (Envelope element / Thermal zone / Thermal bridge), properties and checks summary (Complies, DIN 4108-2/-3/-4/-7/-10, EN ISO 6946) render; diagrams shows its title. So the earlier rejection was a stale-guest artefact fixed by the restage.

Remaining failures (guest/other, not intake):
- Model viewport: `Render error: color buffer cardinality differs from the vertex count` (World3dHost mesh-geometry-and-appearance `bind`); the guest mesh colour buffer length mismatches the vertex count.
- After ~11 s of exchange turns (3522 turns), `command-ingress-stalled`, then guest `WorkLimit: original close demand exceeds the whole grant of its turn and can never fit` in step `displaced-surfaces` (closing_body true); subsequent inputs fail with `original-turn.source-not-ready`. Guest-side (Rust reactor grant vs close demand for displaced surfaces).

Host TS changes: none remain (debug log reverted).
