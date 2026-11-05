
const fs=require("fs");
const root="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/";
const identityType="semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>";
const staged=new Map();
function edit(tail,change){const path=root+tail;const before=fs.readFileSync(path,"utf8");const after=change(before);if(after===before)throw Error("unchanged "+path);staged.set(path,{before,after});}
function threadCalls(s,names){for(const name of names){let from=0;while(true){const start=s.indexOf(name+"(",from);if(start<0)break;if(s.slice(Math.max(0,start-3),start)==="fn "){from=start+name.length+1;continue}const open=start+name.length;let depth=1,end=open+1;for(;end<s.length&&depth;end++){if(s[end]==="(")depth++;else if(s[end]===")")depth--;}if(depth)throw Error("unbalanced call "+name);let call=s.slice(start,end);if(!call.endsWith(",cx)"))throw Error("unexpected call tail "+call);const next=call.slice(0,-4)+",identity,cx)";s=s.slice(0,start)+next+s.slice(end);from=start+next.length;}}return s;}
function once(s,a,b){if(s.split(a).length!==2)throw Error("expected one "+a.slice(0,100));return s.replace(a,b);}
edit("🧵️retained-command/🔄️full-operation/📥️emission/🦀️.rs",s=>once(s,"impl<A:ArtifactApp> MountedTypedCommandFullOperation<A>{",`impl<A:ArtifactApp> MountedTypedCommandFullOperation<A>{
 /// 🤲️ Direct emissions retain their original receiving loan while borrowing the host authoring authority.
 pub fn borrows_original_emission_authority(&self)->bool{self.original_emission_caller.is_some()&&self.completion.is_none()&&self.identity.is_none()}
 /// 🪪️ The borrowed authoring route requires the exact mounted source, decoder and issued lease.
 pub fn validate_original_emission_authoring(&self,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
  if !self.borrows_original_emission_authority()||self.cancellation_lease.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original emission authoring requires its retained caller and publication lease").into())}
  self.validate_original_producer_source(cx)
 }
`));
edit("🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🏃️mounted-run-and-pump/🦀️.rs",s=>{
s=once(s,"    async fn publish_mounted_typed_operation_run_admitted(",`    /// 🤲️ Publishes a direct emission through the actual caller loan without creating another authoring ceiling.
    pub async fn publish_original_emission_run(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>,identity:&mut ${identityType},cx:&mut semio_framework_job::StepContext<'_>,retained:RetainedCloneGrant)->Result<(),semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        mounted.validate_original_emission_authoring(cx)?;
        self.publish_mounted_typed_operation_run_admitted(mounted,identity,cx,retained).await?;
        if mounted.cancellation_lease.as_ref().is_some_and(|lease|lease.token.is_cancelled_now()||lease.publication_claim.is_cancelled()){return Ok(())}
        self.child_composition.follow_derivable_children(&mut *self.stores,identity).await.map_err(Into::into)
    }

    async fn publish_mounted_typed_operation_run_admitted(`);
s=once(s,"            let owned_child_route = !acknowledged_page && mounted.owned_child_group.is_some();","            let cancelled=mounted.cancellation_lease.as_ref().is_some_and(|lease|lease.token.is_cancelled_now()||lease.publication_claim.is_cancelled());\n            let owned_child_route = !acknowledged_page && !cancelled && mounted.owned_child_group.is_some();");
s=s.replaceAll("else if !acknowledged_page &&","else if !acknowledged_page && !cancelled &&");
return s;});
edit("🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🔁️stall-witness-and-unit-advance/🦀️.rs",s=>{
s=once(s,"advance_typed_operation_publication_one(&mut self,cx:","advance_typed_operation_publication_one(&mut self,identity:&mut "+identityType+",cx:");
s=once(s,"self.advance_typed_operation_publication_unit(cx)","self.advance_typed_operation_publication_unit(identity,cx)");
s=once(s,"advance_typed_operation_publication_unit(&mut self,cx:","advance_typed_operation_publication_unit(&mut self,identity:&mut "+identityType+",cx:");
const start=s.indexOf("        if self.tools.tool_operations.get(operation_id).is_some_and(|mounted|mounted.cancellation_lease.is_none()");
const end=s.indexOf("        if self.tools.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Worker)",start);
if(start<0||end<0)throw Error("lazy block boundaries");
s=s.slice(0,start)+s.slice(end);
s=once(s,"        let outcome = self.publish_mounted_typed_operation_run(&mut mounted,cx,retained).await;\n        let mut driver_result = Ok(());",`        let outcome=if mounted.borrows_original_emission_authority(){self.publish_original_emission_run(&mut mounted,identity,cx,retained).await}else{self.publish_mounted_typed_operation_run(&mut mounted,cx,retained).await};`);
const revalidationStart=s.indexOf("        if driver_result.is_ok() && mounted.published_artifact");
const revalidationEnd=s.indexOf("        self.tools.tool_operations.insert_admitted(operation_id, mounted);",revalidationStart);
if(revalidationStart<0||revalidationEnd<0)throw Error("revalidation boundaries");
s=s.slice(0,revalidationStart)+"        let driver_result=self.revalidate_mounted_publication(&mut mounted,identity,cx).await;\n"+s.slice(revalidationEnd);
s=once(s,"        driver_result.map_err(Into::into)","        driver_result");
const mark="    /// 🪟️ Continues the window-config authority refresh";
s=once(s,mark,`    /// 🧭️ Revalidates a published document through the same owned completion or original caller loan.
    async fn revalidate_mounted_publication(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>,identity:&mut ${identityType},cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        if !mounted.published_artifact||mounted.interaction_revalidated{return Ok(())}
        let meta=mounted.meta.clone();
        if mounted.borrows_original_emission_authority(){
            mounted.validate_original_emission_authoring(cx)?;
            self.as_render().revalidate_interaction_on_document_change(&meta,identity).await?;
            mounted.interaction_revalidated=true;
            return Ok(())
        }
        let receipt=mounted.identity.take().ok_or_else(||plugin_sdk_fault("mounted revalidation lacks its original authoring receipt"))?;
        let lease=mounted.cancellation_lease.as_ref().ok_or_else(||plugin_sdk_fault("mounted revalidation lacks its original cancellation lease"))?;
        let token=lease.token.clone();let publication=lease.publication_claim.clone();let mut progress=mounted.identity_progress;
        let mut observer=|next:semio_framework_value::native_encoding::NativeEncodeProgress|{progress=next;!token.is_cancelled_now()&&!publication.is_cancelled()};
        let mut owned_identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let result=self.as_render().revalidate_interaction_on_document_change(&meta,&mut owned_identity).await;
        mounted.identity=Some(owned_identity.pause().map_err(|error|plugin_sdk_fault(error.to_string()))?);drop(observer);mounted.identity_progress=progress;
        result?;mounted.interaction_revalidated=true;Ok(())
    }

`+mark);
return s;});
edit("🧵️instance/🧾️typed-operations/📥️staging/⌨️typing/🦀️.rs",s=>{
for(const name of ["prepare_original_tool_dispatch","advance_original_typing_before","advance_original_typing_dispatch","advance_original_typing_pass"]){
 const re=new RegExp("(fn "+name+"\\([^\\n]*),cx:");if(!re.test(s))throw Error(name+" signature");s=s.replace(re,"$1,identity:&mut "+identityType+",cx:");
}
s=s.replaceAll("advance_original_typing_dispatch(action,args,metadata,typing,cx)","advance_original_typing_dispatch(action,args,metadata,typing,identity,cx)").replaceAll("advance_original_typing_pass(action,args,metadata,typing,false,cx)","advance_original_typing_pass(action,args,metadata,typing,false,identity,cx)").replaceAll("advance_original_typing_pass(action,args,metadata,typing,true,cx)","advance_original_typing_pass(action,args,metadata,typing,true,identity,cx)").replaceAll("advance_typed_operation_publication_one(cx)","advance_typed_operation_publication_one(identity,cx)");
return s;});
edit("🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs",s=>{
for(const name of ["drive_original_host_configuration","capture_original_direct_tool_tag","prepare_action_typing_before","prepare_action_tool_dispatch"]){
 const re=new RegExp("(fn "+name+"\\([^\\n]*),cx:");if(!re.test(s))throw Error(name+" signature");s=s.replace(re,"$1,identity:&mut "+identityType+",cx:");
}
const callNames=["prepare_original_tool_dispatch","advance_original_typing_before","drive_original_host_configuration","prepare_action_typing_before","prepare_action_tool_dispatch"];
s=threadCalls(s,callNames);
return s;});
edit("🧵️instance/🔌️plugin-app-impl/🦀️.rs",s=>{
s=once(s,"self.typed_ctx().advance_typed_operation_publication_one(cx)","self.typed_ctx().advance_typed_operation_publication_one(identity,cx)");
return threadCalls(s,["capture_original_direct_tool_tag"]);});
edit("🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs",s=>once(s,"advance_typed_operation_publication_one(same)","advance_typed_operation_publication_one(identity,same)"));
edit("🧪️tests/🔬️app-typed-command-full-operation/⚖️slot-and-publication-fairness/🦀️.rs",s=>once(s,"advance_typed_operation_publication_one(same)","advance_typed_operation_publication_one(&mut identity,same)"));
for(const[path,{before}]of staged)if(fs.readFileSync(path,"utf8")!==before)throw Error("concurrent change "+path);
for(const[path,{after}]of staged){fs.writeFileSync(path,after);console.log(path);}
