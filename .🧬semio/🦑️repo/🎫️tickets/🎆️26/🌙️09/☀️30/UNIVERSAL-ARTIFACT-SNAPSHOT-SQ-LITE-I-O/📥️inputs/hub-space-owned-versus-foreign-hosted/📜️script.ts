import {readFileSync,writeFileSync} from "node:fs";
import {resolve,join} from "node:path";
const repo=resolve(import.meta.dir,"../../../../../../../../.."),root="🌎️hub/🧩️compositions/🪐️space",fixture=root+"/🧫️fixtures/🪶️snapshot-owner-census/🔣️.json",schema=root+"/🧫️fixtures/🪶️snapshot-owner-census/🧬️schema/🔣️.json",test=root+"/🧪️tests/🔬️interactive-job-catalog/🦀️.rs";
const pairs:{path:string,before:string,after:string}[]=[];
for(const path of[fixture,schema]){const before=readFileSync(join(repo,path),"utf8"),value=JSON.parse(before),corpus=path===fixture?value:value.const;if(corpus.owners.length!==2||Object.hasOwn(corpus,"hostedArtifactKinds")||corpus.metadataOnlyDefinitions.length!==0)throw Error("exact owned corpus guard");corpus.hostedArtifactKinds=[];pairs.push({path,before,after:JSON.stringify(value,null,2)+"\n"});}
const before=readFileSync(join(repo,test),"utf8"),old='assert_eq!(plugin.manifest.hosted_artifact_kinds.iter().map(|row|(row.id.clone(),row.schema.clone())).collect::<BTreeSet<_>>(),owners.iter().map(|row|(row["kind"].as_str().unwrap().to_string(),row["schema"].as_str().unwrap().to_string())).collect());';
if(before.split(old).length!==2)throw Error("exact hosted metadata expectation guard");
const next='let expected_hosted=fixture["hostedArtifactKinds"].as_array().unwrap().iter().map(|row|(row["kind"].as_str().unwrap().to_string(),row["schema"].as_str().unwrap().to_string())).collect::<BTreeSet<_>>();\n    assert_eq!(plugin.manifest.hosted_artifact_kinds.iter().map(|row|(row.id.clone(),row.schema.clone())).collect::<BTreeSet<_>>(),expected_hosted,"foreign hosted membership is independent of the two complete owned Runtime declarations");';
pairs.push({path:test,before,after:before.replace(old,next)});writeFileSync(join(import.meta.dir,"guarded-pairs.json"),JSON.stringify({pairs},null,2)+"\n");
for(const p of pairs)if(readFileSync(join(repo,p.path),"utf8")!==p.before)throw Error("concurrent owned/hosted guard "+p.path);
for(const p of pairs)writeFileSync(join(repo,p.path),p.after);
console.log("[DEBUG] Hub complete owned Runtime2 and separate authored foreign-hosted0 census mounted paths=3 all_provider_assertions_preserved=true");
