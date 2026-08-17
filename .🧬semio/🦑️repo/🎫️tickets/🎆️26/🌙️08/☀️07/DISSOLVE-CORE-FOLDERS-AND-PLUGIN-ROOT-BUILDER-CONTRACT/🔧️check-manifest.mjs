import { readFileSync, readdirSync, writeFileSync } from "fs";
import { join } from "path";
import { execSync } from "child_process";

const MODULES = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules";
const manifestDir = readdirSync(MODULES).find((n) => n.includes("manifest"));
const path = join(MODULES, manifestDir, "🟦️component.ts");
const t = readFileSync(path, "utf8");
console.log("lines", t.split("\n").length);
console.log("--- first 80 ---");
console.log(t.split("\n").slice(0, 80).join("\n"));
console.log("--- has GeneratedMirror?", t.includes("GeneratedMirror"));
console.log("--- has AppDefinition?", t.includes("AppDefinition"));
console.log("--- has WindowLayout?", t.includes("WindowLayout"));
console.log("--- has PluginManifest?", t.includes("PluginManifest"));
