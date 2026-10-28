#!/usr/bin/env python3
"""census-summary.py <jsonl>...: streams the one.py rows; prints count, p50/p90/p99/max of the peak tree RSS, the files above 600 MB and the number of timeouts/ceiling kills."""
import json,sys
peaks=[];rows=[];bad=0
for p in sys.argv[1:]:
    for l in open(p):
        r=json.loads(l);peaks.append(r['peakMb']);rows.append(r);bad+=r['why']!='ok'
peaks.sort();n=len(peaks);q=lambda f:peaks[min(n-1,int(n*f))]
print(f"files {n}  p50 {q(.5)}  p90 {q(.9)}  p99 {q(.99)}  max {peaks[-1]} MB  timeouts/kills {bad}  above 600 MB: {sum(p>600 for p in peaks)}  above 1000 MB: {sum(p>1000 for p in peaks)}")
for r in sorted(rows,key=lambda r:-r['peakMb'])[:14]:
    print(f"{r['peakMb']:6d} MB {r['wallS']:7.1f} s rc {r['rc']:>3} {r['file'][-95:]}")
