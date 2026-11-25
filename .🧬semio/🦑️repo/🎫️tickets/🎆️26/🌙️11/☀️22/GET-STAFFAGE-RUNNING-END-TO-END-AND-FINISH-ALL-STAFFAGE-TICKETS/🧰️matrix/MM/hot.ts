/** 🔥️ Times the phases of the matrix of assets from a frozen tree of the staffage sources, short enough for the sampling profiler: `bun [--cpu-prof] hot.ts --tree <root of a tree with 🎓️teaching> --only suitability,evaluate,render,derived [--count n] <asset id>:<denominator> …`.
 *
 * Phases: `suitability` (the first `resolveRepresentation`), `evaluate` (n instants spread over the poses and clips, with the SHA-256 of the evaluated rings so that two trees compare bit for bit), `render` (the seven modes of the default pose), `derived` (the derived-view findings). Lines carry the `[DEBUG] ` prefix and give processor seconds.
 *
 * @see ./phases.ts — the same on the live tree
 */
import { createHash } from "node:crypto";
import { join, resolve } from "node:path";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const S = join(resolve(flag("tree")!), "🎓️teaching", "🏛️architecture", "🧍️staffage");
const only = (flag("only") ?? "suitability,evaluate,render,derived").split(",");
const count = Number(flag("count") ?? 8);
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const { resolveViews } = await import(join(S, "🔨️modules/🔭️multiview/🟦️.ts"));
const { DEFAULT_PRINT_PROFILE, resolveRepresentation } = await import(join(S, "🔨️modules/🖨️representation/🟦️.ts"));
const { MODE_LIST, derivedFindings, drawOptions, loadAsset, scaleSelector } = await import(join(S, "🧪️tests/🔭️multiview-coverage/🧰️.ts"));
const cpu = (): number => {
  const used = process.cpuUsage();
  return (used.user + used.system) / 1e6;
};
for (const [index, word] of args.entries()) {
  if (!word.includes(":") || word.startsWith("--") || args[index - 1] === "--tree") continue;
  const [id, scale] = word.split(":") as [string, string];
  const denominator = Number(scale);
  const asset = loadAsset(id);
  const say = (label: string, seconds: number, extra = ""): void => console.log(`[DEBUG] ${word} ${label} ${seconds.toFixed(3)} s ${extra}`);
  let before = cpu();
  if (only.includes("suitability")) {
    const resolution = resolveRepresentation(asset, { denominator }, DEFAULT_PRINT_PROFILE);
    say("suitability", cpu() - before, createHash("sha256").update(JSON.stringify(resolution.suitability ?? resolution.kind)).digest("hex").slice(0, 16));
  }
  const queries = [...asset.poses.map((pose: { id: string }) => ({ kind: "pose", pose: pose.id })), ...asset.clips.flatMap((clip: { id: string; durationS: number }) => Array.from({ length: 6 }, (_, i) => ({ kind: "clip", clip: clip.id, timeS: (clip.durationS * (i + 0.37)) / 6 })))];
  const step = Math.max(1, Math.floor(queries.length / count));
  const picked = queries.filter((_, i) => i % step === 0).slice(0, count);
  if (only.includes("evaluate")) {
    const digest = createHash("sha256");
    before = cpu();
    for (const query of picked) {
      let text: string;
      try {
        const state = evaluate(asset, query, scaleSelector(denominator));
        text = JSON.stringify([state.shapes.elevation, state.shapes.front, state.shapes.plan].map((shapes) => shapes.map((shape: { rings: unknown }) => shape.rings)));
      } catch (error) {
        text = String(error);
      }
      digest.update(text);
    }
    const spent = cpu() - before;
    say("evaluate", spent / picked.length, `per state (${picked.length} states) ${digest.digest("hex").slice(0, 16)}`);
  }
  if (only.includes("render")) {
    const query = { kind: "pose", pose: asset.presentation.defaultPose };
    resolveViews(asset, query, { kind: "scale", denominator }, "light-line", drawOptions("light-line", denominator, false, "en"));
    for (const mode of MODE_LIST) {
      before = cpu();
      resolveViews(asset, query, { kind: "scale", denominator }, mode, drawOptions(mode, denominator, false, "en"));
      say(`render ${mode}`, cpu() - before);
    }
  }
  if (only.includes("derived")) {
    before = cpu();
    derivedFindings(asset, scale, denominator);
    say("derived", cpu() - before);
  }
}
