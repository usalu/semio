import { rustTokens } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
console.log(rustTokens("a.b: C, x::y, ..z, w:Q, p . q 'a: loop").map(t=>t.kind[0]+":"+t.text).join(" "));
