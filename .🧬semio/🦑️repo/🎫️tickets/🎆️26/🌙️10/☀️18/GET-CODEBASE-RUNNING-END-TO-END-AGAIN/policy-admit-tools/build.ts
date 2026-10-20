/**
 * 🧱️ Stages the SDK foundation admission: applies anchored edits to copies of the live policy files and writes `plan.json` (base hashes, edits, staged hashes) for `apply.ts`.
 *
 *   bun build.ts            # reads the live tree, writes staged/ and plan.json (never touches the repo)
 */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const here = import.meta.dir;
const root = process.env.SEMIO_ROOT ?? resolve(here, "../../../../../../../..");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const direction = `${library}/🕸️dependencies/🧭️direction`;
const sha = (text: string | Buffer): string => createHash("sha256").update(text).digest("hex");
const foundation = ["@semio-tech/framework-hash", "@semio-tech/framework-interaction", "@semio-tech/framework-manifest", "@semio-tech/framework-trace", "@semio-tech/ui", "@semio-tech/ui-contract", "@semio-tech/ui-react", "@semio-tech/ui-styling"];

type Edit = { old: string; new: string };
const edited: { id: string; target: string; staged: string; edits: Edit[] }[] = [
  {
    id: "taxonomy",
    target: `${library}/🔣️taxonomy.json`,
    staged: "10-taxonomy.json",
    edits: [{
      old: `      "sdkPackage": "@semio-tech/framework",\n      "toolingRoles": []`,
      new: `      "sdkPackage": "@semio-tech/framework",\n      "sdkFoundationPackages": [\n${foundation.map(name => `        "${name}"`).join(",\n")}\n      ],\n      "toolingRoles": []`,
    }],
  },
  {
    id: "schema",
    target: `${library}/🧬️schema/🧱️dependency-direction/🔣️.json`,
    staged: "20-schema.json",
    edits: [
      { old: `            "sdkPackage",\n            "toolingRoles"\n          ],`, new: `            "sdkPackage",\n            "sdkFoundationPackages",\n            "toolingRoles"\n          ],` },
      {
        old: `            "sdkPackage": {\n              "type": "string",\n              "minLength": 1\n            },`,
        new: `            "sdkPackage": {\n              "type": "string",\n              "minLength": 1\n            },\n            "sdkFoundationPackages": {\n              "type": "array",\n              "minItems": 1,\n              "uniqueItems": true,\n              "items": {\n                "type": "string",\n                "pattern": "^@[^/\\\\s]+/[^/\\\\s]+$"\n              }\n            },`,
      },
    ],
  },
  {
    id: "construction",
    target: `${direction}/🏗️construction/🟨️.cjs`,
    staged: "30-construction.cjs",
    edits: [
      { old: `"rulePrefix|sdkPackage|toolingRoles|version"`, new: `"rulePrefix|sdkFoundationPackages|sdkPackage|toolingRoles|version"` },
      {
        old: `throw Error("Dependency policy requires canonical authored public API authority");`,
        new: `throw Error("Dependency policy requires canonical authored public API authority");\nif (!Array.isArray(api.sdkFoundationPackages) || !api.sdkFoundationPackages.length || new Set(api.sdkFoundationPackages).size !== api.sdkFoundationPackages.length || api.sdkFoundationPackages.some(name => typeof name !== "string" || !/^@[^/\\s]+\\/[^/\\s]+$/u.test(name) || name === api.sdkPackage)) throw Error("Dependency policy requires unique admitted SDK foundation package names");`,
      },
      {
        old: `const RESOLVED_NODE_BUILTIN_PATH =`,
        new: `/** 🗝️ The authored public targets of the admitted SDK foundation packages (\`publicApi.sdkFoundationPackages\`): neutral modules that carry keys once published by the SDK package, shared by plugins and renderer hosts. */\nconst SDK_FOUNDATION_TARGETS = exportTargetPatterns(workspacePackages.filter(pkg => api.sdkFoundationPackages.includes(pkg.name)));\nconst RESOLVED_NODE_BUILTIN_PATH =`,
      },
      { old: `  ...RENDERER_HOST_WINDOW_KIT_TARGETS,\n`, new: `  ...RENDERER_HOST_WINDOW_KIT_TARGETS,\n  ...SDK_FOUNDATION_TARGETS,\n` },
      { old: `/** 🔌️ Every plugin source owner consumes only the admitted neutral public SDK. */`, new: `/** 🔌️ Every plugin source owner consumes only the admitted neutral public SDK and its foundation packages. */` },
      { old: `use the neutral authored public SDK; private framework`, new: `use the neutral authored public SDK and the admitted SDK foundation packages; private framework` },
      { old: `pathNot: exportTargetPatterns(sdk) } };`, new: `pathNot: [...exportTargetPatterns(sdk), ...SDK_FOUNDATION_TARGETS] } };` },
      { old: `the react renderer host may depend only on ui/styling, framework-core`, new: `the react renderer host may depend only on ui/styling, the admitted SDK foundation packages, framework-core` },
    ],
  },
  {
    id: "bootstrap-type",
    target: `${direction}/🚀️bootstrap/🟦️.ts`,
    staged: "40-bootstrap.ts",
    edits: [{ old: `sdkPackage:string;toolingRoles:readonly string[]}>`, new: `sdkPackage:string;sdkFoundationPackages:readonly string[];toolingRoles:readonly string[]}>` }],
  },
];
const created = [
  { id: "test-fixture", target: `${direction}/🏗️construction/🧪️tests/🔑️sdk-foundation-admission/🧫️fixtures/🔣️.json`, staged: "60-sdk-foundation-admission.fixture.json" },
  { id: "test", target: `${direction}/🏗️construction/🧪️tests/🔑️sdk-foundation-admission/🟦️.ts`, staged: "61-sdk-foundation-admission.test.ts" },
];

const entries: unknown[] = [];
for (const row of edited) {
  const base = readFileSync(join(root, row.target), "utf8");
  let text = base;
  for (const edit of row.edits) {
    const count = text.split(edit.old).length - 1;
    if (count !== 1) throw Error(`${row.id}: anchor occurs ${count}x: ${edit.old.slice(0, 80)}`);
    text = text.replace(edit.old, () => edit.new);
  }
  writeFileSync(join(here, "staged", row.staged), text);
  entries.push({ op: "edit", ...row, baseSha256: sha(base), stagedSha256: sha(text) });
}
for (const row of created) entries.push({ op: "create", ...row, stagedSha256: sha(readFileSync(join(here, "staged", row.staged))) });
writeFileSync(join(here, "plan.json"), JSON.stringify({ schemaVersion: 1, foundation, entries }, null, 2) + "\n");
console.log(entries.map((row: any) => `${row.op}\t${row.id}\t${row.staged}`).join("\n"));
