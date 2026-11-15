import sys
root=sys.argv[1]
def edit(path,pairs):
    s=open(root+"/"+path).read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:60])
        s=s.replace(a,b)
    open(root+"/"+path,"w").write(s)
edit("🔁️replica/⌨️command/🟦️.ts",[
 ('|"compaction"|"verify"|','|"compaction"|"consolidate"|"verify"|'),
 ('  compaction:{valued:["--worktree","--store","--lang"],plain:["--plan","--apply","--no-blobs"]},','  compaction:{valued:["--worktree","--store","--lang"],plain:["--plan","--apply","--no-blobs"]},\n  consolidate:{valued:["--worktree","--store","--lang"],plain:["--apply"]},'),
 ('"--poll","--bootstrap-minutes","--concurrency","--lang"],plain:["--once"]}','"--poll","--bootstrap-minutes","--consolidate-minutes","--concurrency","--lang"],plain:["--once"]}'),
 (' if(parsed.operation==="compaction"){',' if(parsed.operation==="consolidate"){\n  const result=await consolidateGitV1({worktree:options.worktree,apply:parsed.flags.has("--apply"),storeRoot,signal,sink:event=>sink(event)});\n  say(`consolidate.${result.outcome}`,{packs:result.packsBefore,bytes:formatReplicaBytesV1(result.bytesBefore),retired:result.retired,freed:formatReplicaBytesV1(result.bytesFreed)});\n  return {exitCode:["failed","unpublished","space","locked"].includes(result.outcome)?1:0};\n }\n if(parsed.operation==="compaction"){'),
 ('import {refreshBootstrapV1,runBootstrapCommandV1} from "../🥾️bootstrap/🟦️.ts";','import {refreshBootstrapV1,runBootstrapCommandV1} from "../🥾️bootstrap/🟦️.ts";\nimport {consolidateGitV1,refreshConsolidationV1} from "../🧱️consolidate/🟦️.ts";'),
 ('machine===undefined?Promise.resolve():refreshBootstrapV1(','machine===undefined||consolidateMinutes===0?Promise.resolve():refreshConsolidationV1({worktree:where(context.root,values["--worktree"]??context.root),storeRoot:ownStore,minutes:consolidateMinutes},signal,sink),machine===undefined?Promise.resolve():refreshBootstrapV1('),
 ('  const minutes=values["--bootstrap-minutes"]===undefined?','  const consolidateMinutes=values["--consolidate-minutes"]===undefined?1440:whole("--consolidate-minutes",values["--consolidate-minutes"],0,10080);\n  const minutes=values["--bootstrap-minutes"]===undefined?'),
])
t=[("nothing","One pack and nothing to retire: nothing to do.","Ein Pack und nichts zu entfernen: nichts zu tun."),
("planned","Plan: {packs} packs, {bytes}; {retired} would be retired and about {freed} freed (run with --apply).","Plan: {packs} Packs, {bytes}; {retired} würden entfernt und etwa {freed} frei (mit --apply ausführen)."),
("consolidated","Consolidated: {retired} packs retired, {freed} freed; every fleet machine follows through the replica.","Zusammengeführt: {retired} Packs entfernt, {freed} frei; alle Maschinen des Fleets folgen über das Replikat."),
("locked","Another consolidation of this worktree is running; nothing was done.","Eine andere Zusammenführung dieses Arbeitsverzeichnisses läuft; es wurde nichts getan."),
("space","Not enough free disk space for the new pack; nothing was written.","Nicht genug freier Speicherplatz für das neue Pack; es wurde nichts geschrieben."),
("unpublished","The new pack was not published to the store in time; nothing was retired. Run again later.","Das neue Pack wurde nicht rechtzeitig im Speicher veröffentlicht; nichts wurde entfernt. Später erneut ausführen."),
("failed","git could not write or verify the new pack; nothing was retired.","git konnte das neue Pack nicht schreiben oder prüfen; nichts wurde entfernt.")]
body="".join('    "consolidate.%s": {\n      "en": "%s",\n      "de": "%s"\n    },\n'%x for x in t)
edit("🔁️replica/📖️texts/🔣️.json",[('    "error.compaction": {',body+'    "error.compaction": {')])
