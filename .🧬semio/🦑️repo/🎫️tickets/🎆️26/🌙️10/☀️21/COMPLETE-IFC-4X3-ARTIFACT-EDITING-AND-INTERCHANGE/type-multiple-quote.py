from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type-guidance.rs');s=p.read_text(encoding='utf8')
s=s.replace('key.capacity()+value.as_str().map_or(0,|text|text.len())','key.capacity()+match value{DslValue::String(value)=>value.capacity(),_=>0}').replace('    let capacity=capacity+text.len();\n','')
s=s.replace('            23 => demand.capacity_bytes=', '            4 => demand.copy_bytes+=size_of::<Field>()+size_of::<super::type_input::Input>(),\n            24 if self.page.as_ref().is_some_and(|page|self.state.field==page.fields.len())=>{},\n            23 => demand.capacity_bytes=')
s=s.replace('64+size_of::<String>()','128+size_of::<String>()')
data=s.encode();old=p.stat().st_size
with p.open('r+b')as f:f.write(data);f.write(b' '*max(0,old-len(data)))
p=Path('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️21/COMPLETE-IFC-4X3-ARTIFACT-EDITING-AND-INTERCHANGE/host-type-multiple-input-plan.md');s=p.read_text(encoding='utf8').replace('retain the original request through the existing RetainedCloneSource pattern, copy existing raw rows through the actual retained DslValue cursor one row/child/scaffold frontier per turn, and build only the missing blank row.','retain the original donated request unchanged, reconstruct only the already closed two-field reference-row structure under original allocation admission, copy each original raw text in exact UTF-8 chunks of at most 64 bytes per turn, and build only the missing blank row. The closed row grammar makes a general graph/request clone unnecessary; every reserved row vector/string and partial row remains on the typed owner close ladder.')
p.write_text(s,encoding='utf8')
