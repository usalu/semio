"""➗ Observe fresh generated helpers and independent ordered SQLite vector arithmetic."""
import hashlib
import json
import math
import platform
import sqlite3
import inspect
from pathlib import Path
import ifcopenshell
import ifcopenshell.express.rules.IFC4X3_ADD2 as rules
import ifcopenshell.validate

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
validation=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation'
root=validation/'🧪️vector-arithmetic'
data=json.loads((root/'cases.json').read_text(encoding='utf-8'))
source=root/data['source']['file']

class ArithmeticOracle:
    def observe(self,case):raise NotImplementedError

def equivalent(a,b,tolerance=0):
    if isinstance(a,(int,float)) and isinstance(b,(int,float)):return math.isfinite(a) and math.isfinite(b) and abs(a-b)<=tolerance
    if isinstance(a,dict) and isinstance(b,dict):return a.keys()==b.keys() and all(equivalent(a[k],b[k],tolerance) for k in a)
    if isinstance(a,(list,tuple)) and isinstance(b,(list,tuple)):return len(a)==len(b) and all(equivalent(x,y,tolerance) for x,y in zip(a,b))
    return a==b

class GeneratedArithmeticOracle(ArithmeticOracle):
    def observe(self,case):
        model=ifcopenshell.open(str(source));before=model.to_string()
        result=dict(id=case['id'],owner=case['owner'])
        try:
            args=[model.by_id(int(row['id'])) if row['kind']=='entity' else float(row['value']) if row['kind']=='real' else None for row in case['args']]
            function=getattr(rules,next(name for name in dir(rules) if name.upper()==case['owner']))
            value=function(*args)
            projected=dict(kind='unknown') if value is None or value is rules.INDETERMINATE else dict(kind='vector',orientation=list(value.Orientation.DirectionRatios),magnitude=value.Magnitude)
            result.update(value=projected,rawType=type(value).__name__,rawRepresentation=str(value),status='agree' if equivalent(projected,case['expected'],case['tolerance']) else 'divergent-value')
        except Exception as error:result.update(status='unsupported',error=dict(kind=type(error).__name__,message=str(error)))
        result.update(sourceUnchanged=model.to_string()==before)
        if not result['sourceUnchanged']:result['sourceAfter']=model.to_string()
        return result

class SqliteArithmeticOracle(ArithmeticOracle):
    def __init__(self):self.model=ifcopenshell.open(str(source))
    def components(self,arg):
        if arg['kind']=='unknown':return None
        entity=self.model.by_id(int(arg['id']))
        return (list(entity.Orientation.DirectionRatios),entity.Magnitude) if entity.is_a('IfcVector') else (list(entity.DirectionRatios),1)
    def observe(self,case):
        result=dict(id=case['id'],owner=case['owner']);owner=case['owner'];args=case['args']
        if any(row['kind']=='unknown' for row in args):value=dict(kind='unknown')
        else:
            a,ma=self.components(args[1] if owner=='IFCSCALARTIMESVECTOR' else args[0]);b,mb=self.components(args[1]) if owner!='IFCSCALARTIMESVECTOR' else (a,ma)
            if len(a)!=len(b):value=dict(kind='unknown')
            else:
                with sqlite3.connect(':memory:') as db:
                    db.execute('CREATE TABLE components(axis INTEGER PRIMARY KEY,a REAL,b REAL)')
                    db.executemany('INSERT INTO components VALUES(?,?,?)',[(i,x,b[i]) for i,x in enumerate(a)])
                    na,nb=db.execute('SELECT sqrt(SUM(a*a)),sqrt(SUM(b*b)) FROM components').fetchone()
                    if owner=='IFCSCALARTIMESVECTOR':
                        magnitude=float(args[0]['value'])*ma
                        orientation=[row[0] for row in db.execute('SELECT a/?*? FROM components ORDER BY axis',(na,-1 if magnitude<0 else 1))]
                        value=dict(kind='vector',orientation=orientation,magnitude=abs(magnitude))
                    else:
                        db.execute('CREATE TABLE output(axis INTEGER PRIMARY KEY,value REAL)')
                        db.execute('INSERT INTO output SELECT axis,(a/?)*? + (b/?)*? FROM components',(na,ma,nb,mb*(1 if owner=='IFCVECTORSUM' else -1)))
                        magnitude=db.execute('SELECT sqrt(SUM(value*value)) FROM output').fetchone()[0]
                        orientation=[row[0] for row in db.execute('SELECT value FROM output ORDER BY axis')] if magnitude>0 else [row[0] for row in db.execute('SELECT a/? FROM components ORDER BY axis',(na,))]
                        value=dict(kind='vector',orientation=orientation,magnitude=magnitude)
        return dict(**result,value=value,status='agree' if equivalent(value,case['expected'],case['tolerance']) else 'divergent-value')

def main():
    generated=GeneratedArithmeticOracle();sql=SqliteArithmeticOracle()
    functions=[generated.observe(case) for case in data['functions']];arithmetic=[sql.observe(case) for case in data['functions']]
    model=ifcopenshell.open(str(source));logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(model,logger,express_rules=True)
    helpers=[dict(owner=owner,sha256=hashlib.sha256(inspect.getsource(getattr(rules,next(name for name in dir(rules) if name.upper()==owner))).encode()).hexdigest()) for owner in sorted({row['owner'] for row in data['functions']})]
    result=dict(python=platform.python_version(),ifcopenshell=ifcopenshell.version,sqlite=sqlite3.sqlite_version,casesSha256=hashlib.sha256((root/'cases.json').read_bytes()).hexdigest(),sourceSha256=hashlib.sha256(source.read_bytes()).hexdigest(),functions=functions,arithmetic=arithmetic,helpers=helpers,physicalDiagnostics=logger.statements)
    (ticket/'🗑️generated/issued-express-arithmetic-oracle.json').write_text(json.dumps(result,indent=2,default=str)+'\n',encoding='utf-8')
    print('[DEBUG] '+json.dumps(dict(functions={status:sum(row['status']==status for row in functions) for status in ['agree','divergent-value','unsupported']},sourceMutations=[row['id'] for row in functions if not row['sourceUnchanged']],sql={status:sum(row['status']==status for row in arithmetic) for status in ['agree','divergent-value','unsupported']})))

if __name__=='__main__':main()
