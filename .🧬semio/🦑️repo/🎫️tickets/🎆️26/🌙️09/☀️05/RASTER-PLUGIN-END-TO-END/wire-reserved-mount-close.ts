import {readFileSync,writeFileSync} from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs";let source=readFileSync(path,"utf8");
function replace(old:string,next:string){if(source.split(old).length!==2)throw new Error(`expected one occurrence: ${old}`);source=source.replace(old,next)}
replace("}else if self.owns_original_reserved_permit(same){","}else if self.owns_original_reserved_mount(same){self.close_original_reserved_mount(same)?}else if self.owns_original_reserved_permit(same){");
replace("&&!self.owns_original_reserved_permit(cx)&&!self.owns_original_host_event_source(cx)","&&!self.owns_original_reserved_mount(cx)&&!self.owns_original_reserved_permit(cx)&&!self.owns_original_host_event_source(cx)");
writeFileSync(path,source);
