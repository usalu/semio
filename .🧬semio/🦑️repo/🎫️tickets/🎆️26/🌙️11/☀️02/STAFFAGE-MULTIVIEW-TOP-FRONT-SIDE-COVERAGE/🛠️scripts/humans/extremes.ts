import { readdirSync, readFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { validateAsset } = await import(`${L}/🔨️modules/🗂️catalog/🟦️.ts`);
const only = process.argv[2];
const dir = `${L}/🗂️catalog/👤️humans`;
for (const slug of readdirSync(dir).sort()) {
  if (only && only !== "all" && slug !== only) continue;
  const a = JSON.parse(readFileSync(`${dir}/${slug}/🔣️.json`, "utf8")).asset;
  const issues = validateAsset(a).filter((i: any) => i.code.startsWith("extent-"));
  for (const i of issues) {
    const m = /representations\.(\d+)\.geometry\.(\w+)/.exec(i.path)!;
    const [, k, view] = m; const code = i.code.replace("extent-", "");
    const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: +k } });
    const axis = code === "front-depth" ? "x" : code === "front-height" ? "y" : code === "plan-width" ? "x" : "y";
    const d = a.dimensions; const nominal = { "front-depth": d.depthM, "front-height": d.heightM, "plan-width": d.widthM, "plan-depth": d.depthM }[code];
    const rows = st.shapes[view].filter((s: any) => s.shape.role !== "ground").map((s: any) => { const p = s.rings.flat().map((q: any) => q[axis]); return { id: s.shape.id, bone: s.shape.bone, kind: s.shape.kind, solid: s.shape.solid?.kind, lo: Math.min(...p), hi: Math.max(...p) }; });
    const lo = Math.min(...rows.map((r: any) => r.lo)), hi = Math.max(...rows.map((r: any) => r.hi));
    const near = (v: number, key: "lo" | "hi") => rows.filter((r: any) => Math.abs(r[key] - v) < 0.004).map((r: any) => `${r.id}(${r.kind}${r.solid ? "/" + r.solid : ""})`).join(",");
    console.log(`${slug} ${k} ${code} nominal ${nominal} measured ${(hi - lo).toFixed(3)} lo ${lo.toFixed(3)} [${near(lo, "lo")}] hi ${hi.toFixed(3)} [${near(hi, "hi")}]`);
  }
}
