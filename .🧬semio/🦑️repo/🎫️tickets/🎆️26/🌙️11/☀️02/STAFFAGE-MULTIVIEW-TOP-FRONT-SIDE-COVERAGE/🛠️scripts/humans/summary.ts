const a = JSON.parse(await Bun.file(process.argv[2]).text());
for (const [slug, r] of Object.entries<any>(a)) {
  console.log(`## ${slug} dims h${r.dims.heightM} w${r.dims.widthM} d${r.dims.depthM} abd:${r.abd} tw:${r.twist} sym:${JSON.stringify(r.symbols)} boneZ:${r.boneZ}`);
  for (const [k, s] of Object.entries<any>(r.scales)) {
    if (s.status) { console.log(`  ${k}: UNAVAILABLE alt=${JSON.stringify(s.alt)}`); continue; }
    const d = s.dim;
    console.log(`  ${k}: cnt ${s.counts.elevation}/${s.counts.front}/${s.counts.plan} unb${s.unbound.length} bonemiss[${s.idmiss.join(";")}] handed[${s.handed.join(";")}] mism ${JSON.stringify(s.mism)} tol${s.tol} e${d.e} f${d.f} p${d.p} fz${d.fz} pz${d.pz}`);
  }
}
