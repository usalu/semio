/** 🧪️ Validates the portable WFC journey with Ajv and loads every admitted scenario through Bun. */
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(import.meta.url);
const Ajv = require("ajv");
const cases = JSON.parse(readFileSync(new URL("./wfc-browser-cases.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("./wfc-browser-schema.json", import.meta.url), "utf8"));
const validate = new Ajv().compile(schema);
if (!validate(cases)) throw new Error(JSON.stringify(validate.errors));
if (new Set(cases.variants.map((row: { variant: string }) => row.variant)).size !== 5) throw new Error("five distinct variants");
if (new Set(cases.variants.map((row: { port: number }) => row.port)).size !== 5) throw new Error("five distinct ports");
const { controlWorkloadForms } = await import("./wfc-browser-scenario.ts");
for (const row of cases.variants) {
  for (const locale of ["en", "de"]) {
    const forms = controlWorkloadForms(row, locale);
    if (row.workload.slots) {
      if (forms.length !== 256 || new Set(forms.map(form => form.args.id)).size !== 256 || forms.some(form => form.action !== "create-slot")) throw Error("public graph workload " + row.variant);
    } else if (forms.length !== 1 || forms[0]!.action !== (row.variant === "bitmap" ? "resize-output" : row.variant === "grid2d" ? "resize-grid" : "resizeGrid")) throw Error("public grid workload " + row.variant);
  }
  const scenario = (await import("./wfc-browser-" + row.variant + ".ts")).timeTravelScenarioV1;
  if (scenario.variant !== row.variant || JSON.stringify(scenario.order) !== JSON.stringify(cases.steps)) throw new Error("scenario contract " + row.variant);
  if (resolve(import.meta.dir, "wfc-browser-" + row.variant + ".ts").length > 256) throw new Error("path budget");
}
console.log("WFC browser inputs: Ajv schema, five scenario exports and both-locale public workload expansions pass");
