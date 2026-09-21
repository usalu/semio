/** 🩺️ Evaluates the hub script's own neutral actor admission against the lease fixture to name the failing clause. */
import { readFileSync, writeFileSync } from "node:fs";

const repoRoot = "/Users/ueli/Documents/semio";
const script = readFileSync(repoRoot + "/🌎️hub/📦️packages/🦀️rust/📜️script.ts", "utf8");
const take = (name: string): string => {
  const start = script.indexOf(`function ${name}(`);
  if (start < 0) throw new Error("missing " + name);
  let depth = 0, index = script.indexOf("{", start);
  for (let cursor = index; cursor < script.length; cursor += 1) {
    if (script[cursor] === "{") depth += 1;
    else if (script[cursor] === "}") { depth -= 1; if (depth === 0) return script.slice(start, cursor + 1); }
  }
  throw new Error("unbalanced " + name);
};
const source = [take("documentOpenNeutralObject"), take("documentOpenNeutralBrowserActor")].join("\n");
const modulePath = process.env.TMPDIR + "jc1-neutral-actor.ts";
writeFileSync(modulePath, source + "\nexport { documentOpenNeutralBrowserActor };\n");
const { documentOpenNeutralBrowserActor: check } = await import(modulePath);

const fixture = JSON.parse(readFileSync(repoRoot + "/🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json", "utf8"));
for (const [name, row] of [["plan", fixture.plan], ["manifest", fixture.manifest]] as const) {
  try {
    check(row.browserActor, row.package, row.surface?.rendererTarget, name === "manifest");
    console.log(`${name}: ADMITTED`);
  } catch (error) {
    console.log(`${name}: REFUSED -> ${(error as Error).message}`);
    console.log(`   renderer=${JSON.stringify(row.surface?.rendererTarget)} policy=${JSON.stringify(row.browserActor?.codegenPolicy)} interfaces=${JSON.stringify(row.browserActor?.importInterfaces)} keys=${JSON.stringify(Object.keys(row.browserActor ?? {}))}`);
  }
}
