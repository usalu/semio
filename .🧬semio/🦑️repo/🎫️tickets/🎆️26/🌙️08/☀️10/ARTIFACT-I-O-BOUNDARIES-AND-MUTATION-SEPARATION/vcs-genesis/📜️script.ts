import {resolve,join} from "node:path";
import {readFileSync,mkdirSync,writeFileSync} from "node:fs";
const root=resolve(import.meta.dir,"../../../../../../../..");
const ticket=resolve(import.meta.dir,"..");
if(process.argv[2]==="source"){
 const p=Bun.spawn(["bun","test",resolve(import.meta.dir,"🟦️.ts")],{cwd:root,stdout:"inherit",stderr:"inherit"});
 if(await p.exited!==0)throw Error("Genesis ownership/refusal law failed");
 const owners=JSON.parse(readFileSync(join(import.meta.dir,"🔣️owners.json"),"utf8")).rustOwners as string[];
 for(const owner of owners){const parser=Bun.spawn(["rustfmt","--emit","stdout","--config","skip_children=true","--edition","2021",join(root,owner)],{cwd:root,stdout:"ignore",stderr:"inherit"});if(await parser.exited!==0)throw Error("Genesis Rust syntax refused "+owner);}
 console.log(`[DEBUG] Genesis actual Rust syntax owners=${owners.length}; source syntax only`);
}else if(process.argv[2]==="native"){
 const{runCargoTestsV1,readCargoTestPolicyV1}=await import("../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts");
 const manifest=join(import.meta.dir,"Cargo.toml");
 const policy={version:1,manifestPath:manifest,targetDirectory:join(ticket,"🗑️generated/presentation-value-native/target"),buildDirectory:join(ticket,"🗑️generated/presentation-value-native/target/build"),leaseDirectory:join(ticket,"🗑️generated/vcs-genesis-native/captures/compiler-leases"),nextest:false,configPath:join(root,".config/nextest.toml"),level:"long",assertionBudgets:{fundamental:60000,quick:60000,long:60000,exhaustive:60000},buildBudgetMs:600000,assertionThreads:1,artifactDirectory:join(ticket,"🗑️generated/vcs-genesis-native/captures"),retainArtifacts:true,coverageEnabled:false,coveragePath:null,rustMinStack:"67108864"};
 process.env.SEMIO_CARGO_TEST_POLICY=JSON.stringify(policy);process.env.SEMIO_TEST_LEVEL="long";
 const files=["🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🌱️genesis/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🌱️genesis/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/📝️text/🌱️genesis/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🌱️genesis/🧫️fixtures/🔣️.json","🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts",manifest,join(import.meta.dir,"🦀️.rs"),import.meta.path];
 const before=files.map(file=>({file:resolve(root,file),body:readFileSync(resolve(root,file),"utf8")}));
 const custody=join(ticket,"🗑️generated/vcs-genesis-native/custody");mkdirSync(custody,{recursive:true});
 const receipt=join(custody,`owners-${Date.now()}.json`);writeFileSync(receipt,JSON.stringify({before,terminal:false}));
 await runCargoTestsV1({manifestPath:manifest,packages:[],cwd:import.meta.dir,extraArgs:["--lib","--offline","--","--nocapture"]},readCargoTestPolicyV1(process.env));
 const exact=before.every(owner=>readFileSync(owner.file,"utf8")===owner.body);writeFileSync(receipt,JSON.stringify({before,terminal:true,exact}));if(!exact)throw Error("Genesis native source/producer interval advanced");console.log(`[DEBUG] Genesis native actual defining sources/fixture/driver/producer exact=${exact}; owners=${before.length}`);
}else throw Error("Expected source or native");
