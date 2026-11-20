/** 🔬️ Round 3 red squirrel: prints the root and the anchors of a clip over time (planted root, ground) or of every pose, from the product's `evaluate`. Usage: bun r3-red-squirrel-probe.ts [--file doc.json] clip <id> [steps] | poses */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const args = process.argv.slice(2);
const fileAt = args.indexOf("--file");
const asset: any = fileAt >= 0 ? JSON.parse(readFileSync(args[fileAt + 1]!, "utf8")).asset : (() => { const { entry, asset } = entryOf("animals/red-squirrel"); return merged(asset, loadSource(entry)!); })();
if (fileAt >= 0) args.splice(fileAt, 2);
const f = (n: number): string => (n >= 0 ? "+" : "") + n.toFixed(4);
const line = (state: any, label: string): void => {
  const root = state.root;
  const an = asset.anchors.filter((a: any) => a.kind === "contact").map((a: any) => { const p = state.anchors[a.id].at; return `${a.id.replace("ground-", "g-").replace("bark-", "b-")} ${f(p.x)} ${f(p.y)}`; });
  console.log(label, "root", f(root.x), f(root.y), "|", an.join(" | "), "| contacts", (state.contacts ?? []).join(","));
};
if (args[0] === "keys") { const s: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose } as never); console.log(Object.keys(s), JSON.stringify(s.anchors).slice(0, 500), JSON.stringify(s.root)); } else if (args[0] === "clip") {
  const clip = asset.clips.find((c: any) => c.id === args[1]);
  const n = Number(args[2] ?? 12);
  for (let i = 0; i <= n; i++) { const t = (clip.durationS * i) / n; line(evaluate(asset, { kind: "clip", clip: clip.id, timeS: t } as never), `t ${t.toFixed(3)}`); }
} else for (const p of asset.poses) line(evaluate(asset, { kind: "pose", pose: p.id } as never), p.id.padEnd(16));
