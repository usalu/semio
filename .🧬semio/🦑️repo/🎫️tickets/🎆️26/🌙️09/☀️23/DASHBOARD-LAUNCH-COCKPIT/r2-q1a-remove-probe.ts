import { patch } from "./r2-q1a-edit.ts";
import { readFileSync } from "node:fs";

const file = "⌨️tui/🧪️tests/🔬️unit/🦀️.rs";
const text = readFileSync(`C:/git/semio/🧰️framework/🔨️modules/🖱️ui/${file}`, "utf8").replaceAll("\r\n", "\n");
const start = text.indexOf("\n#[test]\nfn zz_size_probe()");
if (start < 0) throw new Error("probe missing");
patch(file, [[text.slice(start), ""]]);
console.log("probe removed");
