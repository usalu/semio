/** 🌐 Measure the actual WindowChrome implementation and CSS through Chromium. */
import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";

const ticket = path.dirname(fileURLToPath(import.meta.url));
const root = process.cwd();
const require = createRequire(path.join(root, "package.json"));
const { build } = require("esbuild");
const { compile } = require("@tailwindcss/node");
const { Scanner } = require("@tailwindcss/oxide");
const ui = path.join(root, "🧰️framework/🔨️modules/🖱️ui");
const output = path.join(ticket, "🗑️generated/exact-tests");
await fs.mkdir(output, { recursive: true });
const adapter=path.join(root,"🧰️framework/🔨️modules/🏃️process/🧰️tools/🎭️playwright/🟦️.ts");
const adapterPath=path.join(output,"oracle-browser-tools.cjs");
const adapterBundle=await build({entryPoints:[adapter],bundle:true,write:false,platform:"node",format:"cjs",packages:"external",logLevel:"silent"});
await fs.writeFile(adapterPath,adapterBundle.outputFiles[0].contents);
const {chromium}=await require(adapterPath).loadPlaywrightV1(root);
const cssPath = path.join(ui, "🧵️styles/🎨️.css");
const compiler = await compile(await fs.readFile(cssPath, "utf8"), { base: path.dirname(cssPath), onDependency() {} });
const scanner = new Scanner({ sources: [{ base: ui, pattern: "**/*.{ts,tsx}", negated: false }, ...compiler.sources] });
const css = compiler.build(scanner.scan());
const chrome = path.join(ui, "🧱️elements/🗂️WindowChrome/🟦️.tsx").replaceAll("\\", "/");
const silhouette = path.join(ui, "🧱️elements/🔲️WindowSilhouette/🟦️.tsx").replaceAll("\\", "/");
const navbar = path.join(ui, "🧱️elements/🔝️Navbar/🟦️.tsx").replaceAll("\\", "/");
const footer = path.join(ui, "🧱️elements/🔚️Footer/🟦️.tsx").replaceAll("\\", "/");
const tree = path.join(ui,"🧱️elements/🌳️Tree/🟦️.tsx").replaceAll("\\","/");
const popover = path.join(ui,"🧱️elements/🗨️Popover/🟦️.tsx").replaceAll("\\","/");
const iconContract=JSON.parse(await fs.readFile(path.join(root,"🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🔣️.json"),"utf8"));
const iconSVG=await fs.readFile(path.join(root,iconContract.icons["maximize-2"]),"utf8");
const entry = `import React from "react"; import {createRoot} from "react-dom/client";
import {WindowChrome, windowChromeTitleChipClass, measureWindowSilhouetteMetrics} from ${JSON.stringify(chrome)};
import * as silhouette from ${JSON.stringify(silhouette)};
import {Navbar,SemioLogo,navbarFillItem} from ${JSON.stringify(navbar)};
import {Footer} from ${JSON.stringify(footer)};
import {Tree} from ${JSON.stringify(tree)};
import {Popover,PopoverTrigger,PopoverContent} from ${JSON.stringify(popover)};
const icon=<span style={{display:"contents"}} dangerouslySetInnerHTML={{__html:${JSON.stringify(iconSVG.replace('<svg','<svg data-icon width="16" height="16"'))}}}/>;
function App(){return <>{[false,true].map((labeled,i)=><WindowChrome key={i} style={{position:"absolute",left:50,top:50+i*350,width:400,height:300}} stackDataAttrs={{"data-case":labeled?"labeled":"icon"}} titleChips={<button className={windowChromeTitleChipClass}><span>1.1 Luft</span></button>} {...(labeled?{enlarge:{id:"enlarge",slot:"fullscreen",icon,label:"Fullscreen",onClick(){}}}:{capRightChips:<button data-slot="fullscreen" className="flex size-medium items-center justify-center border-0 bg-transparent text-element">{icon}</button>})} body={<div/>}/>)}</>}
const reactRoot=createRoot(document.getElementById("root"));
function Popup(){return <Navbar label="oracle" showFullscreenToggle={false} items={[{content:<Popover open><PopoverTrigger asChild><button id="oracle-energy" className={windowChromeTitleChipClass}>Energie</button></PopoverTrigger><PopoverContent align="start" side="bottom"><Tree sections={[{id:"energy",items:[{id:"oracle:tree:energy",label:"Energie",defaultOpen:true,items:[{id:"oracle:tree:first",label:"Physikalische Grundlagen"},{id:"oracle:tree:second",label:"Lüftung"}]}]}]}/></PopoverContent></Popover>}]}/>}
function Pane({title,level="window"}){return <WindowChrome level={level} style={{width:"100%",height:"100%"}} titleChips={<button className={windowChromeTitleChipClass}>{title}</button>} capRightChips={<button data-slot="fullscreen" className="flex size-medium items-center justify-center border-0 bg-transparent text-element">{icon}</button>} body={<div/>}/>}
function Surfaces(){return <>{["base","window","pane","panel","dialog","menu"].map((level,index)=><div key={level} data-surface-case={level} style={{position:"absolute",left:10+index*260,top:20,width:240,height:180}}><Pane level={level} title={level}/></div>)}<WindowChrome level="window" style={{position:"absolute",left:20,top:250,width:400,height:300}} titleChips={<button className={windowChromeTitleChipClass}>parent</button>} body={<div data-surface-case="window-over-window" style={{position:"absolute",left:20,top:60,width:300,height:180}}><Pane title="nested"/></div>}/><div style={{position:"absolute",left:500,top:300,width:400,height:250,backgroundColor:"#06191f"}}><div data-surface-case="window-over-opaque" style={{position:"absolute",left:20,top:30,width:300,height:180}}><Pane title="opaque backdrop"/></div></div></>}
function Overview(){const item=value=>({content:<span className="text-xs leading-none">{value}</span>});return <div style={{height:900,display:"flex",flexDirection:"column"}}><Navbar label="semio" showFullscreenToggle={false} items={[{content:<SemioLogo className="size-medium"/>},item("semio"),navbarFillItem(),{...item("Energie"),centered:true},item("Architektur · Gebäudesysteme")]}/><main data-slot="overview-canvas" style={{flex:1,minHeight:0,display:"grid",gridTemplateRows:"repeat(2,minmax(0,1fr))",padding:3.2,gap:3.2}}>{[["Physikalische Grundlagen","Lüftung","Heizen"],["Kühlen","Energiebilanz"]].map((row,index)=><div key={index} style={{display:"grid",gridTemplateColumns:'repeat('+row.length+',minmax(0,1fr))',gap:3.2}}>{row.map(title=><Pane key={title} title={title}/>)}</div>)}</main><Footer items={[item("Alle Lektionen"),navbarFillItem(),item("5 Fenster")]}/></div>}
window.oracle={measureWindowSilhouetteMetrics,silhouette,renderOverview(){reactRoot.render(<Overview/>)},renderSurfaces(){reactRoot.render(<Surfaces/>)},renderPopup(){reactRoot.render(<Popup/>)},renderActive(){reactRoot.render(<WindowChrome active level="window" style={{position:"absolute",left:50,top:50,width:400,height:300}} titleChips={<button className={windowChromeTitleChipClass}>active</button>} body={<div/>}/>)} }; reactRoot.render(<App/>);`;
const bundle = await build({ stdin: { contents: entry, resolveDir: root, loader: "tsx" }, bundle: true, write: false, format: "iife", platform: "browser", alias: {"@semio-tech/framework-transient":path.join(root,"🧰️framework/🔨️modules/🫧️transient/🟦️.ts"),"@semio-tech/framework-interaction":path.join(root,"🧰️framework/🔨️modules/🕹️interaction/🟦️.ts")}, define: { "process.env.NODE_ENV": '"production"' }, logLevel: "silent" });
const browser = await chromium.launch({ headless: true });
try {
  const page = await browser.newPage({ viewport: { width: 1600, height: 900 }, deviceScaleFactor: 1 });
  await page.route("http://oracle.local/**", async route => {
    const pathname = decodeURIComponent(new URL(route.request().url()).pathname);
    if (pathname === "/") return route.fulfill({ contentType: "text/html", body: '<html class="dark" lang="de"><body style="background:var(--base);color:var(--foreground)"><div id="root"></div></body></html>' });
    const local = pathname.startsWith("/🖼️assets/") ? path.join(root,"🧰️framework/🔨️modules",pathname.slice(1)) : path.resolve(root, "." + pathname);
    if (!local.startsWith(root + path.sep)) return route.abort();
    try { return route.fulfill({ body: await fs.readFile(local) }); } catch { return route.abort(); }
  });
  const errors = [];
  page.on("pageerror", error => errors.push(error.message));
  await page.goto("http://oracle.local/");
  await page.addStyleTag({ content: css });
  await page.addScriptTag({ content: bundle.outputFiles[0].text });
  await page.waitForSelector('[data-case="labeled"] [data-window-silhouette-border] path');
  await page.evaluate(() => document.fonts.ready);
  const measured = await page.evaluate(() => {
    const rect = element => { const r = element.getBoundingClientRect(); return Object.fromEntries(["x", "y", "width", "height"].map(k => [k, r[k]])); };
    const describe = element => {
      const s = getComputedStyle(element);
      return {rect:rect(element),fontFamily:s.fontFamily,fontSize:s.fontSize,lineHeight:s.lineHeight,paddingLeft:s.paddingLeft,gap:s.gap,strokeWidth:s.strokeWidth,backgroundColor:s.backgroundColor,color:s.color,backdropFilter:s.backdropFilter};
    };
    const cases = [...document.querySelectorAll("[data-case]")].map(stack => ({
      name: stack.dataset.case, stack:describe(stack),
      cap:describe(stack.querySelector('[data-slot="window-chrome-cap"]')),
      title:describe(stack.querySelector('[data-slot="window-chrome-chip-cap"]')),
      titleButton:describe(stack.querySelector('[data-slot="window-chrome-chip-cap"] button')),
      control:describe(stack.querySelector('[data-slot="fullscreen"]')),
      body:describe(stack.querySelector('[data-slot="window-chrome-body"]')),
      metrics:window.oracle.measureWindowSilhouetteMetrics(stack),
      path:stack.querySelector('[data-window-silhouette-border] path').getAttribute("d"),
      border:describe(stack.querySelector('[data-window-silhouette-border] path')),
    }));
    const canvas = document.createElement("canvas"), context = canvas.getContext("2d");
    const text = ["1.1 Luft","1.1 Air","Fullscreen","Vollbild","Physik","Energie","gg","MM","Physikalische Grundlagen","Heizen","Lüftung","Ágy"].map(value => {
      context.font='500 11.2px Anta'; const m=context.measureText(value); return {value,size:11.2,weight:500,advance:m.width,left:m.actualBoundingBoxLeft,right:m.actualBoundingBoxRight,ascent:m.actualBoundingBoxAscent,descent:m.actualBoundingBoxDescent,fontAscent:m.fontBoundingBoxAscent,fontDescent:m.fontBoundingBoxDescent};
    });
    const silhouetteCases=[{width:400,height:300,advance:0},{width:400,height:300,advance:1000},{width:29.1,height:50,advance:0},{width:30,height:50,advance:0},{width:80,height:50,advance:1000}].map(input=>{
      const title=Math.min(input.advance+6.4,192,input.width-22.4);
      const metrics={width:input.width,height:input.height,top:{depth:22.4,chips:[{left:0,right:title},{left:input.width-22.4,right:input.width}]},bottom:{depth:0,chips:[]}};
      return {...input,outline:window.oracle.silhouette.windowSilhouetteOutline(metrics,0).map(p=>[p.x,p.y]),borderOutline:window.oracle.silhouette.windowSilhouetteOutline(metrics,1).map(p=>[p.x,p.y])};
    });
    return {cases,text,silhouetteCases,fontLoaded:document.fonts.check('11.2px Anta'),schema:"teaching-exact-browser-oracle/v1"};
  });
  measured.errors = errors;
  if (!measured.fontLoaded || errors.length) throw new Error("The actual UI oracle did not load its canonical font or raised a browser error");
  await page.screenshot({ path:path.join(output,"browser.png") });
  await page.evaluate(() => window.oracle.renderOverview());
  await page.waitForSelector('[data-slot="overview-canvas"] [data-window-silhouette-border] path');
  measured.overview=await page.evaluate(()=>Object.fromEntries(["navbar","footer","overview-canvas"].map(slot=>{const r=document.querySelector('[data-slot="'+slot+'"]').getBoundingClientRect();return [slot,Object.fromEntries(["x","y","width","height"].map(k=>[k,r[k]]))]})));
  measured.glass=await page.evaluate(()=>Object.fromEntries(["window-chrome-body-surface","window-chrome-chip-cap","window-chrome-controls"].map(slot=>{const s=getComputedStyle(document.querySelector('[data-slot="'+slot+'"]'));return[slot,{backgroundColor:s.backgroundColor,color:s.color,backdropFilter:s.backdropFilter,opacity:s.opacity}]})));
  await page.screenshot({path:path.join(output,"overview.png")});
  const sharp=require("sharp");
  measured.pixels=Object.fromEntries(await Promise.all([[100,100],[200,40],[100,40],[520,40],[100,10]].map(async([left,top])=>{const p=await sharp(path.join(output,"overview.png")).extract({left,top,width:1,height:1}).removeAlpha().raw().toBuffer();return[left+","+top,Array.from(p)]})));
  await page.evaluate(()=>window.oracle.renderSurfaces());
  await page.waitForSelector('[data-surface-case="menu"] [data-slot="window-chrome-body-surface"]');
  measured.surfaceCases=await page.evaluate(()=>[...document.querySelectorAll('[data-surface-case]')].map(host=>{const el=host.querySelector('[data-slot="window-chrome-body-surface"]'),s=getComputedStyle(el),r=el.getBoundingClientRect();return{level:host.dataset.surfaceCase,backgroundColor:s.backgroundColor,backdropFilter:s.backdropFilter,point:[Math.round(r.x+r.width/2),Math.round(r.y+r.height/2)]}}));
  await page.screenshot({path:path.join(output,"surfaces.png")});
  for(const sample of measured.surfaceCases){const [left,top]=sample.point;sample.pixel=Array.from(await sharp(path.join(output,"surfaces.png")).extract({left,top,width:1,height:1}).removeAlpha().raw().toBuffer())}
  await page.evaluate(()=>window.oracle.renderActive());
  await page.waitForSelector('[data-active="true"] [data-window-silhouette-border] path');
  await page.mouse.move(1000,800);
  await page.evaluate(()=>{for(const animation of document.getAnimations())animation.finish()});
  const inactiveHover=await sharp(await page.screenshot()).raw().toBuffer();
  await page.mouse.move(200,200);
  await page.evaluate(()=>{for(const animation of document.getAnimations())animation.finish()});
  const activeHover=await sharp(await page.screenshot()).raw().toBuffer();
  measured.activeWindowShellChangesOnBodyHover=!inactiveHover.equals(activeHover);
  await page.evaluate(()=>window.oracle.renderPopup());
  await page.waitForSelector('[id="oracle:tree:second"] [data-slot="tree-label"]');
  const popupState=()=>page.evaluate(()=>{
    const menu=document.querySelector('[data-slot="popover-content"]'),m=menu.getBoundingClientRect();
    return{background:getComputedStyle(menu).backgroundColor,rect:{x:m.x,y:m.y,width:m.width,height:m.height},insideNavbar:!!menu.closest('[data-slot="navbar"]'),rows:[...document.querySelectorAll('[data-slot="tree-item-row"]')].map(row=>{const label=row.querySelector('[data-slot="tree-label"]'),r=row.getBoundingClientRect();return{id:row.id,foreground:getComputedStyle(label).color,row_background:getComputedStyle(row).backgroundColor,label_background:getComputedStyle(label).backgroundColor,hover_path:row.getAttribute('data-tree-hover-path'),rect:{x:r.x,y:r.y,width:r.width,height:r.height}}})};
  });
  await page.mouse.move(1200,800);
  await page.evaluate(()=>{for(const animation of document.getAnimations())animation.finish()});
  measured.popup={normal:await popupState()};
  await page.screenshot({path:path.join(output,"popup-normal.png")});
  const leaf=measured.popup.normal.rows.find(row=>row.id==="oracle:tree:first").rect;
  await page.mouse.move(leaf.x+leaf.width/2,leaf.y+leaf.height/2);
  await page.evaluate(()=>{for(const animation of document.getAnimations())animation.finish()});
  measured.popup.hover=await popupState();
  await page.screenshot({path:path.join(output,"popup-hover.png")});
  await fs.writeFile(path.join(output, "browser.json"), JSON.stringify(measured, null, 2));
  console.log(JSON.stringify({fontLoaded:measured.fontLoaded,errors,overview:measured.overview,glass:measured.glass,pixels:measured.pixels,surfaceCases:measured.surfaceCases}));
} finally { await browser.close(); }
