import sys; sys.path.insert(0,'.')
from author_09_lib import *
slug='skateboarder'
v=seed(slug)
body=v['50']['body']
for s in body:
    if s['kind']=='loft' and s['id'] in ('upper-arm-l','upper-arm-r','forearm-l','forearm-r','thigh-l','thigh-r','shank-l','shank-r','hoodie'):
        for q in s['sections']: q['exponent']=float(sys.argv[1])
save(slug,v)
