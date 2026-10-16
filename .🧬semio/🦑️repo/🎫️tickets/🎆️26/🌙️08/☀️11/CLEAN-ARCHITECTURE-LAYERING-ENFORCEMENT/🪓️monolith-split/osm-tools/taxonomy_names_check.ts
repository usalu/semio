import { semanticDirectoryMemberAdmitsName } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { readFileSync } from "node:fs";
const head = JSON.parse(readFileSync(process.argv[2], "utf8"));
const names = readFileSync(process.argv[3], "utf8").split("\n").filter(Boolean);
let bad = 0;
for (const n of names) {
  const [row, name] = n.split("\t");
  const ok = semanticDirectoryMemberAdmitsName(head.semanticDirectoryMemberKinds[row], name);
  if (!ok) bad++;
  console.log(ok ? "OK  " : "FAIL", row, name);
}
console.log(bad === 0 ? "ALL ADMITTED" : bad + " NOT ADMITTED");
