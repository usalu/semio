import {runtimeParseTomlV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⚙️runtime/📄️parse/🟦️.ts";
import {readFileSync} from "node:fs";
const t=readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/Cargo.toml","utf8");
const p=runtimeParseTomlV1(t) as any;
console.log(JSON.stringify(p.package?.metadata?.semio)?.slice(0,300));
