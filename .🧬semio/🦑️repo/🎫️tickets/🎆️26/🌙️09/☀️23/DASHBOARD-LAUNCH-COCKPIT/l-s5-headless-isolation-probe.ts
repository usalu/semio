/** 🧪 Exercises the hub headless Stdio isolation proof (extracted from the owner script) against the real manifest and hostile copies. */
import { cpSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { isAbsolute, join, relative, resolve } from "node:path";
import { repoCacheDirectory } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../.."), manifest = "🌎️hub/📦️packages/🦀️rust/📋️project.json";
const script = readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/📜️script.ts"), "utf8");
const body = (name: string): string => {
  const start = script.indexOf(`function ${name}(`), end = script.indexOf("\n}\n", start);
  if (start < 0 || end < 0) throw new Error(`missing ${name}`);
  return script.slice(start, end + 2);
};
const source = new Bun.Transpiler({ loader: "ts" }).transformSync(`${body("headlessStdioOwnedCargoTarget")}\n${body("proveHeadlessStdioCommandIsolation")}\nreturn proveHeadlessStdioCommandIsolation;`);
const prove = new Function("readFileSync", "join", "resolve", "relative", "isAbsolute", "repoCacheDirectory", source)(readFileSync, join, resolve, relative, isAbsolute, repoCacheDirectory) as (root: string) => void;
prove(repoRoot);
console.log("real manifest: isolated");
const scratch = mkdtempSync(join(import.meta.dir, "🗑️generated/launch-s5/headless-isolation-"));
try {
  const hostile: [string, (targets: any) => void][] = [
    ["shared roots", (t) => { t["native-catalog-selection-check"].options.env = t["native-openable-catalog-provider-check"].options.env; }],
    ["absolute root", (t) => { const env = t["native-catalog-selection-check"].options.env; env.SEMIO_TEST_ARTIFACT_DIR = join(repoRoot, env.SEMIO_TEST_ARTIFACT_DIR); env.CARGO_TARGET_DIR = `${env.SEMIO_TEST_ARTIFACT_DIR}/cargo-target`; }],
    ["outside the repository cache", (t) => { t["native-catalog-selection-check"].options.env = { SEMIO_TEST_ARTIFACT_DIR: "🗑️generated/selection", CARGO_TARGET_DIR: "🗑️generated/selection/cargo-target" }; }],
    ["foreign cargo target", (t) => { t["native-catalog-selection-check"].options.env.CARGO_TARGET_DIR = ".🧬semio/🦑️repo/⚡️cache/cargo/target"; }],
    ["missing stdio-only configuration", (t) => { delete t["native-openable-catalog-provider-check"].configurations; }],
    ["stdio-only without its argument", (t) => { t["native-openable-catalog-provider-check"].configurations["stdio-only"].args = ""; }],
    ["missing env", (t) => { delete t["native-openable-catalog-provider-check"].options.env; }],
    ["renamed command", (t) => { t["native-catalog-selection-check"].options.command = "bun ./📜️script.ts other"; }],
    ["not generated", (t) => { t["native-catalog-selection-check"].options.env = { SEMIO_TEST_ARTIFACT_DIR: ".🧬semio/🦑️repo/⚡️cache/tests/hub/selection", CARGO_TARGET_DIR: ".🧬semio/🦑️repo/⚡️cache/tests/hub/selection/cargo-target" }; }],
  ];
  let denied = 0;
  for (const [name, mutate] of hostile) {
    const root = join(scratch, String(denied)), project = JSON.parse(readFileSync(join(repoRoot, manifest), "utf8"));
    mutate(project.targets);
    mkdirSync(join(root, "🌎️hub/📦️packages/🦀️rust"), { recursive: true });
    writeFileSync(join(root, manifest), JSON.stringify(project));
    let message = "";
    try { prove(root); } catch (error) { message = (error as Error).message; }
    if (!message) throw new Error(`hostile manifest admitted: ${name}`);
    console.log(`denied ${name}: ${message}`);
    denied++;
  }
  console.log(`hostile manifests denied=${denied}/${hostile.length}`);
} finally {
  rmSync(scratch, { recursive: true, force: true });
}
void cpSync;
