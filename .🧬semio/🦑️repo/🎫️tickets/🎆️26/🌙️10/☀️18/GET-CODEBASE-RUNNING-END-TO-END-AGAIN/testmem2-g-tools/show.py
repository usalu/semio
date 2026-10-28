import json,sys
for l in open(sys.argv[1]):
    r=json.loads(l);print(r['peakMb'],r['wallS'],r['rc'],r['why'],r['pass'],r['fail'],r['file'].split(sys.argv[2])[-1] if len(sys.argv)>2 else r['file'])
