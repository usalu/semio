import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
const root = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost";
const path = `${root}/🏗️dock-assembly/🪟️panel-reveal-and-tool-tabs/🟦️.ts`;
const text = readFileSync(path, "utf8");
const start = text.indexOf("    const deferred = agentDeferredPlansRef.current;");
const end = text.indexOf("  }, []);", start);
if (start < 0 || end < 0) throw new Error("route extraction markers absent");
const callback = text.slice(start, end).replace("    agentArtifactRouteRef.current = async", "    return async");
const names = "agentDeferredPlansRef, sessionRef, spawnedAppsRef, loadedPluginsRef, onActionRef, activeWindowIdRef";
const typeNames = names.split(", ").map(name => JSON.stringify(name)).join(" | ");
const header = `/** 🌉️ Runs live agent artifact commands through the shell's owned action lane. */
import { shellAppFault, type ShellAppFrameV1 } from "@semio-tech/framework/mcp/channel";
import type { useDockModel } from "../🏗️dock-assembly/🧱️dock-model-and-persistence/🟦️.ts";
import { spawnedIdOfWindowInstanceV1, spawnedWindowInstanceIdV1 } from "../🪟️spawned-program/🟦️.ts";

type AgentArtifactPortsV1 = Pick<ReturnType<typeof useDockModel>, ${typeNames}>;

/** 🗿️ Creates an instance-addressed route without mounting the shell chrome. */
export function createAgentArtifactRouteV1({ ${names} }: AgentArtifactPortsV1): NonNullable<ReturnType<typeof useDockModel>["agentArtifactRouteRef"]["current"]> {
${callback}
}
`;
mkdirSync(`${root}/🌉️agent-artifact-route`, { recursive: true });
writeFileSync(`${root}/🌉️agent-artifact-route/🟦️.ts`, header);
writeFileSync(path, text.slice(0, start) + `    agentArtifactRouteRef.current = createAgentArtifactRouteV1({ ${names} });\n` + text.slice(end));
let host = readFileSync(path, "utf8").replace('import { shellAppFault, type ShellAppFrameV1 } from "@semio-tech/framework/mcp/channel";', 'import { createAgentArtifactRouteV1 } from "../../🌉️agent-artifact-route/🟦️.ts";');
writeFileSync(path, host);
