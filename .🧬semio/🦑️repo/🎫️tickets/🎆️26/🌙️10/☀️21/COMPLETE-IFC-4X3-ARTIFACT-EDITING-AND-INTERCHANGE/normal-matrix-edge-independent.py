import pathlib,ctypes,sys,json,math
p=pathlib.Path(__file__).resolve().parent;folder=str(p/'🗑️generated/determinant')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(folder,b,len(b));folder=b.value or folder
sys.path.insert(0,folder);import mpmath as mp
mp.mp.dps=1600
rows=[{'id':'minimum-subnormal-normal','columns':[[1.,0.,0.],[0.,1.,0.],[0.,0.,1.]],'normal':[math.ulp(0.),0.,0.]},{'id':'uniform-tiny-matrix','columns':[[1e-300,0.,0.],[0.,1e-300,0.],[0.,0.,1e-300]],'normal':[1.,1.,1.]},{'id':'large-cofactor-cancellation','columns':[[1e200,1e200,0.],[1e200,math.nextafter(1e200,math.inf),0.],[0.,0.,1e-200]],'normal':[1.,2.,0.]}]
for row in rows:
 a=mp.matrix([[mp.mpf(row['columns'][c][r])for c in range(3)]for r in range(3)]);n=mp.matrix([mp.mpf(x)for x in row['normal']]);v=a.T**-1*n;d=v/mp.sqrt(sum(x*x for x in v));row['expectedDirection']=[float(x)for x in d];row['highPrecisionDirection']=[mp.nstr(x,90)for x in d];row['determinantSign']=int(mp.sign(mp.det(a)))
(p/'🗑️generated/gltf-normals/matrix-edge-mp.json').write_text(json.dumps({'version':mp.__version__,'decimalPrecision':mp.mp.dps,'rows':rows},indent=2),encoding='utf8');print(json.dumps([{'id':r['id'],'direction':r['expectedDirection']}for r in rows]))
