"""🧭 Retain actual generated vector helper and independent SQLite numeric observations."""
import hashlib
import json
import math
import platform
import sqlite3
from pathlib import Path
import ifcopenshell
import ifcopenshell.express.rules.IFC4X3_ADD2 as rules
import ifcopenshell.validate

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
root=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️vector'
data=json.loads((root/'cases.json').read_text(encoding='utf-8'))

class VectorOracle:
    def observe(self,case):raise NotImplementedError

def equivalent(a,b,tolerance=0):
    if isinstance(a,(int,float)) and isinstance(b,(int,float)):return math.isfinite(a) and math.isfinite(b) and abs(a-b)<=tolerance
    if isinstance(a,dict) and isinstance(b,dict):return a.keys()==b.keys() and all(equivalent(a[k],b[k],tolerance) for k in a)
    if isinstance(a,(list,tuple)) and isinstance(b,(list,tuple)):return len(a)==len(b) and all(equivalent(x,y,tolerance) for x,y in zip(a,b))
    return a==b

class GeneratedVectorOracle(VectorOracle):
    def __init__(self):self.model=ifcopenshell.open(str(root/'source.ifc'))
    def project(self,value):
        if value is None:return dict(kind='unknown')
        if isinstance(value,(int,float)):return dict(kind='real',value=value)
        if isinstance(value,(list,tuple)):return dict(kind='directions',items=[list(row.DirectionRatios) for row in value])
        if hasattr(value,'DirectionRatios'):return dict(kind='direction',ratios=list(value.DirectionRatios))
        if hasattr(value,'Orientation'):return dict(kind='vector',orientation=list(value.Orientation.DirectionRatios),magnitude=value.Magnitude)
        raise TypeError('Unsupported independent result '+type(value).__name__)
    def observe(self,case):
        result=dict(id=case['id'],owner=case['owner'])
        try:
            before=self.model.to_string()
            function=getattr(rules,next(name for name in dir(rules) if name.upper()==case['owner']))
            value=function(*[self.model.by_id(int(id)) if id is not None else None for id in case['args']])
            projected=self.project(value)
            unchanged=self.model.to_string()==before
            result.update(value=projected,rawType=type(value).__name__,rawRepresentation=str(value),sourceUnchanged=unchanged,status='agree' if unchanged and equivalent(projected,case['expected'],case['tolerance']) else 'divergent-value')
        except Exception as error:result.update(status='unsupported',error=dict(kind=type(error).__name__,message=str(error)))
        return result

class SqliteVectorArithmeticOracle(VectorOracle):
    def __init__(self,model):self.model=model
    def ratios(self,id):return list(self.model.by_id(int(id)).DirectionRatios) if id is not None else None
    def observe(self,case):
        result=dict(id=case['id'],owner=case['owner']);owner=case['owner']
        if owner not in ('IFCDOTPRODUCT','IFCCROSSPRODUCT','IFCORTHOGONALCOMPLEMENT'):return dict(**result,status='unsupported',reason='outside independent SQL scalar arithmetic slice')
        args=[self.ratios(id) for id in case['args']]
        with sqlite3.connect(':memory:') as db:
            db.execute('CREATE TABLE ratios(axis INTEGER,a REAL,b REAL)')
            a=args[0];b=args[1] if len(args)>1 else None
            if a is None or (len(args)>1 and b is None) or (owner=='IFCDOTPRODUCT' and len(a)!=len(b)) or (owner=='IFCCROSSPRODUCT' and (len(a)!=3 or len(b)!=3)) or (owner=='IFCORTHOGONALCOMPLEMENT' and len(a)!=2):observed=dict(kind='unknown')
            elif owner=='IFCORTHOGONALCOMPLEMENT':
                db.execute('INSERT INTO ratios VALUES(1,?,?)',(a[0],a[1]));values=db.execute('SELECT -b,a FROM ratios').fetchone();observed=dict(kind='direction',ratios=values)
            else:
                db.executemany('INSERT INTO ratios VALUES(?,?,?)',[(i,x,b[i]) for i,x in enumerate(a)])
                na,nb=db.execute('SELECT sqrt(SUM(a*a)),sqrt(SUM(b*b)) FROM ratios').fetchone()
                if owner=='IFCDOTPRODUCT':observed=dict(kind='real',value=db.execute('SELECT SUM((a/?)*(b/?)) FROM ratios',(na,nb)).fetchone()[0])
                else:
                    values=db.execute('SELECT x.a*y.b-x.b*y.a,y.a*z.b-y.b*z.a,z.a*x.b-z.b*x.a FROM (SELECT a/? AS a,b/? AS b FROM ratios WHERE axis=1) x,(SELECT a/? AS a,b/? AS b FROM ratios WHERE axis=2)y,(SELECT a/? AS a,b/? AS b FROM ratios WHERE axis=0)z',(na,nb,na,nb,na,nb)).fetchone()
                    magnitude=math.sqrt(sum(n*n for n in values))
                    observed=dict(kind='vector',orientation=list(values) if magnitude>0 else a,magnitude=magnitude)
        return dict(**result,value=observed,status='agree' if equivalent(observed,case['expected'],case['tolerance']) else 'divergent-value')

def main():
    oracle=GeneratedVectorOracle();sql=SqliteVectorArithmeticOracle(oracle.model)
    functions=[oracle.observe(case) for case in data['functions']]
    arithmetic=[]
    for case in data['functions']:
        try:arithmetic.append(sql.observe(case))
        except Exception as error:arithmetic.append(dict(id=case['id'],owner=case['owner'],status='unsupported',error=dict(kind=type(error).__name__,message=str(error))))
    logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(oracle.model,logger,express_rules=True)
    result=dict(python=platform.python_version(),ifcopenshell=ifcopenshell.version,sqlite=sqlite3.sqlite_version,inputSha256=hashlib.sha256((root/'cases.json').read_bytes()).hexdigest(),sourceSha256=hashlib.sha256((root/'source.ifc').read_bytes()).hexdigest(),functions=functions,arithmetic=arithmetic,physicalDiagnostics=logger.statements)
    (ticket/'🗑️generated/issued-express-vector-oracle.json').write_text(json.dumps(result,indent=2,default=str)+'\n',encoding='utf-8')
    print('[DEBUG] '+json.dumps(dict(functions={status:sum(row['status']==status for row in functions) for status in ['agree','divergent-value','unsupported']},sql={status:sum(row['status']==status for row in arithmetic) for status in ['agree','divergent-value','unsupported']},physicalDiagnostics=len(logger.statements))))

if __name__=='__main__':main()
