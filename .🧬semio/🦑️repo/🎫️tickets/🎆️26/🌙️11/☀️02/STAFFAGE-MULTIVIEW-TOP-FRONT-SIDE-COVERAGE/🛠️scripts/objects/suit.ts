const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { resolveRepresentation } = await import(`${L}/🔨️modules/🖨️representation/🟦️.ts`);
const { PRINT_PROFILES } = await import(`${L}/🔨️modules/🖨️representation/🟦️.ts`).catch(() => ({ PRINT_PROFILES: undefined }));
const { readdirSync, readFileSync } = await import("node:fs");
const only = process.argv.slice(2);
const profiles = PRINT_PROFILES?.profiles ?? [];
for (const cat of ["🪑️objects", "🛋️furniture"])
  for (const slug of readdirSync(`${L}/🗂️catalog/${cat}`)) {
    if (only.length && !only.includes(slug)) continue;
    const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
    const out: string[] = [];
    for (const k of ["500", "200", "100", "50"]) {
      for (const p of profiles.length ? profiles : [undefined]) {
        const r = resolveRepresentation(a, { denominator: +k }, p);
        if (r.kind === "unsuitable") out.push(`1:${k}/${p?.id} UNSUITABLE ${r.suitability.violations.slice(0, 3).map((v: any) => `${v.check} ${v.shapes.slice(0, 2).join("+")} ${v.measuredMm.toFixed(3)}<${v.requiredMm}`).join("; ")}`);
        else if (r.kind === "unavailable" && a.representations[k]?.status === "authored") out.push(`1:${k}/${p?.id} UNAVAILABLE`);
      }
    }
    console.log(slug.padEnd(24), out.length ? out.join(" | ") : "all suitable");
  }
