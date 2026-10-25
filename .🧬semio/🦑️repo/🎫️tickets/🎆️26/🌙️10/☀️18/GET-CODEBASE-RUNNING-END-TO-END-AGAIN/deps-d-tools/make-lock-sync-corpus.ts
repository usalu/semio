import {readFileSync,writeFileSync} from "node:fs";
import {javascriptLockRefusalTextV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔒️lock-sync/🟦️.ts";
const doc=JSON.parse(readFileSync(process.argv[2]!,"utf8"));
for(const row of doc.texts)row.expected=javascriptLockRefusalTextV1(row.kind,row.scope,row.drift,row.languages);
writeFileSync(process.argv[3]!,JSON.stringify(doc,null,1)+"\n");
for(const row of doc.texts)console.log("---",row.name,"\n"+row.expected);
