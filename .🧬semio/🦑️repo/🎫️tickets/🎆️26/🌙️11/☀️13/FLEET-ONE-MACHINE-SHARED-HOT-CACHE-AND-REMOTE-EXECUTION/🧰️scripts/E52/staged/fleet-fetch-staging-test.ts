/** 🛖️ Law: a download that has no worktree root and no store stages its bytes in the operating system's temporary directory and never below the working directory, so a run started inside a member directory leaves no cache root there. */
import {createHash} from "node:crypto";
import {existsSync,mkdtempSync,readdirSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {expect,test} from "bun:test";
import {fleetFetchContextV1,pinnedFetchV1} from "../../🟦️.ts";

test("a download without root and store leaves nothing in the working directory",async()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"fetch-cwd-"))),before=process.cwd(),body="upstream body",sha256=createHash("sha256").update(body).digest("hex");
 try{
  process.chdir(directory);
  const asked:string[]=[],fetchFake=(async(url:string,init?:RequestInit)=>{
   asked.push((init?.method??"GET")+" "+url.replace(/^https?:\/\/[^/]+/u,""));
   if(url.includes("/cas/")&&(init?.method??"GET")==="GET")return new Response("",{status:404});
   if(url.includes("/claim/"))return new Response(JSON.stringify({state:"granted"}),{status:200});
   if(url.includes("/cas/"))return new Response("",{status:200});
   return new Response(body,{status:200});
  })as unknown as typeof fetch;
  const context={...fleetFetchContextV1({SEMIO_FLEET_FETCH:"off"},undefined),remote:{endpoint:"http://fleet.test",instance:"i",token:"t",authenticated:true},fetch:fetchFake};
  expect(context.root).toBeUndefined();expect(context.storeRoot).toBeUndefined();
  const response=await pinnedFetchV1(context,{url:"https://example.test/blob.zip",sha256});
  expect(await response.text()).toBe(body);
  expect(asked.some(row=>row==="GET /blob.zip")).toBe(true);
  expect(readdirSync(directory)).toEqual([]);expect(existsSync(join(directory,".cache"))).toBe(false);
 }finally{process.chdir(before);rmSync(directory,{recursive:true,force:true});}
});
