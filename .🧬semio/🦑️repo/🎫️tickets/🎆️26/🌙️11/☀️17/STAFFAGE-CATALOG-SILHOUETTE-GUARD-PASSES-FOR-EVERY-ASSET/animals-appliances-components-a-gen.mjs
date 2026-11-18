import { readFileSync } from "node:fs";
export default (asset) => {
  for (const o of JSON.parse(readFileSync(process.env.OPS, "utf8"))) {
    const arr = o.kind === "dim" ? [] : o.kind === "shape" ? asset.representations[o.scale].geometry[o.view] : asset.volumes[o.scale].body;
    if (o.kind === "dim") { let n = asset; for (const k of o.path.slice(0, -1)) n = n[k]; n[o.path.at(-1)] = o.value; continue; }
    if (o.add) { arr.splice(arr.findIndex((x) => x.id === o.after) + 1, 0, o.add); continue; }
    if (o.replace) { arr[arr.findIndex((x) => x.id === o.id)] = o.replace; continue; }
    const t = arr.find((x) => x.id === o.id);
    if (o.tfn) new Function("t", o.tfn)(t);
    if (o.copyall) { const src = asset.volumes[o.from].body.find((x) => x.id === o.id); for (const k of Object.keys(src)) if (k !== "id") t[k] = JSON.parse(JSON.stringify(src[k])); }
    if (o.setsolid) Object.assign(t.solid, o.setsolid);
    if (o.set) Object.assign(t, o.set);
    if (o.secs) for (const [i, kv] of Object.entries(o.secs)) Object.assign(t.sections[i], kv);
    if (o.copy) { const src = asset.volumes[o.from ?? o.scale].body.find((x) => x.id === o.copy); t.sections = JSON.parse(JSON.stringify(src.sections)).map((q) => ({ ...q, at: [q.at[0], q.at[1], q.at[2] * (o.mirror ? -1 : 1)] })); }
    if (o.fn) { const f = new Function("q", "i", "s", o.fn); t.sections.forEach((q, i) => f(q, i, t)); }
    if (o.allsecs) for (const q of t.sections) Object.assign(q, o.allsecs);
  }
};
