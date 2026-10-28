import{createScriptProcessEnvelope,readScriptPolicy}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{createBuildBootstrapCapabilitiesV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📤️child/🟦️.ts";
const policy=readScriptPolicy({version:1,owner:"repository:build",maximumElapsedMilliseconds:Number(process.env.CHAIN_WIRE_MS??3600000)});
const root=process.cwd(),capabilities=await createBuildBootstrapCapabilitiesV1(root,["run","@semio-tech/repo-lib:test"],process.env);
process.stdout.write(JSON.stringify(createScriptProcessEnvelope(policy,capabilities,Date.now())));
