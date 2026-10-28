// [cachegc] End-to-end probe of the closure read worker: (1) door-equivalent capture of the cache kernel (the parent's admitted resolution map), (2) the worker snapshot built by the rule of captureBuildSourceClosureV1 for the "closure" scope (base) and for closureReadWorkerScopeV1 (batch), each passed to the REAL buildSourceClosureNodeArgumentsV1, (3) the static one executed by the REAL Node loader + worker: readiness, one inference record read through the session protocol, clean EOF. usage: bun worker-e2e.ts <root> [static|closure] [scope root: the tree whose session module names the static scope, default <root>] [image dir]. Reads <root> only; writes only the probe image. Not a library file.
import { createHash } from "node:crypto";
import { spawn } from "node:child_process";
import { mkdirSync, rmSync, writeFileSync, lstatSync, readFileSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";
import { brotliCompressSync } from "node:zlib";
const [root, mode = "static", scopeRoot = root, imageDir = join(root, ".cache", "cgprobe")] = process.argv.slice(2), L = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/", C = L + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/";
const kernel = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/💾️cache/🟦️.ts", worker = relative(root, C + "👷️closure-read-worker/🟨️.mjs");
const auth = await import(L + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts"), closure = await import(L + "🏗️build/📥️source/🔏️closure/🟦️.ts"), node = await import(L + "🏗️build/📥️source/🔏️closure/🟢️node/🟨️.mjs");
const sourceSchema = JSON.parse(readFileSync(L + "🏗️build/📥️source/🔏️closure/🧬️schema/🔣️.json", "utf8")), P = JSON.parse(readFileSync(C + "🧬️schema/🔣️.json", "utf8")).definitions.Protocol.const;
const captured = await auth.captureNxAuthorizerSourcesV1(root, kernel, new AbortController().signal, { installed: true });
const bytes: Map<string, Uint8Array> = auth.nxAuthorizerCapturedBytesV1(captured), admitted: Map<string, Record<string, string>> = auth.nxAuthorizerCapturedImportsV1(root, captured);
const installed = (t: string) => /^installed:(?:@[a-z0-9._-]+\/)?[a-z0-9._-]+@[^\s]{1,64}$/u.test(t);
let scope: { kind: string; roots?: string[]; leaves?: string[] } = { kind: "closure" };
if (mode === "static") scope = (await import(scopeRoot + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟦️.ts")).closureReadWorkerScopeV1(scopeRoot);
const schemas = new Map<string, string[]>(); for (const path of admitted.keys()) { const parts = path.split("/"), i = parts.indexOf("🧬️schema"); if (i >= 0 && extname(path) === ".json") { const p = parts.slice(0, i).join("/"); schemas.set(p, [...(schemas.get(p) ?? []), path]); } }
const files = new Map<string, Uint8Array>(), pending = [worker, ...(scope.kind === "static" ? scope.roots! : [])];
while (pending.length) {
  const path = pending.pop()!; if (files.has(path)) continue; const b = bytes.get(path); if (!b) throw Error("[DEBUG] closure.source-not-admitted " + path); files.set(path, b); if (extname(path) === ".json") continue;
  const text = Buffer.from(b).toString("utf8"), followed = scope.kind === "static" ? closure.staticImportSpecifiersV1(text) as Set<string> : undefined;
  for (const t of schemas.get(dirname(path) === "." ? "" : dirname(path)) ?? []) pending.push(t);
  for (const [s, t] of Object.entries(admitted.get(path)!)) { if (followed && !followed.has(s)) continue; if (!installed(t)) pending.push(t); }
  for (const m of text.matchAll(/(["'])(\.{1,2}\/[^"'\\\r\n]+\.(?:[cm]?[jt]sx?|json))\1/gu)) { const s = m[2]!; if (/\.d\.[cm]?ts$/u.test(s) || followed && !s.endsWith(".json")) continue; const c = relative(root, resolve(root, dirname(path), s)).replaceAll("\\", "/"); if (admitted.has(c)) pending.push(c); }
}
const snapshot = { version: 1, root: resolve(root), entry: worker, files: [...files].sort(([a], [b]) => Buffer.compare(Buffer.from(a), Buffer.from(b))).map(([path, b]) => ({ path, sha256: createHash("sha256").update(b).digest("hex"), base64: Buffer.from(b).toString("base64"), imports: Object.fromEntries(Object.entries(admitted.get(path)!).filter(([, t]) => installed(t) || files.has(t))) })) };
const size = Buffer.byteLength(JSON.stringify(snapshot)) + 1;
let args: { argv: string[]; bootstrap: string; digest: string };
try { args = node.buildSourceClosureNodeArgumentsV1(snapshot, sourceSchema.definitions.Policy.const, P.heapMiB, "node"); }
catch (error) { console.log(JSON.stringify({ mode, kernelFiles: admitted.size, workerFiles: files.size, bootstrapBytes: size, refused: String((error as Error).message) })); process.exit(mode === "closure" ? 0 : 1); }
const image = imageDir; rmSync(image, { recursive: true, force: true });
const leaf = join(image, "inference", "selections", "c".repeat(64), "np", "d".repeat(24) + ".json"); mkdirSync(dirname(leaf), { recursive: true });
const record = brotliCompressSync(Buffer.from(JSON.stringify({ digest: "a".repeat(64), resultSha256: createHash("sha256").update("{}").digest("hex"), authorityObservations: [], blocks: [], observations: [] }) + "\n{}")); writeFileSync(leaf, record);
const st = lstatSync(leaf, { bigint: true }), stamp = [st.dev, st.ino, st.mode, st.nlink, st.size, st.mtimeNs, st.ctimeNs].join(":");
const child = spawn("node", args.argv, { cwd: image, stdio: ["pipe", "pipe", "pipe"] }); const exited = new Promise<number | null>(done => child.on("exit", c => done(c))); let out = "", err = ""; child.stdout.on("data", d => { out += d; }); child.stderr.on("data", d => { err += d; });
const line = async (n: number) => { for (let i = 0; i < 600 && out.split("\n").length <= n; i++) await Bun.sleep(50); return out.split("\n")[n - 1] ?? null; };
const t0 = performance.now(); child.stdin.write(args.bootstrap);
const ready = await line(1), readyMs = Math.round(performance.now() - t0);
child.stdin.write(JSON.stringify({ version: 1, maximumReplyBytes: 1 << 20 }) + "\n");
child.stdin.write(JSON.stringify({ files: [{ path: leaf, stamp, kind: "inference", bytes: record.length }], imageRoot: image, remainingBytes: P.maximumProofBytes, root: process.cwd() }) + "\n");
const reply = await line(2); child.stdin.write("null\n"); child.stdin.end();
const code = await Promise.race([exited, Bun.sleep(30000).then(() => { child.kill("SIGKILL"); return -1; })]);
let answer: { rows?: { hash?: string }[] } = {}; try { answer = JSON.parse(reply ?? "{}"); } catch {}
const ok = ready === JSON.stringify({ version: 1, source: args.digest }) && answer.rows?.length === 1 && code === 0;
console.log(JSON.stringify({ mode, kernelFiles: admitted.size, workerFiles: files.size, bootstrapBytes: size, ready: ready === JSON.stringify({ version: 1, source: args.digest }), readyMs, reply: (reply ?? "").slice(0, 300), exit: code, stderr: err.slice(0, 600), ok }));
rmSync(image, { recursive: true, force: true }); process.exit(ok ? 0 : 1);
