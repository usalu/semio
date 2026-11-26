import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from f72lib import *
assert have()
s=open(TEST,encoding="utf8").read()
def sub(a,b,count=1):
    global s
    n=s.count(a)
    if n!=count:print("COUNT",n,"expected",count,a[:70]);raise SystemExit(1)
    s=s.replace(a,b)
sub('import{privateRepositoryWorkspaceV1,testNxToolkitDirectoryV1}from','import{privateRepositoryWorkspaceV1,testCheckoutRootV1,testNxToolkitDirectoryV1}from')
sub('owner=join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts"),text=','owner=join(testCheckoutRootV1(),"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts"),text=')
sub('const workspace=process.cwd();await invoke(original=>withRepositoryRustUnitCodeV1','const workspace=repository.root;await invoke(original=>withRepositoryRustUnitCodeV1')
write(TEST,s)
