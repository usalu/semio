from pathlib import Path
import re,subprocess
g=Path(__file__).resolve().parent/'🗑️generated'; groups=(g/'current-2d-fill-template-stacks.log').read_text().split('current-native-stack.so')
adds=[re.findall(r'history-fill-template\(\+(0x[0-9a-f]+)',s) for s in groups[1:]]
unique=list(dict.fromkeys(a for group in adds for a in group))
res=subprocess.run(['addr2line','-f','-C','-e',str(g/'current-2d-hist-relink/history-fill-template'),*unique],capture_output=True,text=True).stdout.splitlines()[::2]
names=dict(zip(unique,res))
for i,group in enumerate(adds):
 print('sample',i+1)
 rows=[names[a] for a in group];print('\n'.join(row.split('>::')[-1][:115] for row in rows[:17]))
