import glob,json,sys,os
since=int(sys.argv[1]) if len(sys.argv)>1 else 0
crate=sys.argv[2] if len(sys.argv)>2 else 'semio_s_artifact_norm_en1995_mutations'
for f in glob.glob('/Users/ueli/Documents/semio/.cache/b/r/m/u/x/*.jsonl'):
    for l in open(f,encoding='utf8',errors='replace'):
        if '"crate":"%s"'%crate not in l or 'peakRss' not in l: continue
        d=json.loads(l)
        if d['atMs']>=since: print(os.path.basename(f),d['atMs'],d['kind'],round(d['peakRssBytes']/1048576),'MiB',d['milliseconds']//1000,'s')
