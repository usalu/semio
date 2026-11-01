import re,sys
t=open(sys.argv[1]).read().split('\n');seen=set()
for i,l in enumerate(t):
    if l.startswith('error') and 'could not compile' not in l:
        for j in range(i+1,i+3):
            m=re.search(r'--> /Users/ueli/Documents/semio/(.*?):(\d+):(\d+)',t[j])
            if m:
                loc=m.group(1).split('/📦️packages/🦀️rust/')[-1][-70:]+':'+m.group(2)+':'+m.group(3)
                if (l[:100],loc) not in seen:seen.add((l[:100],loc));print(loc,'|',l[:130])
                break
