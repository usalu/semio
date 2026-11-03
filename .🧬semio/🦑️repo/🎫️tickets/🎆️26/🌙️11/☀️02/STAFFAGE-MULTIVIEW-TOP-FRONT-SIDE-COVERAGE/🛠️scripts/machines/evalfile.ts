import { readFileSync } from "node:fs";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [file, pose, scale] = process.argv.slice(2);
const a = JSON.parse(readFileSync(file, "utf8")).asset;
try { const st: any = evaluate(a, { kind: "pose", pose }, { kind: "scale", scale: { denominator: Number(scale) } } as any); console.log("ok", Object.values(st.shapes).map((v: any) => v.length).join("/")); } catch (e) { console.log("FAIL", (e as Error).message.slice(0, 120)); }
