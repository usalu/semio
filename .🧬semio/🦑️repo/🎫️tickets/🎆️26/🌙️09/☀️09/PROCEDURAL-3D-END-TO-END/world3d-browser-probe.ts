import { createServer } from "vite";
import { chromium } from "playwright";
import { resolve } from "node:path";
import { copyFile, mkdir, writeFile } from "node:fs/promises";
import { PNG } from "pngjs";
import { BoxGeometry, Mesh, MeshBasicMaterial, PerspectiveCamera, Raycaster, Vector2, Vector3 } from "three";

const verifyComponent = process.argv.includes("--component");
const verifyLarge = process.argv.includes("--large");
const generated = resolve(import.meta.dirname, "🗑️generated", verifyComponent ? "world-browser-components" : verifyLarge ? "world-browser-large" : "world-browser");
const checkout = process.cwd();
await mkdir(generated, {recursive:true});
await Promise.all([
  copyFile(resolve(import.meta.dirname,"world3d-browser-index.html"), resolve(generated,"index.html")),
  copyFile(resolve(import.meta.dirname,"world3d-browser-mount.tsx"), resolve(generated,"mount.tsx")),
  verifyLarge ? copyFile(resolve(import.meta.dirname,"🗑️generated/preview-direction/large-hover.json"), resolve(generated,"large-hover.json")) : writeFile(resolve(generated,"large-hover.json"), JSON.stringify({instances:[],hoveredIds:[]})),
]);
const server = await createServer({
  resolve:{ alias:{
    "probe:world-host":resolve(checkout,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx"),
    "probe:window-scope":resolve(checkout,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🌉️scene-host-contract/🟦️.ts"),
    "probe:hover-fixture":resolve(checkout,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/hovered-instances.json"),
    "probe:component-fixture":resolve(import.meta.dirname,"component-pickability.json"),
    "probe:grouped-hover":resolve(checkout,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🎯️grouped-hover.json")
  } },
  configFile:false, root:generated, cacheDir:resolve(generated, "vite-cache"),
  server:{ host:"127.0.0.1", fs:{ allow:[checkout] } },
  optimizeDeps:{ entries:[resolve(generated,"index.html")], include:["react","react-dom","react-dom/client","react/jsx-runtime","react/jsx-dev-runtime","scheduler"] },
  esbuild:{ jsx:"automatic" },
});
const verifyDistinct = process.argv.includes("--distinct");
const verifyCoincident = verifyDistinct || process.argv.includes("--coincident");
let browser;
try {
  await server.listen();
  const address=server.resolvedUrls!.local[0];
  console.log("World3d probe serving",address);
  browser = await chromium.launch({ args:["--enable-unsafe-swiftshader","--use-angle=swiftshader"] });
  const page = await browser.newPage({ viewport:{ width:900,height:560 },deviceScaleFactor:1 });
  page.on("pageerror", error=>console.log("[DEBUG] browser error",error.message));
  page.on("console", message=> { if (message.type()==="error") console.log("[DEBUG] browser console",message.text()); });
  page.on("response", response=> { if(response.status()>=400) console.log("[DEBUG] response",response.status(),response.url().slice(-240)); });
  await page.goto(address + (verifyComponent ? "?component" : verifyLarge ? "?large" : verifyCoincident ? (verifyDistinct ? "?coincident&distinct" : "?coincident") : ""),{waitUntil:"domcontentloaded",timeout:60000});
  const host=page.locator('[data-surface-id="procedural3d-browser-witness"]');
  const canvas=host.locator("canvas");
  await canvas.waitFor({state:"visible",timeout:45000});
  await page.waitForFunction(() => {
    const canvas = document.querySelector<HTMLCanvasElement>('[data-surface-id="procedural3d-browser-witness"] canvas');
    return canvas && canvas.width >= 700 && canvas.height >= 400;
  });
  await page.evaluate(()=>new Promise<void>(done=>requestAnimationFrame(()=>requestAnimationFrame(()=>done()))));
  const facts=await canvas.evaluate(element=>({ width:element.width,height:element.height,webgl:!!element.getContext("webgl2") }));
  if(!facts.webgl || facts.width<700 || facts.height<400) throw Error(JSON.stringify(facts));
  console.log("World3d canvas",facts);
  const origin = Number(await host.getAttribute("data-world-geometry-revision"));
  if(!Number.isInteger(origin)||origin<1) throw Error("Expected mounted geometry revision");
  const fixture = await page.evaluate(()=>window.__fixture);
  const rectangle = (await canvas.boundingBox())!;
  const authoredCamera = JSON.parse((await host.getAttribute("data-viewport-camera-json"))!);
  const camera = new PerspectiveCamera(authoredCamera.fov, rectangle.width / rectangle.height, 0.01, 1000);
  camera.position.fromArray(authoredCamera.position);
  camera.up.fromArray(authoredCamera.up ?? [0,0,1]);
  camera.zoom = authoredCamera.zoom;
  camera.lookAt(new Vector3().fromArray(authoredCamera.target));
  camera.updateProjectionMatrix();
  camera.updateMatrixWorld(true);
  const centers = fixture.centers.map((x:number)=>{
    const point = new Vector3(x,0,0).project(camera);
    return [Math.round((point.x+1)*rectangle.width/2),Math.round((1-point.y)*rectangle.height/2)] as const;
  });
  if (verifyComponent) {
    const { highlightedInput, editable, retained } = fixture.component;
    const geometry = new BoxGeometry(1.3, 1.2, 1.2);
    const material = new MeshBasicMaterial();
    const objects = fixture.component.instances.map(record => {
      const object = new Mesh(geometry, material);
      object.name = record.id;
      object.position.fromArray(record.position);
      object.updateMatrixWorld();
      return object;
    });
    const ray = new Raycaster();
    ray.setFromCamera(new Vector2(0, 0), camera);
    const closest = [...new Set(ray.intersectObjects(objects).map(hit=>hit.object.name))];
    if (closest[0] !== retained || closest[1] !== editable) throw Error("Unexpected geometry depths " + JSON.stringify(closest));
    geometry.dispose();
    material.dispose();
    const point = (index: number) => [rectangle.x + centers[index][0], rectangle.y + centers[index][1]] as const;
    const before = PNG.sync.read(await canvas.screenshot());
    await page.evaluate(id => window.__project({ hoveredId:id, hoveredIds:[id], hoverSourceWindowId:"graph-window:1" }), highlightedInput);
    await page.waitForFunction(id => JSON.parse(document.querySelector("[data-guest-selection-json]")!.getAttribute("data-guest-selection-json")!).hoveredId === id, highlightedInput);
    await page.evaluate(()=>new Promise<void>(done=>requestAnimationFrame(()=>requestAnimationFrame(()=>done()))));
    const after = PNG.sync.read(await canvas.screenshot());
    let pixels = 0;
    for (let row=centers[0][1]-12; row<=centers[0][1]+12; row++) for (let col=centers[0][0]-12; col<=centers[0][0]+12; col++) {
      const at=(row*before.width+col)*4;
      if (before.data[at] !== after.data[at] || before.data[at+1] !== after.data[at+1] || before.data[at+2] !== after.data[at+2]) pixels++;
    }
    if (pixels < 250) throw Error("Graph hover failed to paint the render-only input: " + pixels);
    if(Number(await host.getAttribute("data-world-geometry-revision"))!==origin) throw Error("Graph hover recycled geometry");
    await page.mouse.click(...point(0));
    await page.mouse.move(...point(1));
    await page.mouse.click(...point(1));
    await page.waitForFunction(id => window.__actions.some(action=>action.action==="interactionSelect"&&JSON.parse(action.args.targets).some(target=>target.id.startsWith(id + ".face."))),editable);
    const picks = await page.evaluate(()=>window.__actions.filter(action=>["interactionHover","interactionSelect"].includes(action.action)).flatMap(action=>JSON.parse(action.args.targets)));
    if (picks.length === 0 || picks.some(target=>!target.id.startsWith(editable + ".face."))) throw Error("Read-only component publication " + JSON.stringify(picks));
    const [x,y] = point(1);
    const marquee = async (id: string, expectHits: boolean) => {
      await page.evaluate(id => { window.__activeObject(id); window.__actions.length = 0; },id);
      await page.waitForFunction(id => JSON.parse(document.querySelector("[data-guest-selection-json]")!.getAttribute("data-guest-selection-json")!).activeObjectId === id,id);
      await page.mouse.move(x-60,y-60);
      await page.mouse.down();
      await page.mouse.move(x+60,y+60,{steps:8});
      await page.mouse.up();
      if (expectHits) await page.waitForFunction(()=>window.__actions.some(action=>action.action==="interactionSelect"),undefined,{timeout:5000});
      await page.evaluate(()=>new Promise<void>(done=>requestAnimationFrame(()=>requestAnimationFrame(()=>done()))));
      return await page.evaluate(()=>JSON.parse(window.__actions.findLast(action=>action.action==="interactionSelect")?.args.targets ?? "[]"));
    };
    const skipped = await marquee(retained, false);
    const selected = await marquee(editable, true);
    if (skipped.length || !selected.length || selected.some(target=>!target.id.startsWith(editable + ".face."))) throw Error("Wrong marquee targets " + JSON.stringify({skipped,selected}));
    console.log("Component mode", {highlightPixels:pixels, rayTargets:picks, skipped, marqueeTargets:selected});
  } else if (verifyLarge) {
    const point = [rectangle.x + centers[0][0], rectangle.y + centers[0][1]] as const;
    await page.mouse.move(...point);
    await page.waitForFunction(() => window.__actions.some(action => action.action === "interactionHover" && JSON.parse(action.args.targets).length > 0));
    const action = await page.evaluate(() => window.__actions.findLast(action => action.action === "interactionHover" && JSON.parse(action.args.targets).length > 0));
    const targets = JSON.parse(action.args.targets);
    if (JSON.stringify(targets) !== JSON.stringify(fixture.largeTargets)) throw Error(JSON.stringify(targets));
    if (Buffer.byteLength(action.args.targets) >= 4096) throw Error("Hover reference overflowed the action");
    await page.evaluate(hoveredIds => window.__project({ hoveredId: hoveredIds[0], hoveredIds, hoverSourceWindowId:"independent-graph-window:1" }), fixture.ids);
    await page.waitForFunction(ids => JSON.stringify(JSON.parse(document.querySelector("[data-guest-selection-json]")!.getAttribute("data-guest-selection-json")!).hoveredIds) === JSON.stringify(ids), fixture.ids);
    if (Number(await host.getAttribute("data-world-geometry-revision")) !== origin) throw Error("Coincident hover replaced the geometry");
    await page.mouse.click(...point);
    await page.waitForFunction(() => window.__actions.some(action => action.action === "interactionSelect"));
    const selected = await page.evaluate(() => JSON.parse(window.__actions.findLast(action => action.action === "interactionSelect")!.args.targets));
    if (selected.length !== 1 || !fixture.largeInteractionIds.includes(selected[0].id)) throw Error(JSON.stringify(selected));
    await page.mouse.move(-10,-10);
    await page.waitForFunction(() => window.__actions.findLast(action => action.action === "interactionHover")?.args.targets === "[]");
    console.log("Oversized mounted hover", { instances: fixture.ids.length, bytes:Buffer.byteLength(action.args.targets), selected:selected.length, cleared:true });
  } else if (verifyCoincident) {
    const geometry = Number(await host.getAttribute("data-world-geometry-revision"));
    if ((fixture.meshIds[0] !== fixture.meshIds[1]) !== verifyDistinct) throw Error("Wrong coincident mesh identities");
    for (const id of fixture.ids) {
      await page.evaluate(hoveredId => window.__project({ hoveredId, hoveredIds:[hoveredId], hoverSourceWindowId:"graph-window:1" }), id);
      await page.waitForFunction(expected => JSON.parse(document.querySelector("[data-guest-selection-json]")!.getAttribute("data-guest-selection-json")!).hoveredId === expected, id);
    }
    await page.evaluate(() => window.__project({ hoveredId:null, hoveredIds:[], hoverSourceWindowId:"graph-window:1" }));
    const point = (index: number) => [rectangle.x + centers[index][0], rectangle.y + centers[index][1]] as const;
    await page.mouse.move(...point(0));
    await page.waitForFunction(() => window.__actions.some(action => action.action === "interactionHover" && JSON.parse(action.args.targets).length > 0));
    const action = await page.evaluate(() => window.__actions.findLast(action => action.action === "interactionHover" && JSON.parse(action.args.targets).length > 0));
    const targets = JSON.parse(action.args.targets);
    if (JSON.stringify(targets) !== JSON.stringify(fixture.groupedHover)) throw Error(JSON.stringify(targets));
    if (Number(await host.getAttribute("data-world-geometry-revision")) !== geometry) throw Error("Coincident hover recycled geometry");
    await page.mouse.click(...point(0));
    await page.waitForFunction(() => window.__actions.some(action => action.action === "interactionSelect"));
    const selection = await page.evaluate(() => window.__actions.findLast(action => action.action === "interactionSelect"));
    const selected = JSON.parse(selection.args.targets);
    if (selected.length !== 1 || !targets.some(target => target.id === selected[0].id)) throw Error(JSON.stringify(selected));
    await page.mouse.move(-10,-10);
    await page.waitForFunction(() => window.__actions.findLast(action => action.action === "interactionHover")?.args.targets === "[]");
    await page.mouse.move(...point(2));
    await page.waitForFunction(() => {
      const targets = window.__actions.findLast(action => action.action === "interactionHover")?.args.targets;
      return targets === JSON.stringify(window.__fixture.independentHover);
    });
    await page.mouse.move(...point(1));
    await page.waitForFunction(() => window.__actions.findLast(action => action.action === "interactionHover")?.args.targets === JSON.stringify(window.__fixture.groupedHover));
    const nextGroup = [targets[0], { ...targets[1], id: targets[1].id + "-next" }];
    await page.evaluate(group => window.__setGroupedHover(group), nextGroup);
    await page.waitForFunction(revision => Number(document.querySelector("[data-world-geometry-revision]")?.getAttribute("data-world-geometry-revision")) > revision, geometry);
    await page.mouse.move(point(0)[0] + 2, point(0)[1]);
    await page.waitForFunction(expected => window.__actions.findLast(action => action.action === "interactionHover")?.args.targets === JSON.stringify(expected), nextGroup);
    console.log("Coincident mounted hover", targets, "single selection", selected, "leave, independent mesh reuse, re-enter, revised alias set");



  } else {
  const before = PNG.sync.read(await canvas.screenshot());
  await page.evaluate(()=>window.__project(window.__fixture.projected));
  await page.waitForFunction(()=>JSON.parse(document.querySelector("[data-guest-selection-json]")!.getAttribute("data-guest-selection-json")!).hoveredIds?.length===3);
  const selected = PNG.sync.read(await canvas.screenshot());
  if(Number(await host.getAttribute("data-world-geometry-revision"))!==origin) throw Error("The group changed geometry revision");
  let changed = 0;
  for(let at=0;at<before.data.length;at+=4) if(before.data[at]!==selected.data[at]||before.data[at+1]!==selected.data[at+1]||before.data[at+2]!==selected.data[at+2]) changed++;
  if(changed<200) throw Error("Projected hover painted "+changed+" changed pixels");
  function changedNear(first:PNG, second:PNG, [x,y]:readonly [number,number]){
    let pixels = 0;
    for(let row=y-12;row<=y+12;row++) for(let col=x-12;col<=x+12;col++){
      const base=(row*first.width+col)*4;
      if(first.data[base]!==second.data[base] || first.data[base+1]!==second.data[base+1] || first.data[base+2]!==second.data[base+2]) pixels++;
    }
    return pixels;
  }
  const regions = centers.map(point=>changedNear(before,selected,point));
  console.log("Projected",fixture.projected.hoveredIds,"changed",changed,"pixels total; per region",regions,"geometry revision",origin);
  if(regions.slice(0,3).some(count=>count<250)||regions[3]>20) throw Error("Unexpected projected regions "+regions.join(", "));
  const last = centers[3];
  await page.mouse.move(rectangle.x+last[0],rectangle.y+last[1]);
  try {
    await page.waitForFunction(()=>window.__actions.some(action=>JSON.stringify(action).includes("unrelated@body")),undefined,{timeout:3500});
  } finally {
    console.log("World pointer publications",await page.evaluate(()=>window.__actions));
  }
  const ray = PNG.sync.read(await canvas.screenshot());
  const override = centers.map(point=>changedNear(before,ray,point));
  if(override.slice(0,3).some(count=>count>20)||override[3]<250) throw Error("Unexpected local ray regions "+override.join(", "));
  console.log("Local ray changed regions",override);
  const publishedSource = (source:string) => page.waitForFunction(
    id => JSON.parse(document.querySelector("[data-guest-selection-json]")!.getAttribute("data-guest-selection-json")!).hoverSourceWindowId===id, source,
  );
  await page.evaluate(()=>window.__project({...window.__fixture.projected,hoverSourceWindowId:"browser-window:1"}));
  await publishedSource("browser-window:1");
  const echo = PNG.sync.read(await canvas.screenshot());
  const echoRegions = centers.map(point=>changedNear(ray,echo,point));
  if(echoRegions.some(count=>count>20)) throw Error("The originating window's delayed reply changed local paint "+echoRegions.join(", "));
  await page.evaluate(()=>window.__project({...window.__fixture.projected,hoverSourceWindowId:"graph-window:1"}));
  await publishedSource("graph-window:1");
  const handoff = PNG.sync.read(await canvas.screenshot());
  const handoffRegions = centers.map(point=>changedNear(before,handoff,point));
  if(handoffRegions.slice(0,3).some(count=>count<250)||handoffRegions[3]>20) throw Error("The other window did not take the paint "+handoffRegions.join(", "));
  console.log("Owner-only handoff while the pointer remained stationary",handoffRegions);
  await page.evaluate(()=>window.__project({hoveredId:null,hoveredIds:[]}));
  await page.mouse.move(-10,-10);
  await page.waitForFunction(()=>window.__actions.some(action=>action.action==="interactionHover"&&action.args.targets==="[]"));
  await page.waitForFunction(()=>JSON.parse(document.querySelector("[data-guest-selection-json]")!.getAttribute("data-guest-selection-json")!).hoveredIds?.length===0);
  const cleared = PNG.sync.read(await canvas.screenshot());
  if(centers.some(point=>changedNear(before,cleared,point)>20)) throw Error("Pointer leave retained hover paint");
  if(Number(await host.getAttribute("data-world-geometry-revision"))!==origin) throw Error("The ray changed geometry revision");
  console.log("Pointer leave cleared all four regions and kept geometry revision",origin);
  }
} finally {
  await browser?.close();
  await server.close();
}
