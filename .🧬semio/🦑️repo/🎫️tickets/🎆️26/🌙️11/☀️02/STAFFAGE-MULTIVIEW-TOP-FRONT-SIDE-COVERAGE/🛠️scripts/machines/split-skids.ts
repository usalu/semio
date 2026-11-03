import { readFileSync, writeFileSync } from "node:fs";
import { fileOf } from "./lib.ts";
type Spec = { id: string; map: Record<string, string>; z: number; width: number; titles?: Record<string, [string, string, string, string]> };
const specs: Spec[] = [
  { id: "excavator", map: { "track-r2": "track-r2", "track-r1": "track-r1", "track-f1": "track-f1", "track-f2": "track-f2" }, z: 1.2, width: 0.6 },
  { id: "diesel-generator", map: { "contact-l": "contact-a", "contact-r": "contact-b" }, z: 1.13, width: 0.3 },
  { id: "oil-pumpjack", map: { "contact-l": "contact-a", "contact-r": "contact-b" }, z: 0.6, width: 1.2 },
];
const TEXT: Record<string, [string, string][]> = {
  excavator: [[ "four load-sharing ground contacts", "eight load-sharing ground contacts (four under each track)"], ["Four equally stiff ground patches under the tracks share the load; a patch that would pull away from the ground is dropped, which exposes the tipping line about the front patch.", "Eight equally stiff ground patches, four under the left and four under the right track, share the load; a patch that would pull away from the ground is dropped, which exposes the tipping line about the two front patches."]],
};
const DE: Record<string, [string, string][]> = {
  excavator: [["vier lastverteilenden Bodenkontakten", "acht lastverteilenden Bodenkontakten (vier unter jeder Kette)"], ["Vier gleich steife Standflächen unter den Ketten teilen die Last; eine Fläche, die vom Boden abheben würde, entfällt; das zeigt die Kipplinie um die vordere Fläche.", "Acht gleich steife Standflächen, vier unter der linken und vier unter der rechten Kette, teilen die Last; eine Fläche, die vom Boden abheben würde, entfällt; das zeigt die Kipplinie um die beiden vorderen Flächen."]],
  "diesel-generator": [["Der Container ruht auf zwei Eckbeschlag-Auflagern", "Der Container ruht auf vier Eckbeschlag-Auflagern"]],
  "oil-pumpjack": [["Die Einheit ruht mit ihrer Platte auf zwei Auflagern", "Die Einheit ruht mit ihrer Platte auf vier Auflagern (links und rechts an beiden Enden)"]],
};
const EN: Record<string, [string, string][]> = {
  ...TEXT,
  "diesel-generator": [["The container rests on two corner-casting supports", "The container rests on four corner-casting supports (two at each end, left and right)"]],
  "oil-pumpjack": [["The unit rests on its slab at two bearings", "The unit rests on its slab at four bearings (left and right at both ends)"]],
};
for (const s of specs) {
  const file = fileOf(s.id);
  let doc = JSON.parse(readFileSync(file, "utf8"));
  const a = doc.asset;
  const names = new Map<string, string[]>();
  const out: any[] = [];
  for (const an of a.anchors) {
    if (!(an.id in s.map)) { out.push(an); continue; }
    const base = s.map[an.id];
    const ids = [`${base}-l`, `${base}-r`];
    names.set(an.id, ids);
    const end = s.id === "excavator" ? "" : an.id === "contact-l" ? ", end at the negative x of the drawing" : ", end at the positive x of the drawing";
    const endDe = s.id === "excavator" ? "" : an.id === "contact-l" ? ", Ende bei negativem x der Zeichnung" : ", Ende bei positivem x der Zeichnung";
    for (const [i, z, en, de] of [[ids[0], s.z, "left", "links"], [ids[1], -s.z, "right", "rechts"]] as const)
      out.push({ ...an, id: i, z, title: { en: `${an.title.en.replace(/, (left|right)$/, "")}${end} (${en})`, de: `${an.title.de.replace(/ (links|rechts)$/, "")}${endDe} (${de})` }, footprintWidthM: s.width });
  }
  a.anchors = out;
  for (const p of a.poses) if (p.contacts) p.contacts = p.contacts.flatMap((c: string) => names.get(c) ?? [c]);
  for (const c of a.clips) if (c.contacts) c.contacts = c.contacts.flatMap((x: any) => (names.get(x.anchor) ?? [x.anchor]).map((n) => ({ ...x, anchor: n })));
  let json = JSON.stringify(doc, null, 1);
  for (const [o, n] of EN[s.id] ?? []) { if (!json.includes(o)) console.log("MISSING en text", s.id, o.slice(0, 50)); json = json.split(o).join(n); }
  for (const [o, n] of DE[s.id] ?? []) { if (!json.includes(o)) console.log("MISSING de text", s.id, o.slice(0, 50)); json = json.split(o).join(n); }
  if (process.argv.includes("--write")) { writeFileSync(file, json + "\n"); console.log("written", s.id, [...names].map(([k, v]) => `${k}->${v}`).join(" ")); }
}
