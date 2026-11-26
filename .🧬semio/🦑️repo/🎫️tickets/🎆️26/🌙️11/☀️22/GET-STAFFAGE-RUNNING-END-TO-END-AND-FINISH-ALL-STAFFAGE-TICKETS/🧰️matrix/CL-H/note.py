"""📝️ Appends one line to the closing section of `requests.md` whose heading starts with the given text (read and written in one step, the rest of the file untouched): note.py <start of the heading> <line>."""
import sys
p='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md'
t=open(p,encoding='utf-8').read().split('\n')
i=next(k for k,l in enumerate(t) if l.startswith(sys.argv[1]))
j=next((k for k in range(i+1,len(t)) if t[k].startswith('## ')),len(t))
while j>i+1 and t[j-1].strip()=='': j-=1
t[j:j]=(['']+[sys.argv[2]]) if not t[j-1].startswith('CL-H rewrote') else [sys.argv[2]]
open(p,'w',encoding='utf-8').write('\n'.join(t))
