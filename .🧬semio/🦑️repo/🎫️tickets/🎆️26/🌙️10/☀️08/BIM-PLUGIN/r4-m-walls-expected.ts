#!/usr/bin/env bun
/**
 * 🔬️ Second implementation of the `m-walls` mutation semantics (TypeScript, no Rust): computes the expected sparse diff and after snapshot of
 * every applied fixture case from its `before` snapshot and `mutation` alone. `bun r4-m-walls-expected.ts` compares them (within 1e-9)
 * with the committed Rust-blessed fixtures; `--fill` writes them into fixtures that are still the `{}` placeholder of `emitLeaf`.
 */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { em, fixtures, JSONF } from "./r3-f1-paths.ts";

type J = any;
const KINDS = ["set-wall-axis", "set-wall-base-offset", "set-wall-type-of", "set-wall-location", "flip-wall", "split-wall", "create-curtain-wall", "delete-curtain-wall", "set-curtain-wall"];
const snap = (value: number) => Math.round(value * 1e9) / 1e9 + 0;
const snapBulge = (value: number) => Math.round(value * 1e12) / 1e12 + 0;

const parts = (axis: J) => {
  const [kind] = Object.keys(axis);
  return { kind, ...axis[kind] } as { kind: "Line" | "Arc"; start: J; end: J; bulge?: number };
};
const build = (kind: string, start: J, end: J, bulge?: number) => (kind === "Line" ? { Line: { start, end } } : { Arc: { start, end, bulge } });
const flipped = (axis: J) => {
  const a = parts(axis);
  return build(a.kind, a.end, a.start, a.kind === "Arc" ? 0 - a.bulge! : undefined);
};
const lengthOf = (axis: J) => {
  const a = parts(axis);
  const chord = Math.hypot(a.end.x - a.start.x, a.end.y - a.start.y);
  const sweep = a.kind === "Arc" ? 4 * Math.abs(Math.atan(a.bulge!)) : 0;
  return sweep < 1e-12 ? chord : (sweep * chord) / (2 * Math.sin(sweep / 2));
};
const split = (axis: J, t: number) => {
  const a = parts(axis);
  let middle: J;
  if (a.kind === "Line") middle = { x: a.start.x + t * (a.end.x - a.start.x), y: a.start.y + t * (a.end.y - a.start.y) };
  else {
    const sweep = 4 * Math.atan(a.bulge!);
    const [dx, dy] = [a.end.x - a.start.x, a.end.y - a.start.y];
    const chord = Math.hypot(dx, dy);
    const radius = chord / (2 * Math.abs(Math.sin(sweep / 2)));
    const lift = chord / 2 / Math.tan(sweep / 2);
    const centre = { x: (a.start.x + a.end.x) / 2 - (lift * dy) / chord, y: (a.start.y + a.end.y) / 2 + (lift * dx) / chord };
    const angle = Math.atan2(a.start.y - centre.y, a.start.x - centre.x) + t * sweep;
    middle = { x: centre.x + radius * Math.cos(angle), y: centre.y + radius * Math.sin(angle) };
  }
  middle = { x: snap(middle.x), y: snap(middle.y) };
  const share = (fraction: number) => (a.kind === "Arc" ? snapBulge(Math.tan(fraction * Math.atan(a.bulge!))) : undefined);
  return { first: build(a.kind, a.start, middle, share(t)), second: build(a.kind, middle, a.end, share(1 - t)), at: snap(t * lengthOf(axis)) };
};

const differs = (left: J, right: J) => JSON.stringify(left) !== JSON.stringify(right);
const sparse = (record: J, named: J, fields: string[]) => Object.fromEntries(fields.filter((f) => named[f] !== undefined && differs(named[f], record[f])).map((f) => [f, named[f]]));
const patched = (collection: string, id: string, fields: J) => ({ [collection]: { [id]: { entry: "Patched", ...fields } } });

export function expectedDiff(kind: string, before: J, m: J): J {
  switch (kind) {
    case "set-wall-axis": return patched("walls", m.id, { axis: m.axis });
    case "set-wall-base-offset": return patched("walls", m.id, { base_offset: m.base_offset });
    case "set-wall-type-of": return patched("walls", m.id, { wall_type: m.wall_type });
    case "set-wall-location": return patched("walls", m.id, { location: m.location });
    case "flip-wall": return patched("walls", m.id, { axis: flipped(before.walls[m.id].axis) });
    case "split-wall": {
      const wall = before.walls[m.id];
      const { first, second, at } = split(wall.axis, m.t);
      const moved = Object.entries<J>(before.openings).filter(([, o]) => o.host === m.id && o.offset >= at);
      return {
        walls: { [m.id]: { entry: "Patched", axis: first }, [m.new_id]: { entry: "Created", ...wall, axis: second } },
        ...(moved.length ? { openings: Object.fromEntries(moved.map(([id, o]) => [id, { entry: "Patched", host: m.new_id, offset: snap(o.offset - at) }])) } : {}),
      };
    }
    case "create-curtain-wall": return { curtain_walls: { [m.id]: { entry: "Created", ...m.curtain_wall } } };
    case "delete-curtain-wall": return { curtain_walls: { [m.id]: { entry: "Deleted" } } };
    case "set-curtain-wall": return patched("curtain_walls", m.id, sparse(before.curtain_walls[m.id], m, ["axis", "base_offset", "top", "u_spacing", "v_spacing", "mullion", "panel_material", "mullion_material", "name"]));
  }
  throw new Error(`unknown kind ${kind}`);
}

export function applyDiff(before: J, diff: J): J {
  const next = structuredClone(before);
  for (const [collection, entries] of Object.entries<J>(diff)) {
    for (const [id, { entry, ...fields }] of Object.entries<J>(entries)) {
      if (entry === "Created") next[collection][id] = fields;
      else if (entry === "Deleted") delete next[collection][id];
      else Object.assign(next[collection][id], fields);
    }
  }
  return next;
}

const same = (left: J, right: J, path = ""): string[] => {
  if (typeof left === "number" && typeof right === "number") return Math.abs(left - right) <= 1e-9 * Math.max(1, Math.abs(right)) ? [] : [`${path}: ${left} != ${right}`];
  if (left === null || right === null || typeof left !== "object" || typeof right !== "object") return left === right ? [] : [`${path}: ${JSON.stringify(left)} != ${JSON.stringify(right)}`];
  const keys = new Set([...Object.keys(left), ...Object.keys(right)]);
  return [...keys].flatMap((key) => (key in left && key in right ? same(left[key], right[key], `${path}/${key}`) : [`${path}/${key}: only ${key in left ? "expected" : "committed"}`]));
};

if (import.meta.main) {
  const root = join(fixtures, em(0x1f9ec) + "mutations");
  const fill = process.argv.includes("--fill");
  let checked = 0;
  let problems = 0;
  for (const dir of readdirSync(root)) {
    const kind = dir.replace(/^[^a-z]+/, "");
    if (!KINDS.includes(kind)) continue;
    for (const name of readdirSync(join(root, dir))) {
      const read = (...segments: string[]) => JSON.parse(readFileSync(join(root, dir, name, ...segments, JSONF), "utf8"));
      if (read(em(0x1f3af) + "outcome").status !== "applied") continue;
      const before = read(em(0x1f4f8) + "snapshot", em(0x2b05) + "before");
      const { mutation: _tag, ...m } = read(em(0x1f9a0) + "mutation");
      const diff = expectedDiff(kind, before, m);
      const after = applyDiff(before, diff);
      for (const [segments, expected] of [[[em(0x1f53a) + "diff"], diff], [[em(0x1f4f8) + "snapshot", em(0x27a1) + "after"], after]] as const) {
        const file = join(root, dir, name, ...segments, JSONF);
        const committed = JSON.parse(readFileSync(file, "utf8"));
        if (Object.keys(committed).length === 0) {
          if (fill) writeFileSync(file, JSON.stringify(expected, null, 2) + "\n");
          else { console.log(`[PLACEHOLDER] ${kind}/${name} ${segments.at(-1)}`); problems += 1; }
          continue;
        }
        const diffs = same(expected, committed);
        checked += 1;
        if (diffs.length) { problems += 1; console.log(`[MISMATCH] ${kind}/${name} ${segments.at(-1)}\n  ${diffs.slice(0, 5).join("\n  ")}`); }
      }
    }
  }
  console.log(problems === 0 ? `all ${checked} committed documents agree with the TypeScript second implementation` : `${problems} problem(s), ${checked} documents compared`);
}
