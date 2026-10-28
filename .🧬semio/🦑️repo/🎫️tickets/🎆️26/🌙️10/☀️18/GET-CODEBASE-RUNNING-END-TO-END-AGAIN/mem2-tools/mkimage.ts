// [mem2] Captures the snapshot of one entry exactly as the door does (captureNxAuthorizerSourcesV1 with installed packages, same rows as captureCompilerBuildSourceClosureV1, canonical text of buildSourceClosureTextV1) and writes <out>/snapshot.json + <out>/loader.js (the stream loader) + <out>/digest. Not a library file.
// REROOT=<dir>: the snapshot names that root (the images stay the captured ones), so a mirror's implementation runs against the live workspace data exactly as a production child whose snapshot root is the workspace.
// usage: bun mkimage.ts <root> <entry relative to root> <out dir> [patches.json: [{suffix,find,replace}], each find exactly once in the one image whose path ends with suffix]
import { mkdirSync, writeFileSync } from "node:fs";
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { join } from "node:path";
const [root, entry, out, patchFile] = process.argv.slice(2) as [string, string, string, string?];
const patches: { suffix: string; find: string; replace: string }[] = patchFile ? JSON.parse(readFileSync(patchFile, "utf8")) : [];
const L = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const auth = await import(L + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts"), closure = await import(L + "🏗️build/📥️source/🔏️closure/🟦️.ts");
const t0 = performance.now(), captured = await auth.captureNxAuthorizerSourcesV1(root, entry, new AbortController().signal, { installed: true });
const bytes = auth.nxAuthorizerCapturedBytesV1(captured), imports = auth.nxAuthorizerCapturedImportsV1(root, captured);
const applied = new Set<number>();
const files = captured.manifest.sources.map((row: { path: string; sha256: string }) => {
  let data = Buffer.from(bytes.get(row.path)!);
  for (const [index, patch] of patches.entries()) if (row.path.endsWith(patch.suffix)) { const text = data.toString("utf8"); if (text.split(patch.find).length !== 2) throw Error("[DEBUG] patch anchor not exactly once: " + patch.suffix); data = Buffer.from(text.replace(patch.find, patch.replace)); applied.add(index); }
  return Object.freeze({ path: row.path, sha256: createHash("sha256").update(data).digest("hex"), base64: data.toString("base64"), imports: Object.freeze({ ...imports.get(row.path)! }) });
});
if (applied.size !== patches.length) throw Error("[DEBUG] patches not applied: " + patches.filter((_patch, index) => !applied.has(index)).map(patch => patch.suffix).join(", "));
const snapshot = Object.freeze({ version: 1 as const, root: process.env.REROOT ?? root, entry, files: Object.freeze(files) }), { text, sha256 } = closure.buildSourceClosureTextV1(snapshot);
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "snapshot.json"), text, { mode: 0o600 });
writeFileSync(join(out, "loader.js"), closure.buildSourceClosureStreamLoaderV1);
writeFileSync(join(out, "digest"), sha256);
console.log(JSON.stringify({ files: files.length, bytes: Buffer.byteLength(text), sha256, seconds: Math.round(performance.now() - t0) / 1000, farm: files.filter((row: { path: string }) => row.path.includes("🏊️project-farm/")).map((row: { path: string }) => row.path.split("🏊️project-farm/")[1]) }));
