import { readFileSync, writeFileSync } from "node:fs";
const root = process.argv[2]!;
const edit = (file: string, pairs: readonly (readonly [string, string])[]): void => {
  let text = readFileSync(`${root}/${file}`, "utf8");
  for (const [from, to] of pairs) {
    if (!text.includes(from)) throw new Error(`missing in ${file}: ${from.slice(0, 70)}`);
    text = text.replace(from, to);
  }
  writeFileSync(`${root}/${file}`, text);
};
edit("✂️section/🟦️.ts", [
  ['elevation: tx("Side view", "Seitenansicht"), front: tx("Front view", "Vorderansicht"), plan: tx("Top view", "Draufsicht") };', 'elevation: tx("Side elevation", "Seitenansicht"), front: tx("Front elevation", "Vorderansicht"), plan: tx("Plan (top view)", "Draufsicht") };\n\nconst SECTION_NAMES: Readonly<Record<"front" | "plan", Text>> = { front: tx("Section", "Schnitt"), plan: tx("Plan section", "Grundriss-Schnitt") };'],
  ['"Section A-A" / "Schnitt A-A" when it is drawn as a section, else "Side view", "Front view" or "Top view" (Seitenansicht, Vorderansicht, Draufsicht)', '"Section A-A" / "Schnitt A-A" for a front cut and "Plan section B-B" / "Grundriss-Schnitt B-B" for a plan cut, else "Side elevation", "Front elevation" or "Plan (top view)" (Seitenansicht, Vorderansicht, Draufsicht)'],
  ['return `${language === "de" ? "Schnitt" : "Section"} ${section.label[language]}`;', 'return `${SECTION_NAMES[viewpoint === "plan" ? "plan" : "front"][language]} ${section.label[language]}`;'],
]);
edit("🟦️.ts", [
  ['tx("Line drawing, dark", "Strichzeichnung, dunkel")', 'tx("Dark line", "Dunkle Strichzeichnung")'],
  ['tx("Solid, dark", "Vollton, dunkel")', 'tx("Dark solid", "Dunkler Vollton")'],
  ['tx("Line drawing, light", "Strichzeichnung, hell")', 'tx("Light line", "Helle Strichzeichnung")'],
  ['tx("Solid, light", "Vollton, hell")', 'tx("Light solid", "Heller Vollton")'],
  ['const caption = options.viewpoint === "elevation" ? viewTitle : `${viewTitle} · ${viewpointCaption(', 'const caption = `${viewTitle} · ${viewpointCaption('],
]);
