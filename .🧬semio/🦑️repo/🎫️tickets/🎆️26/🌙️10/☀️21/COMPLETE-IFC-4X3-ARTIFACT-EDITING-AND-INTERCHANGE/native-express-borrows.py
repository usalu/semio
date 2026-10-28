from pathlib import Path
base=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation')
for file,functions in [('evaluator.rs',['expression','call_inner']),('population.rs',['type_names','convert','initialize','attribute','type_rules','run_population'])]:
 p=base/file;s=p.read_text(encoding='utf-8')
 for name in functions:
  marker='fn '+name+'(';start=s.index(marker);body=s.index('{',start);level=1;i=body+1
  while level:
   if s[i]=='{':level+=1
   elif s[i]=='}':level-=1
   i+=1
  chunk=s[body+1:i-1]
  if 'self.catalog' in chunk or 'self.program' in chunk:
   declarations='\n'
   if 'self.catalog' in chunk:chunk=chunk.replace('self.catalog','catalog');declarations+='let catalog=self.catalog;\n'
   if 'self.program' in chunk:chunk=chunk.replace('self.program','program');declarations+='let program=self.program;\n'
   s=s[:body+1]+declarations+chunk+s[i-1:]
 p.write_text(s,encoding='utf-8',newline='\n')