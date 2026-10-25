import { resolve } from "node:path";
const root = resolve(import.meta.dir, "../../../../../../../..");
const { serveWarmMcpStdio } = await import(`${root}/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🟦️.ts`);
process.exit(await serveWarmMcpStdio(root, ["--folder", ".🧬semio/🔗space/os-mcp", "--scopes", "workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write"], process.env));
