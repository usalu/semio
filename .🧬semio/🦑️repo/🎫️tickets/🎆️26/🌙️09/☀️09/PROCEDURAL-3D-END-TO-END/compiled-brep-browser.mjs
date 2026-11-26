import { chromium } from "playwright";
import { resolve } from "node:path";
import { oracleAtHeight, oracleForLabel } from "./🐍️example-oracle.mjs";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { EXAMPLES_DIR, readExampleFixture } from "./🐍️example-fixtures.mjs";
const scenes = process.env.GEOMETRY_SCENES?.split(",");
const scenarios = scenes?.map(slug => {
  const entry = readdirSync(EXAMPLES_DIR).find(name => readExampleFixture(name)?.example === slug);
  if (!entry) throw Error("Missing authored geometry fixture " + slug);
  const assets = join(EXAMPLES_DIR, entry, "🖼️assets");
  const dsl = readFileSync(join(assets, readdirSync(assets,{withFileTypes:true}).find(child=>child.isDirectory()).name, "🗣️.dsl.semio"), "utf8");
  const sliders = Object.fromEntries([...dsl.matchAll(/input-slider id="([^"]+)"[^\n]*? value=([^ ]+)/g)].map(m => [m[1], Number(m[2])]));
  const neurons = [...dsl.matchAll(/neuron id="([^"]+)" neuron-kind=([^ ]+)/g)].map(m => ({id:m[1],kind:m[2]}));
  const wires = [...dsl.matchAll(/^  [^ ]+ ([^ @]+)@([^ >]+)->([^ @\n]+)@([^ \n]+)$/gm)].map(m => m.slice(1,5));
  return {fixture:readExampleFixture(entry), sliders, neurons, wires};
});
const root = resolve(".");
const loader = "/@fs/" + root + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧩️module-loader/🟦️.ts";
const packs = "/@fs/" + root + "/🧰️framework/🔨️modules/🎁️packs/🟦️.ts";
const url = process.env.GEOMETRY_PREVIEW_URL ?? "http://127.0.0.1:6018";
const mushroom = process.env.GEOMETRY_BREP_SCENE === "mushroom"
  ? [6, 8].map((height) => {
    const expected = oracleAtHeight(oracleForLabel("Hexagonal Mushroom Column"), height);
    return { height, low: expected.boundingBoxMin, high: expected.boundingBoxMax, tolerance: expected.boundingBoxTolerance, minTriangles: expected.minTriangles, volume: expected.volume * height / 6 };
  })
  : null;
const compilerCount = Number(process.env.GEOMETRY_CHROMIUM_COMPILERS ?? 1);
if (!Number.isInteger(compilerCount) || compilerCount < 1 || compilerCount > 4) throw Error("Expected between one and four Chromium WASM compilation tasks");
const browser = await chromium.launch({headless:true,args:[`--js-flags=--wasm-num-compilation-tasks=${compilerCount}`, "--renderer-process-limit=1"]});
const timeout = setTimeout(() => { console.error("[DEBUG] BRep capability deadline"); void browser.close(); }, Number(process.env.GEOMETRY_PROBE_MS ?? 600000));
try {
  const page = await browser.newPage();
  page.context().on("requestfailed", r => console.error("[DEBUG] request failed",r.url(),r.failure()?.errorText));
  page.context().on("response", response => {
    const asset = new URL(response.url()).pathname;
    if (response.status() >= 400 || asset.endsWith("semio_s_plugin_flow_brep_component.core.wasm")) {
      console.log("[DEBUG] HTTP", response.status(), response.url(), response.headers()["content-length"]);
    }
  });
  page.on("pageerror", error => console.error("[DEBUG]", error.message));
  page.on("console", text => { if (text.text().startsWith("[DEBUG]")) console.log(text.text()); });
  await page.route(url + "/geometry-capability", route => route.fulfill({contentType:"text/html",body:"<!doctype html><title>BRep capability verification</title>"}));
  if (process.env.GEOMETRY_DIAGNOSTIC_TURNS) await page.context().route(url => decodeURIComponent(url.pathname).includes("📥️inbound-request/🟦️.ts"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    if (!source.includes("INBOUND_REQUEST_TURN_BUDGET = 64")) throw Error("Missing original browser turn policy");
    const traced = source.replace("const result = answerFor(turn);", "if (drive.capability === 'tessellate' && turns % 32 === 0) {const raw=turn.original??turn;console.log('[DEBUG] Original actor result',turns,Object.keys(raw).join(','),JSON.stringify(raw.retainedReceipt??raw.fuelUsed,(_,x)=>typeof x==='bigint'?x.toString():x));} const result = answerFor(turn);");
    await route.fulfill({response,body:traced.replace("INBOUND_REQUEST_TURN_BUDGET = 64","INBOUND_REQUEST_TURN_BUDGET = "+Number(process.env.GEOMETRY_DIAGNOSTIC_TURNS))});
  });
  if (process.env.GEOMETRY_PROFILE_CORE) await page.context().route("**/semio_s_plugin_flow_brep_component.core.wasm", route => route.continue({url:new URL("/@fs/"+resolve(process.env.GEOMETRY_PROFILE_CORE),url).href}));
  if (process.env.GEOMETRY_PROBE_GRANTS) await page.context().route(/extension-modules.*bridge\.js(?:\?.*)?$/, async route => {
    const response=await route.fetch();
    let source=await response.text();
    const trace = `const seenProbeGrants=new Set();function recordProbeGrant(b){const key=JSON.stringify({fields:Object.keys(b),fuel:b.fuel,grant:b.grant,inputGrant:b.input?.grant,policy:b.retainedPolicy},(_,x)=>typeof x==="bigint"?x.toString():x);if(seenProbeGrants.size<8&&!seenProbeGrants.has(key)){seenProbeGrants.add(key);console.log("[DEBUG] actual worker grant",key);}return b;}\n`;
    source=trace+source.replaceAll("reactor.poll(slice)", "reactor.poll(recordProbeGrant(slice))")
      .replaceAll("reactor.poll(originalContextBudgetWit(turn))","reactor.poll(recordProbeGrant(originalContextBudgetWit(turn)))");
    await route.fulfill({response,body:source});
  });
  await page.goto(url + "/geometry-capability");
  if (process.env.GEOMETRY_PROFILE === "1") {
    const protocol = await page.context().newCDPSession(page);
    const starts = new Set();
    const phase = process.env.GEOMETRY_PROFILE_PHASE;
    let reached = !phase;
    page.on("console", event => {
      if (!reached && event.text().startsWith("[DEBUG]") && event.text().split(" ").includes(phase)) {
        reached = true;
        for (const start of starts) void start();
      }
    });
    protocol.on("Target.attachedToTarget", async ({sessionId,targetInfo}) => {
      if (targetInfo.type !== "worker") return;
      const send = (id,method) => protocol.send("Target.sendMessageToTarget",{sessionId,message:JSON.stringify({id,method})});
      try {
        await send(1,"Profiler.enable");
        const start = async () => {
          starts.delete(start);
          try {
            await send(2,"Profiler.start");
            setTimeout(() => void send(3,"Profiler.stop").catch(()=>{}),30000).unref();
          } catch (error) { console.error("[DEBUG] Worker profile",String(error)); }
        };
        if (reached) await start();
        else starts.add(start);
      } catch (error) { console.error("[DEBUG] Worker profile start",String(error)); }
    });
    protocol.on("Target.receivedMessageFromTarget", ({message}) => {
      const profile = JSON.parse(message).result?.profile;
      if (!profile) return;
      console.log("[DEBUG] Worker CPU samples", JSON.stringify(profile.nodes.filter(n=>n.hitCount>0).sort((a,b)=>b.hitCount-a.hitCount).slice(0,20).map(n=>[n.hitCount,n.callFrame.functionName,n.callFrame.url.slice(-90)])));
    });
    await protocol.send("Target.setAutoAttach", {autoAttach:true,waitForDebuggerOnStart:false,flatten:false});
  }
  page.on("crash",()=>console.error("[DEBUG] Browser page crashed"));
  page.on("framenavigated",frame=>console.log("[DEBUG] Browser frame navigated",frame.url()));
  const result = await page.evaluate(({loader,packs,capabilityGroup,mushroom,scenarios,verifySolid}) => window.geometryProbe = (async () => {
    window.$RefreshReg$ = () => {};
    window.$RefreshSig$ = () => value => value;
    window.__vite_plugin_react_preamble_installed__ = true;
    const [{loadPluginModule},{decodePackValue,decodeMeshPackChunks}] = await Promise.all([import(loader),import(packs)]);
    console.log("[DEBUG] Runtime ready");
    const loadExtension = (id) => {
      const directory = id === "math" ? "🧮️flow-extension-math" : "🧊️flow-extension-brep";
      return loadPluginModule("flow-extension-"+id,"/🧩️extension-modules/"+directory+"/🌉️bridge.js");
    };
    const ext = await loadExtension(capabilityGroup);
    const number = value => ({$schema:"number",value});
    const point = (x,y,z) => ({$schema:"point",x,y,z});
    const numeric = value => Number(typeof value === "object" ? value.value : value);
    async function evaluate(extension, extensionId, operatorId, nodeHash, input) {
      const started = performance.now();
      for (let round=0; ; round++) {
        const body = {neuronId:operatorId,operatorId,nodeHash,operatorVersion:"fixture",inputJson:round ? "" : JSON.stringify(input),dependencyJson:"",roundUnits:256,resume:round>0,windowId:"preview",windowKindId:"preview",extensionId};
        let turns = 1;
        const before = performance.now();
        const bytes = await extension.invoke("evaluate",JSON.stringify(body), {onProgress: progress => { turns = progress.turns + 1; }});
        const reply = decodePackValue(bytes);
        console.log("[DEBUG]",operatorId,round,reply.phase,JSON.stringify(reply.unitsDone,(_,v)=>typeof v==="bigint"?v.toString():v),JSON.stringify(reply.unitsTotal,(_,v)=>typeof v==="bigint"?v.toString():v),Boolean(reply.done),Math.round(performance.now()-started),bytes.length,"actor turns",turns,"request ms",Math.round(performance.now()-before));
        if (reply.done) return reply.output;
      }
    }
    const evaluateInGroup = (operatorId, nodeHash, input) => evaluate(ext, capabilityGroup, operatorId, nodeHash, input);
    try {
      if (capabilityGroup === "math") {
        const output = await evaluateInGroup("math.add",773,{a:number(2),b:number(3)});
        const sum = output.sum?.value;
        if (Number(typeof sum === "object" ? sum?.value : sum) !== 5) throw Error("Expected 2 + 3 = 5");
        const constructed = await evaluateInGroup("math.vector",774,{z:number(6)});
        const vector = constructed.vectorOut;
        const coordinates = ["x","y","z"].map(key => Number(vector?.[key]?.value ?? vector?.[key]));
        if (vector?.$schema !== "vector" || coordinates.join(",") !== "0,0,6") throw Error("Unexpected constructed vector "+coordinates);
        return {sum:5,coordinates};
      }
      if (scenarios) {
        const needsMath = scenarios.some(({neurons}) => neurons.some(node => node.kind.startsWith("math.")));
        const math = needsMath ? await loadExtension("math") : null;
        const defaults = new Map();
        const directories = needsMath ? ["🧮️flow-extension-math","🧊️flow-extension-brep"] : ["🧊️flow-extension-brep"];
        for (const directory of directories) {
          const response = await fetch("/🧩️extension-modules/"+directory+"/🔣️.json");
          if (!response.ok) throw Error("Missing installed extension declaration "+directory);
          const descriptor = await response.json();
          for (const topic of descriptor.manifest.topicContributions) {
            if (!topic.payload?.manifestJson) continue;
            for (const operator of JSON.parse(topic.payload.manifestJson).contributes.operators ?? []) {
              defaults.set(operator.id,Object.fromEntries(operator.inputs.filter(port=>port.default!=null).map(port=>[port.name,port.default])));
            }
          }
        }
        let hash = 89001;
        const results = [];
        const normalize = value => {
          if (typeof value === "bigint") return Number(value);
          if (!value || typeof value !== "object") return value;
          if (Array.isArray(value)) return value.map(normalize);
          if (Object.keys(value).length <= 2 && "value" in value && !("$schema" in value) && typeof value.value !== "object") return normalize(value.value);
          return Object.fromEntries(Object.entries(value).map(([key,item]) => [key,normalize(item)]));
        };
        try {
          for (const {fixture,sliders,neurons,wires} of scenarios) {
            const outputs = new Map(Object.entries(sliders).map(([id,value])=>[id,{number:number(value)}]));
            const required = new Set();
            const select = id => {
              if (!id || required.has(id)) return;
              required.add(id);
              for (const [source,,target] of wires) if (target === id) select(source);
            };
            select(fixture.preview.node);
            select(fixture.expect.kernelVolumeNode);
            const pending = neurons.filter(node => required.has(node.id));
            while (pending.length) {
              const index = pending.findIndex(node => wires.filter(edge=>edge[2]===node.id).every(edge=>outputs.has(edge[0])));
              if (index < 0) throw Error("Cannot order authored graph "+fixture.example);
              const [{id,kind}] = pending.splice(index,1);
              const input = {...defaults.get(kind)};
              for (const [source,port,,target] of wires.filter(edge=>edge[2]===id)) {
                const value=outputs.get(source)[port];
                if (value === undefined) throw Error("Missing authored channel "+source+"@"+port);
                input[target]=value;
              }
              const group = kind.split(".")[0];
              console.log("[DEBUG] Authored operator",fixture.example,id,kind);
              outputs.set(id,normalize(await evaluate(group==="math"?math:ext,group,kind,++hash,input)));
            }
            const preview=outputs.get(fixture.preview.node)?.[fixture.preview.channel];
            let mesh;
            if (typeof preview?.preview === "string" && preview.preview) {
              mesh=decodeMeshPackChunks([preview.preview]);
            } else {
              if (!preview?.handle) throw Error("Missing authored preview handle or inline mesh "+fixture.example);
              const request=JSON.stringify({handle:preview.handle,nodeHash:++hash,tolerance:fixture.tessellationTolerance,windowId:"preview",windowKindId:"preview"});
              for(let round=0;round<1024 && !mesh;round++){
                const reply=decodePackValue(await ext.invoke("tessellate",request,{onProgress:({turns})=>{if(turns%32===0)console.log("[DEBUG] Authored tessellation turns",fixture.example,round,turns)}}));
                if(reply.done) mesh=reply.mesh;
              }
            }
            if(!mesh) throw Error("Missing tessellation "+fixture.example);
            const positions=Array.from(mesh.positions??[],numeric),indices=Array.from(mesh.indices??[],numeric),edges=Array.from(mesh.edgePositions??[],numeric);
            const coordinates=positions.length?positions:edges;
            const low=[Infinity,Infinity,Infinity],high=[-Infinity,-Infinity,-Infinity];
            coordinates.forEach((value,index)=>{const axis=index%3;low[axis]=Math.min(low[axis],value);high[axis]=Math.max(high[axis],value)});
            const triangles=indices.length/3,segments=edges.length/6,expected=fixture.expect;
            if (!coordinates.length || !coordinates.every(Number.isFinite) || indices.some(i=>!Number.isInteger(i)||i<0||i>=positions.length/3) || triangles<fixture.delivery.minTriangles || segments<fixture.delivery.minEdgeSegments) throw Error("Unexpected authored primitive counts "+JSON.stringify({example:fixture.example,triangles,segments}));
            if(low.some((v,i)=>Math.abs(v-expected.boundingBoxMin[i])>expected.boundingBoxTolerance)||high.some((v,i)=>Math.abs(v-expected.boundingBoxMax[i])>expected.boundingBoxTolerance))throw Error("Unexpected authored bounds "+JSON.stringify({example:fixture.example,low,high}));
            let volume = expected.kernelVolumeNode
              ? numeric(outputs.get(expected.kernelVolumeNode)[expected.kernelVolumeChannel].value)
              : null;
            if (volume === null && verifySolid && expected.closed) {
              if (!preview?.handle) throw Error("Closed solid lacks a live kernel handle");
              const result = normalize(await evaluateInGroup("brep.measure.volume", ++hash, {geometry:preview}));
              volume = numeric(result.volume.value);
            }
            if (volume !== null && (!Number.isFinite(volume) || Math.abs(volume-expected.volume) > (expected.kernelVolumeTolerance ?? 1e-9))) {
              throw Error("Unexpected authored kernel volume "+volume);
            }
            let meshVolume = null;
            let weldedEdges = null;
            if (expected.closed && (verifySolid || typeof preview.preview === "string" && preview.preview)) {
              let sixVolume = 0;
              const edgesByKey = verifySolid ? new Map() : null;
              const welded = point => point.map(value => Math.round(value * 100000)).join(",");
              for (let i = 0; i < indices.length; i += 3) {
                const [a,b,c] = indices.slice(i,i+3).map(index => positions.slice(3*index,3*index+3));
                sixVolume += a[0]*(b[1]*c[2]-b[2]*c[1]) + a[1]*(b[2]*c[0]-b[0]*c[2]) + a[2]*(b[0]*c[1]-b[1]*c[0]);
                if (edgesByKey) {
                  const [p,q,r] = [a,b,c].map(welded);
                  for (const [start,end] of [[p,q],[q,r],[r,p]]) {
                    if (start === end) throw Error("Collapsed authored mesh edge");
                    const [low,high] = start < end ? [start,end] : [end,start];
                    const key = low + "|" + high;
                    const tally = edgesByKey.get(key) ?? [0,0];
                    tally[0]++;
                    tally[1] += start < end ? 1 : -1;
                    edgesByKey.set(key,tally);
                  }
                }
              }
              meshVolume = (verifySolid ? sixVolume : Math.abs(sixVolume)) / 6;
              if (Math.abs(meshVolume-expected.volume)>expected.volumeTolerance) throw Error("Unexpected authored mesh volume "+meshVolume);
              if (edgesByKey) {
                for (const [count,orientation] of edgesByKey.values()) {
                  if (count !== 2 || orientation !== 0) throw Error("Open or inconsistently oriented authored mesh");
                }
                weldedEdges = edgesByKey.size;
              }
            }
            results.push({example:fixture.example,triangles,segments,low,high,volume,meshVolume,weldedEdges});
            console.log("[DEBUG] Authored installed result",JSON.stringify(results.at(-1)));
          }
        } finally { await math?.dispose?.(); }
        return results;
      }
      const profile = mushroom
        ? (await evaluateInGroup("brep.curve.polygon",771,{radius:number(0.5),sides:number(6)})).wire
        : (await evaluateInGroup("brep.curve.circle",771,{center:point(0,0,0),normal:point(0,0,1),radius:number(2)})).curve;
      if (!profile?.handle) throw Error("Missing curve handle "+JSON.stringify(profile));
      const checkExtrusion = async (nodeHash, vector, expected) => {
        const extrusion = await evaluateInGroup("brep.solid.extrude",nodeHash,{wire:profile,vector});
        if (!extrusion.solid?.handle) throw Error("Missing solid handle "+JSON.stringify(extrusion));
        const request = {handle:extrusion.solid.handle,tolerance:0.05,nodeHash,windowId:"preview",windowKindId:"preview"};
        for (let round=0;round<256;round++) {
          const reply = decodePackValue(await ext.invoke("tessellate",JSON.stringify(request)));
          console.log("[DEBUG] tessellate",round,reply.phase,reply.done);
          if (!reply.done) continue;
          const positions = Array.from(reply.mesh.positions,numeric);
          const indices = Array.from(reply.mesh.indices,numeric);
          const vertices = positions.length/3;
          if (vertices < 6 || !Number.isInteger(vertices) || !positions.every(Number.isFinite) ||
              indices.length < (expected?.minTriangles ?? 2) * 3 || indices.length%3 !== 0 ||
              !indices.every(i => Number.isInteger(i) && i>=0 && i<vertices)) throw Error("Invalid triangle mesh");
          const low=[Infinity,Infinity,Infinity], high=[-Infinity,-Infinity,-Infinity];
          positions.forEach((v,i)=>{const a=i%3;low[a]=Math.min(low[a],v);high[a]=Math.max(high[a],v);});
          const expectedLow = expected?.low ?? [-2, -2, 0], expectedHigh = expected?.high ?? [2, 2, 3];
          const tolerances = expected ? Array(3).fill(expected.tolerance) : [0.1, 0.1, 0.02];
          if (low.some((v,axis) => Math.abs(v-expectedLow[axis]) > tolerances[axis]) ||
              high.some((v,axis) => Math.abs(v-expectedHigh[axis]) > tolerances[axis])) {
            throw Error("Unexpected extrusion bounds " + JSON.stringify({low,high}));
          }
          let sixVolume = 0;
          for (let i=0;i<indices.length;i+=3) {
            const a=3*indices[i], b=3*indices[i+1], c=3*indices[i+2];
            sixVolume += positions[a]*(positions[b+1]*positions[c+2]-positions[b+2]*positions[c+1])
              + positions[a+1]*(positions[b+2]*positions[c]-positions[b]*positions[c+2])
              + positions[a+2]*(positions[b]*positions[c+1]-positions[b+1]*positions[c]);
          }
          const volume=Math.abs(sixVolume)/6;
          const expectedVolume = expected?.volume ?? 12*Math.PI;
          if (Math.abs(volume-expectedVolume) > (expected ? 0.0001 : 0.15*expectedVolume)) throw Error("Unexpected extrusion volume "+volume);
          return {curve:profile.handle,solid:extrusion.solid.handle,vertices,triangles:indices.length/3,low,high,volume};
        }
        throw Error("Tessellation did not complete");
      };
      if (!mushroom) return await checkExtrusion(772, {$schema:"vector",x:0,y:0,z:3}, null);
      const math = await loadExtension("math");
      const geometries = [];
      try {
        for (const [index, expected] of mushroom.entries()) {
          const { vectorOut } = await evaluate(math, "math", "math.vector", 780 + index, { z: number(expected.height) });
          const [x, y, z] = ["x", "y", "z"].map((key) => numeric(vectorOut[key]));
          if (vectorOut.$schema !== "vector" || x !== 0 || y !== 0 || z !== expected.height) throw Error("Unexpected authored axis " + JSON.stringify([x, y, z]));
          const geometry = await checkExtrusion(772 + index, { $schema: "vector", x, y, z }, expected);
          geometries.push({ height: expected.height, ...geometry });
          console.log("[DEBUG] Actual mushroom mesh", JSON.stringify(geometries.at(-1)));
        }
      } finally { await math.dispose?.(); }
      return { geometries };
    } catch (problem) { throw Error("Geometry actor failed: "+String(problem?.stack ?? problem?.message ?? problem)); }
    finally { await ext.dispose?.(); }
  })(),{loader,packs,mushroom,scenarios,verifySolid:process.env.GEOMETRY_VERIFY_SOLID === "1",capabilityGroup:process.env.GEOMETRY_CAPABILITY_GROUP ?? "brep"});
  if (result == null || (scenarios && (!Array.isArray(result) || result.length !== scenarios.length))) throw Error("Browser capability probe ended without all expected results");
  console.log("[DEBUG] Actual browser capability result",JSON.stringify(result));
} finally {
  clearTimeout(timeout);
  await browser.close();
}
