import {readFileSync} from"node:fs";
const base="C:/git/semio/🧰️framework/🛍️products/📓️print";
const {vizPrintedKeyRows}=await import(base+"/🔨️modules/📊️visualization-gallery/🟦️.ts");
const rows=vizPrintedKeyRows(readFileSync(base+"/🧾️template/📊️viz-api/🔓️viz-api.tex","utf8"))["api-path-geo-planar"];
const expected=JSON.parse(readFileSync(base+"/🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/🔓️api-freshness.json","utf8")).printed.sharedSourceContracts.scopes.find((scope:any)=>scope.id==="api-path-geo-planar");
console.log(JSON.stringify({rows,expected},null,2));
