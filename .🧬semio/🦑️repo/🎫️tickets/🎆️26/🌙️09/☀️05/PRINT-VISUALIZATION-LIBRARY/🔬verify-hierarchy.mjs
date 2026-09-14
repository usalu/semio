// 🔬 Local d3-hierarchy verification of the HIERARCHY probe fixtures, used while the
// repo test platform is being bootstrapped. Compile the five fixtures of
// 🧰️framework/🛍️products/📓️print/🧪️tests/hierarchy-*/🧫️fixtures/ with xelatex into one
// directory, then run:  node 🔬verify-hierarchy.mjs <directory>
// It is the same comparison the committed adapters make, with the same fixtures,
// the same projections and the same 1e-6 tolerance.
import * as d3 from "d3-hierarchy";
import fs from "fs";


// mirrors semio-viz-hierarchy.sty %region DemoData and semio-viz-data.sty
const DEEP = [
  ["root", "", 0], ["a", "root", 0], ["b", "root", 0], ["c", "root", 0],
  ["a1", "a", 6], ["a2", "a", 3], ["a3", "a", 2],
  ["b1", "b", 8], ["b2", "b", 4],
  ["c1", "c", 5], ["c2", "c", 1], ["c3", "c", 7], ["c4", "c", 2],
];
const UNB = [
  ["r", "", 0], ["s", "r", 0], ["t", "r", 9],
  ["sa", "s", 0], ["sb", "s", 4],
  ["saa", "sa", 0], ["sab", "sa", 3],
  ["saaa", "saa", 2], ["saab", "saa", 5], ["saac", "saa", 1],
];

const make = (rows) =>
  d3.stratify().id((d) => d[0]).parentId((d) => d[1] || null)(rows).sum((d) => +d[2] || 0);
const pre = (root) => { const o = []; root.eachBefore((n) => o.push(n)); return o; };

const round = (v) => (typeof v === "string" ? v : Math.abs(v) < 1e-6 ? 0 : Math.round(v * 1e6) / 1e6);

function load(file, scenario) {
  const out = {};
  for (const line of fs.readFileSync(file, "utf8").trim().split("\n")) {
    const r = JSON.parse(line);
    if (r.scenario !== scenario) continue;
    (out[r.key] ??= []).push(...r.values.map(round));
  }
  return out;
}

function compare(label, subject, oracle) {
  const keys = new Set([...Object.keys(subject), ...Object.keys(oracle)]);
  let bad = [];
  for (const k of keys) {
    const a = subject[k] ?? [], b = (oracle[k] ?? []).map(round);
    if (a.length !== b.length) { bad.push([k, "length", a.length, b.length]); continue; }
    for (let i = 0; i < a.length; i++) {
      if (typeof a[i] === "string" || typeof b[i] === "string") { if (a[i] !== b[i]) bad.push([k, i, a[i], b[i]]); continue; }
      if (Math.abs(a[i] - b[i]) > 1e-6 * Math.max(1, Math.abs(b[i]))) bad.push([k, i, a[i], b[i]]);
    }
  }
  console.log(`${label}: ${bad.length ? "FAIL " + JSON.stringify(bad.slice(0, 8)) : "ok (" + keys.size + " keys)"}`);
  return bad.length === 0;
}

const DIR = process.argv[2];
const F = `${DIR}/hierarchy-tree-cluster.probe.jsonl`;
let ok = true;


const sep = (same, other) => (a, b) => (a.parent === b.parent ? same : other);

function tree(rows, cfg) {
  const r = make(rows), t = d3.tree();
  if (cfg.nodeSize) t.nodeSize(cfg.nodeSize); else t.size(cfg.size);
  if (cfg.sep) t.separation(sep(...cfg.sep));
  t(r);
  return r;
}
function cluster(rows, cfg) {
  const r = make(rows), t = d3.cluster();
  if (cfg.nodeSize) t.nodeSize(cfg.nodeSize); else t.size(cfg.size);
  if (cfg.sep) t.separation(sep(...cfg.sep));
  t(r);
  return r;
}

{
  const a = tree(DEEP, { size: [100, 60] }), b = tree(UNB, { size: [100, 60] });
  ok = compare("tree-size", load(F, "tree-size"), {
    "id/deep": pre(a).map((n) => n.id), "x/deep": pre(a).map((n) => n.x), "y/deep": pre(a).map((n) => n.y),
    "id/unbalanced": pre(b).map((n) => n.id), "x/unbalanced": pre(b).map((n) => n.x), "y/unbalanced": pre(b).map((n) => n.y),
  }) && ok;
}
{
  const a = tree(DEEP, { nodeSize: [12, 20] }), b = tree(UNB, { nodeSize: [12, 20] });
  ok = compare("tree-node-size", load(F, "tree-node-size"), {
    "x/deep": pre(a).map((n) => n.x), "y/deep": pre(a).map((n) => n.y),
    "x/unbalanced": pre(b).map((n) => n.x), "y/unbalanced": pre(b).map((n) => n.y),
  }) && ok;
}
{
  const a = tree(DEEP, { size: [100, 60], sep: [1, 2.5] });
  const b = tree(UNB, { nodeSize: [10, 10], sep: [2, 3] });
  ok = compare("tree-separation", load(F, "tree-separation"), {
    "x/deep": pre(a).map((n) => n.x), "x/unbalanced": pre(b).map((n) => n.x),
  }) && ok;
}
{
  const a = cluster(DEEP, { size: [100, 60] }), b = cluster(UNB, { size: [100, 60] });
  ok = compare("cluster-size", load(F, "cluster-size"), {
    "id/deep": pre(a).map((n) => n.id), "x/deep": pre(a).map((n) => n.x), "y/deep": pre(a).map((n) => n.y),
    "id/unbalanced": pre(b).map((n) => n.id), "x/unbalanced": pre(b).map((n) => n.x), "y/unbalanced": pre(b).map((n) => n.y),
  }) && ok;
}
{
  const a = cluster(DEEP, { nodeSize: [12, 20] });
  const b = cluster(UNB, { nodeSize: [12, 20], sep: [1.5, 3] });
  ok = compare("cluster-node-size", load(F, "cluster-node-size"), {
    "x/deep": pre(a).map((n) => n.x), "y/deep": pre(a).map((n) => n.y),
    "x/unbalanced": pre(b).map((n) => n.x), "y/unbalanced": pre(b).map((n) => n.y),
  }) && ok;
}



const rects = (root) => ({ x0: pre(root).map(n => n.x0), y0: pre(root).map(n => n.y0), x1: pre(root).map(n => n.x1), y1: pre(root).map(n => n.y1) });
const spread = (prefix, root) => Object.fromEntries(Object.entries(rects(root)).map(([k, v]) => [`${prefix}/${k}`, v]));
const tm = (rows, cfg) => {
  const r = make(rows), t = d3.treemap().size(cfg.size);
  if (cfg.tile) t.tile(cfg.tile);
  if (cfg.padding !== undefined) t.padding(cfg.padding);
  if (cfg.inner !== undefined) t.paddingInner(cfg.inner);
  if (cfg.top !== undefined) t.paddingTop(cfg.top).paddingRight(cfg.right).paddingBottom(cfg.bottom).paddingLeft(cfg.left);
  if (cfg.round) t.round(true);
  t(r); return r;
};

// ---- treemap ------------------------------------------------------------
{
  const F = `${DIR}/hierarchy-treemap.probe.jsonl`;
  ok = compare("treemap/tiling", load(F, "tiling"), {
    id: pre(make(DEEP)).map(n => n.id),
    ...spread("squarify", tm(DEEP, { size: [100, 60], tile: d3.treemapSquarify })),
    ...spread("resquarify", tm(DEEP, { size: [100, 60], tile: d3.treemapResquarify })),
    ...spread("slice", tm(DEEP, { size: [100, 60], tile: d3.treemapSlice })),
    ...spread("dice", tm(DEEP, { size: [100, 60], tile: d3.treemapDice })),
    ...spread("slice-dice", tm(DEEP, { size: [100, 60], tile: d3.treemapSliceDice })),
    ...spread("binary", tm(DEEP, { size: [100, 60], tile: d3.treemapBinary })),
  }) && ok;
  ok = compare("treemap/padding", load(F, "padding"), {
    ...spread("padding", tm(DEEP, { size: [100, 60], padding: 2 })),
    ...spread("nested", tm(DEEP, { size: [100, 60], inner: 1.5, top: 6, right: 1, bottom: 1, left: 1 })),
  }) && ok;
  ok = compare("treemap/ratio-and-round", load(F, "ratio-and-round"), {
    ...spread("ratio", tm(DEEP, { size: [100, 60], tile: d3.treemapSquarify.ratio(1) })),
    ...spread("round", tm(DEEP, { size: [100, 60], round: true })),
  }) && ok;
  ok = compare("treemap/unbalanced", load(F, "unbalanced"), {
    id: pre(make(UNB)).map(n => n.id),
    ...spread("squarify", tm(UNB, { size: [80, 50], tile: d3.treemapSquarify })),
    ...spread("binary", tm(UNB, { size: [80, 50], tile: d3.treemapBinary })),
  }) && ok;
}

// ---- partition ----------------------------------------------------------
{
  const F = `${DIR}/hierarchy-partition.probe.jsonl`;
  const pt = (rows, cfg) => { const r = make(rows), t = d3.partition().size(cfg.size); if (cfg.padding) t.padding(cfg.padding); if (cfg.round) t.round(true); t(r); return r; };
  ok = compare("partition/icicle", load(F, "icicle"), {
    id: pre(make(DEEP)).map(n => n.id),
    ...spread("plain", pt(DEEP, { size: [100, 60] })),
    ...spread("padded", pt(DEEP, { size: [100, 60], padding: 1.5 })),
    ...spread("rounded", pt(DEEP, { size: [100, 60], round: true })),
  }) && ok;
  const sun = pt(DEEP, { size: [2 * Math.PI, 24] });
  ok = compare("partition/sunburst", load(F, "sunburst"), {
    id: pre(sun).map(n => n.id),
    "sunburst/startAngle": pre(sun).map(n => n.x0),
    "sunburst/endAngle": pre(sun).map(n => n.x1),
    "sunburst/innerRadius": pre(sun).map(n => n.y0),
    "sunburst/outerRadius": pre(sun).map(n => n.y1),
  }) && ok;
  ok = compare("partition/unbalanced", load(F, "unbalanced"), {
    id: pre(make(UNB)).map(n => n.id),
    ...spread("plain", pt(UNB, { size: [80, 50] })),
  }) && ok;
}

// ---- pack ---------------------------------------------------------------
{
  const F = `${DIR}/hierarchy-pack.probe.jsonl`;
  const circles = (prefix, root) => ({
    [`${prefix}/x`]: pre(root).map(n => n.x), [`${prefix}/y`]: pre(root).map(n => n.y), [`${prefix}/r`]: pre(root).map(n => n.r),
  });
  const pk = (rows, cfg) => {
    const r = make(rows), t = d3.pack().size(cfg.size);
    if (cfg.padding) t.padding(cfg.padding);
    if (cfg.radius) t.radius(cfg.radius);
    t(r); return r;
  };
  ok = compare("pack/default-radius", load(F, "default-radius"), {
    id: pre(make(DEEP)).map(n => n.id),
    ...circles("plain", pk(DEEP, { size: [100, 100] })),
    ...circles("oblong", pk(DEEP, { size: [120, 80] })),
  }) && ok;
  ok = compare("pack/padding", load(F, "padding"), {
    ...circles("padded", pk(DEEP, { size: [100, 100], padding: 3 })),
  }) && ok;
  ok = compare("pack/explicit-radius", load(F, "explicit-radius"), {
    ...circles("value", pk(DEEP, { size: [100, 100], radius: (d) => d.value })),
    ...circles("constant", pk(DEEP, { size: [100, 100], radius: () => 4 })),
  }) && ok;
  ok = compare("pack/unbalanced", load(F, "unbalanced"), {
    id: pre(make(UNB)).map(n => n.id),
    ...circles("plain", pk(UNB, { size: [90, 90] })),
    ...circles("padded", pk(UNB, { size: [90, 90], padding: 2 })),
  }) && ok;
}

// ---- aggregates ---------------------------------------------------------
{
  const F = `${DIR}/hierarchy-aggregates.probe.jsonl`;
  const post = (root) => { const o = []; root.eachAfter(n => o.push(n)); return o; };
  const sorted = (rows, cmp) => { const r = make(rows); r.sort(cmp); return r; };
  ok = compare("aggregates/sum-and-count", load(F, "sum-and-count"), {
    id: pre(make(DEEP)).map(n => n.id),
    sum: pre(make(DEEP)).map(n => n.value),
    count: pre(d3.stratify().id(d => d[0]).parentId(d => d[1] || null)(DEEP).count()).map(n => n.value),
    "id/unbalanced": pre(make(UNB)).map(n => n.id),
    "sum/unbalanced": pre(make(UNB)).map(n => n.value),
    "count/unbalanced": pre(d3.stratify().id(d => d[0]).parentId(d => d[1] || null)(UNB).count()).map(n => n.value),
  }) && ok;
  ok = compare("aggregates/depth-and-height", load(F, "depth-and-height"), {
    depth: pre(make(DEEP)).map(n => n.depth), height: pre(make(DEEP)).map(n => n.height),
    "depth/unbalanced": pre(make(UNB)).map(n => n.depth), "height/unbalanced": pre(make(UNB)).map(n => n.height),
    postorder: post(make(DEEP)).map(n => n.id), "postorder/unbalanced": post(make(UNB)).map(n => n.id),
  }) && ok;
  ok = compare("aggregates/sort", load(F, "sort"), {
    "value-descending": pre(sorted(DEEP, (a, b) => b.value - a.value)).map(n => n.id),
    "value-ascending": pre(sorted(DEEP, (a, b) => a.value - b.value)).map(n => n.id),
    "name-descending": pre(sorted(DEEP, (a, b) => (a.id < b.id ? 1 : a.id > b.id ? -1 : 0))).map(n => n.id),
    "height-descending": pre(sorted(UNB, (a, b) => b.height - a.height)).map(n => n.id),
    "height-ascending": pre(sorted(UNB, (a, b) => a.height - b.height)).map(n => n.id),
  }) && ok;
  const deep = make(DEEP), unb = make(UNB);
  ok = compare("aggregates/traversal", load(F, "traversal"), {
    leaves: deep.leaves().map(n => n.id),
    "leaves/unbalanced": unb.leaves().map(n => n.id),
    ancestors: pre(deep)[Object.fromEntries(pre(deep).map((n, i) => [n.id, i]))["c3"]].ancestors().map(n => n.id),
    "ancestors/unbalanced": pre(unb)[Object.fromEntries(pre(unb).map((n, i) => [n.id, i]))["saab"]].ancestors().map(n => n.id),
    links: deep.links().flatMap(l => [l.source.id, l.target.id]),
  }) && ok;
  const PATHS = [["system/core/data", 5], ["system/core/render", 8], ["system/ui/input", 3], ["system/ui/theme", 4], ["system/io/read", 6], ["system/io/write", 2]];
  const byId = new Map();
  const order = [];
  for (const [p, v] of PATHS) {
    let prefix = "", parent = null;
    for (const seg of p.split("/")) {
      prefix = prefix ? prefix + "/" + seg : seg;
      if (!byId.has(prefix)) { byId.set(prefix, { id: prefix, parent, value: 0 }); order.push(prefix); }
      parent = prefix;
    }
    byId.get(p).value = v;
  }
  const rows = order.map(id => [id, byId.get(id).parent ?? "", byId.get(id).value]);
  const proot = make(rows);
  ok = compare("aggregates/path-stratify", load(F, "path-stratify"), {
    id: pre(proot).map(n => n.id), sum: pre(proot).map(n => n.value), depth: pre(proot).map(n => n.depth),
  }) && ok;
}
process.exit(ok ? 0 : 1);
