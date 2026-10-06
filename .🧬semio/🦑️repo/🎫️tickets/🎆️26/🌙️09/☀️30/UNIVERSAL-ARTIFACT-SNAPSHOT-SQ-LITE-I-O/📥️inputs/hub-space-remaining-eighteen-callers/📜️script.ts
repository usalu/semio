import {readFileSync,writeFileSync} from "node:fs";
import {resolve,join} from "node:path";
const repo=resolve(import.meta.dir,"../../../../../../../../.."),root="🌎️hub/🧩️compositions/🪐️space",engine=join(root,"⚙️engine/🪐️space"),pairs=[];
function edit(path:string,changes:[string,string][]){const before=readFileSync(join(repo,path),"utf8");let after=before;for(const [old,next]of changes){if(!after.includes(old))throw Error("exact remaining caller guard "+path+" "+old);after=after.replaceAll(old,next);}pairs.push({path,before,after});}
const en="ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)";
edit(join(engine,"🦀️.rs"),[
 ["    is_de: bool,","    axes: &semio_framework_plugin::ViewModel,"],
 ["let mut menu = Menu::of(registry);","let mut menu = Menu::of(registry, axes);"],
 ['let phrase = selection_count_phrase(is_de, &[(nodes.len().max(if hit_node.is_some() && nodes.is_empty() { 1 } else { 0 }), if is_de { "Knoten" } else { "node" }, if is_de { "Knoten" } else { "nodes" })]);','let phrase = selection_count_phrase(axes.locale, &[(nodes.len().max(if hit_node.is_some() && nodes.is_empty() { 1 } else { 0 }), semio_framework_plugin::SelectionKind::Node)]);'],
 ['let remove_label = if phrase.is_empty() { labels.context_remove.as_str().to_string() } else { format!("{} ({phrase})", labels.context_remove.as_str()) };','let remove_label = phrase.map_or_else(|| labels.context_remove.as_str().to_string(), |phrase| format!("{} ({phrase})", labels.context_remove.as_str()));'],
 ['        let is_de = matches!(view_state.locale, semio_framework_ui_locale::Locale::De);\n        space_workflow_context_menu_items(registry, labels, is_de, request.surface.as_ref(), &[]).await','        space_workflow_context_menu_items(registry, labels, view_state, request.surface.as_ref(), &[]).await']
]);
edit(join(engine,"👥️presence/🦀️.rs"),[['Err(dsl::__rt::field_error(format!("unknown operation line \'{line}\'")))','Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown operation line \'{line}\'"),semio_framework_diagnostic::TextSpan::at(1,1)))']]);
for(const path of ["🧪️tests/🔬️unit/🦀️.rs","🎭️modes/🌐️main/🪟️windows/🔄️workflow/🦀️.rs"])edit(join(engine,path),[["dsl::ToValue","semio_framework_value::ToValue"]]);
edit(join(engine,"🎚️config/🧪️tests/🔬️unit/🦀️.rs"),[["let backwards = operation.inverse(config);","let backwards = operation.inverse(config).expect(\"valid inverse config mutation\");"]]);
for(const panel of ["🛍️catalogue","🔢️parameters"])edit(join(engine,"📌️panels",panel,"🧪️tests/🔬️unit/🦀️.rs"),[["ViewModel { tree_windows: requests, ..Default::default() }","ViewModel { tree_windows: requests, .."+en+" }"]]);
edit(join(root,"🧪️tests/🔬️retained-store-footprint/🦀️.rs"),[['::protocol::Mutation::inverse(&mutation, &base).len()','::protocol::Mutation::inverse(&mutation, &base).expect("valid genuine Home inverse").len()']]);
edit(join(engine,"🎮️commands/🩹️patch-parameter/🦀️.rs"),[["json::parse(&payload.value)","json::parse(&payload.value, json::JsonMemberPolicy::Reject)"]]);
edit(join(engine,"🎚️config/🦀️.rs"),[["let spec = (variants[ordinal].1)();","let spec = (variants[ordinal].1.ordinary)();"]]);
writeFileSync(join(import.meta.dir,"guarded-pairs.json"),JSON.stringify({pairs},null,2)+"\n");
for(const pair of pairs)if(readFileSync(join(repo,pair.path),"utf8")!==pair.before)throw Error("concurrent exact remaining caller guard "+pair.path);
for(const pair of pairs)writeFileSync(join(repo,pair.path),pair.after);
console.log("[DEBUG] Hub Space current remaining caller closure mounted paths="+pairs.length+" original_assertions_preserved=true genuine_view_axes=true");
