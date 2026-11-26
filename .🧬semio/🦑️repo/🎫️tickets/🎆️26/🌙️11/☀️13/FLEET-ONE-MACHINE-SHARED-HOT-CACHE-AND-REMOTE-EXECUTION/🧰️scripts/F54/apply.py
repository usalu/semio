import os,sys,tempfile
p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts"
t=open(p,encoding="utf8").read()
a1='''/** ⏳️ Expected wall time of a key'''
n1='''/** 🧬️ The rows recorded for the same unit under another profile or other inputs (same owner, crate, target and crate types, a source of comparable size): the evidence a unit has when its exact key was never measured, e.g. a `check` of a crate that only built before. It replaces the cold size model and never the exact history.
 * @see admissionForecastHistoryV1 */
export function admissionSiblingHistoryV1(
  key: string, lines: number,
  history: ReadonlyMap<string, readonly UnitObservationV1[]>,
  localHistory?: ReadonlyMap<string, readonly UnitObservationV1[]>,
): readonly UnitObservationV1[] {
  const own = key.split("|");
  if (own.length < 4) return [];
  const rows: UnitObservationV1[] = [];
  for (const source of localHistory ? [history, localHistory] : [history]) {
    for (const [other, observed] of source) {
      const parts = other.split("|");
      if (parts.length >= 4 && parts[0] === own[0] && parts[1] === own[1] && parts[3] === own[3]) rows.push(...observed);
    }
  }
  return forecastHistoryV1(rows, lines);
}

'''
assert t.count(a1)==1
t=t.replace(a1,n1+a1)
a2='''    const predicted = predictPeakBytesV1(history.map((row) => row.peakRssBytes), { kloc: history.length || componentFinal ? 0 : kloc, component: componentFinal ? componentPeaks.get(pass.profile)! : null });
    const coldWarning = history.length || componentFinal || coldWarned.has(key) ? null'''
n2='''    const sibling = history.length || componentFinal ? [] : admissionSiblingHistoryV1(key, lines, options.history, options.localHistory);
    const predicted = predictPeakBytesV1((history.length ? history : sibling).map((row) => row.peakRssBytes), { kloc: history.length || sibling.length || componentFinal ? 0 : kloc, component: componentFinal ? componentPeaks.get(pass.profile)! : null });
    const coldWarning = history.length || sibling.length || componentFinal || coldWarned.has(key) ? null'''
assert t.count(a2)==1
t=t.replace(a2,n2)
d=os.path.dirname(p);fd,tmp=tempfile.mkstemp(dir=d,suffix=".tmp");os.write(fd,t.encode());os.close(fd);os.rename(tmp,p);print("ok")
