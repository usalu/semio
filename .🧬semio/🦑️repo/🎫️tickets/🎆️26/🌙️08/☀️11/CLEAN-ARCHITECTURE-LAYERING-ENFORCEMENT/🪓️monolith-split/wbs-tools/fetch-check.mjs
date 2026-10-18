const [base, dir] = process.argv.slice(2);
const r = await fetch(base + "/"); const html = await r.text();
console.log("GET /", r.status, r.headers.get("content-type"), html.length + " bytes");
const refs = [...html.matchAll(/(?:src|href)="([^"#?]+\.(?:js|css|svg|ico|webmanifest|png))"/gu)].map((m) => m[1]);
let bad = 0;
for (const ref of new Set(refs)) { const u = new URL(ref, base + "/"); const x = await fetch(u); if (!x.ok) bad++; console.log(x.status, ref); }
console.log("REFS", refs.length, "BAD", bad);
process.exit(bad || r.status !== 200 ? 1 : 0);
