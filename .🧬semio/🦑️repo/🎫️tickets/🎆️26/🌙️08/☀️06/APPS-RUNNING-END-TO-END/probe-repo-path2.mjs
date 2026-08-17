import { existsSync, readdirSync, readFileSync } from "fs";
import { join, resolve } from "path";
const root = process.cwd();
const fw = readdirSync(root).find(x => x.includes("framework") && !x.startsWith("."));
console.log("fw dir", JSON.stringify(fw));
const scopes = readFileSync(join(root, ".storybook/scopes.ts"), "utf8");
const m = scopes.match(/from "([^"]+repo[^"]+index\.ts)"/);
console.log("import path", m?.[1]);
const resolved = resolve(root, ".storybook", m?.[1] ?? "");
console.log("resolved", resolved, existsSync(resolved));
// list repo products
const products = join(root, fw, "🛍️products");
console.log("products", existsSync(products), readdirSync(products).filter(x => x.includes("repo") || x.includes("os")));
const repoLib = join(root, "node_modules/@semio-tech/repo-lib");
console.log("repo-lib link", repoLib, existsSync(repoLib));
import { readlinkSync } from "fs";
try { console.log("repo-lib target", readlinkSync(repoLib)); } catch(e) { console.log(e.message); }
