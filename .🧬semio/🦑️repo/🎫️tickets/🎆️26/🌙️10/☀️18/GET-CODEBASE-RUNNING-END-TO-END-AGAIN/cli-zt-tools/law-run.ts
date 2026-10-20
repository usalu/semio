/** ⚖️ Runs the zero-touch closure and command laws of the given tree on that tree and prints findings; usage: bun law-run.ts <root> */
import {join} from "node:path";
const root=process.argv[2]!,law=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🟦️.ts")),closures=law.checkZeroTouchClosuresV1(root),commands=law.checkZeroTouchCommandsV1(root);
const row=(f:any)=>`${f.closure}|${f.kind}|${f.file.split("/").slice(-3).join("/")}|${f.specifier}`;
console.log(JSON.stringify({closures:{count:Object.keys(closures.files).length,findings:closures.findings.map(row)},commands:{count:Object.keys(commands.files).length,files:commands.files,findings:commands.findings.map(row)}},null,1));
