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
angles=np.linspace(2*math.pi,math.pi,1501)
curve=np.c_[8.5+3.5*np.cos(angles),6+3.5*np.sin(angles),np.zeros(len(angles))]
curve=np.r_[curve,np.c_[np.full(401,5.),np.linspace(6,10,401),np.zeros(401)]]

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



from scipy.optimize import least_squares

def ground_fit(parameters):
    inverse=np.r_[parameters,1.].reshape(3,3)
    pixels=np.r_[floor,observed]
    normalized=np.c_[(pixels[:,0]-700)/500,(pixels[:,1]-467)/500,np.ones(len(pixels))]
    mapped=normalized@inverse.T
    return mapped[:,:2]/mapped[:,2,None]

def ground_residual(parameters):
    mapped=ground_fit(parameters)
    delta=mapped[:-2]-np.array([8.5,6.])
    angle=np.clip(np.arctan2(delta[:,1],delta[:,0]),-math.pi,0.)
    circular=np.c_[8.5+3.5*np.cos(angle),6+3.5*np.sin(angle)]
    straight=np.c_[np.full(len(floor),5.),np.clip(mapped[:-2,1],6,10)]
    front=np.c_[np.clip(mapped[:-2,0],7.3,9.7),np.full(len(floor),2.5)]
    distances=np.minimum.reduce([np.linalg.norm(mapped[:-2]-circular,axis=1),np.linalg.norm(mapped[:-2]-straight,axis=1),np.linalg.norm(mapped[:-2]-front,axis=1)])
    return np.r_[distances/.03,(mapped[-2:]-np.asarray(columns)[:,:2]).ravel()/.03]

origin,forward,right,up,f,cy=camera(np.array([11.2,6.84,.737,-2.43,0,300,450]))
P=np.array([f*right+700*forward,-f*up+cy*forward,forward])
H=np.c_[P[:,:2],-P@origin]
normal=np.array([[500,0,700],[0,500,467],[0,0,1.]])
initial=np.linalg.inv(H)@normal
initial/=initial[2,2]
hfit=least_squares(ground_residual,initial.ravel()[:8],max_nfev=4000)
print('ground',hfit.success,hfit.cost,ground_residual(hfit.x).tolist())
print('mapped',ground_fit(hfit.x).tolist())
(TICKET/"🗑️generated"/"hall-homography-r31.json").write_text(json.dumps({'matrix':np.r_[hfit.x,1.].reshape(3,3).tolist(),'residuals':ground_residual(hfit.x).tolist(),'mapped':ground_fit(hfit.x).tolist(),'status':'inferred ground-plane registration; source outline assumed'},indent=2))
