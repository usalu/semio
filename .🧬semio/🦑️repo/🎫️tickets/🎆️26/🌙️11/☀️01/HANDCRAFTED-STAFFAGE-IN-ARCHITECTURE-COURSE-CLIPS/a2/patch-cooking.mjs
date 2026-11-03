import fs from "node:fs";
const f = process.argv[2];
let s = fs.readFileSync(f, "utf8");
s = s.replace("a.dimensions.widthM = 0.98;", "a.dimensions.widthM = 1.028;");
s = s.replace(
  "];\n\nconst keys = [];",
  `  poseOf("standing-by", T("Standing by the counter", "An der Zeile abwartend"), T("Upright at the counter with the spoon hanging in the lowered hand along the thigh, the head turned toward the pot, waiting for the water to boil.", "Aufrecht an der Zeile, der Löffel hängt in der gesenkten Hand am Oberschenkel, der Kopf ist dem Topf zugewandt, die Person wartet, bis das Wasser kocht."), { torso: { angleDeg: 0 }, "arm-r": { angleDeg: 5 }, "forearm-r": { angleDeg: 10 }, "hand-r": { angleDeg: 0 }, spoon: { angleDeg: -15 }, neck: { angleDeg: -1 }, head: { angleDeg: -3 } }, 1.4, 0.4),
];

const keys = [];`
);
s = s.replace(
  "a.presentation.defaultPose",
  `a.clips.push({
  id: "stir-and-taste", title: T("Stirring and tasting", "Rühren und Probieren"),
  description: T("A four-second loop: the person stirs, lifts the spoon to the mouth to taste, holds it a moment and goes back to stirring.", "Eine Schleife von vier Sekunden: Die Person rührt, führt den Löffel zum Probieren zum Mund, hält kurz inne und rührt weiter."),
  durationS: 4, fps: 30, loop: true, rootMotion: "none",
  keys: [{ t: 0, pose: "stirring", ease: "smooth" }, { t: 1, pose: "stirring", ease: "smooth" }, { t: 2, pose: "tasting", ease: "smooth" }, { t: 2.8, pose: "tasting", ease: "smooth" }, { t: 4, pose: "stirring", ease: "smooth" }],
  contacts: ["heel-l", "ball-l", "heel-r", "ball-r", "counter-contact"].map((anchor) => ({ anchor, fromS: 0, toS: 4 })),
  phases: [{ id: "stir", fromS: 0, toS: 1, title: T("Stirring", "Rühren") }, { id: "taste", fromS: 1, toS: 2.8, title: T("Tasting", "Probieren") }, { id: "back", fromS: 2.8, toS: 4, title: T("Back to stirring", "Zurück zum Rühren") }],
  events: [],
  tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 4, v: 0 }] }, { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.8 }, { t: 2, v: 1.6 }, { t: 4, v: 1.8 }] }],
});
a.presentation.defaultPose`
);
fs.writeFileSync(f, s);
