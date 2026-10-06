import { readFileSync, writeFileSync, copyFileSync, renameSync } from "node:fs";
import { createHash } from "node:crypto";
const root="C:/git/semio",ticket=process.env.SEMIO_TICKET_DIR!,base=root+"/🧰️framework/🛍️products/📓️print";
const path=base+"/🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/🔓️api-freshness.json",before=readFileSync(path,"utf8");
const pairs=[
 ["width","number","76","fp","Maximum fitted display width in millimetres. Raw planar coordinates are scaled uniformly.","Maximale angepasste Darstellungsbreite in Millimetern. Rohe planare Koordinaten werden gleichmäßig skaliert."],
 ["height","number","44","fp","Maximum fitted display height in millimetres. Raw planar coordinates are scaled uniformly.","Maximale angepasste Darstellungshöhe in Millimetern. Rohe planare Koordinaten werden gleichmäßig skaliert."],
 ["palette","string","sequential","tl","Colour ramp used for binned values or density levels.","Farbverlauf für gebündelte Werte oder Dichteniveaus."],
 ["stroke","string","semio-chrome-border-emphasized","tl","Hexagon outline colour as a native colour expression. Density and heat outlines use their palette colour.","Umrissfarbe der Sechsecke als nativer Farbausdruck. Dichte- und Wärmeumrisse nutzen ihre Palettenfarbe."],
 ["lineWidth","string","semio@stroke@hairline","tl","Hexagon and density outline width as a TeX dimension from the theme hairline token.","Umrissbreite der Sechsecke und Dichteflächen als TeX-Dimension aus dem Haarlinienwert des Themas."],
 ["opacity","number","1","fp","Drawing and filling opacity from zero to one.","Zeichen- und Füllungsdeckkraft von null bis eins."],
 ["paletteSteps","integer","5","int","Positive number of final colour-ramp levels. One level uses the palette midpoint.","Positive Anzahl endgültiger Farbverlaufsstufen. Eine Stufe nutzt die Palettenmitte."]
];
const source={file:base+"/🖋️latex/semio-viz-geo-symbols.sty",sha256:createHash("sha256").update(readFileSync(base+"/🖋️latex/semio-viz-geo-symbols.sty")).digest("hex")};
const contract={id:"api-path-geo-planar",scope:"semio / viz / geo / planar",source,keys:pairs.map(([key,type,value,setter,en,de])=>({key,type,default:value,setter,description:{en,de}}))};
let candidate=before;
const binding='    "scopeBindings": {';
if(candidate.split(binding).length!==2)throw Error("scope bindings guard");
candidate=candidate.replace(binding,binding+'\n      "api-path-geo-planar": "semio / viz / geo / planar",');
const shared=candidate.indexOf('"sharedSourceContracts"'),position=candidate.indexOf('"scopes": [',shared)+'"scopes": ['.length;
if(shared<0||position<0||JSON.parse(before).printed.sharedSourceContracts.scopes.some((scope:any)=>scope.id===contract.id))throw Error("shared scope guard");
candidate=candidate.slice(0,position)+'\n'+JSON.stringify(contract,null,2).split('\n').map(line=>'        '+line).join('\n')+','+candidate.slice(position);
copyFileSync(path,ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/api-freshness-planar-before.json");
writeFileSync(ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/planar-source-contract.json",JSON.stringify(contract,null,2));
JSON.parse(candidate);
const temporary=ticket+"/🗑️generated/gallery-planar-contract/fixture-candidate.json";writeFileSync(temporary,candidate);renameSync(temporary,path);
const {verifyPrintApiFreshness}=await import(base+"/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts");
verifyPrintApiFreshness();
