#!/usr/bin/env python3
"""Generates the language-agnostic boot-recovery corpus with hand-authored expectations. usage: gen_corpus.py OUT.json"""
import json,sys

B_NEW="1E4A0CBE-3710-481F-9614-F8F1FCBD61D0"
B_OLD="7C3B9A52-0F8E-4D21-A6B4-93E1D5C07F28"
BOOT=1792504809039
DAY=86400000
SHA_NEW="34d16c82f36a91efb584c5516d5e40321f0f4323ec9984a30a9aa4c5378ea364"
SHA_OLD="4a5dde4d9404f89a46eb59dd93173d8994584e80719e370e50f69ab614f56c90"
SHA_OTHER="ead876cab06f5917aa3c914e4d9e27cdbc9c27ac370be5aa6814a73ffa8030b6"
QUEUE="61f8e9d5030f5f3d37d1357916d59da41b0ea60535a6f9f7b52671339c101104.queue"
OWNER_A="Qk3v8XhT2mZpLw0aYcR1dg"
OWNER_B="n1HfT9sLd0QwYb7xKa2VcA"
ALPHABET="0123456789abcdefghijklmnopqrstuvwxyz"

def b36(n,width):
    s=""
    while n:
        s=ALPHABET[n%36]+s;n//=36
    return (s or "0").rjust(width,"0")
def ticket(arrived,pid,owner):return f"{b36(arrived,10)}-{b36(pid,7)}-{owner}"

def pub(scope="p",writerFree=True,directory="ordinary",intent="valid",receipt="absent",unknown=0,intentSha=SHA_OLD,image=("valid",SHA_OLD)):
    return {"family":"publication-residue","scope":scope,"writerFree":writerFree,"directory":directory,"intent":intent,"receipt":receipt,"unknownEntries":unknown,"intentReceiptSha256":intentSha,"image":{"state":image[0],"sha256":image[1]}}
def tick(name,arrived,pid,scope="c",parsed=True,queue=QUEUE):
    return {"family":"lease-ticket","scope":scope,"queue":queue,"name":name,"parsed":parsed,"arrivedAtMs":arrived,"pid":pid}
def claims(total,previous,scope="c"):return {"family":"lease-claims","scope":scope,"claimDatabases":total,"previousBootClaimDatabases":previous}
def srv(files,scope):return {"family":"tool-server","scope":scope,"files":[{"path":p,"modifiedAtMs":m} for p,m in files]}
def swp(intent,writerFree=True,scope="v"):return {"family":"directory-swap","scope":scope,"writerFree":writerFree,"intent":intent}
def rec(final,temporary,pair,scope="s"):
    return {"family":"persistent-record","scope":scope,"final":{"state":"present" if final else "absent","links":final},"temporary":{"state":"present" if temporary else "absent","links":temporary},"pair":pair}
def clm(sidecar,binding,scope="k"):return {"family":"creator-claim","scope":scope,"sidecar":sidecar,"binding":binding}
def rcp(state,binding,scope="r"):return {"family":"image-receipt","scope":scope,"state":state,"binding":binding}
def lki(key,backing="present",location="scratch",idle=True,lease="free",scope="i"):return {"family":"leaked-image","scope":scope,"key":key,"backing":backing,"location":location,"idle":idle,"lease":lease}
K1="bf971d32c5bf4e3917004378398cbab1ca8afb9802abcea93282580a2f3846ba"
K2="d2689249c4645fd6cd0961b3dcb35132ed4722da554a9796b35e14c428e153bf"
K3="6afe4a6d3232e32139387e399b7c2048454ab8e1167fbd1ce3917dd14d71bc39"

PHASE={"lease-ticket":10,"lease-claims":20,"publication-residue":30,"directory-swap":40,"tool-server":50,"leaked-image":60}
def step(i,family,scope,action,target,reason):return {"index":i,"phase":PHASE[family],"family":family,"scope":scope,"action":action,"target":target,"reason":reason}
def keep(family,scope,reason,verdict="keep"):return {"family":family,"scope":scope,"verdict":verdict,"reason":reason}
def obs(facts,boot=B_NEW):return {"version":1,"bootSession":boot,"bootEpochMs":BOOT,"facts":facts}
def case(name,facts,steps,retained,source="derived"):
    return {"name":name,"source":source,"observation":obs(facts),"plan":{"version":1,"bootSession":B_NEW,"steps":steps,"retained":retained}}

PI,PR,PD=".publication/intent.json",".publication/receipt.json",".publication"
plans=[]
add=plans.append
add(case("publication absent is settled",[pub(directory="absent",intent="absent",receipt="absent",intentSha=None,image=("valid",SHA_NEW))],[],[keep("publication-residue","p","settled")]))
add(case("publication live writer keeps every residue",[pub(writerFree=False,receipt="valid")],[],[keep("publication-residue","p","live-writer")]))
add(case("crash after mkdir leaves an empty directory",[pub(intent="absent",intentSha=None)],[step(0,"publication-residue","p","remove-directory",PD,"empty-directory")],[]))
add(case("crash inside the intent write leaves a torn intent",[pub(intent="torn",intentSha=None)],[step(0,"publication-residue","p","remove-file",PI,"torn-intent"),step(1,"publication-residue","p","remove-directory",PD,"torn-intent")],[]))
add(case("intent durable and old receipt still current rolls back",[pub()],[step(0,"publication-residue","p","remove-file",PI,"rolled-back"),step(1,"publication-residue","p","remove-directory",PD,"rolled-back")],[]))
add(case("candidate written and old receipt still current rolls back",[pub(receipt="valid")],[step(0,"publication-residue","p","remove-file",PR,"rolled-back"),step(1,"publication-residue","p","remove-file",PI,"rolled-back"),step(2,"publication-residue","p","remove-directory",PD,"rolled-back")],[]))
add(case("torn candidate and old receipt still current rolls back",[pub(receipt="torn")],[step(0,"publication-residue","p","remove-file",PR,"rolled-back"),step(1,"publication-residue","p","remove-file",PI,"rolled-back"),step(2,"publication-residue","p","remove-directory",PD,"rolled-back")],[]))
add(case("rename done before the finish step only cleans up",[pub(image=("valid",SHA_NEW))],[step(0,"publication-residue","p","remove-file",PI,"completed-before-crash"),step(1,"publication-residue","p","remove-directory",PD,"completed-before-crash")],[]))
add(case("candidate present while the receipt changed is ambiguous",[pub(receipt="valid",image=("valid",SHA_NEW))],[],[keep("publication-residue","p","ambiguous-candidate","refuse")]))
add(case("missing receipt with a valid intent is refused",[pub(image=("absent",None))],[],[keep("publication-residue","p","receipt-unavailable","refuse")]))
add(case("invalid receipt with a valid intent is refused",[pub(image=("invalid",None))],[],[keep("publication-residue","p","receipt-unavailable","refuse")]))
add(case("unknown entry in the publication directory is retained",[pub(unknown=1)],[],[keep("publication-residue","p","unknown-shape","refuse")]))
add(case("unsafe publication directory is retained",[pub(directory="unsafe",intent="absent",intentSha=None)],[],[keep("publication-residue","p","unknown-shape","refuse")]))
add(case("candidate without intent is retained",[pub(intent="absent",receipt="valid",intentSha=None)],[],[keep("publication-residue","p","candidate-without-intent","refuse")]))
add(case("torn intent next to a candidate is retained",[pub(intent="torn",receipt="valid",intentSha=None)],[],[keep("publication-residue","p","torn-intent-with-candidate","refuse")]))
add(case("intent of unknown shape is retained",[pub(intent="unsafe",intentSha=None)],[],[keep("publication-residue","p","unknown-shape","refuse")]))

old_a=ticket(BOOT-3*DAY,8903,OWNER_A)
old_b=ticket(BOOT-3*DAY+977,27900,OWNER_B)
cur=ticket(BOOT+600000,5310,OWNER_A)
edge=ticket(BOOT-5000,4410,OWNER_A)
near=ticket(BOOT-4999,4411,OWNER_B)
q=lambda n:QUEUE+"/"+n
add(case("previous boot ticket is retired with its claims",[tick(old_a,BOOT-3*DAY,8903),claims(2,2)],[step(0,"lease-ticket","c","retire-ticket",q(old_a),"previous-boot"),step(1,"lease-claims","c","retire-settled-claims","claims","previous-boot")],[],"captured"))
add(case("retired tickets alone schedule one claim retirement",[tick(old_a,BOOT-3*DAY,8903)],[step(0,"lease-ticket","c","retire-ticket",q(old_a),"previous-boot"),step(1,"lease-claims","c","retire-settled-claims","claims","retired-tickets")],[]))
add(case("current boot ticket is kept",[tick(cur,BOOT+600000,5310),claims(1,0)],[],[keep("lease-ticket","c","current-boot"),keep("lease-claims","c","settled")]))
add(case("ticket one millisecond inside the tolerance is kept",[tick(near,BOOT-4999,4411)],[],[keep("lease-ticket","c","current-boot")]))
add(case("ticket exactly at the tolerance is retired",[tick(edge,BOOT-5000,4410)],[step(0,"lease-ticket","c","retire-ticket",q(edge),"previous-boot"),step(1,"lease-claims","c","retire-settled-claims","claims","retired-tickets")],[]))
add(case("unknown ticket name is retained",[tick("README",None,None,parsed=False)],[],[keep("lease-ticket","c","unknown-name","refuse")]))
add(case("two dead tickets keep their observed order",[tick(old_b,BOOT-3*DAY+977,27900),tick(old_a,BOOT-3*DAY,8903),claims(3,3)],[step(0,"lease-ticket","c","retire-ticket",q(old_b),"previous-boot"),step(1,"lease-ticket","c","retire-ticket",q(old_a),"previous-boot"),step(2,"lease-claims","c","retire-settled-claims","claims","previous-boot")],[]))
add(case("old claim databases without tickets are retired",[claims(2,2)],[step(0,"lease-claims","c","retire-settled-claims","claims","previous-boot")],[],"captured"))
add(case("claim databases of this boot are kept",[claims(2,0)],[],[keep("lease-claims","c","settled")]))

H=3600000
add(case("bazel server files of a dead boot are removed",[srv([("server/server.pid.txt",BOOT-H),("server/request_cookie",BOOT-H)],"bazel:234823c9f31c8317")],[step(0,"tool-server","bazel:234823c9f31c8317","remove-file","server/server.pid.txt","previous-boot"),step(1,"tool-server","bazel:234823c9f31c8317","remove-file","server/request_cookie","previous-boot")],[]))
add(case("bazel server files of this boot are kept",[srv([("server/server.pid.txt",BOOT+H)],"bazel:234823c9f31c8317")],[],[keep("tool-server","bazel:234823c9f31c8317","current-boot")]))
add(case("only the stale part of a mixed server is removed",[srv([("server/server.pid.txt",BOOT-H),("server/request_cookie",BOOT+H)],"bazel:234823c9f31c8317")],[step(0,"tool-server","bazel:234823c9f31c8317","remove-file","server/server.pid.txt","previous-boot")],[]))
add(case("absent server files are settled",[srv([],"nx:daemon")],[],[keep("tool-server","nx:daemon","settled")]))
add(case("nx daemon files of a dead boot are removed",[srv([("d/server-process.json",BOOT-2*DAY),("d/d.sock",BOOT-2*DAY)],"nx:daemon")],[step(0,"tool-server","nx:daemon","remove-file","d/server-process.json","previous-boot"),step(1,"tool-server","nx:daemon","remove-file","d/d.sock","previous-boot")],[]))

add(case("pending directory swap is recovered by its owner",[swp("present")],[step(0,"directory-swap","v","recover-directory-swap",".","pending-swap")],[]))
add(case("swap under a live writer is kept",[swp("present",writerFree=False)],[],[keep("directory-swap","v","live-writer")]))
add(case("no swap intent is settled",[swp("absent")],[],[keep("directory-swap","v","settled")]))

add(case("persistent record absent is settled",[rec(0,0,"not-applicable")],[],[keep("persistent-record","s","settled")]))
add(case("persistent record prefix is resumed by its owner",[rec(0,1,"not-applicable")],[],[keep("persistent-record","s","owner-resumes-prefix")]))
add(case("persistent record published is kept",[rec(1,0,"not-applicable")],[],[keep("persistent-record","s","published")]))
add(case("linked persistent record pair is resumed by its owner",[rec(2,2,"same-inode")],[],[keep("persistent-record","s","owner-resumes-link")]))
add(case("unpaired persistent record transaction is retained",[rec(1,1,"different-inode")],[],[keep("persistent-record","s","unpaired-transaction","refuse")]))
add(case("persistent record with three links is retained",[rec(3,0,"not-applicable")],[],[keep("persistent-record","s","unknown-links","refuse")]))
add(case("temporary persistent record with two links is retained",[rec(0,2,"not-applicable")],[],[keep("persistent-record","s","unknown-links","refuse")]))

add(case("creator claim absent is settled",[clm("absent","not-applicable")],[],[keep("creator-claim","k","settled")]))
add(case("complete creator claim with exact identities is retired by its owner",[clm("complete","exact")],[],[keep("creator-claim","k","owner-retires")]))
add(case("complete creator claim after a restart is retired by its owner",[clm("complete","rebound")],[],[keep("creator-claim","k","owner-retires-rebound")]))
add(case("partial creator claim with changed identities is retained",[clm("partial","mismatch")],[],[keep("creator-claim","k","identity-mismatch","refuse")]))
add(case("image receipt absent is unpublished",[rcp("absent","not-applicable")],[],[keep("image-receipt","r","unpublished")]))
add(case("image receipt of this boot is current",[rcp("valid","exact")],[],[keep("image-receipt","r","current")]))
add(case("image receipt of a dead boot is bound at open",[rcp("valid","rebound")],[],[keep("image-receipt","r","bound-at-open")],"captured"))
add(case("unreadable image receipt is retained",[rcp("unreadable","not-applicable")],[],[keep("image-receipt","r","unreadable-receipt","refuse")]))
add(case("image receipt with a different inode is retained",[rcp("valid","mismatch")],[],[keep("image-receipt","r","identity-mismatch","refuse")]))

add(case("an abandoned scratch image with a free lease is detached",[lki(K1)],[step(0,"leaked-image","i","detach-image",K1,"abandoned")],[]))
add(case("an image whose backing file was removed is detached anywhere",[lki(K1,backing="absent",location="workspace",idle=False,lease="held")],[step(0,"leaked-image","i","detach-image",K1,"backing-removed")],[]))
add(case("a scratch image under a held lease is kept",[lki(K1,lease="held")],[],[keep("leaked-image","i","live")]))
add(case("a scratch image that was used recently is kept",[lki(K1,idle=False,lease="unavailable")],[],[keep("leaked-image","i","live")]))
add(case("a scratch image without a derivable lease is kept",[lki(K1,lease="unavailable")],[],[keep("leaked-image","i","unleased")]))
add(case("an image of the workspace is never swept",[lki(K1,location="workspace")],[],[keep("leaked-image","i","owned")]))
add(case("an image outside every scratch root is never swept",[lki(K1,location="other")],[],[keep("leaked-image","i","owned")]))
add(case("two leaked images keep their observed order",[lki(K2),lki(K1),lki(K3,lease="held")],[step(0,"leaked-image","i","detach-image",K2,"abandoned"),step(1,"leaked-image","i","detach-image",K1,"abandoned")],[keep("leaked-image","i","live")]))
add(case("a whole dead boot is recovered in phase order",
 [srv([("server/server.pid.txt",BOOT-H)],"bazel:234823c9f31c8317"),pub(receipt="valid"),tick(old_a,BOOT-3*DAY,8903),swp("present"),claims(1,1),rcp("valid","rebound"),clm("complete","rebound")],
 [step(0,"lease-ticket","c","retire-ticket",q(old_a),"previous-boot"),step(1,"lease-claims","c","retire-settled-claims","claims","previous-boot"),step(2,"publication-residue","p","remove-file",PR,"rolled-back"),step(3,"publication-residue","p","remove-file",PI,"rolled-back"),step(4,"publication-residue","p","remove-directory",PD,"rolled-back"),step(5,"directory-swap","v","recover-directory-swap",".","pending-swap"),step(6,"tool-server","bazel:234823c9f31c8317","remove-file","server/server.pid.txt","previous-boot")],
 [keep("image-receipt","r","bound-at-open"),keep("creator-claim","k","owner-retires-rebound")]))

def ident(dev,ino,mode=16832):return {"dev":str(dev),"ino":str(ino),"mode":mode}
identities=[]
def idc(name,recorded,current,expected,source="derived"):identities.append({"name":name,"source":source,"recorded":recorded,"current":current,"expected":expected})
P,D,R=424182066,424182090,424182091
idc("identical identities are exact",[ident(16777263,P,16877),ident(16777263,D),ident(16777263,R)],[ident(16777263,P,16877),ident(16777263,D),ident(16777263,R)],"exact","captured")
idc("renumbered devices after a restart rebind",[ident(16777232,P,16877),ident(16777232,D),ident(16777232,R)],[ident(16777229,P,16877),ident(16777229,D),ident(16777229,R)],"rebound")
idc("renumbered mount device after a parent re-attach rebinds",[ident(16777263,P,16877),ident(16777263,D),ident(16777263,R)],[ident(16777264,P,16877),ident(16777264,D),ident(16777264,R)],"rebound")
idc("a single identity with a renumbered device rebinds",[ident(16777232,P,16877)],[ident(16777229,P,16877)],"rebound")
idc("replaced directory inode is a mismatch",[ident(16777263,P,16877),ident(16777263,D),ident(16777263,R)],[ident(16777263,P,16877),ident(16777263,D+1),ident(16777263,R)],"mismatch")
idc("changed mode is a mismatch",[ident(16777263,P,16877),ident(16777263,D)],[ident(16777263,P,16877),ident(16777263,D,16877)],"mismatch")
idc("a directory that left the parent device is a mismatch",[ident(16777232,P,16877),ident(16777232,D),ident(16777232,R)],[ident(16777229,P,16877),ident(16777229,D),ident(16777200,R)],"mismatch")
idc("a directory that joined the parent device is a mismatch",[ident(16777232,P,16877),ident(16777240,D)],[ident(16777229,P,16877),ident(16777229,D)],"mismatch")
idc("different lengths are a mismatch",[ident(1,P,16877),ident(1,D)],[ident(1,P,16877)],"mismatch")
idc("same device numbers with changed inode stay a mismatch",[ident(7,P),ident(7,D)],[ident(7,P),ident(7,R)],"mismatch")
idc("two parent identities on different devices rebind pairwise",[ident(16777232,P,16877),ident(16777250,D)],[ident(16777229,P,16877),ident(16777248,D)],"rebound")

backings=[]
def bk(name,recorded,current,expected,source="derived"):backings.append({"name":name,"source":source,"recorded":recorded,"current":current,"expected":expected})
bk("identical strings are exact",'[16777229,422334443,8588886016,16777248]','[16777229,422334443,8588886016,16777248]',"exact","captured")
bk("a restart renumbers the host and mount devices",'[16777232,422334443,8588886016,16777250]','[16777229,422334443,8588886016,16777248]',"rebound","captured")
bk("a re-attach only renumbers the mount device",'[16777229,422334443,8588886016,16777250]','[16777229,422334443,8588886016,16777248]',"rebound")
bk("a changed image inode is a mismatch",'[16777229,422334444,8588886016,16777248]','[16777229,422334443,8588886016,16777248]',"mismatch")
bk("a changed image size is a mismatch",'[16777229,422334443,8588886017,16777248]','[16777229,422334443,8588886016,16777248]',"mismatch")
bk("the fixed backend formula rebinds across a restart",'["darwin",16777232,422334443,8588886016,"m:16777250:2"]','["darwin",16777229,422334443,8588886016,"m:16777248:2"]',"rebound")
bk("the fixed backend formula keeps its platform",'["linux",1,2,3,"x"]','["darwin",1,2,3,"x"]',"mismatch")
bk("different formulas never bind",'[16777229,422334443,8588886016,16777248]','["darwin",16777229,422334443,8588886016,"m"]',"mismatch")
bk("text that is not an identity never binds",'not json','[16777229,422334443,8588886016,16777248]',"mismatch")
bk("an unknown arity never binds",'[1,2,3]','[1,2,3,4]',"mismatch")
bk("negative numbers never bind",'[1,-2,3,4]','[1,-2,3,5]',"mismatch")

epochs=[]
def ep(name,platform,text,ms):epochs.append({"name":name,"platform":platform,"text":text,"expectedMs":ms})
ep("darwin kern.boottime of the incident boot","darwin","{ sec = 1792504809, usec = 39703 } Tue Oct 20 16:00:09 2026\n",1792504809039)
ep("darwin microseconds are an integer count, not a decimal fraction","darwin","{ sec = 1792504809, usec = 5 } Tue Oct 20 16:00:09 2026",1792504809000)
ep("darwin sub-millisecond microseconds truncate to zero","darwin","{ sec = 1792504809, usec = 999 } Tue Oct 20 16:00:09 2026",1792504809000)
ep("darwin 999999 microseconds truncate to 999 milliseconds","darwin","{ sec = 1792504809, usec = 999999 } x",1792504809999)
ep("darwin text without a boot time is rejected","darwin","sysctl: unknown oid",None)
ep("linux btime line","linux","cpu  1 2 3\nbtime 1792504809\nprocesses 42\n",1792504809000)
ep("linux text without btime is rejected","linux","cpu  1 2 3\nprocesses 42\n",None)
ep("windows ISO instant equals the darwin instant","win32","2026-10-20T14:00:09.0397030Z\r\n",1792504809039)
ep("windows instant without fraction","win32","2026-10-20T14:00:09Z",1792504809000)
ep("windows local time zone is rejected","win32","2026-10-20T16:00:09+02:00",None)

def ev(**kw):return json.dumps(kw,separators=(",",":"))
begin=lambda boot,n=1:ev(v=1,type="begin",boot=boot,at=BOOT+1,steps=n,plan=SHA_NEW)
stepev=lambda boot,i=0:ev(v=1,type="step",boot=boot,index=i,action="remove-file",target=".publication/intent.json",outcome="done",detail="")
commit=lambda boot:ev(v=1,type="commit",boot=boot,at=BOOT+2,done=1,skipped=0,retained=0)
abort=lambda boot:ev(v=1,type="abort",boot=boot,at=BOOT+3,index=0,reason="cancelled")
ledgers=[]
def lg(name,text,state,boot,corrupt=False,torn=False):ledgers.append({"name":name,"text":text,"expected":{"state":state,"boot":boot,"corrupt":corrupt,"tornTail":torn}})
lg("empty ledger","","empty",None)
lg("begin only is incomplete",begin(B_NEW)+"\n","incomplete",B_NEW)
lg("begin step commit is committed",begin(B_NEW)+"\n"+stepev(B_NEW)+"\n"+commit(B_NEW)+"\n","committed",B_NEW)
lg("an old commit followed by a new begin is incomplete",begin(B_OLD)+"\n"+commit(B_OLD)+"\n"+begin(B_NEW)+"\n","incomplete",B_NEW)
lg("a new commit after an old one is committed",begin(B_OLD)+"\n"+commit(B_OLD)+"\n"+begin(B_NEW)+"\n"+commit(B_NEW)+"\n","committed",B_NEW)
lg("abort after steps is incomplete",begin(B_NEW)+"\n"+stepev(B_NEW)+"\n"+abort(B_NEW)+"\n","incomplete",B_NEW)
lg("a torn tail after a commit is ignored but reported",begin(B_NEW)+"\n"+commit(B_NEW)+"\n"+'{"v":1,"type":"be',"committed",B_NEW,False,True)
lg("a torn tail after nothing is an empty ledger","{\"v\":1","empty",None,False,True)
lg("a foreign line makes the ledger advisory-corrupt",begin(B_NEW)+"\nwhat is this\n"+commit(B_NEW)+"\n","empty",None,True)
lg("a well formed line of the wrong shape is corrupt",begin(B_NEW)+"\n"+ev(v=1,type="commit",boot=B_NEW)+"\n","empty",None,True)

doc={"version":1,"plans":plans,"identities":identities,"backings":backings,"epochs":epochs,"ledgers":ledgers}
open(sys.argv[1],"w",encoding="utf8").write(json.dumps(doc,indent=2,ensure_ascii=False)+"\n")
print("corpus",len(plans),"plans",len(identities),"identities",len(backings),"backings",len(epochs),"epochs",len(ledgers),"ledgers")
