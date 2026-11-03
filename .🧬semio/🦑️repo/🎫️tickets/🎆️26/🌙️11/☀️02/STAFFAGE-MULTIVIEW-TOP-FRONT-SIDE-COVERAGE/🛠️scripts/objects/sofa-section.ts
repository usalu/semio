import { readFileSync, writeFileSync } from "node:fs";
import { fileOf } from "./patch.ts";

const file = fileOf("🛋️furniture", "three-seat-sofa");
const addition = {
  en: " The front elevation is Section A-A at x = 1.0 m through the middle seat and back cushion, looking back from the front end: the cut cushions and the skirt are the section, the left arm and the neighbouring cushions lie beyond the cut plane and are hidden by it, so only the arm rises above the seat.",
  de: " Die Vorderansicht ist Schnitt A-A bei x = 1,0 m durch das mittlere Sitz- und Rückenkissen, von vorn zurückgesehen: die geschnittenen Kissen und die Zarge bilden den Schnitt, die linke Armlehne und die benachbarten Kissen liegen hinter der Schnittebene und werden von ihr verdeckt, sodass nur die Armlehne über den Sitz ragt.",
};
let text = readFileSync(file, "utf8");
const reps = text.indexOf('"representations"');
const unit = (/\n( +)"\$schema"/.exec(text)?.[1] ?? "  ").length;
for (const band of [500, 200, 100, 50]) {
  const m = new RegExp(`\\n( *)"${band}": \\{\\s*"status": "authored"`).exec(text.slice(reps));
  if (!m) throw new Error(`band ${band}`);
  const from = reps + m.index;
  if (text.slice(from, from + 200000).indexOf('"sections"') >= 0 && text.slice(from, from + 200000).indexOf('"sections"') < text.slice(from, from + 200000).indexOf('"decisions"')) continue;
  const d = /\n( *)"decisions": \{/.exec(text.slice(from))!;
  const at = from + d.index;
  const ind = d[1]!;
  const block = JSON.stringify({ front: { plane: 1.0, label: { en: "A-A", de: "A-A" } } }, null, unit)
    .split("\n")
    .map((l, n) => (n === 0 ? l : ind + l))
    .join("\n");
  text = text.slice(0, at) + `\n${ind}"sections": ${block},` + text.slice(at);
  const sil = new RegExp(`("silhouette": \\{\\s*"en": ")((?:[^"\\\\]|\\\\.)*)("\\s*,\\s*"de": ")((?:[^"\\\\]|\\\\.)*)(")`).exec(text.slice(at))!;
  const start = at + sil.index;
  const replaced = sil[1]! + sil[2]! + addition.en + sil[3]! + sil[4]! + addition.de + sil[5]!;
  text = text.slice(0, start) + replaced + text.slice(start + sil[0].length);
}
writeFileSync(file, text);
