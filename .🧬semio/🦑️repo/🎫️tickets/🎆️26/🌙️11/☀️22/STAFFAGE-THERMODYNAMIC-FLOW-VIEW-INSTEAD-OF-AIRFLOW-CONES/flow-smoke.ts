/** 🧪️ Smoke run of the flow view: develops the transport of catalog assets in named environments, prints timings and field facts and writes SVG drawings into `🗑️generated`. Usage: bun flow-smoke.ts [category/slug:pose|clip:<id>]... [--env id,id] [--vp elevation,front,plan] [--scale 50] */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const S = resolve(here, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const lib = await import(join(S, "📦️packages/🟦️typescript/🟦️.ts"));
const transport = await import(join(S, "🔨️modules/💨️transport/🟦️.ts"));
const presets = JSON.parse(readFileSync(join(S, "🗂️catalog/🌤️environments/🔣️.json"), "utf8")).presets as { id: string; environment: Record<string, number> }[];
const args = process.argv.slice(2);
const flag = (name: string, fallback: string): string => (args.includes(name) ? args[args.indexOf(name) + 1]! : fallback);
const targets = args.filter((a, i) => !a.startsWith("--") && !(args[i - 1] ?? "").startsWith("--"));
const envs = flag("--env", "office-neutral").split(",");
const viewpoints = flag("--vp", "elevation").split(",");
const scale = Number(flag("--scale", "50"));
const developS = Number(flag("--develop", "16"));
const out = join(here, "🗑️generated");
mkdirSync(out, { recursive: true });
for (const target of targets.length > 0 ? targets : ["👤️humans/standing-adult"]) {
  const [path, selector] = target.split(":") as [string, string | undefined];
  const asset = JSON.parse(readFileSync(join(S, "🗂️catalog", path, "🔣️.json"), "utf8")).asset;
  const query = selector === undefined ? { kind: "pose", pose: asset.presentation.defaultPose } : selector.startsWith("clip=") ? { kind: "clip", clip: selector.slice(5), timeS: 0 } : { kind: "pose", pose: selector };
  for (const env of envs) {
    const environment = env === "own" ? asset.thermal.environment : { ...asset.thermal.environment, ...presets.find((p) => p.id === env)!.environment };
    const t0 = performance.now();
    let slices = 0;
    const field = await transport.developTransport(asset, { ...query, timeS: developS }, environment, { maxAgeS: developS, flow: true, pathlines: false, streamlines: false, progress: () => void slices++ });
    const t1 = performance.now();
    const again = transport.solveTransport(asset, { ...query, timeS: field.timeS + 1 / 30 }, environment, { maxAgeS: developS, flow: true, pathlines: false, streamlines: false });
    const t2 = performance.now();
    const flow = field.flow!;
    const speeds = flow.streams.flatMap((s: { speedMS: number[] }) => s.speedMS);
    const ex = flow.streams.flatMap((s: { excessK: number[] }) => s.excessK);
    console.log(`${asset.id} ${JSON.stringify(query)} env=${env} air=${environment.airTempC} rh=${environment.relativeHumidity} | develop ${(t1 - t0).toFixed(0)} ms (${slices} slices), next frame ${(t2 - t1).toFixed(0)} ms | parcels ${field.parcels.length} streams ${flow.streams.length} (${flow.streams.reduce((n: number, s: { points: unknown[] }) => n + s.points.length, 0)} pts) speed max ${Math.max(0, ...speeds).toFixed(3)} excess ${Math.min(0, ...ex).toFixed(2)}..${Math.max(0, ...ex).toFixed(2)} K vapour max ${(Math.max(0, ...flow.puffs.map((p: { vapourExcessKgPerKg: number }) => p.vapourExcessKgPerKg)) * 1000).toFixed(2)} g/kg fog ${flow.puffs.filter((p: { condensateKgPerKg: number }) => p.condensateKgPerKg > 0).length} dew ${flow.ambient.dewPointC.toFixed(1)} | emitters ${field.emitters.map((e: { emitter: string; active: boolean; volumeFlowM3S: number; temperatureC: number }) => `${e.emitter}:${e.active ? (e.volumeFlowM3S * 1000).toFixed(2) + "L/s@" + e.temperatureC.toFixed(1) : "off"}`).join(" ")} (${again.parcels.length})`);
    for (const viewpoint of viewpoints) {
      for (const theme of flag("--theme", "light").split(",")) {
        const options = { ...lib.defaultDepictionOptions("thermal", flag("--lang", "en"), scale, viewpoint, theme), environment, flow: ["streams", "tracers", "moisture", "fog"], flowDevelopS: developS };
        if (viewpoint === "axonometric") await lib.loadAxonometric();
        const state = lib.evaluateDrawing(asset, query, options.representation);
        const scene = lib.depict(state, options, { transport: field });
        const name = `${asset.id}.${env}.${viewpoint}.${scale}.${theme}.svg`;
        writeFileSync(join(out, name), lib.writeSvg(scene));
        console.log(`  wrote ${name} ${scene.widthMm.toFixed(0)}x${scene.heightMm.toFixed(0)} mm, legends: ${scene.legends.map((l: { quantity: string }) => l.quantity).join(", ")}`);
      }
    }
  }
}
