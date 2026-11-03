import { readFileSync, writeFileSync } from "node:fs";

export type Pt = readonly [number, number];
export type Text = { en: string; de: string };
export type Role = "body" | "detail" | "texture" | "glazing" | "ground";

export type Common = { id: string; bone: string; role: Role; z: number; filled: boolean; tone: number; region?: string; regions?: string[]; material?: string; penMm?: number };
export type Shape = Common & Record<string, unknown>;

const r4 = (v: number): number => Math.round(v * 10000) / 10000;
const clean = (shape: Shape): Shape => JSON.parse(JSON.stringify(shape, (_, v) => (typeof v === "number" ? r4(v) : v)));

export type Opts = { region?: string; regions?: string[]; material?: string; penMm?: number; filled?: boolean };

export function make(kind: Record<string, unknown>, id: string, bone: string, role: Role, z: number, tone: number, o: Opts = {}): Shape {
  const { filled = true, ...rest } = o;
  return clean({ ...kind, id, bone, role, z, filled, tone, ...rest } as Shape);
}

export const capsule = (id: string, bone: string, role: Role, z: number, tone: number, x0: number, y0: number, x1: number, y1: number, r0: number, r1: number, o: Opts & { followTip?: boolean } = {}): Shape => {
  const { followTip, ...rest } = o;
  return make({ kind: "capsule", x0, y0, x1, y1, r0, r1, ...(followTip ? { followTip: true } : {}) }, id, bone, role, z, tone, rest);
};
export const ellipse = (id: string, bone: string, role: Role, z: number, tone: number, cx: number, cy: number, rx: number, ry: number, o: Opts & { rotationDeg?: number } = {}): Shape => {
  const { rotationDeg, ...rest } = o;
  return make({ kind: "ellipse", cx, cy, rx, ry, ...(rotationDeg ? { rotationDeg } : {}) }, id, bone, role, z, tone, rest);
};
export const rect = (id: string, bone: string, role: Role, z: number, tone: number, x: number, y: number, w: number, h: number, r: number, o: Opts = {}): Shape => make({ kind: "rect", x, y, w, h, ...(r > 0 ? { r } : {}) }, id, bone, role, z, tone, o);
export const line = (id: string, bone: string, role: Role, z: number, tone: number, points: readonly Pt[], o: Opts = {}): Shape => make({ kind: "poly", points: points.map((p) => [r4(p[0]), r4(p[1])]), closed: false }, id, bone, role, z, tone, { filled: false, ...o });
export const poly = (id: string, bone: string, role: Role, z: number, tone: number, points: readonly Pt[], o: Opts = {}): Shape => make({ kind: "poly", points: points.map((p) => [r4(p[0]), r4(p[1])]), closed: true }, id, bone, role, z, tone, o);

const f = (v: number): string => String(r4(v));

export function smoothClosed(points: readonly Pt[], tension = 0.5): string {
  const n = points.length;
  const at = (i: number): Pt => points[((i % n) + n) % n]!;
  let d = `M ${f(at(0)[0])} ${f(at(0)[1])}`;
  for (let i = 0; i < n; i++) {
    const p0 = at(i - 1);
    const p1 = at(i);
    const p2 = at(i + 1);
    const p3 = at(i + 2);
    const t = tension / 3;
    d += ` C ${f(p1[0] + (p2[0] - p0[0]) * t)} ${f(p1[1] + (p2[1] - p0[1]) * t)} ${f(p2[0] - (p3[0] - p1[0]) * t)} ${f(p2[1] - (p3[1] - p1[1]) * t)} ${f(p2[0])} ${f(p2[1])}`;
  }
  return `${d} Z`;
}

export const pathShape = (id: string, bone: string, role: Role, z: number, tone: number, d: string, o: Opts = {}): Shape => make({ kind: "path", d }, id, bone, role, z, tone, o);

export const mirrorY = (points: readonly Pt[]): Pt[] => points.map(([x, y]) => [x, -y] as const);

/** Closed outline from the left half (y >= 0) listed front to back, mirrored for the right half. */
export function symmetricOutline(left: readonly Pt[]): Pt[] {
  const right = mirrorY(left.slice(1, -1)).reverse();
  return [...left, ...right];
}

export type Pens = Record<Role, number | null>;

export type Rep = { status: string; denominator: number; pens: Pens; geometry: { elevation: Shape[]; plan?: Shape[] } };

/** Index just past the value starting at `start` (object, array or string aware). */
export function endOfValue(text: string, start: number): number {
  const open = text[start];
  if (open !== "{" && open !== "[") throw new Error(`no container at ${start}`);
  let depth = 0;
  let inString = false;
  for (let i = start; i < text.length; i++) {
    const c = text[i]!;
    if (inString) {
      if (c === "\\") i++;
      else if (c === '"') inString = false;
      continue;
    }
    if (c === '"') inString = true;
    else if (c === "{" || c === "[") depth++;
    else if (c === "}" || c === "]") {
      depth--;
      if (depth === 0) return i + 1;
    }
  }
  throw new Error("unbalanced");
}

/** Inserts (or replaces) `"plan"` after the `"elevation"` array of the representation `scale` by text edit, so every other byte of the asset file is preserved. */
export function insertPlan(path: string, plans: Record<string, Shape[]>, viewpoints: boolean): void {
  let text = readFileSync(path, "utf8");
  const repStart = text.indexOf('"representations": {');
  const objStart = text.indexOf("{", repStart + '"representations":'.length);
  for (const scale of ["500", "200", "100", "50"]) {
    const repsEnd = endOfValue(text, objStart);
    const reps = text.slice(objStart, repsEnd);
    const keyAt = reps.indexOf(`"${scale}": {`);
    if (keyAt < 0) throw new Error(`representation ${scale} not found in ${path}`);
    const repOpen = objStart + keyAt + `"${scale}": `.length;
    const repClose = endOfValue(text, repOpen);
    const rep = text.slice(repOpen, repClose);
    const geoAt = rep.indexOf('"geometry": {');
    const geoOpen = repOpen + geoAt + '"geometry": '.length;
    const geoClose = endOfValue(text, geoOpen);
    const geo = text.slice(geoOpen, geoClose);
    const elevKey = geo.indexOf('"elevation": [');
    const elevOpen = geoOpen + elevKey + '"elevation": '.length;
    const elevClose = endOfValue(text, elevOpen);
    const lineStart = text.lastIndexOf("\n", geoOpen + elevKey) + 1;
    const indent = text.slice(lineStart, geoOpen + elevKey).match(/^\s*/u)![0];
    const body = JSON.stringify(plans[scale], null, 2)
      .split("\n")
      .map((l, i) => (i === 0 ? l : indent + l))
      .join("\n");
    const planKey = geo.indexOf('"plan": [');
    if (planKey >= 0) {
      const planOpen = geoOpen + planKey + '"plan": '.length;
      const planClose = endOfValue(text, planOpen);
      text = text.slice(0, planOpen) + body + text.slice(planClose);
    } else {
      text = text.slice(0, elevClose) + `,\n${indent}"plan": ${body}` + text.slice(elevClose);
    }
  }
  if (viewpoints) text = text.replace(/("viewpoints": \[\s*"elevation")(\s*\])/u, (_, a: string, b: string) => `${a},${b.includes("\n") ? `\n${/\n(\s*)\]/u.exec(b)![1]}  ` : " "}"plan"${b}`);
  writeFileSync(path, text);
}

function endOfString(text: string, openQuote: number): number {
  for (let i = openQuote + 1; i < text.length; i++) {
    if (text[i] === "\\") i++;
    else if (text[i] === '"') return i;
  }
  throw new Error("unterminated string");
}

/** Appends a sentence to the `en` and `de` text of a decision of one representation by text edit (idempotent through the marker `plan:` / `Grundriss:`). */
export function appendDecision(path: string, scale: string, field: string, en: string, de: string): void {
  let text = readFileSync(path, "utf8");
  const repsAt = text.indexOf('"representations": {');
  const keyAt = text.indexOf(`"${scale}": {`, repsAt);
  const repOpen = keyAt + `"${scale}": `.length;
  const repClose = endOfValue(text, repOpen);
  const decisionsAt = text.indexOf('"decisions": {', repOpen);
  if (decisionsAt < 0 || decisionsAt > repClose) throw new Error(`no decisions in ${scale}`);
  const fieldAt = text.indexOf(`"${field}": {`, decisionsAt);
  for (const [lang, sentence, marker] of [["de", de, "Grundriss:"], ["en", en, "Plan:"]] as const) {
    const langAt = text.indexOf(`"${lang}": "`, fieldAt);
    const open = text.indexOf('"', langAt + `"${lang}":`.length);
    const close = endOfString(text, open);
    const current = text.slice(open + 1, close);
    if (current.includes(marker)) continue;
    const add = ` ${sentence}`.replaceAll("\\", "\\\\").replaceAll('"', '\\"');
    text = text.slice(0, close) + add + text.slice(close);
  }
  writeFileSync(path, text);
}
