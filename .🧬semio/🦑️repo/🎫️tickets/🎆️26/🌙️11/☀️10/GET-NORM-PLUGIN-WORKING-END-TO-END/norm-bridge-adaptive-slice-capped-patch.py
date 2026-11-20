import sys
p=sys.argv[1]
s=open(p,encoding="utf-8").read()
a="measuredUs = lastPollUs * 3n, sliceEnd = startedUs + (measuredUs > wallUs ? measuredUs : wallUs);"
b="measuredUs = lastPollUs * 3n, wanted = measuredUs > wallUs ? measuredUs : wallUs, sliceEnd = startedUs + (wanted < 8000000n ? wanted : 8000000n);"
assert s.count(a)==1
open(p,"w",encoding="utf-8").write(s.replace(a,b))
