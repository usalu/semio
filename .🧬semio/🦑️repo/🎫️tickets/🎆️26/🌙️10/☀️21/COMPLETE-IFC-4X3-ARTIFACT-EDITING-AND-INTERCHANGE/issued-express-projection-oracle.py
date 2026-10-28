"""🧭 Compare fresh generated projection helpers and independent SQL scalar projections."""
import hashlib
import inspect
import json
import math
import platform
import sqlite3
from pathlib import Path
import ifcopenshell
import ifcopenshell.express.rules.IFC4X3_ADD2 as rules
ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
root=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️projection'
data=json.loads((root/'cases.json').read_text(encoding='utf-8'));source=root/data['source']['file']

class ProjectionOracle:
    def observe(self,case):raise NotImplementedError

def equivalent(a,b,tolerance):
    if isinstance(a,(float,int)) and isinstance(b,(float,int)):return math.isfinite(a) and math.isfinite(b) and abs(a-b)<=tolerance
    if isinstance(a,dict) and isinstance(b,dict):return a.keys()==b.keys() and all(equivalent(a[key],b[key],tolerance) for key in a)
    if isinstance(a,(list,tuple)) and isinstance(b,(list,tuple)):return len(a)==len(b) and all(equivalent(x,y,tolerance) for x,y in zip(a,b))
    return a==b

class GeneratedProjectionOracle(ProjectionOracle):
    def observe(self,case):
        model=ifcopenshell.open(str(source));before=model.to_string();result=dict(id=case['id'],owner=case['owner'])
        try:
            args=[model.by_id(int(row['id'])) if row['kind']=='entity' else int(row['value']) if row['kind']=='integer' else None for row in case['args']]
            function=getattr(rules,next(name for name in dir(rules) if name.upper()==case['owner']))
            value=function(*args)
            def direction(item):return None if item is None or item is rules.INDETERMINATE else list(item.DirectionRatios)
            observed=dict(kind='unknown') if value is None or value is rules.INDETERMINATE else dict(kind='axes',items=[direction(item) for item in value]) if isinstance(value,(list,tuple)) else dict(kind='direction',ratios=direction(value))
            result.update(value=observed,rawType=type(value).__name__,rawRepresentation=str(value),status='agree' if equivalent(observed,case['expected'],case['tolerance']) else 'divergent-value')
        except Exception as error:result.update(status='unsupported',error=dict(kind=type(error).__name__,message=str(error)))
        result['sourceUnchanged']=model.to_string()==before
        if not result['sourceUnchanged']:result['sourceAfter']=model.to_string()
        return result

class SqliteProjectionOracle(ProjectionOracle):
    def __init__(self):self.model=ifcopenshell.open(str(source))
    def ratios(self,arg):return list(self.model.by_id(int(arg['id'])).DirectionRatios) if arg['kind']=='entity' else None
    def normalize(self,db,value):
        if value is None:return None
        db.execute('DELETE FROM scalar');db.executemany('INSERT INTO scalar VALUES(?,?)',enumerate(value));norm=db.execute('SELECT sqrt(SUM(value*value)) FROM scalar').fetchone()[0]
        return [row[0] for row in db.execute('SELECT value/? FROM scalar ORDER BY axis',(norm,))] if norm else None
    def dot(self,db,a,b):
        if a is None or b is None or len(a)!=len(b):return None
        db.execute('DELETE FROM pair');db.executemany('INSERT INTO pair VALUES(?,?,?)',[(i,x,b[i]) for i,x in enumerate(a)]);return db.execute('SELECT SUM(a*b) FROM pair').fetchone()[0]
    def subtract_projection(self,db,a,z,factor):
        if factor is None:return None
        db.execute('DELETE FROM pair');db.executemany('INSERT INTO pair VALUES(?,?,?)',[(i,x,z[i]) for i,x in enumerate(a)]);return [row[0] for row in db.execute('SELECT a-?*b FROM pair ORDER BY axis',(factor,))]
    def first(self,db,z,arg):
        z=self.normalize(db,z)
        if z is None:return None
        if arg is not None and len(arg)!=3:return None
        v=self.normalize(db,arg) if arg is not None else [0,1,0] if z==[1,0,0] else [1,0,0]
        if v is None or len(z)!=len(v):return None
        if arg is not None:
            db.execute('DELETE FROM scalar');db.executemany('INSERT INTO scalar VALUES(?,?)',enumerate([v[1]*z[2]-v[2]*z[1],v[2]*z[0]-v[0]*z[2],v[0]*z[1]-v[1]*z[0]]))
            if db.execute('SELECT SUM(value*value) FROM scalar').fetchone()[0]==0:return None
        residual=self.subtract_projection(db,v,z,self.dot(db,v,z))
        return self.normalize(db,residual) or v
    def second(self,db,z,x,arg):
        z=self.normalize(db,z);x=self.normalize(db,x);v=self.normalize(db,arg if arg is not None else [0,1,0])
        first=self.subtract_projection(db,v,z,self.dot(db,v,z))
        if first is None:return None
        return self.normalize(db,self.subtract_projection(db,first,x,self.dot(db,v,x)))
    def observe(self,case):
        result=dict(id=case['id'],owner=case['owner']);owner=case['owner'];args=case['args']
        if owner=='IFCBASEAXIS' and args[0].get('value')=='3':return dict(**result,status='unsupported',reason='outside independent SQL2D-basis/first/second scalar interface')
        with sqlite3.connect(':memory:') as db:
            db.execute('CREATE TABLE scalar(axis INTEGER PRIMARY KEY,value REAL)');db.execute('CREATE TABLE pair(axis INTEGER PRIMARY KEY,a REAL,b REAL)')
            if owner=='IFCFIRSTPROJAXIS':value=self.first(db,*[self.ratios(arg) for arg in args]);observed=dict(kind='direction',ratios=value) if value is not None else dict(kind='unknown')
            elif owner=='IFCSECONDPROJAXIS':value=self.second(db,*[self.ratios(arg) for arg in args]);observed=dict(kind='direction',ratios=value) if value is not None else dict(kind='unknown')
            else:
                a,b=[self.ratios(arg) for arg in args[1:3]]
                if a is not None:
                    a=self.normalize(db,a);items=[a,[-a[1],a[0]]]
                    if b is not None and self.dot(db,self.normalize(db,b),items[1])<0:items[1]=[-value for value in items[1]]
                elif b is not None:b=self.normalize(db,b);items=[[b[1],-b[0]],b]
                else:items=[[1,0],[0,1]]
                observed=dict(kind='axes',items=items)
        return dict(**result,value=observed,status='agree' if equivalent(observed,case['expected'],case['tolerance']) else 'divergent-value')

def main():
    generated=GeneratedProjectionOracle();sql=SqliteProjectionOracle();functions=[generated.observe(case) for case in data['functions']];arithmetic=[]
    for case in data['functions']:
        try:arithmetic.append(sql.observe(case))
        except Exception as error:arithmetic.append(dict(id=case['id'],owner=case['owner'],status='unsupported',error=dict(kind=type(error).__name__,message=str(error))))
    helpers=[dict(owner=owner,sha256=hashlib.sha256(inspect.getsource(getattr(rules,next(name for name in dir(rules) if name.upper()==owner))).encode()).hexdigest()) for owner in sorted({row['owner'] for row in data['functions']})]
    first=getattr(rules,next(name for name in dir(rules) if name.upper()=='IFCFIRSTPROJAXIS'));ratios=ifcopenshell.open(str(source)).by_id(5).DirectionRatios
    comparison=dict(actualRatiosType=type(ratios).__name__,literalType='list',actualRuntimeEqual=ratios==[1.0,0.0,0.0],sameNumericMembers=list(ratios)==[1.0,0.0,0.0],generatedComparisons=[line.strip() for line in inspect.getsource(first).splitlines() if 'DirectionRatios' in line and '!=' in line],primaryAggregateValueEquality=True)
    result=dict(python=platform.python_version(),ifcopenshell=ifcopenshell.version,sqlite=sqlite3.sqlite_version,casesSha256=hashlib.sha256((root/'cases.json').read_bytes()).hexdigest(),sourceSha256=hashlib.sha256(source.read_bytes()).hexdigest(),functions=functions,arithmetic=arithmetic,helpers=helpers,tupleListComparison=comparison)
    (ticket/'🗑️generated/issued-express-projection-oracle.json').write_text(json.dumps(result,indent=2,default=str)+'\n',encoding='utf-8')
    print('[DEBUG] '+json.dumps(dict(functions={status:sum(row['status']==status for row in functions) for status in ['agree','divergent-value','unsupported']},sourceMutations=[row['id'] for row in functions if not row['sourceUnchanged']],sql={status:sum(row['status']==status for row in arithmetic) for status in ['agree','divergent-value','unsupported']})))

if __name__=='__main__':main()
