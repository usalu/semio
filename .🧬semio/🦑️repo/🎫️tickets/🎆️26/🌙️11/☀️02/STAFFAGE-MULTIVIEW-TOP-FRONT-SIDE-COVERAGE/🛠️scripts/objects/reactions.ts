const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { solveStructure } = await import(`${L}/🔨️modules/🏗️structure/🟦️.ts`);
const { readFileSync, existsSync } = await import("node:fs");
const [slug, cat] = process.argv.slice(2);
const head = `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE/🗑️generated/objects/head/${slug}.json`;
for (const [label, file] of [["HEAD", head], ["NOW", `${L}/🗂️catalog/${cat}/${slug}/🔣️.json`]] as const) {
  const a = JSON.parse(readFileSync(file, "utf8")).asset;
  console.log(label, slug);
  for (const p of a.poses) {
    let r: any;
    try { r = solveStructure(evaluate(a, { kind: "pose", pose: p.id }, { kind: "scale", scale: { denominator: 50 } })); } catch (e) { console.log("  ", p.id, "ERR", (e as Error).message.slice(0, 80)); continue; }
    const resid = Math.max(...Object.values(r.residual).map((v: any) => Math.abs(v)));
    console.log("  ", p.id.padEnd(24), "resid", resid.toExponential(1), "fy", r.reactions.map((x: any) => `${x.anchor}:${x.fyN.toFixed(0)}`).join(" "));
  }
}
