"""🧮 Retain actual derived-array helpers, source diagnostics and independent scalar scans."""
import hashlib
import json
import platform
import sqlite3
import traceback
from pathlib import Path
import ifcopenshell
import ifcopenshell.express.rules.IFC4X3_ADD2 as rules
import ifcopenshell.validate

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
root=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️aggregate'
data=json.loads((root/'cases.json').read_text(encoding='utf-8'))

class AggregateOracle:
    def observe(self,case):raise NotImplementedError

class GeneratedAggregateOracle(AggregateOracle):
    def __init__(self):self.model=ifcopenshell.open(str(root/'source.ifc'))
    def observe(self,case):
        result=dict(id=case['id'],owner=case['owner'])
        try:
            before=self.model.to_string();args=[]
            for row in case['args']:
                value=self.model.by_id(int(row['id']));args.append(getattr(value,row['name'].title()) if row['kind']=='attribute' else value)
            fn=getattr(rules,next(name for name in dir(rules) if name.upper()==case['owner']))
            value=fn(*args)
            observed=dict(kind='logical',value=str(value).lower()) if isinstance(value,bool) else dict(kind='real',value=str(int(value)) if value.is_integer() else str(value)) if isinstance(value,float) else dict(kind='unknown') if value is None else None
            if observed is None:raise TypeError('Unrepresented external result '+type(value).__name__)
            unchanged=self.model.to_string()==before;result.update(value=observed,rawType=type(value).__name__,sourceUnchanged=unchanged,status='agree' if unchanged and observed==case['expected'] else 'divergent-value')
        except Exception as error:result.update(status='unsupported',error=dict(kind=type(error).__name__,message=str(error),trace=[dict(function=frame.name,file=Path(frame.filename).name,line=frame.lineno) for frame in traceback.extract_tb(error.__traceback__)]))
        return result

class SqliteAggregateOracle(AggregateOracle):
    def __init__(self,model):self.model=model
    def observe(self,case):
        owner=case['owner'];result=dict(id=case['id'],owner=owner);entity=self.model.by_id(int(case['sourceEntity']))
        with sqlite3.connect(':memory:') as db:
            if owner in ('IFCCURVEWEIGHTSPOSITIVE','IFCSURFACEWEIGHTSPOSITIVE'):
                values=list(entity.WeightsData);values=[n for row in values for n in row] if owner=='IFCSURFACEWEIGHTSPOSITIVE' else values
                db.execute('CREATE TABLE weights(i INTEGER,w REAL)');db.executemany('INSERT INTO weights VALUES(?,?)',enumerate(values));flag=db.execute('SELECT COUNT(*)=0 FROM weights WHERE w<=0').fetchone()[0];observed=dict(kind='logical',value=str(bool(flag)).lower())
            elif owner=='IFCCONSECUTIVESEGMENTS':
                segments=[list(value.wrappedValue) for value in entity.Segments];db.execute('CREATE TABLE segments(i INTEGER,first INTEGER,last INTEGER)');db.executemany('INSERT INTO segments VALUES(?,?,?)',[(i,row[0],row[-1]) for i,row in enumerate(segments)]);flag=db.execute('SELECT COUNT(*)=0 FROM (SELECT first,LAG(last) OVER(ORDER BY i) previous FROM segments) WHERE previous IS NOT NULL AND first!=previous').fetchone()[0];observed=dict(kind='logical',value=str(bool(flag)).lower())
            else:
                db.execute('CREATE TABLE layers(i INTEGER,t REAL)');db.executemany('INSERT INTO layers VALUES(?,?)',[(i,row.LayerThickness) for i,row in enumerate(entity.MaterialLayers)]);value=db.execute('SELECT SUM(t) FROM layers').fetchone()[0];observed=dict(kind='real',value=str(int(value)) if value.is_integer() else str(value))
        return dict(**result,value=observed,status='agree' if observed==case['expected'] else 'divergent-value')

def main():
    oracle=GeneratedAggregateOracle();sql=SqliteAggregateOracle(oracle.model);functions=[oracle.observe(row) for row in data['functions']];arithmetic=[sql.observe(row) for row in data['functions']]
    logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(oracle.model,logger,express_rules=True)
    cases={int(row['sourceEntity']):row for row in data['functions']};statuses={(row['owner'],row['id']):row['status'] for row in functions};classification=[]
    for diagnostic in logger.statements:
        entity=diagnostic.get('instance');id=entity.id() if hasattr(entity,'id') else None;case=cases.get(id)
        category='external-helper-error' if case and statuses[(case['owner'],case['id'])]=='unsupported' else 'expected-negative-rule' if case and case['schemaRole']=='negative' else 'unexpected'
        classification.append(dict(entityId=str(id),category=category,attribute=diagnostic.get('attribute'),sourceRole=case['schemaRole'] if case else None))
    result=dict(python=platform.python_version(),ifcopenshell=ifcopenshell.version,sqlite=sqlite3.sqlite_version,sourceEntities=len(list(oracle.model)),inputSha256=hashlib.sha256((root/'cases.json').read_bytes()).hexdigest(),sourceSha256=hashlib.sha256((root/'source.ifc').read_bytes()).hexdigest(),functions=functions,arithmetic=arithmetic,expectedNegativeEntityIds=[row['sourceEntity'] for row in data['functions'] if row['schemaRole']=='negative'],physicalDiagnostics=logger.statements,physicalClassification=classification)
    (ticket/'🗑️generated/issued-express-weights-oracle.json').write_text(json.dumps(result,indent=2,default=str)+'\n',encoding='utf-8')
    print('[DEBUG] '+json.dumps(dict(functions={status:sum(row['status']==status for row in functions) for status in ['agree','divergent-value','unsupported']},sql={status:sum(row['status']==status for row in arithmetic) for status in ['agree','divergent-value','unsupported']},physicalDiagnostics=len(logger.statements))))

if __name__=='__main__':main()
