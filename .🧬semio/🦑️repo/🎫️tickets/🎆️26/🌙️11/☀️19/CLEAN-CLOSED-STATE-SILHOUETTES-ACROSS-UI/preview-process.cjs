const fs = require("node:fs");
const path = require("node:path");
const {spawn} = require("node:child_process");
const {createHash} = require("node:crypto");

const root = __dirname;
const identity = createHash("sha256").update(root).digest("hex");
const workspace = path.resolve(root,"../../../../../../..");
const previous = path.resolve(root,"../../☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI");
const staffage = "🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🏗️builder/🌐️vite";
const controlPort = Number(process.env.SEMIO_MATRIX_CONTROL_PORT ?? 5700);
const generated = path.join(root,"🗑️generated",controlPort === 5700 ? "matrix" : "matrix-" + controlPort);
const apps = [
  ["elements",5452,".",path.join(root,"vite-elements.config.ts")],
  ["course",5601,"🎓️teaching/🏛️architecture/📚️course","vite-preview.config.ts"],
  ["os",5602,".","vite-round3-os.config.ts"],
  ["cad",5603,".","vite-round3-cad.config.ts"],
  ["hub",5608,"🌎️hub/🔨️modules/🛡️admin","vite-hub-admin.config.ts","/admin/"],
  ["demo",5609,"♻️mit-bestand/🧺️demonstrator","r3-vite-demonstrator-frozen.config.ts"],
  ["staffage",5471,staffage,path.join(workspace,staffage,"🟦️.ts")],
  ["play",5397,"🏢️semio-tech/🎡️play","vite-play-frozen.config.ts"],
].map(([id,port,cwd,config,route = "/"], index) => ({
  id, port, route, proxy:controlPort + index + 1, cwd:path.join(workspace,cwd), config:path.resolve(previous,config),
}));
const publicApps = apps.map(({id,port,proxy,route}) => ({id,port,proxy,route}));

/** 🩺 Reads the preview's own entry route with a bounded timeout. */
async function readiness(app) {
  try {
    const response = await fetch("http://127.0.0.1:" + app.port + app.route, {redirect:"manual",signal:AbortSignal.timeout(2000)});
    await response.body?.cancel();
    return {ready:response.ok && response.headers.get("content-type")?.includes("text/html") === true, status:response.status};
  } catch {
    return {ready:false, status:"waiting"};
  }
}

/** 🚀 Runs the selected config behind the repository tool registry. */
function launch(app) {
  fs.mkdirSync(generated,{recursive:true});
  const log = fs.openSync(path.join(generated,app.id + "-server.log"),"a");
  const args = ["capacity","run","--memory=512","--","repo","tools","exec","vite","--","--configLoader","native",
    "--config",app.config,"--mode","prototype","--port",String(app.port),"--strictPort","--host","127.0.0.1"];
  const env = {...process.env,
    SEMIO_BUILD_MODE:"prototype",
    SEMIO_VITE_CACHE_DIRECTORY:path.join(workspace,".cache/b/p/m/sil-" + app.id),
    NODE_OPTIONS:[process.env.NODE_OPTIONS,"--experimental-transform-types"].filter(Boolean).join(" "),
    ...(app.id === "staffage" ? {TEACHING_ARCHITECTURE_STAFFAGE_WATCH:"off"} : {}),
  };
  try { return spawn("repo",args,{cwd:app.cwd,env,stdio:["ignore",log,log],windowsHide:true}); }
  finally { fs.closeSync(log); }
}

/** 🛑 Stops only verified child handles kept by this supervisor. */
function createPreviewLifecycle({inspect = readiness,spawnPreview = launch} = {}) {
  const owned = new Map();
  let tail = Promise.resolve();
  const serial = action => {
    const task = tail.then(action);
    tail = task.catch(()=>{});
    return task;
  };
  async function stop(app, owner) {
    for (const [id, entry] of owned) {
      if (app && app !== id) continue;
      if (owner) entry.owners.delete(owner);
      if (owner && entry.owners.size) continue;
      entry.child.kill("SIGTERM");
      await entry.exited;
    }
  }
  async function start(app, owner, restart) {
    const entry = owned.get(app.id);
    if (entry && (!restart || [...entry.owners].some(existing => existing !== owner))) {
      entry.owners.add(owner);
      return "reused";
    }
    if (entry) await stop(app.id,owner);
    if ((await inspect(app)).ready) return "external";
    const child = spawnPreview(app);
    const record = {child,owners:new Set([owner]),exited:null};
    record.exited = new Promise(resolve => {
      const done = () => {
        if (owned.get(app.id) === record) owned.delete(app.id);
        resolve();
      };
      child.once("error",done);
      child.once("exit",done);
    });
    owned.set(app.id,record);
    return "started";
  }
  return {
    startBatch: app => serial(async () => {
      for (const id of owned.keys()) if (id !== app.id) await stop(id,"batch");
      return start(app,"batch",false);
    }),
    stopBatch: () => serial(()=>stop(null,"batch")),
    startManual: (app,restart) => serial(()=>start(app,"manual",restart)),
    stopManual: app => serial(()=>stop(app,"manual")),
    shutdown: () => serial(()=>stop()),
  };
}

module.exports = {apps, publicApps, root, identity, previous, controlPort, generated, readiness, createPreviewLifecycle};
