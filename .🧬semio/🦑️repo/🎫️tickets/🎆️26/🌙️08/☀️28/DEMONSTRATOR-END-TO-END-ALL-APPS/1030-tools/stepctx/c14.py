import glob,re,subprocess
R='/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/'
files=[l for l in open('/private/tmp/claude-501/-Users-ueli-Documents-semio/accc91d1-14b6-46c1-b4d8-0d673bc505b1/scratchpad/ff.txt').read().split('\n') if l]
files=['/Users/ueli/Documents/semio/'+f for f in files]
pat=re.compile(r'\b(job|whole)\.step\((1|4|8|100|1_000|usize::MAX|fuel)\)')
imp='use semio_s_artifact_procedural_3d_geometry_compute::testing::step_with_fuel;\n'
for f in files:
    if '/🧪️tests/' not in f and '/tests/' not in f: continue
    s=open(f,encoding='utf8').read()
    n=pat.sub(lambda m:f'step_with_fuel(&mut *{m.group(1)}, {m.group(2)})',s)
    if n!=s:
        # add import after first 'use ' line
        i=n.index('use ')
        n=n[:i]+imp+n[i:]
        open(f,'w',encoding='utf8').write(n); print(f.split('geometry/')[-1])
