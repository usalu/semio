/**
 * ⚠️ M5a destructive backlog — mirrors `catalog::DESTRUCTIVE_VERB_WORDS` and its contiguous
 * word-run matcher exactly (so this agrees with `semio-os-mcp audit`, which is the gate), then
 * subtracts every id already declared at source. Prints one `<plugin-dir> destructive:a,b,c`
 * argument line per plugin for `🐍️m5a-declare-audience.py`.
 *
 * Run: `bun .🧬semio/.../🐍️m5a-destructive-backlog.ts`
 */
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const registry = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json"), "utf8")) as Array<{ pluginId: string; cratePath: string }>;

const DESTRUCTIVE_VERB_WORDS = ["delete", "remove", "clear", "discard", "purge", "wipe", "erase", "truncate", "setactiveexample", "setfixturejson", "setspecjson", "setsnapshot", "loaddocument", "setdocument", "replacedocument"];

function words(id: string): string[] {
  const out: string[] = [];
  let current = "";
  for (const character of id) {
    if (".-_:".includes(character)) {
      if (current) out.push(current);
      current = "";
      continue;
    }
    if (character >= "A" && character <= "Z" && current) {
      out.push(current);
      current = "";
    }
    current += character.toLowerCase();
  }
  if (current) out.push(current);
  return out;
}

function matched(id: string): string | undefined {
  const parts = words(id);
  for (let start = 0; start < parts.length; start += 1) {
    let run = "";
    for (const part of parts.slice(start)) {
      run += part;
      const hit = DESTRUCTIVE_VERB_WORDS.find((candidate) => candidate === run);
      if (hit) return hit;
    }
  }
  return undefined;
}

function collect(descriptor: any): any[] {
  const out: any[] = [];
  const seen = new Set<unknown>();
  const walk = (node: any): void => {
    if (!node || typeof node !== "object" || seen.has(node)) return;
    seen.add(node);
    if (Array.isArray(node)) return void node.forEach(walk);
    if (typeof node.id === "string" && (node.kind || node.semantics)) out.push(node);
    for (const [key, child] of Object.entries(node)) if (key !== "semantics") walk(child);
  };
  walk(descriptor);
  return out;
}

let open = 0;
for (const entry of registry) {
  const ownerRoot = dirname(dirname(join(repoRoot, entry.cratePath)));
  let descriptor: any;
  try {
    descriptor = JSON.parse(readFileSync(join(ownerRoot, "🔣️.json"), "utf8"));
  } catch {
    continue;
  }
  const ids = new Set<string>();
  for (const action of collect(descriptor)) {
    if ((action.kind ?? "") !== "mutation") continue;
    if (action.semantics?.audience && action.semantics.audience !== "agent") continue;
    if (action.semantics?.effects?.destructive) continue;
    if (matched(action.id)) ids.add(action.id);
  }
  if (ids.size === 0) continue;
  const relative = ownerRoot.slice(repoRoot.length + 1);
  const undeclared = [...ids].filter((id) => {
    const hits = spawnSync("grep", ["-rn", "--include=*.rs", "-F", "-e", `action_destructive("${id}"`, "-e", `"${id}"`, ownerRoot], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 }).stdout ?? "";
    return !hits.split("\n").some((line) => line.includes(`action_destructive("${id}"`) || (line.includes(`"${id}"`) && line.includes(".destructive()")));
  });
  if (undeclared.length === 0) continue;
  open += undeclared.length;
  console.log(`${relative} destructive:${undeclared.join(",")}`);
}
console.error(`# ${open} destructive-class verb(s) open at source under the gate's own lexicon`);
