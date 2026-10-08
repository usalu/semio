import { createHash } from "node:crypto";
import { createReadStream, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { init, parse } from "es-module-lexer";

const workspace = resolve(import.meta.dirname, "../../../../../../../..");
const pages = resolve(process.argv[3] ?? join(workspace, "🏢️semio-tech/🎡️play/dist/pages"));
const report = join(import.meta.dirname, "../📓️published-page-completeness.md");
if (!["audit", "verify-hashes"].includes(process.argv[2] ?? "")) throw new Error("Use audit [pages-directory] or verify-hashes");
const hosts = { play: "play.semio-tech.com", map: "map.assets.semio-tech.com", media: "media.assets.semio-tech.com", modules: "modules.assets.semio-tech.com" };
const failures: string[] = [], imports: Record<string, number> = {}, summaries: string[] = [];
let cancelled = false, scanned = 0, verifiedCoreDigests = 0;
process.once("SIGINT", () => { cancelled = true; });
process.once("SIGTERM", () => { cancelled = true; });
if (process.argv[2] === "verify-hashes") { await verifySyntheticCoreDigests(); process.exit(0); }
await init;

function files(directory: string): string[] {
  if (!existsSync(directory)) return [];
  return readdirSync(directory, { withFileTypes: true }).flatMap(entry => entry.isDirectory() ? files(join(directory, entry.name)) : entry.isFile() ? [join(directory, entry.name)] : []);
}

function asset(url: URL): string | undefined {
  const page = Object.entries(hosts).find(([, host]) => url.origin === `https://${host}`)?.[0];
  if (!page) return undefined;
  return join(pages, page, ...(url.pathname === "/" ? ["index.html"] : url.pathname.split("/").slice(1).map(decodeURIComponent)));
}

function check(specifier: string, file: string, host: string): void {
  if (specifier.startsWith("#") || specifier.startsWith("data:") || specifier.startsWith("blob:")) return;
  if (!/^(?:\.{0,2}\/|[a-z][a-z0-9+.-]*:)/iu.test(specifier)) { failures.push(`Unresolved bare module ${specifier} in ${relative(pages, file)}`); return; }
  const base = new URL(relative(join(pages, Object.keys(hosts).find(key => hosts[key as keyof typeof hosts] === host)!), file).replaceAll("\\", "/"), `https://${host}/`);
  try {
    const target = asset(new URL(specifier, base));
    if (target && (!existsSync(target) || !statSync(target).isFile())) failures.push(`Missing ${specifier} in ${relative(pages, file)}`);
  } catch { failures.push(`Invalid URL ${specifier} in ${relative(pages, file)}`); }
}

for (const [page, host] of Object.entries(hosts)) {
  const directory = join(pages, page), entries = files(directory), bytes = entries.reduce((sum, file) => sum + statSync(file).size, 0);
  const cname = join(directory, "CNAME"), headers = join(directory, "_headers");
  if (!existsSync(cname) || readFileSync(cname, "utf8").trim() !== host) failures.push(`Missing or mismatched CNAME for ${page}`);
  if (!existsSync(join(directory, ".nojekyll"))) failures.push(`Missing .nojekyll for ${page}`);
  if (!existsSync(headers) || !readFileSync(headers, "utf8").includes("Cache-Control: no-cache")) failures.push(`Missing stable asset revalidation metadata for ${page}`);
  if (bytes >= 1_000_000_000) failures.push(`${page} reaches the 1 GB limit: ${bytes}`);
  if (page !== "play" && (!existsSync(headers) || !readFileSync(headers, "utf8").includes("Access-Control-Allow-Origin: *"))) failures.push(`Missing satellite CORS metadata for ${page}`);
  if (page === "play" && (!existsSync(join(directory, "index.html")) || !entries.some(file => file.endsWith(".js")))) failures.push("Play app HTML or JavaScript is missing");
  const kinds = Object.fromEntries([".js", ".wasm", ".woff2", ".png", ".pbf"].map(extension => [extension, entries.filter(file => file.endsWith(extension)).length]));
  summaries.push(`| ${page} | ${host} | ${entries.length} | ${bytes} | ${JSON.stringify(kinds)} |`);
  for (const file of entries) {
    if (cancelled) break;
    if (!/\.(?:js|css|html)$/u.test(file)) continue;
    const source = readFileSync(file, "utf8");
    if (file.endsWith(".js")) {
      let rows: ReturnType<typeof parse>[0] = [];
      try { rows = parse(source)[0]; } catch (error) { failures.push(`Invalid published JavaScript ${relative(pages, file)}: ${String(error)}`); }
      for (const row of rows) {
        if (row.d === -2) continue;
        const key = `${row.d === -1 ? "static" : "dynamic"}:${row.n === undefined ? "expression" : row.n.startsWith(".") ? "relative" : row.n.startsWith("/") ? "root" : "absolute"}`;
        imports[key] = (imports[key] ?? 0) + 1;
        if (row.n !== undefined) check(row.n, file, host);
      }
      for (const match of source.matchAll(/new URL\(\s*(["'])([^"']+)\1\s*,\s*import\.meta\.url\s*\)/gu)) check(match[2]!, file, host);
      for (const match of source.matchAll(/__semioVersionedComponentAssetUrl\(\s*(["'])([^"']+)\1\s*\)/gu)) check(match[2]!, file, host);
    }
    if (/\.(?:css|html)$/u.test(file)) {
      for (const match of source.matchAll(/url\(\s*["']?([^\s"')]+)["']?\s*\)/gu)) check(match[1]!, file, host);
      if (file.endsWith(".html")) for (const match of source.matchAll(/<(?:script|link|img|source)\b[^>]*\b(?:src|href)=["']([^"']+)["']/gu)) check(match[1]!, file, host);
    }
    if (++scanned % 100 === 0) { console.log(`[DEBUG] Audited ${scanned} published scripts/styles/documents`); await new Promise(resolve => setTimeout(resolve, 0)); }
  }
}
for (const directory of ["osm", "vt", "dem"]) if (files(join(pages, "map", directory)).length === 0) failures.push(`Empty or missing map family ${directory}`);
const worker = "🔌️plugin-modules/🧵️shard/🟨️shard-worker.js";
if (!existsSync(join(pages, "play", worker)) || !existsSync(join(pages, "modules", worker))) failures.push("Same-origin shard worker or satellite worker is missing");
else if (readFileSync(join(pages, "play", worker), "utf8") !== readFileSync(join(pages, "modules", worker), "utf8")) failures.push("Play and satellite shard-worker contents differ");
try {
  const runtime = await import(pathToFileURL(join(workspace, "🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🟦️.ts")).href);
  const layout = runtime.playRuntimeModuleLayout();
  for (const [route, directories] of [["🔌️plugin-modules", layout.pluginModuleDirNames], ["🧩️extension-modules", layout.extensionModuleDirNames]] as const) {
    for (const directory of directories) {
      const root = join(pages, "modules", route, directory), entries = files(root);
      if (entries.length === 0) failures.push(`Empty or missing declared runtime directory ${route}/${directory}`);
      if (!["🪞️vendor", "🧵️shard"].includes(directory)) {
        if (!existsSync(join(root, "🌉️bridge.js"))) failures.push(`Missing runtime bridge ${route}/${directory}`);
        if (!entries.some(file => file.endsWith(".wasm"))) failures.push(`Missing runtime Wasm ${route}/${directory}`);
        try { await verifyPublishedCoreDigest(root); verifiedCoreDigests++; console.log(`[DEBUG] Verified shipped descriptor primary core ${route}/${directory}`); }
        catch (error) { failures.push(`Descriptor/core digest failure ${route}/${directory}: ${String(error)}`); }
      }
    }
  }
} catch (error) { failures.push(`Runtime closure could not be admitted: ${String(error)}`); }
if (cancelled) failures.push("Audit cancelled before completion");
writeFileSync(report, `# Published Page Completeness\n\nAudited ${new Date().toISOString()} against ${pages}.\n\n| Page | Host | Files | Bytes | Asset Counts |\n| --- | --- | ---: | ---: | --- |\n${summaries.join("\n")}\n\nVerified ${verifiedCoreDigests} descriptor primary-core SHA256 pairs from final shipped bytes.\n\nImport forms parsed independently with es-module-lexer: ${JSON.stringify(imports)}.\n\n${failures.length ? `## Failures (${failures.length})\n\n${failures.map(failure => `- ${failure}`).join("\n")}` : "All completeness, metadata, byte-budget, literal-import and descriptor/core digest checks passed."}\n\nHTTP hosting behavior and runtime expression imports still require the production browser harness.\n`);
console.log(`[DEBUG] Publication audit ${failures.length ? "failed" : "passed"}: ${report}`);
process.exitCode = failures.length ? 1 : 0;

/** 🔏️ Checks the descriptor finalizer's existing primary-core digest relation. */
async function verifyPublishedCoreDigest(directory: string): Promise<void> {
  const descriptor = JSON.parse(readFileSync(join(directory, "🔣️.json"), "utf8"));
  if (!/^[a-f0-9]{64}$/u.test(descriptor.hashes?.coreWasmSha256 ?? "")) throw new Error("Invalid descriptor core digest");
  const primary = readdirSync(directory).filter(name => name.endsWith(".core.wasm"));
  if (primary.length !== 1) throw new Error("Expected exactly one descriptor primary core");
  const digest = createHash("sha256");
  for await (const chunk of createReadStream(join(directory, primary[0]!))) {
    if (cancelled) throw new Error("Descriptor digest audit cancelled");
    digest.update(chunk);
  }
  if (digest.digest("hex") !== descriptor.hashes.coreWasmSha256) throw new Error("Shipped primary core differs from descriptor SHA256");
}

/** 🧪️ Uses neutral synthetic metadata and an independent SHA implementation before final inventory. */
async function verifySyntheticCoreDigests(): Promise<void> {
  const { strict: assert } = await import("node:assert"), { sha256 } = await import("@noble/hashes/sha2.js");
  const { default: Ajv } = await import("ajv"), { default: fixture } = await import("./🔣️.json"), { default: schema } = await import("./🧬️schema.json");
  assert.equal(new Ajv({ strict: true }).compile(schema)(fixture), true);
  const root = resolve(import.meta.dirname, "../🗑️generated/descriptor-digest-proof");
  rmSync(root, { recursive: true, force: true });
  try {
    for (const row of fixture.cases) {
      const directory = join(root, row.id); mkdirSync(directory, { recursive: true });
      const oracle = Buffer.from(sha256(new TextEncoder().encode(row.content))).toString("hex");
      assert.equal(oracle === row.declared && row.writeCore, row.accepted);
      writeFileSync(join(directory, "🔣️.json"), JSON.stringify({ hashes: { coreWasmSha256: row.declared } }));
      if (row.writeCore) writeFileSync(join(directory, fixture.coreName), row.content);
      let accepted = true;
      try { await verifyPublishedCoreDigest(directory); } catch { accepted = false; }
      assert.equal(accepted, row.accepted, row.id);
      console.log(`[DEBUG] Synthetic descriptor core digest ${row.id} accepted=${accepted} independentSHA=${oracle}`);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}
