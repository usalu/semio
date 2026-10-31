"""🎬 Replay native pairs in fresh scene workers with two-language cache reuse."""
import argparse
import ast
import gc
import importlib.metadata
import importlib.util
import json
import os
from pathlib import Path
import platform
import runpy
import subprocess
import sys
import tempfile

sys.dont_write_bytecode=True
sys.stdout.reconfigure(encoding="utf-8",errors="backslashreplace",line_buffering=True)
sys.stderr.reconfigure(encoding="utf-8",errors="backslashreplace",line_buffering=True)
TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
PLAN=json.loads((TICKET/'native-pairs.json').read_text(encoding='utf-8'))
RUNNER=TICKET/'validate-scenes.py'
parser=argparse.ArgumentParser()
parser.add_argument('--series',choices=tuple(PLAN['series']),required=True)
parser.add_argument('--worker-scene')
args=parser.parse_args()
if args.worker_scene and args.worker_scene not in PLAN['series'][args.series]['scenes']:
    raise ValueError('Worker scene is outside the native plan')
SPEC=importlib.util.spec_from_file_location('batch_provenance',TICKET/'scene-provenance.py')
PROVENANCE=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROVENANCE)
registry=ast.parse((TICKET/'verify-native-ledger.py').read_text(encoding='utf-8'))
expected=ast.literal_eval(next(node.value for node in registry.body if isinstance(node,ast.Assign) and any(isinstance(target,ast.Name) and target.id=='SERIES' for target in node.targets)))
for series,row in PLAN['series'].items():
    if (row['scenes'],row['contracts'])!=(expected[series][1],expected[series][2]):
        raise ValueError('Native plan and acceptance registry disagree: '+series)
if PLAN['languages']!=['en','de']:
    raise ValueError('Both native languages are required')
output=TICKET/'🗑️generated'/('native-batch-'+args.series+('-'+args.worker_scene if args.worker_scene else ''))
output.mkdir(parents=True,exist_ok=True)


def replay(arguments):
    """▶️ Give each native pair fresh command arguments and temporary-directory selection."""
    previous=sys.argv
    sys.argv=[str(RUNNER),*arguments]
    tempfile.tempdir=None
    code=0
    try:
        runpy.run_path(str(RUNNER),run_name='__main__')
    except SystemExit as stopped:
        code=stopped.code
    finally:
        sys.argv=previous
        gc.collect()
    if code not in (None,0):
        raise RuntimeError('Native replay failed: '+str(arguments))


def repo_child(target,label,arguments):
    """🧑‍🏭 Bound rich drawing caches to one owned repo worker and retain its complete output."""
    command=['repo','tools','exec','uv','--','run','--python',sys.executable,'--no-project','python','-B',str(TICKET/'verify.py'),str(target.relative_to(ROOT)),'--label',label,*arguments]
    print('[DEBUG] Native worker '+label,flush=True)
    environment={**os.environ,'PYTHONIOENCODING':'utf-8','PYTHONUTF8':'1','PYTHONDONTWRITEBYTECODE':'1'}
    with (output/(label+'.log')).open('w',encoding='utf-8') as log, subprocess.Popen(command,cwd=ROOT,env=environment,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,encoding='utf-8',errors='backslashreplace') as child:
        for line in child.stdout:
            log.write(line)
            log.flush()
            print(line,end='',flush=True)
        code=child.wait()
    if code:
        raise RuntimeError('Native worker failed: '+label+' exit '+str(code))


def current_sources(source):
    """🔐 Read the complete batch baseline again without reloading cached code."""
    paths=[Path(key.removeprefix('@external:')) if key.startswith('@external:') else ROOT/key for key in source]
    return PROVENANCE.fingerprints(ROOT,paths)


def runtime_identity(packages):
    """🧰 Bind every worker to the same interpreter and dependency versions."""
    versions={}
    for name in packages:
        try:
            versions[name]=importlib.metadata.version(name)
        except importlib.metadata.PackageNotFoundError:
            versions[name]=None
    return {'python':sys.version,'executable':sys.executable,'platform':platform.platform(),'packages':versions}


def read_result(name):
    """🧾 Reject failed, incomplete or changed-source native records immediately."""
    record=json.loads((TICKET/'🗑️generated'/name/'result.json').read_text(encoding='utf-8'))
    if not record.get('success') or not record.get('source_unchanged') or not record.get('source_fingerprints') or record['source_fingerprints']!=record.get('source_fingerprints_end') or record.get('runtime_identity')!=record.get('runtime_identity_end') or record.get('failures') or record.get('errors'):
        raise RuntimeError('Unaccepted native result: '+name)
    return record


def checkpoint(source,runtime,configuration=None):
    """🛑 Stop on disk, interpreter or rendering-config drift before using any warm cache."""
    if current_sources(source)!=source or runtime_identity(runtime['packages'])!=runtime:
        raise RuntimeError('Frozen native batch inputs changed; remaining work cancelled')
    if configuration is not None:
        import manim
        if {key:str(getattr(manim.config,key,None)) for key in configuration}!=configuration:
            raise RuntimeError('Native worker rendering configuration changed')


if not args.worker_scene:
    repo_child(RUNNER,'nc-'+args.series,['--series',args.series,'--contracts'])
contracts=read_result('scenes-'+args.series+'-all-en-de')
if contracts.get('tests')!=PLAN['series'][args.series]['contracts']:
    raise RuntimeError('Native presentation contract count differs')
baseline=contracts['source_fingerprints']
runtime={key:contracts['runtime_identity'][key] for key in ('python','executable','platform','packages')}
checkpoint(baseline,runtime)
report={'series':args.series,'source_fingerprints':baseline,'runtime_identity':runtime,'worker_scope':args.worker_scene,'pairs':[],'complete':False}
if args.worker_scene:
    import manim
    configuration={key:str(getattr(manim.config,key,None)) for key in contracts['render_config']}
    for language in PLAN['languages']:
        checkpoint(baseline,runtime,configuration)
        print('[DEBUG] Native pair '+args.worker_scene+' '+language,flush=True)
        replay(['--series',args.series,'--scenes',args.worker_scene,'--languages',language])
        record=read_result('scenes-'+args.series+'-'+args.worker_scene+'-'+language)
        if record.get('tests')!=1 or any(baseline.get(key)!=value for key,value in record['source_fingerprints'].items()):
            raise RuntimeError('Native pair escaped the frozen batch dependency baseline')
        checkpoint(baseline,runtime,configuration)
        report['pairs'].append({'scene':args.worker_scene,'language':language,'started_at':record['started_at'],'dependency_count':len(record['source_fingerprints'])})
        (output/'result.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
else:
    for scene in PLAN['series'][args.series]['scenes']:
        checkpoint(baseline,runtime)
        repo_child(Path(__file__),'ns-'+args.series+'-'+scene,['--series',args.series,'--worker-scene',scene])
        checkpoint(baseline,runtime)
        for language in PLAN['languages']:
            record=read_result('scenes-'+args.series+'-'+scene+'-'+language)
            report['pairs'].append({'scene':scene,'language':language,'started_at':record['started_at'],'dependency_count':len(record['source_fingerprints'])})
        (output/'result.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
report['complete']=True
(output/'result.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] Native replay completed '+args.series+' '+str(len(report['pairs']))+' pairs',flush=True)
