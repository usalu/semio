import json,numpy as np,sys
f=json.load(open(sys.argv[1]))
for r in f["parameter"]:
    u=np.array(r["rayDirection"],float);d=np.array(r["axis"],float)
    if not np.linalg.norm(u) or not np.linalg.norm(d): assert r["s"] is None;continue
    u/=np.linalg.norm(u);d/=np.linalg.norm(d)
    A=np.array([[d@d,-d@u],[d@u,-u@u]]);b=np.array([-(d@(np.array(r["axisOrigin"],float)-np.array(r["rayOrigin"],float))),-(u@(np.array(r["axisOrigin"],float)-np.array(r["rayOrigin"],float)))])
    if abs(np.linalg.det(A))<1e-9: assert r["s"] is None,r["name"];continue
    s=np.linalg.solve(A,b)[0]; assert abs(s-r["s"])<1e-9,(r["name"],s)
print("oracle ok")
