import sys,os,re;sys.path.insert(0,os.path.dirname(__file__))
from f72lib import *
assert have()
A=LIB+"/🧪️testing/🧫️private-repository/🧰️adoption/🟦️.ts"
a=open(A,encoding="utf8").read()
if "adoptWorkspaceMarkersForV1" not in a:
    a=a.rstrip("\n")+'''
/** 🧭️ Gives one private repository the paired build and package markers of a package workspace, so a script started inside it resolves the private repository as its own workspace root; an existing marker is left as it is. */
export function adoptWorkspaceMarkersForV1(workspace:string):void{
 for(const[name,text]of[["package.json",'{"name":"private-repository","private":true,"workspaces":[]}\\n'],["📋️project.json","{}\\n"]]as const){try{writeFileSync(join(resolve(workspace),name),text,{flag:"wx"});}catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;}}
}
'''
    write(A,a)
s=open(TEST,encoding="utf8").read()
def sub(x,y,count=1):
    global s
    n=s.count(x)
    if n!=count:print("COUNT",n,"expected",count,x[:90]);raise SystemExit(1)
    s=s.replace(x,y)
sub('import{adoptAuthorizerSourcesForV1}from','import{adoptAuthorizerSourcesForV1,adoptWorkspaceMarkersForV1}from')
sub('await adoptAuthorizerSourcesForV1(repository.root,testCheckoutRootV1(),rustCodePolicy.entry);','await adoptAuthorizerSourcesForV1(repository.root,testCheckoutRootV1(),rustCodePolicy.entry);\nadoptWorkspaceMarkersForV1(repository.root);')
sub("writeFileSync(manifest,'[workspace]\\n[package]","writeFileSync(manifest,'cargo-features=[\"trim-paths\"]\\n[workspace]\\n[package]")
sub("writeFileSync(manifest,'[workspace]\\nmembers=","writeFileSync(manifest,'cargo-features=[\"trim-paths\"]\\n[workspace]\\nmembers=")
sub("writeFileSync(join(root,\"Cargo.toml\"),'[workspace]\\nmembers=[\"base\",\"consumer\",\"sibling\"]\\nresolver=\"2\"\\n[profile.prototype]\\ninherits=\"dev\"\\nincremental=false\\ndebug=0\\ncodegen-units=256\\n')","writeFileSync(join(root,\"Cargo.toml\"),'cargo-features=[\"trim-paths\"]\\n[workspace]\\nmembers=[\"base\",\"consumer\",\"sibling\"]\\nresolver=\"2\"\\n[profile.prototype]\\ninherits=\"dev\"\\nincremental=false\\ndebug=0\\ncodegen-units=256\\n')")
write(TEST,s);print("ok")
