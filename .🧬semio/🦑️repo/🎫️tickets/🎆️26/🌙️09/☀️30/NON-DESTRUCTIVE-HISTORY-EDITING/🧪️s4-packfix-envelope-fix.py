import sys,re
VAL='semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, '
total=0
for p in sys.argv[1:]:
    s=open(p).read();o=s
    lines=s.split('\n')
    for i,l in enumerate(lines):
        if 'from_envelope_id(' in l or 'unwrap_binary(' in l:
            l=re.sub(r'\.map_err\(\|(\w+)\|\s*((?:[\w$]+::)*PackError)::Schema\(\1\.to_string\(\)\)\)',r'.map_err(|\1| \2::from(\1.into_value_error()))',l)
        l=re.sub(r'return Err\(((?:[\w$]+::)*PackError)::Schema\((format!\((?:[^()]|\([^()]*\))*\))\)\)',lambda m:f'return Err({m.group(1)}::from({VAL}{m.group(2)})))',l)
        l=re.sub(r'return Err\(((?:[\w$]+::)*PackError)::Schema\(("(?:[^"\\]|\\.)*")\.into\(\)\)\)',lambda m:f'return Err({m.group(1)}::from({VAL}{m.group(2)})))',l)
        lines[i]=l
    s='\n'.join(lines)
    n=sum(1 for a,b in zip(o.split('\n'),s.split('\n')) if a!=b)
    total+=n
    open(p,'w').write(s)
    print(n,p)
print('total',total)
