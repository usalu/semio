import { readFileSync, existsSync, readdirSync, statSync } from "fs";
import { join } from "path";

const flow = "/Users/ueli/Documents/semio/𝒯framework/🛍️products/💻️os/🔨️modules/🌊️flow";
// discover
function findFlow() {
  const root = "/Users/ueli/Documents/semio/𝒯framework";
  const stack = [root];
  while (stack.length) {
    const dir = stack.pop();
    let ents; try { ents = readdirSync(dir); } catch { continue; }
    for (const name of ents) {
      if (["node_modules", "target", ".git"].includes(name)) continue;
      const p = join(dir, name);
      let st; try { st = statSync(p); } catch { continue; }
      if (!st.isDirectory()) continue;
      if (name === "🌊️flow" && p.includes("products") && p.includes("modules")) return p;
      stack.push(p);
    }
  }
  return null;
}
const f = findFlow();
console.log("flow", f);
const coreRs = join(f, "🙰core", "🦀️component.rs");
// try actual core dir name from listing: 🙰core was wrong, it was  lobbies emoji
const coreDir = readdirSync(f).find((n) => n.includes("core") || n.endsWith("core"));
console.log("coreDir", coreDir);
const coreRsPath = join(f, coreDir, "🦀️component.rs");
console.log("coreRsPath", coreRsPath, existsSync(coreRsPath));
console.log(readFileSync(coreRsPath, "utf8").slice(0, 1500));

const wasmExt = readdirSync(join(f, "🌾️extensions")).find((n) => n.includes("wasm"));
console.log("wasmExt", wasmExt);
const wasmRs = join(f, "🌾️extensions", wasmExt, "🦀️component.rs");
console.log(readFileSync(wasmRs, "utf8").slice(0, 2000));

// createFlowSession body
const wasmLoader = "/Users/ueli/Documents/semio/𝒯framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨️engine/🧱️elements/WasmSessionLoader/🟦️component.tsx";
function findWasmLoader() {
  const root = "/Users/ueli/Documents/semio/𝒯framework/🛍️products/💻️os/🔨️modules/📺️renderer";
  const stack = [root];
  while (stack.length) {
    const dir = stack.pop();
    let ents; try { ents = readdirSync(dir); } catch { continue; }
    for (const name of ents) {
      if (["node_modules", "target"].includes(name)) continue;
      const p = join(dir, name);
      let st; try { st = statSync(p); } catch { continue; }
      if (st.isDirectory()) stack.push(p);
      else if (name === "🟦️component.tsx" && p.includes("WasmSessionLoader")) return p;
    }
  }
  return null;
}
const wl = findWasmLoader();
console.log("wasmLoader", wl);
const t = readFileSync(wl, "utf8");
const idx = t.indexOf("export async function createFlowSession");
console.log(t.slice(idx, idx + 800));
