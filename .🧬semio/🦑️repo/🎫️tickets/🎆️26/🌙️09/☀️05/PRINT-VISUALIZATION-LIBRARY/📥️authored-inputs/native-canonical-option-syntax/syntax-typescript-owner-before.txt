/** 📚️ Catalogue kinds are inferred into the same figure as custom numerical layers. */
import { vizParseColor } from "../🎨theme/🟦️.ts";
import catalog from "../../../🖼️assets/🔣️viz-catalog.json";
import type { VizChartSpecification, VizOptionValue } from "../../📸️snapshot/📊️chart/🟦️.ts";

const KINDS = new Map(catalog.kinds.map((entry) => [entry.slug, entry]));
const escape = (value: string): string => value.replace(/[\\{}%#$&_^~]/g, (character) => ({ "\\": "\\textbackslash{}", "{": "\\{", "}": "\\}", "%": "\\%", "#": "\\#", "$": "\\$", "&": "\\&", "_": "\\_", "^": "\\textasciicircum{}", "~": "\\textasciitilde{}" })[character]!);
function token(value: string): string { if (!/^[A-Za-z0-9_.:/-]+$/.test(value)) throw new Error(`invalid grammar identifier ${JSON.stringify(value)}`); return value; }
type Paint = { readonly hex: string; readonly alpha: number; readonly name: string };
function color(value: string): Paint | undefined { try { const rgba = vizParseColor(value), hex = rgba.slice(0, 3).map(channel => Math.round(channel).toString(16).padStart(2, "0")).join("").toUpperCase(), alpha = rgba[3] / 255; return { hex, alpha, name: `semio-print-color-${hex}${alpha === 1 ? "" : `-A${String(alpha).replace(".", "p")}`}` }; } catch { return undefined; } }
function scalar(value: VizOptionValue | null | undefined): string { if (value === null || value === undefined) return ""; if (typeof value === "number" && !Number.isFinite(value)) throw new Error("grammar numbers must be finite"); return typeof value === "string" ? escape(value) : String(value); }
function paint(value: VizOptionValue | null | undefined): string { return typeof value === "string" ? color(value)?.name ?? scalar(value) : scalar(value); }
function options(value: Readonly<Record<string, VizOptionValue>> = {}): string { return Object.keys(value).sort().map((key) => `${token(key)}={${["fill","stroke","color","colors","palette","background","foreground"].includes(key)||key.endsWith("Color")?paint(value[key]):scalar(value[key])}}`).join(","); }

export function inferVizPresetTikz(spec: VizChartSpecification, layeredTikz: string, checkpoint:()=>void=()=>{}): string {
  checkpoint();
  const presets = spec.presets ?? [];
  if (presets.length === 0) return layeredTikz;
  if (spec.language !== "en" && spec.language !== "de") throw new Error("catalogue inference requires explicit language");
  const lines = ["\\begingroup", `\\ExplSyntaxOn\\tl_set:Nn\\l_semio_language_tl{${spec.language}}\\ExplSyntaxOff`, `\\begin{VizFigure}[width=${spec.width},height=${spec.height},title={${escape(spec.title?.[spec.language] ?? "")}}]`];
  const colors = new Map<string, Paint>(), stack: unknown[] = [spec];
  let visited = 0;
  while (stack.length) { if (++visited % 256 === 0) checkpoint(); const value = stack.pop(); if (typeof value === "string") { const paint = color(value); if (paint) colors.set(paint.name, paint); } else if (value && typeof value === "object") stack.push(...Object.values(value)); }
  for (const paint of [...colors.values()].sort((a, b) => a.name.localeCompare(b.name))) { lines.push(`\\definecolor{${paint.name}}{HTML}{${paint.hex}}`); if (paint.alpha !== 1) lines.push(`\\SemioVizPaintAlpha{${paint.name}}{${paint.alpha}}`); }
  const tables = new Set<string>();
  for (const table of spec.tables ?? []) {
    checkpoint();
    const name = token(table.name);
    if (tables.has(name)) throw new Error(`duplicate table ${name}`);
    tables.add(name);
    lines.push(`\\SemioVizTable{${name}}{${table.columns.map(token).join(",")}}`);
    for (const row of table.rows) {checkpoint();lines.push(`\\SemioVizRow{${name}}{${table.columns.map((column) => `{${scalar(row[column])}}`).join(",")}}`);}
  }
  const paintScales=new Set(spec.layers.flatMap(layer=>[layer.encodings?.fill?.scale,layer.encodings?.stroke?.scale]).filter((name):name is string=>name!==undefined));
  for (const scale of spec.scales ?? []) {
    checkpoint();
    const scaleOptions = Object.fromEntries(Object.entries(scale.options ?? {}).map(([key, value]) => [key, Array.isArray(value) ? value.join(",") : value])) as Readonly<Record<string, VizOptionValue>>;
    lines.push(`\\SemioVizScale{${token(scale.name)}}{${token(scale.kind)}}{${scale.domain.map(scalar).join(",")}}{${scale.range.map(paintScales.has(scale.name)?paint:scalar).join(",")}}[${options(scaleOptions)}]`);
  }
  if (spec.theme) {
    const name = spec.theme.name === "semio" || spec.theme.name === undefined ? "default" : token(spec.theme.name);
    lines.push(`\\SemioVizTheme{${name}}[appearance=${spec.theme.appearance ?? "light"}]`);
    if (spec.theme.palette) lines.push(`\\SemioVizThemeSet[colors={${spec.theme.palette.map(paint).join(",")}}]`);
  }
  for (const preset of presets) {
    checkpoint();
    const kind = token(preset.kind), entry = KINDS.get(kind);
    if (!entry) throw new Error(`unknown catalogue kind ${kind}`);
    const data = preset.data;
    if (data !== undefined && data !== "demo" && data !== entry.data && !tables.has(data)) throw new Error(`unknown preset table ${data}`);
    const settings = options(preset.options);
    lines.push(`\\SemioVizChart{${kind}}[${settings}${data===undefined?"":`${settings?",":""}data=${token(data)}`}]`);
  }
  const begin = layeredTikz.indexOf("\n");
  const end = layeredTikz.lastIndexOf("\\end{tikzpicture}");
  if (begin < 0 || end < 0) throw new Error("layer inference did not emit a complete TikZ picture");
  lines.push(layeredTikz.slice(begin + 1, end).trimEnd(), "\\end{VizFigure}", "\\endgroup");
  return `${lines.join("\n")}\n`;
}
