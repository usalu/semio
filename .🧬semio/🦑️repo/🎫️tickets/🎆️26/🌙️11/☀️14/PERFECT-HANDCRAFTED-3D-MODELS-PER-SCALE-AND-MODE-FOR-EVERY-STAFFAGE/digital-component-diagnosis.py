"""🔎 Separate authored semantic issues from each small candidate print cross-section."""
import subprocess,shutil,sys
from pathlib import Path
T=Path(__file__).resolve().parent;G=T/'🗑️generated';bun=shutil.which('bun')
for n in sys.argv[1:]:
    f=G/('building-'+n+'.json')
    for script,key in [('digital-building-semantic.ts','sem'),('digital-component-paper-profile.ts','paper')]:
        if key=='paper' and n not in ('solar-thermal-collector','glass-balustrade','sliding-door'):continue
        with (G/('component-'+key+'-'+n+'.log')).open('w') as out:subprocess.run([bun,'--smol',T/script,f],stdout=out,stderr=out)
    print('[DEBUG]',n,flush=True)
