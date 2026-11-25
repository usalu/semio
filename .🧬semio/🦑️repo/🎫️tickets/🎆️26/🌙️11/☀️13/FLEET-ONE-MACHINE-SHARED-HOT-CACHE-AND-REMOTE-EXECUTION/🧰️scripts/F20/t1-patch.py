base="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/"
def sub(path,old,new):
    t=open(path,encoding="utf8").read()
    assert t.count(old)==1,(path,old[:70],t.count(old))
    open(path,"w",encoding="utf8").write(t.replace(old,new))
m=base+"🟨️.mjs"
sub(m,'''const asyncAdmissionChecksV1=new WeakMap();
''','''const asyncAdmissionChecksV1=new WeakMap();
const carriedVerdictsV1=new WeakMap(),carriedCountsV1={reused:0,probed:0};
/** 🤝️ How many rows of in-round holds took a verdict that a fresh verification gave a moment ago (`reused`) and how many were probed (`probed`) in this process. */
export function observationCarriedCountsV1(){return{...carriedCountsV1};}
''')
sub(m,'''      let verdict=verdicts?.get(row);
      if(verdict===undefined){
        const pending=check(row);
        verdicts?.set(row,pending);
        verdict=await pending;
        verdicts?.set(row,verdict);
      }else if(typeof verdict!=="boolean")verdict=await verdict;''','''      let verdict=verdicts?.get(row);
      if(verdict===undefined){
        const carriedAt=verdicts===undefined?undefined:carriedVerdictsV1.get(row);
        if(carriedAt!==undefined&&performance.now()-carriedAt<OBSERVATION_LIMITS_V1.carriedVerdictMilliseconds){verdict=true;verdicts.set(row,true);carriedCountsV1.reused++;}
        else{
          const began=performance.now(),pending=check(row);
          verdicts?.set(row,pending);
          verdict=await pending;
          verdicts?.set(row,verdict);
          carriedCountsV1.probed++;
          if(verdicts===undefined&&observedWorlds.getStore()===undefined){if(verdict)carriedVerdictsV1.set(row,began);else carriedVerdictsV1.delete(row);}
        }
      }else if(typeof verdict!=="boolean")verdict=await verdict;''')
sc=base+"🧬️schema/🔣️.json"
sub(sc,'        "holdProgressRows": 64,\n','        "holdProgressRows": 64,\n        "carriedVerdictMilliseconds": 60000,\n')
sub(sc,'          "holdProgressRows": "The minimum new contiguous verified observations for reporting source validation progress.",\n','          "holdProgressRows": "The minimum new contiguous verified observations for reporting source validation progress.",\n          "carriedVerdictMilliseconds": "How long a row that a fresh complete verification found to hold is taken as holding by the in-round holds of the next pass, so a pass that restarts for one changed path does not probe the tens of thousands of untouched rows a second time; the fresh verification at the end of every pass never takes a carried verdict and refutes a row that changed meanwhile.",\n')
sub(sc,'          "holdProgressRows": "Die Mindestzahl weiterer zusammenhängend bestätigter Beobachtungen pro Fortschrittsmeldung.",\n','          "holdProgressRows": "Die Mindestzahl weiterer zusammenhängend bestätigter Beobachtungen pro Fortschrittsmeldung.",\n          "carriedVerdictMilliseconds": "Wie lange eine Zeile, die eine frische vollständige Prüfung als gültig befunden hat, von den Runden-Prüfungen des nächsten Durchgangs als gültig übernommen wird, damit ein Durchgang, der wegen eines geänderten Pfads neu startet, die zehntausende unberührten Zeilen nicht ein zweites Mal prüft; die frische Prüfung am Ende jedes Durchgangs übernimmt nie ein getragenes Urteil und widerlegt eine Zeile, die sich inzwischen änderte.",\n')
sub(base+"🟨️.d.mts","export function observationProbeCountsV1():{readdir:number};","export function observationProbeCountsV1():{readdir:number};\nexport function observationCarriedCountsV1():{reused:number;probed:number};")
