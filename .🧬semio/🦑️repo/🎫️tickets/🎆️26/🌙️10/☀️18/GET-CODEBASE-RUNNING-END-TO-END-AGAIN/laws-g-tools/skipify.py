import re,sys,json
S='/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad'
R='/Users/ueli/Documents/semio/'
dry='--dry' in sys.argv
byfile={}
for l in open(S+'/earlyret.txt',encoding='utf-8'):
    path,no,rest=l.rstrip('\n').split(':',2)
    if len(rest)>60: continue
    byfile.setdefault(path,[]).append(int(no))
guard=re.compile(r'^\s*if\s*\(\s*process\.platform\s*(===|!==)\s*"(\w+)"\s*\)\s*return;\s*$')
header=re.compile(r'^\s*(?:for\(.*\))?\s*test\(.*(?:=>\s*\{|\{)\s*$')
done=[];manual=[]
for path,nos in byfile.items():
    lines=open(R+path,encoding='utf-8').read().split('\n')
    edits=[]
    for no in nos:
        g=lines[no-1]; m=guard.match(g)
        if not m: manual.append((path,no,'not a bare guard')); continue
        j=no-2
        if not header.match(lines[j]): manual.append((path,no,'not first statement: '+lines[j][:60])); continue
        cond='process.platform'+m.group(1)+'"'+m.group(2)+'"'
        # skip when guard returns: skip condition is the guard condition
        h=lines[j]
        if h.count('test(')<1: manual.append((path,no,'no test(')); continue
        new=h.replace('test(','test.skipIf('+cond+')(',1)
        edits.append((j,new,no-1))
    for j,new,gi in sorted(edits,reverse=True):
        lines[j]=new; del lines[gi]
        done.append((path[-60:],gi+1))
    if edits and not dry: open(R+path,'w',encoding='utf-8').write('\n'.join(lines))
print('done',len(done)); [print(' ',d) for d in done]
print('manual',len(manual)); [print(' ',m) for m in manual]
