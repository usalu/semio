import { createRoot } from "react-dom/client";
import { InterpretedUiNode } from "probe:interpreter";
import { UiDocumentStore } from "probe:document-store";
import { uiIntentToActionDescriptor } from "probe:action-bridge";
import { UiI18nProvider,createShellI18nInstance } from "@semio-tech/ui-react";
import fixture from "probe:fixture";
import "probe:styles";
const windowId="procedural.play.inspection";
const store=new UiDocumentStore(windowId);
const compose=({prefix,unit,repeat,suffix})=>prefix+unit.repeat(repeat)+suffix;
const locale=new URLSearchParams(location.search).get("locale")==="de"?"de":"en";
const label=locale==="de"?"Notizwert":"Note value";
const layer={kind:"stack",axis:"vertical",gap:"none",padding:{all:"none"},align:"stretch",justify:"start",grow:false,wrap:false};
const leaf={kind:"leaf",width:"hug",height:"hug"};
const row=(id,key,component,children=[],bindings=[],accessibility={})=>({id,key,component,layout:component.type==="input"||component.type==="text"?leaf:layer,children,bindings,style:{},accessibility});
const clock={revision:0};
const leafWidth=512;
const emit=(source)=>{
 const bytes=new TextEncoder().encode(source),decoder=new TextDecoder("utf8",{fatal:true,ignoreBOM:true});
 let digest=0xcbf29ce484222325n;
 for(const byte of bytes)digest=BigInt.asUintN(64,(digest^BigInt(byte))*0x100000001b3n);
 const body=bytes.length>leafWidth?"framework.input.value."+bytes.length+"."+digest.toString(16).padStart(16,"0"):undefined;
 const slices=[];
 for(let start=0;start<bytes.length;){
   let end=Math.min(start+leafWidth,bytes.length);
   while(end<bytes.length&&(bytes[end]&0xc0)===0x80)end--;
   slices.push(decoder.decode(bytes.subarray(start,end)));
   start=end;
 }
 const nodes=[
   row(1,"procedural-play-inspector",{type:"tree"},[2]),
   row(2,"procedural-play-inspector.widget",{type:"treeSection",label:"Selected widget",defaultOpen:true},[3]),
   row(3,"procedural-play-inspector.note",{type:"treeItem",label},[4]),
   row(4,"procedural-play-inspector.note.input",{type:"input",kind:"longText",value:body??source,commit:body?"blurRetainedBody":"blurRetained"},body?[5]:[],[{trigger:"commit",action:{scope:"s.procedural.generation3d",name:"setWidgetInput",version:1},args:{widgetId:"note-雪",channel:"text"}}],{label}),
   ...(body?[row(5,body,{type:"container",role:"plain"},[6]),row(6,"c0",{type:"text",value:slices[0],dataAttributes:Object.fromEntries(slices.slice(1).map((slice,index)=>[String(index+1).padStart(2,"0"),slice]))})]:[]),
 ];
 store.loadSnapshot({surface:windowId,revision:++clock.revision,root:1,layoutEpoch:0n,nodes});
};
window.inspectorProof={actions:[]};
emit(sessionStorage.getItem("note-text")??compose(fixture.edit.first));
createRoot(document.querySelector("#root")).render(
 <UiI18nProvider instance={createShellI18nInstance(locale,"native")}>
   <InterpretedUiNode store={store} windowId={windowId} onAction={()=>{}} onIntent={(intent,owner)=>{
    const action=uiIntentToActionDescriptor(intent);
    window.inspectorProof.actions.push({action,owner});
    const v=action.args.value;
    const text=typeof v==="string"?v:v.flat().join("");
    sessionStorage.setItem("note-text",text);
    queueMicrotask(()=>emit(text));
   }} />
 </UiI18nProvider>
);
