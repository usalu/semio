import { execFileSync as gitExecFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
const T = fileURLToPath(new URL(".", import.meta.url)).replace(/[\\/]$/, "");
const R = gitExecFileSync("git", ["-C", T, "rev-parse", "--show-toplevel"], { encoding: "utf8" }).trim();
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";
import { renderCatalogArtifacts } from "../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔣️icons/🏗️builder/📽️projection/🟦️.ts";

for (const artifact of renderCatalogArtifacts("rust")) {
  if (artifact.path.endsWith("README.md")) continue;
  mkdirSync(dirname(artifact.path), { recursive: true });
  writeFileSync(artifact.path, artifact.content);
}
