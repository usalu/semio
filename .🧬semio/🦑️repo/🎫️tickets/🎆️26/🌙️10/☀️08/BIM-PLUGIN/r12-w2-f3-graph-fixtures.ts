#!/usr/bin/env bun
/**
 * 🪑️ Wave W2 `w2-f3-graph`: writes the committed snapshot of the component and MEP inference corpus under `🧫️fixtures/💡️inferences/🪑️components/🏠️room/📸️snapshot/`: a 6 x 4 m room (four 20 cm walls) on a 3 m storey with
 * parametric families (table, chair, wash basin, lamp, diffuser, a sunk niche, a profile), components of every kind of placement (free-standing, rotated, mirrored, overridden, hosted on either side of a wall, a terminal, one
 * standing in a wall, one outside the storey, ones with a missing family, a profile family, a missing host and a faulty override) and MEP runs (a supply duct, a water pipe that clashes with it, a waste riser, a
 * lighting tray that ends at the lamp, a power tray that touches the duct, a degenerate pipe). The python oracle recomputes everything from this snapshot; `💡️inference/` holds what the oracle writes, never by hand.
 * `bun r12-w2-f3-graph-fixtures.ts` rewrites the snapshot.
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import * as F from "./r3-f1-fixtures.ts";
import { em, fixtures, JSONF } from "./r3-f1-paths.ts";

const zero = { x: "0 m", y: "0 m", z: "0 m" };
const cuboid = (x: string, y: string, z: string, width: string, depth: string, height: string) => ({ Cuboid: { x, y, z, width, depth, height } });
const family = (name: string, category: string) => ({ name, category });
const parameter = (owner: string, name: string, kind: string, value: string) => [`${owner}.${name}`, { family: owner, name, kind, value }] as const;
const solid = (owner: string, name: string, shape: unknown, material = "\"m-oak\"") => ({ family: owner, name, shape, material, visible: "true", offset: zero });
const override = (component: string, name: string, value: string) => [`${component}.${name}`, { component, name, value }] as const;
const rows = (...entries: (readonly [string, unknown])[]) => Object.fromEntries(entries);
const oak = { name: "Oak", category: "Wood", color: { r: 0.6, g: 0.45, b: 0.3 }, density: 700, conductivity: 0.2, specific_heat: 1600 };
const steel = { name: "Steel", category: "Metal", color: { r: 0.6, g: 0.6, b: 0.62 }, density: 7850, conductivity: 50, specific_heat: 500 };

const component = (family: string, x: number, y: number, extra: Record<string, unknown> = {}) => ({ storey: "st-ground", family, position: F.P(x, y), elevation: 0, rotation: 0, mirrored: false, name: family, ...extra });
const mep = (system: string, shape: unknown, path: [number, number, number][], name: string) => ({ storey: "st-ground", system, shape, path: path.map(([x, y, z]) => ({ x, y, z })), name });
const duct = (width: number, height: number) => ({ Duct: { width, height } });
const pipe = (diameter: number) => ({ Pipe: { diameter } });
const tray = (width: number, height: number) => ({ Tray: { width, height } });

const families = {
  "fam-table": family("Table", "Furniture"),
  "fam-chair": family("Chair", "Furniture"),
  "fam-basin": family("Wash basin", "Plumbing"),
  "fam-lamp": family("Lamp", "Lighting"),
  "fam-diffuser": family("Diffuser", "Mechanical"),
  "fam-niche": family("Niche unit", "Casework"),
  "fam-hea": family("HEA 200", "Profile"),
};

const parameters = rows(
  parameter("fam-table", "width", "Length", "1.6 m"),
  parameter("fam-table", "depth", "Length", "0.8 m"),
  parameter("fam-table", "height", "Length", "0.74 m"),
  parameter("fam-chair", "seat", "Length", "0.45 m"),
  parameter("fam-chair", "size", "Length", "0.45 m"),
  parameter("fam-basin", "width", "Length", "0.6 m"),
  parameter("fam-basin", "depth", "Length", "0.45 m"),
  parameter("fam-basin", "height", "Length", "0.2 m"),
  parameter("fam-lamp", "size", "Length", "0.3 m"),
  parameter("fam-lamp", "drop", "Length", "0.15 m"),
  parameter("fam-diffuser", "size", "Length", "0.6 m"),
  parameter("fam-diffuser", "thickness", "Length", "0.08 m"),
  parameter("fam-niche", "width", "Length", "0.5 m"),
  parameter("fam-niche", "depth", "Length", "0.3 m"),
  parameter("fam-niche", "sunk", "Length", "0.1 m"),
  parameter("fam-niche", "height", "Length", "0.6 m"),
  parameter("fam-hea", "h", "Length", "190 mm"),
);

const solids = rows(
  ["s-table", solid("fam-table", "Top", cuboid("0 m", "0 m", "0 m", "width", "depth", "height"))],
  ["s-seat", solid("fam-chair", "Seat", cuboid("0 m", "0 m", "seat", "size", "size", "50 mm"))],
  ["s-back", solid("fam-chair", "Back", cuboid("0 m", "0 m", "seat + 50 mm", "size", "50 mm", "0.45 m"))],
  ["s-hidden", { ...solid("fam-chair", "Hidden", cuboid("0 m", "0 m", "0 m", "2 m", "2 m", "2 m")), visible: "false" }],
  ["s-basin", solid("fam-basin", "Bowl", cuboid("0 m", "0 m", "0 m", "width", "depth", "height"), "\"m-steel\"")],
  ["s-lamp", solid("fam-lamp", "Shade", cuboid("0 m - size / 2", "0 m - size / 2", "0 m - drop", "size", "size", "drop"), "\"m-steel\"")],
  ["s-diffuser", solid("fam-diffuser", "Plate", cuboid("0 m - size / 2", "0 m - size / 2", "0 m - thickness", "size", "size", "thickness"), "\"m-steel\"")],
  ["s-niche", solid("fam-niche", "Box", cuboid("0 m", "0 m - sunk", "0 m", "width", "depth", "height"))],
  ["s-hea", solid("fam-hea", "Section", { Extrusion: { profile: { IShape: { width: "200 mm", depth: "h", web: "6.5 mm", flange: "10 mm" } }, base: "0 m", height: "1 m" } }, "\"m-steel\"")],
);

const components = rows(
  ["c-table", component("fam-table", 2, 2, { rotation: Math.PI / 6 })],
  ["c-table-wide", component("fam-table", 4, 1.2, { name: "Wide table" })],
  ["c-chair", component("fam-chair", 2, 3.4, { rotation: Math.PI, mirrored: true })],
  ["c-chair-high", component("fam-chair", 4.5, 3.2)],
  ["c-basin-south", component("fam-basin", 3, 0.6, { elevation: 0.85, host: "w-south" })],
  ["c-basin-north", component("fam-basin", 3, 3.5, { elevation: 0.85, host: "w-north", mirrored: true })],
  ["c-niche", component("fam-niche", 5.5, 2.5, { elevation: 0.9, host: "w-east", rotation: 0.1 })],
  ["c-lamp", component("fam-lamp", 3, 2, { elevation: 2.9, system: "Lighting" })],
  ["c-diffuser-ok", component("fam-diffuser", 5.5, 2, { elevation: 2.6, system: "Supply" })],
  ["c-diffuser-lost", component("fam-diffuser", 1, 1, { elevation: 2.9, system: "Supply" })],
  ["c-bump", component("fam-table", 0, 1)],
  ["c-high", component("fam-table", 3, 3, { elevation: 3.5 })],
  ["c-ghost", component("fam-ghost", 1, 3)],
  ["c-profile", component("fam-hea", 1, 3)],
  ["c-nowhere", component("fam-basin", 3, 0.6, { host: "w-nowhere" })],
  ["c-bad-override", component("fam-table", 5, 3.4)],
);

const overrides = rows(
  override("c-table-wide", "width", "1.8 m"),
  override("c-chair", "seat", "0.5 m"),
  override("c-chair-high", "seat", "0.55 m"),
  override("c-bad-override", "width", "ghost + 1 m"),
  override("c-bad-override", "depth", "0.5"),
);

const meps = rows(
  ["m-supply", mep("Supply", duct(0.3, 0.2), [[0.5, 2, 2.6], [5.5, 2, 2.6]], "Supply duct")],
  ["m-water", mep("DomesticWater", pipe(0.1), [[3, 0.5, 2.65], [3, 3.5, 2.65]], "Cold water")],
  ["m-waste", mep("Waste", pipe(0.11), [[5, 3.5, 0], [5, 3.5, 2.2], [5.4, 3.5, 2.2]], "Waste riser")],
  ["m-lighting", mep("Lighting", tray(0.2, 0.05), [[0.5, 3, 2.9], [3, 3, 2.9], [3, 2, 2.92]], "Lighting tray")],
  ["m-power", mep("Power", tray(0.3, 0.1), [[0.5, 1, 2.6], [5.5, 1, 2.6]], "Power tray")],
  ["m-return", mep("Return", duct(0.4, 0.25), [[0.5, 0.6, 2.2], [0.5, 0.6, 2.2]], "Degenerate return")],
);

const model = () => {
  const base = F.snap(
    {
      materials: { "m-brick": F.material("Brick"), "m-oak": oak, "m-steel": steel },
      wall_types: { "wt-200": F.wallType("Brick 200", [F.layer("m-brick", 0.2)]) },
      sites: { "site-1": F.site("Plot") },
      buildings: { "bldg-1": F.building("site-1", "House") },
      storeys: { "st-ground": F.storey("bldg-1", "Ground", 0, 3), "st-first": F.storey("bldg-1", "First", 1, 2.8) },
      walls: {
        "w-south": F.wall("st-ground", "wt-200", F.line([0, 0], [6, 0]), F.storeyTop(0), "South"),
        "w-east": F.wall("st-ground", "wt-200", F.line([6, 0], [6, 4]), F.storeyTop(0), "East"),
        "w-north": F.wall("st-ground", "wt-200", F.line([6, 4], [0, 4]), F.storeyTop(0), "North"),
        "w-west": F.wall("st-ground", "wt-200", F.line([0, 4], [0, 0]), F.storeyTop(0), "West"),
      },
    },
    "Components",
  );
  return { ...base, families, family_parameters: parameters, family_solids: solids, components, component_overrides: overrides, mep_elements: meps };
};

const dir = join(fixtures, em(0x1f4a1) + "inferences", em(0x1fa91) + "components", em(0x1f3e0) + "room", em(0x1f4f8) + "snapshot");
mkdirSync(dir, { recursive: true });
writeFileSync(join(dir, JSONF), JSON.stringify(model(), null, 2) + "\n");
console.log("components fixtures written");
