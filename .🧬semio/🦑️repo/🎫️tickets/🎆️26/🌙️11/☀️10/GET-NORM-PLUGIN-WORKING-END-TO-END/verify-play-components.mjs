import {createRequire} from "node:module";
import {spawn} from "node:child_process";
import {createHash} from "node:crypto";
import {createReadStream} from "node:fs";
import {open, readFile, readdir, writeFile} from "node:fs/promises";
import {resolve, join} from "node:path";
import {fileURLToPath} from "node:url";

const ticket = fileURLToPath(new URL(".", import.meta.url));
const root = resolve(ticket, "../../../../../../..");
const output = join(ticket, "🗑️generated");
const play = join(root, "🏢️semio-tech", "🎡️play");
const require = createRequire(join(play, "package.json"));
const cli = resolve(require.resolve("playwright/package.json"), "../cli.js");
const prototype = join(root, ".cache/b/p/m/browser/prototype");
const modules = join(prototype, "modules");
const allIds = ["din4108","din16798","din18599","en1990","en1991","en1992","en1993","en1994","en1995","en1996","en1997","en1998","en1999","iso16757","vdi3805"];
const componentRoutes = new Map([
  ["norm", ["din4108", "din16798", "iso16757", "vdi3805"]],
  ["norm-energy", ["din18599"]],
  ["norm-eurocode", ["en1990", "en1991", "en1997", "en1998"]],
  ["norm-eurocode-structural", ["en1992", "en1993", "en1994"]],
  ["norm-eurocode-materials", ["en1995", "en1996", "en1999"]],
]);
const env = {...process.env, PLAY_E2E_GPU:"swiftshader", PLAYWRIGHT_BASE_URL:process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6033", PLAYWRIGHT_BROWSERS_PATH:join(root,".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright")};

async function run(executable, args, cwd, log, environment = env) {
  const file = await open(join(output,log),"w");
  try {
    return await new Promise((ok,fail) => {
      const child = spawn(executable, args, {cwd, env:environment, stdio:["ignore",file.fd,file.fd]});
      child.once("error",fail);
      child.once("exit",(code,signal) => ok(code ?? (signal ? 1 : 0)));
    });
  } finally {await file.close();}
}

async function digest(stream) {
  const hash = createHash("sha256");
  for await (const chunk of stream) hash.update(chunk);
  return hash.digest("hex");
}

async function currentGeneration(directory) {
  const head = await readFile(join(directory, "head.json"), "utf8").then(source => JSON.parse(source), error => {
    if (error.code !== "ENOENT") throw error;
  });
  return head && /^[a-f0-9]{24}$/.test(head.generation) ? join(directory, head.generation) : undefined;
}

async function verifyDelivery(selected) {
  const wrappers = join(prototype, "component-models");
  const wrapped = new Map();
  for (const entry of await readdir(wrappers, {withFileTypes:true})) {
    if (!entry.isDirectory()) continue;
    const directory = await currentGeneration(join(wrappers, entry.name));
    if (!directory) continue;
    const receipt = await readFile(join(directory, "🧾️receipt.json")).catch(error => {
      if (error.code !== "ENOENT") throw error;
    });
    if (receipt) {
      const pluginId = JSON.parse(receipt).pluginId;
      if (deliveries.get(pluginId)?.receipt.equals(receipt)) wrapped.set(pluginId, {directory, receipt});
    }
  }
  for (const [component, routes] of componentRoutes) {
    if (!routes.some(id => selected.includes(id))) continue;
    const module = deliveries.get(component);
    const wrapper = wrapped.get(component);
    if (!wrapper || !module.receipt.equals(wrapper.receipt)) throw Error(`${component}: wrapper and browser receipts differ`);
    const componentDigest = await digest(createReadStream(join(wrapper.directory,"📦️component.wasm")));
    if (componentDigest !== JSON.parse(module.receipt).wasmSha256) throw Error(`${component}: wrapper does not match its receipt`);
    const files = await readdir(module.path);
    const cores = files.filter(name => name.endsWith(".core.wasm"));
    if (cores.length !== 1) throw Error(`${component}: expected one browser core`);
    const names = ["🧾️receipt.json", "🌉️bridge.js", cores[0].replace(/\.core\.wasm$/u, ".js"), cores[0]];
    const served = {};
    for (const name of names) {
      const url = new URL(["", "🔌️plugin-modules", module.dir, name].map(encodeURIComponent).join("/"),env.PLAYWRIGHT_BASE_URL);
      const response = await fetch(url, {signal:AbortSignal.timeout(30000)});
      if (!response.ok || !response.body) throw Error(`${component}: ${name} returned HTTP ${response.status}`);
      const sha256 = await digest(response.body);
      if (sha256 !== await digest(createReadStream(join(module.path,name)))) throw Error(`${component}: live ${name} differs from publication`);
      served[name] = sha256;
    }
    if (!module.receipt.equals(await readFile(join(module.path,"🧾️receipt.json")))) throw Error(`${component}: serving receipt moved during verification`);
    console.log(JSON.stringify({component,componentDigest,served}));
  }
}

const phase = process.argv[2] ?? "all";
if (!["all","lanes","browser","workflow","delivery","routes"].includes(phase)) throw Error("Unknown verification phase: " + phase);
const ids = process.argv.length > 3 ? process.argv.slice(3) : allIds;
if (ids.some(id => !allIds.includes(id)) || new Set(ids).size !== ids.length) throw Error("Invalid Standards variant selection");
const runName = ids === allIds ? "verified" : `verified-${createHash("sha256").update(ids.join(",")).digest("hex").slice(0, 8)}`;
const required = [...componentRoutes].filter(([,routes]) => routes.some(id => ids.includes(id))).map(([component]) => component);
const receipts = [];
const deliveries = new Map();
for (const dir of await readdir(modules)) {
  const path = join(modules,dir);
  if (!required.some(component => dir.endsWith(component))) continue;
  const receiptBytes = await readFile(join(path,"🧾️receipt.json"));
  const receipt = JSON.parse(receiptBytes);
  if (!required.includes(receipt.pluginId) || deliveries.has(receipt.pluginId)) throw Error("Unexpected or duplicate Standards component " + receipt.pluginId);
  deliveries.set(receipt.pluginId,{dir,path,receipt:receiptBytes});
  const bridge = await readFile(join(path,"🌉️bridge.js"),"utf8");
  if (!bridge.includes("now + admission.maximumElapsedUs")) throw Error("Staged bridge requires an issued deadline in " + dir);
  receipts.push({component:receipt.pluginId,wasmSha256:receipt.wasmSha256,sourceTreeDigest:receipt.sourceTreeDigest});
}
const missing = required.filter(component => !deliveries.has(component));
if (missing.length) throw Error("Missing Standards components " + missing.join(", "));
await writeFile(join(output,`${runName}-components.json`),JSON.stringify(receipts,null,2));
if (["all","lanes"].includes(phase) && await run("bash",[join(ticket,"play-standards-native.sh"),"lane"],root,`${runName}-lanes.log`)) throw Error("Standards lane activation failed");
if (phase === "lanes") process.exit(0);
await verifyDelivery(ids);
if (phase === "delivery") process.exit(0);
let routeExit = 0;
if (phase !== "workflow") {
  const log = `${runName}-${phase === "routes" ? "paint" : "routes"}.jsonl`;
  const routeEnvironment = phase === "routes" ? {...env, PLAY_TEST_REQUIRE_DRAW:"1"} : env;
  routeExit = await run(process.execPath,[join(ticket,"probe-norm-routes.mjs"),...ids],root,log,routeEnvironment);
  const routeRows = (await readFile(join(output,log),"utf8")).split("\n").filter(line => line.startsWith("{")).map(line => JSON.parse(line));
  const failedRoutes = ids.filter(id => {
    const rows = routeRows.filter(row => row.id === id);
    return rows.length !== 1 || rows[0].shell?.shellReady !== id || rows[0].error || rows[0].faults?.length || rows[0].failedRequests?.length || (phase === "routes" && !rows[0].draw?.vertices);
  });
  if (routeExit || failedRoutes.length) console.error("Failed route verification", ...failedRoutes);
  routeExit ||= Number(failedRoutes.length > 0);
}
const testExit = phase === "routes" ? 0 : await run(process.execPath,[cli,"test","--config","🔨️modules/🧪️e2e/🎚️config/🟦️.ts","--output",join(output,`${runName}-panes`),"--grep","boots .*\\((" + ids.join("|") + ")\\)"],play,`${runName}-panes.log`);

for (const [component, routes] of componentRoutes) {
  if (!routes.some(id => ids.includes(id))) continue;
  const module = deliveries.get(component);
  if (!module.receipt.equals(await readFile(join(module.path,"🧾️receipt.json")))) throw Error(`${component}: published receipt moved during browser verification`);
}
process.exitCode = testExit || routeExit;
