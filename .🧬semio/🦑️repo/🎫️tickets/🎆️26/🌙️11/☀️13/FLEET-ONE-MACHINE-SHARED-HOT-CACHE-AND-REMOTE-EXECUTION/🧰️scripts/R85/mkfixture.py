#!/usr/bin/env python3
"""R85: authors the language-agnostic fixture of the containment laws by hand (cases, not derived from the code under test)."""
import json,sys
T=1794000000000;GRACE=60000;ROOT="C:\\Users\\w\\semio";CMD="fed0aa7c7c00cedaa8abc3ae7c9a032a8655b5ef9d0d37095feddf0e08b75442 show projects";DIR=ROOT+"\\.cache\\b\\l\\m\\data"
def row(pid,ppid,start,name=None):
    r={"pid":pid,"ppid":ppid,"startMs":start}
    if name:r["name"]=name
    return r
def ent(pid,start,command=CMD,session=None,holds=("native-fixed-image:C:/x",),directory=DIR):
    e={"pid":pid,"startedAtMs":start,"command":command,"directory":directory,"holds":[{"resource":h,"sinceMs":start+5} for h in holds]}
    if session:e["session"]=session
    return e
W=[{"startedMs":T,"endedMs":T+1380000}]
BASE=[row(4,0,T-9e9+0 if False else 1),row(1000,4,T-9000000,"ready.exe"),row(1001,1000,T-8000000,"bun.exe")]
def case(name,entries,table,windows=W,human=None,self=1001,stage=None,orphans=()):
    return {"name":name,"root":ROOT,"entries":entries,"table":BASE+table,"windows":windows,"human":human,"self":self,"stage":stage,"orphans":list(orphans)}
S0=T+600000
orphan=[case("g1-m05: the show projects of a stalled dependencies try, its launcher long gone, is an orphan of the try",[ent(91672,S0)],[row(91672,40000,S0,"bun.exe")],orphans=[91672]),
 case("a live pid with another start time is another process and is never touched",[ent(91672,S0)],[row(91672,40000,S0+90_000_000,"bun.exe")]),
 case("a registry entry whose process is not in the table is nothing to stop",[ent(91672,S0)],[]),
 case("an entry that an agent session registered is never reaped",[ent(91672,S0,session="claude-code:abc")],[row(91672,1,S0,"bun.exe")]),
 case("a repo command of another checkout is never reaped",[ent(91672,S0,directory="D:\\other\\semio\\.cache\\b\\l\\m\\data")],[row(91672,1,S0,"bun.exe")]),
 case("a command that only names a file of this checkout, run by a lease below the checkout cache, is the checkout's own; a hex-less foreign command is not",[ent(91673,S0,command=ROOT+"\\🧰️framework\\worker.ts"),ent(91674,S0,command="D:\\tools\\other.ts show projects")],[row(91673,1,S0,"bun.exe"),row(91674,1,S0,"bun.exe")],orphans=[91673]),
 case("a lease directory outside the checkout cache is not the checkout's, even when the command names the checkout",[ent(91675,S0,command=ROOT+"\\x.ts",directory="C:\\Users\\w\\AppData\\Local\\semio\\leases")],[row(91675,1,S0,"bun.exe")],orphans=[]),
 case("a test sandbox of the checkout is not a fleet command",[ent(91676,S0,directory=ROOT+"\\.cache\\test-sandbox\\rAbC\\.cache\\b\\c")],[row(91676,1,S0,"bun.exe")],orphans=[]),
 case("a process that started before every stopped try is not an orphan of one",[ent(91672,T-3_600_000)],[row(91672,1,T-3_600_000,"bun.exe")]),
 case("the window of a stopped try ends sixty seconds after its end, inclusive",[ent(701,T+1380000+GRACE),ent(702,T+1380000+GRACE+1)],[row(701,1,T+1380000+GRACE,"bun.exe"),row(702,1,T+1380000+GRACE+1,"bun.exe")],orphans=[701]),
 case("no stopped try, nothing to reap",[ent(91672,S0)],[row(91672,1,S0,"bun.exe")],windows=[]),
 case("a process that started in the working window of a person is theirs; one that started before it is not",[ent(710,T+600000),ent(711,T+400000)],[row(710,1,T+600000,"bun.exe"),row(711,1,T+400000,"bun.exe")],human={"pid":77,"startedMs":T+500000},orphans=[711]),
 case("a process below the long-running wrapper of a person is theirs",[ent(720,S0)],[row(55,4,T-8_000_000,"powershell.exe"),row(720,55,S0,"bun.exe")],human={"pid":55,"startedMs":T-8_000_000},orphans=[]),
 case("a parent pid that was reused after the child started is no parent",[ent(721,S0)],[row(55,4,S0+1_000_000,"powershell.exe"),row(721,55,S0,"bun.exe")],human={"pid":55,"startedMs":S0+1_000_000-9_000_000_000},orphans=[721]),
 case("the tree of the current stage and everything below it is not an orphan, an unrelated process is",[ent(902,S0),ent(903,S0+1000),ent(904,S0+2000)],[row(900,1000,S0-1000,"bun.exe"),row(901,900,S0,"bun.exe"),row(902,901,S0+500,"bun.exe"),row(903,902,S0+1000,"bun.exe"),row(904,40000,S0+2000,"bun.exe")],stage={"root":{"pid":900,"startMs":S0-1000},"known":[]},orphans=[904]),
 case("a recorded member of the stage whose parent exited is in the stage",[ent(950,S0),ent(951,S0)],[row(900,1000,S0-1000,"bun.exe"),row(950,40000,S0,"bun.exe"),row(951,40000,S0,"bun.exe")],stage={"root":{"pid":900,"startMs":S0-1000},"known":[{"pid":950,"startMs":S0}]},orphans=[951]),
 case("this process and its ancestors are never reaped",[ent(1001,T-8_000_000),ent(1000,T-9_000_000)],[],windows=[{"startedMs":T-10_000_000,"endedMs":T}],orphans=[]),
 case("several orphans come out ordered by pid, one pid once",[ent(800,S0),ent(300,S0+10),ent(800,S0)],[row(800,1,S0,"bun.exe"),row(300,1,S0+10,"node.exe")],orphans=[300,800]),
 case("the edge of the working window of a person is two seconds wide: inside it is theirs, outside it is not",[ent(730,T+498001),ent(731,T+497999)],[row(730,1,T+498001,"bun.exe"),row(731,1,T+497999,"bun.exe")],human={"pid":77,"startedMs":T+500000},orphans=[731]),
 case("the edge of a window of a stopped try is two seconds wide",[ent(740,T-1999),ent(741,T-2001)],[row(740,1,T-1999,"bun.exe"),row(741,1,T-2001,"bun.exe")],orphans=[740]),
 case("paths compare without case and with either separator",[ent(91672,S0,command="c:/USERS/w/SEMIO/script.ts show projects",directory="c:/USERS/w/SEMIO/.cache/b/c")],[row(91672,1,S0,"bun.exe")],orphans=[91672]),
 case("the system pids are never reaped",[ent(3,S0)],[row(3,0,S0,"System")],orphans=[]),
 case("a start time inside the tolerance of the registered one is the same process",[ent(91672,S0+14000)],[row(91672,1,S0,"bun.exe")],orphans=[91672])]
runs=lambda*r:[{"stage":s,"startedMs":a,"endedMs":b,"outcome":o} for s,a,b,o in r]
fx={"delay":[{"elapsedMs":e,"delayMs":d} for e,d in [(0,1000),(59999,1000),(60000,5000),(3600000,5000)]],
 "window":[{"name":"only stopped tries are windows","runs":runs(("native",1,2,"ok"),("dependencies",10,20,"stalled"),("tools",30,40,"stage-failed"),("tools",50,60,"stage-timeout"),("workspace",70,80,"interrupted"),("tools",90,95,"spawn-failed")),"windows":[{"startedMs":10,"endedMs":20},{"startedMs":50,"endedMs":60},{"startedMs":70,"endedMs":80}]},
  {"name":"no run, no window","runs":[],"windows":[]}],
 "orphan":orphan,
 "taskkill":[{"pid":812,"arguments":["/PID","812","/T","/F"]},{"pid":91672,"arguments":["/PID","91672","/T","/F"]}]}
def heir(name,previous,since,table,heirs):return {"name":name,"previous":previous,"sinceMs":since,"table":table,"heirs":heirs}
fx["heir"]=[heir("a launcher that exited after it started a detached child leaves the child to the roster",[{"pid":600,"startMs":T}],T+100,[row(601,600,T+300,"bun.exe")],[601]),
 heir("a process that was already running at the previous sample is not the child of a launcher that just exited",[{"pid":600,"startMs":T}],T+100_000,[row(601,600,T+300,"bun.exe")],[]),
 heir("a parent that is still there is walked by the table, not adopted",[{"pid":600,"startMs":T}],T+100,[row(600,4,T,"cmd.exe"),row(601,600,T+300,"bun.exe")],[]),
 heir("a child that started before the recorded parent only shares its pid",[{"pid":600,"startMs":T+60000}],T+100,[row(601,600,T+300,"bun.exe")],[]),
 heir("a member already in the roster is not adopted again",[{"pid":600,"startMs":T},{"pid":601,"startMs":T+300}],T+100,[row(601,600,T+300,"bun.exe")],[]),
 heir("a parent that was never in the roster adopts nothing",[{"pid":700,"startMs":T}],T+100,[row(601,600,T+300,"bun.exe")],[]),
 heir("the start of the child may lie within the tolerance before the previous sample",[{"pid":600,"startMs":T}],T+10_000,[row(602,600,T+1000,"bun.exe"),row(603,600,T-30000,"bun.exe")],[602]),
 heir("a parent whose pid another process holds now is not adopted from",[{"pid":600,"startMs":T}],T+100,[row(600,4,T+9_000_000,"other.exe"),row(601,600,T+300,"bun.exe")],[])]
out=sys.argv[1]
open(out,"w",encoding="utf8").write("{\n"+",\n".join(" "+json.dumps(k)+": "+("[\n"+",\n".join("  "+json.dumps(c,ensure_ascii=False) for c in v)+"\n ]") for k,v in fx.items())+"\n}\n")
