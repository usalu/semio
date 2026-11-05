import{readFileSync,writeFileSync,renameSync}from"node:fs";
const file="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🎟️unit/🦀️.rs";
let source=readFileSync(file,"utf8");const before="runtime.begin_original_unit_step(instance,export,&mut*self.input,cx)";if(source.split(before).length!==2)throw Error("expected canonical unit begin");source=source.replace(before,"runtime.begin_original_unit_step(instance,export,&mut*self.input,input_budget,cx)");const temp=import.meta.dirname+"/🗑️generated/guest-unit-budget-edit.rs";writeFileSync(temp,source);renameSync(temp,file);
