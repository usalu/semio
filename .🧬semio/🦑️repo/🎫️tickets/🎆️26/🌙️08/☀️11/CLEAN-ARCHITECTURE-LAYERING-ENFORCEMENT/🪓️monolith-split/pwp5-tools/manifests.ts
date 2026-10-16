// 📋️ Dumps the parsed Cargo.toml of every plugin partition crate (the facade and the 80 skeletons) to JSON for the Python gates. usage: bun manifests.ts
import {readFileSync, writeFileSync} from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const rootToml = Bun.TOML.parse(readFileSync(ROOT + "/Cargo.toml", "utf8")) as any;
const out: Record<string, unknown> = {};
for (const m of rootToml.workspace.members as string[]) {
  if (!m.includes("🔌️plugin") || m.includes("🖥️host") || m.includes("🏇️mounted-owner") || m.includes("🏛️ownership") || m.includes("🧬️guest-contract") || m.includes("🖨️describe") || m.includes("🧠️interpreter")) continue;
  try { out[m] = Bun.TOML.parse(readFileSync(ROOT + "/" + m + "/Cargo.toml", "utf8")); } catch (e) { out[m] = {error: String(e)}; }
}
writeFileSync(import.meta.dir + "/../🗑️generated/pwp5-manifests.json", JSON.stringify(out));
console.log(Object.keys(out).length, "manifests;", Object.values(out).filter((v: any) => v.error).length, "unparsed");
