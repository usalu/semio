from pathlib import Path
import subprocess,json,time
p=Path(__file__).parent/'🗑️generated/drawing-native-work-loan-owning1'
cmd=['repo','--service=off','cargo','--store=shared','--configuration=prototype','--','test','--manifest-path',str(Path.cwd()/'🧰️framework/Cargo.toml'),'-p','semio-framework-value-native-decoding','--all-targets','--','--nocapture']
with (p/'native.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
(p/'result.json').write_text(json.dumps({'command':cmd,'outerExit':r.returncode})+'\n')
print('[DEBUG] Native work port owning terminal',r.returncode,flush=True)
