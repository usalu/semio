"""🧮 Author closed language-neutral primary-source numeric boundaries without translating executable owners."""
import json
from pathlib import Path

ticket = Path(__file__).parent
repo = next(p for p in ticket.parents if (p / 'AGENTS.md').exists())
target = repo / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️numeric'
target.mkdir(exist_ok=True)
names = ['LengthExponent','MassExponent','TimeExponent','ElectricCurrentExponent','ThermodynamicTemperatureExponent','AmountOfSubstanceExponent','LuminousIntensityExponent']
def integer(n): return dict(kind='integer',value=str(n))
def real(n): return dict(kind='real',value=str(n))
def logical(n): return dict(kind='logical',value=n)
def enum(n): return dict(kind='enum',value=n)
def entity(name,fields): return dict(kind='entity',type=name,fields=fields)
def aggregate(items,kind='LIST'): return dict(kind='aggregate',aggregate=kind,lower='1',items=items)
def dimensions(values): return entity('IFCDIMENSIONALEXPONENTS',dict(zip(names,map(integer,values))))
def observed(values): return dict(kind='attributes',values=dict(zip(names,map(integer,values))))
rows=[]
def case(id,owner,args,expected,branch,projection=None):
    row=dict(id=id,owner=owner,args=args,expected=expected,branch=branch)
    if projection: row['projection']=projection
    rows.append(row)
def spline(id,d,u,c,m,k,expected,branch): case(id,'IFCCONSTRAINTSPARAMBSPLINE',[integer(d),integer(u),integer(c),aggregate(list(map(integer,m))),aggregate(list(map(real,k)))],logical(str(expected).lower()),branch)
spline('spline-clamped',2,2,2,[3,3],[0,1],True,'minimum clamped knot count and endpoint maximum')
spline('spline-interior',2,3,4,[3,2,3],[0,.5,1],True,'interior multiplicity equals degree')
spline('spline-degree-zero',0,2,2,[2,2],[0,1],False,'Degree lower bound')
spline('spline-one-knot',1,1,1,[4],[0],False,'UpKnots lower bound')
spline('spline-few-controls',3,2,2,[3,4],[0,1],False,'UpCp less than degree')
spline('spline-sum-small',2,2,2,[2,3],[0,1],False,'multiplicity sum below degree plus controls plus two')
spline('spline-sum-large',2,2,2,[3,4],[0,1],False,'multiplicity sum above degree plus controls plus two')
spline('spline-first-zero',2,3,2,[0,3,3],[0,.5,1],False,'first multiplicity lower bound after sum passes')
spline('spline-first-large',2,3,4,[4,1,3],[0,.5,1],False,'first multiplicity upper bound after sum passes')
spline('spline-interior-zero',2,3,2,[3,0,3],[0,.5,1],False,'interior multiplicity lower bound')
spline('spline-interior-large',2,3,5,[3,3,3],[0,.5,1],False,'interior multiplicity upper bound')
spline('spline-last-zero',2,4,2,[3,2,1,0],[0,1,2,3],False,'last multiplicity lower bound after all prior checks pass')
spline('spline-last-large',2,3,4,[3,1,4],[0,.5,1],False,'last multiplicity upper bound')
spline('spline-equal-knots',2,3,4,[3,2,3],[0,0,1],False,'nonincreasing equal knots')
spline('spline-descending-last',2,3,4,[3,2,3],[0,1,.5],False,'late decreasing knot')
spline('spline-negative-origin',2,3,4,[3,2,3],[-2,-1,0],True,'strict order permits negative knot origin')
d=9007199254740993
spline('spline-exact-large-integer',d,2,d,[d+1,d+1],[0,1],True,'exact INTEGER greater than binary64 consecutive precision')
si = {
'METRE':[1,0,0,0,0,0,0],'SQUARE_METRE':[2,0,0,0,0,0,0],'CUBIC_METRE':[3,0,0,0,0,0,0],'GRAM':[0,1,0,0,0,0,0],
'SECOND':[0,0,1,0,0,0,0],'AMPERE':[0,0,0,1,0,0,0],'KELVIN':[0,0,0,0,1,0,0],'MOLE':[0,0,0,0,0,1,0],'CANDELA':[0,0,0,0,0,0,1],
'RADIAN':[0,0,0,0,0,0,0],'STERADIAN':[0,0,0,0,0,0,0],'HERTZ':[0,0,-1,0,0,0,0],'NEWTON':[1,1,-2,0,0,0,0],'PASCAL':[-1,1,-2,0,0,0,0],
'JOULE':[2,1,-2,0,0,0,0],'WATT':[2,1,-3,0,0,0,0],'COULOMB':[0,0,1,1,0,0,0],'VOLT':[2,1,-3,-1,0,0,0],'FARAD':[-2,-1,4,2,0,0,0],
'OHM':[2,1,-3,-2,0,0,0],'SIEMENS':[-2,-1,3,2,0,0,0],'WEBER':[2,1,-2,-1,0,0,0],'TESLA':[0,1,-2,-1,0,0,0],'HENRY':[2,1,-2,-2,0,0,0],
'DEGREE_CELSIUS':[0,0,0,0,1,0,0],'LUMEN':[0,0,0,0,0,0,1],'LUX':[-2,0,0,0,0,0,1],'BECQUEREL':[0,0,-1,0,0,0,0],'GRAY':[2,0,-2,0,0,0,0],'SIEVERT':[2,0,-2,0,0,0,0]}
for name,values in si.items(): case('si-'+name.lower(),'IFCDIMENSIONSFORSIUNIT',[enum(name)],observed(values),'released CASE '+name,names)
units = {'LENGTHUNIT':'METRE','AREAUNIT':'SQUARE_METRE','VOLUMEUNIT':'CUBIC_METRE','MASSUNIT':'GRAM','TIMEUNIT':'SECOND','ELECTRICCURRENTUNIT':'AMPERE','THERMODYNAMICTEMPERATUREUNIT':'KELVIN','AMOUNTOFSUBSTANCEUNIT':'MOLE','LUMINOUSINTENSITYUNIT':'CANDELA','PLANEANGLEUNIT':'RADIAN','SOLIDANGLEUNIT':'STERADIAN','ABSORBEDDOSEUNIT':'GRAY','RADIOACTIVITYUNIT':'BECQUEREL','ELECTRICCAPACITANCEUNIT':'FARAD','DOSEEQUIVALENTUNIT':'SIEVERT','ELECTRICCHARGEUNIT':'COULOMB','ELECTRICCONDUCTANCEUNIT':'SIEMENS','ELECTRICVOLTAGEUNIT':'VOLT','ELECTRICRESISTANCEUNIT':'OHM','ENERGYUNIT':'JOULE','FORCEUNIT':'NEWTON','FREQUENCYUNIT':'HERTZ','INDUCTANCEUNIT':'HENRY','ILLUMINANCEUNIT':'LUX','LUMINOUSFLUXUNIT':'LUMEN','MAGNETICFLUXUNIT':'WEBER','MAGNETICFLUXDENSITYUNIT':'TESLA','POWERUNIT':'WATT','PRESSUREUNIT':'PASCAL'}
for name,base in units.items():
    values=si[base]
    case('correct-'+name.lower(),'IFCCORRECTDIMENSIONS',[enum(name),dimensions(values)],logical('true'),'equal seven-value dimensional entity for '+name)
    wrong=values.copy();wrong[0]+=1
    case('wrong-'+name.lower(),'IFCCORRECTDIMENSIONS',[enum(name),dimensions(wrong)],logical('false'),'one exponent differs for '+name)
case('correct-userdefined','IFCCORRECTDIMENSIONS',[enum('USERDEFINED'),dimensions([0]*7)],logical('unknown'),'released OTHERWISE UNKNOWN, not missing implementation')
def element(values,exponent): return entity('IFCDERIVEDUNITELEMENT',dict(Unit=entity('IFCCONTEXTDEPENDENTUNIT',dict(Dimensions=dimensions(values),Name=dict(kind='string',value='caller-unit'))),Exponent=integer(exponent)))
for id,elements,expected,branch in [
('derive-torque',[element(si['NEWTON'],1),element(si['METRE'],1)],[2,1,-2,0,0,0,0],'two named units add dimensions'),
('derive-reciprocal',[element(si['SECOND'],-2),element(si['METRE'],1)],[1,0,-2,0,0,0,0],'negative exponent scales before sum'),
('derive-cancellation',[element(si['METRE'],3),element(si['METRE'],-3)],[0]*7,'opposite exponents cancel'),
('derive-all-axes',[element([1,-2,3,-4,5,-6,7],-2)],[-2,4,-6,8,-10,12,-14],'every exponent participates')]: case(id,'IFCDERIVEDIMENSIONALEXPONENTS',[aggregate(elements,'SET')],observed(expected),branch,names)
for id,values,out in [('project-two',[3,4],[3,4]),('project-three',[3,4,9],[3,4]),('project-signed',[-3,0,9],[-3,0])]: case(id,'IFCCONVERTDIRECTIONINTO2D',[entity('IFCDIRECTION',dict(DirectionRatios=aggregate(list(map(real,values)))))],dict(kind='attributes',values=dict(DirectionRatios=aggregate(list(map(real,out))))),'copy first two ratios without normalizing',['DirectionRatios'])
for id,flag,left,right,out in [('choose-first',True,integer(9007199254740993),integer(2),integer(9007199254740993)),('choose-second',False,dict(kind='string',value='first'),dict(kind='string',value='second'),dict(kind='string',value='second')),('choose-aggregate',True,aggregate([integer(1),integer(2)]),aggregate([integer(3)]),aggregate([integer(1),integer(2)]))]: case(id,'IFCBOOLEANCHOOSE',[logical(str(flag).lower()),left,right],out,'concrete IF branch preserves selected generic value')
data=dict(schema='semio.ifc4x3.issued-numeric-boundaries.v1',source=dict(schema='IFC4X3_ADD2',sha256='f67c8762b13a099c28082061e6f16b9ef1284ceec34069792afc702725675860'),scope='selected-concrete-boundaries-not-total-branch-coverage',functions=rows)
(target/'cases.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8')
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','const':data}
(target/'cases.schema.json').write_text(json.dumps(schema,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(cases=len(rows),owners=len(set(row['owner'] for row in rows)),target=str(target))))
