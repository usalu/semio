import re,sys
def port(path,sig_start,final_work="1"):
    t=open(path).read()
    a=t.index(sig_start)
    # function end: matching brace
    i=t.index("{",t.index("->",a));depth=0;j=i
    while True:
        c=t[j]
        if c=="{":depth+=1
        elif c=="}":
            depth-=1
            if depth==0:break
        j+=1
    fn=t[a:j+1]
    new=fn
    new=re.sub(r"->\s*Result<RetainedCloneStep,\s*ValueError>","-> RetainedWorkOutcome",new,count=1)
    new=re.sub(r"return Err\((invalid\([^;]*?\))\);",r"return refused(\1);",new)
    new=re.sub(r"return (self\.retirement\.begin_granted\([^;]*?\))\?\.ok_or_else\(\|\| invalid\(([^;]*?)\)\);",r"return owned_step(\1, \2);",new)
    new=re.sub(r"let (\w+) = (self\.close_demands\([^;]*?\))\?;",r"let \1 = match \2 {\n            Ok(demand) => demand,\n            Err(error) => return refused(error),\n        };",new)
    new=re.sub(r"return Ok\((RetainedCloneStep::[A-Za-z]+\([^;]*?\))\);",r"return idle_step(\1);",new)
    # final tail expression Ok(...)
    m=re.search(r"\n(\s*)Ok\((RetainedCloneStep::[A-Za-z]+\(.*\))\)\n(\s*)\}$",new,re.S)
    if m:
        new=new[:m.start()]+"\n"+m.group(1)+"RetainedWorkOutcome { result: Ok("+m.group(2)+"), work: "+final_work+" }\n"+m.group(3)+"}"
    t=t[:a]+new+t[j+1:]
    open(path,"w").write(t)
    return new
if __name__=="__main__":
    print(port(sys.argv[1],sys.argv[2],*(sys.argv[3:4])))
