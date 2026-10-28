"""🎬 Independent compilation of each neutral imperative fixture, retaining oracle failures."""
import importlib.util,json,os,pathlib,subprocess,sys
module=pathlib.Path(sys.argv[1]).resolve();sys.path.insert(0,str(module));cases=json.loads(pathlib.Path(sys.argv[2]).read_text(encoding='utf-8-sig'));output=pathlib.Path(sys.argv[3]).resolve();rows=[]
for index,row in enumerate(cases):
 source=output/f'statement-{index}.exp';compiled=output/f'statement-{index}.py'
 source.write_text('\n'.join(['SCHEMA StatementVectors;',f"FUNCTION {row['name']}{row['source']} END_FUNCTION;",'END_SCHEMA;']),encoding='utf-8')
 completed=subprocess.run([sys.executable,'-m','ifcopenshell.express.rule_compiler',str(source),str(compiled)],env={**os.environ,'PYTHONPATH':str(module)},capture_output=True,text=True,timeout=30)
 if completed.returncode:rows.append({'name':row['name'],'oracleError':completed.stderr.splitlines()[-1]});continue
 try:
  spec=importlib.util.spec_from_file_location('statement_vectors',compiled);scope=importlib.util.module_from_spec(spec);spec.loader.exec_module(scope)
  actual=str(getattr(scope,row['name'])(*[int(argument) for argument in row['arguments']]))
  rows.append({'name':row['name'],'actual':actual,'expected':row['expected'],'matches':actual==row['expected']})
 except Exception as error:rows.append({'name':row['name'],'oracleError':str(error)})
(output/'express-statement-oracle.json').write_text(json.dumps(rows,indent=2),encoding='utf-8');print(json.dumps(rows))
