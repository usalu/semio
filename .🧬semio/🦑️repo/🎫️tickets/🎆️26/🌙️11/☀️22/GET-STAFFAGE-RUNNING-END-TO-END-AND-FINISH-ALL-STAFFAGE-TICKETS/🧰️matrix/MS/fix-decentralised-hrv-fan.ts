/** 🌬️ The multiview repair of `decentralised-hrv-fan` at 1:50 (idempotent, the last step of its chain): seen from outside, the square tray on the round sleeve is the same T from ahead and from above, so the plan was a quarter turn of the front and drew less than 1:100. The finer band now shows what 1:100 already shows and what 1:50 can print: the tray as floor and rim, and inside the sleeve the ceramic core, the three struts, the motor can with its cap and the five blades. Every inner part is the side-elevation shape of the same id swept along the duct axis over the depth its body solid has (`extruded`, `sweep: "w"`), so the three struts (120 degrees apart) and the five blades (72 degrees apart) read differently from ahead and from above and follow the rotor in every pose. Depths are read from the 1:50 body.
 *  bun fix-decentralised-hrv-fan.ts [--in file] [--out file] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { load, round, save } from "./lib.ts";

const loaded = await load("decentralised-hrv-fan");
const asset = loaded.asset;
const rep = asset.representations["50"];
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } } as never);
const worlds = worldSolids(asset, state.bones as never, 50) as any[];
const depthOf = (id: string): [number, number] => {
  const hits = worlds.filter((world) => world.id === id || world.id.startsWith(`${id}#`) || world.id.startsWith(`${id}:`));
  if (hits.length === 0) throw new Error(`no body solid "${id}" at 1:50 (${[...new Set(worlds.map((world) => world.id))].join(", ")})`);
  let [low, high] = [Infinity, -Infinity];
  for (const world of hits) for (let i = 2; i < world.positions.length; i += 3) { low = Math.min(low, world.positions[i]); high = Math.max(high, world.positions[i]); }
  return [round(low, 4), round(high, 4)];
};
const INNER: readonly { id: string; solid: string; z: number }[] = [
  { id: "core-disc", solid: "core", z: 3.2 },
  { id: "strut-1", solid: "strut", z: 3.27 },
  { id: "strut-2", solid: "strut", z: 3.27 },
  { id: "strut-3", solid: "strut", z: 3.27 },
  ...[1, 2, 3, 4, 5].map((n) => ({ id: `blade-${n}`, solid: `blade-${n}`, z: 3.3 })),
  { id: "hub", solid: "hub", z: 3.4 },
  { id: "hub-cap", solid: "hub-cap", z: 3.5 },
];
const twin = (id: string): any => {
  const found = rep.geometry.elevation.find((shape: any) => shape.id === id);
  if (found === undefined) throw new Error(`no elevation shape "${id}" at 1:50`);
  return found;
};
const inner = INNER.map(({ id, solid, z }) => {
  const [z0, z1] = depthOf(solid);
  return { ...twin(id), z, solid: { kind: "extruded", sweep: "w", z0, z1 } };
});
const TRAY = { floor: [-0.1075, -0.1], rim: [-0.12, -0.1075] } as const;
const flat = (view: "front" | "plan", id: string): any => {
  const found = rep.geometry[view].find((shape: any) => shape.id === id);
  if (found === undefined) throw new Error(`no ${view} shape "${id}" at 1:50`);
  return found;
};
const span = (view: "front" | "plan", shape: any, [from, to]: readonly [number, number]): any => {
  const { r: _r, ...rest } = shape;
  return view === "front" ? { ...rest, x: from, w: round(to - from, 4) } : { ...rest, y: from, h: round(to - from, 4) };
};
for (const view of ["front", "plan"] as const) {
  const plate = flat(view, "plate");
  rep.geometry[view] = [span(view, plate, TRAY.floor), { ...span(view, plate, TRAY.rim), id: "tray-rim" }, flat(view, "ring"), flat(view, "rim"), ...inner];
}
const d = rep.decisions;
d.silhouette = {
  en: "A 0.26 m square base of the inner cover, 5.2 mm on paper, seen from the room with cover and filter taken off: the collar of the fan housing as a ring, the black funnel of the EPP jacket round the ceramic honeycomb, and in front of it five radial fan blades on the motor can, held by three struts. Front and plan, 4.8 mm by 5.2 mm on paper, show the unit side-on as a T: the tray as its 7.5 mm floor and 12.5 mm rim on the room face of the wall and the round sleeve that crosses 0.22 m of wall, with the 208 mm collar for the outer hood at its end. Inside the sleeve they show what the side elevation shows, each part over its own depth along the duct: the ceramic core of 100 mm, the three struts, the motor can with its cap and the five blades. One strut stands upright and one blade hangs down, so the front elevation and the plan are two different drawings.",
  de: "Ein quadratischer Sockel der Innenblende von 0,26 m, auf Papier 5,2 mm, vom Raum aus gesehen bei abgenommener Blende und ohne Filter: der Kragen des Ventilatorgehäuses als Ring, der schwarze Trichter des EPP-Mantels um die Keramikwabe und davor fünf radiale Flügel am Motorbecher, den drei Streben halten. Vorderansicht und Grundriss, 4,8 mm auf 5,2 mm, zeigen das Gerät von der Seite als T: die Wanne mit 7,5 mm Boden und 12,5 mm Rand auf der Raumseite der Wand und die runde Hülse, die 0,22 m Wand durchquert, mit dem Kragen von 208 mm für die Außenhaube am Ende. In der Hülse zeigen sie, was die Seitenansicht zeigt, jedes Teil über seine eigene Tiefe entlang des Kanals: den Keramikkern von 100 mm, die drei Streben, den Motorbecher mit Kappe und die fünf Flügel. Eine Strebe steht senkrecht und ein Flügel hängt nach unten, daher sind Vorderansicht und Grundriss zwei verschiedene Zeichnungen.",
};
d.omitted = {
  en: d.omitted.en.replace(" In front and plan everything inside tray and sleeve.", " In front and plan the screw bosses and the terminal box inside the tray, the jacket, the stator, the shaft and the bearing."),
  de: d.omitted.de.replace(" In Vorderansicht und Grundriss alles in Wanne und Hülse.", " In Vorderansicht und Grundriss die Schraubdome und der Klemmkasten in der Wanne, der Mantel, der Stator, die Welle und das Lager."),
};
console.log(`decentralised-hrv-fan: 1:50 front ${rep.geometry.front.length} and plan ${rep.geometry.plan.length} shapes, file ${(await save(loaded)) ? "written" : "unchanged"}`);
console.log(`omitted: ${d.omitted.en}`);
