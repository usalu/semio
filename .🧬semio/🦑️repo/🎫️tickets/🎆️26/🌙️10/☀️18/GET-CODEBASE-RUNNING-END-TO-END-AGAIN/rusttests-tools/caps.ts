import{createBuildBootstrapCapabilitiesV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📤️child/🟦️.ts";
import{issueRepositoryScriptProvidersV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
const root=process.cwd(),env:NodeJS.ProcessEnv={...process.env,SEMIO_WORKSPACE_ROOT:root,SEMIO_BUILD_MODE:"prototype"};
issueRepositoryScriptProvidersV1(env);
const args=process.argv.slice(2);
const c=await createBuildBootstrapCapabilitiesV1(root,args,env);
console.log(JSON.stringify(c,null,1).slice(0,6000));
