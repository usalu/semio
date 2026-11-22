/** 🎨️ Compares stable before/hover/after RGBA pixels from one unchanged preview rectangle. */
export function paintEvidence(before, hovered, after, threshold = 24, minimum = 20) {
  if (before.length !== hovered.length || before.length !== after.length || before.length % 4) throw Error("Matching RGBA screenshots required");
  let enter = 0, leave = 0, restored = 0, overlap = 0;
  for (let i = 0; i < before.length; i += 4) {
    const delta = (a, b) => Math.abs(a[i] - b[i]) + Math.abs(a[i + 1] - b[i + 1]) + Math.abs(a[i + 2] - b[i + 2]);
    const on = delta(before, hovered) >= threshold, off = delta(hovered, after) >= threshold, drift = delta(before, after) >= threshold;
    enter += Number(on); leave += Number(off); restored += Number(drift); overlap += Number(on && off && !drift);
  }
  const pass = overlap >= minimum && overlap >= 0.5 * Math.max(enter, leave) && restored <= Math.max(10, overlap * 0.2);
  return { pass, enter, leave, restored, overlap, threshold, minimum };
}
