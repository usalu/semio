import sys,os,re;sys.path.insert(0,os.path.dirname(__file__))
from f72lib import *
assert have()
s=open(TEST,encoding="utf8").read()
def sub(x,y,count=1):
    global s
    n=s.count(x)
    if n!=count:print("COUNT",n,"expected",count,x[:90]);raise SystemExit(1)
    s=s.replace(x,y)
sub('crate="🧰️framework/🔨️modules/🚪️io/🔤️base64",member=crate+"/📦️packages/🦀️rust",root=repository.root,put=','crate="🧰️framework/🔨️modules/🚪️io/🔤️base64",member="🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust",root=repository.root,put=')
sub('const lock=fs.readFileSync(join(checkout,"Cargo.lock"),"utf8")','const lock=fs.readFileSync(join(checkout,"🧰️framework/Cargo.lock"),"utf8")')
sub('put("Cargo.toml",fs.readFileSync(join(checkout,"Cargo.toml"),"utf8")','put("🧰️framework/Cargo.toml",fs.readFileSync(join(checkout,"🧰️framework/Cargo.toml"),"utf8")')
sub('put("Cargo.lock",lock.slice','put("🧰️framework/Cargo.lock",lock.slice')
write(TEST,s)
