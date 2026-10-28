"""🧭 Observe generated helpers and independent relational classification through owned test interfaces."""
import hashlib, json, platform, sqlite3
from pathlib import Path
from typing import Protocol
import ifcopenshell
import ifcopenshell.validate
import ifcopenshell.express.rules.IFC4X3_ADD2 as rules

ticket=Path(__file__).resolve().parent
repo=ticket.parents[6]
root=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️classification'
data=json.loads((root/'cases.json').read_text(encoding='utf-8'))
raw=(root/'source.ifc').read_bytes()
logical=lambda value:dict(kind='logical',value=str(value).lower())

class ClassificationOracle(Protocol):
    def observe(self,case:dict)->dict: ...

class GeneratedHelperOracle:
    def observe(self,case):
        model=ifcopenshell.file.from_string(raw.decode('utf-8'))
        before=model.to_string()
        def decode(arg):
            if arg['kind']=='unknown':return None
            if arg['kind']=='string':return arg['value']
            if arg['kind']=='entity':return model.by_id(int(arg['id']))
            if arg['kind']=='set':return rules.express_set([model.by_id(int(id)) for id in arg['ids']])
            raise ValueError('Undeclared oracle argument')
        result=dict(owner=case['owner'],id=case['id'],parameterDomain=case['parameterDomain'])
        try:
            helper=getattr(rules,next(name for name in dir(rules) if name.upper()==case['owner']))
            value=helper(*[decode(arg) for arg in case['args']])
            if value is rules.INDETERMINATE or value is None:observed=dict(kind='indeterminate')
            elif value is rules.unknown or isinstance(value,str) and value=='UNKNOWN':observed=logical('unknown')
            elif isinstance(value,bool):observed=logical(value)
            else:raise TypeError('Unrepresented helper scalar '+type(value).__name__+': '+repr(value))
            result.update(value=observed,rawType=type(value).__name__,rawScalar=value if isinstance(value,(str,bool,int,float)) or value is None else None,status=('storage-only' if value is None else 'exact') if observed==case['expected'] else 'divergent')
        except Exception as error:result.update(status='unsupported',error=dict(type=type(error).__name__,message=str(error)))
        result['sourceUnchanged']=before==model.to_string()
        return result

class RelationalOracle:
    def __init__(self):
        self.model=ifcopenshell.file.from_string(raw.decode('utf-8'))
        self.db=sqlite3.connect(':memory:')
        self.db.executescript('CREATE TABLE types(id INTEGER,name TEXT); CREATE TABLE dimensions(id INTEGER,n INTEGER); CREATE TABLE members(parent INTEGER,ordinal INTEGER,child INTEGER); CREATE TABLE units(id INTEGER,role TEXT); CREATE TABLE profiles(id INTEGER,parent INTEGER); CREATE TABLE current(id INTEGER); CREATE TABLE edges(ordinal INTEGER,start INTEGER,end INTEGER); CREATE TABLE policy(label TEXT,name TEXT,dimension INTEGER);')
        schema=ifcopenshell.ifcopenshell_wrapper.schema_by_name('IFC4X3_ADD2')
        parents={}
        for declaration in schema.declarations():
            select=declaration.as_select_type()
            if select:
                for member in select.select_list():parents.setdefault(member.name().upper(),[]).append(select.name().upper())
        for entity in self.model:
            declaration=schema.declaration_by_name(entity.is_a()).as_entity()
            names=set()
            while declaration:
                names.add(declaration.name().upper())
                declaration=declaration.supertype()
            pending=list(names)
            while pending:
                for parent in parents.get(pending.pop(),[]):
                    if parent not in names:names.add(parent);pending.append(parent)
            self.db.executemany('INSERT INTO types VALUES(?,?)',[(entity.id(),name) for name in names])
            try:dimension=entity.Dim
            except (AttributeError,TypeError):dimension=None
            if dimension is not None:self.db.execute('INSERT INTO dimensions VALUES(?,?)',(entity.id(),dimension))
            if entity.is_a('IfcGeometricSet'):
                self.db.executemany('INSERT INTO members VALUES(?,?,?)',[(entity.id(),i,item.id()) for i,item in enumerate(entity.Elements)])
            if entity.is_a('IfcNamedUnit') or entity.is_a('IfcDerivedUnit'):self.db.execute('INSERT INTO units VALUES(?,?)',(entity.id(),entity.UnitType))
            if entity.is_a('IfcDerivedProfileDef'):self.db.execute('INSERT INTO profiles VALUES(?,?)',(entity.id(),entity.ParentProfile.id()))
        policy={'Point':['IFCPOINT','IFCCARTESIANPOINTLIST'],'PointCloud':['IFCCARTESIANPOINTLIST3D'],'Curve':['IFCCURVE'],'Curve2D':['IFCCURVE'],'Curve3D':['IFCCURVE'],'Segment':['IFCSEGMENT'],'Surface':['IFCSURFACE'],'Surface2D':['IFCSURFACE'],'Surface3D':['IFCSURFACE'],'SectionedSurface':['IFCSECTIONEDSURFACE'],'FillArea':['IFCANNOTATIONFILLAREA'],'Text':['IFCTEXTLITERAL'],'AdvancedSurface':['IFCBSPLINESURFACE'],'Annotation2D':['IFCPOINT','IFCCURVE','IFCGEOMETRICCURVESET','IFCANNOTATIONFILLAREA','IFCTEXTLITERAL'],'GeometricSet':['IFCGEOMETRICSET','IFCPOINT','IFCCURVE','IFCSURFACE'],'GeometricCurveSet':['IFCGEOMETRICCURVESET','IFCGEOMETRICSET','IFCPOINT','IFCCURVE'],'Tessellation':['IFCTESSELLATEDITEM'],'SurfaceOrSolidModel':['IFCTESSELLATEDITEM','IFCSHELLBASEDSURFACEMODEL','IFCFACEBASEDSURFACEMODEL','IFCSOLIDMODEL'],'SurfaceModel':['IFCTESSELLATEDITEM','IFCSHELLBASEDSURFACEMODEL','IFCFACEBASEDSURFACEMODEL'],'SolidModel':['IFCSOLIDMODEL'],'SweptSolid':['IFCEXTRUDEDAREASOLID','IFCREVOLVEDAREASOLID'],'AdvancedSweptSolid':['IFCSWEPTAREASOLID','IFCSWEPTDISKSOLID','IFCSECTIONEDSOLIDHORIZONTAL'],'CSG':['IFCBOOLEANRESULT','IFCCSGPRIMITIVE3D','IFCCSGSOLID'],'Clipping':['IFCCSGSOLID','IFCBOOLEANCLIPPINGRESULT'],'Brep':['IFCFACETEDBREP'],'AdvancedBrep':['IFCMANIFOLDSOLIDBREP'],'BoundingBox':['IFCBOUNDINGBOX'],'SectionedSpine':['IFCSECTIONEDSPINE'],'LightSource':['IFCLIGHTSOURCE'],'MappedRepresentation':['IFCMAPPEDITEM']}
        for label,names in policy.items():
            dimension=2 if label in ('Curve2D','Surface2D') else 3 if label in ('Curve3D','Surface3D') else None
            self.db.executemany('INSERT INTO policy VALUES(?,?,?)',[(label,name,dimension) for name in names])
    def count(self,query,args=()):return self.db.execute(query,args).fetchone()[0]
    def has(self,id,name):return bool(self.count('SELECT COUNT(*) FROM types WHERE id=? AND name=?',(id,name)))
    def observe(self,case):
        owner,args=case['owner'],case['args']
        result=dict(owner=owner,id=case['id'],parameterDomain=case['parameterDomain'])
        if owner in ('IFCLOOPHEADTOTAIL','IFCPATHHEADTOTAIL'):
            self.db.execute('DELETE FROM edges')
            source=None if args[0]['kind']=='unknown' else self.model.by_id(int(args[0]['id']))
            if source:
                for i,edge in enumerate(source.EdgeList):
                    base=edge.EdgeElement
                    a,b=(base.EdgeStart.id(),base.EdgeEnd.id()) if edge.Orientation else (base.EdgeEnd.id(),base.EdgeStart.id())
                    self.db.execute('INSERT INTO edges VALUES(?,?,?)',(i,a,b))
            broken=self.count('SELECT COUNT(*) FROM edges a JOIN edges b ON b.ordinal=a.ordinal+1 WHERE a.end<>b.start')
            observed=logical(False if broken else True if owner=='IFCLOOPHEADTOTAIL' else 'unknown')
        elif owner=='IFCTAPEREDSWEPTAREAPROFILES':
            start=None if args[0]['kind']=='unknown' else int(args[0]['id']);end=None if args[1]['kind']=='unknown' else int(args[1]['id'])
            if self.has(end,'IFCDERIVEDPROFILEDEF'):
                parent=self.db.execute('SELECT parent FROM profiles WHERE id=?',(end,)).fetchone()[0]
                observed=logical('unknown' if start is None else start==parent)
            elif self.has(start,'IFCPARAMETERIZEDPROFILEDEF'):
                mismatch=self.count('SELECT COUNT(*) FROM (SELECT name FROM types WHERE id=? EXCEPT SELECT name FROM types WHERE id=?)',(start,end))+self.count('SELECT COUNT(*) FROM (SELECT name FROM types WHERE id=? EXCEPT SELECT name FROM types WHERE id=?)',(end,start))
                observed=logical(mismatch==0)
            else:observed=logical(False)
        else:
            items=args[-1]['ids']
            self.db.execute('DELETE FROM current')
            self.db.executemany('INSERT INTO current VALUES(?)',[(int(id),) for id in items])
            if owner=='IFCCORRECTFILLAREASTYLE':
                counts={name:self.count('SELECT COUNT(*) FROM current c JOIN types t ON t.id=c.id WHERE t.name=?',(name,)) for name in ['IFCEXTERNALLYDEFINEDHATCHSTYLE','IFCFILLAREASTYLEHATCHING','IFCFILLAREASTYLETILES','IFCCOLOUR']}
                e,h,t,c=[counts[name] for name in ['IFCEXTERNALLYDEFINEDHATCHSTYLE','IFCFILLAREASTYLEHATCHING','IFCFILLAREASTYLETILES','IFCCOLOUR']]
                observed=logical(not(e>1 or e==1 and(h>0 or t>0 or c>0) or c>1 or h>0 and t>0))
            elif owner=='IFCCORRECTUNITASSIGNMENT':
                good=True
                for name in ['IFCNAMEDUNIT','IFCDERIVEDUNIT']:
                    n,distinct=self.db.execute('SELECT COUNT(*),COUNT(DISTINCT u.role) FROM current c JOIN types t ON t.id=c.id JOIN units u ON u.id=c.id WHERE t.name=? AND u.role<>?',(name,'USERDEFINED')).fetchone()
                    good=good and n==distinct
                observed=logical(good and self.count("SELECT COUNT(*) FROM current c JOIN types t ON t.id=c.id WHERE t.name='IFCMONETARYUNIT'")<=1)
            elif owner=='IFCSHAPEREPRESENTATIONTYPES':
                label=args[0]['value']
                if not self.count('SELECT COUNT(*) FROM policy WHERE label=?',(label,)):observed=dict(kind='indeterminate')
                else:
                    accepted=self.count('SELECT COUNT(DISTINCT c.id) FROM current c JOIN types t ON t.id=c.id JOIN policy p ON p.name=t.name AND p.label=? LEFT JOIN dimensions d ON d.id=c.id WHERE (p.dimension IS NULL OR p.dimension=d.n) AND (?<>\'SweptSolid\' OR NOT EXISTS(SELECT 1 FROM types excluded WHERE excluded.id=c.id AND excluded.name IN (\'IFCEXTRUDEDAREASOLIDTAPERED\',\'IFCREVOLVEDAREASOLIDTAPERED\'))) AND (?<>\'GeometricCurveSet\' OR NOT EXISTS(SELECT 1 FROM members m JOIN types child ON child.id=m.child WHERE m.parent=c.id AND child.name=\'IFCSURFACE\'))',(label,label,label))
                    if label=='BoundingBox' and len(items)>1:accepted=0
                    observed=logical(accepted==len(items))
            elif owner=='IFCTOPOLOGYREPRESENTATIONTYPES':
                label=args[0]['value'];families={'Vertex':['IFCVERTEX'],'Edge':['IFCEDGE'],'Path':['IFCPATH'],'Face':['IFCFACE'],'Shell':['IFCOPENSHELL','IFCCLOSEDSHELL']}
                if label=='Undefined':observed=logical(True)
                elif label not in families:observed=dict(kind='indeterminate')
                else:observed=logical(self.count('SELECT COUNT(DISTINCT c.id) FROM current c JOIN types t ON t.id=c.id WHERE t.name IN ('+','.join('?' for _ in families[label])+')',families[label])==len(items))
            else:raise ValueError('Unrepresented relational owner')
        return dict(**result,value=observed,status='exact' if observed==case['expected'] else 'divergent')

def main():
    sdk=GeneratedHelperOracle();sql=RelationalOracle()
    helpers=[sdk.observe(case) for case in data['functions']]
    relational=[sql.observe(case) for case in data['functions']]
    model=ifcopenshell.file.from_string(raw.decode('utf-8'))
    logger=ifcopenshell.validate.json_logger()
    ifcopenshell.validate.validate(model,logger,express_rules=True)
    output=dict(python=platform.python_version(),ifcOpenShell=ifcopenshell.version,sqlite=sqlite3.sqlite_version,casesSha256=hashlib.sha256((root/'cases.json').read_bytes()).hexdigest(),sourceSha256=hashlib.sha256(raw).hexdigest(),entities=len(list(model)),helpers=helpers,relational=relational,validation=logger.statements,nativeExecuted=False,branchCoverageComplete=False)
    (ticket/'🗑️generated/issued-express-classification-oracle.json').write_text(json.dumps(output,indent=2,default=str)+'\n',encoding='utf-8')
    print('[DEBUG] '+json.dumps(dict(helpers={status:sum(row['status']==status for row in helpers) for status in ['exact','storage-only','divergent','unsupported']},sqlExact=sum(row['status']=='exact' for row in relational),sourceUnchanged=sum(row['sourceUnchanged'] for row in helpers),diagnostics=len(logger.statements))))

if __name__=='__main__':main()
