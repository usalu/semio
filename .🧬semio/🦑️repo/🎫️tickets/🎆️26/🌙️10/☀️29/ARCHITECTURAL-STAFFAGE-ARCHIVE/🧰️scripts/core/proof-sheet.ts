import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { parseArgs } from "node:util";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const { loadCatalogFromFiles } = await import(`${S}/🗂️catalog/📥️files/🟦️.ts`);
const { evaluate } = await import(`${S}/🧮️evaluation/🟦️.ts`);
const { defaultDepictionOptions, depict } = await import(`${S}/🎨️depiction/🟦️.ts`);
const { explanationBox } = await import(`${S}/🎨️depiction/🪧️explanation/🟦️.ts`);
const { DEFAULT_PRINT_PROFILE, resolveRepresentation } = await import(`${S}/🖨️representation/🟦️.ts`);
const { writeSvg } = await import(`${S}/📤️export/🖼️svg/🟦️.ts`);
const { writePdf } = await import(`${S}/📤️export/📕️pdf/🟦️.ts`);

const { values } = parseArgs({ options: { assets: { type: "string" }, out: { type: "string" }, name: { type: "string", default: "proof-sheet" }, png: { type: "boolean", default: true }, language: { type: "string", default: "en" } } });
const out = values.out!;
mkdirSync(out, { recursive: true });
const language = values.language as "en" | "de";
const catalog = await loadCatalogFromFiles();
const wanted = values.assets === "all" ? catalog.assets.map((a: any) => a.id) : values.assets!.split(",");
const UNCHECKED = { id: "unchecked", title: { en: "Unchecked", de: "Ungeprüft" }, description: { en: "", de: "" }, minStrokeMm: 0, minGapMm: 0, minFeatureMm: 0, minHatchSpacingMm: 0, source: "illustrative" };
const A3 = { w: 420, h: 297 };
const INK = "#001117";
const mm = (v: number): string => (Number.isFinite(v) ? v.toFixed(2) : "–");

type Item = any;
const text = (id: string, x: number, y: number, t: string, size = 2, anchor = "start", weight = "normal"): Item => ({ kind: "text", id, x, y, text: t, sizeMm: size, color: INK, anchor, weight });
const line = (id: string, pts: [number, number][], width = 0.18, closed = false): Item => ({ kind: "path", id, commands: [...pts.map(([x, y], i) => ({ op: i === 0 ? "M" : "L", x, y })), ...(closed ? [{ op: "Z" }] : [])], stroke: { color: INK, widthMm: width, cap: "butt" } });
const shiftItem = (item: Item, dx: number, dy: number): Item => item.kind === "text" ? { ...item, x: item.x + dx, y: item.y + dy } : { ...item, commands: item.commands.map((c: any) => c.op === "Z" ? c : c.op === "C" ? { ...c, x1: c.x1 + dx, y1: c.y1 + dy, x2: c.x2 + dx, y2: c.y2 + dy, x: c.x + dx, y: c.y + dy } : { ...c, x: c.x + dx, y: c.y + dy }) };

type Cell = { d: number; scene: any | null; widthMm: number; above: number; below: number; label: string[]; box: any | null };

function cellOf(asset: any, d: number): Cell {
  const resolution = resolveRepresentation(asset, { denominator: d }, DEFAULT_PRINT_PROFILE);
  const head = `1:${d}`;
  if (resolution.kind === "unavailable") {
    const box = explanationBox(asset, resolution, language, 62);
    return { d, scene: null, widthMm: 64, above: 0, below: 0, label: [`${head} ${language === "de" ? "nicht verfügbar" : "unavailable"}`], box };
  }
  const suit = resolution.suitability;
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
  const o = defaultDepictionOptions("light-line", language, d);
  const scene = depict(state, { ...o, profile: UNCHECKED, marginMm: 1.5, annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: true } });
  const status = resolution.kind === "authored" ? (language === "de" ? "gezeichnet" : "authored") : (language === "de" ? "NICHT DRUCKBAR" : "UNSUITABLE");
  const label = [`${head} ${status}`, `${language === "de" ? "min. Strich/Lücke/Einzelheit" : "min stroke/gap/feature"} ${mm(suit.minStrokeMm)}/${mm(suit.minGapMm)}/${mm(suit.minFeatureMm)} mm`];
  if (!suit.suitable) label.push(`${suit.violations.length} ${language === "de" ? "Verletzungen" : "violations"}: ${[...new Set(suit.violations.map((v: any) => v.check))].join(", ")}`);
  return { d, scene, widthMm: Math.max(scene.widthMm, 60, 5 * (1000 / d) + 26), above: scene.metadata.origin.yMm, below: scene.heightMm - scene.metadata.origin.yMm, label, box: null };
}

function pageOf(asset: any, rows: Cell[][], pageNo: number, pages: number): any {
  const layers = new Map<string, Item[]>();
  const add = (layer: string, item: Item): void => void (layers.get(layer) ?? layers.set(layer, []).get(layer)!).push(item);
  const meta = rows.flat().find((c) => c.scene)?.scene?.metadata ?? { asset: asset.id, title: asset.title, query: { kind: "pose", pose: asset.presentation.defaultPose }, timeS: 0, view: "light-line", viewpoint: "elevation", scale: { denominator: 0 }, resolution: "unavailable", representation: { kind: "none" }, profile: "none", paperUnits: "mm", modelUnits: "m", mmPerMetre: 0, origin: { xMm: 0, yMm: 0 } };
  add("annotations", text("sheet:title", 10, 11, `${asset.title[language]} (${asset.id})`, 5, "start", "bold"));
  add("annotations", text("sheet:sub", 10, 17, `${asset.dimensions.heightM} × ${asset.dimensions.widthM} m, ${asset.dimensions.massKg} kg · ${language === "de" ? "Pose" : "pose"} ${asset.presentation.defaultPose} · ${language === "de" ? "Profil" : "profile"} ${DEFAULT_PRINT_PROFILE.id} · ${language === "de" ? "A3 quer, wahre Papiergröße (100 % drucken)" : "A3 landscape, true paper size (print at 100 %)"} · ${pageNo}/${pages}`, 2.6));
  let y = 24;
  for (const row of rows) {
    const above = Math.max(...row.map((c) => c.above), 14);
    const below = Math.max(...row.map((c) => c.below), 0);
    const labelH = 11;
    const base = y + labelH + above;
    let x = 10;
    for (const cell of row) {
      cell.label.forEach((l, i) => add("annotations", text(`label:${cell.d}:${i}`, x, y + 3 + i * 3.4, l, i === 0 ? 2.8 : 2.2, "start", i === 0 ? "bold" : "normal")));
      if (cell.scene) {
        const dx = x - 0;
        const dy = base - cell.scene.metadata.origin.yMm;
        for (const layer of cell.scene.layers) for (const item of layer.items) add(layer.id, shiftItem({ ...item, id: `${cell.d}:${item.id}` }, dx, dy));
        const k = 1000 / cell.d;
        const by = base + below + 4;
        add("annotations", line(`ref10:${cell.d}`, [[x, by], [x + 10, by]], 0.35));
        add("annotations", line(`ref10a:${cell.d}`, [[x, by - 1], [x, by + 1]], 0.25));
        add("annotations", line(`ref10b:${cell.d}`, [[x + 10, by - 1], [x + 10, by + 1]], 0.25));
        add("annotations", text(`ref10t:${cell.d}`, x + 12, by + 0.7, `10 mm ${language === "de" ? "Papier" : "paper"} = ${(10 * cell.d / 1000).toFixed(cell.d >= 200 ? 0 : 1)} m`, 2));
        const by2 = by + 4;
        add("annotations", line(`ref1m:${cell.d}`, [[x, by2], [x + k, by2]], 0.35));
        add("annotations", line(`ref1ma:${cell.d}`, [[x, by2 - 1], [x, by2 + 1]], 0.25));
        add("annotations", line(`ref1mb:${cell.d}`, [[x + k, by2 - 1], [x + k, by2 + 1]], 0.25));
        add("annotations", text(`ref1mt:${cell.d}`, x + k + 2, by2 + 0.7, `1 m = ${k.toFixed(k < 10 ? 1 : 0)} mm`, 2));
        const bar = 5 * k;
        const by3 = by2 + 4;
        for (let i = 0; i < 5; i++) add("annotations", { kind: "path", id: `bar:${cell.d}:${i}`, commands: [{ op: "M", x: x + i * k, y: by3 - 0.7 }, { op: "L", x: x + (i + 1) * k, y: by3 - 0.7 }, { op: "L", x: x + (i + 1) * k, y: by3 + 0.7 }, { op: "L", x: x + i * k, y: by3 + 0.7 }, { op: "Z" }], stroke: { color: INK, widthMm: 0.15 }, ...(i % 2 === 0 ? { fill: { color: INK } } : {}) });
        add("annotations", text(`bart:${cell.d}`, x + bar + 2, by3 + 0.7, `0–5 m (1:${cell.d})`, 2));
      } else if (cell.box) {
        cell.box.lines.forEach((l: any, i: number) => add("annotations", text(`box:${cell.d}:${i}`, x + 1, y + labelH + 4 + i * 2.6, l.text, l.bold ? 2.2 : 1.8, "start", l.bold ? "bold" : "normal")));
        add("annotations", line(`boxf:${cell.d}`, [[x, y + labelH], [x + 62, y + labelH], [x + 62, y + labelH + cell.box.heightMm * 1.3], [x, y + labelH + cell.box.heightMm * 1.3]], 0.2, true));
      }
      x += cell.widthMm + 6;
    }
    y = base + below + 4 + 16;
  }
  const order = ["background", "ground", "fill", "field", "body", "detail", "texture", "glazing", "diagrams", "forces", "annotations", "legend"];
  return {
    widthMm: A3.w, heightMm: A3.h, background: "#ffffff",
    layers: order.filter((id) => layers.has(id)).map((id) => ({ id, title: { en: id, de: id }, items: layers.get(id)! })),
    legends: [],
    metadata: { ...meta, asset: asset.id, title: asset.title, view: "light-line", scale: { denominator: 0 }, resolution: "authored", representation: { kind: "none" }, mmPerMetre: 0, origin: { xMm: 0, yMm: 0 } },
  };
}

const pages: any[] = [];
for (const id of wanted) {
  const asset = catalog.assets.find((a: any) => a.id === id);
  if (!asset) throw new Error(`unknown asset ${id}`);
  const cells = [500, 200, 100, 50].map((d) => cellOf(asset, d));
  const rows: Cell[][] = [[]];
  let x = 10;
  for (const cell of cells) {
    if (x + cell.widthMm > A3.w - 8 && rows[rows.length - 1]!.length > 0) { rows.push([]); x = 10; }
    rows[rows.length - 1]!.push(cell);
    x += cell.widthMm + 6;
  }
  const grouped: Cell[][][] = [];
  let used = 24;
  for (const row of rows) {
    const h = Math.max(...row.map((c) => c.above), 14) + Math.max(...row.map((c) => c.below), 0) + 11 + 24;
    if (grouped.length === 0 || used + h > A3.h - 6) { grouped.push([]); used = 24; }
    grouped[grouped.length - 1]!.push(row);
    used += h;
  }
  grouped.forEach((g, i) => pages.push({ asset: id, page: pageOf(asset, g, i + 1, grouped.length) }));
  console.log(`[DEBUG] ${id} ${grouped.length} page(s)`);
}
for (const { asset, page } of pages) {
  const svgPath = join(out, `${values.name}-${asset}.svg`);
  writeSvg ? writeFileSync(svgPath, writeSvg(page)) : null;
  if (values.png) execFileSync("rsvg-convert", ["-d", "100", "-p", "100", "-f", "png", "-o", svgPath.replace(/\.svg$/, ".png"), svgPath]);
}
writeFileSync(join(out, `${values.name}.pdf`), writePdf(pages.map((p) => p.page)));
console.log(`wrote ${pages.length} pages to ${out}`);
