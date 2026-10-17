import sys,re
# usage: postfix.py adapters <file>            -> drop blank lines directly before `// #endregion 🔌️Adapters`
#        postfix.py gap <barrel> <needle>      -> blank line between consecutive export lists whose spec contains needle
#        postfix.py doc <file> <new first docstring paragraph file>
cmd=sys.argv[1]; f=sys.argv[2]
t=open(f,encoding="utf8").read()
if cmd=="adapters":
    t2=re.sub(r'\n\n(\s*// #endregion 🔌️Adapters)',r'\n\1',t)
    print("changed" if t2!=t else "same")
elif cmd=="gap":
    n=sys.argv[3]
    t2=re.sub(r'(\} from "[^"]*'+re.escape(n)+r'[^"]*";)\nexport \{',r'\1\n\nexport {',t)
    print("changed" if t2!=t else "same")
open(f,"w",encoding="utf8").write(t2)
