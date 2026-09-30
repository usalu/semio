/** 🔬️ W2-R energy: reads every energy leaf payload schema whole through W1-D's `mutationInputDefs` + `argControl`, parses every
 * `x-semio-ui` with the manifest `parseInputUi`, compiles each leaf with the strict Ajv oracle (`x-semio-ui` vocabulary
 * registered) and judges every committed fixture payload with Ajv as a second third-party oracle beside
 * `🧪️w2-r-energy-check.py`. A strict-compile failure the HEAD version shares is pre-existing. */
import { argControl, mutationInputDefs } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";
import { parseInputUi } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🟦️.ts";
import { semioSchemaAjvV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { Glob } from "bun";

const repo = "/Users/ueli/Documents/semio";
const scope = "✏️s/🔌️plugins/🔋️energy";
const leaves = [...new Glob(`${scope}/**/🧬️mutations/*/🧬️schema/🔣️.json`).scanSync({ cwd: repo })].sort();
const controls = new Map<string, number>();
const failures: string[] = [];
const validators = new Map<string, (payload: unknown) => boolean>();
let inputs = 0;
let annotations = 0;
const walk = (node: unknown, path: string): void => {
  if (Array.isArray(node)) node.forEach((child) => walk(child, path));
  else if (node !== null && typeof node === "object")
    for (const [key, child] of Object.entries(node)) {
      if (key !== "x-semio-ui") walk(child, path);
      else {
        annotations += 1;
        try {
          parseInputUi(child);
        } catch (error) {
          failures.push(`INPUTUI ${path}: ${(error as Error).message}`);
        }
      }
    }
};
const headSchema = (path: string): unknown => {
  try {
    return JSON.parse(execFileSync("git", ["show", `HEAD:${path}`], { cwd: repo, encoding: "utf8" }));
  } catch {
    return undefined;
  }
};
const compiles = (schema: unknown): string | undefined => {
  try {
    semioSchemaAjvV1().compile(schema as object);
    return undefined;
  } catch (error) {
    return (error as Error).message;
  }
};
for (const path of leaves) {
  const schema = JSON.parse(readFileSync(`${repo}/${path}`, "utf8"));
  walk(schema, path);
  try {
    for (const def of mutationInputDefs(schema, () => undefined)) {
      inputs += 1;
      const kind = argControl(def).kind;
      controls.set(kind, (controls.get(kind) ?? 0) + 1);
    }
  } catch (error) {
    failures.push(`READER ${path}: ${(error as Error).message}`);
  }
  const refusal = compiles(schema);
  if (refusal !== undefined) {
    const head = headSchema(path);
    failures.push(`${head !== undefined && compiles(head) !== undefined ? "AJV-PREEXISTING" : "AJV"} ${path}: ${refusal}`);
  } else validators.set(path.split("/🧬️schema/🔣️.json")[0]!, semioSchemaAjvV1().compile(schema));
}
let fixtures = 0;
let negatives = 0;
for (const path of [...new Glob(`${scope}/**/🦠️mutation/🔣️.json`).scanSync({ cwd: repo })].sort()) {
  const [owner, rest] = path.split("/🧫️fixtures/") as [string, string];
  const head = rest.split("/").filter((segment) => segment !== "🧬️mutations")[0]!;
  const candidates = [...validators.keys()].filter((leaf) => leaf.startsWith(`${owner}/`) && leaf.split("/").at(-1)!.startsWith(head));
  const exact = candidates.find((leaf) => leaf.split("/").at(-1) === head);
  const leaf = exact ?? (candidates.length === 1 ? candidates[0] : undefined);
  if (leaf === undefined) {
    failures.push(`UNMAPPED ${path}`);
    continue;
  }
  const payload = JSON.parse(readFileSync(`${repo}/${path}`, "utf8"));
  const properties = JSON.parse(readFileSync(`${repo}/${leaf}/🧬️schema/🔣️.json`, "utf8")).properties ?? {};
  const instance = "payload" in payload && !("payload" in properties) ? payload.payload : Object.fromEntries(Object.entries(payload).filter(([key]) => !["mutation", "kind"].includes(key) || key in properties));
  fixtures += 1;
  const outcomePath = `${repo}/${path.replace("/🦠️mutation/🔣️.json", "/🎯️outcome/🔣️.json")}`;
  const outcome = existsSync(outcomePath) ? JSON.parse(readFileSync(outcomePath, "utf8")) : {};
  const valid = validators.get(leaf)!(instance);
  const negative = outcome.status === "rejected" && outcome.code === "mutation.invariant";
  if (negative) negatives += 1;
  if (negative && typeof outcome.invariant !== "string" && valid) failures.push(`NEGATIVE ${path}`);
  if ((!negative || typeof outcome.invariant === "string") && !valid) failures.push(`FIXTURE ${path}`);
}
for (const failure of failures) console.log(failure);
console.log(`leaves=${leaves.length} inputs=${inputs} annotations=${annotations} fixtures=${fixtures} negatives=${negatives} controls=${JSON.stringify(Object.fromEntries([...controls].sort()))} failures=${failures.length}`);
process.exit(failures.some((failure) => !failure.startsWith("AJV-PREEXISTING")) ? 1 : 0);
