import sys
p=sys.argv[1]
s=open(p,encoding="utf-8").read()
a="          let result = await actor.api.poll(admitted, msg.commandPage, undefined, msg.budget);"
b='''          for (let index = 0; index < admitted.length - 1; index += 1) {
            const earlier = await actor.api.poll([admitted[index]], undefined, undefined, msg.budget);
            if (!shardTurnCarriesNothing(earlier)) throw new Error("actor-poll.event-batch-carried: an event of a batched crossing answered with something the host must act on; send it as its own crossing");
          }
          let result = await actor.api.poll(admitted.length > 1 ? [admitted[admitted.length - 1]] : admitted, msg.commandPage, undefined, msg.budget);'''
assert s.count(a)==1
open(p,"w",encoding="utf-8").write(s.replace(a,b))
