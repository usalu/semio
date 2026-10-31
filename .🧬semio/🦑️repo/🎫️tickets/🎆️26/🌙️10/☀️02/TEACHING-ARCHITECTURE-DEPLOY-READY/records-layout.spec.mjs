import{writeFileSync}from"node:fs";import{dirname,join}from"node:path";import{fileURLToPath}from"node:url";import{test,expect,QUIZZES,enter,authorPreview,goHome,activeOverview,primary,card,pane,layoutQuiet}from"../../../../../../../🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🚶️learner/🟦️.ts";
const labels={en:{size:"Text size",largest:"Largest",normal:"Normal"},de:{size:"Textgröße",largest:"Maximal",normal:"Normal"}};
for(const initial of["en","de"])test("records-"+initial,async({device})=>{
 const learner=await device(initial),page=learner.page,seen=[];
 const board=()=>pane(page,"board"),table=()=>board().locator("table.course-fold"),frame=()=>table().locator("..");
 const openBoard=async(fromPane=true)=>{if(fromPane)await goHome(learner);else{await expect(activeOverview(page)).toBeVisible();await expect(page.locator("[data-layered-pane][data-opened]")).toHaveCount(0);}await primary(card(page,"board")).click();await expect(board()).toHaveAttribute("data-opened","");await expect(table()).toBeVisible();};
 const inspect=async(stage,locale)=>{
  await layoutQuiet(page);
  await expect.poll(()=>frame().evaluate(n=>n.scrollWidth-n.clientWidth)).toBeLessThanOrEqual(1);
  const sideways=await page.evaluate(()=>[document.documentElement,...document.querySelectorAll("#course-main, #course-main *")].filter(n=>n.closest("[inert]")===null).filter(n=>n===document.documentElement||["auto","scroll"].includes(getComputedStyle(n).overflowX)).filter(n=>n.scrollWidth>n.clientWidth+1).map(n=>({tag:n.tagName,classes:String(n.className).slice(0,512),overflow:n.scrollWidth-n.clientWidth})).slice(0,32));
  expect(sideways).toEqual([]);
  const observed=await frame().evaluate(n=>{const t=n.querySelector("table.course-fold");return{width:n.clientWidth,scroll:n.scrollWidth,folded:n.hasAttribute("data-folded"),minimum:getComputedStyle(t).getPropertyValue("--course-records-minimum-fold").trim(),display:getComputedStyle(t).display,font:getComputedStyle(t).fontFamily,rootFont:getComputedStyle(document.documentElement).fontSize,headers:[...t.querySelectorAll("thead button")].map(b=>b.textContent.trim()),columnRoles:[...t.querySelectorAll("thead th")].every(th=>th.getAttribute("role")==="columnheader"),rows:t.querySelectorAll("tbody tr").length}});
  expect(observed.headers).toHaveLength(QUIZZES.length+6);
  for(const quiz of QUIZZES)expect(observed.headers).toContain(quiz.title[locale]);
  expect(observed.columnRoles).toBe(true);expect(observed.rows).toBeGreaterThan(0);
  if(observed.minimum==="1")expect(observed.folded).toBe(true);
  expect(observed.display).toBe(observed.folded?"block":"table");
  for(const button of await table().locator("thead button").all()){await expect(button).toBeVisible();await expect(button).toBeEnabled();}
  seen.push({stage,locale,...observed});
 };
 try{
  await enter(learner,{kind:"anonymous"});await authorPreview(learner);await openBoard(false);
  let locale=initial;
  for(const width of[1920,375,1440,1920]){await page.setViewportSize({width,height:1000});await inspect("width-"+width,locale);}
  const header=table().getByRole("button",{name:QUIZZES[0].title[locale],exact:true});
  await header.click();await expect(header.locator("..")).toHaveAttribute("aria-sort","descending");
  await header.click();await expect(header.locator("..")).toHaveAttribute("aria-sort","ascending");await expect(header).toBeFocused();
  await frame().evaluate(n=>{window.recordsProbe={frame:n,table:n.querySelector("table"),focused:document.activeElement,font:n.style.fontFamily};n.style.fontFamily="monospace";});
  await inspect("custom-font",locale);
  expect(await frame().evaluate(n=>n===window.recordsProbe.frame&&n.querySelector("table")===window.recordsProbe.table&&document.activeElement===window.recordsProbe.focused)).toBe(true);
  expect(await table().evaluate(n=>getComputedStyle(n).fontFamily)).toContain("monospace");
  await frame().evaluate(n=>{n.style.fontFamily=window.recordsProbe.font;});await inspect("restored-font",locale);
  locale=initial==="en"?"de":"en";
  await page.getByRole("banner").getByRole("button",{name:locale==="de"?"Deutsch":"English",exact:true}).click();
  await expect(page.locator(".course-app")).toHaveAttribute("lang",locale);await inspect("locale-change",locale);
  await goHome(learner);await primary(card(page,"prefs")).click();
  await pane(page,"prefs").getByRole("group",{name:labels[locale].size,exact:true}).getByRole("button",{name:labels[locale].largest,exact:true}).click();
  await openBoard();await inspect("largest-text",locale);
  await page.setViewportSize({width:375,height:1000});await inspect("largest-narrow",locale);
  await page.setViewportSize({width:1920,height:1000});
  await goHome(learner);await primary(card(page,"prefs")).click();
  await pane(page,"prefs").getByRole("group",{name:labels[locale].size,exact:true}).getByRole("button",{name:labels[locale].normal,exact:true}).click();
  await openBoard();await inspect("restored-text",locale);
  await table().getByRole("button",{name:QUIZZES[0].title[locale],exact:true}).click();
  await expect(table().getByRole("button",{name:QUIZZES[0].title[locale],exact:true}).locator("..")).toHaveAttribute("aria-sort",/ascending|descending/);
 }finally{writeFileSync(join(dirname(fileURLToPath(import.meta.url)),"🗑️generated","records-browser-quiz-data-fixed-"+initial+".json"),JSON.stringify({initial,seen},null,2));}
});
