import { createReadStream, lstatSync, readFileSync } from "node:fs";
import { extname, isAbsolute, join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { IncomingMessage, ServerResponse } from "node:http";

const workspace = process.cwd();
const { MODULE_PLUGIN_ROUTE } = await import(pathToFileURL(resolve(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts")).href);
const products = resolve(workspace, ".cache/b/p/m");
const support = join(products, "browser", "prototype", "modules");
const generated = join(import.meta.dirname, "🗑️generated");
const rawGuest = join(generated, "guest");
const descriptors = join(generated, "bim-descriptor-current");
const fonts = join(products, "infinite", "prototype", "fonts");

const types = new Map([[".js", "text/javascript"], [".wasm", "application/wasm"], [".json", "application/json"]]);

export function rawGuestIsReady(): boolean {
  const linked = join(rawGuest, "linked-component.sha256"), prepared = join(rawGuest, "prepared-component.sha256"), described = join(descriptors, "component.sha256");
  for (const file of [linked, prepared, described, join(rawGuest, "bridge.js"), join(rawGuest, "bim.js"), join(descriptors, "🛂️.descriptor.semio"), join(descriptors, "🔣️.json")]) {
    if (!lstatSync(file, { throwIfNoEntry: false })?.isFile()) return false;
  }
  const digest = readFileSync(linked, "utf8").trim();
  return /^[0-9a-f]{64}$/u.test(digest) && readFileSync(prepared, "utf8").trim() === digest && readFileSync(described, "utf8").trim() === digest;
}

export function bimShellModuleMiddleware(req: IncomingMessage, res: ServerResponse, next: () => void): void {
  const pathname = decodeURIComponent(new URL(req.url ?? "/", "http://127.0.0.1").pathname);
  if (!pathname.startsWith(MODULE_PLUGIN_ROUTE + "/")) return next();
  const [directory, ...segments] = pathname.slice(MODULE_PLUGIN_ROUTE.length + 1).split("/");
  if (!segments.length || segments.some(segment => !segment || segment.startsWith(".") || segment.includes("\\"))) return next();
  const requested = segments.join("/");
  let root = support, name = directory + "/" + requested;
  if (directory === "🏬️bim") {
    root = rawGuest;
    name = requested;
    if (requested === "🌉️bridge.js") name = "bridge.js";
    else if (requested === "🛂️.descriptor.semio" || requested === "🔣️.json") root = descriptors;
  } else if (directory === "🪞️vendor" && requested === "🔤️guestslim-typst-fonts.bin") {
    root = fonts;
    name = requested;
  } else if (directory !== "🧵️shard" && directory !== "🪞️vendor") return next();
  const path = resolve(root, name), rel = relative(root, path);
  if (!rel || rel.startsWith("..") || isAbsolute(rel)) return next();
  const stat = lstatSync(path, { throwIfNoEntry: false });
  if (!stat?.isFile()) {
    res.statusCode = 404;
    res.end("Diagnostic browser module is unavailable");
    return;
  }
  res.setHeader("Content-Type", types.get(extname(path)) ?? "application/octet-stream");
  res.setHeader("Content-Length", stat.size);
  res.setHeader("Cache-Control", "no-store");
  if (req.method === "HEAD") res.end();
  else {
    const stream = createReadStream(path);
    res.once("close", () => stream.destroy());
    stream.once("error", error => res.destroy(error));
    stream.pipe(res);
  }
}
