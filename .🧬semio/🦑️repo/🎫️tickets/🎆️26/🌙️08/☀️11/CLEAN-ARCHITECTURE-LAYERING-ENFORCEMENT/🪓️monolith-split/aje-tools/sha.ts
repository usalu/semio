/** 🔐️ Hashes every asset target of the batch (working tree versus the HEAD blob) and decodes the inlined font data urls to compare their bytes. Usage: bun sha.ts */
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { createServer } from "/Users/ueli/Documents/semio/node_modules/vite/dist/node/index.js";
const ROOT = "/Users/ueli/Documents/semio";
const sha = (b: Buffer) => createHash("sha256").update(b).digest("hex");
const OS = "🧰️framework/🛍️products/💻️os", ASSETS = "🧰️framework/🔨️modules/🖼️assets";
const files = [
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🖼️assets/🎬️demo/🗣️.dsl.semio",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🖼️assets/🎬️demo/🗣️.dsl.semio",
  ...["◻️2d", "🧊️3d", "🖐️5d"].flatMap(d => d === "◻️2d" ? ["🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest", "➡️hexagonal-cut-concrete-forest-right/🖼️assets/➡️hexagonal-cut-concrete-forest"] : ["🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest", "🏢️nakagin-capsule/🖼️assets/🏢️nakagin-capsule"]).length ? [] : [],
];
for (const [d, ex] of [["◻️2d", "🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest"], ["◻️2d", "➡️hexagonal-cut-concrete-forest-right/🖼️assets/➡️hexagonal-cut-concrete-forest"], ["🧊️3d", "🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest"], ["🧊️3d", "🏢️nakagin-capsule/🖼️assets/🏢️nakagin-capsule"], ["🖐️5d", "🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest"], ["🖐️5d", "🏢️nakagin-capsule/🖼️assets/🏢️nakagin-capsule"]]) files.push(`✏️s/🔌️plugins/🧱️block/🗿️artifacts/${d}/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/${ex}/🗣️.dsl.semio`);
files.push(
  `${OS}/🔨️modules/♾️infinite/🖼️assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg`,
  `${OS}/🔨️modules/♾️infinite/🖼️assets/🏛️rathaus-ahlen-grundriss/🖼️.png`,
  `${ASSETS}/🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule_J.glb`,
  `${ASSETS}/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf`,
  `${ASSETS}/🔤️fonts/🚀️anta/➕️latin-ext/📖️regular/🔤️outline.ttf`,
  `${ASSETS}/🔤️fonts/🚀️anta/🧮️math/📖️regular/🔤️outline.ttf`,
  `${ASSETS}/🔤️fonts/🚀️anta/🔣️symbols/📖️regular/🔤️outline.ttf`,
  `${ASSETS}/👆️cursor/🖱️default/☀️light.svg`,
  "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🎨️.css",
  "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🧪️tests/🧫️fixtures/🚦️sqlite-cohort/🔣️.json",
);
let bad = 0;
for (const f of files) {
  const now = readFileSync(`${ROOT}/${f}`);
  let head: Buffer | null = null;
  try { head = execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "show", `HEAD:${f}`], { cwd: ROOT, maxBuffer: 1 << 28, stdio: ["ignore", "pipe", "ignore"] }); } catch {}
  const same = head ? sha(head) === sha(now) : null;
  if (same === false) bad++;
  console.log(`${sha(now).slice(0, 16)} ${head ? (same ? "==HEAD" : "DIFFERS-FROM-HEAD") : "untracked"} ${now.length}B ${f.split("/").slice(-3).join("/")}`);
}
const server = await createServer({ root: ROOT, configFile: false, logLevel: "silent", appType: "custom", server: { middlewareMode: true, fs: { allow: [ROOT] }, watch: null }, resolve: { alias: [] }, optimizeDeps: { noDiscovery: true, include: [] } });
for (const [spec, f] of [["@semio-tech/assets/fonts/anta/latin/regular/outline", 8], ["@semio-tech/assets/fonts/anta/latin-ext/regular/outline", 7], ["@semio-tech/assets/fonts/anta/math/regular/outline", 6], ["@semio-tech/assets/fonts/anta/symbols/regular/outline", 5]] as const) {
  const mod = await server.ssrLoadModule(spec) as Record<string, string>;
  const url = Object.values(mod)[0];
  const m = url.match(/^data:[^,]*;base64,(.*)$/su);
  const bytes = m ? Buffer.from(m[1], "base64") : Buffer.alloc(0);
  const file = readFileSync(`${ROOT}/${files[files.length - f]}`);
  const ok = sha(bytes) === sha(file);
  if (!ok) bad++;
  console.log(ok ? "inline-bytes==file" : "INLINE-BYTES-DIFFER", spec);
}
await server.close();
console.log({ files: files.length, bad });
