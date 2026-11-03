import { writeFileSync } from "node:fs";
import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { solveChain } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const out = process.argv[2]!;
const lines: string[] = [];
const asset = catalog().find((a) => a.id === "spiral-stair")!;
const r = (p: { x: number; y: number }): string => `(${p.x.toFixed(3)}, ${p.y.toFixed(3)})`;
const subjects: [string, object][] = [
  ...["self-weight", "walker-on-tread-8", "point-load-top-tread"].map((pose): [string, object] => [pose, { kind: "pose", pose }]),
  ...[0, 1, 3.5, 7, 7.5, 8, 10, 14].map((time): [string, object] => ["climb@" + time, { kind: "clip", clip: "climb", timeS: time }]),
];
for (const [name, subject] of subjects) {
  const s = evaluate(asset, subject as never, { kind: "scale", scale: { denominator: 50 } });
  const b = s.bones;
  const offsets = Object.fromEntries(Object.entries(b).map(([id, e]) => [id, (e as { offset: object }).offset]));
  const ch = solveChain(asset.rig, offsets as never, { x: 0, y: 0, angleDeg: 0 });
  lines.push(`  chain walker-y origin x ${ch["walker-y"]!.frame.o[0].toFixed(3)} lateral z ${ch["walker-y"]!.originZ.toFixed(3)}`);
  lines.push(`${name}: walker-x origin ${r(b["walker-x"]!.origin)} walker-y origin ${r(b["walker-y"]!.origin)}`);
}
writeFileSync(out, lines.join("\n") + "\n", "utf8");
