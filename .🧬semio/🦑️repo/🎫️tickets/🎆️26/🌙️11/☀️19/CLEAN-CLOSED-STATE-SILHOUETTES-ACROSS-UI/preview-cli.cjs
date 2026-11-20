const fs = require("node:fs");
const {spawn} = require("node:child_process");
const path = require("node:path");
const {apps,controlPort,generated,root,identity,readiness} = require("./preview-process.cjs");

const control = "http://127.0.0.1:" + controlPort;
const pause = () => new Promise(resolve=>setTimeout(resolve,500));

/** 🧭 Uses the existing ticket runner or starts its independent local process. */
async function ensureRunner(autoStart) {
  const get = () => fetch(control + "/previews",{signal:AbortSignal.timeout(2000)}).catch(()=>null);
  const own = async response => response?.ok && (await response.json().catch(()=>null))?.identity === identity;
  const current = await get();
  if (await own(current)) return true;
  if (current) throw new Error("This port is in use by another or older runner. Finish its batch before switching to this runner.");
  if (!autoStart) return false;
  fs.mkdirSync(generated,{recursive:true});
  const log = fs.openSync(path.join(generated,"runner.log"),"a");
  try {
    const child = spawn("repo",["tools","exec","--light","node","--",path.join(root,"runner-server.cjs")],
      {cwd:root,stdio:["ignore",log,log],detached:true,windowsHide:true});
    child.on("error",error=>process.stderr.write(String(error) + "\n"));
    child.unref();
  } finally { fs.closeSync(log); }
  const deadline = performance.now() + 15000;
  while (performance.now() < deadline) {
    const response = await get();
    if (await own(response)) return true;
    await pause();
  }
  throw new Error("The preview supervisor did not respond. See " + path.join(generated,"runner.log"));
}

/** 👁️ Starts or stops the requested previews and confirms their actual entry routes. */
async function main([action = "start", id = "all"]) {
  const selected = id === "all" ? apps : apps.filter(app=>app.id===id);
  if (!["start","restart","stop"].includes(action) || !selected.length) throw new Error("Usage: preview-cli.cjs start|restart|stop [app|all]");
  if (!await ensureRunner(action !== "stop")) return;
  const response = await fetch(control + "/previews/" + action + "/" + id,{method:"POST"});
  if (!response.ok) throw new Error(await response.text());
  if (action === "stop") return;
  const started = await response.json();
  await Promise.all(selected.map(async app=>{
    const deadline = performance.now() + 90000;
    while (performance.now() < deadline) {
      const status = await readiness(app);
      if (status.ready) {
        console.log(app.id + " (" + started[app.id] + ") http://127.0.0.1:" + app.port + app.route);
        return;
      }
      await pause();
    }
    throw new Error(app.id + " did not start. See " + path.join(generated,app.id + "-server.log"));
  }));
}

main(process.argv.slice(2)).catch(error=>{
  console.error(error.message);
  process.exitCode = 1;
});
