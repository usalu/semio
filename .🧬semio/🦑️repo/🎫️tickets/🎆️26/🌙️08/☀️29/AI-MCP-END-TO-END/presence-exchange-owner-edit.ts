import * as fs from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs",before=fs.readFileSync(path,"utf8"),a=before.indexOf("    async fn exchange_presence"),b=before.indexOf("    /// 🔁️ Retries",a);if(a<0||b<a)throw Error("presence exchange changed");
const body=`    async fn exchange_presence<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,effects:&mut Vec<Effect>,identity:&mut EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        if !matches!(self.status,CommandIngressStatus::Idle){return Ok(())}let Some(CommandIngressOwner::Presence{cursor,exchange,exchange_close,terminal,terminal_fault,..})=self.loan.state_mut()else{return Ok(())};let cursor=cursor.clone();
        if !close_exchange_owner(exchange,exchange_close,cx)?{return Ok(())}
        if *terminal{let fault=terminal_fault.take();self.status=terminal_command_ingress(cursor,fault);self.loan.put_state(None);return Ok(())}
        if !semio_framework_job::admit_original_construction(cx,exchange,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:std::mem::size_of_val(exchange),capacity_bytes:0,release_bytes:0,depth:1},|_|semio_framework_value::RetainedWorkOutcome{result:Ok((semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState::empty(),semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Option<semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState>>(),..Default::default()})),work:1})?{return Ok(())}
        let mut output=plugin_exchange_turn(runtime,cursor.instance,&mut None,exchange.as_mut().expect("presence retains its Source output state"),identity,cx).await?;
        if output.presence_terminal==Some(cursor.seq){*terminal=true;*terminal_fault=output.presence_terminal_fault.take();}
        route_exchange_output(cursor.instance,output,effects,cx)
    }

`;
if(fs.readFileSync(path,"utf8")!==before)throw Error("concurrent presence exchange edit");const temp=import.meta.dir+"/🗑️generated/presence-exchange-owner.tmp";fs.writeFileSync(temp,before.slice(0,a)+body+before.slice(b));fs.renameSync(temp,path);
