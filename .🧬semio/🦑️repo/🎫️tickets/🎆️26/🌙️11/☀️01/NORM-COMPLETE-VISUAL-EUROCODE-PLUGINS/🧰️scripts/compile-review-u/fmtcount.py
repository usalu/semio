import re,sys
MACROS=("format!","println!","eprintln!","print!","eprint!","write!","writeln!","panic!","assert!","assert_eq!","assert_ne!","debug_assert!","unreachable!","todo!","unimplemented!","format_args!")
def split_args(s,i):
    # s[i] is char after '(' ; return (args list of (start,end), end index of ')')
    depth=0;args=[];start=i;n=len(s);j=i
    while j<n:
        c=s[j]
        if c=='"':
            j+=1
            while j<n and s[j]!='"':
                j+=2 if s[j]=='\\' else 1
        elif c=="'" :
            m=re.match(r"'(\\.[^']*|[^\\'])'",s[j:j+12])
            if m: j+=len(m.group(0))-1
        elif c in '([{': depth+=1
        elif c in ')]}':
            if depth==0:
                args.append((start,j)); return args,j
            depth-=1
        elif c==',' and depth==0:
            args.append((start,j)); start=j+1
        elif c=='/' and s.startswith('//',j):
            k=s.find('\n',j); j=k if k>0 else n
        j+=1
    return args,n
def count_ph(lit):
    lit=lit.replace('{{','').replace('}}','')
    pos=0;named=0
    for m in re.finditer(r'\{([^{}]*)\}',lit):
        inner=m.group(1)
        name=inner.split(':')[0].strip()
        # width/precision args like {:.*} or {:1$}
        extra=len(re.findall(r'\.\*',inner))
        if name=='' : pos+=1+extra
        elif name.isdigit(): pass
        else: named+=1
    return pos
for p in open(sys.argv[1],encoding='utf-8').read().split('\n'):
    p=p.strip()
    if not p: continue
    try: s=open(p,encoding='utf-8').read()
    except Exception: continue
    for mac in MACROS:
        for m in re.finditer(re.escape(mac)+r'\(',s):
            if m.start()>0 and (s[m.start()-1].isalnum() or s[m.start()-1]=='_'): continue
            args,end=split_args(s,m.end())
            if not args: continue
            # find the format literal index
            idx={"assert!":1,"assert_eq!":2,"assert_ne!":2,"debug_assert!":1,"write!":1,"writeln!":1}.get(mac,0)
            if idx>=len(args): continue
            a,b=args[idx]
            lit=s[a:b].strip()
            if not lit.startswith('"'): continue
            # literal may be followed by nothing
            mm=re.fullmatch(r'"((?:[^"\\]|\\.)*)"',lit,re.S)
            if not mm: continue
            n=count_ph(mm.group(1))
            rest=[x for x in args[idx+1:] if s[x[0]:x[1]].strip()!='']
            # named args `name = expr` are not positional
            positional=[x for x in rest if not re.match(r'\s*[a-z_][a-z0-9_]*\s*=\s*[^=]',s[x[0]:x[1]])]
            if n!=len(positional):
                line=s.count('\n',0,m.start())+1
                print(f"{p.split('norm/')[-1].replace('🏅️standards/🔖️1/🪆️subsets/✳️any/','')}:{line}: {mac} placeholders={n} args={len(positional)}")
