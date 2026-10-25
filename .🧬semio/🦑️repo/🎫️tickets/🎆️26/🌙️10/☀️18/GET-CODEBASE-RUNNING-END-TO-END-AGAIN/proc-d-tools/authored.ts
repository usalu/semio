/** Authored census environment: `bun authored.ts <cmd...>` runs the command under a born envelope with artifacts + real route env; prints the child's tail. */
import {createBuildBootstrapCapabilitiesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📤️child/🟦️.ts";
import {issueRepositoryScriptProvidersV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
import {createScriptProcessEnvelope,scriptProcessEnvironment} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {mkdirSync} from "node:fs";
const root="/Users/ueli/Documents/semio",art="/private/tmp/claude-501/pd/authored";mkdirSync(art,{recursive:true});
const base:NodeJS.ProcessEnv={...process.env,SEMIO_WORKSPACE_ROOT:root,SEMIO_BUILD_MODE:"prototype",SEMIO_BUILD_TOOL:"semio",SEMIO_TEST_ARTIFACT_DIR:art};issueRepositoryScriptProvidersV1(base);
const capabilities={...await createBuildBootstrapCapabilitiesV1(root,process.env.AUTHORED_SELECTION?JSON.parse(process.env.AUTHORED_SELECTION):["run","@semio-tech/framework-process:test-census"],base),artifacts:art};
const env=scriptProcessEnvironment(createScriptProcessEnvelope({version:1,owner:"proc-d:authored",maximumElapsedMilliseconds:300000},capabilities,Date.now()),base) as NodeJS.ProcessEnv;
const child=Bun.spawn(process.argv.slice(2),{cwd:root,env,stdout:"inherit",stderr:"inherit"});process.exit(await child.exited);
