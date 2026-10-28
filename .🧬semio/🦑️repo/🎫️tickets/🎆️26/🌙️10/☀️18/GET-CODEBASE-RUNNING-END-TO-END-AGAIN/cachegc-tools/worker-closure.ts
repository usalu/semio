// [cachegc] Simulates captureBuildSourceClosureV1 ("closure" scope) of the closure-read worker inside a door-equivalent capture of a kernel entry: the same admitted resolution map (nxAuthorizerCapturedImportsV1), the same module-owned schema groups and quoted literal rule. usage: bun worker-closure.ts <root> [kernel entry] [worker entry]
import { createHash } from "node:crypto";
import { dirname, extname, relative, resolve } from "node:path";
const [root, kernel = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/💾️cache/🟦️.ts", worker = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/👷️closure-read-worker/🟨️.mjs"] = process.argv.slice(2);
const L = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const auth = await import(L + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts");
const captured = await auth.captureNxAuthorizerSourcesV1(root, kernel, new AbortController().signal, { installed: true });
const bytes: Map<string, Uint8Array> = auth.nxAuthorizerCapturedBytesV1(captured), imports: Map<string, Record<string, string>> = auth.nxAuthorizerCapturedImportsV1(root, captured);
const installed = (t: string) => t.startsWith("installed:"), closureModule = await import(L + "🏗️build/📥️source/🔏️closure/🟦️.ts"), STATIC = process.env.STATIC === "1";
const schemas = new Map<string, string[]>();
for (const path of imports.keys()) { const parts = path.split("/"), i = parts.indexOf("🧬️schema"); if (i >= 0 && extname(path) === ".json") { const p = parts.slice(0, i).join("/"); schemas.set(p, [...(schemas.get(p) ?? []), path]); } }
const roots = (process.env.ROOTS ?? "").split(",").filter(Boolean), files = new Map<string, Uint8Array>(), pending = [worker, ...roots], via = new Map<string, string>();
const push = (from: string, to: string, how: string) => { if (!files.has(to) && !via.has(to)) via.set(to, how + " " + from); pending.push(to); };
while (pending.length) {
  const path = pending.pop()!; if (files.has(path)) continue; const b = bytes.get(path); if (!b) throw Error("[DEBUG] not admitted " + path); files.set(path, b);
  if (extname(path) === ".json") continue;
  for (const t of schemas.get(dirname(path) === "." ? "" : dirname(path)) ?? []) push(path, t, "schema");
  const followed = STATIC ? closureModule.staticImportSpecifiersV1(Buffer.from(b).toString("utf8")) as Set<string> : undefined;
  for (const [spec, t] of Object.entries(imports.get(path)!)) if (!installed(t) && (!followed || followed.has(spec))) push(path, t, "import");
  for (const m of Buffer.from(b).toString("utf8").matchAll(/(["'])(\.{1,2}\/[^"'\\\r\n]+\.(?:[cm]?[jt]sx?|json))\1/gu)) { if (/\.d\.[cm]?ts$/u.test(m[2]) || STATIC && !m[2].endsWith(".json")) continue; const c = relative(root, resolve(root, dirname(path), m[2])).replaceAll("\\", "/"); if (imports.has(c)) push(path, c, "literal"); }
}
let raw = 0, json = 0; for (const [p, b] of files) { raw += b.length; json += Math.ceil(b.length / 3) * 4 + Buffer.byteLength(p) + 64 + 120 + JSON.stringify(imports.get(p)).length; }
if (process.env.DYN) for (const [p, b] of files) { const t = Buffer.from(b).toString("utf8"), d = [...t.matchAll(/\bimport\s*\(/gu)].length, r = [...t.matchAll(/\brequire\s*\(/gu)].length; if (d || r) console.log("dynamic", d, r, p.split("/").slice(-4).join("/")); }
console.log(JSON.stringify({ static: STATIC, kernelFiles: imports.size, workerFiles: files.size, rawBytes: raw, approxJsonBytes: json, channelLimit: 16777216 }));
const how = new Map<string, [number, number]>(); for (const [p, b] of files) { const k = (via.get(p) ?? "entry").split(" ")[0]; const v = how.get(k) ?? [0, 0]; how.set(k, [v[0] + 1, v[1] + b.length]); } console.log("by edge kind", JSON.stringify([...how]));
const lit = [...files].filter(([p]) => via.get(p)?.startsWith("literal")).sort((a, b) => b[1].length - a[1].length).slice(0, 12); for (const [p, b] of lit) console.log("literal", b.length, p.slice(-90), "<-", via.get(p)!.slice(-90));
const sch = [...files].filter(([p]) => via.get(p)?.startsWith("schema")).sort((a, b) => b[1].length - a[1].length).slice(0, 12); for (const [p, b] of sch) console.log("schema", b.length, p.slice(-90), "<-", via.get(p)!.slice(-90));
for (const target of (process.env.CHAIN ?? "").split(",").filter(Boolean)) { let p = [...files.keys()].find(k => k.endsWith(target)); console.log("== chain to", target); while (p && via.has(p)) { const [kind, ...rest] = via.get(p)!.split(" "); console.log(" ", kind, p.split("/").slice(-5).join("/")); p = rest.join(" "); } if (p) console.log("  entry", p.split("/").slice(-5).join("/")); }
if (process.env.SHOW) for (const p of process.env.SHOW.split(",")) { const k = [...imports.keys()].find(x => x.endsWith(p)); console.log("imports of", k, JSON.stringify(k && imports.get(k))); }
