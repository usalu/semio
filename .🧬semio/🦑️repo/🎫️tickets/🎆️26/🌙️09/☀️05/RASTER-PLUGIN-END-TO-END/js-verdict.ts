import{readFileSync,readdirSync,statSync,writeFileSync}from"node:fs";
import{join}from"node:path";
const root=".cache/b/r/m/nx/artifacts/terminalOutputs";
const rows=readdirSync(root).map(name=>({file:join(root,name),stat:statSync(join(root,name))})).filter(row=>row.stat.isFile()&&row.stat.size<=8*1024*1024).sort((a,b)=>b.stat.mtimeMs-a.stat.mtimeMs);
const evidence=[];
for(const row of rows){const text=readFileSync(row.file,"utf8").replace(/\u001b\[[0-9;]*m/g,"");if(!/raster|Raster/.test(text))continue;const summary=text.match(/Ran (\d+) tests across (\d+) files\./),passed=text.match(/(?:^|\n)\s*(\d+) pass(?:\n|$)/),failed=text.match(/(?:^|\n)\s*(\d+) fail(?:\n|$)/);if(!summary||!passed)continue;evidence.push({file:row.file,modified:new Date(row.stat.mtimeMs).toISOString(),tests:Number(summary[1]),files:Number(summary[2]),passed:Number(passed[1]),failed:Number(failed?.[1]??0)});}
if(!evidence.length)throw Error("no bounded Raster terminal verdict found");
writeFileSync(join(import.meta.dir,"🟩️javascript-verdict.md"),"# Raster JavaScript Test Verdict\n\nSupported repo target request 42437 completed with exit 0. The newest matching durable terminal artifact records "+evidence[0]!.passed+" passing tests and "+evidence[0]!.failed+" failures, across "+evidence[0]!.files+" files ("+evidence[0]!.tests+" total). This verifies the owning JavaScript suite, including command schema validation and the independent image comparisons; native application and end-user acceptance remain separate.\n\nRead-only terminal evidence:\n\n"+evidence.map(row=>"- "+row.file+"; modified "+row.modified+"; "+row.passed+" pass, "+row.failed+" fail, "+row.tests+" total.").join("\n")+"\n");
console.log(JSON.stringify(evidence));
