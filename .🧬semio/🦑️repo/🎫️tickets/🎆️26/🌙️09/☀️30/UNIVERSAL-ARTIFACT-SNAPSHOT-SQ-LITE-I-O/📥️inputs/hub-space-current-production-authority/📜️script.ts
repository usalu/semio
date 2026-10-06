import {readFileSync,writeFileSync} from "node:fs";
import {resolve,join} from "node:path";

const repo=resolve(import.meta.dir,"../../../../../../../../..");
const root="🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space";
const pairs=[];
for(const [leaf,old,newValue]of [["🎚️config",'#[dsl(id = "s.spacecfg")]','#[artifact(id = "s.spacecfg")]'],["👥️presence",'#[dsl(extension = "space.presence")]','#[artifact(extension = "space.presence")]']]){
    const path=join(root,leaf,"🦀️.rs"),before=readFileSync(join(repo,path),"utf8"),derive="value_derive::FromValue, dsl::DslArtifact)]";
    if(!before.includes(old)||!before.includes(derive))throw Error("current artifact authority guard "+path);
    const after=before.replace(old,newValue).replace(derive,"value_derive::FromValue, dsl::DslRecord, dsl::DslArtifact)]");
    pairs.push({path,before,after});
}
const path=join(root,"🦀️.rs"),before=readFileSync(join(repo,path),"utf8"),old="InteractiveJobClassification, Label, LocalizedLabel, MergeMode";
if(!before.includes(old)||before.includes("use semio_framework_ui_locale::{Label, LocalizedLabel};"))throw Error("current locale authority guard");
pairs.push({path,before,after:before.replace(old,"InteractiveJobClassification, MergeMode").replace("use std::collections::HashMap;","use semio_framework_ui_locale::{Label, LocalizedLabel};\nuse std::collections::HashMap;")});
writeFileSync(join(import.meta.dir,"guarded-pairs.json"),JSON.stringify({pairs},null,2)+"\n");
for(const pair of pairs)if(readFileSync(join(repo,pair.path),"utf8")!==pair.before)throw Error("concurrent current authority guard "+pair.path);
for(const pair of pairs)writeFileSync(join(repo,pair.path),pair.after);
console.log("[DEBUG] Hub Space exact Artifact metadata, canonical Record derives and Locale authority mounted paths="+pairs.length);
