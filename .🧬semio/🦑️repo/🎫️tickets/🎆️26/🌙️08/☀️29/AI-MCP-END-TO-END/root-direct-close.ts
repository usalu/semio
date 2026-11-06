import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname}from"node:path";const t=dirname(import.meta.path);function edit(p:string,f:(s:string)=>string){const s=readFileSync(p,"utf8"),n=f(s),tmp=t+"/🗑️generated/root-direct-close-"+Math.random().toString(16).slice(2);if(n===s)throw Error("unchanged");writeFileSync(tmp,n);if(readFileSync(p,"utf8")!==s)throw Error("CAS");renameSync(tmp,p)}
edit("🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🦀️.rs",s=>{
 s=s.replace("if let Some(receiver)=self.receiver.get_mut(){receiver.begin_close()?}self.closing=true","self.closing=true");
 const a="let turn=if let Some(receiver)=self.receiver.get_mut().filter(|receiver|!receiver.terminal_is_empty()){receiver.close_step(child,actual)}else{super::close_native_receiver_backing::<I,T,_>(&mut self.receiver,&mut self.retired_header,g,actual)}";
 if(!s.includes(a))throw Error("backing branch");s=s.replace(a,"let turn=super::close_native_receiver_backing::<I,T,_>(&mut self.receiver,&mut self.retired_header,g,actual)");
 s=s.replace("original.terminal_is_empty(){Ok(fixed(0))}","original.terminal_is_empty(){Ok(fixed(size_of::<Option<OriginalInput<I,A>>>()))}");
 s=s.replace("self.original.take();return Ok(RetainedCloneStep::Complete(receipt(0)))","self.original.take();return Ok(RetainedCloneStep::Complete(receipt(size_of::<Option<OriginalInput<I,A>>>()))))");
 s=s.replace("if !self.closing{Ok(fixed(0))}","if !self.closing{Ok(fixed(size_of::<bool>()))}").replace("if !retained_clone_grant_admits(g,fixed(0)){return RetirementStep::BudgetExhausted}return match self.begin_close(){Ok(())=>RetirementStep::Progress(receipt(0),1)","if !retained_clone_grant_admits(g,fixed(size_of::<bool>())){return RetirementStep::BudgetExhausted}return match self.begin_close(){Ok(())=>RetirementStep::Progress(receipt(size_of::<bool>()),1)");
 return s;
});
edit("🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🪡️field/🦀️.rs",s=>{
 s=s.replace("self.cursor.begin_close();self.closing=true","self.closing=true");
 s=s.replace("if !self.cursor.terminal_is_empty(){return self.cursor.close_demands(body);}","if !self.cursor.terminal_is_empty(){return if self.cursor.begin_close_work_demand()?!=0{Ok(fixed(size_of::<bool>()))}else{self.cursor.close_demands(body)};}");
 s=s.replace("if !self.cursor.terminal_is_empty(){return self.cursor.close_work_demand(body)}","if !self.cursor.terminal_is_empty(){let q=self.cursor.begin_close_work_demand()?;return if q!=0{Ok(q)}else{self.cursor.close_work_demand(body)}}");
 const a="if !self.cursor.terminal_is_empty(){let turn=self.cursor.close_step(grant,actual);";
 s=s.replace(a,"if !self.cursor.terminal_is_empty(){if self.cursor.begin_close_work_demand()?!=0{let turn=self.cursor.begin_close_paid();work=turn.work;return turn.result.map(|()|RetainedCloneStep::Progress(receipt(d))).map_err(Into::into)}let turn=self.cursor.close_step(grant,actual);");
 s=s.replace("turn.result.map_err(Into::into)}\n })();RetainedWorkOutcome","turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}\n })();RetainedWorkOutcome");
 s=s.replace("pub fn ready(&self)->bool{!self.closing&&","pub fn ready(&self)->bool{!self.closing&&!self.has_pending_recovery()&&");
 return s;
});console.log("Original/Intrinsic closure now selects direct paid receiver/cursor intent");