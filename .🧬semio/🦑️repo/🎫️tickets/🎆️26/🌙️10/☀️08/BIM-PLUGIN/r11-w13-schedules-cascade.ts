#!/usr/bin/env bun
/**
 * 🌊️ R11 `w13-schedules`: adds the case `cascades-the-schedules` to `delete-storey` (a schedule scoped to the deleted storey leaves with it, an unscoped one and one scoped to another storey stay) and mounts its test.
 * `bun r11-w13-schedules-cascade.ts` writes the fixtures and the test file and splices the mount into the `delete_storey` block of the artifact root (idempotent); bless `after`/`diff` with `BIM_BLESS=1 cargo test`.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import * as F from "./r3-f1-fixtures.ts";
import { emitCase } from "./r3-f1-gen-leaf.ts";
import { artifact, em, RS } from "./r3-f1-paths.ts";

const column = (field: string, total = false) => ({ key: { Field: { field } }, total });
const schedule = (name: string, storeys: string[]) => ({ name, category: "Wall", columns: [column("Name"), column("Length", true)], sort: [], filter: [], group: [], itemize: true, storeys, phases: [] });
const before: any = F.scene();
before.schedules = { "sch-ground": schedule("Ground walls", ["st-ground"]), "sch-both": schedule("Walls on both", ["st-ground", "st-first"]), "sch-first": schedule("First walls", ["st-first"]), "sch-all": schedule("All walls", []) };

const mounts = emitCase("delete-storey", em(0x1f6ae) + "delete-storey", "DeleteStorey", { name: "cascades-the-schedules", emoji: 0x1f4cb, before, mutation: { id: "st-ground" }, outcome: { status: "applied" } });
const root = join(artifact, RS);
const source = readFileSync(root, "utf8");
if (source.includes("mod tests_cascades_the_schedules;")) {
  console.log("already mounted");
} else {
  const start = source.indexOf("pub mod delete_storey {");
  const end = source.indexOf("\n" + " ".repeat(24) + "}\n", start);
  if (start < 0 || end < 0) throw new Error("mount block of delete-storey not found");
  writeFileSync(root, source.slice(0, end + 1) + mounts + source.slice(end + 1));
  console.log("mounted");
}
