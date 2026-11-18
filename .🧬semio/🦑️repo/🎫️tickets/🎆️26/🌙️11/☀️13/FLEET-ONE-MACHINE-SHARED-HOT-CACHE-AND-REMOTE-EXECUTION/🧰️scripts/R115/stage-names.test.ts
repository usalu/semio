/** 🧭️ Contract: an owner that runs long with no row names what it waits on, in English and German, and the name changes with the wait: the farm child, then the memory governor, then the process itself. Oracle: the lines of `reportParkedOwnersV1` at each point of one `stagedPrepareV1`. */
import{expect,test}from"bun:test";
import{reportParkedOwnersV1,stagedPrepareV1}from"../../🟨️.mjs";

const lineAt=(name:string)=>{const lines:string[]=[];reportParkedOwnersV1([{name,seconds:200,rows:0}],new Map(),(line:string)=>lines.push(line));return lines[0]??"";};

test("the parked line names the farm child, then the memory governor, then the process",async()=>{
 const name="@semio-tech/norm-x-rs";let atFarm="",atResume="",release=()=>{};
 const gate=new Promise<void>(resolve=>{release=resolve;});
 const away=async(work:()=>Promise<unknown>)=>{const value=await work();atResume=lineAt(name);await gate;return value;};
 const run=stagedPrepareV1(name,away,async()=>{atFarm=lineAt(name);return true;});
 await Bun.sleep(20);release();expect(await run).toBe(true);
 const atProcess=lineAt(name);
 expect(atFarm).toContain("farm child");expect(atFarm).not.toContain("memory governor to let");
 expect(atResume).toContain("memory governor");expect(atResume).toContain("Speichersteuerung");
 expect(atProcess).toContain("in this process");expect(atProcess).toContain("diesem Prozess");
 for(const line of [atFarm,atResume,atProcess]){expect(line).toContain(" / Graph: ");expect(line).toContain("0 rows");expect(line).toContain("ohne Zeile");}
});

test("a failing farm preparation still leaves the process stage",async()=>{
 const name="@semio-tech/norm-y-rs";
 await expect(stagedPrepareV1(name,async(work:()=>Promise<unknown>)=>work(),async()=>{throw new Error("boom");})).rejects.toThrow("boom");
 expect(lineAt(name)).toContain("in this process");
});
