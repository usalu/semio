import base64,json,pathlib,numpy as np
root=pathlib.Path("✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations")
def read(p):return json.loads(p.read_text(encoding="utf8"))
def pixels(layer):return np.full((256,256,4),255,dtype=np.uint8) if not layer["pixels"] else np.frombuffer(base64.b64decode(layer["pixels"]),dtype=np.uint8).reshape(256,256,4)
y,x=np.indices((256,256))
for case in ["🖌️dabs","🧽️erases"]:
 p=root/"🖌️apply-paint-stroke"/case
 before=read(p/"📸️snapshot/⬅️before/🔣️.json")["objects"][0]["paintLayers"][0]
 expected=pixels(before).copy();stroke=read(p/"🦠️mutation/🔣️.json")["ApplyPaintStroke"]
 for u,v in stroke["points"]:
  cx=int(np.floor(u*255+0.5));cy=int(np.floor((1-v)*255+0.5));distance=np.hypot(x-cx,y-cy);mask=distance<=stroke["radius"]
  if stroke["eraser"]:
   alpha=np.floor((stroke["hardness"]+(1-stroke["hardness"])*(1-distance/stroke["radius"]))*stroke["opacity"]*255+0.5)
   expected[:,:,3][mask]=np.maximum(0,expected[:,:,3][mask].astype(np.int32)-alpha[mask].astype(np.int32))
  else:expected[mask,:3]=[200,30,30]
 actual=pixels(read(p/"📸️snapshot/➡️after/🔣️.json")["objects"][0]["paintLayers"][0]);np.testing.assert_array_equal(actual,expected)
 applied=pixels(before).copy().reshape(-1);runs=read(p/"🔺️diff/🔣️.json")["objects"]["modified"][0]["patch"]["paintLayers"]["edits"][0]["runs"]
 for run in runs:
  data=np.frombuffer(base64.b64decode(run["bytes"]),dtype=np.uint8);start=run["offset"];applied[start:start+data.size]=data
 np.testing.assert_array_equal(applied.reshape(256,256,4),expected)
 print(ascii(case)+": independent NumPy geometry and sparse writes agree")
