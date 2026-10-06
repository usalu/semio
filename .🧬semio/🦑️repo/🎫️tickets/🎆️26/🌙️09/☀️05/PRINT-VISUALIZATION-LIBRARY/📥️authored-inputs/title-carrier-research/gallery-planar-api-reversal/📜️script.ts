import{readFileSync,writeFileSync}from"node:fs";
import{createHash}from"node:crypto";
const ticket=process.env.SEMIO_TICKET_DIR!,api="C:/git/semio/🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-api/🔓️viz-api.tex",after=readFileSync(api,"utf8"),deltas:{before:string;after:string}[]=[];
const literalOld=[
String.raw`  \SemioTableRow{\Key{points} & \Key{string} & \Key{demo-points-dense} & \ApiText{Named geographic point collection binned into hexagonal cells.}{Benannte geografische Punktsammlung für die Aufteilung in sechseckige Zellen.}}`,
String.raw`  \SemioTableRow{\Key{radius} & \Key{number} & --- & \ApiText{Radius in millimetres, or the column that drives it.}{Radius in Millimetern oder die Spalte, die ihn steuert.}}`,
String.raw`  \SemioTableRow{\Key{render} & \Key{string} & --- & \ApiText{How the cells or units are rendered.}{Wie die Zellen bzw. Einheiten gezeichnet werden.}}`,
String.raw`  \SemioTableRow{\Key{bandwidth} & \Key{number} & --- & \ApiText{Kernel bandwidth of the density estimate; '0' uses Silverman's rule.}{Bandbreite des Kerns der Dichteschätzung; '0' nutzt Silvermans Regel.}}`,
String.raw`  \SemioTableRow{\Key{levels} & \Key{integer} & --- & \ApiText{Number of contour levels.}{Anzahl der Höhenlinienniveaus.}}`
];
const prior=readFileSync(ticket+"/🗑️generated/api-title-contract/candidate/api-candidate.tex","utf8");
const priorSection=prior.slice(prior.indexOf("\\subsection{\\Key{geo-hexbin}}"),prior.indexOf("\\subsection{\\Key{geo-route}}"));
const old=priorSection.split("\n").filter(line=>line.startsWith("  \\SemioTableRow"));
if(JSON.stringify(old.map(line=>line.trimEnd()))!==JSON.stringify(literalOld.map(line=>line.replace(/\\u([0-9a-f]{4})/gi,(_,value)=>String.fromCharCode(parseInt(value,16))))))throw Error("prior body rows differ from handpicked before contract");
let before=after;
const reverse=(previous:string,current:string)=>{if(before.split(current).length!==2)throw Error("exact reverse guard: "+current.slice(0,70));before=before.replace(current,previous);deltas.push({before:previous,after:current});};
reverse("","\\ApiTitleDefine{api-path-geo-planar}{Planar display}{Planare Darstellung}\n");
reverse("",readFileSync(ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/planar-key-table.tex","utf8"));
reverse("\\subsection{\\Key{geo-hexbin}}\n\\ApiKeyScope{api-path-geo-base}","\\subsection{\\Key{geo-hexbin}}\n\\ApiKeyScope{api-path-geo-planar}");
const section=after.slice(after.indexOf("\\subsection{\\Key{geo-hexbin}}"),after.indexOf("\\subsection{\\Key{geo-route}}"));
const rows=section.split("\n").filter(line=>line.startsWith("  \\SemioTableRow"));
if(rows.length!==old.length)throw Error("five direct row guard");
for(let index=0;index<old.length;index++)reverse(old[index]!,rows[index]!);
const sha=(value:string)=>createHash("sha256").update(value).digest("hex");
if(sha(before)!=="0c5a2690bcf60d465b4fd436a96375cb3aca0ef53ff0b7d37d312b4a0b857b6b")throw Error("full API reverse SHA mismatch: "+sha(before));
writeFileSync(ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/api-planar-before.tex",before);
writeFileSync(ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/api-planar-candidate.tex",after);
writeFileSync(ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/api-planar-scoped-deltas.json",JSON.stringify(deltas,null,2));
console.log(JSON.stringify({before:sha(before),after:sha(after),exactDeltas:deltas.length}));
