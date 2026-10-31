import{writeFileSync}from"node:fs";import{join}from"node:path";import{test,expect,enter,handle,authorPreview,quizOf,playQuiz,answerRun,submitRun,expectFeedback,expectSaved,card,pane}from"/home/deck/semio/🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🚶️learner/🟦️.ts";
test("observe the saved physics learner link after reload",async({device})=>{
 const learner=await device("en"),page=learner.page;
 await learner.context.addInitScript(()=>{
  const rows=[],ids=new WeakMap();let serial=0,overflow=0;
  const id=n=>{if(!n||typeof n!=="object")return null;if(!ids.has(n))ids.set(n,++serial);return ids.get(n)};
  const data=n=>n instanceof Element?{id:id(n),tag:n.tagName,card:n.closest("[data-layered-card]")?.getAttribute("data-layered-card"),pane:n.closest("[data-layered-pane]")?.getAttribute("data-layered-pane"),href:n.closest("a")?.getAttribute("href"),connected:n.isConnected}:null;
  const save=x=>{if(rows.length<256)rows.push({at:performance.now(),url:location.href,...x});else overflow++};
  window.physicsNavigationObservation=()=>({rows,overflow});
  const observed=e=>e.composedPath().some(n=>n instanceof Element&&n.closest('[data-layered-card="learner"]'));
  for(const target of [window,document])for(const capture of [true,false])for(const type of ["pointerdown","pointerup","click"])target.addEventListener(type,e=>{
   if(!observed(e))return;save({type,stage:(target===window?"window":"document")+(capture?":capture":":bubble"),target:data(e.target),path:e.composedPath().slice(0,8).map(data),prevented:e.defaultPrevented,button:e.button,pointer:e.pointerType});
   queueMicrotask(()=>save({type,stage:"final-microtask",target:data(e.target),prevented:e.defaultPrevented}));
  },{capture,passive:true});
  for(const type of ["hashchange","popstate"])window.addEventListener(type,()=>save({type}));
  new MutationObserver(changes=>{for(const m of changes){if(m.type==="attributes"){if(m.target.matches('[data-layered-pane="learner"],[data-layered-card="learner"] a'))save({type:"attribute",target:data(m.target),name:m.attributeName,value:m.target.getAttribute(m.attributeName)});}else for(const [kind,nodes]of[["removed",m.removedNodes],["added",m.addedNodes]])for(const n of nodes)if(n instanceof Element){const a=n.matches('[data-layered-card="learner"] a')?n:n.querySelector('[data-layered-card="learner"] a');if(a)save({type:kind,target:data(a)});}}}).observe(document,{subtree:true,childList:true,attributes:true,attributeFilter:["href","inert","data-opened"]});
 });
 try{
  await enter(learner,{kind:"pseudonym",handle:handle("Navigation Probe")});await authorPreview(learner);
  const quiz=quizOf("physics");await playQuiz(learner,quiz.id,"easy");await answerRun(learner,quiz,"perfect");await submitRun(learner);expect((await expectFeedback(learner,quiz,"perfect","easy")).score).toBe(100);await expectSaved(learner);
  await page.reload();await card(page,"learner").getByRole("link").click({noWaitAfter:true});await expect(pane(page,"learner")).toHaveAttribute("data-opened","");
 }finally{writeFileSync("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/🗑️generated/physics-navigation-observation.json",JSON.stringify(await page.evaluate(()=>window.physicsNavigationObservation?.()),null,2));}
});
