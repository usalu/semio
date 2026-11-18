"""Wheelchair user 1:500: the legibility-thickened rear wheel ring (0.065 m) grew outward to a 0.3305 m radius and sank 26 mm below the ground; keep the true 0.305 m tyre radius (24 inch wheel) and thicken it inward."""
import importlib.util,re
spec=importlib.util.spec_from_file_location('e','humans-animals-edit.py');e=importlib.util.module_from_spec(spec);spec.loader.exec_module(e)
S='👤️humans/wheelchair-user'
orig,doc=e.loadpair(S)
RO,RI,NO,NI=0.3305,0.2655,0.305,0.24
def fix(d):
    out=[]
    for sub in [p for p in d.split('Z') if p.strip()]:
        nums=[float(x) for x in re.findall(r'-?\d+\.\d+',sub)]
        k=NO/RO if (nums[0]**2+nums[1]**2)**0.5>0.3 else NI/RI
        out.append('M'+' L'.join('%.5f %.5f'%(nums[i]*k,nums[i+1]*k) for i in range(0,len(nums),2))+' Z')
    return ' '.join(out)
for s in doc['asset']['representations']['500']['geometry']['elevation']:
    if s['id']=='wheel-ring': s['d']=fix(s['d'])
e.save(S,doc,orig)
