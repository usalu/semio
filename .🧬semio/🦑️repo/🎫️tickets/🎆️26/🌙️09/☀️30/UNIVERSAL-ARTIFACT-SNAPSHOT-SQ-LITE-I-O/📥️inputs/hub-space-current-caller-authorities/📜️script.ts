import {readFileSync,writeFileSync,readdirSync} from "node:fs";
import {resolve,join,relative} from "node:path";
const repo=resolve(import.meta.dir,"../../../../../../../../..");
const root="🌎️hub/🧩️compositions/🪐️space",pairs=[];
function visit(path:string){for(const entry of readdirSync(join(repo,path),{withFileTypes:true})){const child=join(path,entry.name);if(entry.isDirectory())visit(child);else if(entry.name==="🦀️.rs"){
 const before=readFileSync(join(repo,child),"utf8");let after=before;
 for(const [old,next]of [["dsl::DslRecord","semio_framework_dsl_record_derive::DslRecord"],["dsl::DslOps","semio_framework_dsl_record_derive::DslEnum"],["dsl::DslVariants","semio_framework_dsl_record::DslVariants"],["dsl::RecordSpec","semio_framework_dsl_record::RecordSpec"],["dsl::ParseOptions","semio_framework_dsl_record::ParseOptions"],["dsl::SourceMode","semio_framework_dsl_record::SourceMode"],["dsl::JoinMode","semio_framework_dsl_record::JoinMode"],["dsl::parse(","semio_framework_dsl_record::parse("],["dsl::print(","semio_framework_dsl_record::print("],["semio_framework_plugin::LocalizedLabel","semio_framework_ui_locale::LocalizedLabel"],["semio_framework_plugin::Locale","semio_framework_ui_locale::Locale"],["semio_framework_plugin::Terminology","semio_framework_ui_locale::Terminology"],["semio_framework_plugin::app_labels","semio_framework_ui_locale::app_labels"],["store::ToValue","semio_framework_value::ToValue"]])after=after.replaceAll(old,next);
 after=after.replace(/use semio_framework_plugin::\{([^}]+)\};/g,(whole,items)=>{const names=items.split(",").map((name:string)=>name.trim()).filter(Boolean),locale=names.filter((name:string)=>["LocalizedLabel","Locale","Terminology"].includes(name));return locale.length?"use semio_framework_ui_locale::{"+locale.join(", ")+"};\nuse semio_framework_plugin::{"+names.filter((name:string)=>!locale.includes(name)).join(", ")+"};":whole;});
 after=after.replace('use semio_framework_pack_json::{self, Value};','use semio_framework_pack_json::{self as json, Value};');
 const error='Err(dsl::__rt::field_error(format!("unknown mutation line \'{line}\'")))';
 if(after.includes(error))after=after.replace(error,'Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown mutation line \'{line}\'"),semio_framework_diagnostic::TextSpan::at(1,1)))');
 if(after!==before)pairs.push({path:child,before,after});
}}}
visit(join(root,"⚙️engine"));
const path=join(root,"📦️packages/🦀️rust/Cargo.toml"),before=readFileSync(join(repo,path),"utf8"),anchor='semio-framework-dsl = { path = "../../../../../🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust" }';
if(!before.includes(anchor)||before.includes("semio-framework-dsl-record ="))throw Error("exact direct authority dependency guard");
const dependencies='semio-framework-dsl-record = { path = "../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust" }\nsemio-framework-dsl-record-derive = { path = "../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust" }\nsemio-framework-schema-state = { workspace = true }\nsemio-framework-schema-composition = { workspace = true }';
pairs.push({path,before,after:before.replace(anchor,anchor+"\n"+dependencies)});
if(pairs.length<40)throw Error("complete actual current caller roster guard");
writeFileSync(join(import.meta.dir,"guarded-pairs.json"),JSON.stringify({pairs},null,2)+"\n");
for(const pair of pairs)if(readFileSync(join(repo,pair.path),"utf8")!==pair.before)throw Error("concurrent exact caller authority guard "+pair.path);
for(const pair of pairs)writeFileSync(join(repo,pair.path),pair.after);
console.log("[DEBUG] Hub Space explicit canonical Record, Locale, schema and JSON caller authorities mounted paths="+pairs.length);
