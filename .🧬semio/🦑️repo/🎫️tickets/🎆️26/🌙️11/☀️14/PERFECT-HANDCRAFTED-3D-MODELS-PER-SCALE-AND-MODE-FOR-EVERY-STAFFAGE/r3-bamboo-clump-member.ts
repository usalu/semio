/** 📐️ Prints, for one structure member of the bamboo clump at a scale, the world box of its mesh, the strokes the product draws it and its body culm as in a view, and how far the member stroke corners lie from the body strokes. Usage: `bun r3-bamboo-clump-member.ts <scale> <member id> <body id> <view>` */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { strokesOf, worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { load } from "./r3-lib.ts";

const [scale, memberId, bodyId, view] = [Number(process.argv[2]), process.argv[3]!, process.argv[4]!, process.argv[5] as "front"];
const { asset } = load("vegetation/bamboo-clump", "r3-bamboo-clump-member.ts <scale> <member id> <body id> <view>");
const state = evaluate(asset as never, { kind: "pose", pose: asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: scale } } as never) as never as { bones: never };
const lists = worldSolids as never as (a: unknown, b: unknown, s: number, list?: string) => { id: string; positions: Float64Array }[];
const box = (w: { positions: Float64Array }) => [0, 1, 2].map((k) => { const v = Array.from({ length: w.positions.length / 3 }, (_, i) => w.positions[i * 3 + k]!); return [Math.min(...v).toFixed(4), Math.max(...v).toFixed(4)].join(".."); }).join("  ");
const body = lists(asset, state.bones, scale).find((w) => w.id === bodyId)!;
const member = lists(asset, state.bones, scale, "structure").find((w) => w.id === memberId);
console.log("body  ", box(body));
if (!member) throw new Error(`no world solid ${memberId} in the structure list (does worldSolids take a list?)`);
console.log("member", box(member));
const [sb, sm] = [strokesOf(view, body as never), strokesOf(view, member as never)];
console.log("body strokes", sb.length, "member strokes", sm.length);
const fmt = (r: readonly { x: number; y: number }[]) => r.map((p) => `(${p.x.toFixed(4)}, ${p.y.toFixed(4)})`).join(" ");
for (const r of sm.slice(0, 4)) console.log("member stroke", fmt(r));
for (const r of [...sb.slice(0, 2), ...sb.slice(-2)]) console.log("body stroke  ", fmt(r));
