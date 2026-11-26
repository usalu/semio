import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { nativeBuildPassPrerequisitesV1, nativeBuildPrerequisiteOrderV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/📋️prerequisites/🟦️.ts";

const fixture = JSON.parse(readFileSync(resolve(import.meta.dirname, "release-font-prerequisites.json"), "utf8"));
const root = resolve(import.meta.dirname, "../../../../../../..");

for (const { path, target, cargoPrerequisites } of fixture.projects) {
  const adopted = JSON.parse(readFileSync(resolve(root, path), "utf8")).targets[target].metadata.semio.release;
  const jobOrder = nativeBuildPrerequisiteOrderV1(adopted.jobs);
  const ordered = jobOrder.map(id => adopted.jobs.find((job: { id: string }) => job.id === id));
  const sequence = jobOrder.filter(id => fixture.browserDependencyOrder.includes(id));
  const prerequisites = nativeBuildPassPrerequisitesV1(ordered, adopted.stageAfter).map(job => job.id);
  const browser = adopted.stageAfter.browser.map((dependency: string) => dependency.slice(4));
  if (JSON.stringify(sequence) !== JSON.stringify(fixture.browserDependencyOrder)) throw Error(path + " has incorrect browser font job order");
  if (JSON.stringify(browser) !== JSON.stringify(fixture.browserDependencyOrder)) throw Error(path + " does not bind all font jobs to browser publication");
  if (JSON.stringify(prerequisites) !== JSON.stringify(cargoPrerequisites)) throw Error(path + " unnecessarily waits before Cargo: " + prerequisites.join(", "));
  console.log(JSON.stringify({ path, sequence, browser, prerequisites }));
}
