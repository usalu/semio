/** 🧰️ Shared code of the s4 volume scripts: the staffage modules read from the worktree (never written), the short constructors the handcrafted volumes are written with, and the idempotent splice of one `asset.volumes[scale]` value into the text of an asset document that leaves every other byte as it is.
 *
 * @see ./prove.ts — the proof of a script on a copy outside the catalog
 * @see ./dump.ts — the authoring aid that lists a finer volume */
import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

export const S = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
export const mod = <T = any>(relative: string): Promise<T> => import(join(S, relative));

/** 🔎️ The catalog file of an asset id. */
export function assetFile(id: string): string {
  const catalog = join(S, "🗂️catalog");
  for (const group of readdirSync(catalog, { withFileTypes: true })) if (group.isDirectory() && existsSync(join(catalog, group.name, id, "🔣️.json"))) return join(catalog, group.name, id, "🔣️.json");
  throw new Error(`no asset ${id}`);
}

export type Solid = Record<string, unknown>;
export type Volume = { readonly status: "authored"; readonly denominator: number; readonly decisions: readonly { readonly en: string; readonly de: string }[]; readonly body: readonly Solid[]; readonly thermal: readonly Solid[]; readonly structure: readonly Solid[] };
export type Script = { readonly id: string; readonly volumes: Readonly<Record<string, Volume>> };
type V3 = readonly [number, number, number];
type V2 = readonly [number, number];
type Head = readonly [id: string, bone: string, material: string, tone: number, role?: string];
const head = ([id, bone, material, tone, role = "body"]: Head, kind: string): Solid => ({ id, kind, bone, material, role, tone });

/** 📦️ A box centred at `at` with edges along u, v, w. */
export const box = (h: Head, size: V3, at: V3, extra: Solid = {}): Solid => ({ ...head(h, "box"), size, ...(at.every((value) => value === 0) ? {} : { transform: { translation: at } }), ...extra });
/** 🧇️ A polygon on `plane` (xy: u, v along w; yz: v, w along u; zx: w, u along v) pushed from `from` to `to`. */
export const ext = (h: Head, plane: "xy" | "yz" | "zx", profile: readonly V2[], from: number, to: number, extra: Solid = {}): Solid => ({ ...head(h, "extrude"), profile, plane, from, to, ...extra });
/** 🏺️ An [axial, radial] profile about `axis` through `origin`; a profile that starts and ends on the axis is a filled body. */
export const rev = (h: Head, axis: V3, origin: V3, profile: readonly V2[], extra: Solid = {}): Solid => ({ ...head(h, "revolve"), profile, axis, origin, ...extra });
/** 💊️ The hull of two spheres. */
export const cap = (h: Head, p0: V3, p1: V3, r0: number, r1: number, extra: Solid = {}): Solid => ({ ...head(h, "capsule"), p0, p1, r0, r1, ...extra });
/** 🥚️ An ellipsoid centred at `at` with semi-axes along u, v, w. */
export const ell = (h: Head, radii: V3, at: V3, extra: Solid = {}): Solid => ({ ...head(h, "ellipsoid"), radii, ...(at.every((value) => value === 0) ? {} : { transform: { translation: at } }), ...extra });
/** 🔗️ A thermal or structural reference to a body solid of the same scale. */
export const of = (id: string, body: string, binding: Solid): Solid => ({ id, of: body, ...binding });
/** 🔲️ The rectangle [a0, a1] x [b0, b1] as a counter-clockwise profile. */
export const rect = (a0: number, a1: number, b0: number, b1: number): readonly V2[] => [[a0, b0], [a1, b0], [a1, b1], [a0, b1]];

const same = (a: unknown, b: unknown): boolean => JSON.stringify(a) === JSON.stringify(b);

/** 🧭️ The span [start, end) of the value of `path` in a JSON text, by a scanner that reads strings and nesting and nothing else; `null` when the path does not exist. */
export function spanOf(text: string, path: readonly string[]): readonly [number, number] | null {
  let i = 0;
  const space = (): void => { while (/\s/u.test(text[i] ?? "x")) i++; };
  const string = (): string => {
    const start = i++;
    while (text[i] !== '"') i += text[i] === "\\" ? 2 : 1;
    return JSON.parse(text.slice(start, ++i)) as string;
  };
  const skip = (): void => {
    space();
    if (text[i] === '"') return void string();
    if (text[i] === "{" || text[i] === "[") {
      for (let depth = 0; ; ) {
        if (text[i] === '"') { string(); continue; }
        if (text[i] === "{" || text[i] === "[") depth++;
        if (text[i] === "}" || text[i] === "]") depth--;
        i++;
        if (depth === 0) return;
      }
    }
    while (!/[,\]}\s]/u.test(text[i] ?? ",")) i++;
  };
  for (const key of path) {
    space();
    if (text[i] !== "{") return null;
    i++;
    for (let found = false; !found; ) {
      space();
      if (text[i] === "}") return null;
      if (text[i] === ",") { i++; continue; }
      const name = string();
      space();
      i++;
      space();
      if (name === key) found = true;
      else skip();
    }
  }
  const start = i;
  skip();
  return [start, i];
}

/** 🪡️ The text of an asset document with the volumes of `script` written in: `unchanged` when every scale already holds exactly that volume, `written` when a scale was `unavailable` (or `force` replaces an authored one), and a refusal by name when the document is another asset, lacks the scale, lacks a bone, material, region, layer or anchor the volume names, or holds another authored volume. Only the bytes of the replaced values change. */
export function applyTo(text: string, script: Script, force = false): { readonly text: string; readonly result: "written" | "unchanged" } {
  let out = text;
  let result: "written" | "unchanged" = "unchanged";
  const asset = (JSON.parse(text) as { asset: any }).asset;
  if (asset.id !== script.id) throw new Error(`refused: the document is ${asset.id}, the script is for ${script.id}`);
  const bones = new Set<string>(asset.rig.bones.map((bone: any) => bone.id));
  const materials = new Set<string>(asset.materials.map((material: any) => material.id));
  const regions = new Map<string, Set<string>>(asset.thermal.regions.map((region: any) => [region.id, new Set(region.layers.map((layer: any) => layer.id))]));
  const anchors = new Set<string>(asset.anchors.map((anchor: any) => anchor.id));
  for (const [scale, volume] of Object.entries(script.volumes)) {
    const lacks: string[] = [];
    for (const solid of [...volume.body, ...volume.thermal, ...volume.structure] as any[]) {
      if (solid.bone !== undefined && !bones.has(solid.bone)) lacks.push(`bone ${solid.bone}`);
      if (solid.material !== undefined && !materials.has(solid.material)) lacks.push(`material ${solid.material}`);
      if (solid.member !== undefined && !bones.has(solid.member)) lacks.push(`bone ${solid.member}`);
      for (const bone of solid.members ?? []) if (!bones.has(bone)) lacks.push(`bone ${bone}`);
      if (solid.region !== undefined && !regions.has(solid.region)) lacks.push(`region ${solid.region}`);
      else if (solid.layer !== undefined && !regions.get(solid.region)!.has(solid.layer)) lacks.push(`layer ${solid.region}/${solid.layer}`);
      if (solid.anchor !== undefined && !anchors.has(solid.anchor)) lacks.push(`anchor ${solid.anchor}`);
    }
    if (lacks.length > 0) throw new Error(`refused: ${script.id} 1:${scale} names what the asset no longer has: ${[...new Set(lacks)].join(", ")}`);
    const span = spanOf(out, ["asset", "volumes", scale]);
    if (span === null) throw new Error(`refused: ${script.id} has no volumes["${scale}"]`);
    const current = JSON.parse(out.slice(span[0], span[1]));
    if (same(current, volume)) continue;
    if (current.status !== "unavailable" && !force) throw new Error(`refused: ${script.id} volumes["${scale}"] is ${current.status} with other content; pass --force to replace it`);
    const lineStart = out.lastIndexOf("\n", span[0]) + 1;
    const indent = /^\s*/u.exec(out.slice(lineStart, span[0]))![0];
    out = out.slice(0, span[0]) + JSON.stringify(volume, null, 1).split("\n").join(`\n${indent}`) + out.slice(span[1]);
    result = "written";
  }
  return { text: out, result };
}

/** 🚀️ The command line of an asset script: `<script> <asset document> [--force]` writes the volumes into that file and prints `written` or `unchanged`. */
export function main(script: Script, argv: readonly string[] = process.argv.slice(2)): void {
  const file = argv.find((arg) => !arg.startsWith("--"));
  if (file === undefined) throw new Error(`usage: ${script.id}.ts <asset document> [--force]`);
  const applied = applyTo(readFileSync(file, "utf8"), script, argv.includes("--force"));
  if (applied.result === "written") writeFileSync(file, applied.text);
  console.log(`${script.id} ${Object.keys(script.volumes).map((scale) => `1:${scale}`).join(" ")} ${applied.result} ${file}`);
}
