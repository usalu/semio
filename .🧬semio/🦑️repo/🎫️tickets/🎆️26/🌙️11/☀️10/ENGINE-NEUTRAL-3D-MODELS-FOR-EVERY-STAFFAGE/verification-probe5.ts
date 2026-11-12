const K = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model";
const { triangulatePolygon, signedArea } = await import(`${K}/🔺️tessellation/🔺️polygon/🟦️.ts`);
const outer: [number, number][] = [[-2, 0.06], [2, 0.06], [2, 1.26], [-2, 1.26]];
const h1: [number, number][] = [[-1.965, 0.095], [-0.015, 0.095], [-0.015, 1.225], [-1.965, 1.225]].reverse() as any;
const h2: [number, number][] = [[0.015, 0.095], [1.965, 0.095], [1.965, 1.225], [0.015, 1.225]].reverse() as any;
console.log(signedArea(outer), signedArea(h1), signedArea(h2));
const t = triangulatePolygon(outer, [h1, h2]);
console.log(t.length / 3, "expected 14");
const pts = [...outer, ...h1, ...h2];
let a = 0; for (let i = 0; i < t.length; i += 3) { const [p, q, r] = [pts[t[i]!]!, pts[t[i + 1]!]!, pts[t[i + 2]!]!]; a += ((q[0] - p[0]) * (r[1] - p[1]) - (q[1] - p[1]) * (r[0] - p[0])) / 2; }
console.log("area", a, "expected", 4 * 1.2 - 2 * 1.95 * 1.13);
