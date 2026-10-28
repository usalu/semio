"""🔬 Observe real generated helpers and separate SQLite arithmetic behind owned interfaces, retaining raw discrepancies."""
import hashlib
import json
import platform
import sqlite3
from pathlib import Path
from typing import Protocol
import ifcopenshell
import ifcopenshell.express.rules.IFC4X3_ADD2 as rules

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
source=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️numeric/cases.json'
data=json.loads(source.read_text(encoding='utf-8'))
names=['LengthExponent','MassExponent','TimeExponent','ElectricCurrentExponent','ThermodynamicTemperatureExponent','AmountOfSubstanceExponent','LuminousIntensityExponent']
class NumericOracle(Protocol):
    def observe(self,case:dict)->dict: ...
def plain(value):
    if value is None:return dict(kind='unknown')
    if isinstance(value,bool):return dict(kind='logical',value=str(value).lower())
    if isinstance(value,int):return dict(kind='integer',value=str(value))
    if isinstance(value,float):return dict(kind='real',value=str(int(value)) if value.is_integer() else str(value))
    if isinstance(value,str):return dict(kind='string',value=value)
    if isinstance(value,(list,tuple)):return dict(kind='aggregate',aggregate='LIST',lower='1',items=list(map(plain,value)))
    raise TypeError('Unrepresented independent result '+type(value).__name__)
class GeneratedHelperOracle:
    def observe(self,case):
        file=ifcopenshell.file(schema='IFC4X3_ADD2')
        def decode(row):
            kind=row['kind']
            if kind=='integer':return int(row['value'])
            if kind=='real':return float(row['value'])
            if kind in ('string','enum'):return row['value']
            if kind=='logical':return row['value']=='true'
            if kind=='unknown':return None
            if kind=='aggregate':return [decode(item) for item in row['items']]
            if kind=='entity':
                result=file.create_entity(row['type'])
                for name,item in row['fields'].items():setattr(result,name,decode(item))
                return result
            raise TypeError('Unsupported independent argument '+kind)
        result=dict(id=case['id'],owner=case['owner'])
        try:
            fn=getattr(rules,next(name for name in dir(rules) if name.upper()==case['owner']))
            value=fn(*[decode(arg) for arg in case['args']])
            if 'projection' in case:observed=dict(kind='attributes',values={name:plain(getattr(value,name)) for name in case['projection']})
            elif case['owner']=='IFCCORRECTDIMENSIONS' and (value is None or value=='UNKNOWN'):observed=dict(kind='logical',value='unknown')
            else:observed=plain(value)
            result.update(value=observed,rawType=type(value).__name__,rawScalar=value if isinstance(value,(str,int,float,bool)) or value is None else None,status='agree' if observed==case['expected'] else 'divergent-value')
        except Exception as error:result.update(status='unsupported',error=dict(kind=type(error).__name__,message=str(error)))
        return result
class SqliteArithmeticOracle:
    def observe(self,case):
        owner=case['owner'];result=dict(id=case['id'],owner=owner)
        with sqlite3.connect(':memory:') as db:
            if owner=='IFCCONSTRAINTSPARAMBSPLINE':
                degree,up,cp=[int(row['value']) for row in case['args'][:3]]
                multiplicities=[int(row['value']) for row in case['args'][3]['items']]
                if any(abs(n)>2**63-1 for n in [degree,up,cp,*multiplicities,sum(multiplicities),degree+cp+2]):return dict(**result,status='unsupported',reason='outside signed 64-bit SQLite INTEGER arithmetic boundary')
                db.execute('CREATE TABLE knots(i INTEGER,m INTEGER,k REAL)')
                db.executemany('INSERT INTO knots VALUES(?,?,?)',[(i+1,int(m['value']),float(k['value'])) for i,(m,k) in enumerate(zip(case['args'][3]['items'],case['args'][4]['items']))])
                total=db.execute('SELECT SUM(m) FROM knots WHERE i<=?',(up,)).fetchone()[0]
                result['storage']=dict(degree=str(degree),controls=str(cp),multiplicitySum=str(total),sumType=db.execute('SELECT typeof(SUM(m)) FROM knots WHERE i<=?',(up,)).fetchone()[0],exactSigned64=True)
                invalid=db.execute('SELECT COUNT(*) FROM (SELECT i,m,k,LAG(k) OVER(ORDER BY i) previous FROM knots WHERE i<=?) WHERE m<1 OR (i IN (1,?) AND m>?) OR (i>1 AND i<? AND m>?) OR (i>1 AND k<=previous)',(up,up,degree+1,up,degree)).fetchone()[0]
                observed=dict(kind='logical',value=str(degree>=1 and up>=2 and cp>=degree and total==degree+cp+2 and invalid==0).lower())
            elif owner=='IFCCORRECTDIMENSIONS':
                unit=case['args'][0]['value']
                if unit=='USERDEFINED':observed=dict(kind='logical',value='unknown')
                else:
                    role={'LENGTHUNIT':'METRE','AREAUNIT':'SQUARE_METRE','VOLUMEUNIT':'CUBIC_METRE','MASSUNIT':'GRAM','TIMEUNIT':'SECOND','ELECTRICCURRENTUNIT':'AMPERE','THERMODYNAMICTEMPERATUREUNIT':'KELVIN','AMOUNTOFSUBSTANCEUNIT':'MOLE','LUMINOUSINTENSITYUNIT':'CANDELA','PLANEANGLEUNIT':'RADIAN','SOLIDANGLEUNIT':'STERADIAN','ABSORBEDDOSEUNIT':'GRAY','RADIOACTIVITYUNIT':'BECQUEREL','ELECTRICCAPACITANCEUNIT':'FARAD','DOSEEQUIVALENTUNIT':'SIEVERT','ELECTRICCHARGEUNIT':'COULOMB','ELECTRICCONDUCTANCEUNIT':'SIEMENS','ELECTRICVOLTAGEUNIT':'VOLT','ELECTRICRESISTANCEUNIT':'OHM','ENERGYUNIT':'JOULE','FORCEUNIT':'NEWTON','FREQUENCYUNIT':'HERTZ','INDUCTANCEUNIT':'HENRY','ILLUMINANCEUNIT':'LUX','LUMINOUSFLUXUNIT':'LUMEN','MAGNETICFLUXUNIT':'WEBER','MAGNETICFLUXDENSITYUNIT':'TESLA','POWERUNIT':'WATT','PRESSUREUNIT':'PASCAL'}
                    independent=rules.IfcDimensionsForSIUnit(role[unit])
                    db.execute('CREATE TABLE dimensions(i INTEGER,actual INTEGER,expected INTEGER)')
                    db.executemany('INSERT INTO dimensions VALUES(?,?,?)',[(i,int(case['args'][1]['fields'][name]['value']),getattr(independent,name)) for i,name in enumerate(names)])
                    observed=dict(kind='logical',value=str(db.execute('SELECT COUNT(*) FROM dimensions WHERE actual!=expected').fetchone()[0]==0).lower())
            elif owner=='IFCDERIVEDIMENSIONALEXPONENTS':
                db.execute('CREATE TABLE terms(axis INTEGER,dimension INTEGER,exponent INTEGER)')
                db.executemany('INSERT INTO terms VALUES(?,?,?)',[(i,int(item['fields']['Unit']['fields']['Dimensions']['fields'][name]['value']),int(item['fields']['Exponent']['value'])) for item in case['args'][0]['items'] for i,name in enumerate(names)])
                values=db.execute('SELECT axis,SUM(dimension*exponent) FROM terms GROUP BY axis ORDER BY axis').fetchall()
                observed=dict(kind='attributes',values={names[i]:dict(kind='integer',value=str(n)) for i,n in values})
            else:return dict(**result,status='unsupported',reason='outside SQLite arithmetic interface')
        return dict(**result,value=observed,status='agree' if observed==case['expected'] else 'divergent-value')
def main():
    oracle:NumericOracle=GeneratedHelperOracle();sql:NumericOracle=SqliteArithmeticOracle()
    result=dict(python=platform.python_version(),ifcopenshell=ifcopenshell.version,sqlite=sqlite3.sqlite_version,inputSha256=hashlib.sha256(source.read_bytes()).hexdigest(),functions=[oracle.observe(case) for case in data['functions']],arithmetic=[sql.observe(case) for case in data['functions'] if case['owner'] in ('IFCCONSTRAINTSPARAMBSPLINE','IFCCORRECTDIMENSIONS','IFCDERIVEDIMENSIONALEXPONENTS')])
    output=ticket/'🗑️generated/issued-express-numeric-oracle.json';output.write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
    print('[DEBUG] '+json.dumps(dict(output=str(output),counts={status:sum(row['status']==status for row in result['functions']) for status in ['agree','divergent-value','unsupported']},arithmetic={status:sum(row['status']==status for row in result['arithmetic']) for status in ['agree','divergent-value','unsupported']})))
if __name__=='__main__':main()
