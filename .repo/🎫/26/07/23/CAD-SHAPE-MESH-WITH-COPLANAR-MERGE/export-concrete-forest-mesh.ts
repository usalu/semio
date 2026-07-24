#!/usr/bin/env bun
/** Regenerate concrete forest left from CAD B-Rep wires + coplanar merge (no fill_holes). */
import { spawnSync } from "node:child_process";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const meshOut = join(repoRoot, "lowpoly/example/concrete-forest-left.mesh.json");
const targetDir = "/tmp/semio-lowpoly-mesh-fix";

function runCargoTest(args: string[], env: Record<string, string>): string {
  const result = spawnSync("cargo", args, {
    cwd: repoRoot,
    encoding: "utf8",
    env: { ...process.env, CARGO_TARGET_DIR: targetDir, ...env },
  });
  const output = `${result.stdout ?? ""}\n${result.stderr ?? ""}`;
  if (result.status !== 0) throw new Error(`cargo ${args.join(" ")} failed:\n${output}`);
  return output;
}

const output = runCargoTest(
  ["test", "-p", "lowpoly_core", "export_concrete_forest_left_lowpoly_mesh_json", "--", "--nocapture"],
  { EXPORT_LOWPOLY_FOREST_MESH: "1" },
);

const meshMatch = output.match(/LOWPOLY_FOREST_MESH_JSON_START\n([\s\S]*?)\nLOWPOLY_FOREST_MESH_JSON_END/);
if (!meshMatch?.[1]) throw new Error(`failed to export concrete forest mesh json\n${output.slice(-4000)}`);
writeFileSync(meshOut, meshMatch[1]);
console.log(`[DEBUG] wrote ${meshOut}`);

runCargoTest(["test", "-p", "lowpoly_core", "print_default_projection_json_for_example_asset", "--", "--nocapture"], {
  LOWPOLY_WRITE_DEFAULT_FIXTURE: "1",
});
console.log(`[DEBUG] wrote default.lowpoly.json`);
