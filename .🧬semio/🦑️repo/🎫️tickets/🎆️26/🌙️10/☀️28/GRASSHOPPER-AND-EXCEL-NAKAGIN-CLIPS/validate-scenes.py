"""🎬 Validate requested native scene lifecycles with progress and ticket-local render output."""
import argparse
import ast
from datetime import datetime, timezone
import hashlib
import importlib.util
import importlib.metadata
import inspect
import platform
import json
import os
from pathlib import Path
import sys
sys.dont_write_bytecode=True
import unittest
import traceback
sys.stdout.reconfigure(errors="backslashreplace")
parser=argparse.ArgumentParser()
parser.add_argument('--series',choices=('excel','grasshopper'),required=True)
parser.add_argument('--scenes',default='')
parser.add_argument('--languages',default='en,de')
parser.add_argument('--contracts',action='store_true')
parser.add_argument('--provenance-only',action='store_true')
args=parser.parse_args()
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
folder={'excel':'📊️excel','grasshopper':'🦗️grasshopper'}[args.series]
SERIES_FOLDER=folder
output=Path(__file__).parent/'🗑️generated'/('scenes-'+args.series+'-'+(args.scenes or 'all').replace(',','-')+'-'+args.languages.replace(',','-'))
output.mkdir(parents=True,exist_ok=True)
for key in ('SEMIO_TEACHING_OUTPUT_ROOT','TMPDIR','TEMP','TMP'):
    os.environ[key]=str(output)
sys.path.insert(0,str(root))
import manim
p=root/'🎓️teaching/🏛️architecture'/folder/'🎬️clip/🧪️tests/🎭️presentation/🐍️.py'
started_at=datetime.now(timezone.utc).isoformat()
spec=importlib.util.spec_from_file_location('scene_contracts_'+args.series,p)
module=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=module
spec.loader.exec_module(module)

provenance_path=Path(__file__).with_name('scene-provenance.py')
provenance_spec=importlib.util.spec_from_file_location('native_scene_provenance',provenance_path)
provenance=importlib.util.module_from_spec(provenance_spec)
provenance_spec.loader.exec_module(provenance)
entries=module.discover()
requested=set(filter(None,args.scenes.split(',')))
if args.series=='grasshopper':
    all_scene_paths={Path(entry[2]).resolve() for entry in entries}
    selected_entries=[entry for entry in entries if args.contracts or not requested or entry[1].__name__ in requested or entry[0] in requested]
    selected_paths={Path(entry[2]).resolve() for entry in selected_entries}
    visuals=module.VISUALS
else:
    all_scene_paths={Path(entry['script']).resolve() for entry in entries}
    selected_entries=[entry for entry in entries if args.contracts or not requested or entry['scene'] in requested or entry['folder'] in requested]
    selected_paths={Path(entry['script']).resolve() for entry in selected_entries}
    for entry in selected_entries:
        module.scene_class(entry)
    visuals=importlib.import_module(module.SERIES+'.🎨️visuals.🐍️')
if not selected_paths:
    raise ValueError('No selected scene dependencies found')
ignored_scenes=all_scene_paths-selected_paths

def registry_inputs(series):
    """📚 Declare known lazy registry data before capturing the native baseline."""
    if series!='excel':
        return ()
    manifest=Path(__file__).with_name('excel-registry-inputs.json')
    schema=manifest.with_name('excel-registry-inputs.schema.json')
    paths=tuple(root/path for path in json.loads(manifest.read_text(encoding='utf-8')))
    if len(set(paths))!=len(paths) or any(not path.is_file() or not path.resolve().is_relative_to(root) for path in paths):
        raise ValueError('Registry inputs must name unique existing repository files')
    return (manifest,schema,*paths)

def dependency_files():
    clip=root/'🎓️teaching/🏛️architecture'/SERIES_FOLDER/'🎬️clip'
    manifest=[*visuals.source_files(),*registry_inputs(args.series)]
    imported=[]
    for loaded in tuple(sys.modules.values()):
        filename=getattr(loaded,'__file__',None)
        if not filename:
            continue
        path=Path(filename).resolve()
        if path.is_file() and path.is_relative_to(root) and path not in ignored_scenes and not {'site-packages','dist-packages','__pycache__'}&set(path.parts):
            imported.append(path)
            source_files=getattr(loaded,'source_files',None)
            if callable(source_files) and not any(parameter.default is inspect.Parameter.empty and parameter.kind not in (inspect.Parameter.VAR_POSITIONAL,inspect.Parameter.VAR_KEYWORD) for parameter in inspect.signature(source_files).parameters.values()):
                manifest.extend(source_files())
    dynamic=[*p.parent.glob('*.json')]
    if args.contracts and args.series=='excel':
        dynamic.extend(clip.glob('🗣️transcript.*.md'))
    from manim._config.utils import config_file_paths
    config_inputs=[*config_file_paths(),root/'manim.cfg',Path.cwd()/'manim.cfg']
    return provenance.dependency_paths(root,selected_paths,(*manifest,*imported,*(path for path in config_inputs if path.is_file())),(p,Path(__file__),provenance_path,*(Path(__file__).with_name(name) for name in ('replay-native-pairs.py','native-pairs.json','native-pairs.schema.json','verify-native-ledger.py','verify.py')),*dynamic))

def semantic_fingerprints():
    return provenance.fingerprints(root,dependency_files())

def runtime_identity():
    versions={}
    for name in ('manim','manimpango','pycairo','numpy','rhino3dm','Pillow','networkx'):
        try:
            versions[name]=importlib.metadata.version(name)
        except importlib.metadata.PackageNotFoundError:
            versions[name]=None
    return {'python':sys.version,'executable':sys.executable,'platform':platform.platform(),'packages':versions,'languages':args.languages.split(','),'contracts':args.contracts}

source_fingerprints=semantic_fingerprints()
runtime_start=runtime_identity()
render_config_start={key:str(getattr(manim.config,key,None)) for key in ('renderer','pixel_width','pixel_height','frame_width','frame_height','frame_rate','background_color','transparent','disable_caching','write_to_movie','save_last_frame','format','media_dir','text_dir','tex_dir')}
if args.provenance_only:
    (output/'provenance.json').write_text(json.dumps({'started_at':started_at,'source_fingerprints':source_fingerprints,'runtime_identity':runtime_start,'initial_render_config':render_config_start,'declared_native_configurations':provenance.render_configurations(p),'selected_scenes':[str(path.relative_to(root)) for path in sorted(selected_paths)]},ensure_ascii=False,indent=2),encoding='utf-8')
    print('[DEBUG] Captured '+str(len(source_fingerprints))+' dependency inputs',flush=True)
    raise SystemExit(0)

if args.contracts:
    suites=unittest.defaultTestLoader.loadTestsFromModule(module)
    def tests(suite):
        for item in suite:
            if isinstance(item,unittest.TestSuite):
                yield from tests(item)
            elif item._testMethodName!='test_native_lifecycle':
                yield item
    suite=unittest.TestSuite(tests(suites))
elif args.series=='grasshopper':
    requested=set(filter(None,args.scenes.split(',')))
    selected=[]
    for folder,base,path in module.discover():
        if requested and base.__name__ not in requested and folder not in requested:
            continue
        def traced(base):
            def step(self,key,*animations,**kwargs):
                print('[DEBUG] '+args.series+' '+base.__name__+' '+os.environ.get('SEMIO_TEACHING_LANGUAGE','')+' '+key,flush=True)
                return super(trace,self).step(key,*animations,**kwargs)
            trace=type('Traced'+base.__name__,(base,),{'step':step})
            return trace
        selected.append((folder,traced(base),path))
    if not selected:
        raise ValueError('No requested Grasshopper scenes found')
    module.discover=lambda:selected
    module.CASES['languages']=args.languages.split(',')
    suite=unittest.TestSuite([module.GrasshopperPresentationContract('test_native_lifecycle')])
else:
    original_discover=module.discover
    requested=set(filter(None,args.scenes.split(',')))
    all_entries=original_discover()
    selected=[entry for entry in all_entries if not requested or entry['scene'] in requested or entry['folder'] in requested]
    if not selected:
        raise ValueError('No requested scenes found')
    module.discover=lambda:selected
    module.CASES['languages']=args.languages.split(',')
    original_class=module.scene_class
    def traced_class(entry):
        base=original_class(entry)
        def traced(method):
            def dispatch(self,key,*animations,**kwargs):
                print('[DEBUG] '+args.series+' '+entry['scene']+' '+os.environ.get('SEMIO_TEACHING_LANGUAGE','')+' '+key,flush=True)
                return getattr(super(trace,self),method)(key,*animations,**kwargs)
            return dispatch
        namespace={method:traced(method) for method in ('step','_clause') if callable(getattr(base,method,None))}
        trace=type('Traced'+base.__name__,(base,),namespace)
        return trace

    module.scene_class=traced_class
    suite=unittest.TestSuite([module.SceneContract('test_native_lifecycle')])

class ImmediateResult(unittest.TextTestResult):
    """🔎 Keep native subtest failures visible even when an external process supervisor stops the run."""
    def addSubTest(self,test,subtest,error):
        super().addSubTest(test,subtest,error)
        params=dict(subtest.params)
        end_fingerprints=semantic_fingerprints()
        record={'started_at':started_at,'source_fingerprints':source_fingerprints,'source_fingerprints_end':end_fingerprints,'provenance_scope':'all-scenes' if args.contracts else 'selected-dependencies','runtime_identity':runtime_start,'render_config':render_config_start,'source_unchanged':provenance.unchanged(source_fingerprints,end_fingerprints),'series':args.series,'scene':str(params.get('scene','')).removeprefix('Traced'),'language':params.get('language'),'outcome':'success' if error is None else 'error'}
        if error is not None:
            record.update(error_type=error[0].__name__,message=str(error[1]))
        with (output/'checks.jsonl').open('a',encoding='utf-8') as stream:
            stream.write(json.dumps(record,ensure_ascii=False)+'\n')
            stream.flush()
        if error is not None:
            print('[DEBUG] Native subtest failed: '+str(subtest),flush=True)
            traceback.print_exception(*error,file=sys.stdout)
            sys.stdout.flush()

result=unittest.TextTestRunner(verbosity=2,resultclass=ImmediateResult,failfast=True).run(suite)
end_fingerprints=semantic_fingerprints()
runtime_end=runtime_identity()
stable=provenance.unchanged(source_fingerprints,end_fingerprints) and runtime_start==runtime_end
(output/'result.json').write_text(json.dumps({'started_at':started_at,'source_fingerprints':source_fingerprints,'source_fingerprints_end':end_fingerprints,'provenance_scope':'all-scenes' if args.contracts else 'selected-dependencies','runtime_identity':runtime_start,'runtime_identity_end':runtime_end,'render_config':render_config_start,'source_unchanged':stable,'tests':result.testsRun,'failures':len(result.failures),'errors':len(result.errors),'success':result.wasSuccessful()}),encoding='utf-8')
raise SystemExit(0 if result.wasSuccessful() and stable else 1)
