import * as fs from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs",before=fs.readFileSync(path,"utf8");let text=before;
function one(old:string,value:string){if(text.split(old).length!==2)throw Error("exchange source region changed: "+old.slice(0,80));text=text.replace(old,value)}
one("command: Option<(u64, PluginCommandIngress)>, identity:","command: &mut Option<(u64, PluginCommandIngress)>, state:&mut semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState, identity:");
one("        output.retry_command = command;","        output.retry_command = None;");
one("    let mut frames: Vec<protocol::AppFrame> = Vec::new();\n    let mut effect_bytes: Vec<Vec<u8>> = Vec::new();\n    let mut event_bytes: Vec<Vec<u8>> = Vec::new();","    let mut frames=&mut state.frames;\n    let mut effect_bytes=&mut state.effects;\n    let mut event_bytes=&mut state.events;");
const start=text.indexOf("    let command = match command {"),end=text.indexOf("    let presence_outcome =",start);if(start<0||end<start)throw Error("command advance region changed");
text=text.slice(0,start)+`    if let Some((seq,ingress))=command.take(){match ingress.advance_original(COMMAND_INGRESS_MOVES_PER_TURN,cx){
        PluginCommandIngressStep::Pending(ingress)=>{*command=Some((seq,ingress));return Ok(PluginExchangeOutput::default())},
        PluginCommandIngressStep::Ready(owner)=>*command=Some((seq,PluginCommandIngress::Decoded(owner))),
        PluginCommandIngressStep::TerminalOriginalFailure(failure)=>return Err(failure.into()),
        PluginCommandIngressStep::TerminalFault(fault)=>{command_terminal_fault=Some(semio_framework_diagnostic::encode_fault_bytes(&fault));push_app_fault(frames,Some(seq),fault).await;return Ok(PluginExchangeOutput{frames:encode_exchange_frames(frames).await,effects:std::mem::take(effect_bytes),events:std::mem::take(event_bytes),command_terminal_fault,..Default::default()})}
    }}
`+text.slice(end);
const pa=text.indexOf("    let presence_outcome = match presence_outcome {"),pb=text.indexOf("    if let Some(outcome) = presence_outcome",pa);if(pa<0||pb<pa)throw Error("presence failure region changed");text=text.slice(0,pa)+"    let presence_outcome=presence_outcome?;\n"+text.slice(pb);
one("    if let Some((envelope_seq, mut owner)) = command {","    if let Some((envelope_seq,PluginCommandIngress::Decoded(owner)))=command.as_mut(){let envelope_seq=*envelope_seq;");
const old='result=>{let ingress=match result{Ok(None)=>PluginCommandIngress::Decoded(owner),Err(failure)=>{let identity=owner.receiving.as_ref().map_or((cx.operation(),cx.generation()),|owner|owner.identity());PluginCommandIngress::FailedDecoded{owner,failure:Some(failure),identity,close_failure:None,compound:None,raw_media:None}},Ok(Some(_))=>unreachable!()};return Ok(PluginExchangeOutput{frames:encode_exchange_frames(&frames).await,effects:effect_bytes,events:event_bytes,retry_command:Some((envelope_seq,ingress)),command_terminal_fault,presence_pending,presence_terminal,presence_terminal_fault,..Default::default()})}';
one(old,'result=>{if let Err(failure)=result{let identity=owner.receiving.as_ref().map_or((cx.operation(),cx.generation()),|owner|owner.identity());retain_decoded_exchange_failure(command,Some(failure),identity,None);}return Ok(PluginExchangeOutput::default())}');
const ma=text.indexOf("        if media_pending{"),mb=text.indexOf("        let original_media_applied=",ma);if(ma<0||mb<ma)throw Error("media pending source region changed");text=text.slice(0,ma)+`        if media_pending{if original_media_failure.is_some()||original_media_domain.is_some(){retain_decoded_exchange_failure(command,original_media_failure,(cx.operation(),cx.generation()),original_media_domain)}return Ok(PluginExchangeOutput::default())}
`+text.slice(mb);
for(const[name,field]of[["original_preview","preview"],["original_configuration","configuration"],["original_media_artifact","media_artifact"],["original_media_in","media_input"]]){
 text=text.replace("        let mut "+name+"=None;\n","");text=text.replaceAll(name,"state."+field);
}
one("        if let Some((seq,result))=manifest_result{","        if let Some(result)=manifest_result{state.manifest=Some(result)}\n        if let Some((seq,result))=state.manifest.as_ref(){let seq=*seq;");
text=text.replaceAll("state.configuration.take().expect", "state.configuration.as_ref().expect").replaceAll("state.preview.take().expect", "state.preview.as_ref().expect");
const helper=`
fn retain_decoded_exchange_failure(slot:&mut Option<(u64,PluginCommandIngress)>,failure:Option<semio_framework_job::OriginalTurnFailure>,identity:(semio_framework_job::OperationId,semio_framework_job::Generation),raw_media:Option<semio_plugin_decl_app::MediaArtifactError>){
    let Some((seq,PluginCommandIngress::Decoded(owner)))=slot.take()else{unreachable!("decoded exchange retains its exact Source owner")};*slot=Some((seq,PluginCommandIngress::FailedDecoded{owner,failure,identity,close_failure:None,compound:None,raw_media}));
}
`;
text+=helper;
if(fs.readFileSync(path,"utf8")!==before)throw Error("concurrent exchange source owner edit");const temp=import.meta.dir+"/🗑️generated/exchange-source-owner.tmp";fs.writeFileSync(temp,text);fs.renameSync(temp,path);
