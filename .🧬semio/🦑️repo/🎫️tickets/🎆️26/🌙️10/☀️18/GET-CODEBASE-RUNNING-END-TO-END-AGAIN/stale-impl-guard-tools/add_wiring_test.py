L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
p=L+"🏗️build/🧷️format-stamps/🧪️tests/🟦️.ts"
s=open(p,encoding="utf-8").read()
marker='test("an unreadable or incomplete table'
assert s.count(marker)==1
new='''test("the wiring stays in place: lease admission and wait hook, owner declarations, readers that refuse, child exit code and the outermost restart",()=>{
 const text=(...parts:string[])=>readFileSync(join(library,...parts),"utf8"),leases=readFileSync(join(library,"../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts"),"utf8");
 expect(leases).toContain('Symbol.for("'+protocol.admissionGuardRegistry+'")');
 expect(leases).toContain('runAdmissionGuardsV1(options,"waiting")');expect(leases).toContain("return admitResourceLeaseV1(options,upgraded)");expect(leases).toContain("return admitResourceLeaseV1(options,await acquire())");expect(leases).toContain("return admitResourceLeaseV1(options,lease)");
 const graph=text("🏗️build","🕸️graph","🟦️.ts");
 expect(graph).toContain('declareFormatStampV1("graph.generation",generationSchemaV1)');expect(graph).toContain('requireCurrentFormatStampsV1(graphFormatStampsV1,"graph generation","refuse")');expect(graph).toContain('requireCurrentFormatStampsV1(graphFormatStampsV1,"graph generation");return discoverBuildGraphPartV1');
 expect(text("🏗️build","👁️observation","🟨️.mjs")).toContain('declareFormatStampV1("graph.record",sourceAnalysisSchemaV1)');
 expect(text("🏗️build","🕸️graph","📥️inference","🔖️stamps","🟨️.mjs")).toContain('declareFormatStampV1("graph.inferrers", schema)');
 expect(text("⚡️caching","🚀️bootstrap","📦️dependencies","🚦️concurrent","🟦️.ts")).toContain('declareFormatStampV1("dependency.walk",generationSchema)');
 const storage=text("⚡️caching","🚀️bootstrap","📦️dependencies","🧾️generation","💾️storage","🟦️.ts");
 expect(storage).toContain('requireCurrentFormatStampsV1(["dependency.walk"],"dependency generation receipt","refuse")');expect(storage).toContain("staleImplementationOfV1(error)");
 const format=text("⚡️caching","💾️storage","🔒️enforcement","📦️budget","📥️opening","🏪️store","📇️format","🟦️.ts");
 expect(format).toContain('declareFormatStampV1("cache.graph-image",schema)');expect(format).toContain('requireCurrentFormatStampsV1(["cache.graph-image"],"graph cache image","refuse")');
 expect(text("🏗️build","⌨️command","📥️entry","🟦️.ts")).toContain("catch(error){exitIfStaleImplementationV1(error);throw error;}");
 const script=text("🏗️build","📜️script.ts");
 expect(script).toContain("runWithStaleReexecV1(()=>runBuildCliV1(root,argv,environment)");expect(script).toContain("outermost:environment[SCRIPT_PROCESS_INVOCATION_ENV]===undefined");expect(script).toContain("if(import.meta.main)await runBuildMainV1(");
 expect(text("🏗️build","🕸️graph","📥️inference","🔖️stamps","🧬️schema","🔣️.json")).toContain('"🏗️build/🧷️format-stamps/"');
});

'''
s=s.replace(marker,new+marker)
open(p,"w",encoding="utf-8").write(s)
