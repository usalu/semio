from pathlib import Path
T=Path(__file__).parent
G=T/'🗑️generated'
for suffix in ['json','txt']:
 p=G/f'type-value-select-oracle.{suffix}'
 (G/f'type-value-select-oracle-before.{suffix}').write_bytes(p.read_bytes())
p=T/'type-value-select-oracle.py'
s=p.read_text(encoding='utf-8-sig').replace("if name=='IFCLOGICAL'and value=='UNKNOWN'", "if name in {'IFCBOOLEAN','IFCLOGICAL'}and value=='UNKNOWN'")
p.write_text(s,encoding='utf-8',newline='\n')
