import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from patchlib import rep, sha

root = sys.argv[1]
J = os.path.join(root, "🔣️.json")
S = os.path.join(root, "🧬️schema", "🔣️.json")
M = os.path.join(root, "🟨️.mjs")
D = os.path.join(root, "🟨️.d.mts")
for path in (J, S, M, D):
    print(path[-40:], sha(path))

rep(J, ' "pollMilliseconds": 3000,\n', ' "pollMilliseconds": 3000,\n "doorPollMilliseconds": 500,\n')
rep(J, "A run yields at most maximumYields times;", "A holder that is gone is never waited for: every doorPollMilliseconds the front door reads the live holders of the door the run queued in, by pid and start time, and starts again at once when none is left. A run yields at most maximumYields times;")
rep(J, "Ein Lauf gibt höchstens maximumYields Mal ab;", "Auf einen beendeten Halter wird nie gewartet: alle doorPollMilliseconds liest die Vordertür die lebenden Halter der Tür, in der sich der Lauf angestellt hat, nach Pid und Startzeit und startet sofort neu, sobald keiner mehr da ist. Ein Lauf gibt höchstens maximumYields Mal ab;")
rep(J, '''   "de": "graph: Generation {generation} ist aktuell oder die Wartezeit endete nach {seconds} s; Neustart mit der Anfragezeit des ersten Starts"
  }
''', '''   "de": "graph: Generation {generation} ist aktuell oder die Wartezeit endete nach {seconds} s; Neustart mit der Anfragezeit des ersten Starts"
  },
  "holderGone": {
   "en": "graph: no live holder is ahead any more (the holder of the door ended); starting again after {seconds} s with the request time of the first start",
   "de": "graph: kein lebender Halter ist mehr vor diesem Lauf (der Halter der Tür ist beendet); Neustart nach {seconds} s mit der Anfragezeit des ersten Starts"
  }
''')

rep(S, '''  "pollMilliseconds",
  "progressMilliseconds",''', '''  "pollMilliseconds",
  "doorPollMilliseconds",
  "progressMilliseconds",''')
rep(S, '''  "progressMilliseconds": {
   "type": "integer",
   "minimum": 1000
  },''', '''  "doorPollMilliseconds": {
   "type": "integer",
   "minimum": 100,
   "maximum": 1000
  },
  "progressMilliseconds": {
   "type": "integer",
   "minimum": 1000
  },''')
rep(S, '''    "waiting",
    "resumed"
   ],''', '''    "waiting",
    "resumed",
    "holderGone"
   ],''')
rep(S, '''    "resumed": {
     "type": "object",
     "additionalProperties": false,
     "required": [
      "en",
      "de"
     ],
     "properties": {
      "en": {
       "type": "string",
       "minLength": 1
      },
      "de": {
       "type": "string",
       "minLength": 1
      }
     }
    }
''', '''    "resumed": {
     "type": "object",
     "additionalProperties": false,
     "required": [
      "en",
      "de"
     ],
     "properties": {
      "en": {
       "type": "string",
       "minLength": 1
      },
      "de": {
       "type": "string",
       "minLength": 1
      }
     }
    },
    "holderGone": {
     "type": "object",
     "additionalProperties": false,
     "required": [
      "en",
      "de"
     ],
     "properties": {
      "en": {
       "type": "string",
       "minLength": 1
      },
      "de": {
       "type": "string",
       "minLength": 1
      }
     }
    }
''')

rep(M, 'import{doorSuffixV1,rememberedDoorV1}from"../🪧️door-board/🟨️.mjs";', 'import{doorSuffixV1,readDoorV1,rememberedDoorV1}from"../🪧️door-board/🟨️.mjs";')
rep(M, '/** ⏳️ Consumes a graph yield ticket, settles its wait preparation, then polls for a newer generation or the bounded wake. Carries the initial request time and next round to the replacement. */\nexport async function waitGraphQueueYieldV1({environment=process.env,now=Date.now,sleep=pause,write=defaultWrite,read=readGeneration,signal,onAccepted}={}){',
    '/** ⏳️ Consumes a graph yield ticket, settles its wait preparation, then polls for a newer generation, for the door to lose its last live holder (read by pid and start time, so a holder that already ended is never waited for) or for the bounded wake. Carries the initial request time and next round to the replacement. */\nexport async function waitGraphQueueYieldV1({environment=process.env,now=Date.now,sleep=pause,write=defaultWrite,read=readGeneration,holders=readDoorV1,signal,onAccepted}={}){')
rep(M, ' const started=now();let reported=started;\n', ' const started=now(),watched=typeof ticket.door==="string"&&ticket.door!=="";let reported=started;\n')
rep(M, '''  if(now()-reported>=policy.progressMilliseconds){''', '''  if(watched&&holders(ticket.door).length===0){write(render("holderGone",{seconds:Math.round(elapsed/1000)},environment));break;}
  if(now()-reported>=policy.progressMilliseconds){''')
rep(M, '  await sleep(policy.pollMilliseconds,signal);', '  await sleep(watched?Math.min(policy.pollMilliseconds,policy.doorPollMilliseconds):policy.pollMilliseconds,signal);')

rep(D, 'pollMilliseconds:number;progressMilliseconds:number;', 'pollMilliseconds:number;doorPollMilliseconds:number;progressMilliseconds:number;')
rep(D, 'Record<"waiting"|"resumed",', 'Record<"waiting"|"resumed"|"holderGone",')
rep(D, '/** ⏳️ The front door\'s wait after a yielded run: polls the pointer until a newer generation was published or the wake time passed. */', '/** ⏳️ The front door\'s wait after a yielded run: polls the pointer until a newer generation was published, the door has no live holder left (pid and start time) or the wake time passed. */')
rep(D, 'read?:(pointer:string)=>number;signal?:AbortSignal;', 'read?:(pointer:string)=>number;holders?:(door:string)=>readonly unknown[];signal?:AbortSignal;')
print("yield patched")
