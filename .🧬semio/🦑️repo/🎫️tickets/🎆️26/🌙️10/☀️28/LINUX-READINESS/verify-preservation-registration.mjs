import fs from "node:fs";
import path from "node:path";
import assert from "node:assert/strict";
import {fileURLToPath} from "node:url";
import {readProcessTestTableV1,expandProcessTestSuiteV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🗂️suites/🟦️.ts";
import "./validate-project-metadata.mjs";
const root=process.cwd(),base="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching",table=readProcessTestTableV1(JSON.parse(fs.readFileSync(path.join(root,base,"🗂️suites/🔣️.json"),"utf8"))),short="🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🧪️tests/🟦️.ts",physical="🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🧪️tests/🪢️custody/🟦️.ts",files=[short,physical],owners=file=>table.suites.filter(row=>expandProcessTestSuiteV1(row,files,table).includes(file));
assert.deepEqual(owners(short).map(row=>row.id),["bootstrap-dependencies"]);assert.deepEqual(owners(physical).map(row=>row.id),["javascript-storage"]);assert.equal(owners(short)[0].budgetMs,15000);assert.equal(owners(physical)[0].budgetMs,300000);
assert.equal(table.suites.find(row=>row.id==="javascript-storage").files.filter(file=>file===physical).length,1);
for(const file of files){assert(fs.existsSync(path.join(root,base,file)));assert(path.join(root,base,file).length<=256);}
const source=fs.readFileSync(path.join(root,base,short),"utf8");assert(!source.includes("privateRepositoryWorkspaceV1"));assert(!source.includes("imports actual binding custody"));assert(source.includes("readonly capsule presence"));assert(source.includes("complete multi-parent absence"));assert(source.includes("presence cancellation and parent churn"));
console.log(JSON.stringify({at:new Date().toISOString(),shortOwner:owners(short)[0].id,physicalOwner:owners(physical)[0].id,physicalBudgetMs:owners(physical)[0].budgetMs,physicalRoutes:owners(physical).length}));
