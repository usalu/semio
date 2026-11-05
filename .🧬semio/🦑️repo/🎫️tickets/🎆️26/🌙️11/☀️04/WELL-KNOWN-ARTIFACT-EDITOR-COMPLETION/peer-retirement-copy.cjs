
const fs=require("fs");
const file="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️peer/🦀️.rs";
const before=fs.readFileSync(file,"utf8");const start=before.indexOf("impl PeerPresenceRootRetirement {"),end=before.indexOf("impl Drop for PeerPresenceRootRetirement",start);if(start<0||end<0)throw Error("owner boundary");let s=before.slice(start,end);
function one(a,b){if(s.split(a).length!==2)throw Error("expected one "+a.slice(0,110));s=s.replace(a,b);}
one("        if let Some(entry) = self.entry.as_ref() {\n            return Ok({let mut demand=entry.next_demand(body)?;","        if let Some(entry) = self.entry.as_ref() {\n            if entry.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:2*size_of::<Option<semio_framework_value::retirement::arc_custody::SharedControlledRetirement<PeerPresenceEntry>>>(),depth:1,..Default::default()})}\n            return Ok({let mut demand=entry.next_demand(body)?;");
one("copy_bytes: size_of::<PeerPresenceRoot>(), release_bytes:","copy_bytes: 2*size_of::<Option<std::sync::Arc<PeerPresenceRoot>>>()+2*size_of::<Option<PeerPresenceRoot>>(), release_bytes:");
one("            return Ok(RetirementDemand { copy_bytes: if self.cursor < root.len { size_of::<std::sync::Arc<PeerPresenceEntry>>() } else { 0 }, depth: 1, ..Default::default() });",`            let copy=if self.cursor<root.len{
                size_of::<usize>()+2*size_of::<Option<std::sync::Arc<PeerPresenceEntry>>>()+if root.entries[self.cursor].is_some(){2*size_of::<Option<semio_framework_value::retirement::arc_custody::SharedControlledRetirement<PeerPresenceEntry>>>()}else{0}
            }else{2*size_of::<Option<PeerPresenceRoot>>()};
            return Ok(RetirementDemand{copy_bytes:copy,depth:1,..Default::default()});`);
one("        if let Some(entry) = self.entry.as_mut() {\n            let child =",`        if let Some(entry) = self.entry.as_mut() {
            if entry.terminal_is_empty(){self.entry.take();return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))}
            let child =`);
one("            if entry.terminal_is_empty() { drop(self.entry.take()); }\n","");
one("copied_bytes: size_of::<PeerPresenceRoot>(), released_bytes: release","copied_bytes: demand.copy_bytes, released_bytes: release");
one("            let copy = size_of::<std::sync::Arc<PeerPresenceEntry>>();\n            if grant.maximum_copy_bytes < copy { return Ok(PluginLifecycleStep::Progress(empty)); }\n","");
one("copied_bytes: copy, ..empty","copied_bytes: demand.copy_bytes, ..empty");
one("            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));","            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes:demand.copy_bytes, ..empty }));");
one("        root.len = 0;\n        drop(self.owned.take());\n        Ok(PluginLifecycleStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))","        drop(self.owned.take());\n        Ok(PluginLifecycleStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes:demand.copy_bytes, ..empty }))");
if(fs.readFileSync(file,"utf8")!==before)throw Error("concurrent owner edit");fs.writeFileSync(file,before.slice(0,start)+s+before.slice(end));
console.log(file);
