// 🛠️ Repairs the live lifted scripts: argv pass-through for routers, bare entry for programs that own their process. usage: bun repair.ts <scripts.txt> [--write] [--only <script>]
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { BIRTH_OPENER, bareScript, dispatchOptions, isTestProgram } from "./shape.ts";

const REPO = "/Users/ueli/Documents/semio";
const write = process.argv.includes("--write");
const only = process.argv.includes("--only") ? process.argv[process.argv.indexOf("--only") + 1] : null;
const PARAMETERIZED = /export async function runScriptProgram\(programInvocation: ScriptInvocation\): Promise<void> \{/u;
const tally: Record<string, number> = {};
const count = (key: string): void => { tally[key] = (tally[key] ?? 0) + 1; };

function ownBirthModule(source: string): string {
  if (!PARAMETERIZED.test(source)) return source;
  let text = source.replace(PARAMETERIZED, "export async function runScriptProgram(): Promise<void> {");
  text = text.split("withScriptInvocationSignalV1(programInvocation,new AbortController().signal,").join(`${BIRTH_OPENER}(process.env,`);
  const unused = (name: string): boolean => !new RegExp(`\\b${name}\\b`, "u").test(text.replace(/import[^;]*;/gu, ""));
  text = text.replace(/\nimport type \{ ScriptInvocation \} from "[^"]+";/u, (row) => (unused("ScriptInvocation") ? "" : row));
  return text.replace(/import\s*(\{)([^}]*)\}\s*from\s*("[^"]+");/gu, (row, _open, names: string, specifier: string) => {
    const list = names.split(",").map((name) => name.trim()).filter(Boolean);
    if (!list.includes("withScriptInvocationSignalV1") && !list.includes("type ScriptInvocation")) return row;
    const next = list.filter((name) => !(name === "withScriptInvocationSignalV1" && unused(name)) && !(name === "type ScriptInvocation" && unused("ScriptInvocation")));
    if (!next.includes(BIRTH_OPENER) && !unused(BIRTH_OPENER)) next.unshift(BIRTH_OPENER);
    const compact = !/\s/u.test(row.slice(6, 7));
    return `import${compact ? "" : " "}{${compact ? "" : " "}${next.join(compact ? "," : ", ")}${compact ? "" : " "}}${compact ? "" : " "}from${compact ? "" : " "}${specifier};`;
  });
}

for (const script of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  if (only && script !== only) continue;
  const scriptSource = readFileSync(resolve(REPO, script), "utf8");
  const spec = /import \{[^}]*\brunScriptProgram\b[^}]*\} from "([^"]+)"/u.exec(scriptSource)?.[1];
  if (!spec) { count("no-import"); continue; }
  const modulePath = resolve(REPO, dirname(script), spec);
  if (!existsSync(modulePath)) { count("no-module"); continue; }
  const moduleSource = readFileSync(modulePath, "utf8");
  const moduleText = ownBirthModule(moduleSource);
  const owned = isTestProgram(script);
  let next: string;
  if (owned) {
    const header = scriptSource.slice(0, scriptSource.search(/^import\b/mu)).replace(/\s+$/u, "");
    next = bareScript(header, spec);
  } else {
    const commands = [...scriptSource.matchAll(/\.register\("([^"]+)"/gu)].map((row) => row[1]!);
    const command = /defaultCommand: "([^"]+)"/u.exec(scriptSource)?.[1] ?? /argv: \["([^"]+)"\]/u.exec(scriptSource)?.[1] ?? commands[0];
    if (!command) { count("skip:no-command"); console.log("SKIP no-command", script); continue; }
    next = scriptSource.replace(/\{ invocation: original(?:, defaultCommand: "[^"]+")?(?:, argv: \[[^\]]*\])? \}/u, dispatchOptions(command)).replace(/await runScriptProgram\([^)]*\);/gu, "await runScriptProgram();");
    if (!/argv: \[/u.test(next)) { count("skip:no-terminal"); console.log("SKIP no-terminal", script); continue; }
  }
  const moduleChanged = moduleText !== moduleSource, scriptChanged = next !== scriptSource;
  count(`${owned ? "bare" : "router"}:${moduleChanged || scriptChanged ? "changed" : "same"}`);
  if (!write) continue;
  if (readFileSync(modulePath, "utf8") !== moduleSource || readFileSync(resolve(REPO, script), "utf8") !== scriptSource) { count("raced"); continue; }
  if (moduleChanged) writeFileSync(modulePath, moduleText);
  if (scriptChanged) writeFileSync(resolve(REPO, script), next);
}
console.log(tally);
