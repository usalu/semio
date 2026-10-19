import {readFileSync,existsSync} from "node:fs";
import {join,dirname,resolve} from "node:path";
const root=process.cwd(),C=process.argv[2]!;
const rows:any[]=JSON.parse(readFileSync(join(C,"🗑️generated/store-out/census.json"),"utf8"));
const verbs=new Set(["build","fonts","build-release","build-cli","compile","publish","wasm","native-build","mcp-build"]);
const hooked={cargo:/buildAdmittedCargoArtifactsV1|buildRepositoryCargoArtifacts|runArtifactRustPackageMain/u,declared:/runDeclaredRustBuildV1/u,stage:/stageRepositoryArtifacts|withRepositoryArtifactDestinationV1|buildViteArtifact|runViteBuild/u};
const out:any[]=[];
for(const r of rows){if(!verbs.has(r.verb)&&!verbs.has(r.target))continue;
 const project=JSON.parse(readFileSync(join(root,r.file),"utf8")),target=project.targets[r.target],m=/^bun (\S*📜️script\.ts) (\S+)/u.exec(target.options.command)!;
 const cwd=resolve(root,target.options.cwd??dirname(r.file)),script=resolve(cwd,m[1]),text=readFileSync(script,"utf8"),verb=m[2];
 const reg=new RegExp(String.raw`register\(\s*"${verb}"\s*,\s*(\w+)`,"u").exec(text);let body=/runArtifactRustPackageMain/u.test(text)&&verb==="build"?"runArtifactRustPackageMain":"";
 if(reg){const cls=reg[1]!,start=text.search(new RegExp(String.raw`class\s+${cls}\b`,"u"));if(start>=0){const next=text.slice(start+10).search(/\nclass\s|\nconst router/u);body=text.slice(start,next<0?undefined:start+10+next);}}
 const cargo=hooked.cargo.test(body),declared=hooked.declared.test(body),stage=hooked.stage.test(body),intent=target.metadata?.semio?.rustBuildIntent!==undefined;
 out.push({project:r.project,target:r.target,file:r.file,script:script.replace(root+"/",""),verb,cls:reg?.[1]??null,cargo,declared,stage,intent,whole:r.kinds,cache:r.cache,outputs:r.outputs});}
console.log(JSON.stringify(out,null,1));
