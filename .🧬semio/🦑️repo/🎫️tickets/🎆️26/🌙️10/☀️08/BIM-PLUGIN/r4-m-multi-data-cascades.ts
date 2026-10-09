#!/usr/bin/env bun
/**
 * 🌊️ Part B of `m-multi-data`: the four foundation delete leaves (`delete-site`, `delete-building`, `delete-storey`, `delete-wall`) now
 * cascade over every collection that has a create leaf (see `🌊️cascade`). This script retires the cases that expected a refusal for
 * kinds that cascade now, rewrites the boilerplate and fixtures of the leaves with their new cases and merges the new case mounts into
 * the artifact root. Idempotent; blessed `after`/`diff` files are never overwritten (bless with `BIM_BLESS=1 cargo test`).
 */
import { existsSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { house } from "./r3-m-multi-data-leaves.ts";
import { artifact, em, fixtures, JSONF, mutations, RS } from "./r3-f1-paths.ts";

type Label = { en: string; de: string };
const target = (entity: string, label: Label): Prop => ({ name: "id", rust: "String", schema: { type: "string" }, ui: { widget: "reference", role: "target", label, ref: { kind: entity }, group: "target", order: 10 } });
const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const one = "vec![self.id.clone()]";

const windowTypes = { "win-1": { name: "Window", width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.07, frame_depth: 0.12, panes: 2, material: "m-brick" } };
const withWindow = (extra: Parameters<typeof F.scene>[0] = {}) => ({ ...F.scene({ ...extra, openings: { "o-1": F.opening("w-south", "Window") } }), window_types: windowTypes });
const siteOnly = () => F.snap({ sites: { "site-1": F.site("Plot") } });
const siteBuilding = () => F.snap({ sites: { "site-1": F.site("Plot") }, buildings: { "bldg-1": F.building("site-1", "House") } });
const E = { applied: 0x2705, cascade: 0x1f30a, whole: 0x1f3d7, empty: 0x1f9f2, pinned: 0x1f6ab, missing: 0x26d4 };

export const leaves: Leaf[] = [
  {
    kind: "delete-site", emoji: 0x1f9f9, variant: "DeleteSite", verb: "delete", entity: "site", binaryTag: 1, displayName: "Delete Site",
    doc: "Removes a site together with its buildings and everything inside them (storeys, grid lines, walls, curtain walls, columns, beams, slabs, roofs, stairs, railings, spaces, openings) and their properties and classifications; refuses while a surviving element still constrains its top to a removed storey.",
    props: [target("site", { en: "Site", de: "Standort" })],
    inverseRows: { bounded: 8191 },
    label: { en: 'format!("Delete site \\"{}\\" with everything on it", self.id)', de: 'format!("Standort \\"{}\\" samt Inhalt löschen", self.id)' },
    target: one,
    cases: [
      { name: "removes", emoji: E.applied, before: siteOnly(), mutation: { id: "site-1" }, outcome: ok },
      { name: "cascades-its-buildings", emoji: E.cascade, before: F.scene(), mutation: { id: "site-1" }, outcome: ok },
      { name: "cascades-the-whole-site", emoji: E.whole, before: house(), mutation: { id: "site-1" }, outcome: ok },
      { name: "missing", emoji: E.missing, before: siteBuilding(), mutation: { id: "site-2" }, outcome: reject("mutation.target-missing", ["site-2"]) },
    ],
  },
  {
    kind: "delete-building", emoji: 0x1f3da, variant: "DeleteBuilding", verb: "delete", entity: "building", binaryTag: 3, displayName: "Delete Building",
    doc: "Removes a building together with its storeys, grid lines and everything on them (walls, curtain walls, columns, beams, slabs, roofs, stairs, railings, spaces, openings) and their properties and classifications; refuses while a surviving element still constrains its top to a removed storey.",
    props: [target("building", { en: "Building", de: "Gebäude" })],
    inverseRows: { bounded: 8191 },
    label: { en: 'format!("Delete building \\"{}\\" with everything in it", self.id)', de: 'format!("Gebäude \\"{}\\" samt Inhalt löschen", self.id)' },
    target: one,
    cases: [
      { name: "removes", emoji: E.applied, before: siteBuilding(), mutation: { id: "bldg-1" }, outcome: ok },
      { name: "cascades-its-storeys", emoji: E.cascade, before: F.scene(), mutation: { id: "bldg-1" }, outcome: ok },
      { name: "cascades-the-whole-building", emoji: E.whole, before: house(), mutation: { id: "bldg-1" }, outcome: ok },
      { name: "missing", emoji: E.missing, before: siteBuilding(), mutation: { id: "bldg-2" }, outcome: reject("mutation.target-missing", ["bldg-2"]) },
    ],
  },
  {
    kind: "delete-storey", emoji: 0x1f6ae, variant: "DeleteStorey", verb: "delete", entity: "storey", binaryTag: 8, displayName: "Delete Storey",
    doc: "Removes a storey together with everything on it (walls, curtain walls, columns, beams, slabs, roofs, stairs, railings, spaces and the openings of its walls) and their properties and classifications; refuses while a surviving element still constrains its top to the storey.",
    props: [target("storey", { en: "Storey", de: "Geschoss" })],
    inverseRows: { bounded: 8191 },
    label: { en: 'format!("Delete storey \\"{}\\" with everything on it", self.id)', de: 'format!("Geschoss \\"{}\\" samt Inhalt löschen", self.id)' },
    target: one,
    cases: [
      { name: "cascades-the-walls", emoji: E.applied, before: F.scene(), mutation: { id: "st-ground" }, outcome: ok },
      { name: "empty-storey", emoji: E.empty, before: F.scene(), mutation: { id: "st-first" }, outcome: ok },
      { name: "cascades-the-opening", emoji: E.cascade, before: withWindow(), mutation: { id: "st-ground" }, outcome: ok },
      { name: "cascades-everything-on-it", emoji: E.whole, before: house(), mutation: { id: "st-ground" }, outcome: ok },
      { name: "constrains-another-wall", emoji: E.pinned, before: F.scene({ walls: { "w-first": F.wall("st-first", "wt-300", F.line([0, 0], [8, 0]), F.toStorey("st-ground", 0.2), "First") } }), mutation: { id: "st-ground" }, outcome: reject("mutation.target-referenced", ["st-ground"]) },
      { name: "missing", emoji: E.missing, before: F.scene(), mutation: { id: "st-attic" }, outcome: reject("mutation.target-missing", ["st-attic"]) },
    ],
  },
  {
    kind: "delete-wall", emoji: 0x1f4a5, variant: "DeleteWall", verb: "delete", entity: "wall", binaryTag: 10, displayName: "Delete Wall",
    doc: "Removes a wall together with the openings it hosts and the properties and classifications of both.",
    props: [target("wall", { en: "Wall", de: "Wand" })],
    inverseRows: { bounded: 4096 },
    label: { en: 'format!("Delete wall \\"{}\\" with its openings", self.id)', de: 'format!("Wand \\"{}\\" samt Öffnungen löschen", self.id)' },
    target: one,
    cases: [
      { name: "removes", emoji: E.applied, before: F.scene(), mutation: { id: "w-east" }, outcome: ok },
      { name: "cascades-the-opening", emoji: E.cascade, before: withWindow(), mutation: { id: "w-south" }, outcome: ok },
      { name: "cascades-the-opening-and-data", emoji: E.whole, before: house(), mutation: { id: "w-south" }, outcome: ok },
      { name: "missing", emoji: E.missing, before: F.scene(), mutation: { id: "w-missing" }, outcome: reject("mutation.target-missing", ["w-missing"]) },
    ],
  },
];

const retired: Record<string, string[]> = { "delete-site": ["has-buildings"], "delete-building": ["has-storeys"], "delete-storey": ["hosts-an-opening"], "delete-wall": ["hosts-an-opening"] };
const byName = (parent: string, suffix: string) => readdirSync(parent).find((name) => name.endsWith(suffix) && name.length > suffix.length && !name.includes("."));

const retire = (source: string) => {
  let text = source;
  for (const [kind, names] of Object.entries(retired)) {
    const leafName = byName(mutations, kind)!;
    for (const name of names) {
      const tests = join(mutations, leafName, em(0x1f9ea) + "tests");
      const testDir = byName(tests, name);
      text = text.replace(new RegExp(`\\n *#\\[cfg\\(test\\)\\]\\n *#\\[path = "[^"\\n]*${leafName}/${em(0x1f9ea)}tests/[^/\\n"]*${name}/${RS}"\\]\\n *mod tests_[a-z_]+;`), "");
      if (testDir) rmSync(join(tests, testDir), { recursive: true });
      const fixtureLeaf = join(fixtures, em(0x1f9ec) + "mutations", leafName);
      const fixtureDir = existsSync(fixtureLeaf) ? byName(fixtureLeaf, name) : undefined;
      if (fixtureDir) rmSync(join(fixtureLeaf, fixtureDir), { recursive: true });
    }
  }
  return text;
};

const merge = (source: string, kind: string, block: string): [string, number] => {
  const start = source.indexOf(`pub mod ${kind.replaceAll("-", "_")} {`);
  const end = source.indexOf("\n" + " ".repeat(24) + "}\n", start);
  if (start < 0 || end < 0) throw new Error(`mount block of ${kind} not found`);
  const section = source.slice(start, end);
  const lines = block.split("\n");
  const added: string[] = [];
  lines.forEach((line, index) => {
    const mounted = line.match(/^ *mod (tests_[a-z_]+);$/);
    if (mounted && !section.includes(`mod ${mounted[1]};`)) added.push(lines[index - 2], lines[index - 1], line);
  });
  return [added.length ? source.slice(0, end + 1) + added.join("\n") + "\n" + source.slice(end + 1) : source, added.length / 3];
};

if (import.meta.main) {
  const root = join(artifact, RS);
  const raw = readFileSync(root, "utf8");
  const crlf = raw.includes("\r\n");
  let source = retire(raw.replaceAll("\r\n", "\n"));
  for (const leaf of leaves) {
    const block = emitLeaf(leaf);
    const [next, count] = merge(source, leaf.kind, block);
    source = next;
    console.log(`${leaf.kind}: ${count} case mounts added`);
  }
  writeFileSync(root, crlf ? source.replaceAll("\n", "\r\n") : source);
}
void JSONF;
