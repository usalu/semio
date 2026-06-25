import * as fs from "node:fs";
import * as path from "node:path";

const ROOT = path.resolve(import.meta.dirname, "../../../..");
const EXCLUDE_DIRS = new Set([".git", "node_modules", "target", ".nx", "dist", "storybook-static", "pkg"]);

const REPLACEMENTS: [string, string][] = [
  ["@semio-tech/semio-asset", "@semio-tech/semio-asset"],
  ["@semio-tech/semio-icon", "@semio-tech/semio-icon"],
  ["@semio-tech/semio-logo", "@semio-tech/semio-logo"],
  ["@semio-tech/semio-image", "@semio-tech/semio-image"],
  ["asset/", "asset/"],
  ["asset", "asset"],
  ["repo-semio", "repo-semio"],
  ["semio_horizontal_dark", "semio_horizontal_dark"],
  ["semio_horizontal", "semio_horizontal"],
  ["semio_socialpreview", "semio_socialpreview"],
  ["semio_codeicon", "semio_codeicon"],
  ["semio_inkscape", "semio_inkscape"],
  ["semiofile_deserialize", "semiofile_deserialize"],
  ["semiofile_serialize", "semiofile_serialize"],
  ['<title>semio</title>', "<title>semio</title>"],
  ['<title id="title1">semio</title>', '<title id="title1">semio</title>'],
  ["usalu/semio", "usalu/semio"],
  ["The semio monorepo asset bundle.", "The semio monorepo asset bundle."],
  ["semio colors", "semio colors"],
];

function walk(dir: string, callback: (file: string) => void) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    if (EXCLUDE_DIRS.has(entry.name)) continue;
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      if (full.includes(`${path.sep}.repo${path.sep}⚡`)) continue;
      walk(full, callback);
    } else {
      callback(full);
    }
  }
}

function isTextFile(file: string) {
  return !/\.(png|ico|zip|jpg|jpeg|gif|webp|wasm|glb|ghx|gh|db|woff2?|ttf|eot)$/i.test(file);
}

let modified = 0;
walk(ROOT, (file) => {
  if (!isTextFile(file)) return;
  const rel = path.relative(ROOT, file);
  if (rel.startsWith(".repo/🎫/26/06/22/RENAME-DESIGN-TECHNOLOGY-TO-COMPOSE/move-semio-assets.ts")) return;
  let content = fs.readFileSync(file, "utf8");
  let next = content;
  for (const [from, to] of REPLACEMENTS) next = next.replaceAll(from, to);
  if (next !== content) {
    fs.writeFileSync(file, next, "utf8");
    modified++;
    console.log(rel);
  }
});

console.log(`Updated ${modified} files.`);
