import { createHash } from "node:crypto";
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";

const root = process.env.SEMIO_FIXTURE_REPO_ROOT!, ticket = dirname(dirname(import.meta.dir));
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const names = ["🚨️transaction-sentinel-cases", "💉️ticket-important-exact-mutations"];
const moves = names.map(name => {
  const old = library + "/🧫️fixtures/" + name + "/🔣️.json", next = old.replace("/🧫️fixtures/", "/🖼️assets/");
  const before = readFileSync(join(root, old)), sha256 = createHash("sha256").update(before).digest("hex");
  mkdirSync(dirname(join(root, next)), { recursive: true });
  renameSync(join(root, old), join(root, next));
  if (createHash("sha256").update(readFileSync(join(root, next))).digest("hex") !== sha256) throw new Error("Move changed bytes");
  return { old, new: next, sha256 };
});
const listed = spawnSync("rg", ["--files", library, "-g", "*.ts", "-g", "*.json"], { cwd: root, encoding: "utf8" });
if (listed.status !== 0) throw new Error(listed.stderr);
const modified: string[] = [];
for (const path of listed.stdout.trim().split("\n")) {
  const before = readFileSync(join(root, path), "utf8");
  let after = before;
  for (const name of names) after = after.replaceAll("🧫️fixtures/" + name, "🖼️assets/" + name);
  if (path === library + "/🧹️normalization/🟦️.ts") {
    after = after.replaceAll("TRANSACTION_SENTINEL_CASES_FIXTURE_PATH", "TRANSACTION_SENTINEL_CASES_CATALOG_PATH").replaceAll("TICKET_IMPORTANT_EXACT_MUTATIONS_FIXTURE_PATH", "TICKET_IMPORTANT_EXACT_MUTATIONS_CATALOG_PATH");
    after = after.replaceAll("fixtureContentHash", "catalogContentHash").replaceAll("fixturePath", "catalogPath");
    after = after.replace('"contentState", "catalogPath", "catalogContentHash",', '"contentState",');
    after = after.replace("|manifestPath|catalogPath|serializedInputPath", "|manifestPath|serializedInputPath");
    after = after.replaceAll("sentinel cases authority fixture", "sentinel cases authority catalog").replaceAll("sentinel cases fixture", "sentinel cases catalog").replaceAll("Serialized sentinel fixture bytes", "Serialized sentinel catalog bytes");
  }
  if (after !== before) { writeFileSync(join(root, path), after); modified.push(path); }
}
writeFileSync(join(ticket, "📓️normalization-runtime-catalogs-2026-09-09.md"), "# Normalization Runtime Catalog Assets\n\nIndependent audit traced two more library JSON files to synchronous Clean plan, validation and apply reads. They are current normalization authority catalogs and now live under assets, retaining exact bytes. Constants and serialized sentinel authority coordinates now call them catalogs. The sentinel authority uses catalogPath/catalogContentHash consistently; no legacy field fallback was added. Test reads follow the same current paths. Runtime verification follows below.\n\n## Preserved Moves\n\n\x60\x60\x60json\n" + JSON.stringify(moves, null, 2) + "\n\x60\x60\x60\n\n## Authored Paths\n\n\x60\x60\x60json\n" + JSON.stringify([...new Set([...moves.flatMap(row => [row.old, row.new]), ...modified, relative(root, import.meta.path)])].sort(), null, 2) + "\n\x60\x60\x60\n");
console.log("[DEBUG] " + JSON.stringify({ moves, modified }));

