/** 🩺️ Slice B3a2 — `gis3d` (🏔️gisterrain, `s.gis.gisterrain@1/*#editor`, react :6083) interaction
 * probe: the SECOND gis artifact, so "a real mutation observed live" is not a gismap-only claim.
 *
 * Vertical exaggeration is the terrain's one editable document property
 * (`🎮️commands/🏔️exaggeration/🦀️.rs`, `ChangeExaggeration`, `Emit::amend` under one coalesce key so a
 * whole slider drag folds into ONE undoable edit). Its `ActionArgDef::slider("value", 0.5..5.0)` is
 * already in the staged manifest, so the rail can stage a destination and the verb is not the
 * dead "no `action_args`" shape §9.1 found on gismap. `4.0` is staged rather than the default `2.5`
 * so the dispatch has something to diff against whatever the demo example carries.
 */
import { runInteractionProbe } from "./🐍️b3a-interaction-probe.mjs";

await runInteractionProbe({
  plugin: "gis3d",
  variant: "gis3d",
  port: Number(process.env.SEMIO_PROBE_PORT ?? 6083),
  action: "setExaggeration",
  args: { value: Number(process.env.SEMIO_PROBE_EXAGGERATION ?? 4.0) },
});
