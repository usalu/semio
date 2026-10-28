"""🔎 Sample current title-free native bodies without rendering."""
import pathlib,json,hashlib,importlib.util,time
import av
from PIL import Image,ImageDraw
t=pathlib.Path(__file__).parent
spec=importlib.util.spec_from_file_location("iv",t/"inspection-verify.py")
v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
plan=v.plan(json.loads(v.MANIFEST.read_text(encoding="utf-8"))["videos"],t/"🗑️generated/i/de/spectator")
wanted={1,6,9,10,31,32,66,67,68}
paths={}
for _,entries in plan.values():
 for e in entries:
  p=e["source"];n=int(p.parents[1].name)
  if n in wanted:paths[n]=p
out=t/"🗑️generated/inspection-cache/title-free";out.mkdir(parents=True,exist_ok=True)
rows=[];accepted=[]
for n,p in sorted(paths.items()):
 cert=p.with_suffix(".render.json")
 if not cert.is_file():rows.append({"ordinal":n,"status":"missing certificate"});continue
 raw=cert.read_bytes();state=json.loads(raw)
 stale=[name for name,digest in state["sources"].items() if not pathlib.Path(name).is_file() or v.sha(pathlib.Path(name))!=digest]
 if stale:rows.append({"ordinal":n,"status":"stale","paths":stale});continue
 stat=(p.stat().st_size,p.stat().st_mtime_ns)
 with av.open(str(p)) as c:
  stream=c.streams.video[0];duration=float(stream.duration*stream.time_base)
 times=[min(2,duration/4),duration/2,max(0,duration-2)]
 row=Image.new("RGB",(1440,296),"#222222");draw=ImageDraw.Draw(row)
 for j,seconds in enumerate(times):
  with av.open(str(p)) as c:
   stream=c.streams.video[0];stream.codec_context.thread_count=1
   c.seek(int(seconds/stream.time_base),stream=stream,backward=True)
   f=next(f for f in c.decode(stream) if float(f.pts*f.time_base)>=seconds)
   im=f.to_image();assert im.size==(480,270)
   file=out/(str(n)+"-"+str(j)+".png");im.save(file);row.paste(im,(j*480,0))
   draw.text((j*480+5,275),"source "+str(n)+" @ "+str(round(float(f.pts*f.time_base),2))+"s",fill="white")
 assert raw==cert.read_bytes() and stat==(p.stat().st_size,p.stat().st_mtime_ns)
 assert all(v.sha(pathlib.Path(name))==digest for name,digest in state["sources"].items())
 accepted.append(row);rows.append({"ordinal":n,"status":"current sampled","scene":p.stem,"source_bindings":len(state["sources"]),"duration":duration,"sample_seconds":times})
for k in range(0,len(accepted),3):
 sheet=Image.new("RGB",(1440,296*len(accepted[k:k+3])),"#222222")
 for j,row in enumerate(accepted[k:k+3]):sheet.paste(row,(0,j*296))
 sheet.save(out/("sheet-"+str(k//3+1)+".png"))
(t/"inspection-title-free-body-samples.md").write_text("# Current Title-Free Body Samples\n\nRead-only decoded samples. Every source certificate binding matched disk before and after sampling; certificate and movie stat stayed unchanged. No render started.\n\n"+json.dumps(rows,ensure_ascii=False,indent=2)+"\n\nBody sampling only; chapter envelopes, full movies, audio and 55-media delivery remain unaccepted.\n",encoding="utf-8")
print("[DEBUG] "+json.dumps(rows,ensure_ascii=False),flush=True)
