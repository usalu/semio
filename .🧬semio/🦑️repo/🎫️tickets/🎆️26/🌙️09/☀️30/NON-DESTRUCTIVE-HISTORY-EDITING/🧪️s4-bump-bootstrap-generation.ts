/**
 * 🧬️ S4-BUMP: computes the trusted stdio+GIS bootstrap profile generation with the hub's OWN framing oracle
 * (`trustedBootstrapProfileEncoding` + `projectTrustedBootstrapCodecsV1` in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`), loaded in memory:
 * the oracle functions (and the top-level functions/constants they reach) are lifted from the hub script source, transpiled and evaluated,
 * so no framing is re-implemented here. The `trusted-stdio-gis-bundle-check --source` route that prints the same value is blocked by a
 * stale central schema catalog (coordinator `schema generate`). It first proves the lift by re-deriving the committed generation with the
 * fixture's literals set back to `--from`, then prints the generation at the fixture's current literals. `--receipts-rev <rev>` reads the
 * codec receipt sources at a git revision (the closure the committed generation was sealed over); `--oracle-rev <rev>` lifts the oracle
 * from the hub script at that revision.
 */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

const ROOT = "/Users/ueli/Documents/semio";
const oracleRevision = process.argv.includes("--oracle-rev") ? process.argv[process.argv.indexOf("--oracle-rev") + 1]! : null;
const source = oracleRevision === null ? readFileSync(join(ROOT, "🌎️hub/📦️packages/🦀️rust/📜️script.ts"), "utf8") : spawnSync("git", ["-c", "core.quotePath=false", "show", `${oracleRevision}:🌎️hub/📦️packages/🦀️rust/📜️script.ts`], { cwd: ROOT, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 }).stdout;
const declarations = new Map<string, string>();
for (const match of source.matchAll(/^(?:async )?function ([A-Za-z0-9_]+)|^(?:const|let) ([A-Za-z0-9_]+)\s*(?::[^=]+)?=/gmu)) {
  const name = (match[1] ?? match[2])!;
  const start = match.index!;
  const lineEnd = source.indexOf("\n", start);
  const firstLine = source.slice(start, lineEnd);
  const closing = match[1] ? source.indexOf("\n}\n", start) + 2 : firstLine.trimEnd().endsWith(";") ? lineEnd : Math.min(...["\n};", "\n];", "\n});"].map((marker) => source.indexOf(marker, start)).filter((index) => index >= 0)) + 3;
  const end = match[1] ? closing : closing;
  if (!declarations.has(name)) declarations.set(name, source.slice(start, end));
}
const wanted = new Set<string>();
const visit = (name: string): void => {
  if (wanted.has(name) || !declarations.has(name)) return;
  wanted.add(name);
  for (const token of declarations.get(name)!.matchAll(/[A-Za-z_][A-Za-z0-9_]*/gu)) if (token[0] !== name && /^(?:project)?[Tt]rustedBootstrap|^TRUSTED_BOOTSTRAP|^documentOpenNeutral/u.test(token[0])) visit(token[0]);
};
visit("trustedBootstrapProfileEncoding");
visit("projectTrustedBootstrapCodecsV1");
const ordered = [...declarations.keys()].filter((name) => wanted.has(name));
const transpiler = new Bun.Transpiler({ loader: "ts" });
const code = transpiler.transformSync(ordered.map((name) => declarations.get(name)!).join("\n\n"));
const oracle = new Function("createHash", "Buffer", `${code}\nreturn { trustedBootstrapProfileEncoding, projectTrustedBootstrapCodecsV1 };`)(createHash, Buffer) as {
  trustedBootstrapProfileEncoding: (profile: unknown, codecs: unknown) => Buffer;
  projectTrustedBootstrapCodecsV1: (stdio: unknown, gis: unknown) => { codecs: unknown };
};

const fixture = JSON.parse(readFileSync(join(ROOT, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json"), "utf8"));
const revision = process.argv.includes("--receipts-rev") ? process.argv[process.argv.indexOf("--receipts-rev") + 1]! : null;
const receipt = (path: string): unknown =>
  JSON.parse(revision === null ? readFileSync(join(ROOT, path), "utf8") : spawnSync("git", ["-c", "core.quotePath=false", "show", `${revision}:${path}`], { cwd: ROOT, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 }).stdout);
const sources = revision === null ? fixture.sources : (receipt("🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json") as typeof fixture).sources;
const codecs = oracle.projectTrustedBootstrapCodecsV1(receipt(sources.stdioReceipts), receipt(sources.gisReceipts)).codecs;
const from = Number(process.argv[process.argv.indexOf("--from") + 1]);
const atVersion = (version: number) => {
  const profile = structuredClone(fixture.profile);
  for (const record of profile.packages) record.executionProtocol.appChannelVersion = version;
  return createHash("sha256").update(oracle.trustedBootstrapProfileEncoding(profile, codecs)).digest("hex");
};
const proof = atVersion(from);
if (proof !== fixture.profile.generationId) throw new Error(`lifted oracle does not re-derive the committed generation: ${proof} ≠ ${fixture.profile.generationId}`);
console.log(`[bootstrap-generation] lifted ${ordered.length} declarations (${ordered.join(", ")}); v${from} re-derives ${proof}`);
console.log(`[bootstrap-generation] current ${createHash("sha256").update(oracle.trustedBootstrapProfileEncoding(fixture.profile, codecs)).digest("hex")}`);
