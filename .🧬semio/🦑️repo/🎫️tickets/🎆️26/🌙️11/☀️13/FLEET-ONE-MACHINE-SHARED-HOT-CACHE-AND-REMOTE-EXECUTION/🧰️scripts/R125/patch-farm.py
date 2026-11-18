import sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R125/"
p=S+"stage/farm.mjs";t=open(p,encoding="utf8").read()
a="/** 🛑️ Whether the farm holds its next job"
doc='''/** 📏️ What the next farm child is booked at: the declared megabytes of a child until three children have reported their footprint, then the 75th percentile of the measured footprints plus 15 percent, never under 768 MB and never over the declared size, so the reservation follows what children really hold and the gate carries as many of them as the memory allows.
 * @see ./🧪️tests/🧮️measured-ask/🟦️.ts */
export const askWorkerMegabytesV1 = (declaredMegabytes, footprintsMegabytes) => {
  const sizes = footprintsMegabytes.filter(value => Number.isFinite(value) && value > 0).sort((left, right) => left - right);
  if (sizes.length < 3) return declaredMegabytes;
  return Math.max(768, Math.min(declaredMegabytes, Math.ceil(sizes[Math.min(sizes.length - 1, Math.ceil(sizes.length * 0.75) - 1)] * 1.15)));
};

'''
if a not in t: sys.exit("anchor farmHeld")
t=t.replace(a,doc+a,1)
b="megabytes: policy.workerMegabytes, environment, signal, write });"
if b not in t: sys.exit("anchor ask")
t=t.replace(b,"megabytes: askWorkerMegabytesV1(policy.workerMegabytes, [...peaks.values()].map(bytes => bytes / 1048576)), environment, signal, write });",1)
open(p,"w",encoding="utf8").write(t)
