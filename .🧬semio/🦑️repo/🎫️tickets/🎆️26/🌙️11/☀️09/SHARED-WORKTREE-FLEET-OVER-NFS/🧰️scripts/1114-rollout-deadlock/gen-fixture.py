import json,sys
OUT=sys.argv[1]
NOW=1_000_000_000_000
D="d"*64; RUN="1"*64; OTHER="e"*64
L={"freshMs":60000,"waitMs":120000,"turnMs":300000,"canaryMs":60000}
def peer(o,code=RUN,age=1000,sync=True,fallback=False,group="g1"):
    return {"originId":o,"groupId":group,"code":code,"wallMs":NOW-age,"sync":sync,"fallback":fallback}
def case(name,expected,self="g1-m02",digest=D,running=RUN,waited=0,authored=False,peers=None,retired=None,snapshots=None,refusals=None,healthy=None):
    return {"name":name,"input":{"digest":digest,"running":running,"originId":self,"groupId":"g1","now":NOW,"waitedMs":waited,"authored":authored,"peers":peers or [],"retired":retired or [],"snapshots":snapshots or [],"refusals":refusals or [],"healthy":healthy or {},"limits":L},"expected":expected}
ok=lambda r:{"ok":True,"role":r}
no=lambda why,origin,s:{"ok":False,"why":why,"origin":origin,"seconds":s}
fleet=[peer("g1-server"),peer("g1-m01"),peer("g1-m02"),peer("g1-m03")]
ref=lambda o,w=1,d=D:{"digest":d,"originId":o,"reason":"self-check failed","wallMs":w}
cases=[
 case("the code that runs is admitted",ok("running"),digest=RUN),
 case("a refusal blocks the digest even when it is authored here",no("refused","g1-m07",0),authored=True,refusals=[ref("g1-m07")]),
 case("a refusal blocks the digest even with a store snapshot and a healthy canary",no("refused","g1-m07",0),snapshots=[D],peers=[peer("g1-server",D)],healthy={"g1-server":600000},refusals=[ref("g1-m07")]),
 case("a refusal of another digest does not matter",no("mixed","g1-server",120),peers=fleet,refusals=[ref("g1-m07",1,OTHER)]),
 case("the earliest refusal is named, a tie goes to the lower id",no("refused","g1-m01",0),refusals=[ref("g1-m07",50),ref("g1-m03",20),ref("g1-m01",20)]),
 case("a store snapshot admits at once",ok("snapshot"),snapshots=[D],peers=fleet),
 case("a snapshot of another digest does not",no("mixed","g1-server",120),snapshots=[OTHER],peers=fleet),
 case("a canary healthy for the set time admits the follower",ok("follower"),peers=[peer("g1-server",D)],healthy={"g1-server":60000}),
 case("a canary one second short of the set time keeps the follower waiting",no("canary","g1-server",1),peers=[peer("g1-server",D)],healthy={"g1-server":59000}),
 case("a canary seen for the first time makes the follower wait the whole time",no("canary","g1-server",60),peers=[peer("g1-server",D)],healthy={}),
 case("a half second short rounds up to a whole second",no("canary","g1-server",1),peers=[peer("g1-server",D)],healthy={"g1-server":59500}),
 case("the healthiest of several canaries decides",ok("follower"),peers=[peer("g1-m01",D),peer("g1-server",D)],healthy={"g1-m01":70000,"g1-server":1000}),
 case("a canary with a stale attestation is no canary",no("mixed","g1-m01",120),peers=[peer("g1-server",D,age=61000),peer("g1-m01"),peer("g1-m03")],healthy={"g1-server":600000}),
 case("a canary with a fresh attestation 60 s old still counts",ok("follower"),peers=[peer("g1-server",D,age=60000)],healthy={"g1-server":60000}),
 case("a canary that runs a fallback code is no canary",no("mixed","g1-m01",120),peers=[peer("g1-server",D,fallback=True),peer("g1-m01")],healthy={"g1-server":600000}),
 case("a canary with sync off is no canary",no("mixed","g1-m02",120),peers=[peer("g1-server",D,sync=False)],healthy={"g1-server":600000}),
 case("a canary of another group is no canary",no("mixed","g1-m02",120),peers=[peer("g2-server",D,group="g2")],healthy={"g2-server":600000}),
 case("a retired canary is no canary",no("mixed","g1-m02",120),peers=[peer("g1-m09",D)],retired=["g1-m09"],healthy={"g1-m09":600000}),
 case("the own attestation is never a canary",no("mixed","g1-m02",120),peers=[peer("g1-m02",D)],healthy={"g1-m02":600000}),
 case("code written entirely here goes first at once",ok("author"),authored=True,peers=fleet),
 case("a canary running the code outranks authorship",no("canary","g1-server",60),authored=True,peers=[peer("g1-server",D)]),
 case("the deadlock: the closure was written on three origins, the group server waits the set time first",no("mixed","g1-server",120),self="g1-server",peers=fleet),
 case("the group server one second before its time",no("mixed","g1-server",1),self="g1-server",waited=119001,peers=fleet),
 case("the group server at its time goes first",ok("leader"),self="g1-server",waited=120000,peers=fleet),
 case("a worker waits behind the group server and the lower ids",no("turn","g1-m01",600),self="g1-m02",waited=120000,peers=fleet),
 case("a worker at half its turn",no("turn","g1-m01",300),self="g1-m02",waited=420000,peers=fleet),
 case("the third candidate goes after two turns",ok("leader"),self="g1-m02",waited=720000,peers=[peer("g1-server"),peer("g1-m01")]),
 case("the second candidate waits one turn",no("turn","g1-server",300),self="g1-m01",waited=120000,peers=fleet),
 case("a stale server does not lead, the lowest fresh id does",ok("leader"),self="g1-m01",waited=120000,peers=[peer("g1-server",age=61000),peer("g1-m02"),peer("g1-m03")]),
 case("with a stale server the next id waits one turn behind the lowest",no("turn","g1-m01",300),self="g1-m02",waited=120000,peers=[peer("g1-server",age=61000),peer("g1-m01"),peer("g1-m03")]),
 case("without a server the lowest id leads",ok("leader"),self="g1-m01",waited=120000,peers=[peer("g1-m02"),peer("g1-m03")]),
 case("two candidates racing at the same moment: the lower id goes",ok("leader"),self="g1-m01",waited=120000,peers=[peer("g1-server",age=61000),peer("g1-m02")]),
 case("two candidates racing at the same moment: the higher id waits",no("turn","g1-m01",300),self="g1-m02",waited=120000,peers=[peer("g1-server",age=61000),peer("g1-m01")]),
 case("an origin with sync off never leads",no("turn","g1-m02",300),self="g1-m03",waited=120000,peers=[peer("g1-server",sync=False),peer("g1-m01",sync=False),peer("g1-m02")]),
 case("an origin on a fallback code never leads",ok("leader"),self="g1-m02",waited=120000,peers=[peer("g1-server",fallback=True),peer("g1-m01",fallback=True)]),
 case("a retired origin never leads",ok("leader"),self="g1-m02",waited=120000,peers=[peer("g1-server"),peer("g1-m01")],retired=["g1-server","g1-m01"]),
 case("an origin of another group never leads",ok("leader"),self="g1-m02",waited=120000,peers=[peer("g2-server",group="g2"),peer("g1-m01",group="g2")]),
 case("a writer clock 30 s ahead still counts as fresh",no("turn","g1-server",300),self="g1-m01",waited=120000,peers=[peer("g1-server",age=-30000)]),
 case("a writer clock 61 s ahead is stale",ok("leader"),self="g1-m01",waited=120000,peers=[peer("g1-server",age=-61000)]),
 case("ordering is bytewise: m10 comes after m02",no("turn","g1-m02",300),self="g1-m10",waited=120000,peers=[peer("g1-m02")]),
 case("a lone origin goes after the wait",ok("leader"),self="g1-m02",waited=120000),
 case("a lone origin before the wait",no("mixed","g1-m02",120),self="g1-m02",waited=0),
]
def step(now,peers,expected): return {"now":now,"peers":peers,"expected":expected}
def tp(o,code,at,fallback=False,sync=True): return {"originId":o,"groupId":"g1","code":code,"wallMs":at,"sync":sync,"fallback":fallback}
T=NOW
trackers=[
 {"name":"the time starts at the first reading and grows with the readings","digest":D,"originId":"g1-m02","freshMs":60000,"steps":[
  step(T,[tp("g1-server",D,T)],{"g1-server":0}),
  step(T+5000,[tp("g1-server",D,T+5000)],{"g1-server":5000}),
  step(T+65000,[tp("g1-server",D,T+65000)],{"g1-server":65000})]},
 {"name":"a gap longer than the freshness window starts the time again","digest":D,"originId":"g1-m02","freshMs":60000,"steps":[
  step(T,[tp("g1-server",D,T)],{"g1-server":0}),
  step(T+61000,[tp("g1-server",D,T+61000)],{"g1-server":0})]},
 {"name":"a gap of exactly the freshness window keeps the time","digest":D,"originId":"g1-m02","freshMs":60000,"steps":[
  step(T,[tp("g1-server",D,T)],{"g1-server":0}),
  step(T+60000,[tp("g1-server",D,T+60000)],{"g1-server":60000})]},
 {"name":"an unhealthy reading forgets the origin and a healthy one starts again","digest":D,"originId":"g1-m02","freshMs":60000,"steps":[
  step(T,[tp("g1-server",D,T)],{"g1-server":0}),
  step(T+10000,[tp("g1-server",D,T+10000,fallback=True)],{}),
  step(T+20000,[tp("g1-server",D,T+20000)],{"g1-server":0}),
  step(T+30000,[tp("g1-server",D,T+30000)],{"g1-server":10000})]},
 {"name":"a stale attestation is not healthy","digest":D,"originId":"g1-m02","freshMs":60000,"steps":[
  step(T,[tp("g1-server",D,T-61000)],{}),
  step(T+1000,[tp("g1-server",D,T+1000)],{"g1-server":0})]},
 {"name":"other codes, sync off, the own attestation and a missing origin are not healthy","digest":D,"originId":"g1-m02","freshMs":60000,"steps":[
  step(T,[tp("g1-server",OTHER,T),tp("g1-m01",D,T,sync=False),tp("g1-m02",D,T),tp("g1-m03",D,T)],{"g1-m03":0}),
  step(T+1000,[tp("g1-server",D,T+1000)],{"g1-server":0})]},
 {"name":"two origins have their own times","digest":D,"originId":"g1-m02","freshMs":60000,"steps":[
  step(T,[tp("g1-server",D,T)],{"g1-server":0}),
  step(T+10000,[tp("g1-server",D,T+10000),tp("g1-m01",D,T+10000)],{"g1-server":10000,"g1-m01":0}),
  step(T+20000,[tp("g1-server",D,T+20000),tp("g1-m01",D,T+20000)],{"g1-server":20000,"g1-m01":10000})]},
]
json.dump({"version":1,"decisions":cases,"trackers":trackers},open(OUT,"w",encoding="utf8"),ensure_ascii=False,indent=1)
print(len(cases),"decisions",len(trackers),"trackers")
