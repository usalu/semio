import { openCtx } from "./kit-lib.ts";
const [spec, scaleS] = process.argv.slice(2);
const c = openCtx(spec!, Number(scaleS));
const seen = new Set<string>();
for (const v of ["elevation", "front", "plan"] as const) for (const s of c.state[v] as any[]) {
  const k = `${s.shape.id}|${s.shape.bone}|${s.shape.region ?? (s.shape.regions ?? []).join("+")}|${s.shape.role}`;
  if (!seen.has(k)) { seen.add(k); }
}
console.log([...seen].join("\n"));
