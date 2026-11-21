import { createRoot } from "react-dom/client";
import { InterpretedUiNode } from "probe:interpreter";
import { UiDocumentStore } from "probe:document-store";
import { TreeWindowContext } from "probe:tree-context";
import { createTreeWindowContextV1, createTreeWindowSchedulerV1, uiIntentToActionDescriptor } from "probe:tree-scheduler";
import { UiI18nProvider, createShellI18nInstance } from "@semio-tech/ui-react";
import { WINDOW_RESOURCE } from "@semio-tech/ui-contract/window-resource";
import fixture from "probe:fixture";
import "probe:styles";

const published = await fetch("/fixture/native").then(response => response.json());
const resourceKey = published.pages[0].slice(0,published.pages[0].lastIndexOf("."));
const windowId = "procedural3d-preview";
const bodyKey = "preview";
const store = new UiDocumentStore(windowId);
const paths = new Map();
const revisions: number[] = [];
let revision = 0;
const stack = { kind: "stack", axis: "vertical", gap: "none", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: false };
const leaf = { kind: "leaf", width: "hug", height: "hug" };
const node = (key,component,children=[],bindings=[],layout=leaf) => ({key,component,children,bindings,layout,accessibility:{},style:{}});
const scheduler = createTreeWindowSchedulerV1({
  refresh(key) { void publish().finally(() => scheduler.settled(key)); }
});
const requestFor = (entries,key) => entries.find(request=>request.nodeKey===key && request.bodyKey===bodyKey);
function windowFor(total,offset=0) { return {total,offset,rowExtent:"standard"}; }
const binding = targetId => ({trigger:"hoverPreview",action:{scope:"procedural3d-view",name:"interactionHover",version:1},args:{channel:"pointer",domainId:"graph",targetId,granularityId:"handle"}});
async function publish() {
  const requests = scheduler.viewStateFields().treeWindows ?? [];
  const page = requestFor(requests,resourceKey)?.offset ?? 1;
  function item(key,label,target,path,children,defaultOpen=false) {
    const request = requestFor(requests,path.join("␟"));
    const open = request?.open ?? defaultOpen;
    const offset = Math.min(request?.offset ?? 0,Math.max(children.length - 1,0));
    const visible = open ? children.slice(offset,offset + (request?.rows || children.length)) : [];
    return node(key,{type:"treeItem",label,defaultOpen,window:windowFor(children.length,offset)},visible,[binding(target)],stack);
  }
  const branch = async (name,typed) => {
    const parent = ["values",typed.id];
    const side = typed.input ? "in" : "out";
    const direction = [...parent, side];
    const needsImage = (requestFor(requests,parent.join("␟"))?.open ?? false)
      && (requestFor(requests,direction.join("␟"))?.open ?? false);
    let image;
    if (name==="output" && needsImage) {
      revisions.push(page);
      const data = await fetch("/fixture/page/" + page).then(response=>response.text());
      const desc = published.pages[page-1];
      const chunks = data.match(/.{1,512}/g) ?? [];
      const leaves = [];
      for (let start=0; start<chunks.length; start+=33) {
        const group=chunks.slice(start,start+33);
        leaves.push(node("chunk-"+start,{type:"text",value:group[0],dataAttributes:Object.fromEntries(group.slice(1).map((text,index)=>[String(index+1).padStart(2,"0"),text]))}));
      }
      image=node(published.imageKey,{type:"image",src:desc,alt:typed.alt,sourceBodyBytes:data.length},[node(desc,{type:"container",role:"plain"},leaves,[],stack)]);
    } else {
      image=node("framework.tree.image.input",{type:"image",src:fixture.image.dataUrl,alt:typed.alt});
    }
    const preview=node("image-preview",{type:"treeItem",label:"Image preview",defaultOpen:true},[image],[binding(typed.id)],stack);
    const dataValue=item("1","dataUrl",typed.id,[...direction,"1"],[node("source-text",{type:"text",value:"Raw data URL"})]);
    const schema=node("2",{type:"treeItem",label:"schema",description:"image"},[],[binding(typed.id)]);
    return item(typed.id,typed.alt.slice("Image preview · ".length),typed.id,parent,[item(side,typed.direction,typed.id,direction,[preview,dataValue,schema])]);
  };
  const records = await Promise.all(Object.entries(fixture.channels).map(([name,typed])=>branch(name,typed)));
  const root=node("channel-tree",{type:"tree"},[node("values",{type:"treeSection",label:"Values",defaultOpen:true,window:windowFor(records.length)},records,[],stack)],[],stack);
  const nodes=[];
  function flatten({children,...record},parent=null) {
    const path=JSON.stringify([parent,record.key]);
    if (!paths.has(path)) paths.set(path,paths.size+1);
    const id=paths.get(path);
    nodes.push({...record,id,children:children.map(child=>flatten(child,id))});
    return id;
  }
  const id=flatten(root);
  store.loadSnapshot({surface:windowId,revision:++revision,root:id,layoutEpoch:0n,nodes});
}
window.channelImageProof={intents:[],revisions,requests:()=>scheduler.viewStateFields().treeWindows??[]};
await publish();
createRoot(document.querySelector("#root")).render(
  <UiI18nProvider instance={createShellI18nInstance("en","native")}>
    <div style={{width:640,height:570,overflow:"auto"}}>
      <TreeWindowContext.Provider value={createTreeWindowContextV1(bodyKey,scheduler)}>
        <InterpretedUiNode store={store} windowId={windowId} onAction={()=>{}} onIntent={(intent)=>{window.channelImageProof.intents.push(uiIntentToActionDescriptor(intent));}} />
      </TreeWindowContext.Provider>
    </div>
  </UiI18nProvider>
);
