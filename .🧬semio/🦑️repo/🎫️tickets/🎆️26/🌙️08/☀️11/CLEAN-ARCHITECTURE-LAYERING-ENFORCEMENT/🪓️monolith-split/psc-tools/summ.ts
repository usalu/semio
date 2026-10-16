import { readFileSync } from "node:fs";
const e: any[] = JSON.parse(readFileSync(process.argv[2], "utf8"));
const area = (f: string) => {
  const p = f.split("/");
  if (p[0] === "🧰️framework") return p.slice(0, f.startsWith("🧰️framework/🔨️modules") || f.startsWith("🧰️framework/🛍️products") ? 3 : 2).join("/") + (p[0]==="🧰️framework"&&p[1]==="🛍️products"&&p[2]==="🦑️repo" ? "/"+p[3]+"/"+p[4] : p[1]==="🛍️products"&&p[2]==="💻️os" ? "/"+p[3]+"/"+p[4]:"");
  return p.slice(0, 3).join("/");
};
const g = (k: (x: any) => string, filt: (x: any) => boolean = () => true) => { const m = new Map<string, number>(); for (const x of e.filter(filt)) m.set(k(x), (m.get(k(x)) ?? 0) + 1); return [...m].sort((a, b) => b[1] - a[1]); };
console.log("== by owner/test"); console.log(g(x => x.owner + (x.test ? " test" : " prod")));
console.log("== by area prod"); for (const [k, n] of g(x => x.owner + " " + area(x.file), x => !x.test)) console.log(n, k);
console.log("== by target prod"); for (const [k, n] of g(x => x.owner + " " + x.resolved, x => !x.test)) console.log(n, k);
console.log("== kinds"); console.log(g(x => x.kind + (x.defaultName ? " default" : "") + (x.namespaceName ? " ns" : "") + (x.exportStar ? " star" : "") + (x.sideEffect ? " side" : "") + (/\.json$/.test(x.resolved) ? " json" : "")));
