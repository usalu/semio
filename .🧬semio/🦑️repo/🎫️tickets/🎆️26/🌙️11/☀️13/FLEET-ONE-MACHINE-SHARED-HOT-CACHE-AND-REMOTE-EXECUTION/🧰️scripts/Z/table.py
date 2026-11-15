import json,sys
d=json.load(open(sys.argv[1]))
for p in d['phases']:
    print(f"### phase {p['phase']}: exit {p['offload']['status']}, wall {p['offload']['wallMs']} ms (local reference {p['reference']['wallMs']} ms), offloaded {p['offloadedUnits']}/{len(p['units'])}, median round trip {p['roundTripMedianMs']} ms, bytes shipped {p['bytes']['shipped']} / uploaded {p['bytes']['uploaded']} / returned {p['bytes']['returned']}, byte-identical {p['identical']}/{len(p['compared'])}, declines {p['declines']}, scheduler offloadedIn {p.get('schedulerOffloadedIn')}")
    print("| crate | ran on | round trip ms | shipped B | uploaded B | returned B | reason |")
    print("|---|---|---|---|---|---|---|")
    for u in p['units']:
        w=u['where'].replace('ueli-mac.fritz.box-','worker-')
        print(f"| {u['crate']} | {w} | {u['roundTripMs']} | {u['bytesShipped']} | {u['bytesUploaded']} | {u['bytesReturned']} | {u['reason'] or ''} |")
