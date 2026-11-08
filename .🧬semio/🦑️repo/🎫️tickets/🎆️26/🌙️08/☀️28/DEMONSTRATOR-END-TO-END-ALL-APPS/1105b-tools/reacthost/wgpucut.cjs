const fs=require("fs");const E=process.argv[2];
function edit(f,pairs){const p=E+"/"+f;let s=fs.readFileSync(p,"utf8");for(const [a,b,all] of pairs){const n=s.split(a).length-1;if(all?n<1:n!==1){console.error("count",n,a.slice(0,80));process.exit(1)}s=s.split(a).join(b)}fs.writeFileSync(p,s)}
edit("🐚️plugin-bridge/🔌️plugin-module-loader/🧲️contributions-and-backbone/🟦️.ts",[
 ["{ pluginId, consumedTopics, emptyInvocation","{ pluginId, consumedTopicsFor, emptyInvocation"],
 ["readonly consumedTopics: readonly string[] }","readonly consumedTopicsFor: (appId: string) => readonly string[] }"],
 ["wgpuBuildScopedContributionsPack(pluginId, undefined, consumedTopics)","wgpuBuildScopedContributionsPack(pluginId, undefined, consumedTopicsFor(appId))"]]);
edit("🐚️plugin-bridge/🔌️plugin-module-loader/🟦️.ts",[
 ["consumedTopics: readonly string[]): Promise<WgpuPluginHandle>","consumedTopicsFor: (appId: string) => readonly string[]): Promise<WgpuPluginHandle>"],
 ["createContributionsAndBackbone({ pluginId, consumedTopics, emptyInvocation","createContributionsAndBackbone({ pluginId, consumedTopicsFor, emptyInvocation"]]);
edit("🎞️frame-worker/🧵️state/🟦️.ts",[
 ['import type { PluginCatalog } from "@semio-tech/framework";','import { consumedTopicsForApp, type PluginCatalog } from "@semio-tech/framework";\nimport { contributionManifests } from "../../🐚️plugin-bridge/🎛️contributions-and-effects/🟦️.ts";'],
 ["export const consumedTopics = (id: string) => [...(workerCatalog?.plugins ?? []), ...(workerCatalog?.extensions ?? [])].find(row => row.pluginId === id)?.consumes ?? [];",
  "export const consumedTopics = (id: string) => (appId: string): readonly string[] =>\n  consumedTopicsForApp([...(workerCatalog?.plugins ?? []), ...(workerCatalog?.extensions ?? [])], [...contributionManifests].map(([pluginId, manifest]) => ({ pluginId, manifest })), id, appId);"]]);
