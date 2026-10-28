/** 🌉️ Contract and oracle: the lazy routing bridge of the lease module must reach the routing instance that issued a control. A test runner (Vitest on Node) loads the lease module through its own module graph, so a bridge that loads routing on its own hands the control to a second copy whose issuance table is empty ("Live Original resource control issuance required"). The witness runs the call shape of the course proctor build (an issued control taken through `withResourceLeases`) in the real runner. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createRequire} from "node:module";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";

test("a runner that loads the lease module through its own module graph can take a lease with the control it issued (the call shape of the course proctor build)",()=>{
 const vitest=join(dirname(createRequire(import.meta.url).resolve("vitest/package.json")),"vitest.mjs"),fixtures=fileURLToPath(new URL("./🧫️fixtures/",import.meta.url));
 const run=spawnSync("node",[vitest,"run","--config",join(fixtures,"🟨️.mjs"),"--configLoader","native","--root",fixtures],{encoding:"utf8"});
 expect(run.status,(run.stdout??"")+(run.stderr??"")).toBe(0);
},170000);
