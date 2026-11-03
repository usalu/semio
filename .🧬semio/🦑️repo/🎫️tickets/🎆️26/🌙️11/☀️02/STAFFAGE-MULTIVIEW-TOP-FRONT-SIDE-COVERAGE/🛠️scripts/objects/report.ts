import { readFileSync, writeFileSync } from "node:fs";
const d = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const out = process.argv[3]!;
const notes: Record<string, { artic: string; support: string; asym: string; emit: string; symbols: string }> = {
  "bicycle-stand": { artic: "pose-only (shade/lean), rigid hoop; clips move no geometry", support: "two leg feet in plan and front", asym: "symmetric hoop", emit: "none", symbols: "plan-true (true-extent) 3 views; plan-mark (nominal sign) 3 views" },
  chalkboard: { artic: "leaf slides 2 m (dx): elevation and plan follow, front end-on leaf follows", support: "wall hooks in the frame", asym: "leaf starts left; leaf strip on one face", emit: "none", symbols: "none" },
  "charcoal-grill": { artic: "lid hinge 0..115 deg, lid vent 0..90, ash damper 0..90: lid solids follow; lid-axis fix pending (see report)", support: "two legs and two wheels, wheel side asymmetric", asym: "handle on the lid, wheels on one side, side handles", emit: "vent-top (flue dir fix pending), bowl-mouth", symbols: "none" },
  "cooking-pot": { artic: "no articulation (thermal poses)", support: "base ring", asym: "two side handles along x", emit: "steam-top above the pot", symbols: "none" },
  "drinking-fountain": { artic: "press button (drinking pose), jet", support: "pedestal base fixed", asym: "nozzle and button on one side", emit: "nozzle", symbols: "none" },
  "market-parasol": { artic: "opening of 8 ribs (abduction), tilt (mast), furling (runner): revolved canopy about the crown", support: "concrete base block (hidden under canopy in plan)", asym: "tilt direction forward", emit: "none", symbols: "none" },
  "patio-heater": { artic: "knob; pose-only", support: "base plate hidden under the dome in plan", asym: "knob and door on one side", emit: "flue-vent, reflector-top", symbols: "none" },
  "playground-swing": { artic: "pendulum of the seat on both chains, front/plan follow", support: "two A-frame feet pairs", asym: "symmetric", emit: "none", symbols: "none" },
  "retractable-bollard": { artic: "shaft rises/lowers, nudged", support: "sleeve and frame ring in ground", asym: "power unit on the left below grade", emit: "none", symbols: "plan-true (true-extent) 3 views; plan-dot (true-position sign) 3 views" },
  "shopping-trolley": { artic: "gate raised, load", support: "four castors, two fixed and two swivel", asym: "handle at rear", emit: "none", symbols: "none" },
  stepladder: { artic: "spread legs, load test", support: "four feet", asym: "front/rear frames differ", emit: "none", symbols: "none" },
  "storage-crate-stack": { artic: "crates sink into each other (dy)", support: "bottom crate base", asym: "symmetric", emit: "none", symbols: "none" },
  "street-lamp": { artic: "pole sway, lamp dimming", support: "foundation", asym: "arm to one side (+x)", emit: "lamp-plume", symbols: "none" },
  tealight: { artic: "flame flicker", support: "cup base", asym: "symmetric", emit: "flame-top", symbols: "none" },
  "waste-bin": { artic: "lid opens", support: "two feet pairs", asym: "lid hinge at the back", emit: "none", symbols: "none" },
  "bar-stool": { artic: "swivel twist (3 poses, clip), seat pad press", support: "three feet ring", asym: "backrest on one side, rotates with twist", emit: "none", symbols: "stool-plan 3 views" },
  bookshelf: { artic: "shelf pin adjust (dy), load", support: "two plinths", asym: "back panel on the left (+z)", emit: "none", symbols: "shelf-plan 3 views" },
  "cafe-chair": { artic: "tilt on two legs, pull out", support: "four legs, tilted pose rests on rear legs", asym: "back on one end (-x)", emit: "none", symbols: "none" },
  "dining-table": { artic: "extend (leaves slide), set", support: "four legs", asym: "extension leaf at centre", emit: "none", symbols: "table-plan 3 views" },
  "folding-deck-chair": { artic: "fold/unfold hinges", support: "four feet", asym: "faces +x", emit: "none", symbols: "chair-plan 3 views" },
  "height-adjustable-desk": { artic: "column and stage telescoping", support: "two feet pairs", asym: "monitor and tray on one side", emit: "none", symbols: "none" },
  "office-swivel-chair": { artic: "swivel twist (2 poses, clip), height, recline", support: "five castors", asym: "backrest rotates with twist", emit: "none", symbols: "none" },
  "operating-table": { artic: "table top tilt (12 deg), lamp on", support: "table base and stand base castors", asym: "lamp stand on +x side", emit: "light-top", symbols: "none" },
  "park-bench": { artic: "occupancy only", support: "two cast ends", asym: "back at +z, seat faces -z", emit: "none", symbols: "none" },
  "picnic-table": { artic: "benches fold (stowed, half-folded)", support: "two trestles", asym: "symmetric", emit: "none", symbols: "picnic-plan 3 views" },
  "single-bed": { artic: "head section raise (slat-head)", support: "legs", asym: "head at x=0, head board taller", emit: "none", symbols: "bed-plan 3 views" },
  "three-seat-sofa": { artic: "cushion compression (dy)", support: "four feet", asym: "back at +z, seats face -z", emit: "none", symbols: "sofa-plan 3 views" },
};
let md = "# sonnet-objects per-asset table (objects 15 + furniture 12)\n\nProduced from `T/🛠️scripts/objects/audit.ts` (every pose and clip sample of every authored band, three views, extents versus dimensions) and manual proof renders. Counts are shapes elevation/front/plan.\n\n| Asset | Dim H x W x D | Band 1:500 | 1:200 | 1:100 | 1:50 | View-extents vs dims (default pose, 1:50) | Cross-view mismatches over all poses/clips | Articulation | Support | Asymmetry | Emitters | Symbols |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|\n";
for (const [slug, r] of Object.entries<any>(d)) {
  const n = notes[slug]!;
  const band = (k: string) => {
    const s = r.scales[k];
    if (!s) return "-";
    if (s.status) return `unavailable (${s.alt.map((a: any) => a.denominator).join("/")})`;
    const c = s.counts;
    return `${c.elevation}/${c.front}/${c.plan}`;
  };
  const s50 = r.scales["50"];
  const ext = s50?.dim?.e ? `E ${s50.dim.e.join("x")} F ${s50.dim.f.join("x")} P ${s50.dim.p.join("x")}` : "n/a";
  const bad = Object.entries<any>(r.scales).filter(([, s]) => !s.status && !String(s.mism.bad).startsWith("0/")).map(([k, s]) => `1:${k} ${s.mism.bad}`).join(", ") || "none";
  md += `| ${slug} | ${r.dims.heightM} x ${r.dims.widthM} x ${r.dims.depthM} | ${band("500")} | ${band("200")} | ${band("100")} | ${band("50")} | ${ext} | ${bad} | ${n.artic} | ${n.support} | ${n.asym} | ${n.emit} | ${n.symbols} |\n`;
}
writeFileSync(out, md);
console.log("wrote", out);
