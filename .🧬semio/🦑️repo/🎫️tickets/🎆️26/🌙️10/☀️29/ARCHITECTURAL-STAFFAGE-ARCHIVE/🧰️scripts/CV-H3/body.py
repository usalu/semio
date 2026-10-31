import copy
from kit import *
HAIR_V1="M 0.2151 -0.0893 C 0.2565 -0.0933 0.2926 -0.0516 0.2906 -0.005 C 0.2885 0.0615 0.2503 0.1072 0.1923 0.1042 C 0.1551 0.1112 0.122 0.1291 0.0879 0.139 C 0.062 0.1489 0.0538 0.1191 0.0776 0.0993 C 0.0982 0.0844 0.1158 0.0596 0.1551 0.0516 C 0.2027 0.0457 0.2337 0.0079 0.2378 -0.0397 C 0.2389 -0.0655 0.2337 -0.0834 0.2151 -0.0893 Z"
HAIR_CAP=[(0.115,0.07),(0.16,0.10),(0.225,0.10),(0.28,0.06),(0.2926,-0.005),(0.255,-0.075),(0.2151,-0.0893),(0.2337,-0.04),(0.215,0.015),(0.15,0.052)]
HAIR_200=[(0.095,0.10),(0.15,0.112),(0.23,0.108),(0.29,0.05),(0.2926,-0.005),(0.255,-0.075),(0.2151,-0.0893),(0.2337,-0.04),(0.205,0.01),(0.14,0.05),(0.10,0.072)]
FOOT_SHOE_100="M -0.068 -0.0827 C -0.074 -0.052 -0.069 -0.012 -0.056 0.034 C -0.02 0.040 0.012 0.040 0.034 0.034 C 0.062 0.020 0.094 0.004 0.1344 -0.0153 L 0.1344 -0.0827 Z"
TOE_SHOE_100="M 0 0.0674 C 0.012 0.0626 0.022 0.058 0.0295 0.0538 C 0.064 0.0376 0.0952 0.0290 0.0952 0.0124 C 0.0952 0.0031 0.089 0 0.0776 0 L 0 0 Z"
FOOT_SHOE_200="M -0.068 -0.0827 C -0.074 -0.05 -0.068 -0.01 -0.056 0.034 C -0.02 0.040 0.03 0.036 0.07 0.018 C 0.1 0.004 0.12 -0.01 0.1344 -0.0153 L 0.1344 -0.0827 Z"
TOE_SHOE_200="M 0 0.0674 C 0.035 0.052 0.0952 0.030 0.0952 0.0124 C 0.0952 0.0031 0.089 0 0.0776 0 L 0 0 Z"
THUMB_L="M -0.0039 0.0235 C 0.0255 0.049 0.0646 0.0705 0.096 0.0803 C 0.1068 0.0833 0.1146 0.0735 0.1068 0.0637 C 0.0901 0.049 0.0686 0.0392 0.0451 0.0215 Z"

SHOE_FULL="M -0.068 -0.0827 C -0.074 -0.05 -0.068 -0.01 -0.056 0.034 C -0.02 0.040 0.03 0.036 0.07 0.022 C 0.13 0.012 0.2296 0.020 0.2296 -0.0827 Z"
def side_z(sd): return 1.0 if sd=='l' else 6.0

def build_body(scale,P):
    s=P['s']; W=P['W']; out=[]
    skin=P.get('skin_tone',0.8)
    R=lambda k:P[k]
    tg=P['torso']; pg=P['pelvis']
    # ---------------------------------------------------------------- 1:100 and 1:200
    if scale in (100,200):
        c100=(scale==100)
        # head and hair
        out.append(shp("head","head","body",5,True,skin,P_(cpath(HEAD_100 if c100 else HEAD_200,s)),region=P['head']))
        h=P['hair']
        out.append(shp("hair","head","body",5.4,True,h['tone'],P_(P['hair_d'][scale] if 'hair_d' in P else cpath(HAIR_CAP if c100 else HAIR_200,s)),region=h['region']))
        out.append(shp("torso","torso","body",3.2,True,tg['tone'],P_(profile(P.get('torso_st',TORSO),s)),region=tg['region']))
        out.append(shp("pelvis","pelvis","body",3.05,True,pg['tone'],P_(profile(PELVIS,s)),region=pg['region']))
        for sd in "lr":
            z=side_z(sd); u=P['sleeve']; fs=P.get('fsleeve')
            # arm
            if u['x1'] is None:
                out.append(shp(f"upper-arm-{sd}",f"upper-arm-{sd}","body",z+.4,True,u['tone'],P_(profile(UARM,s,cap1='round')),region=u['region']))
            else:
                out.append(shp(f"upper-arm-{sd}",f"upper-arm-{sd}","body",z+.38,True,skin,P_(profile(UARM,s,cap1='round')),region=P['arm_skin']))
                out.append(shp(f"sleeve-{sd}",f"upper-arm-{sd}","body",z+.4,True,u['tone'],P_(profile(sub(UARM,0,u['x1'],0.004),s)),region=u['region']))
            if scale==100 or True:
                if fs:
                    out.append(shp(f"forearm-{sd}",f"forearm-{sd}","body",z+.42,True,fs['tone'],P_(profile(FARM,s,'round','round')),region=fs['region']))
                else:
                    out.append(shp(f"forearm-{sd}",f"forearm-{sd}","body",z+.42,True,skin,P_(profile(FARM,s,'round','round')),region=P['arm_skin']))
            if c100:
                out.append(shp(f"hand-{sd}",f"hand-{sd}","body",z+.45,True,skin,P_(profile(HAND,s)),region=P['hand']))
                out.append(shp(f"thumb-{sd}",f"hand-{sd}","body",z+.47,True,skin,P_(xform_d(THUMB_L,(s,0,0,s,0,0))),region=P['hand']))
            else:
                out.append(shp(f"hand-{sd}",f"hand-{sd}","body",z+.45,True,skin,P_(profile(sub(HAND,-0.012,0.1861),s)),region=P['hand']))
            # leg
            tgl=P['thigh_g']
            if tgl['x1'] is None:
                out.append(shp(f"thigh-{sd}",f"thigh-{sd}","body",z,True,tgl['tone'],P_(profile(THIGH,s,cap1='round')),region=tgl['region']))
            else:
                tb=P.get('thigh_base')
                out.append(shp(f"thigh-skin-{sd}",f"thigh-{sd}","body",z-.02,True,tb['tone'] if tb else skin,P_(profile(THIGH,s,cap1='round')),region=tb['region'] if tb else P['leg_skin']))
                out.append(shp(f"thigh-{sd}",f"thigh-{sd}","body",z,True,tgl['tone'],P_(profile(sub(THIGH,0,tgl['x1'],tgl.get('grow',0.008)),s)),region=tgl['region']))
            sg=P.get('shank_g')
            if sg:
                out.append(shp(f"shank-{sd}",f"shank-{sd}","body",z+.02,True,sg['tone'],P_(profile(SHANK,s,'round')),region=sg['region']))
            else:
                out.append(shp(f"shank-{sd}",f"shank-{sd}","body",z+.02,True,skin,P_(profile(SHANK,s,'round')),region=P['leg_skin']))
                so=P.get('sock')
                if so and c100:
                    out.append(shp(f"sock-{sd}",f"shank-{sd}","body",z+.05,True,so['tone'],P_(profile(sub(SHANK,so['x0'],0.4343,0.003),s)),region=so['region']))
            sh=P['shoe']
            out.append(shp(f"shoe-{sd}",f"foot-{sd}","body",z+.2,True,sh['tone'],P_(xform_d(FOOT_SHOE_100 if c100 else FOOT_SHOE_200,(s,0,0,s,0,0))),region=sh['region']))
            out.append(shp(f"shoe-toe-{sd}",f"toe-{sd}","body",z+.2,True,sh['tone'],P_(xform_d(TOE_SHOE_100 if c100 else TOE_SHOE_200,(s,0,0,s,0,0))),region=sh['region']))
    return out

def v1_shapes(a1,drop=()):
    """1:50: the v1 outlines are the raw material of the 1:50 drawing; only `lod` is removed."""
    out=[]
    for sh in a1['geometry']['elevation']:
        if sh['id'] in drop: continue
        n={k:v for k,v in sh.items() if k!='lod'}
        out.append(n)
    return out

def merged500(P,pose_bones,rep_regions):
    """1:500 composed forms: head, trunk and pelvis are one path; every limb keeps its joints (upper arm, forearm with hand, thigh, shank, shoe, toe)."""
    s=P['s']; W=P['W']; out=[]; tone=P.get('tone500',0.0)
    ref=lambda d,frm,to:reframe(d,frm,to,W)
    head=cpath(HEAD_200,s); hair=cpath(HAIR_200,s)
    figure=" ".join([profile(P.get('torso_st',TORSO),s),ref(profile(PELVIS,s),"pelvis","torso"),ref(head,"head","torso"),ref(hair,"head","torso")])
    out.append(shp("figure","torso","body",3.2,True,tone,P_(figure),regions=P['reg_fig']))
    for sd in "lr":
        z=side_z(sd)
        out.append(shp(f"upper-arm-{sd}",f"upper-arm-{sd}","body",z+.4,True,tone,P_(profile(UARM,s,cap1='round')),regions=P['reg_uarm']))
        farm=" ".join([profile(FARM,s,'round'),ref(profile(sub(HAND,-0.012,0.1861),s),f"hand-{sd}",f"forearm-{sd}")])
        out.append(shp(f"forearm-{sd}",f"forearm-{sd}","body",z+.42,True,tone,P_(farm),regions=P['reg_farm_r'] if sd=='r' and 'reg_farm_r' in P else P['reg_farm']))
        out.append(shp(f"thigh-{sd}",f"thigh-{sd}","body",z,True,tone,P_(profile(P.get('thigh500',THIGH),s,cap1='round')),regions=P['reg_thigh']))
        out.append(shp(f"shank-{sd}",f"shank-{sd}","body",z+.02,True,tone,P_(profile(SHANK,s,'round')),regions=P['reg_shank']))
        out.append(shp(f"shoe-{sd}",f"foot-{sd}","body",z+.2,True,tone,P_(xform_d(FOOT_SHOE_200,(s,0,0,s,0,0))),regions=["shoes"]))
        out.append(shp(f"toe-{sd}",f"toe-{sd}","body",z+.2,True,tone,P_(xform_d(TOE_SHOE_200,(s,0,0,s,0,0))),regions=["shoes"]))
    return out

def v50(a1,P,drop_ids,add_ids):
    """1:50 = the v1 detail drawing (face, hair, hands, shoes, seams, accessories) on the smooth-limbed kit bodies of the 1:100 drawing."""
    kit={sh['id']:sh for sh in build_body(100,P)}
    out=[{k:v for k,v in sh.items() if k!='lod'} for sh in a1['geometry']['elevation'] if sh['id'] not in drop_ids]
    for i in add_ids:
        out.append(copy.deepcopy(kit[i]))
    return out
