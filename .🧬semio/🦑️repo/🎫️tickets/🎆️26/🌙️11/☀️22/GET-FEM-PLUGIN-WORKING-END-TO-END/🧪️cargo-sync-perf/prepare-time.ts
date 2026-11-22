import { createHash } from "node:crypto";
import { resolve } from "node:path";
import { cargoWorkspaceForManifest, cargoWorkspaceMembers, prepareCargoOwners } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
import type { CargoPhysicalObservationsV1 } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/👁️witness/🟦️.ts";

const root = resolve(process.env.SEMIO_WORKSPACE_ROOT ?? "C:/semio");
const manifest = process.argv[2]!;
const names = process.argv.slice(3);
const time = <T>(label: string, work: () => T): T => {
  const start = performance.now(), cpu = process.cpuUsage();
  const value = work();
  const used = process.cpuUsage(cpu);
  console.error("[DEBUG]", label, "ms", Math.round(performance.now() - start), "cpu-ms", Math.round((used.user + used.system) / 1000));
  return value;
};
const observations: CargoPhysicalObservationsV1 = new Map();
const scope = time("scope", () => cargoWorkspaceForManifest(root, manifest, observations));
const prepared = time("prepare", () => prepareCargoOwners(root, scope, names));
console.error("[DEBUG] owners", prepared.owners.length, "manifests", prepared.manifests.length, "recipes", prepared.recipes.length);
const scopes = prepared.owners.map(owner => cargoWorkspaceForManifest(root, owner === "." ? "Cargo.toml" : owner + "/Cargo.toml", observations));
for (const s of scopes) time("members " + s.manifest, () => cargoWorkspaceMembers(root, s, observations).length);
console.error("[DEBUG] observations", observations.size, createHash("sha256").update(JSON.stringify([...observations.keys()])).digest("hex").slice(0, 8));
