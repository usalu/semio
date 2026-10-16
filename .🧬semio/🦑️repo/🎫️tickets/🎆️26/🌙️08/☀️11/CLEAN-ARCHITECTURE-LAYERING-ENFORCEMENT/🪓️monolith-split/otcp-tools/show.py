import csv,sys
rows=list(csv.DictReader(open(sys.argv[1] if len(sys.argv)>1 else '🗑️generated/otcp-census-units.tsv',encoding='utf8'),delimiter='\t'))
for r in rows:
    if r['unit'] in ('devdep-lib','example'): continue
    print(f"{r['crate'].replace('semio-framework-','')[:34]:34} {r['label'][:12]:12} {r['unit']:11} {r['target'][:34]:34} {r['files']:>4} {r['bytes']:>8} {r['lines']:>6} {r['ratio']:>6} {r['binding']:>6} {'OVER' if r['over']=='True' else ''}")
