import { createRoot } from "react-dom/client";
import { InterpretedUiNode } from "probe:interpreter";
import { UiDocumentStore } from "probe:document-store";
import { UiI18nProvider,createShellI18nInstance } from "@semio-tech/ui-react";
import fixture from "probe:fixture";
import projected from "probe:native";
import "probe:styles";

const store=new UiDocumentStore("incoming-hover");
const stack={kind:"stack",axis:"vertical",gap:"none",padding:{all:"none"},align:"stretch",justify:"start",grow:false,wrap:false};
const leaf={kind:"leaf",width:"hug",height:"hug"};
const node=(id,key,component,children=[],style={},accessibility={},layout=stack)=>({id,key,component,children,layout,style,accessibility,bindings:[]});
let revision=0;
function publish(index){
 const marked=new Set(projected[index]);
 const values=fixture.candidates.map((port,i)=>node(10+i,port.id,{type:"treeItem",label:port.id,selected:port.id==="source@!isurface"},[],marked.has(port.id)?{highlighted:true}:{}));
 const plane=marked.has("free@!iplane")?{highlighted:true}:{};
 const records=[
   node(1,"root",{type:"container",role:"plain"},[2,50,53]),
   node(2,"values-tree",{type:"tree"},[3]),
   node(3,"values",{type:"treeSection",label:"Values",defaultOpen:true},values.map(row=>row.id)),
   ...values,
   node(50,"free.input.plane.origin.coordinates",{type:"container",role:"group",label:"Plane origin"},[51],plane),
   node(51,"free.input.plane.origin.x",{type:"container",role:"field",label:"Plane origin x"},[52]),
   node(52,"free.input.plane.origin.x.input",{type:"input",kind:"number",value:"1"},[],{},{label:"Plane origin x"},leaf),
   node(53,"free.input.plane.normal.coordinates",{type:"container",role:"group",label:"Plane normal"},[54],plane),
   node(54,"free.input.plane.normal.z",{type:"container",role:"field",label:"Plane normal z"},[55]),
   node(55,"free.input.plane.normal.z.input",{type:"input",kind:"number",value:"0"},[],{},{label:"Plane normal z"},leaf),
 ];
 store.loadSnapshot({surface:"incoming-hover",revision:++revision,root:1,layoutEpoch:0n,nodes:records});
}
window.incomingHoverProof={publish,intents:[]};
publish(projected.length-1);
createRoot(document.querySelector("#root")).render(
 <UiI18nProvider instance={createShellI18nInstance("en","native")}>
  <InterpretedUiNode store={store} windowId="incoming-hover" onAction={()=>{}} onIntent={intent=>window.incomingHoverProof.intents.push(intent)} />
 </UiI18nProvider>
);
