import { load } from "./graph.mjs";
const { tops, lines, markerLines } = load(process.argv[2]);
const covered = new Set();
for (const t of tops) for (let l = t.startLine; l <= t.endLine; l++) covered.add(l);
let n = 0;
lines.forEach((text, i) => { const l = i + 1; if (!covered.has(l) && !markerLines.has(l) && text.trim() !== "") { n++; console.log(l, text.slice(0, 160)); } });
console.log("dangling", n, "of", lines.length, "lines; tops", tops.length);
