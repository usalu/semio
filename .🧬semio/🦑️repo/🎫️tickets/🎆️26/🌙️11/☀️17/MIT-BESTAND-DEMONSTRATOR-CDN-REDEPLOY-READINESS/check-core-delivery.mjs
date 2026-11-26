import { mkdtemp, readdir, readFile, rm, stat } from "node:fs/promises";
import { basename, dirname, join } from "node:path";
import { spawnSync } from "node:child_process";
import { browserCoreName, compressBrowserCore, gzipCoreLoader } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/🗜️cores/🟦️.ts";

const [site, generated] = process.argv.slice(2);
const work = await mkdtemp(join(generated, "codec-"));
let files = 0, published = 0, count = 0, largest = 0;
const validate = [
  "import gzip, pathlib, sys",
  "with open(sys.argv[1], 'rb') as raw, gzip.open(sys.argv[2], 'rb') as stored:",
  " while True:",
  "  block = raw.read(1048576)",
  "  assert block == stored.read(1048576)",
  "  if not block: break",
].join("\n");
try {
  const entries = await readdir(site, {recursive: true, withFileTypes: true});
  for (const entry of entries) {
    if (!entry.isFile()) continue;
    const path = join(entry.parentPath, entry.name);
    let size = (await stat(path)).size;
    files++;
    if (browserCoreName.test(entry.name)) {
      const encoded = join(work, "payload.bin");
      await compressBrowserCore(path, encoded);
      const check = spawnSync("python3", ["-c", validate, path, encoded], {stdio: "inherit"});
      if (check.status !== 0) throw Error("Independent decompression failed: " + path);
      const loader = join(dirname(path), basename(path).replace(browserCoreName, ".js"));
      const raw = await readFile(loader, "utf8");
      published += Buffer.byteLength(gzipCoreLoader(raw, [entry.name])) - Buffer.byteLength(raw);
      size = (await stat(encoded)).size;
      largest = Math.max(largest, size);
      await rm(encoded);
      count++;
    }
    published += size;
  }
  console.log(JSON.stringify({files, count, published, largest}));
  if (!count || published > 1_000_000_000) process.exitCode = 1;
} finally { await rm(work, {recursive:true, force:true}); }
