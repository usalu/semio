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
    return np.r_[(projected-observed).ravel()/4,(projected_width-widths)/10,distances/4,parameters[4]/.1,(parameters[2]-1.)/1.5]


initial=[11.2,6.84,.737,-2.43,0,300,450]
fit=least_squares(residual,initial,bounds=([10.,6.1,.35,-3.1,-.2,180,350],[14,10.,1.8,-1.6,.2,1400,550]),max_nfev=1000)
result={"parameters":dict(zip(["x","y","z","yaw","pitch","focal_pixels","cy"],map(float,fit.x))),"success":bool(fit.success),"cost":float(fit.cost),"residuals":residual(fit.x).tolist(),"column_pixels":project(columns,fit.x).tolist(),"status":"inferred candidate, historical dimensional accuracy unresolved"}
print(json.dumps(result,indent=2))
(TICKET/"🗑️generated"/"hall-camera-r31.json").write_text(json.dumps(result,indent=2))
