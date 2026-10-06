import {readFileSync,writeFileSync} from "node:fs";
import {resolve,join} from "node:path";
const repo=resolve(import.meta.dir,"../../../../../../../../.."),path="🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🧪️tests/🔬️unit/🦀️.rs",before=readFileSync(join(repo,path),"utf8");
const old="space_workflow_context_menu_items(&registry, labels, false,",next="space_workflow_context_menu_items(&registry, labels, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native),";
if(before.split(old).length!==3)throw Error("exact original English menu test callers guard");
const after=before.replaceAll(old,next);writeFileSync(join(import.meta.dir,"guarded-pair.json"),JSON.stringify({path,before,after},null,2)+"\n");
if(readFileSync(join(repo,path),"utf8")!==before)throw Error("concurrent menu test caller guard");writeFileSync(join(repo,path),after);
console.log("[DEBUG] Hub Space two original menu test callers explicit English Native axes mounted paths=1 assertions_preserved=true");
