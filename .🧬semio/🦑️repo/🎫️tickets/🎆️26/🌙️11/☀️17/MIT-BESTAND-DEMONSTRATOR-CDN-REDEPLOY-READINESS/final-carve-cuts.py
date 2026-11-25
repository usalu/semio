import sys,os,importlib.util,collections
spec=importlib.util.spec_from_file_location('m',os.path.join(os.path.dirname(os.path.abspath(__file__)),'final-carve-manifest-closure.py'));m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
proc=m.closure(['semio-hub-process-component']);dem=m.closure(['semio-hub-demonstrator-component'])
cut=['semio-s-artifact-procedural-3d-editor','semio-s-artifact-puzzle-3d-editor','semio-s-artifact-procedural-generation3d','semio-s-artifact-puzzle-3d','semio-s-artifact-gis-gismap','semio-s-artifact-cad-cad-editor']
d1=m.closure(['semio-hub-demonstrator-component'],drop=set(cut))
print('D1 demonstrator',len(dem),'->',len(d1),'removed',len(dem-d1),dict(collections.Counter(m.fam(n) for n in dem-d1).most_common(8)))
orig=m.edges
def narrowed(keep):
    m.edges=lambda n:[(k,False) for k in keep] if n=='semio-s-artifact-stdio-semio' else orig(n)
    r=(m.closure(['semio-hub-process-component']),m.closure(['semio-hub-demonstrator-component'],drop=set(cut)),m.closure(['semio-hub-demonstrator-component']))
    m.edges=orig;return r
base=['semio-s-artifact-stdio-semio-base','semio-s-artifact-stdio-semio-brep','semio-s-artifact-stdio-semio-flow','semio-s-artifact-stdio-contract']
p2,d2,d3=narrowed(base)
print('P1 process roster brep+flow+base:',len(proc),'->',len(p2),'removed',len(proc-p2),dict(collections.Counter(m.fam(n) for n in proc-p2).most_common(10)))
print('process direct stdio families kept:',sorted({m.fam(n) for n in p2 if 'stdio' in n}))
print('D1+P1 demonstrator:',len(d1),'->',len(d2))
for n,mb in [('semio-s-artifact-stdio-dwg-snapshot-objects',229),('semio-s-artifact-stdio-binary-io-registry',190),('semio-s-artifact-stdio-semio-presentation-member',165),('semio-s-artifact-stdio-txt-tests',160),('semio-s-artifact-stdio-semio-model-member',153),('semio-s-artifact-stdio-semio-brep-member',145),('semio-s-artifact-stdio-semio-graph-editor',140),('semio-s-artifact-stdio-semio-document-member',104),('semio-plugin-actor-input',205)]:
    print(f'{mb:4d} {n}: proc={n in proc} dem={n in dem} dem-after-D1={n in d1} proc-after-P1={n in p2}')
print('tests crates in release closure (process):',sorted(n for n in proc if n.endswith('-tests')))
print('tests crates in release closure (demonstrator):',sorted(n for n in dem if n.endswith('-tests')))
print('stdio-semio edges',len(orig('semio-s-artifact-stdio-semio')))
