"""🌊 Observe fresh surface/curve helpers and independent SQL identity/dimension projections."""
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
root=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️surface-curve'
data=json.loads((root/'cases.json').read_text(encoding='utf-8'));source=root/'source.ifc'

class SurfaceCurveOracle:
    def observe(self,case):raise NotImplementedError

class GeneratedSurfaceCurveOracle(SurfaceCurveOracle):
    def observe(self,case):
        model=ifcopenshell.open(str(source));before=model.to_string();result=dict(id=case['id'],owner=case['owner'])
        try:
            arg=model.by_id(int(case['args'][0]['id'])) if case['args'][0]['kind']=='entity' else None
            helper=getattr(rules,next(name for name in dir(rules) if name.upper()==case['owner']));value=helper(arg)
            if value is None or value is rules.INDETERMINATE:observed=dict(kind='unknown')
            elif case['owner']=='IFCGETBASISSURFACE':observed=dict(kind='surfaces',value=[str(id) for id in sorted(item.id() for item in value)])
            elif case['owner']=='IFCASSOCIATEDSURFACE':observed=dict(kind='surface',value=str(value.id()))
            else:observed=dict(kind='integer',value=str(value))
            result.update(value=observed,status='agree' if observed==case['expected'] else 'divergent-value',rawType=type(value).__name__,rawRepresentation=str(value))
        except Exception as error:result.update(status='unsupported',error=dict(kind=type(error).__name__,message=str(error)))
        result['sourceUnchanged']=model.to_string()==before
        if not result['sourceUnchanged']:result['sourceAfter']=model.to_string()
        return result

class SqliteSurfaceCurveOracle(SurfaceCurveOracle):
    def __init__(self):self.model=ifcopenshell.open(str(source))
    def prepare(self,db):
        db.executescript('CREATE TABLE nodes(id INTEGER PRIMARY KEY,type TEXT,raw_count INTEGER,z_present INTEGER,position_count INTEGER); CREATE TABLE ancestry(id INTEGER,name TEXT,PRIMARY KEY(id,name)); CREATE TABLE refs(id INTEGER,role TEXT,ordinal INTEGER,target INTEGER,PRIMARY KEY(id,role,ordinal)); CREATE TABLE members(id INTEGER,member INTEGER,surface INTEGER,PRIMARY KEY(id,member,surface));')
        families=['IfcCartesianPoint','IfcCartesianPointList','IfcPlacement','IfcLine','IfcConic','IfcPolyline','IfcTrimmedCurve','IfcGradientCurve','IfcSegmentedReferenceCurve','IfcCompositeCurve','IfcBSplineCurve','IfcOffsetCurve2D','IfcOffsetCurve3D','IfcOffsetCurveByDistances','IfcPolynomialCurve','IfcPcurve','IfcIndexedPolyCurve','IfcSpiral','IfcCurveSegment','IfcCompositeCurveSegment','IfcSurfaceCurve','IfcCompositeCurveOnSurface']
        roles=[('IfcPlacement','Location'),('IfcLine','Pnt'),('IfcConic','Position'),('IfcPolyline','Points'),('IfcTrimmedCurve','BasisCurve'),('IfcCompositeCurve','Segments'),('IfcBSplineCurve','ControlPointsList'),('IfcPolynomialCurve','Position'),('IfcPcurve','BasisSurface'),('IfcIndexedPolyCurve','Points'),('IfcSpiral','Position'),('IfcCurveSegment','ParentCurve'),('IfcCompositeCurveSegment','ParentCurve'),('IfcSurfaceCurve','AssociatedGeometry')]
        for entity in self.model:
            count=len(entity.Coordinates) if entity.is_a('IfcCartesianPoint') else len(entity.CoordList[0]) if entity.is_a('IfcCartesianPointList') else None
            z=int(entity.CoefficientsZ is not None) if entity.is_a('IfcPolynomialCurve') else None
            position_count=len(entity.Position.Location.Coordinates) if entity.is_a('IfcPolynomialCurve') else None
            db.execute('INSERT INTO nodes VALUES(?,?,?,?,?)',(entity.id(),entity.is_a(),count,z,position_count))
            db.executemany('INSERT INTO ancestry VALUES(?,?)',[(entity.id(),name) for name in families if entity.is_a(name)])
            for family,role in roles:
                if entity.is_a(family):
                    value=getattr(entity,role);items=value if isinstance(value,(list,tuple)) else [value]
                    db.executemany('INSERT INTO refs VALUES(?,?,?,?)',[(entity.id(),role,ordinal,item.id()) for ordinal,item in enumerate(items) if item is not None])
    def family(self,db,id,name):return bool(db.execute('SELECT 1 FROM ancestry WHERE id=? AND name=?',(id,name)).fetchone())
    def surface_set(self,db,id,depth=0):
        if id is None:return []
        if depth>64:raise ValueError('Relational surface source cycle/depth outside interface')
        if self.family(db,id,'IfcPcurve'):return [row[0] for row in db.execute("SELECT target FROM refs WHERE id=? AND role='BasisSurface'",(id,))]
        if self.family(db,id,'IfcSurfaceCurve'):return [row[0] for row in db.execute("SELECT DISTINCT p.target FROM refs a JOIN refs p ON p.id=a.target AND p.role='BasisSurface' WHERE a.id=? AND a.role='AssociatedGeometry' ORDER BY p.target",(id,))]
        if not self.family(db,id,'IfcCompositeCurveOnSurface'):return []
        segments=[row[0] for row in db.execute("SELECT target FROM refs WHERE id=? AND role='Segments' ORDER BY ordinal",(id,))]
        db.execute('DELETE FROM members WHERE id=?',(id,))
        for ordinal,segment in enumerate(segments):
            parent=db.execute("SELECT target FROM refs WHERE id=? AND role='ParentCurve'",(segment,)).fetchone()
            values=self.surface_set(db,parent[0] if parent else None,depth+1)
            db.executemany('INSERT INTO members VALUES(?,?,?)',[(id,ordinal,value) for value in values])
        return [row[0] for row in db.execute('SELECT surface FROM members WHERE id=? GROUP BY surface HAVING COUNT(DISTINCT member)=? ORDER BY surface',(id,len(segments)))]
    def dimension(self,db,id):
        if id is None:return None
        predicates={name:"EXISTS(SELECT 1 FROM ancestry a WHERE a.id=n.id AND a.name='"+name+"')" for name in ['IfcCartesianPoint','IfcCartesianPointList','IfcPlacement','IfcLine','IfcConic','IfcPolyline','IfcTrimmedCurve','IfcGradientCurve','IfcSegmentedReferenceCurve','IfcCompositeCurve','IfcBSplineCurve','IfcOffsetCurve2D','IfcOffsetCurve3D','IfcOffsetCurveByDistances','IfcPolynomialCurve','IfcPcurve','IfcIndexedPolyCurve','IfcSpiral','IfcCurveSegment','IfcCompositeCurveSegment']}
        def has(name):return predicates[name]
        roles=[('IfcLine','Pnt'),('IfcConic','Position'),('IfcPolyline','Points'),('IfcTrimmedCurve','BasisCurve'),('IfcCompositeCurve','Segments'),('IfcBSplineCurve','ControlPointsList'),('IfcIndexedPolyCurve','Points'),('IfcSpiral','Position'),('IfcCurveSegment','ParentCurve'),('IfcCompositeCurveSegment','ParentCurve'),('IfcPlacement','Location')]
        fixed=' OR '.join(has(name) for name in ['IfcGradientCurve','IfcSegmentedReferenceCurve','IfcOffsetCurve3D','IfcOffsetCurveByDistances','IfcPcurve'])
        scalar=f"CASE WHEN {has('IfcCartesianPoint')} OR {has('IfcCartesianPointList')} THEN n.raw_count WHEN {has('IfcGradientCurve')} OR {has('IfcSegmentedReferenceCurve')} THEN 3 WHEN {has('IfcOffsetCurve2D')} THEN 2 WHEN {fixed} THEN 3 WHEN {has('IfcPolynomialCurve')} THEN CASE WHEN n.z_present=0 AND n.position_count=2 THEN 2 ELSE 3 END END"
        next_ref='CASE '+' '.join(f"WHEN {has(family)} THEN (SELECT target FROM refs WHERE id=n.id AND role='{role}' AND ordinal=0)" for family,role in roles)+' END'
        query=f'WITH RECURSIVE chain(id,depth,value) AS (SELECT ?,0,NULL UNION ALL SELECT {next_ref},c.depth+1,{scalar} FROM chain c JOIN nodes n ON n.id=c.id WHERE c.depth<64 AND c.value IS NULL) SELECT value FROM chain WHERE value IS NOT NULL LIMIT 1'
        row=db.execute(query,(id,)).fetchone();return row[0] if row else None
    def observe(self,case):
        id=int(case['args'][0]['id']) if case['args'][0]['kind']=='entity' else None
        with sqlite3.connect(':memory:') as db:
            self.prepare(db)
            if case['owner']=='IFCASSOCIATEDSURFACE':rows=db.execute("SELECT target FROM refs WHERE id=? AND role='BasisSurface'",(id,)).fetchall();value=dict(kind='surface',value=str(rows[0][0])) if rows else dict(kind='unknown')
            elif case['owner']=='IFCGETBASISSURFACE':value=dict(kind='surfaces',value=[str(item) for item in self.surface_set(db,id)])
            else:dim=self.dimension(db,id);value=dict(kind='integer',value=str(dim)) if dim is not None else dict(kind='unknown')
        return dict(id=case['id'],owner=case['owner'],value=value,status='agree' if value==case['expected'] else 'divergent-value')

def main():
    generated=GeneratedSurfaceCurveOracle();sql=SqliteSurfaceCurveOracle();functions=[generated.observe(case) for case in data['functions']];relational=[]
    for case in data['functions']:
        try:relational.append(sql.observe(case))
        except Exception as error:relational.append(dict(id=case['id'],owner=case['owner'],status='unsupported',error=dict(kind=type(error).__name__,message=str(error))))
    model=ifcopenshell.open(str(source));logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(model,logger,express_rules=True)
    classifications=[]
    for diagnostic in logger.statements:
        instance=diagnostic.get('instance');id=instance.id() if hasattr(instance,'id') else None;attribute=diagnostic.get('attribute','');message=diagnostic.get('message','')
        kind='expected-rule-false' if id==89 and attribute=='IfcPolynomialCurve.CorrectPositionDim' else 'helper-evaluation-error' if attribute=='IfcCompositeCurveOnSurface.SameSurface' and ('multiply sequence' in message or 'concatenate list' in message) else 'unexplained'
        classifications.append(dict(entityId=str(id),attribute=attribute,kind=kind,declaredNegative=id in [62,64,69,89]))
    with sqlite3.connect(':memory:') as db:
        sql.prepare(db);source_rules=[dict(entityId=str(entity.id()),owner='IFCCOMPOSITECURVEONSURFACE',rule='SAMESURFACE',value=bool(sql.surface_set(db,entity.id()))) for entity in model if entity.is_a('IfcCompositeCurveOnSurface')]
        source_rules += [dict(entityId=str(id),owner='IFCPOLYNOMIALCURVE',rule='CORRECTPOSITIONDIM',value=bool(value)) for id,value in db.execute("SELECT id,((position_count=2 AND z_present=0) OR position_count=3) FROM nodes WHERE type='IfcPolynomialCurve'")]
    helpers=[dict(owner=owner,sha256=hashlib.sha256(inspect.getsource(getattr(rules,next(name for name in dir(rules) if name.upper()==owner))).encode()).hexdigest()) for owner in sorted({row['owner'] for row in data['functions']})]
    result=dict(python=platform.python_version(),ifcopenshell=ifcopenshell.version,sqlite=sqlite3.sqlite_version,casesSha256=hashlib.sha256((root/'cases.json').read_bytes()).hexdigest(),sourceSha256=hashlib.sha256(source.read_bytes()).hexdigest(),functions=functions,relational=relational,helpers=helpers,physicalDiagnostics=logger.statements,diagnosticClassifications=classifications,sourceRuleProjections=source_rules,surfaceIdentityObservation=dict(first=model.by_id(20).id(),second=model.by_id(21).id(),sameExplicitPosition=model.by_id(20).Position.id()==model.by_id(21).Position.id()))
    (ticket/'🗑️generated/issued-express-surface-curve-oracle.json').write_text(json.dumps(result,indent=2,default=str)+'\n',encoding='utf-8')
    print('[DEBUG] '+json.dumps(dict(functions={status:sum(row['status']==status for row in functions) for status in ['agree','divergent-value','unsupported']},sql={status:sum(row['status']==status for row in relational) for status in ['agree','divergent-value','unsupported']},physicalDiagnostics=len(logger.statements))))

if __name__=='__main__':main()
