from pathlib import Path
T=Path(__file__).parent
G=T/'🗑️generated'
for suffix in ['json','txt']:
 p=G/f'type-value-select-oracle.{suffix}'
 (G/f'type-value-select-oracle19.{suffix}').write_bytes(p.read_bytes())
