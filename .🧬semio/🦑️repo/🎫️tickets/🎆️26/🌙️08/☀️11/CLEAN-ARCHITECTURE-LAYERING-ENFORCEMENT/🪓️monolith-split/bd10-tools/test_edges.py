import json,re,sys,os
sys.dont_write_bytecode=True
REPO="/Users/ueli/Documents/semio"
IDENT={"agg":"semio_s_artifact_raster_raster","model":"semio_s_artifact_raster_raster_model","diff":"semio_s_artifact_raster_raster_diff","mutations":"semio_s_artifact_raster_raster_mutations","layers":"semio_s_artifact_raster_raster_mutations_layers","pixels":"semio_s_artifact_raster_raster_mutations_pixels","io":"semio_s_artifact_raster_raster_io","editor":"semio_s_artifact_raster_raster_editor","viewer":"semio_s_artifact_raster_raster_viewer","host":"semio_s_artifact_raster_raster_host","examples":"semio_s_artifact_raster_raster_examples"}
c=json.load(open(sys.argv[1]))
only=sys.argv[2:]
for n,v in c.items():
    if only and n not in only: continue
    libset=set(v['lib'])
    for f in v['test']:
        if f in libset or not f.endswith('.rs'): continue
        t=open(os.path.join(REPO,f)).read()
        hits=[m for m,i in IDENT.items() if m!=n and re.search(r'(?<![\w])'+i+r'(?![\w])',t)]
        if hits: print(n, hits, f.split('✳️any/')[-1] if '✳️any/' in f else f)
