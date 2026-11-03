import { readFileSync, writeFileSync } from "node:fs";
const file = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/ivy-on-trellis/🔣️.json";
const t = readFileSync(file, "utf8"); const indent = t.split("\n")[1]!.match(/^ */)![0].length; const doc = JSON.parse(t); const g = doc.asset.representations["500"].geometry;
for (const s of g.front) if (s.id === "crown-L") s.regions = "leaves-upper" === s.regions?.[0] ? undefined : s.regions;
const crown = g.front.find((s: any) => s.id === "crown-L"); delete crown.regions; crown.region = "leaves-upper";
for (const v of ["elevation", "front", "plan"]) for (const s of g[v]) if (s.id === "screen" && Array.isArray(s.regions) && !s.regions.includes("stems")) s.regions.push("stems");
const rep = doc.asset.representations["500"]; rep.decisions.merged = { en: rep.decisions.merged.en.replace(" The ivy stems (under 12 mm thick, below the print minimum at this scale) are hidden under the leaf mat, so their region is merged into the lower leaf shapes.", " The ivy stems (under 12 mm thick, below the print minimum at this scale) are hidden under the leaf mat, so their region is merged into the screen panel."), de: rep.decisions.merged.de.replace("ihre Region ist daher in die unteren Blattformen eingeschmolzen.", "ihre Region ist daher in die Tafelform eingeschmolzen.") };
writeFileSync(file, JSON.stringify(doc, null, indent) + "\n");
for (const v of ["elevation", "front", "plan"]) console.log(v, g[v].filter((s: any) => (s.regions ?? []).includes("stems")).map((s: any) => s.id).join(","));
