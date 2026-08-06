import fs from "fs";
import path from "path";
const OS = fs.readFileSync("/tmp/os-path.txt", "utf8").trim();
const modules = path.join(OS, fs.readdirSync(OS).find((x) => x.includes("modules")));
const store = path.join(modules, fs.readdirSync(modules).find((x) => x.includes("store")));
const syncCargo = path.join(store, fs.readdirSync(store).find((x) => x.includes("sync")),
  fs.readdirSync(path.join(store, fs.readdirSync(store).find((x) => x.includes("sync")))).find((x) => x.includes("implementations")),
  "rust".length ? fs.readdirSync(path.join(store, fs.readdirSync(store).find((x) => x.includes("sync")),
    fs.readdirSync(path.join(store, fs.readdirSync(store).find((x) => x.includes("sync")))).find((x) => x.includes("implementations")))).find((x) => x.includes("rust")) : "",
  "Cargo.toml");
console.log(syncCargo);
console.log(fs.readFileSync(syncCargo, "utf8"));

// What does pack cli need for SchemaResolver - is registry required for default compile?
const lib = path.join(OS, fs.readdirSync(OS).find((x) => x.includes("packages")),
  fs.readdirSync(path.join(OS, fs.readdirSync(OS).find((x) => x.includes("packages")))).find((x) => x.includes("rust")),
  fs.readdirSync(path.join(OS, fs.readdirSync(OS).find((x) => x.includes("packages")),
    fs.readdirSync(path.join(OS, fs.readdirSync(OS).find((x) => x.includes("packages")))).find((x) => x.includes("rust")))).find((x) => x.includes("lib") || x.endsWith("lib.rs")));
console.log("\nlib", lib);
// check if pack::cli SchemaResolver is used from registry only
const packCli = path.join(modules, fs.readdirSync(modules).find((x) => x.includes("pack")),
  fs.readdirSync(path.join(modules, fs.readdirSync(modules).find((x) => x.includes("pack")))).find((x) => x.includes("cli")),
  "component.rs");
const packCliPath = (() => {
  const pack = path.join(modules, fs.readdirSync(modules).find((x) => x.includes("pack")));
  const cli = path.join(pack, fs.readdirSync(pack).find((x) => x.includes("cli")));
  return path.join(cli, fs.readdirSync(cli).find((x) => x.includes("component")));
})();
const pct = fs.readFileSync(packCliPath, "utf8");
console.log("SchemaResolver trait:\n", pct.slice(pct.indexOf("trait SchemaResolver"), pct.indexOf("trait SchemaResolver") + 400));
console.log("pack cli cfg:", [...pct.matchAll(/#\[cfg\([^\]]+\]/g)].slice(0,10).map(m=>m[0]));
