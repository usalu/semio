import { readFileSync, writeFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
type T = { en: string; de: string };
const t = (en: string, de: string): T => ({ en, de });
const jobs: Record<string, { file: string; drop?: Record<string, string[]>; decisions: Record<string, T> }> = {
  chalkboard: {
    file: `${L}/🪑️objects/chalkboard/🔣️.json`,
    decisions: {
      silhouette: t("Sliding blackboard sign 8.0 x 2.5 mm: the 4 m wall board as one long rectangle with the 2 m leaf as a second rectangle on top that slides left and right with the leaf bone; the side view is the board face, the front view the 0.08 m end of frame, tray and leaf, the plan the 0.16 mm thin top edge with the leaf strip.", "Zeichen der Schiebetafel 8,0 x 2,5 mm: die 4 m lange Wandtafel als ein langes Rechteck mit dem 2-m-Flügel als zweitem Rechteck darüber, das mit dem Flügelknochen nach links und rechts gleitet; die Seitenansicht ist die Tafelfläche, die Vorderansicht das 0,08 m dicke Ende aus Rahmen, Ablage und Flügel, die Draufsicht die 0,16 mm dünne Oberkante mit dem Flügelstreifen."),
      retained: t("Overall width 4 m, height 1.26 m, the tray as its own band at the foot, the sliding leaf with its real travel of 2 m.", "Gesamtbreite 4 m, Höhe 1,26 m, die Ablage als eigener Streifen am Fuß, der Schiebeflügel mit seinem echten Weg von 2 m."),
      merged: t("Frame, board face and chalk groove are one rectangle.", "Rahmen, Tafelfläche und Kreidenut sind ein Rechteck."),
      omitted: t("Wall hooks, rest, chalk, frame profile and the leaf rails (all below 0.1 m).", "Wandhaken, Auflager, Kreide, Rahmenprofil und die Flügelschienen (alle unter 0,1 m)."),
      thinElements: t("The 0.08 m depth is 0.16 mm on paper, drawn as the outline of one thin rectangle in the top view and the 0.18 mm outline of the end in the front view.", "Die Tiefe von 0,08 m ist 0,16 mm auf Papier, gezeichnet als Kontur eines dünnen Rechtecks in der Draufsicht und als 0,18-mm-Kontur des Endes in der Vorderansicht."),
      hatching: t("None.", "Keine."),
      styles: t("Board and leaf in one tone, tray darker; one 0.18 mm outline.", "Tafel und Flügel in einem Ton, Ablage dunkler; eine 0,18-mm-Kontur."),
      overlays: t("Thermal regions as at 1:200: frame and board face share one shape, the leaf and the tray stay separate so the leaf moves with its bone.", "Thermische Regionen wie bei 1:200: Rahmen und Tafelfläche teilen eine Form, Flügel und Ablage bleiben getrennt, damit sich der Flügel mit seinem Knochen bewegt."),
    },
  },
  "storage-crate-stack": {
    file: `${L}/🪑️objects/storage-crate-stack/🔣️.json`,
    decisions: {
      silhouette: t("Stack sign 1.2 x 3.0 mm: five tapered crate blocks one above the other in the side and front view, five square outlines on top of each other in plan; every crate is its own shape bound to its own bone, so crates stack, nest and unload with their poses.", "Stapelzeichen 1,2 x 3,0 mm: fünf konische Kistenblöcke übereinander in der Seiten- und Vorderansicht, fünf deckungsgleiche Quadratkonturen in der Draufsicht; jede Kiste ist eine eigene Form an ihrem eigenen Knochen, sodass sich die Kisten mit ihren Haltungen stapeln, ineinanderstellen und abladen."),
      retained: t("Overall size 0.62 x 0.40 x 1.50 m, the five crate joints, the taper of every crate.", "Gesamtmaß 0,62 x 0,40 x 1,50 m, die fünf Kistenfugen, die Verjüngung jeder Kiste."),
      merged: t("Handles, ribs and wall slots vanish into the crate block.", "Griffe, Rippen und Wandschlitze gehen im Kistenblock auf."),
      omitted: t("Handles, ribs, slots and the nesting lip of 0.08 m (0.16 mm).", "Griffe, Rippen, Schlitze und die Stapelkante von 0,08 m (0,16 mm)."),
      thinElements: t("The joint between two crates is the shared 0.18 mm outline; the 0.08 m nesting step is only readable in the poses as the crates sink into each other.", "Die Fuge zwischen zwei Kisten ist die gemeinsame 0,18-mm-Kontur; die Stapelstufe von 0,08 m ist nur in den Haltungen lesbar, wenn die Kisten ineinander sinken."),
      hatching: t("None.", "Keine."),
      styles: t("All crates in one tone; one 0.18 mm outline.", "Alle Kisten in einem Ton; eine 0,18-mm-Kontur."),
      overlays: t("Each crate keeps its thermal region as at 1:200.", "Jede Kiste behält ihre thermische Region wie bei 1:200."),
    },
  },
  "operating-table": {
    file: `${L}/🛋️furniture/operating-table/🔣️.json`,
    drop: { elevation: ["st-rod"] },
    decisions: {
      silhouette: t("Operating-room sign 4.9 x 3.9 mm: the 2 m table top as a long slab on its column and base plate, the lamp stand beside it with its horizontal arm and the lamp head hanging on it; the front view is the 0.5 m wide slab with the stand column behind, the plan the slab, the stand foot, the arm and the round lamp head; the top tilts with its bone.", "Zeichen des OP-Raums 4,9 x 3,9 mm: die 2 m lange Tischplatte als langer Balken auf Säule und Fußplatte, daneben der Leuchtenständer mit waagrechtem Ausleger und daran hängendem Leuchtenkopf; die Vorderansicht ist der 0,5 m breite Balken mit der Ständersäule dahinter, die Draufsicht Balken, Ständerfuß, Ausleger und runder Leuchtenkopf; die Platte kippt mit ihrem Knochen."),
      retained: t("Overall length 2.45 m, lamp height 1.96 m, slab, column, base plate, stand column, arm and lamp head as separate bone-bound shapes.", "Gesamtlänge 2,45 m, Leuchtenhöhe 1,96 m, Platte, Säule, Fußplatte, Ständersäule, Ausleger und Leuchtenkopf als getrennte, an Knochen gebundene Formen."),
      merged: t("Pad and frame of the top are one slab.", "Polster und Rahmen der Platte sind ein Balken."),
      omitted: t("Lamp rod, castors, rails, handles and the control box (all below 0.1 m).", "Leuchtenstab, Rollen, Schienen, Griffe und der Schaltkasten (alle unter 0,1 m)."),
      thinElements: t("The stand column of 0.08 m is 0.16 mm and the arm 0.06 m is 0.12 mm; they are drawn as single outlines of 0.18 mm, which thickens them slightly.", "Die Ständersäule von 0,08 m ist 0,16 mm und der Ausleger von 0,06 m 0,12 mm; sie werden als einfache 0,18-mm-Konturen gezeichnet, was sie leicht verstärkt."),
      hatching: t("None.", "Keine."),
      styles: t("Slab light, base and stand mid-tone, lamp head dark; one 0.18 mm outline.", "Balken hell, Fuß und Ständer mittlerer Ton, Leuchtenkopf dunkel; eine 0,18-mm-Kontur."),
      overlays: t("Thermal regions as at 1:200: the lamp head carries the housing and lens regions.", "Thermische Regionen wie bei 1:200: der Leuchtenkopf trägt die Gehäuse- und Linsenregion."),
    },
  },
};
const matchEnd = (s: string, open: number, o: string, c: string): number => {
  let depth = 0, str = false;
  for (let i = open; i < s.length; i++) {
    const ch = s[i]!;
    if (str) { if (ch === "\\") i++; else if (ch === '"') str = false; continue; }
    if (ch === '"') str = true;
    else if (ch === o) depth++;
    else if (ch === c && --depth === 0) return i;
  }
  throw new Error("unbalanced");
};
for (const [slug, job] of Object.entries(jobs)) {
  let text = readFileSync(job.file, "utf8");
  const reps = text.indexOf('"representations"');
  const m = /\n( *)"500": \{/.exec(text.slice(reps))!;
  const start = reps + m.index + m[0].length - 1;
  const end = matchEnd(text, start, "{", "}");
  const asset = JSON.parse(readFileSync(job.file, "utf8")).asset;
  const base = JSON.parse(JSON.stringify(asset.representations["200"]));
  const rep = { ...base, denominator: 500, validRange: { minDenominator: 350, maxDenominator: 1000 }, decisions: job.decisions };
  for (const [view, ids] of Object.entries(job.drop ?? {})) rep.geometry[view] = rep.geometry[view].filter((s: any) => !ids.includes(s.id));
  const unit = (/\n( +)"\$schema"/.exec(text)?.[1] ?? "  ").length;
  const body = JSON.stringify(rep, null, unit).split("\n").map((l, n) => (n === 0 ? l : m[1] + l)).join("\n");
  text = text.slice(0, start) + body + text.slice(end + 1);
  writeFileSync(job.file, text);
  console.log(slug, "500 band written", Object.keys(rep.geometry).map((v) => `${v}:${rep.geometry[v].length}`).join(" "));
}
