/** 🍰️ Contract: the memo of a slice reads exactly the facts the planner verifies for it (classes naming a slice owner, the environment owner or a pseudo owner that supports a derived fact they hold; never a derived row), so a fact of a project outside the slice that went stale cannot keep the memo from serving. The corpus `./🔣️.json` is authored; the engine's own `observationsHold` is the oracle for the stale case (the live failure of 10/27: a whole-generation memo of the course stayed `stale` on a `typescriptRealpath` row of another project). */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {buildGraphMemoRelevanceV1,graphMemoSliceOwnersV1,graphMemoSliceRowIdsV1} from "../../📤️relevance/🟦️.ts";
import {observationsHold} from "../../../../../👁️observation/🟨️.mjs";
import corpus from "./🔣️.json";

type Index=Parameters<typeof graphMemoSliceRowIdsV1>[0];

test("the slice reads its own classes, the environment and the supports of its derived facts, never a derived row or a class of another project",()=>{
 expect(graphMemoSliceRowIdsV1(corpus.index as unknown as Index,corpus.slice)).toEqual(corpus.expected);
 expect(graphMemoSliceOwnersV1(corpus.graph)).toEqual(corpus.owners);
 expect(graphMemoSliceRowIdsV1(corpus.index as unknown as Index,[])).toEqual([2]);
});

test("a stale fact of a project outside the slice leaves the slice's re-check holding, while the whole generation's re-check is stale (engine oracle)",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"memo-slice-")));
 try{
  for(const name of["slice","outside"])mkdirSync(join(root,name));
  for(const name of["slice/target","outside/old","outside/new"])writeFileSync(join(root,name),name);
  symlinkSync(join(root,"slice/target"),join(root,"slice/link"));symlinkSync(join(root,"outside/new"),join(root,"outside/link"));
  const index={version:1,owners:["@semio-tech/slice","@semio-tech/outside","@env"],rows:[{kind:"typescriptRealpath",path:join(root,"slice/link"),fact:join(root,"slice/target")},{kind:"typescriptRealpath",path:join(root,"outside/link"),fact:join(root,"outside/old")}],classes:[{r:[0],o:[0]},{r:[1],o:[1]}]} as unknown as Index&{rows:{kind:string;path:string;fact:unknown}[]};
  const build=(ids:readonly number[])=>buildGraphMemoRelevanceV1({root,rows:()=>ids.map(id=>index.rows[id]!),frontier:{directories:[],members:[]},excluded:()=>false,isLink:async()=>false});
  const sliced=await build(graphMemoSliceRowIdsV1(index,["@semio-tech/slice"])),whole=await build([0,1]);
  expect(sliced.recheck.rows.length).toBe(1);expect(whole.recheck.rows.length).toBe(2);
  expect(await observationsHold(sliced.recheck.rows,64,new Set(),true)).toBe(true);
  expect(await observationsHold(whole.recheck.rows,64,new Set(),true)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});
