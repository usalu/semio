import re,sys
pat=re.compile(r'^(?P<f>[^ ].*?)\((?P<l>\d+),(?P<c>\d+)\): error (?P<code>TS\d+): ')
keys=sys.argv[2:]
n=0;keep=False
for line in open(sys.argv[1],encoding='utf8'):
    m=pat.match(line)
    if m:
        keep=any(k in m['f'] for k in keys)
        if keep: n+=1
    elif not line.startswith(' '):
        keep=False
    if keep: sys.stdout.write(line)
sys.stderr.write(f"{n}\n")
