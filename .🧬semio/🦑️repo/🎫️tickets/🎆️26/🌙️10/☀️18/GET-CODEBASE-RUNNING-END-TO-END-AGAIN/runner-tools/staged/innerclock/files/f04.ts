/** ⏲️ Law: every inner tool timeout of a level (go `-timeout`, vitest, bun, pytest, dotnet blame-hang, Playwright) is the level budget stretched by the host's contention ratio, clamped to the load-aware budget clock's bounds, exactly as the independent Python model derives it from the stretch-1 corpus; an idle host keeps the corpus vectors and a busy one no longer kills `go test` at the idle budget. */
import {expect,test} from "bun:test";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {bunTestLevelArgs,dotnetLevelArgs,goLevelTestArgs,playwrightTestTimeoutMs,pytestLevelArgs,vitestLevelArgs} from "../../🟦️.ts";
import corpus from "../🔣️.json";

const here=dirname(fileURLToPath(import.meta.url)),policy=fileURLToPath(new URL("../../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/⏲️contention/🔣️.json",import.meta.url));
const ratios=[0,0.5,1,1.3,3,7.9,20];
const saved={budget:process.env.SEMIO_TEST_BUDGET_MS,level:process.env.SEMIO_TEST_LEVEL};

test("stretched inner timeouts equal the independent Python model for every level and contention ratio",()=>{
 delete process.env.SEMIO_TEST_BUDGET_MS;delete process.env.SEMIO_TEST_LEVEL;
 try{
  const python=testPythonV1();if(!python)throw Error("Independent Python oracle required");
  const cases=corpus.levels.flatMap(row=>ratios.map(ratio=>({level:row.level,ratio})));
  const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),fileURLToPath(new URL("../🔣️.json",import.meta.url)),policy],{stdin:Buffer.from(JSON.stringify(cases))});
  expect(result.stderr.toString()).toBe("");
  const model=JSON.parse(result.stdout.toString()) as unknown[];
  cases.forEach((row,index)=>{
   const level=row.level as "fundamental";
   expect<unknown>({go:goLevelTestArgs(level,row.ratio),vitest:vitestLevelArgs(level,row.ratio),bun:bunTestLevelArgs(level,row.ratio),pytest:pytestLevelArgs(level,row.ratio),dotnet:dotnetLevelArgs(level,row.ratio),playwright:playwrightTestTimeoutMs(level,row.ratio)},`${row.level} ×${row.ratio}`).toEqual(model[index]);
  });
 }finally{if(saved.budget===undefined)delete process.env.SEMIO_TEST_BUDGET_MS;else process.env.SEMIO_TEST_BUDGET_MS=saved.budget;if(saved.level===undefined)delete process.env.SEMIO_TEST_LEVEL;else process.env.SEMIO_TEST_LEVEL=saved.level;}
});

test("a host three times busier gives go test three times the fundamental budget, never more than the clock's ceiling",()=>{
 delete process.env.SEMIO_TEST_BUDGET_MS;delete process.env.SEMIO_TEST_LEVEL;
 try{
  expect(goLevelTestArgs("fundamental",3).slice(0,2)).toEqual(["-timeout","45s"]);
  expect(goLevelTestArgs("fundamental",1000).slice(0,2)).toEqual(["-timeout","120s"]);
  expect(goLevelTestArgs("fundamental",0.2).slice(0,2)).toEqual(["-timeout","15s"]);
 }finally{if(saved.budget!==undefined)process.env.SEMIO_TEST_BUDGET_MS=saved.budget;if(saved.level!==undefined)process.env.SEMIO_TEST_LEVEL=saved.level;}
});
