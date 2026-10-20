
test("hermetic class membership is decided from the class table, admits only declared host inputs and yields closed, location-independent manifests",()=>{
 const member=(definition:Record<string,unknown>,target="test",project="@teaching/architecture-course")=>({...task(definition),project,target,id:project+":"+target,root:"proj"}) as any,base={execution:{kind:"command",command:"bun ./📜️script.ts test",cwd:"proj"},outputs:[]};
 const decide=(definition:Record<string,unknown>,inputs:unknown[],cache=true,target="test",project="@teaching/architecture-course")=>api.bazelHermeticDecisionV1(member(definition,target,project),{cache},inputs);
 expect(decide(base,["{projectRoot}/**/*",{env:"SEMIO_TEST_LEVEL"},{runtime:"bun --version"},{externalDependencies:["zod"]}])).toEqual({hermetic:true,class:"ts-test"});
 expect(decide(base,[{env:"HOME"}])).toEqual({hermetic:false,reason:"host-inputs"});
 expect(decide(base,[{runtime:"git rev-parse HEAD"}])).toEqual({hermetic:false,reason:"host-inputs"});
 expect(decide({...base,outputs:undefined},[])).toEqual({hermetic:false,reason:"uncached"});
 expect(decide(base,[],false)).toEqual({hermetic:false,reason:"uncached"});
 expect(decide(base,[],true,"test","@semio-tech/other")).toEqual({hermetic:false,reason:"undeclared"});
 expect(decide(base,[],true,"publish")).toEqual({hermetic:false,reason:"undeclared"});
 expect(decide({...base,execution:{...base.execution,program:"x"}},[])).toEqual({hermetic:false,reason:"unsupported-execution"});
 expect(decide({...base,metadata:{semio:{hermetic:{version:1,tools:["bun"]}}}},[],true,"publish")).toEqual({hermetic:true});
 expect(decide({...base,continuous:true},[])).toEqual({hermetic:false,reason:"continuous"});
 const reasons=Object.keys(schema.definitions.Reasons.const);for(const reason of ["excepted","dependencies"])expect(reasons).toContain(reason);
 const admit=new Ajv({strict:true}).addSchema(schema,"component").compile({$ref:"component#/definitions/Manifest"}),row={path:"node_modules",workspace:{"@a/b":"lib"},tree:{zod:"0/zod@1/node_modules/zod"},bin:{tsc:"../typescript/bin/tsc"}};
 const manifest=api.bazelHermeticManifestV1(member(base),["--x"],{inputs:["proj/a.ts","proj/📜️script.ts"],class:"ts-test",inputEnv:{SEMIO_TEST_LEVEL:"quick"},providers:{"typescript-check":"p/🟦️.ts"},rows:[row]});
 expect(admit(manifest),JSON.stringify(admit.errors)).toBe(true);expect(manifest.class).toBe("ts-test");expect(manifest.inputs).toEqual(["proj/a.ts","proj/📜️script.ts"]);expect(manifest.nodeModules).toEqual([row]);
 expect(JSON.stringify(manifest)).not.toMatch(/\/Users\/|\/home\/|\/var\/|\/private\//u);
 expect(JSON.stringify(api.bazelHermeticManifestV1(member(structuredClone(base)),["--x"],{inputs:["proj/a.ts","proj/📜️script.ts"],class:"ts-test",inputEnv:{SEMIO_TEST_LEVEL:"quick"},providers:{"typescript-check":"p/🟦️.ts"},rows:[row]}))).toBe(JSON.stringify(manifest));
 for(const extra of [{workspace:"/repo"},{scratch:".x"},{inputs:[1]},{class:"Bad"},{nodeModules:[{...row,extra:1}]}])expect(admit({...manifest,...extra}),JSON.stringify(extra)).toBe(false);
 const tree={stores:["/a/node_modules/.bun","/b/node_modules/.bun"],ids:["0/x@1","1/y@2"]},moved={stores:["/c/node_modules/.bun","/d/node_modules/.bun"],ids:["0/x@1","1/y@2"]};
 expect(api.bazelHermeticModulesLabelV1(tree)).toBe(api.bazelHermeticModulesLabelV1(moved));expect(api.bazelHermeticModulesLabelV1(tree)).toMatch(/^m_[a-f0-9]{16}$/u);expect(api.bazelHermeticModulesLabelV1({...tree,ids:["0/x@2","1/y@2"]})).not.toBe(api.bazelHermeticModulesLabelV1(tree));
 expect(()=>api.bazelHermeticModulesManifestV1({stores:[],ids:[]})).toThrow("Closed hermetic dependency manifest required");
 const hostRules=api.BAZEL_HERMETIC_MODULES_STARLARK;expect(hostRules).toContain('"no-sandbox": "1"');expect(hostRules).toContain('"no-remote-exec": "1"');expect(api.BAZEL_HERMETIC_STARLARK).toContain("--modules=");
 expect(api.BAZEL_HERMETIC_DEPENDENCIES).not.toMatch(/Bun\.|from"\.\.|\/Users\//u);
});
