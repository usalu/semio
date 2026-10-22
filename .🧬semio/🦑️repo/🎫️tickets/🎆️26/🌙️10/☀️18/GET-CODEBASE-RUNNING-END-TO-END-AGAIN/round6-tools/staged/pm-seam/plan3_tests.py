"""🧪️ Round 3: the contract test of the schema bases speaks regions (see plan3_regions.py)."""
import os
def declare(edit, paths):
    R1 = os.environ.get("R1", "/private/tmp/pm-seam/r3base")
    T = paths.MANAGERS + "/🧪️tests/🟦️.ts"
    text = open(os.path.join(R1, T), encoding="utf-8", newline="").read()
    start = text.index('test("the bun descriptor materializes the pre-seam install and generation schemas exactly')
    end_marker = "expect(installSchema.$defs.ManagerProtocol.const.runtime).toBe(process.versions.bun);\n });\n"
    end = text.index(end_marker, start) + len(end_marker)
    new = '''test("the bun descriptor materializes the pre-seam values at every manager-specific position of both schema bases, and its recipe keeps the hashed key order",()=>{
  const bun=packageManagerRegistryV1.get("bun"),view=schemaViewV1(bun.descriptor),regions=(base:unknown)=>[...managerRegionsV1(base,view)];
  const pinnedInstall=originalInstall as unknown as (readonly [string,unknown])[],pinnedGeneration=originalGeneration as unknown as (readonly [string,unknown])[];
  const protocolPath="/$defs/ManagerProtocol/const",installRegions=regions(installBase),generationRegions=regions(generationBase);
  expect(installRegions.filter(([path])=>path!==protocolPath)).toEqual(pinnedInstall);
  expect(generationRegions).toEqual(pinnedGeneration);
  expect(installRegions.find(([path])=>path===protocolPath)![1]).toEqual({manager:"bun",label:"Bun",lockFile:"bun.lock",scopeConfig:"bunfig.toml",generationDirectory:"j",strictProfilePrefix:"bun-",hostVersionKey:"bun",cacheVariable:"BUN_INSTALL_CACHE_DIR"});
  const recipePath="/$defs/ProducerRecipeProtocol/const",original=pinnedInstall.find(([path])=>path===recipePath)![1];
  expect(JSON.stringify(installRegions.find(([path])=>path===recipePath)![1])).toBe(JSON.stringify(original));
  expect(JSON.stringify(bun.recipe())).toBe(JSON.stringify(original));
  expect(installSchema.$defs.ManagerProtocol.manager).toBe(bun);
  expect(installSchema.$defs.ManagerProtocol.const.runtime).toBe(process.versions.bun);
 });
'''
    edit(T, "round3-test-regions", "seam3", text[start:end], new)
    edit(T, "round3-test-digests", "seam3", "expect(core.sha256HexV1(core.canonicalJsonV1(value)),manager.id+\" \"+name).toBe(vectorsOf(manager.id).materialized[name]);", "expect(core.sha256HexV1(core.canonicalJsonV1(managerRegionsV1(name===\"install\"?installBase:generationBase,view))),manager.id+\" \"+name).toBe(vectorsOf(manager.id).materialized[name]);")
