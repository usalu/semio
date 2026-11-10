import { readFileSync } from "node:fs";
import * as toml from "@iarna/toml";
const data = toml.parse(readFileSync("D:/semio/✏️s/🔌️plugins/🧩️puzzle/Cargo.toml", "utf8")) as any;
if (data.workspace.dependencies["semio-framework-trace"].path !== "../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust") throw Error("trace dependency mismatch");
console.log("trace workspace dependency is unique");
