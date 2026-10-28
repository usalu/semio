#!/usr/bin/env python3
"""show.py <rows.jsonl>: one line per scenario row (streams the file)."""
import json,sys
for l in open(sys.argv[1]):
    r=json.loads(l);o=r.get('oracle',{})
    print('%-24s wall %6.1f cpu %6.1f rss %5s ran %s hits %s sumRanMs %s err %s | full wall %6.1f cpu %5.1f err %s same %s %s %s'%(r['step'],r['wallS'],r['cpuS'],r.get('peakRssMb'),r.get('ran','-'),r.get('hits','-'),r.get('sumRanMs','-'),r['errors'],o.get('wallS',0),o.get('cpuS',0),o.get('errors'),r.get('same'),r.get('onlyUnits',''),r.get('onlyFull','')))
