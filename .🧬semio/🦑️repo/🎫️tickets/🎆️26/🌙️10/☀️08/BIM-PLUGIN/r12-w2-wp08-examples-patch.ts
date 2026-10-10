#!/usr/bin/env bun
/**
 * 🧗️ Applies `wallDepthFor` to the committed house snapshot in place (the same edit `r4-x-examples-gen.ts` makes while generating the examples). Used while the generator itself is blocked by an unrelated in-flight
 * change of another package; once the generator runs again its output equals this file. Usage: `bun r12-w2-wp08-examples-patch.ts`.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, em, subset } from "./r3-f1-paths.ts";
import { wallDepthFor } from "./r12-w2-wp08-examples.ts";

const file = join(child(subset, "assets"), em(0x1f3e1) + "house", em(0x1f4f8) + "snapshot.json");
const model = JSON.parse(readFileSync(file, "utf8"));
wallDepthFor("house", model);
writeFileSync(file, JSON.stringify(model, null, 2) + "\n");
console.log(`patched ${Object.keys(model.wall_sweeps).length} sweeps, ${Object.keys(model.walls).filter((id) => model.walls[id].top.Roof).length} attached walls`);
