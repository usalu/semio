import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import { sortedIndex } from "lodash";
const rows=readFileSync(process.argv[2],"utf8").trimEnd().split("\n");
for(const row of rows) {
  const [start,active,expected]=row.split("|");
  const list=active ? active.split(",").map(Number) : [];
  const split=sortedIndex(list,Number(start)%1024);
  assert.deepEqual([...list.slice(split),...list.slice(0,split)],expected ? expected.split(",").map(Number):[]);
}
console.log(rows.length+" rotations matched Lodash sortedIndex");
