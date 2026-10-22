import os,sys
mine=['🌊️flow','🌍️gis','🎞️animate','🎪️demonstrator','🏗️fem','🏙️bim','🏛️architect','💠️lowpoly','📋️forms','📏️layout','📐️cad','📜️imperative','🔋️energy','🕸️dag','🖍️draw','🧱️block']
rows=[]
for p in mine:
    base=f'✏️s/🔌️plugins/{p}'
    for dp,dn,fn in os.walk(base):
        dn[:]=[d for d in dn if d not in('node_modules','target')]
        if 'Cargo.lock' in fn and 'Cargo.toml' in fn:
            rel=dp
            rank=2 if '🏭️bridge' in rel else 1 if ('🏅️standards' in rel) else 0
            rows.append((rank,p,rel))
rows.sort()
for r in rows: print(f'{r[1]}\t{r[2]}')
