import { readFileSync, writeFileSync } from "node:fs";
const source=readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh","utf8");
const definitions=source.slice(0,source.indexOf("#region 🔖️Main"));
const child=Bun.spawn(["bash","-s","setup"],{stdin:new TextEncoder().encode(definitions+"\nensure_user_cc\n"),stdout:"inherit",stderr:"inherit",env:{...process.env,SEMIO_WORKSPACE_ROOT:process.cwd()}});
process.exit(await child.exited);
