
import{writeFileSync}from"node:fs";import{dirname,join}from"node:path";import{fileURLToPath}from"node:url";
import{test,expect,enter,handle,activeOverview,boxOf,restingPage,count}from"../../../../../../../🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🚶️learner/🟦️.ts";
test("records actual tiny-view native swipe coordinates",async({device},info)=>{
 const anna=await device("en"),ben=await device("de"),page=ben.page,record={version:1,sent:[]};
 await anna.page.setViewportSize({width:1440,height:100});await page.setViewportSize({width:375,height:200});
 try{
  await enter(anna,{kind:"pseudonym",handle:handle("Scroll Observer")},true);
  await enter(ben,{kind:"pseudonym",handle:handle("Scroll Player")},true);
  await expect.poll(async()=>count(await anna.page.locator("[data-presence-status]").innerText())).toBe(2);
  await expect(activeOverview(page)).toHaveAttribute("data-mode","swipe");
  const box=await boxOf(activeOverview(page)),dx=0,dy=-box.height*.4;
  const start={x:box.x+box.width/2-dx/2,y:box.y+box.height/2-dy/2},end={x:start.x+dx,y:start.y+dy};
  record.before=await page.evaluate(({start,end})=>{
   const root=document.querySelector("[data-course-grid][data-grid-active] > [data-layered-overview]"),port=root.closest("[data-course-scrollport]"),main=document.querySelector("#course-main"),app=document.querySelector(".course-app");
   const rect=n=>{const b=n.getBoundingClientRect();return{x:b.x,y:b.y,width:b.width,height:b.height}};
   const identify=n=>n instanceof Element?{tag:n.tagName,role:n.getAttribute("role"),tabindex:n.getAttribute("tabindex"),card:n.closest("[data-layered-card]")?.getAttribute("data-layered-card"),href:n.closest("a")?.getAttribute("href"),classes:String(n.className).slice(0,180)}:null;
   window.swipeNativeEvents=[];
   const scrollNodes=[];for(let node=document.elementFromPoint(start.x,start.y);node!==null;node=node.parentElement)if(node instanceof HTMLElement&&/auto|scroll/.test(getComputedStyle(node).overflowY))scrollNodes.push(node);window.swipeNativeScrollNodes=scrollNodes;
   const scrollState=node=>({cell:node.getAttribute("data-layered-cell"),classes:node.className,top:node.scrollTop,height:node.scrollHeight,client:node.clientHeight,max:node.scrollHeight-node.clientHeight});
   for(const type of["pointerdown","pointermove","pointerup","touchstart","touchmove","touchend"])window.addEventListener(type,e=>{if(window.swipeNativeEvents.length<128)window.swipeNativeEvents.push({type,x:e.clientX,y:e.clientY,trusted:e.isTrusted,prevented:e.defaultPrevented,target:identify(e.target)})},{capture:true,passive:true});
   const controls=[...port.querySelectorAll("button,input:not([type=hidden]),select,textarea,a[href],[role=button],[tabindex]:not([tabindex='-1'])")].filter(n=>n.closest("[inert],[hidden],[aria-hidden=true]")===null&&n.getClientRects().length).map(n=>({...identify(n),rect:rect(n),descendantControls:n.querySelectorAll("button,input,select,textarea,a[href],[role=button],[tabindex]").length})).sort((a,b)=>b.rect.height-a.rect.height).slice(0,12);
   const b=rect(root);return{viewport:{width:innerWidth,height:innerHeight},root:b,port:rect(port),main:rect(main),app:rect(app),scroll:app.scrollTop,minimum:port.style.minBlockSize,mainMinimum:main.style.minBlockSize,scrollables:scrollNodes.map(scrollState),visible:{left:Math.max(0,b.x),top:Math.max(0,b.y),right:Math.min(innerWidth,b.x+b.width),bottom:Math.min(innerHeight,b.y+b.height)},start,end,startHit:identify(document.elementFromPoint(start.x,start.y)),endHit:identify(document.elementFromPoint(end.x,end.y)),controls};
  },{start,end});
  const cdp=await page.context().newCDPSession(page);
  try{
   const messages=[{type:"touchStart",touchPoints:[start]},...Array.from({length:12},(_,i)=>({type:"touchMove",touchPoints:[{x:start.x+dx*(i+1)/12,y:start.y+dy*(i+1)/12}]})),{type:"touchEnd",touchPoints:[]}];
   for(const payload of messages){record.sent.push({at:Date.now(),payload});await cdp.send("Input.dispatchTouchEvent",payload);if(payload.type==="touchMove")await page.waitForTimeout(160/12)}
  }finally{await cdp.detach()}
  record.after=await page.evaluate(()=>{const n=document.querySelector("[data-course-grid][data-grid-active] > [data-layered-overview]"),b=n.getBoundingClientRect();return{root:{x:b.x,y:b.y,width:b.width,height:b.height},resting:n.getAttribute("data-layered-resting"),events:window.swipeNativeEvents,scrollables:window.swipeNativeScrollNodes.map(node=>({cell:node.getAttribute("data-layered-cell"),classes:node.className,top:node.scrollTop,height:node.scrollHeight,client:node.clientHeight,max:node.scrollHeight-node.clientHeight}))}});
  await expect.poll(()=>restingPage(page),{message:"the original learner-to-badges swipe lands on badges"}).toBe("badges");
 }finally{try{writeFileSync(join(dirname(fileURLToPath(import.meta.url)),"🗑️generated/vertical-swipe-scroll-range.json"),JSON.stringify(record,null,2),{flag:"wx"})}catch(error){info.annotations.push({type:"debug-observer-failure",description:String(error).slice(0,1024)})}}
});
