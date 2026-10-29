"""🏷️ Execute current analytical labels on the actual captured native analysis without a renderer."""
import ast
from pathlib import Path
from types import SimpleNamespace
import numpy as np

ROOT=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip')
class Group:
    """🎨 Minimal recording group; no native frame or font layout is produced."""
    def __init__(self,*args,**kwargs):
        self.submobjects=list(args)
    def add(self,*args):
        self.submobjects.extend(args)
        return self
    def move_to(self,*args):
        return self
    def get_center(self):
        return [0,0,0]

def callback(name,stop,result):
    """🧩 Compile the unchanged actual prefix before animation scheduling."""
    tree=ast.parse((ROOT/'🧩️models'/'🐍️.py').read_text())
    cls=next(n for n in tree.body if isinstance(n,ast.ClassDef))
    method=next(n for n in cls.body if isinstance(n,ast.FunctionDef) and n.name==name)
    for i,node in enumerate(method.body):
        if isinstance(node,ast.Assign) and any((isinstance(t,ast.Name) and t.id==stop) or (isinstance(t,ast.Tuple) and any(isinstance(p,ast.Name) and p.id==stop for p in t.elts)) for t in node.targets):
            method.body=method.body[:i]+[ast.Return(value=ast.Name(id=result,ctx=ast.Load()))]
            break
    space={'VGroup':Group,'Circle':Group,'ValueTracker':lambda v:SimpleNamespace(get_value=lambda:v),'LEFT_PLAN':[-3,0,0],'STRUCT':'structure','GOOD':'good','DATA':'data','MUTED':'muted','STAR':'*'}
    exec(compile(ast.fix_missing_locations(ast.Module(body=[method],type_ignores=[])),'actual-native-analysis-labels','exec'),space)
    return type('Labels',(),{name:space[name]})()

counts=analysis['counts']
expected={'IfcStructuralCurveMember':int(np.count_nonzero([m['kind']=='curve' for m in analysis['members']])),
          'IfcSlab':len(np.unique([m['source_guid'] for m in analysis['members'] if m['kind']=='surface'])),
          'IfcStructuralSurfaceMember':len(analysis['shells']),
          'IfcStructuralPointConnection':int(np.count_nonzero([n['role']=='slab' for n in analysis['nodes']]))}
for language in ('en','de'):
    member=callback('_members','rows','spec')
    member.analysis=analysis
    member.language=lambda:language
    member.tr=lambda en,de:en if language=='en' else de
    member._chains=member.scale_bar=member.villa_plan=lambda *args,**kwargs:Group()
    member.analysis_view=lambda *args,**kwargs:SimpleNamespace(members=Group(*[Group() for _ in range(counts['curve_members'])]),shells=Group(*[Group() for _ in analysis['shells']]),nodes=Group(*[Group() for n in analysis['nodes'] if n['role']=='slab']),outline=Group(),remove=lambda *args:None)
    rows=member._members()
    assert {row[0]:row[1] for row in rows}==expected
    support=callback('_supports','condition','row')
    support.analysis=analysis
    support.language=lambda:language
    support._chains=support.scale_bar=support.villa_plan=support._counter=support.text=lambda *args,**kwargs:Group()
    support.analysis_view=lambda *args,**kwargs:SimpleNamespace(supports=Group(*[Group() for _ in analysis['supports']]),remove=lambda *args:None)
    labels=[]
    def label(en,de,*args,**kwargs):
        labels.append(en if language=='en' else de)
        return Group()
    support.label=label
    support._supports()
    assert any(('selected analytical births' if language=='en' else 'gewählten analytischen Stützenanfängen') in text for text in labels)
print('PASS actual native analytical member/source/shell/node counts and selected-birth labels EN/DE',expected)

exchange_tree=ast.parse((ROOT/'🔌️exchange'/'🐍️.py').read_text())
exchange_cls=next(n for n in exchange_tree.body if isinstance(n,ast.ClassDef))
basis_method=next(n for n in exchange_cls.body if isinstance(n,ast.FunctionDef) and n.name=='_basis')
basis_space={}
exec(compile(ast.fix_missing_locations(ast.Module(body=[basis_method],type_ignores=[])),'actual-native-provenance-callers','exec'),basis_space)
basis=type('Provenance',(),{'_basis':basis_space['_basis']})()
for language in ('en','de'):
    basis.tr=lambda en,de:en if language=='en' else de
    assert facts.thickness['basis']=='derived'
    assert basis._basis(facts.thickness)==('calculated' if language=='en' else 'berechnet')
    assert basis._basis(analysis['supports'][0])==('assumed' if language=='en' else 'angenommen')
print('PASS actual computed thickness and authored support provenance EN/DE')
