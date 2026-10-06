import{readFileSync,writeFileSync,copyFileSync,renameSync}from"node:fs";
import{createHash}from"node:crypto";
const root="C:/git/semio/🧰️framework/🛍️products/📓️print",ticket=process.env.SEMIO_TICKET_DIR!,schemaPath=root+"/🧬️schema/🔣️.json",fixturePath=root+"/🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/🔓️api-freshness.json";
const block=(source:string,name:string,offset=0)=>{const start=source.indexOf('"'+name+'": {',offset),open=source.indexOf('{',start);if(start<0)throw Error(name+" member missing");let depth=0,quoted=false,escaped=false;for(let index=open;index<source.length;index++){const character=source[index];if(quoted){if(escaped)escaped=false;else if(character==='\\')escaped=true;else if(character==='"')quoted=false;}else if(character==='"')quoted=true;else if(character==='{')depth++;else if(character==='}'&&!--depth)return{start:open,end:index+1,text:source.slice(open,index+1)};}throw Error(name+" unbalanced");};
const schemaBefore=readFileSync(schemaPath,"utf8"),familyBlock=block(schemaBefore,"geo-hexbin",schemaBefore.indexOf('"x-semio-family-options"')),family=JSON.parse(familyBlock.text),original=JSON.stringify(family),options=family.options;
const forbidden=["fit","geometry","parallels","projection","rotate"];
for(const key of forbidden){if(!options[key])throw Error("family removal guard "+key);delete options[key];}
const shared=JSON.parse(readFileSync(ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/planar-source-contract.json","utf8"));
for(const row of shared.keys){const option=options[row.key];if(!option)throw Error("effective shared control missing "+row.key);option.description=row.description;if(row.key!=="lineWidth")option.default=row.type==="number"||row.type==="integer"?Number(row.default):row.default;}
for(const key of ["width","height","radius","bandwidth"]){if(options[key].type!=="number"||options[key].syntax!=="expression")throw Error("positive arithmetic guard "+key);options[key].exclusiveMinimum=0;}
options.radius.default=5;options.radius.description={en:"Numeric binning radius in raw planar units. Uniform display fitting scales the visible hexagons.",de:"Numerischer Bündelungsradius in rohen planaren Einheiten. Gleichmäßige Darstellungsanpassung skaliert die sichtbaren Sechsecke."};
options.bandwidth.default=7;options.bandwidth.description={en:"Gaussian kernel standard deviation in raw planar units.",de:"Standardabweichung des Gaußschen Kerns in rohen planaren Einheiten."};
options.render.default="hex";options.render.description={en:"Rendering mode: hex draws bins; density and heat draw Gaussian density levels.",de:"Darstellungsmodus: hex zeichnet Zellen; density und heat zeichnen Gaußsche Dichteniveaus."};
options.levels.default=4;options.levels.description={en:"Number of contour levels for density and heat rendering.",de:"Anzahl der Höhenlinienniveaus für Dichte- und Wärmedarstellung."};
options.points.description={en:"Named planar point collection for hexagonal bins or Gaussian density estimates.",de:"Benannte planare Punktsammlung für sechseckige Zellen oder Gaußsche Dichteschätzungen."};
options.opacity.description={en:"Drawing opacity; hex fill uses the same value, heat fill uses 0.3 times it.",de:"Zeichendeckkraft; Sechseckfüllungen nutzen denselben Wert, Wärmefüllungen das 0,3-Fache."};
if(Object.keys(options).length!==13)throw Error("exact planar thirteen-control boundary");
const formatted=JSON.stringify(family,null,2).split('\n').map((line,index)=>index?'    '+line:line).join('\n'),schemaAfter=schemaBefore.slice(0,familyBlock.start)+formatted+schemaBefore.slice(familyBlock.end);
if(JSON.stringify(JSON.parse(schemaAfter)['x-semio-option-syntax'])!==JSON.stringify(JSON.parse(schemaBefore)['x-semio-option-syntax']))throw Error("generic dictionaries changed");
copyFileSync(schemaPath,ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/planar-schema-before.json");
writeFileSync(ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/planar-family-before.json",familyBlock.text);
writeFileSync(ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/planar-family-candidate.json",JSON.stringify(family,null,2));
const schemaCandidate=ticket+"/🗑️generated/gallery-planar-family-contract/schema-candidate.json";writeFileSync(schemaCandidate,schemaAfter);if(readFileSync(schemaPath,"utf8")!==schemaBefore)throw Error("concurrent schema write");renameSync(schemaCandidate,schemaPath);
const fixtureBefore=readFileSync(fixturePath,"utf8"),nativeStart=fixtureBefore.indexOf('"nativeContracts"'),nativeBlock=block(fixtureBefore,"geo-hexbin",nativeStart),native=JSON.parse(nativeBlock.text);
for(const key of ["width","height","radius","bandwidth"]){native[key]={type:"number",syntax:"expression",default:String(options[key].default),exclusiveMinimum:0};}
let fixtureAfter=fixtureBefore.slice(0,nativeBlock.start)+JSON.stringify(native,null,2).split('\n').map((line,index)=>index?'      '+line:line).join('\n')+fixtureBefore.slice(nativeBlock.end);
const forbiddenMarker='"forbiddenDescriptors": [';
if(fixtureAfter.split(forbiddenMarker).length!==2)throw Error("neutral forbidden guard");
fixtureAfter=fixtureAfter.replace(forbiddenMarker,forbiddenMarker+'\n'+forbidden.map(key=>'      '+JSON.stringify({family:"geo-hexbin",key})+',').join('\n'));
const policyMarker='"sourcePolicy": [';
if(fixtureAfter.split(policyMarker).length!==2)throw Error("neutral source policy guard");
fixtureAfter=fixtureAfter.replace(policyMarker,policyMarker+'\n'+[...Object.keys(options).filter(key=>shared.keys.some((row:any)=>row.key===key)).map(key=>({scope:"semio / viz / geo / planar",key,allowed:true,table:"api-path-geo-planar"})),...forbidden.map(key=>({scope:"semio / viz / geo / planar",key,allowed:false,table:"api-path-geo-planar"}))].map(value=>'      '+JSON.stringify(value)+',').join('\n'));
JSON.parse(fixtureAfter);copyFileSync(fixturePath,ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/planar-family-fixture-before.json");
const fixtureCandidate=ticket+"/🗑️generated/gallery-planar-family-contract/fixture-candidate.json";writeFileSync(fixtureCandidate,fixtureAfter);if(readFileSync(fixturePath,"utf8")!==fixtureBefore)throw Error("concurrent fixture write");renameSync(fixtureCandidate,fixturePath);
const sha=(value:string)=>createHash("sha256").update(value).digest("hex");
console.log(JSON.stringify({schemaBefore:sha(schemaBefore),schemaAfter:sha(schemaAfter),fixtureAfter:sha(fixtureAfter),beforeControls:Object.keys(JSON.parse(original).options).length,afterControls:Object.keys(options).length,removed:forbidden,exclusiveMinimum0:["width","height","radius","bandwidth"]}));
