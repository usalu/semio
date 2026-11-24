/** 🎯️ Counts, per polygon set of a dumped view, the rings whose area on the grid of a unit is exactly the drop limit of 4 cells: the integer oracle drops them, the float oracle kept or dropped them by rounding noise (`area > 4 * unit * unit`).
 *  bun ties.ts <dump.json> <unit> [view-index] */
const [file, unitText, index] = process.argv.slice(2);
const unit = Number(unitText);
const views = await Bun.file(file!).json();
for (const v of index === undefined ? views : [views[Number(index)]]) {
  const parts: string[] = [];
  for (const name of ["filled", "outlines", "reference", "visible", "full"]) {
    let [ties, kept] = [0, 0];
    for (const g of v[name] ?? []) for (const ring of g) {
      const q = (ring as { x: number; y: number }[]).map((p) => [Math.round(p.x / unit), Math.round(p.y / unit)]);
      let [s, f] = [0, 0];
      for (let i = 0; i < q.length; i++) { const [a, b] = [q[i]!, q[(i + 1) % q.length]!]; s += a[0]! * b[1]! - b[0]! * a[1]!; f += a[0]! * unit * (b[1]! * unit) - b[0]! * unit * (a[1]! * unit); }
      if (Math.abs(s / 2) === 4) { ties++; if (Math.abs(f / 2) > 4 * unit * unit) kept++; }
    }
    parts.push(`${name} ${ties} ties (${kept} kept by the float limit)`);
  }
  console.log(`${v.id} 1:${v.scale} ${v.view} unit ${unit}: ${parts.join(", ")}`);
}
