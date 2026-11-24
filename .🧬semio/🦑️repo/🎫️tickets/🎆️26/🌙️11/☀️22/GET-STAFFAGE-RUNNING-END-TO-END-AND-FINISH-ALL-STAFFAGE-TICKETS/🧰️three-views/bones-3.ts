/** 🦴️ Names, for every instant of an asset that the three-views sweep reports, the bones whose drawings disagree between the views: per bone the forward range of side elevation and plan and the height range of side elevation and front, with the shapes of the bone in the view that is out of step.
 *
 * Usage: `bun bones-3.ts <id> [--file <asset document json>] [--scale <n>] [--min <metres, default 0.02>] [--top <n, default 6>] [--all]` (`--all` lists every instant, not only the reported ones)
 * @see ./sweep-2.ts */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { VIEWS, authoredScales, catalog, extentOf, queries, sweep } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

if (import.meta.main) {
  const args = process.argv.slice(2);
  const option = (name: string): string | undefined => (args.indexOf(name) < 0 ? undefined : args[args.indexOf(name) + 1]);
  const id = args[0]!;
  const file = option("--file");
  const min = Number(option("--min") ?? 0.02);
  const top = Number(option("--top") ?? 6);
  const asset = file === undefined ? catalog().find((candidate) => candidate.id === id)! : (JSON.parse(readFileSync(file, "utf8")) as { asset: ReturnType<typeof catalog>[number] }).asset;
  for (const scale of authoredScales(asset).filter((s) => option("--scale") === undefined || s === Number(option("--scale")))) {
    const reported = sweep(asset, scale).map((finding) => finding.message);
    for (const { name, query } of queries(asset)) {
      if (!args.includes("--all") && !reported.some((message) => message.includes(` ${name} `) || message.includes(` ${name}:`))) continue;
      const state = evaluate(asset, query, { kind: "scale", scale: { denominator: scale } });
      const bones = new Set(VIEWS.flatMap((view) => state.shapes[view].map((evaluated) => evaluated.shape.bone)));
      const rows: { delta: number; line: string }[] = [];
      for (const bone of bones) {
        const of = (view: (typeof VIEWS)[number]) => state.shapes[view].filter((evaluated) => evaluated.shape.bone === bone && evaluated.shape.role !== "ground");
        const [e, f, p] = [extentOf(of("elevation")), extentOf(of("front")), extentOf(of("plan"))];
        const names = (view: (typeof VIEWS)[number]): string => of(view).map((evaluated) => `${evaluated.shape.id}:${evaluated.shape.kind}${"solid" in evaluated.shape && evaluated.shape.solid !== undefined ? `/${(evaluated.shape.solid as { kind: string }).kind}` : ""}`).join(" ");
        if (e === undefined) { rows.push({ delta: 9, line: `    ${bone}: no elevation shape; front ${names("front")}; plan ${names("plan")}` }); continue; }
        const dx = p === undefined ? 0 : Math.max(Math.abs(e.minX - p.minX), Math.abs(e.maxX - p.maxX));
        const dy = f === undefined ? 0 : Math.max(Math.abs(e.minY - f.minY), Math.abs(e.maxY - f.maxY));
        if (Math.max(dx, dy) < min) continue;
        const r = (a: number, b: number): string => `[${a.toFixed(3)}, ${b.toFixed(3)}]`;
        rows.push({ delta: Math.max(dx, dy), line: `    ${bone}: ${dx >= min && p !== undefined ? `x side ${r(e.minX, e.maxX)} plan ${r(p.minX, p.maxX)} (${names("plan")}) ` : ""}${dy >= min && f !== undefined ? `y side ${r(e.minY, e.maxY)} front ${r(f.minY, f.maxY)} (${names("front")})` : ""}` });
      }
      console.log(`  1:${scale} ${name}`);
      for (const row of rows.sort((a, b) => b.delta - a.delta).slice(0, top)) console.log(row.line);
    }
  }
}
