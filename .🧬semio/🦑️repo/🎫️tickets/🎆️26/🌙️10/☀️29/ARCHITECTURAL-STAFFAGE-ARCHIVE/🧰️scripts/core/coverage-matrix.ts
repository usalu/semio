import { appendFileSync, writeFileSync } from "node:fs";
import { parseArgs } from "node:util";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const { loadCatalogFromFiles } = await import(`${S}/🗂️catalog/📥️files/🟦️.ts`);
const { evaluate } = await import(`${S}/🧮️evaluation/🟦️.ts`);
const { defaultDepictionOptions, depict } = await import(`${S}/🎨️depiction/🟦️.ts`);
const { modelScene } = await import(`${S}/🎨️depiction/🟦️.ts`);
const { DEFAULT_PRINT_PROFILE, resolveRepresentation } = await import(`${S}/🖨️representation/🟦️.ts`);
const { solveThermal } = await import(`${S}/🌡️thermal/🟦️.ts`);
const { solveStructure } = await import(`${S}/🏗️structure/🟦️.ts`);
const { solveTransport } = await import(`${S}/💨️transport/🟦️.ts`);
const { writeSvg } = await import(`${S}/📤️export/🖼️svg/🟦️.ts`);
const { writePdf } = await import(`${S}/📤️export/📕️pdf/🟦️.ts`);
const { writeDxf } = await import(`${S}/📤️export/📐️dxf/🟦️.ts`);
const { VIEWS } = await import(`${S}/🔗️contracts/🟦️.ts`);

const { values } = parseArgs({ options: { md: { type: "string" }, csv: { type: "string" }, only: { type: "string" }, transport: { type: "boolean", default: true } } });
const catalog = await loadCatalogFromFiles();
const only = values.only ? new Set(values.only.split(",")) : null;
const UNCHECKED = { ...DEFAULT_PRINT_PROFILE, id: "unchecked", minStrokeMm: 0, minGapMm: 0, minFeatureMm: 0, minHatchSpacingMm: 0 };
const csvHead = "asset,scale,query,availability,view,thermal,transport_emitters,transport_overlay,svg,pdf,dxf\n";
writeFileSync(values.csv!, csvHead);
const err = (e: unknown): string => `fail:${String((e as Error).message ?? e).replace(/[\n,"]/g, " ").slice(0, 100)}`;
type Row = { asset: string; scale: number; query: string; availability: string; view: string; thermal: string; emitters: number; overlay: string; svg: string; pdf: string; dxf: string };
const all: Row[] = [];
const summary: string[] = [];
const failures: string[] = [];

for (const asset of catalog.assets) {
  if (only && !only.has(asset.id)) continue;
  const t0 = Date.now();
  const queries = [
    ...asset.poses.filter((p: any) => !p.key).map((p: any) => ({ label: `pose:${p.id}`, query: { kind: "pose", pose: p.id } })),
    ...asset.clips.map((c: any) => ({ label: `clip:${c.id}@mid`, query: { kind: "clip", clip: c.id, timeS: c.durationS / 2 } })),
  ];
  const emitters = asset.thermal.emitters.length;
  let overlayDone = false;
  for (const { label, query } of queries) {
    let thermal: any = null;
    let structure: any = null;
    let thermalErr = "";
    try {
      const physical = evaluate(asset, query as any);
      thermal = asset.thermal.regions.length > 0 ? solveThermal(physical) : null;
      structure = solveStructure(physical);
    } catch (e) { thermalErr = err(e); }
    let transport: any = null;
    let overlay = emitters === 0 ? "n/a" : "skipped";
    for (const d of [500, 200, 100, 50]) {
      const resolution = resolveRepresentation(asset, { denominator: d }, DEFAULT_PRINT_PROFILE);
      const availability = resolution.kind === "authored" ? "authored" : resolution.kind;
      if (resolution.kind === "unavailable") {
        all.push({ asset: asset.id, scale: d, query: label, availability, view: "-", thermal: "-", emitters, overlay: "-", svg: "-", pdf: "-", dxf: "-" });
        continue;
      }
      let state: any;
      let dxf = "";
      try {
        state = evaluate(asset, query as any, { kind: "scale", scale: { denominator: d } });
        dxf = writeDxf(modelScene(state, { viewpoint: "elevation" })).length > 0 ? "ok" : "fail:empty";
      } catch (e) { dxf = err(e); }
      for (const view of VIEWS) {
        let svg = "ok";
        let pdf = "ok";
        let th = view === "thermal" ? (thermal ? "ok" : asset.thermal.regions.length === 0 ? "none" : thermalErr || "fail") : "-";
        let ov = "-";
        try {
          if (!state) throw new Error("no state");
          const options = { ...defaultDepictionOptions(view, "en", d), profile: UNCHECKED };
          const analysis = { ...(thermal ? { thermal } : {}), ...(structure ? { structure } : {}) };
          const scene = depict(state, options, analysis);
          try { if (writeSvg(scene).length === 0) svg = "fail:empty"; } catch (e) { svg = err(e); }
          try { if (writePdf([scene]).length === 0) pdf = "fail:empty"; } catch (e) { pdf = err(e); }
          if (values.transport && emitters > 0 && view === "light-line" && !overlayDone && label.startsWith("pose:")) {
            try {
              transport ??= solveTransport(asset, query as any, asset.thermal.environment);
              depict(state, { ...options, transport: ["parcels", "streamlines"] }, { ...analysis, transport });
              ov = overlay = "ok";
              overlayDone = true;
            } catch (e) { ov = overlay = err(e); }
          }
        } catch (e) { svg = pdf = err(e); }
        all.push({ asset: asset.id, scale: d, query: label, availability, view, thermal: th, emitters, overlay: ov, svg, pdf, dxf });
      }
    }
  }
  const mine = all.filter((r) => r.asset === asset.id);
  const bad = mine.filter((r) => [r.svg, r.pdf, r.dxf, r.thermal, r.overlay].some((v) => v.startsWith("fail")));
  const count = (f: (r: Row) => boolean): number => mine.filter(f).length;
  const cell = (d: number): string => {
    const rs = mine.filter((r) => r.scale === d);
    return `${rs[0]?.availability ?? "?"}${rs[0]?.availability === "unavailable" ? "" : ` ${rs.filter((r) => r.svg === "ok" && r.pdf === "ok").length}/${rs.length}`}`;
  };
  const line = `| ${asset.id} | ${cell(500)} | ${cell(200)} | ${cell(100)} | ${cell(50)} | ${asset.thermal.regions.length ? "yes" : "no"} | ${emitters} | ${mine.some((r) => r.overlay === "ok") ? "ok" : emitters ? "n/a-or-fail" : "n/a"} | ${count((r) => r.dxf.startsWith("fail")) } | ${bad.length} |`;
  summary.push(line);
  for (const r of bad.slice(0, 3)) failures.push(`- ${r.asset} 1:${r.scale} ${r.query} ${r.view}: svg=${r.svg} pdf=${r.pdf} dxf=${r.dxf} thermal=${r.thermal} overlay=${r.overlay}`);
  appendFileSync(values.csv!, mine.map((r) => [r.asset, r.scale, r.query, r.availability, r.view, r.thermal, r.emitters, r.overlay, r.svg, r.pdf, r.dxf].join(",")).join("\n") + "\n");
  console.log(`[DEBUG] ${asset.id} ${((Date.now() - t0) / 1000).toFixed(1)}s rows=${mine.length} bad=${bad.length}`);
}
const total = all.length;
const ok = all.filter((r) => ![r.svg, r.pdf, r.dxf, r.thermal, r.overlay].some((v) => v.startsWith("fail"))).length;
writeFileSync(values.md!, `# Coverage matrix\n\nEvery asset × preset scale × view × (named pose | clip at mid-time). ${total} rows (full detail in the CSV \`🗑️generated/core/coverage-matrix.csv\`), ${ok} without failures. Unavailable scales are decisions of the asset (no export). `unsuitable` = the authored drawing violates office-laser (see check-v2-results.md); it is exported here with an unchecked profile to prove the pipeline, the UI refuses it under office-laser. "svg/pdf" counts are rows (7 views × queries) where both exports succeeded / rows. Export formats: svg, pdf from \`depict\`; dxf from \`modelScene\`. Transport overlay is drawn once per asset on the default pose.\n\n| asset | 1:500 | 1:200 | 1:100 | 1:50 | thermal regions | emitters | transport overlay | dxf failures | failing rows |\n|---|---|---|---|---|---|---|---|---|---|\n${summary.join("\n")}\n\n## First failures per asset\n\n${failures.join("\n") || "none"}\n`);
