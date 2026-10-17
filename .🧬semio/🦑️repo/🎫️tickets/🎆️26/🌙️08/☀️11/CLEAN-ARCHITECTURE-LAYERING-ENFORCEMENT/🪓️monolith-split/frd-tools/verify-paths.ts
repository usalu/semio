// 🧲️ Every literal path handed to the shared reader must exist below the manifest directory of each crate that compiles the file (below the repository root for repo_text! / repo_bytes!).
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join, resolve } from "node:path";
import { rustTokens } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const F = join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split");
const G = join(F, "🗑️generated");
const owners = JSON.parse(readFileSync(join(G, "frd-owners.json"), "utf8")) as Record<string, { manifest: string; cls: string }[]>;
const NAMES = new Set(["fixture_text", "fixture_bytes", "fixture_path", "repo_text", "repo_bytes"]);
const sha = new Map<string, string>();
let checked = 0, missing = 0, files = 0, withoutOwner = 0;
const rows: string[] = [];
for (const [file, list] of Object.entries(owners)) {
  const abs = join(ROOT, file);
  if (!existsSync(abs)) continue;
  const text = readFileSync(abs, "utf8");
  if (!text.includes("semio_framework_test_fixtures")) continue;
  files += 1;
  const tokens = rustTokens(text);
  for (let i = 0; i + 4 < tokens.length; i += 1) {
    if (tokens[i]!.kind !== "identifier" || !NAMES.has(tokens[i]!.text) || tokens[i + 1]!.text !== "!" || tokens[i + 2]!.text !== "(" || tokens[i + 3]!.kind !== "string" || tokens[i + 4]!.text !== ")") continue;
    const literal = JSON.parse(tokens[i + 3]!.text) as string;
    if (literal.startsWith("@")) continue;
    if (!list.length) { withoutOwner += 1; continue; }
    const repo = tokens[i]!.text.startsWith("repo_");
    const hits = list.map(o => existsSync(repo ? resolve(ROOT, literal) : resolve(ROOT, dirname(o.manifest), literal)));
    checked += 1;
    if (!hits.some(Boolean)) { missing += 1; rows.push(`${file}\t${tokens[i]!.text}\t${literal}\t${list.map(o => o.manifest).join(";")}`); }
  }
}
writeFileSync(join(G, "frd-verify-paths.tsv"), rows.join("\n") + "\n");
console.log(JSON.stringify({ files, checked, missing, withoutOwner }));
