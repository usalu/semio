"""🧩️ Applies (or checks) the staged change of preadmit-g to a root: the unified diff with `patch -p1 -F0` (every hunk must match its context exactly, so a peer edit near a hunk stops the batch before the first byte is written: all files are dry-run first), then the two documentation paragraphs (EN + DE) anchored in `📓️build-guide.md`. Usage: python3 apply.py check <root> | apply <root> | patch-only <root> (the diff without the guide, for a throw-away overlay). Needs batch 4 (phase-g) in the root."""
import os,subprocess,sys
HERE=os.path.dirname(os.path.abspath(__file__))
DIFF=os.path.join(HERE,"staged","preadmit.diff")
GUIDE="🧰️framework/🛍️products/🦑️repo/📓️build-guide.md"
EN="A run of a tool whose executors are gated (Bazel) whose class has mostly executed nothing so far (a fully cached or covered plan) holds only a small quiet reservation, answers at once and passes the queue barrier (`[capacity] run: forecast quiet from 5 earlier run(s) of its class (100% of them executed nothing): 1500 MB reserved now, the full 5632 MB only if work has to run`); the planner, a graph rebuild and the first Bazel action that really runs first grow it into the full request in the fair queue, with the position the run arrived with and a progress line (`[capacity] run: work has to run, waiting 12s to grow the reservation from 1500 MB to 5632 MB: not enough free memory`), and a run that executed anything is learned as busy, so the run after an edit asks for the full grant at the door. "
DE="Ein Lauf eines Werkzeugs mit bewachten Ausführern (Bazel), dessen Klasse bisher meist nichts ausgeführt hat (vollständig zwischengespeicherter oder abgedeckter Plan), hält nur eine kleine ruhige Reservierung, antwortet sofort und überholt die Warteschlangen-Sperre (`[capacity] run: ruhig prognostiziert aus 5 frueheren Laeufen seiner Klasse (100% fuehrten nichts aus): jetzt 1500 MB reserviert, die vollen 5632 MB nur, wenn Arbeit laufen muss`); der Planer, ein Graph-Neuaufbau und die erste Bazel-Aktion, die wirklich läuft, vergrößern sie zuerst in der fairen Warteschlange auf die volle Anfrage, mit der Position, mit der der Lauf angekommen ist, und einer Fortschrittszeile (`[capacity] run: es muss Arbeit laufen, wartet seit 12 s darauf, die Reservierung von 1500 MB auf 5632 MB zu vergroessern: nicht genug freier Speicher`); ein Lauf, der etwas ausgeführt hat, wird als beschäftigt gelernt, sodass der Lauf nach einer Änderung die volle Zusage schon an der Tür verlangt. "
def docs(root,write):
    path=os.path.join(root,GUIDE);text=open(path,encoding="utf8").read()
    for marker,paragraph in (("`SEMIO_HOST_CAPACITY=off` is the escape hatch",EN),("`SEMIO_HOST_CAPACITY=off` ist die Notausstiegsluke",DE)):
        if paragraph in text:continue
        if text.count(marker)!=1:raise SystemExit(f"guide anchor not unique ({text.count(marker)}): {marker}")
        text=text.replace(marker,paragraph+marker)
    if write:
        temporary=path+".preadmit-g.tmp"
        with open(temporary,"w",encoding="utf8") as handle:handle.write(text)
        os.replace(temporary,path)
def patch(root,dry):
    command=["patch","-p1","-F0","--no-backup-if-mismatch","-s","-d",root,"-i",DIFF]+(["--dry-run"] if dry else [])
    result=subprocess.run(command,capture_output=True,text=True)
    if result.returncode:raise SystemExit("patch refused:\n"+result.stdout+result.stderr)
if __name__=="__main__":
    if len(sys.argv)!=3 or sys.argv[1] not in ("check","apply","patch-only"):raise SystemExit(__doc__)
    root=sys.argv[2];patch(root,True)
    if sys.argv[1]!="patch-only":docs(root,False)
    if sys.argv[1]!="check":
        patch(root,False)
        if sys.argv[1]=="apply":docs(root,True)
    print(sys.argv[1],"ok")
