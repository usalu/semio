"""📐 Observe generated placement/point helpers and independent strict-epsilon SQL arithmetic."""
import hashlib
import inspect
import json
import platform
import sqlite3
from pathlib import Path
import ifcopenshell
import ifcopenshell.express.rules.IFC4X3_ADD2 as rules
import ifcopenshell.validate

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
root=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️placement'
data=json.loads((root/'cases.json').read_text(encoding='utf-8'))

class PlacementOracle:
    def observe(self,case):raise NotImplementedError

class GeneratedPlacementOracle(PlacementOracle):
    def __init__(self):self.model=ifcopenshell.open(str(root/'source.ifc'))
    def decode(self,value):
        if value['kind']=='unknown':return None
        if value['kind']=='entity':return self.model.by_id(int(value['id']))
        return float(value['value'])
    def observe(self,case):
        result=dict(id=case['id'],owner=case['owner'])
        try:
            before=self.model.to_string()
            fn=getattr(rules,next(name for name in dir(rules) if name.upper()==case['owner']))
            value=fn(*[self.decode(arg) for arg in case['args']])
            if value is rules.INDETERMINATE:observed=dict(kind='logical',value='unknown') if case['owner'] in ('IFCSAMECARTESIANPOINT','IFCSAMEDIRECTION','IFCSAMEAXIS2PLACEMENT') else dict(kind='unknown')
            elif value is None:observed=dict(kind='unknown')
            elif isinstance(value,bool):observed=dict(kind='logical',value=str(value).lower())
            elif isinstance(value,int):observed=dict(kind='integer',value=str(value))
            elif value=='UNKNOWN':observed=dict(kind='logical',value='unknown')
            else:raise TypeError('Unsupported independent result '+type(value).__name__)
            unchanged=self.model.to_string()==before
            result.update(value=observed,rawType=type(value).__name__,rawScalar='INDETERMINATE' if value is rules.INDETERMINATE else value,sourceUnchanged=unchanged,status='agree' if unchanged and observed==case['expected'] else 'divergent-value')
        except Exception as error:result.update(status='unsupported',error=dict(kind=type(error).__name__,message=str(error)))
        return result

class SqliteEpsilonOracle(PlacementOracle):
    def __init__(self,model):self.model=model
    def observe(self,case):
        result=dict(id=case['id'],owner=case['owner'])
        if case['owner']=='IFCCORRECTLOCALPLACEMENT':
            axis,parent=case['args']
            child=None if axis['kind']=='unknown' else self.model.by_id(int(axis['id'])).is_a().upper()
            placement=None if parent['kind']=='unknown' else self.model.by_id(int(parent['id']))
            parent_kind=None if placement is None else placement.is_a().upper()
            dim=placement.RelativePlacement.Dim if parent_kind=='IFCLOCALPLACEMENT' else None
            with sqlite3.connect(':memory:') as db:
                value=db.execute("SELECT CASE WHEN ? IS NULL THEN 1 WHEN ?='IFCGRIDPLACEMENT' THEN NULL WHEN ?='IFCLOCALPLACEMENT' AND ?='IFCAXIS2PLACEMENT2D' THEN 1 WHEN ?='IFCLOCALPLACEMENT' AND ?='IFCAXIS2PLACEMENT3D' THEN ?=3 ELSE NULL END",(parent_kind,parent_kind,parent_kind,child,parent_kind,child,dim)).fetchone()[0]
            observed=dict(kind='unknown') if value is None else dict(kind='logical',value=str(bool(value)).lower())
            return dict(**result,value=observed,parentKind=parent_kind,parentDim=dim,childKind=child,status='agree' if observed==case['expected'] else 'divergent-value')
        if case['owner'] not in ('IFCSAMECARTESIANPOINT','IFCSAMEDIRECTION'):return dict(**result,status='unsupported',reason='outside independent strict-epsilon scalar interface')
        field='Coordinates' if case['owner']=='IFCSAMECARTESIANPOINT' else 'DirectionRatios'
        def values(arg):
            if arg['kind']=='unknown':return [None,None,None]
            row=list(getattr(self.model.by_id(int(arg['id'])),field));return row+[0.]*(3-len(row))
        a,b=map(values,case['args'][:2]);epsilon=case['args'][2]
        epsilon=1e-6 if epsilon['kind']=='unknown' else float(epsilon['value'])
        with sqlite3.connect(':memory:') as db:
            db.execute('CREATE TABLE components(i INTEGER,a REAL,b REAL)');db.executemany('INSERT INTO components VALUES(?,?,?)',[(i,a[i],b[i]) for i in range(3)])
            false,unknown=db.execute('SELECT SUM(CASE WHEN NOT(a+?>b AND a<b+?) THEN 1 ELSE 0 END),SUM(CASE WHEN (a+?>b AND a<b+?) IS NULL THEN 1 ELSE 0 END) FROM components',(epsilon,epsilon,epsilon,epsilon)).fetchone()
            value='false' if false else 'unknown' if unknown else 'true'
        observed=dict(kind='logical',value=value)
        return dict(**result,value=observed,status='agree' if observed==case['expected'] else 'divergent-value')

def main():
    oracle=GeneratedPlacementOracle();sql=SqliteEpsilonOracle(oracle.model)
    functions=[oracle.observe(case) for case in data['functions']];arithmetic=[sql.observe(case) for case in data['functions']]
    logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(oracle.model,logger,express_rules=True)
    helper_sources={owner:hashlib.sha256(inspect.getsource(getattr(rules,next(name for name in dir(rules) if name.upper()==owner))).encode('utf-8')).hexdigest() for owner in sorted(set(case['owner'] for case in data['functions']))}
    result=dict(python=platform.python_version(),ifcopenshell=ifcopenshell.version,sqlite=sqlite3.sqlite_version,inputSha256=hashlib.sha256((root/'cases.json').read_bytes()).hexdigest(),sourceSha256=hashlib.sha256((root/'source.ifc').read_bytes()).hexdigest(),helperSourceSha256=helper_sources,functions=functions,arithmetic=arithmetic,physicalDiagnostics=logger.statements)
    (ticket/'🗑️generated/issued-express-placement-oracle.json').write_text(json.dumps(result,indent=2,default=str)+'\n',encoding='utf-8')
    print('[DEBUG] '+json.dumps(dict(functions={status:sum(row['status']==status for row in functions) for status in ['agree','divergent-value','unsupported']},sql={status:sum(row['status']==status for row in arithmetic) for status in ['agree','divergent-value','unsupported']},physicalDiagnostics=len(logger.statements))))

if __name__=='__main__':main()
