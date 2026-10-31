from pathlib import Path
import subprocess,json,time
p=Path(__file__).parent/'🗑️generated/drawing-native-work-loan-check1'
cmd=['repo','--service=off','cargo','--store=shared','--configuration=prototype','--','check','--manifest-path',str(Path.cwd()/'🧰️framework/Cargo.toml'),'-p','semio-framework-value-native-decoding','--lib']
with (p/'native.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
(p/'result.json').write_text(json.dumps({'command':cmd,'outerExit':r.returncode})+'\n')
print('[DEBUG] Native work port production check terminal',r.returncode,flush=True)
