# Kernel Target Failure in Whole 13

Actual failure retained from the complete whole13 runtime. It is not substituted by native-only suites or narrower acceptance.

```
teFileSync(manifest,fixture.manifest);writeFileSync(join(temporary,"src/lib.rs"),fixture.source);writeFileSync(join(temporary,"tests/selection.rs"),fixture.integration);writeFileSync(config,fixture.config);
38 |   const base=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8")).policies[0];
39 |   for(const row of fixture.cases){
40 |    process.env.SEMIO_TEST_LEVEL="fundamental";process.env.SEMIO_COVERAGE="0";
41 |    const environment={...process.env,CARGO_TARGET_DIR:target,CARGO_BUILD_TARGET_DIR:target,CARGO_BUILD_BUILD_DIR:target,SEMIO_CARGO_TEST_POLICY:JSON.stringify({...base,manifestPath:manifest,targetDirectory:target,configPath:config,artifactDirectory:temporary,retainArtifacts:false,coverageEnabled:false})};let calls=0;
42 |    const Runner=runInNewContext(body+";TestScript",{BundleScript:class{},resolve,process:{env:environment},resolveTestLevel:(args:string[])=>{const resolved=levels.resolveTestLevel(args);environment.SEMIO_TEST_LEVEL=resolved.level;return resolved;},readCargoTestPolicyV1:api.readCargoTestPolicyV1,runCargo:()=>{throw Error("Kernel runner bypasses exact bounded Cargo policy");},runCargoTestsV1:async(request:Parameters<typeof api.runCargoTestsV1>[0],policy:Parameters<typeof api.runCargoTestsV1>[1])=>{calls++;expect(request.manifestPath).toBe(manifest);expect(request.packages).toEqual([fixture.packageName]);expect([...request.extraArgs!]).toEqual(row.extraArgs);expect(policy.level).toBe(row.level);await api.runCargoTestsV1({...request,environment},policy);}});
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       ^
error: expect(received).toEqual(expected)

  [
+   "--features",
+   "mutation-testing",
    "--lib",
    "os_store::component::tests",
    "--",
    "--nocapture",
  ]

- Expected  - 0
+ Received  + 2

      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-13/snapshot/🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧪️tests/🟦️.ts:42:643)
      at run (file:///:4:30)
      at <anonymous> (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-cargo-callers/current-origin-whole-test-13/snapshot/🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧪️tests/🟦️.ts:43:65)
(fail) Kernel package executes neutral target selections through its exact bounded Cargo driver [605.88ms]
   Compiling semio-nextest-execution-fixture v0.0.0 (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/controlled-car
```

Whole13 candidate producer and exact roster were frozen. Caller target selection must bind its real manifest and preserve original neutral case/assertion meaning; expected argument changes need canonical schema/oracle evidence, not an assertion removal.
