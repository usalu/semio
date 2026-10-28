"""🔁️ quiet16 batch `lastrun` (stacked on `hold`), edits by anchor: the door forecasts a run by its class's own last outcome. A run is forecast quiet when the newest run of its class executed nothing (however long ago, since `hold` grows a wrong quiet grant when work runs), or when the newest run executed work and finished within the follow-up window; the decayed quiet share and the minimum of quiet runs (`quietShareBasisPoints`, `quietMinimumSamples`) are gone from the forecast, the policy, the schema, the fixtures and the Python ports.
usage: python3 -I edit_lastrun.py <root>   (exit 1 = an anchor is missing or not unique; idempotent). The oracles regenerate their fixtures afterwards (make_lastrun.sh)."""
import os, stat, sys

H = "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
T = H + "🧪️tests/"
KNOBS_COMPACT = ('"quietShareBasisPoints":8000,"quietMinimumSamples":2,', "")
EDITS = {
    H + "🌦️forecast/🟦️.ts": [
        ("""import {estimateHostRatioV1,hostSampleWeightsV1,recordHostOutcomeV1,type HostEstimateStoreV1} from "../📐️estimates/🟦️.ts";

const UNIT=1_000_000;
""", """import {estimateHostRatioV1,hostSampleWeightsV1,recordHostOutcomeV1,type HostEstimateStoreV1} from "../📐️estimates/🟦️.ts";
"""),
        ("""/** 🌦️ What the door expects of a run before it starts: `quiet` when the class of the run has mostly executed nothing so far (a fully cached or covered plan), the small reservation such a run has needed (`megabytes`) and the time it needs to reach it, the effective quiet runs behind the forecast and the share of quiet runs among the recent ones, in basis points; `followUpMilliseconds` (only when the follow-up rule alone made the forecast quiet) is the time since the earlier run of the class finished. */""",
         """/** 🌦️ What the door expects of a run before it starts: `quiet` when the newest run of its class executed nothing (a fully cached or covered plan) or followed closely on one that executed work, the small reservation such a run has needed (`megabytes`) and the time it needs to reach it, the effective quiet runs behind the size and the share of quiet runs among the recent ones, in basis points (both said in the forecast line); `followUpMilliseconds` (only when the follow-up rule made the forecast quiet) is the time since the earlier run of the class finished. */"""),
        ("""/** 🔮️ Forecasts whether the next run of a class executes nothing: it does when the earlier run of the class finished at most `quietFollowUpMilliseconds` ago (a run that ended well left its results cached, so the run that follows it is a replay unless the world changed in between: the follow-up rule, which also starts a class that has no quiet history yet; a wrong forecast costs nothing because the upgrade protects, a missed one costs the whole queue wait) or when enough decayed quiet runs exist (`quietMinimumSamples`), they are at least `quietShareBasisPoints` of the quiet and busy runs together, the newest of all runs was quiet (one busy run flips the forecast at once, so the run after an edit asks for the full grant) and the learned quiet size stays within `quietMaximumMegabytes`. The size is the quiet class's own estimate (decayed quantile of the peaks plus margin, trust-blended with the reference) against `quietReferenceMegabytes`; a forecast never changes what a run may use, only what it must reserve before it knows. */""",
         """/** 🔮️ Forecasts whether the next run of a class executes nothing, by the class's own last outcome: it does when the newest run of the class executed nothing, however long ago (the run's own last outcome), or when the newest run executed work and finished at most `quietFollowUpMilliseconds` ago (it left its results cached, so the run that follows it is a replay unless the world changed in between: the follow-up rule); one busy run outside the window flips the forecast at once, so the run long after an edit asks for the full grant. A wrong quiet forecast costs only the growth when work runs (an executor asks, or the run's tree outgrows the quiet reservation), a missed one costs the whole queue wait, so the forecast leans quiet. The learned quiet size must stay within `quietMaximumMegabytes`; it is the quiet class's own estimate (decayed quantile of the peaks plus margin, trust-blended with the reference) against `quietReferenceMegabytes`; a forecast never changes what a run may use, only what it must reserve before it knows. */"""),
        ("""const history=quietWeight>=rules.quietMinimumSamples*UNIT&&share>=rules.quietShareBasisPoints&&newest(quiet)>newest(busy),since=nowMs-Math.max(newest(quiet),newest(busy)),followUp=!history&&rules.quietFollowUpMilliseconds>0&&Math.max(newest(quiet),newest(busy))>=0&&since>=0&&since<=rules.quietFollowUpMilliseconds,quiet_=(history||followUp)&&megabytes<=rules.quietMaximumMegabytes;""",
         """const own=newest(quiet)>newest(busy),since=nowMs-newest(busy),followUp=!own&&rules.quietFollowUpMilliseconds>0&&newest(busy)>=0&&since>=0&&since<=rules.quietFollowUpMilliseconds,quiet_=(own||followUp)&&megabytes<=rules.quietMaximumMegabytes;"""),
    ],
    H + "📏️decision/🟦️.ts": [
        ("quietReferenceMegabytes:number;quietShareBasisPoints:number;quietMinimumSamples:number;quietMaximumMegabytes:number;", "quietReferenceMegabytes:number;quietMaximumMegabytes:number;"),
    ],
    H + "🧬️schema/🔣️.json": [
        ("""        "quietReferenceMegabytes",
        "quietShareBasisPoints",
        "quietMinimumSamples",
        "quietMaximumMegabytes",""", """        "quietReferenceMegabytes",
        "quietMaximumMegabytes","""),
        ('"quiet": "[capacity] {operation}: forecast quiet from {samples} earlier run(s) of its class ({percent}% of them executed nothing): {reserved} MB reserved now, the full {full} MB only if work has to run",', '"quiet": "[capacity] {operation}: forecast quiet because the last run of its class executed nothing ({samples} quiet run(s) behind its size, {percent}% of the recent runs): {reserved} MB reserved now, the full {full} MB only if work has to run",'),
        ('"quiet": "[capacity] {operation}: ruhig prognostiziert aus {samples} früheren Läufen seiner Klasse ({percent}% führten nichts aus): jetzt {reserved} MB reserviert, die vollen {full} MB nur, wenn Arbeit laufen muss",', '"quiet": "[capacity] {operation}: ruhig prognostiziert, weil der letzte Lauf seiner Klasse nichts ausführte ({samples} ruhige Läufe hinter ihrer Größe, {percent}% der jüngsten Läufe): jetzt {reserved} MB reserviert, die vollen {full} MB nur, wenn Arbeit laufen muss",'),
        ("""        "quietShareBasisPoints": {
          "type": "integer",
          "minimum": 1,
          "maximum": 10000
        },
        "quietMinimumSamples": {
          "type": "integer",
          "minimum": 1,
          "maximum": 64
        },
""", ""),
    ],
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/🟦️.ts": [
        ("""/^\\[capacity\\] run: forecast quiet from \\d+ earlier run\\(s\\) of its class \\(100% of them executed nothing\\): \\d+ MB reserved now""", """/^\\[capacity\\] run: forecast quiet because the last run of its class executed nothing \\(\\d+ quiet run\\(s\\) behind its size, 100% of the recent runs\\): \\d+ MB reserved now"""),
    ],
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json": [
        ("""      "quietShareBasisPoints": 8000,
      "quietMinimumSamples": 2,
""", ""),
    ],
    T + "🧫️fixtures/🔣️.json": [KNOBS_COMPACT, KNOBS_COMPACT],
    T + "🪜️phases/🧫️fixtures/🔣️.json": [KNOBS_COMPACT, KNOBS_COMPACT],
    T + "📐️estimates/🧫️fixtures/🔣️.json": [KNOBS_COMPACT, KNOBS_COMPACT],
    T + "🌤️followup/🧫️fixtures/📼️recorded.json": [("""   "quietShareBasisPoints": 8000,
   "quietMinimumSamples": 2,
""", "")],
    T + "📐️estimates/🐍️oracle/🐍️.py": [('"quietShareBasisPoints": 8000, "quietMinimumSamples": 2, ', "")],
    T + "🔼️upgrade/🐍️oracle/🐍️.py": [
        ("newest-run rule, quiet size from the estimates oracle", "the run's own last outcome or the follow-up window, quiet size from the estimates oracle"),
        ("""    history=wq>=e["quietMinimumSamples"]*UNIT and share>=e["quietShareBasisPoints"] and newest(quiet)>newest(busy)
    latest=max(newest(quiet),newest(busy));since=now-latest
    follow_up=(not history) and e.get("quietFollowUpMilliseconds",0)>0 and latest>=0 and since>=0 and since<=e["quietFollowUpMilliseconds"]
    verdict=(history or follow_up) and megabytes<=e["quietMaximumMegabytes"]""",
         """    own=newest(quiet)>newest(busy);since=now-newest(busy)
    follow_up=(not own) and e.get("quietFollowUpMilliseconds",0)>0 and newest(busy)>=0 and since>=0 and since<=e["quietFollowUpMilliseconds"]
    verdict=(own or follow_up) and megabytes<=e["quietMaximumMegabytes"]"""),
    ],
    T + "🌤️followup/🟦️.ts": [
        ("""test("The chain goes on: every run after a finished run is forecast quiet for the follow-up window, the window ends, the forecast equals the oracle on every step",()=>{""", """test("The chain goes on: every run after a finished run is forecast quiet for the follow-up window, after the window the class's own last run decides, the forecast equals the oracle on every step",()=>{"""),
        (""" expect(replay.afterWindow.expect.quiet,"without a run for longer than the window and without enough quiet history the door asks for the full grant again").toBe(false);""", """ expect(replay.afterWindow.expect,"the window is over and the class's own last run executed nothing, so the door stays quiet by its own outcome").toMatchObject({quiet:true});
 expect(replay.afterWindow.expect,"not by the follow-up rule").not.toHaveProperty("followUpMilliseconds");"""),
    ],
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🌤️followup/🟦️.ts": [
        ("""test("A class without a finished run in the follow-up window asks for the full grant at the door",async()=>{
 const directory=fresh(),machine=holdMachine(directory);
 await finished(directory,policy.estimates.quietFollowUpMilliseconds+60_000,false);
 await sleep(40);
 let started=false;
 const waiting=withBuildCapacityV1(args,{SEMIO_HOST_CAPACITY_DIRECTORY:directory},async()=>{started=true;},ports(directory));
 await sleep(300);expect(started,"one old quiet run is neither history nor a follow-up").toBe(false);
 machine.release();await waiting;await machine.done;expect(started).toBe(true);
},30000);""", """test("A class whose last run executed work and ended before the follow-up window asks for the full grant at the door, one whose last run executed nothing stays quiet however long ago it ran",async()=>{
 const directory=fresh(),machine=holdMachine(directory);
 await finished(directory,policy.estimates.quietFollowUpMilliseconds+120_000,false);await finished(directory,policy.estimates.quietFollowUpMilliseconds+60_000,true);
 await sleep(40);
 let started=false;
 const waiting=withBuildCapacityV1(args,{SEMIO_HOST_CAPACITY_DIRECTORY:directory},async()=>{started=true;},ports(directory));
 await sleep(300);expect(started,"the last run executed work and the window is over").toBe(false);
 machine.release();await waiting;await machine.done;expect(started).toBe(true);
 const quiet=fresh(),full=holdMachine(quiet);
 await finished(quiet,7*86_400_000,false);
 await sleep(40);
 let answered=false;
 const served=withBuildCapacityV1(args,{SEMIO_HOST_CAPACITY_DIRECTORY:quiet},async granted=>{answered=true;expect(granted.handle!.reservedMegabytes).toBeLessThan(4096);},ports(quiet));
 await sleep(300);expect(answered,"its own last run, a week ago, executed nothing").toBe(true);
 await served;full.release();await full.done;
},30000);"""),
    ],
    T + "🔼️upgrade/🟦️.ts": [
        ("""expect(byName("all-quiet-1").quiet,"one quiet run is not enough evidence").toBe(false);""", """expect(byName("all-quiet-1").quiet,"the class's own last run executed nothing").toBe(true);"""),
    ],
}


def main():
    root = os.path.abspath(sys.argv[1])
    for rel, pairs in EDITS.items():
        path = os.path.join(root, rel)
        text = open(path, encoding="utf8").read()
        for index, (old, new) in enumerate(pairs):
            if (old, new) == KNOBS_COMPACT:
                continue
            if old not in text and (new == "" or new in text):
                continue
            if text.count(old) != 1:
                print("ANCHOR", text.count(old), rel[-60:], old[:90]); sys.exit(1)
            text = text.replace(old, new)
        if KNOBS_COMPACT in pairs:
            text = text.replace(KNOBS_COMPACT[0], "")
        mode = stat.S_IMODE(os.stat(path).st_mode)
        temporary = path + ".quiet16.tmp"
        open(temporary, "w", encoding="utf8").write(text); os.chmod(temporary, mode); os.replace(temporary, path)
        print("edited", rel[-70:])


main()
