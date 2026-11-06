import * as fs from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs",before=fs.readFileSync(path,"utf8");let text=before;
const start=text.indexOf("    async fn exchange_generic"),end=text.indexOf("    /// 📄️ Admits one inbound",start);if(start<0||end<start)throw Error("exchange owner region changed");
text=text.slice(0,start)+`    async fn exchange_generic<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,effects:&mut Vec<Effect>,identity:&mut EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        let Some(CommandIngressOwner::Generic{cursor,command})=self.loan.state_mut()else{return Ok(())};let cursor=cursor.clone();
        let mut output=plugin_exchange_boxed(runtime,cursor.instance,command,identity,cx).await?;
        if let Some(retry)=output.retry_command.take(){assert!(command.is_none(),"exchange transfers one retained command owner");*command=Some(retry)}
        let pending=command.is_some();
        self.status=if pending{match advance_command_cursor(cursor.clone()){Ok(terminal)=>CommandIngressStatus::CommandPending(terminal),Err(cursor)=>CommandIngressStatus::Fault{cursor,fault:b"plugin.command-page-index-exhausted".to_vec()}}}else{terminal_command_ingress(cursor.clone(),output.command_terminal_fault.take())};
        if !pending{self.loan.put_state(None)}
        route_exchange_output(cursor.instance,output,effects,cx)
    }

`+text.slice(end);
text=text.replaceAll("plugin_exchange_boxed(runtime, cursor.instance, None, identity,cx)","plugin_exchange_boxed(runtime, cursor.instance, &mut None, identity,cx)");
text=text.replace('command: command.cancel(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.command-cancelled-by-close"), "command ingress was cancelled by instance close")),','command: command.map(|(seq,command)|(seq,command.cancel(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.command-cancelled-by-close"), "command ingress was cancelled by instance close")))),');
text=text.replace('Generic{cursor,command:semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress::Encoded(command)}','Generic{cursor:cursor.clone(),command:Some((cursor.seq,semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress::Encoded(command)))}');
if(fs.readFileSync(path,"utf8")!==before)throw Error("concurrent command exchange consumer edit");const temp=import.meta.dir+"/🗑️generated/command-exchange-borrow.tmp";fs.writeFileSync(temp,text);fs.renameSync(temp,path);
