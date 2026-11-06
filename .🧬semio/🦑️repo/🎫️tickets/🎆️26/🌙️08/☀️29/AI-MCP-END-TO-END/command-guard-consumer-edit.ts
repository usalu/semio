import * as fs from "node:fs";
const file="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs";
const before=fs.readFileSync(file,"utf8");let text=before;
const one=(old:string,value:string)=>{if(text.split(old).length!==2)throw Error("changed owned region: "+old);text=text.replace(old,value)};
one("    retained: Option<CommandIngressOwner>,\n    key: Option<instance_lifetime::NativeCloseKey>,","    loan: CommandIngressLoan,");
const a=text.indexOf("    fn resume<"),b=text.indexOf("    /// 🚫️ Cancels",a);
if(a<0||b<a)throw Error("resume region changed");
text=text.slice(0,a)+`    fn resume<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        step_retiring_command_ingress(cx)?;
        let ready=COMMAND_INGRESS.with(|slots|slots.borrow_mut()[COMMAND_INGRESS_LIVE_SLOT].settle_step(cx))?;if !ready{return Ok(false)}
        if let Some(key)=COMMAND_INGRESS.with(|slots|slots.borrow()[COMMAND_INGRESS_LIVE_SLOT].peek().map(|entry|entry.key)){runtime.guest_lifetimes.try_borrow().map_err(|_|reactor_close_fault("command Source lifetime is busy"))?.get(key.instance()).ok_or_else(||reactor_close_fault("retained command Source lifetime is missing"))?.command_origin.validate(cx)?;}
        if !self.loan.lend(cx)?{return Ok(false)}
        if let Some(key)=self.loan.key(){if native_close_key(runtime,key.instance()).ok()!=Some(key){self.close_instances.push(key.instance());}}
        Ok(true)
    }

`+text.slice(b);
text=text.replaceAll("self.retained.as_ref()","self.loan.state()").replaceAll("self.retained.as_mut()","self.loan.state_mut()").replaceAll("self.retained.take()","self.loan.take_state()").replaceAll("self.retained.is_none()","self.loan.state().is_none()").replaceAll("matches!(self.retained,","matches!(self.loan.state(),").replaceAll("match self.retained {","match self.loan.state() {");
let start=0;
while((start=text.indexOf("self.retained=",start))>=0||(start=text.indexOf("self.retained =",0))>=0){
 const eq=text.indexOf("=",start);let depth=0,quote=false,escape=false,end=-1;
 for(let i=eq+1;i<text.length;i++){let ch=text[i];if(quote){if(escape)escape=false;else if(ch==="\\")escape=true;else if(ch==='"')quote=false;continue}if(ch==='"'){quote=true;continue}if("({[".includes(ch))depth++;else if(")}]".includes(ch))depth--;else if(ch===";"&&depth===0){end=i;break}}
 if(end<0)throw Error("unclosed owned assignment");const rhs=text.slice(eq+1,end).trim();text=text.slice(0,start)+"self.loan.put_state("+rhs+")"+text.slice(end);start=0;
}
const old='        if self.loan.state().is_none() {\n            self.key = native_close_key(runtime, cursor.instance).ok();\n        }';
one(old,'        if self.loan.state().is_none(){let key=native_close_key(runtime,cursor.instance)?;if !self.loan.admit_entry(key,cx)?{self.status=CommandIngressStatus::Backpressure(cursor);return Ok(())}}');
text=text.replaceAll('self.key.expect("a retained owner holds its exact lifetime")','self.loan.key().expect("a retained owner holds its exact lifetime")');
const f=text.indexOf("    fn finish(mut self)"),g=text.indexOf("\n    }",f)+6;if(f<0||g<6)throw Error("finish region changed");
text=text.slice(0,f)+`    fn finish(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<CommandIngressStatus,semio_framework_job::OriginalTurnFailure>{
        let status=std::mem::replace(&mut self.status,CommandIngressStatus::Idle);let status=command_ingress_while_owned(status,self.loan.state().map(command_ingress_owner_cursor));self.loan.clear_empty_entry(cx)?;self.loan.restore()?;Ok(status)
    }`+text.slice(g);
if(fs.readFileSync(file,"utf8")!==before)throw Error("concurrent command guard consumer edit");const temp=import.meta.dir+"/🗑️generated/command-guard-consumer-rs.tmp";fs.writeFileSync(temp,text);fs.renameSync(temp,file);
