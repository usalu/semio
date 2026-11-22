"""🎛️ Check semantic contracts, representative print bands, and the staged repair on each component."""
import json,shutil,subprocess,sys
from pathlib import Path
T=Path(__file__).resolve().parent;G=T/'🗑️generated';bun=shutil.which('bun')
for n in sys.argv[1:]:
    path=G/('building-'+n+'.json');asset=json.loads(path.read_bytes())['asset']
    def execute(script,extra,name):
        with (G/('component-'+name+'-'+n+'.log')).open('w') as out:
            return subprocess.run([bun,'--smol',T/script,path,*extra],stdout=out,stderr=out).returncode
    execute('digital-building-semantic.ts',[],'sem')
    cases=[f'{k}/{j}' for k,v in asset['representations'].items() if v['status']=='authored' for j in {int(k),v['validRange']['minDenominator'],v['validRange']['maxDenominator']}]
    execute('digital-paper-candidate.ts',cases,'print')
    print('[DEBUG]',n,'drafted',flush=True)
    if n in ('glass-balustrade','sliding-door','solar-thermal-collector'):
        k='200' if n=='glass-balustrade' else '500';execute('digital-scale-proof.ts',[k,'volumes-silhouette'],'silhouette')
    if n in ('solid-brick-wall-1960','glass-balustrade','sliding-door','solar-thermal-collector'):
        if (G/('component-sem-'+n+'.log')).read_text().strip() or '"suitable":false' in (G/('component-print-'+n+'.log')).read_text():continue
        if n!='solid-brick-wall-1960' and '"issues":[]' not in (G/('component-silhouette-'+n+'.log')).read_text():continue
        with (G/('component-pass-'+n+'.log')).open('w') as out:status=subprocess.run([sys.executable,T/'digital-building-readback.py',n],stdout=out,stderr=out).returncode
        print('[DEBUG]',n,status,flush=True)
