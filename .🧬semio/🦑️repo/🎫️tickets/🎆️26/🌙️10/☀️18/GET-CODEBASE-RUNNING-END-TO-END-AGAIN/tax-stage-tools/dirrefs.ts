// 🔎 Lists every reference (full path substring or resolving relative quoted string) to the four emoji-only test directories. usage: bun dirrefs.ts
import {readFileSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
const REPO = "/Users/ueli/Documents/semio", C = `${REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`;
const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const dirs = [`${L}/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🧪️tests/🛂️`, `${L}/🏃️process/🦀️cargo/🧪️testing/📦️executable/🧪️tests/🌿️`, `${L}/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/🦀️`, `${L}/🏗️build/🎚️mode/🦀️profile/🧪️tests/🦀️`];
const files = readFileSync(`${C}/tax-stage-tools/index/grep-files.txt`, "utf8").split("\n").filter(Boolean);
for (const f of files) {
  let text: string; try { text = readFileSync(join(REPO, f), "utf8"); } catch { continue; }
  const lines = text.split("\n");
  lines.forEach((line, i) => {
    for (const d of dirs) {
      const old = d + "/🟦️.ts", oldDirAsFile = d + ".ts";
      if (line.includes(d) || line.includes(oldDirAsFile)) console.log(`${f.replace(L, "L")}:${i + 1}: FULL ${d.split("/").slice(-4).join("/")} :: ${line.trim().slice(0, 140)}`);
    }
    for (const m of line.matchAll(/(["'`])(\.{1,2}\/[^"'`\n]*?)\1/gu)) {
      const abs = relative(REPO, resolve(dirname(join(REPO, f)), m[2]!));
      for (const d of dirs) if (abs === d || abs === d + "/🟦️.ts" || abs.startsWith(d + "/") || abs === d + ".ts") console.log(`${f.replace(L, "L")}:${i + 1}: REL ${m[2]} -> ${d.split("/").slice(-3).join("/")}`);
    }
  });
}
