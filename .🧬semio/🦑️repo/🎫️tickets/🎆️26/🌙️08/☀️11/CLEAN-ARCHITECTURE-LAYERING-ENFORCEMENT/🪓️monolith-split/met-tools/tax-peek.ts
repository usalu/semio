import { readFileSync } from "node:fs";
const t = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const raw = t.semanticDirectoryMemberKinds;
const entries: [string, any][] = Array.isArray(raw) ? raw.map((x: any, i: number) => [String(i), x]) : Object.entries(raw);
console.log("rows", entries.length);
const testsRows = entries.filter(([, x]) => x.ownerKindIds?.includes("tests"));
console.log(testsRows.map(([k, x]) => ({ k, o: x.ownerKindIds, n: x.memberNames.length, s: x.source })));
for (const n of process.argv.slice(3)) console.log(n, entries.filter(([, x]) => x.memberNames.includes(n)).map(([k, x]) => `${k}:${x.ownerKindIds.join("+")}:${x.source}`));
console.log(Object.keys(t).slice(0, 80));
