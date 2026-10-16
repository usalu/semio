import csv,sys
crate=sys.argv[1]; src=sys.argv[2] if len(sys.argv)>2 else '🗑️generated/otcp-census-overlimit-files.tsv'
rows=[r for r in csv.DictReader(open(src,encoding='utf8'),delimiter='\t') if r['label']=='native' and r['unit']=='lib-test' and r['crate']=='semio-framework-'+crate]
tot=[0,0,0]
for r in rows:
    t=r['notInLibNormal']=='true'
    print(f"{'T' if t else 'n'} {r['bytes']:>7} {r['lines']:>5} {r['kind'][:5]:5} {r['file'].replace('🧰️framework/🛍️products/💻️os/🔨️modules/','M/')}")
    if t: tot[0]+=1; tot[1]+=int(r['bytes']); tot[2]+=int(r['lines'])
print("test-only",tot)
