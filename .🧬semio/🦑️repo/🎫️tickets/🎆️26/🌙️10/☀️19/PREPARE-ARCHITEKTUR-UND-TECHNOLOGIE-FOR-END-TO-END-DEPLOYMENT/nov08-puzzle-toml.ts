import { readFileSync } from "node:fs";
const data = Bun.TOML.parse(readFileSync("D:/semio/✏️s/🔌️plugins/🧩️puzzle/Cargo.toml", "utf8")) as any;
if (data.workspace.dependencies["semio-framework-trace"].path !== "../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust") throw Error("trace dependency mismatch");
console.log("trace workspace dependency is unique");
