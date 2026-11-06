import * as fs from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs",before=fs.readFileSync(path,"utf8"),a=before.indexOf("    async fn exchange_generic"),b=before.indexOf("    /// 📄️ Admits",a);if(a<0||b<a)throw Error("exchange controller changed");
const body=`    async fn exchange_generic<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,effects:&mut Vec<Effect>,identity:&mut EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        let Some(CommandIngressOwner::Generic{cursor,command,exchange,exchange_close,output,..})=self.loan.state_mut()else{return Ok(())};let cursor=cursor.clone();
        self.status=match advance_command_cursor(cursor.clone()){Ok(terminal)=>CommandIngressStatus::CommandPending(terminal),Err(cursor)=>CommandIngressStatus::Fault{cursor,fault:b"plugin.command-page-index-exhausted".to_vec()}};
        if matches!(command.as_ref(),Some((_,semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress::FailedDecoded{..}))){if !close_exchange_owner(exchange,exchange_close,cx)?{return Ok(())}let Some((seq,failed))=command.take()else{unreachable!()};match failed.retire_failed_step_on_original(cx){semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Pending(failed)=>*command=Some((seq,failed)),semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Failed(failure)=>return Err(failure)}return Ok(())}
        if output.is_none(){let state=exchange.as_mut().expect("retained Source exchange state precedes command execution");match plugin_exchange_turn(runtime,cursor.instance,command,state,identity,cx).await{
            Ok(produced)=>{if state.applied{*output=Some(produced)}else{return route_exchange_output(cursor.instance,produced,effects,cx)}},
            Err(failure)=>{if matches!(command.as_ref(),Some((_,semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress::Decoded(_)))){semio_plugin_runtime_exchange::exch_plugin_exchange::retain_decoded_exchange_failure(command,Some(failure),(cx.operation(),cx.generation()),None);return Ok(())}return Err(failure)}
        }}
        if !close_exchange_owner(exchange,exchange_close,cx)?{return Ok(())}
        if let Some((_,semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress::Decoded(owner)))=command.as_mut(){if !cx.with_original_retirement(|cx|owner.close_original_step(cx))?{return Ok(())}}
        if command.is_some(){let bytes=std::mem::size_of_val(command);let grant=cx.retained_grant();if cx.retirement_should_yield()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(())}command.take();cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);}
        let mut produced=output.take().expect("actual semantic output survives controlled input retirement");self.status=terminal_command_ingress(cursor.clone(),produced.command_terminal_fault.take());self.loan.put_state(None);route_exchange_output(cursor.instance,produced,effects,cx)
    }

`;
if(fs.readFileSync(path,"utf8")!==before)throw Error("concurrent exchange controller edit");const temp=import.meta.dir+"/🗑️generated/exchange-controller.tmp";fs.writeFileSync(temp,before.slice(0,a)+body+before.slice(b));fs.renameSync(temp,path);
