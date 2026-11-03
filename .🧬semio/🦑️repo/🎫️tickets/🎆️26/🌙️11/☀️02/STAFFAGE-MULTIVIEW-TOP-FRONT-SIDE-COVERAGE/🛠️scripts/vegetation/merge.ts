import { readFileSync, writeFileSync } from "node:fs";
const CAT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation";
const jobs: [string, string, string[], string[], { en: string; de: string }][] = [
  ["hosta-clump", "petioles", ["200"], ["blades-inner", "blades-outer"], { en: " The petioles (12 to 32 mm thick, 0.06 to 0.16 mm at this scale) are below the print minimum and hidden under the blades, so their region is merged into the blade shapes of the inner blades.", de: " Die Blattstiele (12 bis 32 mm dick, 0,06 bis 0,16 mm in diesem Maßstab) liegen unter der Druckgrenze und sind unter den Spreiten verdeckt; ihre Region ist daher in die Formen der inneren Spreiten eingeschmolzen." }],
  ["ivy-on-trellis", "stems", ["100", "200", "500"], ["leaves-lower", "leaves-upper"], { en: " The ivy stems (under 12 mm thick, below the print minimum at this scale) are hidden under the leaf mat, so their region is merged into the lower leaf shapes.", de: " Die Efeutriebe (unter 12 mm dick, in diesem Maßstab unter der Druckgrenze) sind unter dem Blattteppich verdeckt; ihre Region ist daher in die unteren Blattformen eingeschmolzen." }],
];
for (const [d, region, bands, prefs, note] of jobs) {
  const file = `${CAT}/${d}/🔣️.json`; const t = readFileSync(file, "utf8"); const indent = t.split("\n")[1]!.match(/^ */)![0].length; const doc = JSON.parse(t); const a = doc.asset;
  for (const den of bands) {
    const rep = a.representations[den]; if (rep.status !== "authored") continue;
    for (const v of ["elevation", "front", "plan"]) {
      const shapes = rep.geometry[v]; let target: any;
      for (const p of prefs) { target = shapes.find((s: any) => s.region === p); if (target) break; }
      if (!target) { console.log("no target", d, den, v); continue; }
      target.regions = [target.region, region]; delete target.region; console.log(d, den, v, "merged into", target.id, JSON.stringify(target.regions));
    }
    rep.decisions.merged = { en: rep.decisions.merged.en + note.en, de: rep.decisions.merged.de + note.de };
  }
  writeFileSync(file, JSON.stringify(doc, null, indent) + "\n");
}
