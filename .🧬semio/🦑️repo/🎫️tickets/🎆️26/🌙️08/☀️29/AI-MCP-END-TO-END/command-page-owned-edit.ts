import * as fs from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs";
const before=fs.readFileSync(path,"utf8"),start=before.indexOf("    async fn admit_first_generic_page"),end=before.indexOf("    /// 🏁️ Reports the status",start);
if(start<0||end<start)throw Error("owned command-page region changed");
const body=`    async fn admit_first_generic_page<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,incoming:&mut Option<(CommandPageCursor,FixedCommandPage)>,effects:&mut Vec<Effect>,identity:&mut EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        let Some((cursor,_))=incoming.as_ref()else{return Ok(())};let cursor=cursor.clone();
        if cx.should_yield()||cx.fuel_remaining()==0{self.status=CommandIngressStatus::Backpressure(cursor);return Ok(())}
        let mut pages=None;let turn=CommandPageSet::admit_source_step(&mut pages,cursor.page_count as usize,cx.retained_grant());
        self.retained=pages.map(|pages|CommandIngressOwner::GenericAssembly{cursor:cursor.clone(),pages});
        let progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_fuel(turn.work);cx.consume_retained(progress)?;turn.result?;
        let Some(CommandIngressOwner::GenericAssembly{cursor:active,pages})=self.retained.take()else{self.status=CommandIngressStatus::Backpressure(cursor);return Ok(())};
        self.continue_generic_assembly(runtime,incoming,(active,pages),effects,identity,cx).await
    }

    async fn continue_generic_assembly<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,incoming:&mut Option<(CommandPageCursor,FixedCommandPage)>,assembly:(CommandPageCursor,CommandPageSet),effects:&mut Vec<Effect>,identity:&mut EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        let(active,pages)=assembly;self.retained=Some(CommandIngressOwner::GenericAssembly{cursor:active,pages});
        let Some((cursor,_))=incoming.as_ref()else{return Ok(())};let cursor=cursor.clone();
        let Some(CommandIngressOwner::GenericAssembly{pages,..})=self.retained.as_mut()else{unreachable!()};
        if cursor.page_index as usize!=pages.len(){self.status=CommandIngressStatus::Fault{cursor,fault:b"plugin.command-page-order".to_vec()};return Ok(())}
        if cx.should_yield()||cx.fuel_remaining()==0{self.status=CommandIngressStatus::Backpressure(cursor);return Ok(())}
        let turn=pages.capture_source_step(incoming,cx.retained_grant());let progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_fuel(turn.work);cx.consume_retained(progress)?;turn.result?;
        if incoming.is_some(){self.status=CommandIngressStatus::Backpressure(cursor);return Ok(())}
        if let Some(CommandIngressOwner::GenericAssembly{cursor:active,..})=self.retained.as_mut(){active.page_index=cursor.page_index;}
        self.status=CommandIngressStatus::PageAccepted(cursor.clone());
        if cursor.page_index.checked_add(1)!=Some(cursor.page_count){return Ok(())}
        let Some(CommandIngressOwner::GenericAssembly{pages,..})=self.retained.take()else{unreachable!()};
        match semio_framework::kernel::PagedCommand::try_from_pages(pages){Ok(command)=>self.retained=Some(CommandIngressOwner::Generic{cursor,command:semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress::Encoded(command)}),Err((fault,pages))=>{self.retained=Some(CommandIngressOwner::GenericAssembly{cursor:cursor.clone(),pages});self.status=CommandIngressStatus::Fault{cursor,fault:semio_framework_diagnostic::encode_fault_bytes(&fault)};return Ok(())}}
        self.exchange_generic(runtime,effects,identity,cx).await
    }

`;
if(fs.readFileSync(path,"utf8")!==before)throw Error("concurrent command-page edit");
const temp=import.meta.dir+"/🗑️generated/command-page-owned-rs.tmp";
fs.writeFileSync(temp,before.slice(0,start)+body+before.slice(end));fs.renameSync(temp,path);
