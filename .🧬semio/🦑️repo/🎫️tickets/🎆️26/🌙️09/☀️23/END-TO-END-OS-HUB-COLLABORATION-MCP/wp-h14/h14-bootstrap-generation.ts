/** 🧮️ H14 14b (ticket-local, one-off): derives the stdio+GIS bootstrap fixture's `profile.generationId` with the hub script's
 * OWN derivation (`trustedBootstrapProfileEncoding` over `projectTrustedBootstrapCodecsV1` of the fixture's receipts, exactly
 * as `proveTrustedStdioGisBootstrapFixture` checks it), for the channel versions given on the command line. The hub script is
 * loaded from a rewritten copy (relative imports made absolute, the two functions exported, its main guard dropped) so no
 * tree file changes. Usage: bun h14-bootstrap-generation.ts [--write] <version…>; `--write` stores the first version's id
 * into the fixture (generationId + every rotation field that quoted the previous id). */
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const repo = "/Users/ueli/Documents/semio";
const scriptPath = join(repo, "🌎️hub/📦️packages/🦀️rust/📜️script.ts");
const fixturePath = join(repo, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json");
const base = dirname(scriptPath);
const guard = 'if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });';
let source = readFileSync(scriptPath, "utf8");
if (!source.includes(guard)) throw new Error("hub script main guard moved");
source = source.replace(/(from\s+|import\()"(\.{1,2}\/[^"]+)"/gu, (_, head: string, specifier: string) => `${head}"${resolve(base, specifier)}"`);
source = source.replace(guard, "export { trustedBootstrapProfileEncoding, projectTrustedBootstrapCodecsV1 };");
const copyDirectory = join(repo, ".tmp-ticket/wp-h14/🗑️generated/hub-script-copy");
mkdirSync(copyDirectory, { recursive: true });
const copyPath = join(copyDirectory, "hub-script.ts");
writeFileSync(copyPath, source);
const hub = await import(copyPath);
const write = process.argv.includes("--write");
const versions = process.argv.slice(2).filter((argument) => argument !== "--write").map(Number);
const text = readFileSync(fixturePath, "utf8");
const fixture = JSON.parse(text);
const stdio = JSON.parse(readFileSync(join(repo, fixture.sources.stdioReceipts), "utf8"));
const gis = JSON.parse(readFileSync(join(repo, fixture.sources.gisReceipts), "utf8"));
const codecs = hub.projectTrustedBootstrapCodecsV1(stdio, gis).codecs;
const derived = versions.map((version) => {
  const profile = structuredClone(fixture.profile);
  for (const record of profile.packages) record.executionProtocol.appChannelVersion = version;
  return { version, generationId: createHash("sha256").update(hub.trustedBootstrapProfileEncoding(profile, codecs)).digest("hex") };
});
console.log(JSON.stringify({ stored: fixture.profile.generationId, derived }));
if (write) {
  const [{ version, generationId }] = derived;
  if (!fixture.profile.packages.every((record: any) => record.executionProtocol.appChannelVersion === version)) throw new Error("fixture packages do not state the derived version");
  const previous = fixture.profile.generationId;
  const occurrences = text.split(previous).length - 1;
  writeFileSync(fixturePath, text.replaceAll(previous, generationId));
  console.log(JSON.stringify({ replaced: previous, with: generationId, occurrences }));
}
