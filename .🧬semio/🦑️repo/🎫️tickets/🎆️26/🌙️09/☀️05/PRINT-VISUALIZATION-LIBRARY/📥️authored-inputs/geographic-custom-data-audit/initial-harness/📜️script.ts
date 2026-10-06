import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {changeVizChartValue} from 'C:/git/semio/🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/🟦️.ts';
import {applyVizChartDiff} from 'C:/git/semio/🧰️framework/🛍️products/📓️print/🧬️schema/🔀️diff/🟦️.ts';
import {inferVizChart} from 'C:/git/semio/🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🟦️.ts';
const root='C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/PRINT-VISUALIZATION-LIBRARY';
const fixture=JSON.parse(readFileSync('C:/git/semio/🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🔣️.json','utf8'));
const catalog=JSON.parse(readFileSync('C:/git/semio/🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-catalog.json','utf8'));
const records=[];mkdirSync(root+'/🗑️generated/geographic-custom-data-audit',{recursive:true});
for(const family of ['geo-basemap','spatial-scalar-field','geo-route','spatial-vector-field']){
 const kind=catalog.kinds.find((x:any)=>x.family===family);if(!kind)throw Error(family);
 let snapshot=structuredClone(fixture.nativeGeoPalette.snapshot);
 snapshot.chart.tables=[{name:'authored-geo',columns:['id','ring','lon','lat','value','u','v'],rows:[{id:'a',ring:'a',lon:0,lat:0,value:1,u:1,v:0},{id:'b',ring:'a',lon:1,lat:0,value:2,u:1,v:0},{id:'c',ring:'a',lon:0,lat:1,value:3,u:1,v:0}]}];
 snapshot.chart.presets=[{kind:kind.slug,data:'authored-geo',options:family==='geo-basemap'?{geometry:'authored-geo'}:family==='spatial-scalar-field'?{grid:'authored-geo'}:family==='geo-route'?{routes:'authored-geo'}:{}}];
 const changed=changeVizChartValue(snapshot,{path:['tables','0','rows','0','value'],value:4});const replay=applyVizChartDiff(snapshot,changed.diff);const inferred=await inferVizChart(replay.snapshot);
 writeFileSync(root+'/🗑️generated/geographic-custom-data-audit/'+family+'.tex',inferred.tikz??'');records.push({family,kind:kind.slug,mutationMessages:changed.messages,replayMessages:replay.messages,edits:changed.diff.edits.length,complete:inferred.complete,diagnostics:inferred.diagnostics,tikz:inferred.tikz});
}
writeFileSync(root+'/🗑️generated/geographic-custom-data-audit/emission.json',JSON.stringify(records,null,2));console.log('[DEBUG] '+JSON.stringify(records.map(({tikz,...r})=>({...r,bridgeCommands:tikz?.match(/\\SemioViz(?:GeoFromTable|ValueGrid|Geometry)/g)??[]}))));import {compileVizProbeDocument} from 'C:/git/semio/🧰️framework/🛍️products/📓️print/🔨️modules/🧪️viz-probe/🟦️.ts';
const consumed=[];
for(const record of records){
 let probe=record.family==='geo-basemap'||record.family==='geo-route'?'\\ExplSyntaxOn\\semio_viz_geom_map_inline:nn{authored-geo}{\\semio_viz_probe_values:nx{part/value}{\\use_iii:nnnn ##1}}\\ExplSyntaxOff':record.family==='spatial-vector-field'?'\\ExplSyntaxOn\\semio_viz_georte_field:nn{1}{0}\\semio_viz_probe_values:nx{field/uv}{\\fp_use:N\\l_semio_viz_georte_fu_fp,\\fp_use:N\\l_semio_viz_georte_fv_fp}\\ExplSyntaxOff':'';
 try{const result=await compileVizProbeDocument({case:'geographic-custom-data',scenario:record.family,documentClass:'semio',documentClassOptions:'type=paper,language=en',packages:['semio-viz'],geometry:true,preamble:['\\title{Geographic Custom Data Audit}','\\author{Semio}','\\date{}','\\errorcontextlines=200'],body:[{raw:record.tikz.replace('\\end{VizFigure}',probe+'\n\\end{VizFigure}')}]},{workDir:root+'/🗑️generated/geographic-custom-data-audit/native/'+record.family,keepWorkDir:true});consumed.push({family:record.family,complete:true,records:result});}catch(error){consumed.push({family:record.family,complete:false,error:String(error)});}
 writeFileSync(root+'/🗑️generated/geographic-custom-data-audit/native-consumption.json',JSON.stringify(consumed,null,2));console.log('[DEBUG] native '+JSON.stringify(consumed.at(-1)));
}
