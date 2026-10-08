#!/usr/bin/env bun
/** 🎬️ Writes the demo snapshot source (`🖼️assets/🎬️demo/📸️snapshot.json`); the DSL text is blessed from it by `BIM_BLESS=1 cargo test bless_the_demo_text`. */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, em, subset } from "./r3-f1-paths.ts";
import * as F from "./r3-f1-fixtures.ts";

const demo = F.snap(
  {
    materials: { "m-brick": F.material("Brick"), "m-wool": { ...F.material("Mineral Wool"), category: "Insulation", density: 40, conductivity: 0.035, specific_heat: 1030 } },
    wall_types: { "wt-300": F.wallType("Brick 300", [F.layer("m-brick", 0.2), F.layer("m-wool", 0.1, "Insulation")]) },
    sites: { "site-1": F.site("Plot") },
    buildings: { "bldg-1": F.building("site-1", "House") },
    storeys: { "st-ground": F.storey("bldg-1", "Ground", 0, 3), "st-first": F.storey("bldg-1", "First", 1, 2.8) },
    walls: {
      "w-south": F.wall("st-ground", "wt-300", F.line([0, 0], [8, 0]), F.storeyTop(0), "South"),
      "w-east": F.wall("st-ground", "wt-300", F.line([8, 0], [8, 6]), F.storeyTop(0), "East"),
      "w-north": F.wall("st-ground", "wt-300", F.line([8, 6], [0, 6]), F.storeyTop(0), "North"),
      "w-west": F.wall("st-ground", "wt-300", F.line([0, 6], [0, 0]), F.storeyTop(0), "West"),
    },
  },
  "Demo House",
);
const assets = join(child(subset, "assets"), em(0x1f3ac) + "demo");
writeFileSync(join(assets, em(0x1f4f8) + "snapshot.json"), JSON.stringify(demo, null, 2) + "\n");
console.log("demo snapshot written");
