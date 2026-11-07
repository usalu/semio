import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const generated = join(process.env.SEMIO_TICKET_DIR!, "🗑️generated"), manifestBytes = readFileSync(join(generated, "bazel-msvc-manifest.json"));
const hash = (data: Uint8Array) => createHash("sha256").update(data).digest("hex");
if (hash(manifestBytes) !== "a8e854fa9f3fbaeac27d7304e297f1660d7137401389deb57cf7983f33c16fef") throw Error("Untrusted manifest");
const packages = JSON.parse(manifestBytes.toString("utf8")).packages.filter((item: { id: string }) => item.id === "Microsoft.VC.14.50.18.0.CRT.x64.Store.base");
if (packages.length !== 1 || packages[0].payloads.length !== 1) throw Error("Ambiguous CRT package");
const payload = packages[0].payloads[0], url = new URL(payload.url);
if (url.protocol !== "https:" || url.hostname !== "download.visualstudio.microsoft.com" || payload.sha256 !== "2ed606cdc1cb7e5d311ec19364b7df9e9932817ebfed761ae358cd438936dd57") throw Error("CRT payload refused");
const head = await fetch(url, { method: "HEAD", signal: AbortSignal.timeout(30_000) }), length = Number(head.headers.get("content-length"));
if (!head.ok || length < 1 || length > 60_000_000) throw Error("CRT payload size refused");
const response = await fetch(url, { signal: AbortSignal.timeout(120_000) });
if (!response.ok || new URL(response.url).hostname !== url.hostname || Number(response.headers.get("content-length")) !== length) throw Error("CRT payload length changed");
const data = new Uint8Array(await response.arrayBuffer());
if (data.length !== length || hash(data) !== payload.sha256) throw Error("CRT payload hash mismatch");
writeFileSync(join(generated, "msvc-4.vsix"), data, { flag: "wx" });
console.log(JSON.stringify({ declaredSize: payload.size, actualSize: data.length, sha256: hash(data) }));
