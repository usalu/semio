import re,sys
p=sys.argv[1]
s=open(p,encoding="utf-8").read()
pat=re.compile(r"(?:          // 🧩️[^\n]*\n(?:          //[^\n]*\n)*)?          for \(let index = 0; index < admitted\.length - 1;.*?let result = await actor\.api\.poll\(admitted\.length > 1 [^\n]*",re.S)
assert len(pat.findall(s))==1
new='''          if (admitted.length > 1) throw new Error("actor-poll.event-count: a crossing carries at most one event; send each event as its own crossing");
          let result = await actor.api.poll(admitted, msg.commandPage, undefined, msg.budget);'''
open(p,"w",encoding="utf-8").write(pat.sub(lambda m:new,s))
