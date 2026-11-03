import { L, load, save } from "./lib.ts";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const ext = (rings: any[], key: "x" | "y") => { const v = rings.flatMap((s: any) => s.rings.flat().map((q: any) => q[key])); return [Math.min(...v), Math.max(...v)] as const; };
const body = (shapes: any[]) => shapes.filter((s) => s.shape.role !== "ground");

export function matchForward(slug: string, scales?: number[], tol = 0.004): void {
  const doc = load(slug); const a = doc.asset;
  const measure = (k: number) => {
    const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: k } });
    return { e: ext(body(st.shapes.elevation), "x"), p: ext(body(st.shapes.plan), "x"), st };
  };
  for (const rep of Object.values<any>(a.representations)) {
    if (rep.status !== "authored" || (scales && !scales.includes(rep.denominator))) continue;
    for (const edge of [1, 0] as const) {
      for (let it = 0; it < 3; it++) {
        const m = measure(rep.denominator);
        const diff = m.p[edge] - m.e[edge];
        if (Math.abs(diff) <= tol) break;
        const ids = body(m.st.shapes.plan).filter((s: any) => Math.abs(s.rings.flat().map((q: any) => q.x).reduce((u: number, v: number) => (edge ? Math.max(u, v) : Math.min(u, v)), edge ? -1e9 : 1e9) - m.p[edge]) < 0.002).map((s: any) => s.shape.id);
        for (const id of ids) {
          const s = rep.geometry.plan.find((x: any) => x.id === id); if (!s) continue;
          const r = (n: number) => +n.toFixed(5);
          if (s.kind === "capsule") {
            const before = Math.abs(diff);
            let changed = false;
            for (const [f0, f1] of [["x1", "x0"], ["x0", "x1"]] as const) {
              const save0 = s[f0]; s[f0] = r(s[f0] - diff);
              const m2 = measure(rep.denominator);
              if (Math.abs(m2.p[edge] - m2.e[edge]) < before - 0.0005) { changed = true; console.log(slug, rep.denominator, edge ? "hi" : "lo", id, f0, `${-diff > 0 ? "+" : ""}${(-diff).toFixed(3)}`); break; }
              s[f0] = save0; void f1;
            }
            if (!changed) console.log(slug, rep.denominator, id, "capsule unchanged");
          } else if (s.kind === "poly") { console.log(slug, rep.denominator, id, "poly needs manual", (-diff).toFixed(3)); }
          else if (s.kind === "ellipse") { s.cx = r(s.cx - diff / 2); s.rx = r(s.rx + (edge ? -diff : diff) / 2); console.log(slug, rep.denominator, id, "ellipse", edge ? "hi" : "lo"); }
          else if (s.kind === "rect") { console.log(slug, rep.denominator, id, "rect needs manual"); }
        }
      }
    }
  }
  save(slug, doc);
}
if (import.meta.main) matchForward(process.argv[2]!, process.argv[3] ? process.argv[3].split(",").map(Number) : undefined);
