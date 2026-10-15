import { readdirSync } from "node:fs";
const dir = process.argv[2];
const names = readdirSync(dir).filter(n => !n.endsWith(".rs"));
for (const n of names) {
  const m = n.match(/^(.*?)️/u);
  const e = m ? m[1] : "?";
  console.log(n.slice(0,40), JSON.stringify(e), /^\p{RGI_Emoji}$/v.test(e), n.codePointAt(n.indexOf("️")-0) );
}
const heads = names.map(n => n.match(/^(.*?)️/u)?.[1]); console.log("dups", heads.length - new Set(heads).size);
