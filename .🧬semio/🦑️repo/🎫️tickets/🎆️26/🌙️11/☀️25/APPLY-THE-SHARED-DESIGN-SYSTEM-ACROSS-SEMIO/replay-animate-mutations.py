"""🧪 Replay Animate mutations using the native compiler receipt and frozen libraries."""
import json, os, pathlib, shlex, subprocess, sys
P=pathlib.Path
d=P(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
r=d.parent/"animate-public-current/frozen-deps"
runtime=d/"t"
output=runtime/"mut-test"
output.mkdir(exist_ok=True)
name="semio_s_artifact_animate_presentation_mutations"
line=next(l for l in d.joinpath("native-final.log").read_text().splitlines() if "Running "+chr(96) in l and "--crate-name "+name+" " in l and " --test " in l)
old=shlex.split(line.split(chr(96))[1])
workspace=P("✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation").resolve()
manifest=(workspace/old[4]).parent
assert (manifest/"Cargo.toml").is_file()
args=[old[0]]
i=1
while i<len(old):
 a=old[i]
 b=old[i+1] if i+1<len(old) else ""
 if a=="--out-dir":
  args.extend([a,str(output)])
  i+=2
 elif a=="-L" and b.startswith("dependency="):
  i+=2
 elif a=="-C" and b.startswith("incremental="):
  i+=2
 elif a=="-Z" and b.startswith("threads="):
  i+=2
 elif a=="--extern":
  alias,path=b.split("=",1)
  actual=r/P(path).name
  assert actual.is_file(), actual
  args.extend([a, f"{alias}={actual}"])
  i+=2
 else:
  args.append(a)
  i+=1
args.extend(["-L","dependency="+str(r),"-Z","threads=2","-Z","no-parallel-backend"])
env={**os.environ,"CARGO_MANIFEST_DIR":str(manifest),"CARGO_PKG_NAME":name.replace("_","-"),"CARGO_PKG_VERSION":"0.1.0","CARGO_CRATE_NAME":name,"TMPDIR":str(runtime)}
with (d/"mut-replay.log").open("w") as log:
 result=subprocess.run(args,cwd=workspace,env=env,stderr=log,stdout=log)
 if result.returncode:
  print((d/"mut-replay.log").read_text()[-4000:])
  sys.exit(result.returncode)
 executable=next(p for p in output.glob(name+"-*") if p.is_file() and not p.suffix and os.access(p,os.X_OK))
 print(executable,flush=True)
 for pattern in ("compile_presentation_site_writes_static_bundle","compile_scene_to_assets_writes_mp4","title_cards_match_the_neutral_xml_oracle"):
  result=subprocess.run([str(executable),pattern,"--nocapture"],cwd=runtime,env=env,stderr=log,stdout=log)
  print(pattern, result.returncode,flush=True)
 result=subprocess.run([str(executable),"--test-threads=2"],cwd=runtime,env=env,stderr=log,stdout=log)
sys.exit(result.returncode)
