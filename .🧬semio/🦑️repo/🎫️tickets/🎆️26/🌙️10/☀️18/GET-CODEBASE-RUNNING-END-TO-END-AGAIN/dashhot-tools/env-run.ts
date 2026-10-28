/** 🧾 Graph-free official-envelope runner (after ver-f-tools/proc-run.ts), parameterised by root: `bun env-run.ts <root> <project:target> <artifact-dir> -- <bun args...>`; runs `bun <args>` in <root> under a Script process envelope issued for `repo run <project:target>`. */
import {spawn} from "node:child_process";
import {mkdirSync} from "node:fs";
import {join,resolve} from "node:path";
const [rootArg,request,artArg,dash,...args]=process.argv.slice(2);
if(!rootArg||!request||!artArg||dash!=="--"||!args.length)throw Error("usage: bun env-run.ts <root> <project:target> <artifact-dir> -- <bun args...>");
const root=resolve(rootArg),art=resolve(artArg),lib=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),proc=join(root,"🧰️framework/🔨️modules/🏃️process");
const {createBuildBootstrapCapabilitiesV1}=await import(join(lib,"🏗️build/📥️caller/📤️child/🟦️.ts"));
const {issueRepositoryScriptProvidersV1}=await import(join(lib,"🏃️process/🔌️providers/🟦️.ts"));
const {pinnedRustToolchainV1}=await import(join(proc,"🧪️testing/🧰️toolchain/📌️pinned/🟦️.ts"));
const {createScriptProcessEnvelope,scriptProcessEnvironment}=await import(join(proc,"🧭️routing/📥️invocation/🏃️process/🟦️.ts"));
mkdirSync(art,{recursive:true});
const base:NodeJS.ProcessEnv={LANG:"en_US.UTF-8",...process.env,SEMIO_WORKSPACE_ROOT:root,SEMIO_BUILD_MODE:process.env.SEMIO_BUILD_MODE??"prototype",SEMIO_BUILD_TOOL:"semio",SEMIO_TEST_ARTIFACT_DIR:art};
Object.assign(base,pinnedRustToolchainV1(undefined,base).environment);issueRepositoryScriptProvidersV1(base);delete base.SEMIO_SCRIPT_STALL_MS;
const capabilityBase={...base};delete capabilityBase.SEMIO_TEST_ARTIFACT_DIR;
const capabilities=await createBuildBootstrapCapabilitiesV1(root,["run",request],capabilityBase);
const envelope=createScriptProcessEnvelope({version:1,owner:"dashhot:env-run",maximumElapsedMilliseconds:Number(process.env.DH_WALL??900)*1000},capabilities,Date.now());
const child=spawn("bun",args,{cwd:root,env:{...scriptProcessEnvironment(envelope,base),SEMIO_TEST_ARTIFACT_DIR:art} as NodeJS.ProcessEnv,stdio:["ignore","inherit","inherit"]});
child.on("close",code=>process.exit(code??1));
