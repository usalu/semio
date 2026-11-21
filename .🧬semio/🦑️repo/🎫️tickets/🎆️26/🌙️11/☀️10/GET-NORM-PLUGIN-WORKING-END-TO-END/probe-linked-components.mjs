import {open,stat,access} from "node:fs/promises";
import {setTimeout} from "node:timers/promises";
import {basename,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {spawn} from "node:child_process";
const ticket = fileURLToPath(new URL(".",import.meta.url));
const root = resolve(ticket,"../../../../../../..");
const run = resolve(process.argv[2]);
const pending = new Map([
  ["norm",["din4108","din16798","iso16757","vdi3805"]],
  ["norm-energy",["din18599"]],
  ["norm-eurocode",["en1990","en1991","en1997","en1998"]],
  ["norm-eurocode-structural",["en1992","en1993","en1994"]],
  ["norm-eurocode-materials",["en1995","en1996","en1999"]]
]);
const selection = process.argv.slice(3);
for (const family of selection) if (!pending.has(family)) throw Error("Unknown guest " + family);
if (selection.length) for (const family of pending.keys()) if (!selection.includes(family)) pending.delete(family);
const viewportOnly = process.env.PLAY_VIEWPORT_INITIAL_ONLY === "1";
const probe = viewportOnly ? "probe-eurocode-viewports.mjs" : "probe-norm-routes.mjs";
const suffix = viewportOnly ? "-viewport-initial" : "-preview";
const output = await open(join(ticket,"🗑️generated",basename(run)+suffix+".jsonl"),"w");
try {
  while (pending.size) {
    for (const [key,ids] of pending) {
      const name = "semio-hub-" + key + "-component";
      const base = join(run,"w",name,"transpiled",name.replaceAll("-","_"));
      const core = base + ".core.wasm";
      const bindings = base + ".js";
      try {
        await access(base + ".d.ts");
        await access(bindings);
        if (!(await stat(core)).size) continue;
      } catch {continue;}
      const env = {...process.env,PLAYWRIGHT_BASE_URL:process.env.PLAY_COMPONENT_PREVIEW_URL ?? "http://127.0.0.1:6034",PLAY_TEST_GUEST:core,PLAY_TEST_GUEST_JS:bindings,PLAYWRIGHT_BROWSERS_PATH:join(root,".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright")};
      const code = await new Promise((done,fail) => {
        const child = spawn(process.execPath,[join(ticket,probe),...ids],{cwd:root,env,stdio:["ignore",output.fd,"inherit"]});
        child.once("exit",done); child.once("error",fail);
      });
      if (code !== 0) throw Error("Early browser probe failed for " + key);
      pending.delete(key);
    }
    if (pending.size) {
      const completed = await stat(join(run,"r/report.md")).then(() => true,() => false);
      if (completed) throw Error("Build ended before preview for " + [...pending.keys()].join(", "));
      await setTimeout(4000);
    }
  }
} finally {await output.close();}
