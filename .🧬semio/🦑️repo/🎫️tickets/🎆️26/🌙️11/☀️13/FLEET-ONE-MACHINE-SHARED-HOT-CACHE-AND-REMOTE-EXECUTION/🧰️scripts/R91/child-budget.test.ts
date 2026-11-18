/** 🪣️ Contract: a walk child is recycled before its resident size passes the memory the plan reserved for it (`childBytes`), so the process tree of a slice worker stays within the reservation the farm took: children that were reserved at 768 MiB and recycled at 1.5 GiB stood at 1.4 GB each in the tree the service judges (10 children, 5 GB on a 16 GB worker). */
import{expect,test}from"bun:test";
import{WALK_FARM_V1}from"../../🟨️.mjs";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};

test("a walk child is recycled at or below the bytes the farm reserved for it, and the declared numbers say so in the schema",()=>{
 expect(WALK_FARM_V1.recycleBytes).toBeLessThanOrEqual(WALK_FARM_V1.childBytes);
 expect(WALK_FARM_V1.recycleBytes).toBe(schema.definitions.Policy.const.recycleBytes);
 expect(WALK_FARM_V1.childBytes).toBeGreaterThan(0);
 expect(WALK_FARM_V1.maximumChildren*WALK_FARM_V1.recycleBytes).toBeLessThanOrEqual(8*805306368);
});
