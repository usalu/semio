import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const policy = await import(pathToFileURL(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts")).href);
for (const mode of ["prototype", "release"]) {
  const spec = { version: 1, mode, target: "wasm32-wasip2", producer: "wasm-component", operation: "build", compositions: ["semio-hub-procedural-component"] };
  const rendered = policy.renderCargoProfileConfigurationV1(spec, "🌎️hub/Cargo.toml", "repository");
  console.log(`[DEBUG] ${mode} role=${rendered.plan.role} target=${JSON.stringify(rendered.plan.target)} overrides=${rendered.plan.packageOverrides.length}`);
  console.log(rendered.selectors.filter((line: string) => !line.includes(".package.")).join("\n"));
}
