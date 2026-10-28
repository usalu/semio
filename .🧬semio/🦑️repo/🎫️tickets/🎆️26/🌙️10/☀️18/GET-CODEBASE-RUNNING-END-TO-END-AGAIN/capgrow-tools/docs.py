"""📝️ capgrow: inserts the EN and DE guide bullets of the growth of a reduced grant and the multi-core compile into the hot build guide by anchor, keeping the file mode: python3 -I docs.py check|apply|revert [--root DIR] (exit 2 = anchor drift)."""
import os,sys
args=sys.argv[1:];root=args[args.index("--root")+1] if "--root" in args else "/Users/ueli/Documents/semio";mode=args[0]
PATH=os.path.join(root,"🧰️framework/🛍️products/🦑️repo/📓️build-guide.md")
EN="   - **Growing a reduced grant**: a Bazel run admitted with fewer jobs than it asked for (`[capacity] admitted run with 1 of 8 job(s): the host cannot carry more right now; the run grows to more jobs once memory frees up`) asks once per heartbeat to grow: it grows only while nothing else of ours waits, after its last reservation was observed (its ramp), on two consecutive heartbeats and to the most jobs whose memory fits with one more job's memory left free; it never shrinks back (`[capacity] run: grown from 1 to 8 of 8 job(s) ...`). Processes that start work afterwards follow the grant row: the Bazel invocation at its capacity point (`--parallel` carries the jobs it may grow into, `--jobs` follows the plan) and every planned compile.\n   - **Compile cores** (`🎟️capacity/🧮️units`): a planned compile under a run's grant takes the cores the grant's job memory carries at the unit footprint the rustc broker measured (75th percentile of the newest unit peaks, 2048 MB until 8 units are measured), at least the granted jobs and at most half the host; the broker never holds more than that memory. A 1-job grant with 2048 MB and 900 MB units compiles on 2 cores instead of 1.\n"
DE="   - **Wachsen einer reduzierten Freigabe**: ein Bazel-Lauf, der mit weniger Jobs zugelassen wurde, als er wollte (`[capacity] run mit 1 von 8 Job(s) zugelassen: der Rechner traegt gerade nicht mehr; der Lauf waechst auf mehr Jobs, sobald Speicher frei wird`), fragt je Herzschlag an zu wachsen: er waechst nur, solange nichts anderes von uns wartet, nachdem seine letzte Reservierung beobachtet ist (seine Rampe), an zwei Herzschlaegen in Folge und auf die meisten Jobs, deren Speicher mit dem Speicher eines weiteren Jobs als Reserve passt; er schrumpft nie zurueck (`[capacity] run: nach ... s von 1 auf 8 von 8 Job(s) gewachsen ...`). Prozesse, die danach Arbeit starten, folgen der Freigabe: der Bazel-Aufruf an seinem Kapazitaetspunkt (`--parallel` traegt die Jobs, in die er wachsen darf, `--jobs` folgt dem Plan) und jede geplante Kompilierung.\n   - **Kompilierkerne** (`🎟️capacity/🧮️units`): eine geplante Kompilierung unter der Freigabe eines Laufs nimmt die Kerne, die der Job-Speicher der Freigabe beim vom rustc-Broker gemessenen Platzbedarf einer Einheit traegt (75. Perzentil der neuesten Einheitsspitzen, 2048 MB bis 8 Einheiten gemessen sind), mindestens die zugelassenen Jobs und hoechstens den halben Rechner; der Broker haelt nie mehr als diesen Speicher. Eine Freigabe mit 1 Job, 2048 MB und Einheiten von 900 MB kompiliert auf 2 statt 1 Kern.\n"
ANCHORS=[("   - `SEMIO_HOST_CAPACITY=off` is the escape hatch;",EN),("   - `SEMIO_HOST_CAPACITY=off` ist die Notausstiegsluke;",DE)]
text=open(PATH,encoding="utf8").read();fmode=os.stat(PATH).st_mode&0o7777
def write(value):
    temporary=PATH+".capgrow.tmp";open(temporary,"w",encoding="utf8").write(value);os.chmod(temporary,fmode);os.replace(temporary,PATH)
applied=all(block in text for _,block in ANCHORS);anchored=all(text.count(anchor)==1 for anchor,_ in ANCHORS)
if mode=="check":print("applied" if applied else "pending" if anchored else "drift");sys.exit(0 if applied or anchored else 2)
if mode=="apply":
    if applied:print("already applied");sys.exit(0)
    if not anchored:print("drift: anchor missing");sys.exit(2)
    for anchor,block in ANCHORS:text=text.replace(anchor,block+anchor)
    write(text);print("applied")
elif mode=="revert":
    if not applied:print("not applied");sys.exit(0)
    for _,block in ANCHORS:text=text.replace(block,"")
    write(text);print("reverted")
