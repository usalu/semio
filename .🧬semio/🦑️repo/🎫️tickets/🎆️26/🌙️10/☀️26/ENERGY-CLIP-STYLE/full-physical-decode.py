"""🎞️ Decode the complete final publication once through registered FFmpeg."""
from pathlib import Path
import importlib,os,subprocess,sys
ticket=Path(__file__).resolve().parent
workspace=next(path for path in ticket.parents if (path/'🧰️framework').is_dir())
sys.path.insert(0,str(workspace))
runtime=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️')
movie=ticket/'🗑️generated/full-physical/de/spectator/rendered/Full_Physical_Fundamentals_spectator_de_640x360p6.mp4'
environment={**os.environ,'SEMIO_TOOL_FFMPEG':runtime.ffmpeg_command()[0]}
command=['C:/Users/Ueli/AppData/Local/semio/bin/repo.exe','tools','exec','ffmpeg','--light','--lang=en','--','-hide_banner','-v','error','-xerror','-nostdin','-i',str(movie),'-map','0:v:0','-map','0:a:0','-f','null','-']
result=subprocess.run(command,cwd=workspace,env=environment,capture_output=True,text=True,encoding='utf-8',errors='replace')
log=ticket/'🗑️generated/full-physical/full-decode.log'
log.write_text('[DEBUG] Registered FFmpeg complete video and audio decode\n[DEBUG] returncode='+str(result.returncode)+'\n'+result.stdout+result.stderr,encoding='utf-8')
report=ticket/'full-physical-render.md'
with report.open('a',encoding='utf-8') as stream:
    stream.write('\n## Independent FFmpeg Decode\n\nOne complete error-only FFmpeg video and audio decode ran through the registered repo tools interface with -xerror. Return code: '+str(result.returncode)+'. Diagnostic output bytes: '+str(len(result.stdout.encode())+len(result.stderr.encode()))+'.\n')
print('[DEBUG] Full FFmpeg decode returncode='+str(result.returncode))
if result.stdout or result.stderr:
    print(result.stdout+result.stderr)
raise SystemExit(result.returncode)
