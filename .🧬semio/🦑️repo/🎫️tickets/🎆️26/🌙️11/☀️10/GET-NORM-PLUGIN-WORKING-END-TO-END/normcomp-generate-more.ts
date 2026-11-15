import { execFileSync as gitExecFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
const T = fileURLToPath(new URL(".", import.meta.url)).replace(/[\\/]$/, "");
const R = gitExecFileSync("git", ["-C", T, "rev-parse", "--show-toplevel"], { encoding: "utf8" }).trim();
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";
import { readEntityCatalog } from "../../../../../../../🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📥️source/🟦️.ts";
import { generatedTargets } from "../../../../../../../🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📋️plan/🟦️.ts";
import { renderMetabolismArtifacts } from "../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🏗️builder/📽️projection/🟦️.ts";

const write = (path: string, content: string | Uint8Array): void => { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, content); };
for (const target of generatedTargets(readEntityCatalog(R))) write(target.path, target.content);
for (const artifact of renderMetabolismArtifacts()) if (!artifact.path.endsWith("README.md")) write(artifact.path, artifact.content);
