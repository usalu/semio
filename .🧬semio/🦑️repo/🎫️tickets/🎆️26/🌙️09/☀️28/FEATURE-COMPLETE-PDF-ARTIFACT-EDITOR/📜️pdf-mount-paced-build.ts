/** 🏁 Projects and mounts the exact paced PDF build as soon as signed module products appear. */
import assert from "node:assert/strict";
import { closeSync, openSync } from "node:fs";
import { join } from "node:path";

const [build,previous,run] = process.argv.slice(2);
assert(/^[a-z0-9-]+$/u.test(build) && /^[a-z0-9-]+$/u.test(run) && /^[a-f0-9]{64}$/u.test(previous));
const dir = join(import.meta.dir,"🗑️generated");
const browser = ".cache/b/p/m/browser/prototype";
const entries = [join(browser,"modules","📘️stdio-pdf"),join(browser,"components","94d8ca0d8ba9c1594ac4ea4c")];
const deadline = Date.now() + 120 * 60 * 1000;
const receipt = async (path: string) => {
  if (!await Bun.file(join(path,".semio-artifact.json")).exists()) return undefined;
  const file = Bun.file(join(path,"🧾️receipt.json"));
  if (!await file.exists()) return undefined;
  const result = await file.json();
  assert.equal(result.pluginId,"stdio-pdf");
  return result.wasmSha256 !== previous && /^[a-f0-9]{64}$/u.test(result.wasmSha256) && result.wasmBytes > 0 ? result.wasmSha256 : undefined;
};
let sha: string;
while (true) {
  const [module,component] = await Promise.all(entries.map(receipt));
  if (module !== undefined && module === component) {
    sha=module;
    break;
  }
  const log = await Bun.file(join(dir,`pdf-browser-${build}-pipeline.log`)).text();
  const results = [...log.matchAll(/\[DEBUG\] components-pdf finished with (\d+)/gu)];
  assert(results.every(result => ["0","75"].includes(result[1])), "PDF producer ended before publication");
  assert(Date.now() < deadline,"The signed paced PDF product did not publish");
  await Bun.sleep(4000);
}
console.log("[DEBUG] Component and Browser module agree on the new receipt",sha);

let child: ReturnType<typeof Bun.spawn> | undefined;
const cancel = () => child?.kill("SIGINT");
process.once("SIGINT",cancel);
process.once("SIGTERM",cancel);
async function invoke(path: string, options: string[], filename: string, env: NodeJS.ProcessEnv): Promise<number> {
  const log = openSync(join(dir,filename),"wx");
  try {
    child = Bun.spawn(["repo","tools","exec","--light","bun","--",join(import.meta.dir,path),...options],{env,stdout:log,stderr:log});
    return await child.exited;
  } finally {
    closeSync(log);
    child=undefined;
  }
}
const projection = run+"-projected";
const verified=await invoke("📜️pdf-browser-build-candidates.ts",["session","assets"],`pdf-browser-${projection}-pipeline.log`,{
  ...process.env,CARGO_NET_OFFLINE:"true",SEMIO_PDF_BROWSER_RUN:projection,
});
assert.equal(verified,0);
assert.equal(await receipt(entries[0]),sha,"The signed module changed during projection");
process.exitCode=await invoke("📜️pdf-browser-mounted-suite.ts",[run,"rotation"],`pdf-browser-${run}-mounted-suite.log`,{
  ...process.env,SEMIO_VITE_HMR:"0",SEMIO_PDF_TRACE_OPEN:"1",SEMIO_PDF_TRACE_FLOW:"1",SEMIO_PDF_TRACE_GUEST:"1",
});
console.log("[DEBUG] Paced PDF mounted rotation result",process.exitCode);
