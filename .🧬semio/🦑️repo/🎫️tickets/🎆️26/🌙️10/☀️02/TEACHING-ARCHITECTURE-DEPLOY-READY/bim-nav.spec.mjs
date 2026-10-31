import{writeFileSync}from'node:fs';import{dirname,join}from'node:path';import{fileURLToPath}from'node:url';import{test}from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🚶️learner/🟦️.ts";

const root=dirname(fileURLToPath(import.meta.url));
test.use({device:async({device},use,info)=>{
 const opened=[],warnings=[];
 await use(async(...args)=>{
  const value=await device(...args);opened.push(value);value.page.on('console',message=>{if(message.text().startsWith('[DEBUG] BIM observer')&&warnings.length<16)warnings.push(message.text().slice(0,1024))});
  await value.context.addInitScript(()=>{
   const key="__semio_debug_bim_navigation_v1",rows=[],ids=new WeakMap();let serial=0,overflow=0;
   const identify=node=>{if(!node||typeof node!=="object")return null;if(!ids.has(node))ids.set(node,++serial);return ids.get(node)};
   const data=node=>node instanceof Element?{id:identify(node),tag:node.tagName,href:node.closest("a")?.getAttribute("href"),card:node.closest("[data-layered-card]")?.getAttribute("data-layered-card"),pane:node.closest("[data-layered-pane]")?.getAttribute("data-layered-pane"),connected:node.isConnected}:null;
   const state=()=>({hash:location.hash,activeGrid:document.querySelector("[data-course-grid][data-grid-active]")?.getAttribute("data-course-grid"),anchors:[...document.querySelectorAll('[data-layered-card="learner"] a')].slice(0,8).map(data),panes:[...document.querySelectorAll('[data-layered-pane="learner"]')].slice(0,8).map(n=>({...data(n),opened:n.hasAttribute("data-opened"),inert:n.hasAttribute("inert")}))});
   const save=row=>{if(rows.length<512)rows.push({at:performance.now(),wall:Date.now(),url:location.href,frame:window===top?"top":"child",...row});else overflow++};
   const history=()=>{try{return JSON.parse(sessionStorage.getItem(key)||"[]")}catch{return[]}};
   window.bimNavigationObservation=()=>({history:history(),current:{url:location.href,rows,overflow}});
   const relevant=event=>event.composedPath().some(n=>n instanceof Element&&n.closest('[data-layered-card="learner"]'));
   for(const target of[window,document])for(const capture of[true,false])for(const type of["pointerdown","pointerup","mousedown","mouseup","click"])target.addEventListener(type,event=>{
    if(!relevant(event))return;
    save({type,phase:(target===window?"window":"document")+(capture?":capture":":bubble"),target:data(event.target),path:event.composedPath().slice(0,8).map(data),trusted:event.isTrusted,prevented:event.defaultPrevented,button:event.button,pointer:event.pointerType,state:state()});
    queueMicrotask(()=>save({type,phase:"microtask",target:data(event.target),trusted:event.isTrusted,prevented:event.defaultPrevented,state:state()}));
   },{capture,passive:true});
   for(const type of["hashchange","popstate"])window.addEventListener(type,event=>save({type,trusted:event.isTrusted,state:state()}),{passive:true});
   new MutationObserver(changes=>{for(const change of changes){
    if(change.type==="attributes"){if(change.target.matches('[data-layered-pane="learner"],[data-layered-card="learner"] a'))save({type:"attribute",name:change.attributeName,value:change.target.getAttribute(change.attributeName),target:data(change.target),state:state()});}
    else for(const[kind,nodes]of[["removed",change.removedNodes],["added",change.addedNodes]])for(const node of nodes)if(node instanceof Element){const anchors=[...(node.matches('[data-layered-card="learner"] a')?[node]:[]),...node.querySelectorAll('[data-layered-card="learner"] a')].slice(0,8);for(const anchor of anchors)save({type:kind,target:data(anchor),state:state()});}
   }}).observe(document,{subtree:true,childList:true,attributes:true,attributeFilter:["href","inert","data-opened"]});
   window.addEventListener("pagehide",()=>{try{const prior=history();prior.push({url:location.href,rows,overflow});sessionStorage.setItem(key,JSON.stringify(prior.slice(-5)))}catch(error){console.warn("[DEBUG] BIM observer storage: "+String(error).slice(0,512))}},{passive:true});
  });
  return value;
 });
 const observations=[];
 for(const item of opened){try{observations.push(await item.page.evaluate(()=>window.bimNavigationObservation?.()??null))}catch(error){observations.push({unavailable:String(error)})}}
 try{writeFileSync(join(root,"🗑️generated/bim-navigation-observation.json"),JSON.stringify({version:1,at:new Date().toISOString(),warnings,observations},null,2),{flag:"wx"})}catch(error){const description=String(error).slice(0,1024);info.annotations.push({type:"debug-observer-failure",description});process.stderr.write("[DEBUG] BIM observer receipt: "+description+"\n")}
}});

await import("../../../../../../../🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎯️quiz-runs/🟦️.ts");
