import { load } from "./lib.ts";
const [slug, k, view, re] = process.argv.slice(2);
const a = load(slug!).asset;
for (const s of a.representations[k!].geometry[view!].filter((s: any) => new RegExp(re!).test(s.id))) console.log(JSON.stringify(s).slice(0, 700));
