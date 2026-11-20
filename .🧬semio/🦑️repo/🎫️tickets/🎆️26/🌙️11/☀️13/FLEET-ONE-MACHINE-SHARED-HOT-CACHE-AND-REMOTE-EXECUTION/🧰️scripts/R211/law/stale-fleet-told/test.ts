/** 🧮️ Law: a verdict that is not `holds` of a farm child on a record that the read has just taken from the fleet is told like the verdict of the first look: the named row (`staleRowLine`), or for a verdict without a row (`other`: another key, `missing`: no record) a row of the kind `verdict` that names the verdict, goes to the read's log, up to the same number of lines. The verdict of the first look runs before the record is imported, answers "missing" for a record not stored yet and is the only one that was told, so 24 of 28 stale records of the read of 2026-11-18 15:46 UTC were refused without a word.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";

test("the verdict asked after the import is told when it is not holds, with its row or its name",async()=>{
 const text=await Bun.file(new URL("../../../🟨️.mjs",import.meta.url)).text();
 const at=text.indexOf("return farm.observe(observeJob).finally(");
 expect(at).toBeGreaterThan(0);
 const body=text.slice(at,at+700);
 expect(body).toContain('reply?.verdict!==undefined&&reply.verdict!=="holds"&&staleRowTold++<400');
 expect(body).toContain('reply.staleRow??Object.freeze({kind:"verdict",path:String(reply.verdict)');
 expect(body).toContain("ledger.staleRowLine(language,name,row)");
});

test("a record whose rows hold but which cannot be reused (not reusable, inventory records of its value missing here) is told too, with the reason",async()=>{
 const text=await Bun.file(new URL("../../../🟨️.mjs",import.meta.url)).text();
 const at=text.indexOf("reusable:async value=>{const why=");
 expect(at).toBeGreaterThan(0);
 const body=text.slice(at,at+900);
 expect(body).toContain("inferenceInventoryReferencesHoldV1(workspaceRoot,value,context.phaseControl)");
 expect(body).toContain('kind:"reuse"');expect(body).toContain("inventory records named by the record are not stored here");
 expect(body).toContain("staleRowTold++<400");
});
