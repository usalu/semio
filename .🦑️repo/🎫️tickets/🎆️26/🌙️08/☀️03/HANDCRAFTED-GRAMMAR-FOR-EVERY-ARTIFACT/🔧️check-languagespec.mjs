import { readFileSync, existsSync, readdirSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";
const ticket = dirname(fileURLToPath(import.meta.url));
const fw = readdirSync(".").find((x) => x.includes("framework"));
const os = join(fw, "🛍️products", readdirSync(join(fw, "🛍️products")).find((x) => x.includes("os")));
const dsl = join(os, "🔨️modules", readdirSync(join(os, "🔨️modules")).find((x) => x.includes("dsl")));
const lib = readFileSync(join(dsl, "⚡️implementations/🦀️rust/📦️lib.rs"), "utf8");
const ok = {
  protocol: lib.includes("pub protocol:"),
  protocol_path: lib.includes("pub protocol_path:"),
  Pack: /LanguageRole[\s\S]*Pack/.test(lib),
  Spr: /LanguageRole[\s\S]*Spr/.test(lib),
  passthrough_hooks: lib.includes("passthrough_hooks"),
  derived_copies_protocol: lib.includes("protocol: parent.protocol"),
};
const p1 = join(os, "🔨️modules", readdirSync(join(os, "🔨️modules")).find((x) => x.includes("plugin")), "📦️packages/🦀️rust/Cargo.toml");
const pluginRoot = join(os, "🔨️modules", readdirSync(join(os, "🔨️modules")).find((x) => x.includes("plugin")));
const pPkg = join(pluginRoot, "📦️packages/🦀️rust/Cargo.toml");
const pImpl = join(pluginRoot, "⚡️implementations/🦀️rust/Cargo.toml");
console.log(JSON.stringify({ ok, pluginPkg: existsSync(pPkg), pluginImpl: existsSync(pImpl), pPkgName: existsSync(pPkg) ? readFileSync(pPkg, "utf8").match(/name = "([^"]+)"/)?.[1] : null, pImplName: existsSync(pImpl) ? readFileSync(pImpl, "utf8").match(/name = "([^"]+)"/)?.[1] : null }, null, 2));
