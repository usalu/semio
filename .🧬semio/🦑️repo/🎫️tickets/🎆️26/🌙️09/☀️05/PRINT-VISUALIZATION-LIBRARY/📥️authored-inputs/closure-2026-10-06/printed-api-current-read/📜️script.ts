import {readFileSync,writeFileSync} from "node:fs";
import {vizApiReference,vizPrintedFamilyFindings,vizPrintedPackageFindings,vizImplementedFamilyKeyScopes,vizPrintedKeyRows,vizDeclaredKeyNames,vizCoverageReport} from "C:/git/semio/🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🟦️.ts";
const base="🧰️framework/🛍️products/📓️print/",source=readFileSync(base+"🧾️template/📊️viz-api/🔓️viz-api.tex","utf8"),fixture=JSON.parse(readFileSync(base+"🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/🔓️api-freshness.json","utf8"));
const result={coverage:vizCoverageReport(),families:vizPrintedFamilyFindings(source,fixture.printed.scopeBindings),packages:vizPrintedPackageFindings(source),scopes:vizImplementedFamilyKeyScopes(),api:vizApiReference(),tables:vizPrintedKeyRows(source)};
writeFileSync(import.meta.dir+"/../printed-api-current-snapshot.json",JSON.stringify(result,null,2));console.log(JSON.stringify({families:result.families.length,packages:result.packages.length,totals:result.api.totals}));

