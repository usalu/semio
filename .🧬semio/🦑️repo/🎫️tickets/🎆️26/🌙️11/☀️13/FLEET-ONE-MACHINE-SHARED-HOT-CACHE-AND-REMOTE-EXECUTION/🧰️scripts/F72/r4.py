import sys,os,re;sys.path.insert(0,os.path.dirname(__file__))
from f72lib import *
assert have()
s=open(TEST,encoding="utf8").read()
def sub(a,b,count=1):
    global s
    n=s.count(a)
    if n!=count:print("COUNT",n,"expected",count,a[:90]);raise SystemExit(1)
    s=s.replace(a,b)
sub('const outputRoot=relative(process.cwd(),','const outputRoot=relative(repository.root,')
sub('const root=mkdtempSync(join(artifacts,"sp-"));try{','const root=await repository.scratch("sp-");try{')
sub('const workspace=process.cwd(),directory=mkdtempSync(join(artifacts,"on-")),','const workspace=repository.root,directory=await repository.scratch("on-"),')
n=s.count('join(workspace,".nx/installation');print("nx",n)
s=s.replace('join(workspace,".nx/installation','join(testCheckoutRootV1(),".nx/installation')
# 77: sealed program as owner
old='withUnitDescriptorFixture(original,createRequestedIssuedResourceControlV1(codePhase,300000),async({file,units,outputRoot,control})=>{const workspace=process.cwd(),env=scriptInvocationEnvironment(original,{...process.env,SEMIO_RUST_UNIT_DIGEST:"unbound"}),owner=join(testCheckoutRootV1(),"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts"),text='
new='withUnitDescriptorFixture(original,createRequestedIssuedResourceControlV1(codePhase,300000),async({file,units,outputRoot,control})=>{const workspace=process.cwd(),env=scriptInvocationEnvironment(original,{...process.env,SEMIO_RUST_UNIT_DIGEST:"unbound"}),owner=join(workspace,code.program),text='
sub(old,new)
sub('(_code,codePhase)=>withUnitDescriptorFixture(original,createRequestedIssuedResourceControlV1(codePhase,300000),async({file','(code,codePhase)=>withUnitDescriptorFixture(original,createRequestedIssuedResourceControlV1(codePhase,300000),async({file')
write(TEST,s)
