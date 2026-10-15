import {join} from "node:path";
const root=process.env.SEMIO_WORKSPACE_ROOT??process.cwd();
const {verifyStdioCommandOwnership}=await import(join(root,"✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/📦️artifact-package-graph/🟦️.ts"));
await verifyStdioCommandOwnership(root);
console.log("[DEBUG] Stdio metadata Original, managed custody and owned capture Source law passed");
