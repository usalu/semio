"""⏱️ Read-only profiling of the unchanged native slab partition oracle."""
import importlib,json,time,unittest
from pathlib import Path
from shapely.geometry import mapping
from shapely.geometry.base import BaseGeometry

ticket=Path(__file__).parent;generated=ticket/'🗑️generated';started=time.monotonic();state={'slab':None,'operations':0}
module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.⚡️partitions')
api=module.api
original_slab=api._slab_geometry
original_difference=BaseGeometry.difference
original_intersection=BaseGeometry.intersection
original_union=module.unary_union

def phase(name,**values):
    record={'phase':name,'seconds':time.monotonic()-started,**state,**values}
    (generated/'slab-oracle-progress-r69.json').write_text(json.dumps(record),encoding='utf8')

def slab(model,source):
    state['slab']=source['tag'];phase('native-slab-start');begin=time.monotonic();result=original_slab(model,source)
    phase('native-slab-end',elapsed=time.monotonic()-begin,exclusive=len(result['parts']),full=len(result['full_parts']))
    return result

def binary(name,original,left,right,*args,**kwargs):
    state['operations']+=1
    (generated/'slab-oracle-current-r69.json').write_text(json.dumps({'operation':name,'slab':state['slab'],'left':mapping(left),'right':mapping(right)}),encoding='utf8')
    phase(name+'-start');begin=time.monotonic();result=original(left,right,*args,**kwargs);phase(name+'-end',elapsed=time.monotonic()-begin)
    return result

def difference(left,right,*args,**kwargs):
    return binary('difference',original_difference,left,right,*args,**kwargs)

def intersection(left,right,*args,**kwargs):
    return binary('intersection',original_intersection,left,right,*args,**kwargs)

def union(values):
    state['operations']+=1
    (generated/'slab-oracle-current-r69.json').write_text(json.dumps({'operation':'unary_union','slab':state['slab'],'geometries':[mapping(value) for value in values]}),encoding='utf8')
    phase('unary-union-start',count=len(values));begin=time.monotonic();result=original_union(values);phase('unary-union-end',elapsed=time.monotonic()-begin)
    return result

api._slab_geometry=slab;BaseGeometry.difference=difference;BaseGeometry.intersection=intersection;module.unary_union=union
try:
    result=unittest.TextTestRunner().run(unittest.defaultTestLoader.loadTestsFromName('SlabPartitionContract.test_source_region_surfaces_and_sleeves_match_independent_geos',module))
    record={'seconds':time.monotonic()-started,'tests':result.testsRun,'failures':[(str(t),m) for t,m in result.failures],'errors':[(str(t),m) for t,m in result.errors]}
    (generated/'slab-oracle-result-r69.json').write_text(json.dumps(record));print(record)
finally:
    api._slab_geometry=original_slab;BaseGeometry.difference=original_difference;BaseGeometry.intersection=original_intersection;module.unary_union=original_union
