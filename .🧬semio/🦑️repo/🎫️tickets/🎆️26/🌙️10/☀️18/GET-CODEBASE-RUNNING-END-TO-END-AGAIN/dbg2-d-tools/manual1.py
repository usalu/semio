import sys
R="/Users/ueli/Documents/semio"
C=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
scope=[l for l in open(C+"/🗑️generated/dbg2-d/scope.txt",encoding="utf8").read().split("\n") if l]
def F(*keys):
    m=[f for f in scope if all(k in f for k in keys)]
    assert len(m)==1,(keys,len(m))
    return R+"/"+m[0]
def rep(path,old,new,count=1):
    t=open(path,encoding="utf8").read()
    n=t.count(old)
    if n==0:
        print("SKIP",path.split("/")[-3:],old[:60]);return
    if n!=count:
        print("FAIL",n,"!=",count,path.split("/")[-3:],old[:80]);sys.exit(1)
    open(path,"w",encoding="utf8").write(t.replace(old,new))
    print("ok",path.split("/")[-3:],len(old))
J=lambda s: s
# kernel runner fixture
f=F("kernel-test-runner")
rep(f,' eprintln!(\\"[DEBUG] kernel driver actual owned library selection\\");','')
rep(f,' eprintln!(\\"[DEBUG] kernel driver actual integration selection\\");','')
# ordered
f=F("🗂️ordered","source-contract")
rep(f,'\nconsole.log("[DEBUG] ordered map Source corpus retains byte-order JSON oracle through checked existing package adapter");\n','')
# neural
f=F("🧠️neural","retirement","source-contract")
rep(f,'console.log("[DEBUG] actual Neural retirement independent UTF8/stable JSON, grant accounting, cache ownership and evaluation oracle assertions passed");\n\n','')
# job retirement context
f=F("🧵️job","🎟️context","🧪️tests")
rep(f,'console.log("[DEBUG] original close independent currency relation verified");','')
# pack zlib
f=F("🎒️pack","🛫️encode/🫳️borrowed/🎒️document/⏳️cursor","🧪️tests")
rep(f,"console.log('[DEBUG] system zlib confirmed three retained codec1 segments');","")
rep(f,'eprint!("{}",String::from_utf8_lossy(&oracle.stdout));','')
# dsl occurrences
f=F("🗣️dsl","🔤️occurrences")
rep(f,"console.log('[DEBUG] independent SQLite retains duplicate keys and nine exact IEEE words');","")
rep(f,'eprint!("{}",String::from_utf8_lossy(&output.stdout));','')
# fresh component
f=F("🆕️fresh-component")
rep(f,"process.stdout.write('[DEBUG] unauthorized owner executed')","process.stdout.write('unauthorized owner executed')")
# embedded browser
f=F("🪆️embedded-browser")
rep(f,"log(`[DEBUG] embedded acceptance failed ","log(`embedded acceptance failed ")
# ownership
f=F("📏️ownership")
rep(f,'console.log("[DEBUG] independent Node product-removal loader','console.log("[TRACE] independent Node product-removal loader')
rep(f,'node.stdout.includes("[DEBUG] independent Node product-removal loader")','node.stdout.includes("[TRACE] independent Node product-removal loader")')
rep(f,'    console.log(node.stdout.split("\\n").filter(line => line.startsWith("[DEBUG]")).join("\\n"));\n','')
# inventory
f=F("📇️inventory")
rep(f,"[DEBUG] Independent generated inventory","[TRACE] Independent generated inventory",3)
# os runtime
f=F("📥️runtime","🧪️tests")
rep(f,'println!("[DEBUG] Original renderer hosts and activation projection agree with alternate catalog");','')
rep(f,'println!("[DEBUG] Specific actual native publication plugins={} playgrounds={}",catalog.plugins.len(),catalog.playgrounds.len());','')
# docstring
f=F("extension-resources-topic-and-command")
rep(f,"/// [DEBUG] 🧪️ Names","/// 🧪️ Names")
# retirement demand
f=F("♻️retirement","🎟️demand","🧪️tests")
rep(f,'use std::io::Write;writeln!(std::io::stderr(),"[DEBUG] Original retirement family depth={depth} body={body} leafVisits={visits} foreignBodyHints={foreign} quoteDepth={} quoteHeap=0",d.depth).unwrap();','')
rep(f,'let(d,visits,foreign)=quote(&owner,body);','let(_,visits,foreign)=quote(&owner,body);')
# durable group
f=F("durable-group","map-assembly-and-decision-laws")
rep(f,'    std::panic::set_hook(Box::new(|information|{use std::io::Write as _;let _=writeln!(std::io::stderr().lock(),"[DEBUG] durable binding panic={information}");}));\n','')
rep(f,'            {use std::io::Write as _;writeln!(std::io::stderr().lock(),"[DEBUG] durable binding case={} begins",row["id"]).unwrap();}\n','')
rep(f,'                if turn%10000==0{use std::io::Write as _;writeln!(std::io::stderr().lock(),"[DEBUG] durable binding case={} phase={} turn={}",row["id"],step,turn).unwrap();}\n','')
rep(f,'for turn in 0..4_000_000 {\n                if prepared.binding_step!=step{break;}','for _ in 0..4_000_000 {\n                if prepared.binding_step!=step{break;}')
rep(f,'None=>{use std::io::Write as _;let frontier=match step{0=>prepared.parent.member_encoding.frontier(),1=>prepared.drawing.member_encoding.frontier(),_=>prepared.value.member_encoding.frontier()};writeln!(std::io::stderr().lock(),"[DEBUG] durable blocked phase={step} turn={turn} frontier={frontier:?} heap={heap:?}").unwrap();\n','None=>{\n')
rep(f,'let mut closed=false;let mut born=0;let mut released=0;\nfor close_turn in 0..4_000_000{','let mut closed=false;\nfor _ in 0..4_000_000{')
rep(f,'born+=heap.requested_bytes;released+=heap.released_bytes;if done{closed=true;writeln!(std::io::stderr().lock(),"[DEBUG] blocked durable original custody cancelled turns={close_turn} born={born} released={released} six-current-issuer-slots-empty").unwrap();break;}}','if done{closed=true;break;}}')
