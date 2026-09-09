import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, readdirSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";

const root = process.env.SEMIO_FIXTURE_REPO_ROOT!;
const ticket = dirname(dirname(import.meta.dir));
const os = "🧰️framework/🛍️products/💻️os";
const previous = os + "/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback";
const harness = os + "/🧪️testkit/🧩️jcoprobe/🌐️harness";
const entries = [
  ["🧵️worker.js", os + "/🧪️tests/🧩️jco-callback/🟨️.mjs"],
  ["🌐️.html", harness + "/🌐️.html"],
  ["🖥️host-shim.js", harness + "/🧩️support/🖥️host-shim.js"],
  ...readdirSync(join(root, previous, "🪞️preview2-shim")).map(name => ["🪞️preview2-shim/" + name, harness + "/🧩️support/🪞️preview2-shim/" + name]),
];
const moves = entries.map(([name, destination]) => {
  const old = previous + "/" + name, source = readFileSync(join(root, old));
  mkdirSync(dirname(join(root, destination!)), { recursive: true });
  renameSync(join(root, old), join(root, destination!));
  const digest = (value: Buffer) => createHash("sha256").update(value).digest("hex");
  if (digest(source) !== digest(readFileSync(join(root, destination!)))) throw new Error("Move bytes changed");
  return { old, new: destination!, sha256: digest(source) };
});
const modified = new Set<string>();
function update(path: string, transform: (source: string) => string) {
  const before = readFileSync(join(root, path), "utf8"), after = transform(before);
  if (after !== before) { writeFileSync(join(root, path), after); modified.add(path); }
}
const worker = moves[0]!.new, page = moves[1]!.new;
update(worker, source => source.replace('"./jcoprobe.js"', JSON.stringify(relative(dirname(worker), previous + "/jcoprobe.js"))));
update(page, source => source.replace('"./🧵️worker.js"', JSON.stringify("/" + relative(os, worker))));
for (const variant of ["📞️out-callback", "⚡️out-jspi-explicit"]) {
  const path = os + "/🧫️fixtures/🧩️jcoprobe/🌐️harness/" + variant + "/jcoprobe.js";
  update(path, source => source.replace(/from '([^']+)'/g, (match, value) => {
    const old = join(dirname(path), value).replaceAll("\\", "/");
    const target = moves.find(row => row.old === old);
    return target ? "from '" + relative(dirname(path), target.new).replaceAll("\\", "/") + "'" : match;
  }));
}
const taxonomy = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
update(taxonomy, source => {
  for (const row of moves) source = source.replaceAll(row.old, row.new);
  return source;
});
const report = "# JCO Browser Fixture Classification Follow-up\n\nIndependent coordinator inspection found handwritten worker assertions, a browser page, host shim and vendored preview shims mixed with generated guest examples. They are executable test/support code. The assertions now occupy a canonical OS test case; the page and shims belong to JCO testkit. Generated guest examples remain fixtures; their imports now reach the physical testkit support. Every initial move preserved SHA-256. Runtime validation follows below.\n\n## Preserved Moves\n\n\x60\x60\x60json\n" + JSON.stringify(moves, null, 2) + "\n\x60\x60\x60\n\n## Authored Paths\n\n\x60\x60\x60json\n" + JSON.stringify([...new Set([...moves.flatMap(row => [row.old, row.new]), ...modified, harness + "/📜️script.ts", relative(root, import.meta.path)])].sort(), null, 2) + "\n\x60\x60\x60\n";
writeFileSync(join(ticket, "📓️jco-browser-fixture-classification-2026-09-09.md"), report);
console.log("[DEBUG] " + JSON.stringify({ moves: moves.length, modified: [...modified] }));

