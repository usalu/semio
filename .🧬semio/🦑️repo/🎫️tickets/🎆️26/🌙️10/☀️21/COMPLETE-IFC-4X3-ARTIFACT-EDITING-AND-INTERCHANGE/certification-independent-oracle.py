"""🔬 Observe unchanged official certification inputs behind an independent oracle interface."""
import ctypes, hashlib, json, re, sys, time
from pathlib import Path
root=Path(__file__).resolve().parent
runtime=str(root/'🗑️generated/oracle')
if sys.platform=='win32':
    buffer=ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(runtime,buffer,len(buffer))
    runtime=buffer.value or runtime
sys.path.insert(0,runtime)
class CertificationOracle:
    def observe(self,path): raise NotImplementedError
class IfcOpenShellOracle(CertificationOracle):
    def __init__(self):
        import ifcopenshell,ifcopenshell.validate
        self.library=ifcopenshell
        self.validation=ifcopenshell.validate
    def observe(self,path):
        start=time.perf_counter();model=self.library.open(str(path));parsed=time.perf_counter()
        logger=self.validation.json_logger();self.validation.validate(model,logger,express_rules=True)
        counts={}
        for entity in model: counts[entity.is_a()]=counts.get(entity.is_a(),0)+1
        return dict(version=self.library.version,headerSchema=list(model.header.file_schema.schema_identifiers),schemaIdentifier=getattr(model,'schema_identifier',None),descriptions=list(model.header.file_description.description),entities=sum(counts.values()),types=counts,parseSeconds=parsed-start,fullExpressSeconds=time.perf_counter()-parsed,diagnostics=logger.statements)
inputs=json.loads((root/'🗑️generated/clipping-official-input-observations.json').read_text(encoding='utf-8-sig'))
output=root/'🗑️generated/certification-independent-observations.json'
rows=[];oracle=IfcOpenShellOracle()
for item in inputs:
    path=Path(item['local']);raw=path.read_bytes();text=raw.decode('utf-8-sig')
    row=dict(path=item['path'],url=item['url'],commit=item['commit'],sha256=hashlib.sha256(raw).hexdigest(),bytes=len(raw),physicalEntityLines=len(re.findall(r'^\s*#\d+\s*=',text,re.M)),features={name:len(re.findall(pattern,text,re.I)) for name,pattern in {'complexRecords':r'=\s*\(\s*IFC','binaryLiterals':r'"[0-3][0-9A-F]*"','typedValues':r'IFC[A-Z0-9_]+\s*\('}.items()})
    try: row['oracle']=oracle.observe(path)
    except Exception as error: row['error']=dict(type=type(error).__name__,message=str(error))
    rows.append(row);output.write_text(json.dumps(dict(nativeExecution=False,rows=rows),ensure_ascii=False,indent=2,default=str)+'\n',encoding='utf-8')
    print('[DEBUG] '+json.dumps(dict(file=path.name,bytes=len(raw),entities=row.get('oracle',{}).get('entities'),diagnostics=len(row.get('oracle',{}).get('diagnostics',[])),error=row.get('error')),ensure_ascii=False),flush=True)
