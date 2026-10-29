"""📷 Inferred camera fit against existing hall controls; output retains residuals rather than claiming measured stations."""
import json,math
from pathlib import Path
import numpy as np
from scipy.optimize import least_squares

TICKET=Path(__file__).parent
case=json.loads((TICKET/"hall-glazing-registration-r31.json").read_text())
columns=[np.array([10.898,6.006,0.]),np.array([8.128,6.006,0.])]
observed=np.array([case["controls"][k]["base_pixel"] for k in ("near_column","far_column")])
widths=np.array([106.,29.])
floor=np.array(case["floor_trace_pixels"])
curve=np.c_[np.asarray(json.loads((TICKET/"🗑️generated"/"ground-registration-r31-result.json").read_text())["trace_world"]),np.zeros(15)]

def camera(parameters):
    x,y,z,yaw,pitch,f,cy=parameters
    forward=np.array([math.cos(pitch)*math.cos(yaw),math.cos(pitch)*math.sin(yaw),math.sin(pitch)])
    right=np.array([math.sin(yaw),-math.cos(yaw),0.])
    up=np.cross(right,forward)
    return np.array([x,y,z]),forward,right,up,f,cy

def project(points,parameters):
    origin,forward,right,up,f,cy=camera(parameters)
    relative=np.asarray(points)-origin
    depth=relative@forward
    depth=np.maximum(depth,.03)
    return np.c_[700+f*(relative@right)/depth,cy-f*(relative@up)/depth]

def residual(parameters):
    origin,forward,right,up,f,cy=camera(parameters)
    projected=project(columns,parameters)
    projected_width=np.array([np.linalg.norm(project([p+.14*right],parameters)[0]-project([p-.14*right],parameters)[0]) for p in columns])
    projected_curve=project(curve,parameters)
    depths=(curve-origin)@forward
    projected_curve=projected_curve[depths>.1]
    segments=projected_curve[1:]-projected_curve[:-1]
    relative=floor[:,None,:]-projected_curve[None,:-1,:]
    fraction=np.clip(np.sum(relative*segments[None,:,:],axis=2)/np.maximum(np.sum(segments**2,axis=1),1e-12)[None,:],0,1)
    nearest=projected_curve[None,:-1,:]+fraction[:,:,None]*segments[None,:,:]
    distances=np.linalg.norm(floor[:,None,:]-nearest,axis=2).min(axis=1)
    head=project([columns[1]+np.array([0,0,2.91])],parameters)[0]
    return np.r_[(projected-observed).ravel()/4,(projected_width-widths)/10,distances/4,(head-np.array([845.,220.]))/5,parameters[4]/.1]



results=[]
for label,near,far,initial in [('8-9',[10.898,6.006,0],[8.128,6.006,0],[11.5,7.1,1.4,-2.48,0,500,370]),('9-15',[8.128,6.006,0],[4.737,8.23,0],[9.5,7,1.4,-3.1,0,500,370]),('9-10',[8.128,6.006,0],[4.75,6,0],[9.5,7,1.4,-2.8,0,500,370]),('8-15',[10.898,6.006,0],[4.737,8.23,0],[11.6,7.5,1.4,-2.9,0,500,370])]:
    columns=[np.array(near,float),np.array(far,float)]
    fit=least_squares(residual,initial,bounds=([8.3,6.1,.5,-3.8,-.2,180,300],[14,11.,1.9,-1.6,.2,1400,550]),max_nfev=1500)
    results.append({'hypothesis':label,'parameters':fit.x.tolist(),'cost':float(fit.cost),'success':bool(fit.success),'residuals':residual(fit.x).tolist(),'column_pixels':project(columns,fit.x).tolist()})
print(json.dumps(results,indent=2))
(TICKET/'🗑️generated'/'hall-camera-hypotheses-r31.json').write_text(json.dumps(results,indent=2))
