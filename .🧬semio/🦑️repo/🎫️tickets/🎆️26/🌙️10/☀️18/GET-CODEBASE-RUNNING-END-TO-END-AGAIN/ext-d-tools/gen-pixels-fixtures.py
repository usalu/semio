import json,os
import numpy as np
f32=np.float32
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🔲️pixels/"
def lcg(seed,n):
    s=seed;out=[]
    for _ in range(n):
        s=(s*6364136223846793005+1442695040888963407)&0xFFFFFFFFFFFFFFFF
        out.append(s>>56)
    return out
def weights(src,dst):
    ratio=f32(src)/f32(dst);scale=max(ratio,f32(1.0));rows=[]
    for out in range(dst):
        c=(f32(out)+f32(0.5))*ratio
        left=int(np.floor(c-scale));left=min(max(left,0),src-1)
        right=int(np.ceil(c+scale));right=min(max(right,left+1),src)
        c=c-f32(0.5);ws=[];total=f32(0)
        for i in range(left,right):
            d=abs((f32(i)-c)/scale)
            w=f32(1.0)-d if d<f32(1.0) else f32(0.0)
            ws.append(w);total=total+w
        rows.append((left,[w/total for w in ws]))
    return rows
def triangle(img,w,h,dw,dh):
    if (dw,dh)==(w,h): return list(img)
    V=weights(h,dh);H=weights(w,dw)
    mid=[[f32(0)]*4 for _ in range(w*dh)]
    for r,(top,ws) in enumerate(V):
        for c in range(w):
            s=[f32(0)]*4
            for o,wt in enumerate(ws):
                at=((top+o)*w+c)*4
                for ch in range(4): s[ch]=s[ch]+f32(img[at+ch])*wt
            mid[r*w+c]=s
    out=[]
    for r in range(dh):
        for c,(left,ws) in enumerate(H):
            s=[f32(0)]*4
            for o,wt in enumerate(ws):
                m=mid[r*w+left+o]
                for ch in range(4): s[ch]=s[ch]+m[ch]*wt
            for ch in range(4):
                v=min(max(float(s[ch]),0.0),255.0)
                out.append(int(np.floor(v+0.5)) if True else 0)
    return out
cases=[]
for name,(w,h,dw,dh,seed) in {"downscale-3x-wide":(9,4,3,4,1),"downscale-both-uneven":(7,5,3,2,2),"upscale":(3,2,7,5,3),"one-pixel-column":(5,6,1,6,4),"identity":(4,4,4,4,5),"large-reduction":(16,12,2,3,6)}.items():
    img=lcg(seed,w*h*4)
    cases.append({"name":name,"width":w,"height":h,"targetWidth":dw,"targetHeight":dh,"pixels":img,"expected":triangle(img,w,h,dw,dh)})
os.makedirs(P+"🧪️tests/🧫️fixtures/📉️triangle-resize",exist_ok=True)
json.dump(cases,open(P+"🧪️tests/🧫️fixtures/📉️triangle-resize/🔣️.json","w",encoding="utf8"),indent=1)
# orientation
def apply(img,w,h,o):
    a=np.array(img,dtype=np.uint8).reshape(h,w,4)
    t={1:lambda a:a,2:lambda a:a[:,::-1],3:lambda a:a[::-1,::-1],4:lambda a:a[::-1],5:lambda a:a.transpose(1,0,2),6:lambda a:np.rot90(a,-1),7:lambda a:np.rot90(a,-1)[:,::-1] if False else np.rot90(a,1)[:,::-1],8:lambda a:np.rot90(a,1)}[o](a)
    return t.shape[1],t.shape[0],t.reshape(-1).tolist()
src=lcg(9,3*2*4)
ap=[]
for o in range(1,9):
    w2,h2,px=apply(src,3,2,o);ap.append({"exif":o,"width":3,"height":2,"pixels":src,"expected":{"width":w2,"height":h2,"pixels":px}})
def tiff(big,entries,value):
    import struct
    e=">" if big else "<"
    head=(b"MM\x00*" if big else b"II*\x00")+struct.pack(e+"I",8)
    body=struct.pack(e+"H",len(entries))
    for tag,fmt,count,val in entries: body+=struct.pack(e+"HHIHH",tag,fmt,count,val,0)
    return (head+body).hex()
tv=[{"name":"little-endian orientation 6","hex":tiff(False,[(0x0112,3,1,6)],6),"expected":6},
    {"name":"big-endian orientation 3","hex":tiff(True,[(0x010f,2,1,0),(0x0112,3,1,3)],3),"expected":3},
    {"name":"orientation 9 refused","hex":tiff(False,[(0x0112,3,1,9)],9),"expected":None},
    {"name":"orientation 0 refused","hex":tiff(False,[(0x0112,3,1,0)],0),"expected":None},
    {"name":"no orientation entry","hex":tiff(True,[(0x010f,2,1,0)],0),"expected":None},
    {"name":"wrong format long","hex":tiff(False,[(0x0112,4,1,6)],6),"expected":None},
    {"name":"count two","hex":tiff(False,[(0x0112,3,2,6)],6),"expected":None},
    {"name":"bad magic","hex":"4949 2b00 08000000 0000".replace(" ",""),"expected":None},
    {"name":"truncated directory","hex":tiff(False,[(0x0112,3,1,6)],6)[:-10],"expected":None},
    {"name":"directory offset beyond block","hex":(b"II*\x00"+(1000).to_bytes(4,"little")).hex(),"expected":None}]
os.makedirs(P+"🖼️image/📥️decode/🧪️tests/🧫️fixtures/🧭️orientation",exist_ok=True)
json.dump({"tiff":tv,"apply":ap},open(P+"🖼️image/📥️decode/🧪️tests/🧫️fixtures/🧭️orientation/🔣️.json","w",encoding="utf8"),indent=1)
print(len(cases),len(tv),len(ap))
