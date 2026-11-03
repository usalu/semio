import { readFileSync, writeFileSync } from "node:fs";
const S = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const plan = [["gas-hob", "🔌️appliances"], ["cooking-pot", "🪑️objects"], ["electric-kettle", "🔌️appliances"], ["scroll-compressor", "🏗️machinery"], ["expansion-valve", "🏗️machinery"], ["finned-heat-exchanger", "🏗️machinery"], ["sorption-wheel", "🔌️appliances"], ["plate-heat-exchanger", "🔌️appliances"], ["evaporative-humidifier", "🔌️appliances"]];
const distinct = {
  "gas-hob": ["A 0.30 m domino gas hob whose flame crown lifts out of the plate when lit, whose pan support glows and which releases about 2.5 kW.", "Ein 0,30 m breites Domino-Gaskochfeld, dessen Flammenkranz beim Zünden aus der Platte wächst, dessen Pfannenträger aufglüht und das rund 2,5 kW abgibt."],
  "cooking-pot": ["A 0.22 m stock pot cut open through its axis with 3 L of water that warms to the boil, bobs and sends up a steam plume.", "Ein 0,22 m großer, durch die Achse aufgeschnittener Kochtopf mit 3 l Wasser, das sich bis zum Sieden erwärmt, wogt und eine Dampffahne aufsteigen lässt."],
  "electric-kettle": ["A 1.7 L jug kettle on its base that heats, boils with steam at the spout and rattles its lid.", "Ein 1,7-l-Wasserkocher auf seinem Sockel, der heizt, mit Dampf am Ausguss kocht und mit dem Deckel klappert."],
  "scroll-compressor": ["A 0.43 m hermetic scroll compressor with a sight glass on the orbiting scroll, a hot discharge dome and a cold suction stub.", "Ein 0,43 m hoher hermetischer Scrollverdichter mit Schauglas auf die orbitierende Spirale, heißem Druckdom und kaltem Saugstutzen."],
  "expansion-valve": ["A brass thermostatic expansion valve with a sensing bulb whose outlet and body run cold while the liquid inlet stays warm.", "Ein thermostatisches Expansionsventil aus Messing mit Fühler, dessen Ausgang und Körper kalt laufen, während der Flüssigkeitseingang warm bleibt."],
  "finned-heat-exchanger": ["A 0.52 m finned tube coil in section that is cold as evaporator or cooling coil and hot as condenser or heating coil.", "Ein 0,52 m hoher Lamellenrohr-Register im Schnitt, der als Verdampfer oder Kühler kalt und als Verflüssiger oder Erhitzer heiß ist."],
  "sorption-wheel": ["A 0.62 m desiccant wheel face-on with a turning honeycomb matrix and a hot regeneration sector.", "Ein 0,62 m großes Sorptionsrad in der Frontansicht mit drehender Wabenmatrix und heißem Regenerationssektor."],
  "plate-heat-exchanger": ["A 0.5 m by 0.6 m cross-flow plate heat exchanger in section with a hatched plate pack, four collars and a bypass damper flap.", "Ein 0,5 m mal 0,6 m großer Kreuzstrom-Plattenwärmeübertrager im Schnitt mit schraffiertem Plattenpaket, vier Stutzen und Bypass-Klappe."],
  "evaporative-humidifier": ["A 0.5 m by 0.6 m evaporative humidifier cut open: drops fall over a fluted pad that cools below the air temperature.", "Ein aufgeschnittener Verdunstungsbefeuchter von 0,5 m mal 0,6 m: Tropfen fallen über eine Wellenmatte, die unter die Lufttemperatur abkühlt."],
};
const rows = [];
for (const [id, folder] of plan) {
  const a = JSON.parse(readFileSync(`${S}/${folder}/${id}/🔣️.json`, "utf8")).asset;
  const row = {
    id, category: a.category, title: a.title, heightM: a.dimensions.heightM, widthM: a.dimensions.widthM,
    poses: a.poses.filter((p) => !p.key).map((p) => p.id), clips: a.clips.map((c) => c.id),
    representations: Object.fromEntries(["50", "100", "200", "500"].map((k) => [k, a.representations[k].status])),
    symbols: [], analytics: ["thermal", ...(a.thermal.emitters.length ? ["transport"] : []), "structural"],
    viewpoints: ["elevation"], exports: a.exports, distinction: { en: distinct[id][0], de: distinct[id][1] }, completion: "authored",
  };
  rows.push(JSON.stringify(row, null, 2).split("\n").map((l) => "    " + l).join("\n"));
}
writeFileSync(process.argv[2], rows.join(",\n") + ",\n");
