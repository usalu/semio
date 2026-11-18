S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R124/"
def rd(p):return open(p,encoding="utf8").read()
def wr(p,s):open(p,"w",encoding="utf8").write(s)
def sub(s,a,b,count=1):
    assert s.count(a)==count,(a[:70],s.count(a))
    return s.replace(a,b)
r=rd(S+"stage/runner.mjs")
r=sub(r,'pressure: "reasonNothingFits", off:','pressure: "reasonPressure", waiting: "reasonWaiting", starting: "reasonStarting", memory: "reasonMemory", cores: "reasonCores", off:')
r=sub(r,'const reasonOf = (key, fields = {}) => ({ reason: key, reasonText: key === null || REASONS[key] === undefined ? null : { en: fill(T.en[REASONS[key]], fields), de: fill(T.de[REASONS[key]], fields) } });',
'const reasonOf = (key, fields = {}, textKey = key) => ({ reason: key, reasonText: textKey === null || REASONS[textKey] === undefined ? null : { en: fill(T.en[REASONS[textKey]], fields), de: fill(T.de[REASONS[textKey]], fields) } });')
r=sub(r,'const childMegabytes = () => ownerPoolChildMegabytesV1(samples);','const childMegabytes = () => ownerPoolChildMegabytesV1(samples, [...children].filter(child => child.ready && child.rss > 0).map(child => child.rss));')
r=sub(r,'  const report = (target, reason) => {\n','  const report = ({ children: target, reason, limit }) => {\n')
r=sub(r,'newest = open[0] ?? [...waves.values()].sort((a, b) => b.createdMs - a.createdMs)[0];\n',
'newest = open[0] ?? [...waves.values()].sort((a, b) => b.createdMs - a.createdMs)[0];\n    const starting = why === null && open.length > 0 && children.size < target, seconds = starting ? Math.ceil(ownerPoolStartDelayV1({ children: [...children], lastStartMs, now: now(), pollMilliseconds: targetPollMilliseconds }) / 1000) : Math.round((limit?.waitedMilliseconds ?? 0) / 1000), shown = starting ? "starting" : why;\n    const fields = { waves: waves.size, children: children.size, target, seconds, need: limit?.needMegabytes, free: limit?.freeMegabytes, floor: limit?.floorMegabytes, held: limit?.heldMegabytes, cores: limit?.cores, booked: limit?.bookedCores, idle: limit?.idleCores == null ? "?" : Math.floor(limit.idleCores), swap: Math.round(limit?.swapOutMegabytesPerSecond ?? 0) }, textKey = (shown === "nothing-fits" || shown === "yielding") && limit !== undefined ? (limit.limit === "memory" ? "memory" : "cores") : shown;\n')
r=sub(r,'...reasonOf(why, { waves: waves.size }) });','...reasonOf(shown, fields, textKey) });')
r=sub(r,'report(target.children, target.reason);','report(target);')
wr(S+"stage/runner.mjs",r)
