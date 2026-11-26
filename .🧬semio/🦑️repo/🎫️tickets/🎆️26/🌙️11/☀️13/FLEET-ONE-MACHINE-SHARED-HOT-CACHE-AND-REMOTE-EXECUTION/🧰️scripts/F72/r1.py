import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from f72lib import *
assert have()
s=open(TEST,encoding="utf8").read()
def sub(a,b,count=1):
    global s
    n=s.count(a)
    if n!=count:print("COUNT",n,"expected",count,a[:70]);raise SystemExit(1)
    s=s.replace(a,b)
sub('const repository=await privateRepositoryWorkspaceV1();','const repository=await privateRepositoryWorkspaceV1();\nconst invoke=<T,>(body:(original:ScriptInvocation)=>Promise<T>):Promise<T>=>repository.invoke(body,{maximumElapsedMilliseconds:240000});',1)
n=s.count("repository.invoke(")-1
s=s.replace("await repository.invoke(","await invoke(")
import re
s=re.sub(r'(?<![\w.])repository\.invoke\((?=(async )?original=>)',"invoke(",s)
print("remaining raw",s.count("repository.invoke("))
sub('\\n[profile.prototype]\\ninherits="dev"\\nincremental=false\\ndebug=0\\n[profile.release]\\nincremental=false\\n','\\n[profile.prototype]\\ninherits="dev"\\nincremental=false\\ndebug=0\\ncodegen-units=256\\n[profile.release]\\nincremental=false\\ncodegen-units=1\\n')
sub('[profile.prototype]\\ninherits="dev"\\nincremental=false\\ndebug=0\\n\'','[profile.prototype]\\ninherits="dev"\\nincremental=false\\ndebug=0\\ncodegen-units=256\\n\'')
sub('[profile.prototype]\\ninherits="dev"\\n\'','[profile.prototype]\\ninherits="dev"\\ncodegen-units=256\\n\'',3)
sub('workspaceRoot:workspace,cwd:root,program:"cargo"','workspaceRoot:root,cwd:root,program:"cargo"')
write(TEST,s)
