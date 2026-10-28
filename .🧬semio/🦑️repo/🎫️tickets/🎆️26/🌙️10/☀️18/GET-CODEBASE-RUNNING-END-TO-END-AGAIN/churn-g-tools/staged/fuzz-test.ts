/** 🎲️ Contract: a build that observes a tree while a deterministic peer renames, deletes, rewrites, tears, swaps and moves its files ends CONSISTENT for every seed (the manifest it wrote equals one of the tree states the independent Python peer produced, no crash, no hang, no partial write); the same oracle convicts an observer without the stable snapshot (negative control), and the peer's schedule is a pure function of the seed. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";

type Summary={variant:string;mode:string;seeds:number;outcomes:Record<string,number>;meanWall:number;restarted:number;retried:number};
const python=testPythonV1()!,here=dirname(fileURLToPath(import.meta.url)),root=fileURLToPath(new URL("../../../../../../../",import.meta.url)),harness=join(here,"🐍️.py");
function fuzz(variant:string,seeds:string,extra:readonly string[]=[]):Summary{
 const out=realpathSync(mkdtempSync(join(tmpdir(),"churn-fuzz-")));
 try{
  const done=Bun.spawnSync([python,"-I",harness,"run","--lib",root,"--variant",variant,"--seeds",seeds,"--mode","sync","--jobs","4","--out",out,"--timeout","90",...extra]);
  const lines=done.stdout.toString().trim().split("\n");
  if(done.exitCode!==0)throw Error("Fuzz harness failed: "+done.stderr.toString().slice(-600));
  return JSON.parse(lines.at(-1)!) as Summary;
 }finally{rmSync(out,{recursive:true,force:true});}
}
const schedule=(seed:number):unknown=>JSON.parse(Bun.spawnSync([python,"-I","-c","import importlib.util,json,sys\nspec=importlib.util.spec_from_file_location('harness',sys.argv[1])\nm=importlib.util.module_from_spec(spec)\nspec.loader.exec_module(m)\nprint(json.dumps(m.Peer('/nonexistent',int(sys.argv[2]),'sync',6).ops))",harness,String(seed)]).stdout.toString());

test("the peer's schedule is a pure function of the seed",()=>{
 expect(schedule(7)).toEqual(schedule(7));expect(schedule(7)).not.toEqual(schedule(8));expect((schedule(7) as unknown[]).length).toBe(6);
});

test("sixteen seeds of the fixture build under the real restart supervisor end consistent while the peer churns the tree",()=>{
 const summary=fuzz("stable","0:16");
 expect(summary.outcomes).toEqual({consistent:16});
 expect(summary.retried).toBeGreaterThan(0);
},180_000);

test("negative control: the same peer and oracle convict an observer that lists and hashes without the stable snapshot (its stale-protocol check raises the plain error the closure-read session used to raise, and only the snapshot cures its mixes)",()=>{
 const summary=fuzz("naive","0:24",["--naive"]);
 expect(summary.outcomes.hang??0).toBe(0);
 expect((summary.outcomes.mix??0)+(summary.outcomes.partial??0)).toBeGreaterThan(0);expect(summary.outcomes.crash??0).toBeGreaterThan(0);
},180_000);
