import { loadMcpSchema } from "@semio-tech/framework-os-mcp/testing/fixtures";
const schema: any = await loadMcpSchema();
for (const k of ["InferenceRetainedAuthority","InferenceRetainedTurnInput","InferenceSubmitInput","InferenceOriginalWork"]) console.log(k, JSON.stringify(schema.$defs[k]).slice(0, 1100));
