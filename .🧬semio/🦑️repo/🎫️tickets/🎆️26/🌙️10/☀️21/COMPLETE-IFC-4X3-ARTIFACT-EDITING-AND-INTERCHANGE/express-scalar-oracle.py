"""🧮 Compile each shared scalar vector through independent IfcOpenShell EXPRESS code generation."""
import importlib.util,json,os,pathlib,subprocess,sys
module=pathlib.Path(sys.argv[1]).resolve();sys.path.insert(0,str(module))
cases=json.loads(pathlib.Path(sys.argv[2]).read_text(encoding='utf-8-sig'))
output=pathlib.Path(sys.argv[3]).resolve();rows=[];env={**os.environ,'PYTHONPATH':str(module)}
for index,row in enumerate(cases):
 source=output/f'scalar-{index}.exp';compiled=output/f'scalar-{index}.py'
 expression=row['expression'];prefix=[]
 if expression=='[1,2][2]':prefix=['LOCAL L:LIST[2:2] OF INTEGER := [1,2]; END_LOCAL;'];expression='L[2]'
 if any(ord(character)>127 for character in expression):rows.append({'expression':expression,'oracleError':'IfcOpenShell EXPRESS source parser rejects this Unicode literal'});continue
 result={'integer':'INTEGER','logical':'LOGICAL','string':'STRING','real':'REAL'}[row['kind']]
 source.write_text('\n'.join(['SCHEMA SemanticVectors;',f'FUNCTION Vector(Unused:INTEGER):{result};',*prefix,f'RETURN({expression});','END_FUNCTION;','END_SCHEMA;']),encoding='utf-8')
 completed=subprocess.run([sys.executable,'-m','ifcopenshell.express.rule_compiler',str(source),str(compiled)],env=env,capture_output=True,text=True,timeout=30)
 if completed.returncode:rows.append({'expression':row['expression'],'oracleError':completed.stderr.splitlines()[-1]});continue
 try:
  spec=importlib.util.spec_from_file_location('independent_semantic_vector',compiled);scope=importlib.util.module_from_spec(spec);spec.loader.exec_module(scope)
  result=getattr(scope,'Vector')(0)
  actual=str(result).lower() if row['kind']=='logical' else str(result)
  rows.append({'expression':row['expression'],'expected':row['value'],'actual':actual,'matches':actual==row['value']})
 except Exception as error:rows.append({'expression':row['expression'],'oracleError':str(error)})
(output/'express-scalar-oracle.json').write_text(json.dumps(rows,indent=2,ensure_ascii=False),encoding='utf-8')
print(json.dumps({'total':len(rows),'matches':sum(row.get('matches',False) for row in rows),'differences':[row for row in rows if not row.get('matches',False)]},ensure_ascii=True))

