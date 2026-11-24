/** ⚖️ Runs `exactIou` of an oracle module on dumped views and prints the exact IoU beside the owned one (or the error thrown) with the time taken.
 *  bun compare.ts <oracle.ts> <dump.json>… */
const [oracle, ...files] = process.argv.slice(2);
const { exactIou } = await import(oracle!);
for (const file of files) {
  for (const v of await Bun.file(file).json()) {
    const t = performance.now();
    let text: string;
    try { const e = exactIou(v); text = `exact ${e.toFixed(9)} owned ${v.iou.toFixed(6)} diff ${Math.abs(e - v.iou).toFixed(6)}`; }
    catch (error) { text = `THROWS ${String(error).slice(0, 120)}`; }
    console.log(`${v.id}\t1:${v.scale}\t${v.view}\t${text}\t${Math.round(performance.now() - t)} ms\tgroups ${v.filled.length}/${v.outlines.length}/${v.reference.length}`);
  }
}
