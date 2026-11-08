import{readFileSync,writeFileSync}from"node:fs";
const p="🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧳️slots/🦀️.rs",producer=p.replace("/🦀️.rs","/🏭️producer/🦀️.rs");
let s=readFileSync(producer,"utf8");s=s.replace("self.kind=None;self.source=None}step","self.kind=None}step");s=s.replace("if self.kind.is_some()&&self.source!=c.source","if self.source.is_some()&&self.source!=c.source");writeFileSync(producer,s);
s=readFileSync(p,"utf8");s=s.replace("if self.retired.is_some(){let demand=self.close_demand", "if let Some(retired)=self.retired.as_ref(){retired.validate_context(&Context{source:effect.work_source_turn(),operation:Operation::Close(None)})?;let demand=self.close_demand");writeFileSync(p,s);
