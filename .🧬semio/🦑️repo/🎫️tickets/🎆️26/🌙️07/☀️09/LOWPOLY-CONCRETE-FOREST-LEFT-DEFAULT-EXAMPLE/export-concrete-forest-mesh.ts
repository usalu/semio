#!/usr/bin/env bun
/** Export tessellated concrete forest left halfedge mesh for lowpoly default example. */
import { execFileSync } from "node:child_process";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../..");
const meshOut = join(repoRoot, "lowpoly/example/concrete-forest-left.mesh.json");
const fixtureOut = join(repoRoot, "lowpoly/example/default.lowpoly.json");

const output = execFileSync("cargo", ["test", "-p", "lowpoly_core", "--test", "export_concrete_forest_mesh", "export_concrete_forest_left_lowpoly_mesh_json", "--", "--nocapture"], {
  cwd: repoRoot,
  encoding: "utf8",
  env: { ...process.env, EXPORT_LOWPOLY_FOREST_MESH: "1" },
});

const meshMatch = output.match(/LOWPOLY_FOREST_MESH_JSON_START\n([\s\S]*?)\nLOWPOLY_FOREST_MESH_JSON_END/);
if (!meshMatch?.[1]) throw new Error("failed to export concrete forest mesh json");
writeFileSync(meshOut, meshMatch[1]);
console.log(`[DEBUG] wrote ${meshOut}`);

execFileSync("cargo", ["test", "-p", "lowpoly_core", "print_default_fixture_json_for_example_asset", "--", "--nocapture"], {
  cwd: repoRoot,
  encoding: "utf8",
  env: { ...process.env, LOWPOLY_WRITE_DEFAULT_FIXTURE: "1" },
});
console.log(`[DEBUG] wrote ${fixtureOut}`);
