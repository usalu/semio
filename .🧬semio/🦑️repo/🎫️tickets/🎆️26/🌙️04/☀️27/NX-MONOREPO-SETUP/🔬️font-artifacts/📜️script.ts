import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";
if (process.argv[2] === "inspect") {
  const opentype = await import("opentype.js");
  const bytes = readFileSync(process.argv[3]), count = bytes.readUInt32LE(0);
  let offset = 4;
  const fonts = [];
  for (let index = 0; index < count; index++) {
    const size = bytes.readUInt32LE(offset); offset += 4;
    const font = opentype.parse(Uint8Array.from(bytes.subarray(offset, offset + size)).buffer);
    assert.ok(font.numGlyphs > 0);
    assert.ok(font.unitsPerEm > 0);
    fonts.push({ glyphs: font.numGlyphs, unitsPerEm: font.unitsPerEm });
    offset += size;
  }
  assert.equal(offset, bytes.length);
  assert.ok(count > 0);
  writeFileSync(process.argv[4], JSON.stringify(fonts, null, 2) + "\n");
  console.log("[DEBUG] Independent OpenType consumer parsed " + count + " restored fonts");
  process.exit(0);
}
const root = process.cwd(), ticket = dirname(dirname(fileURLToPath(import.meta.url)));
const evidence = join(ticket, "🗑️generated", "font-restoration-" + Date.now());
mkdirSync(evidence, { recursive: true });
const output = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/dist");
const outputs = ["font-tool", "fonts"];
const snapshot = (): Record<string, unknown> => {
  const result: Record<string, unknown> = {};
  for (const directory of outputs) for (const name of readdirSync(join(output, directory))) {
    const path = join(output, directory, name);
    result[relative(output, path)] = { sha256: createHash("sha256").update(readFileSync(path)).digest("hex"), mode: lstatSync(path).mode & 0o777 };
  }
  return result;
};
const run = async (label: string): Promise<string> => {
  const child = Bun.spawn(["bun", "nx", "run", "semio-framework-os-infinite:fonts", "--output-style=stream"], { cwd: root, stdout: "pipe", stderr: "pipe" });
  const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  writeFileSync(join(evidence, label + ".log"), stdout + stderr);
  assert.equal(code, 0, stdout + stderr);
  return stdout;
};
await run("warm");
const before = snapshot();
for (const name of outputs) renameSync(join(output, name), join(evidence, name));
try {
  const log = await run("restore");
  for (const target of ["font-tool", "fonts"]) assert.match(log, new RegExp("semio-framework-os-infinite:" + target + " .*local cache"));
  assert.deepEqual(snapshot(), before);
} finally { for (const name of outputs) if (!existsSync(join(output, name))) renameSync(join(evidence, name), join(output, name)); }
const oracle = Bun.spawn(["node", fileURLToPath(import.meta.url), "inspect", join(output, "fonts/🔤️guestslim-typst-fonts.bin"), join(evidence, "fonts.json")], { cwd: root, stdout: "pipe", stderr: "pipe" });
const [stdout, stderr, code] = await Promise.all([new Response(oracle.stdout).text(), new Response(oracle.stderr).text(), oracle.exited]);
writeFileSync(join(evidence, "oracle.log"), stdout + stderr);
assert.equal(code, 0, stdout + stderr);
writeFileSync(join(ticket, "📓️font-artifacts.md"), "# Font Artifact Restoration\n\nNx restored the deleted native font tool and packed font asset with identical file hashes and modes. The independent Node/OpenType consumer parsed every restored font and verified nonempty glyph tables and valid units per em.\n\n" + stdout);
console.log("[DEBUG] Font tool and asset Nx restoration plus independent OpenType consumer: PASS");
