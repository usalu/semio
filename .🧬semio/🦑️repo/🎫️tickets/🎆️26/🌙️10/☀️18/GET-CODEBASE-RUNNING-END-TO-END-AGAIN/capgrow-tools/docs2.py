"""📝️ capgrow round 2: the guide bullets say that a run may grow right after its admission and that only a previous growth must ramp first (EN + DE, by anchor, keeping the mode): python3 -I docs2.py check|apply|revert [--root DIR] (exit 2 = drift)."""
import os,sys
args=sys.argv[1:];root=args[args.index("--root")+1] if "--root" in args else "/Users/ueli/Documents/semio";mode=args[0]
PATH=os.path.join(root,"🧰️framework/🛍️products/🦑️repo/📓️build-guide.md")
PAIRS=[("after its last reservation was observed (its ramp), on two consecutive heartbeats","after the reservation of its previous growth was observed (its ramp; the admission reservation counts in full while it ramps, so a run may grow right after its admission), on two consecutive heartbeats"),
 ("nachdem seine letzte Reservierung beobachtet ist (seine Rampe), an zwei Herzschlaegen in Folge","nachdem die Reservierung seines vorigen Wachstums beobachtet ist (seine Rampe; die Reservierung der Zulassung zaehlt voll, solange sie anlaeuft, also darf ein Lauf gleich nach seiner Zulassung wachsen), an zwei Herzschlaegen in Folge"),
 ("   - `SEMIO_HOST_CAPACITY=off` is the escape hatch;","   - **Accounting of other runs**: a grant's reservation counts in full while it ramps (its class's time to peak, 60 s for `repo capacity run --memory`), then by what its process tree holds; a holder that holds nothing (`sleep`) therefore stops holding memory after its ramp, while a command that really uses its memory keeps it counted. The `available` of a log line is the host sample, never net of reservations.\n   - `SEMIO_HOST_CAPACITY=off` is the escape hatch;"),
 ("   - `SEMIO_HOST_CAPACITY=off` ist die Notausstiegsluke;","   - **Anrechnung anderer Laeufe**: die Reservierung einer Freigabe zaehlt voll, solange sie anlaeuft (die Zeit ihrer Klasse bis zur Spitze, 60 s fuer `repo capacity run --memory`), danach mit dem, was ihr Prozessbaum haelt; ein Halter, der nichts haelt (`sleep`), haelt daher nach seiner Rampe keinen Speicher mehr, ein Befehl, der seinen Speicher wirklich nutzt, bleibt angerechnet. Das `available` einer Logzeile ist die Messung des Rechners, nie abzueglich der Reservierungen.\n   - `SEMIO_HOST_CAPACITY=off` ist die Notausstiegsluke;")]
text=open(PATH,encoding="utf8").read();fmode=os.stat(PATH).st_mode&0o7777
applied=all(new in text for _,new in PAIRS);pending=all(text.count(old)==1 for old,_ in PAIRS)
if mode=="check":print("applied" if applied else "pending" if pending else "drift");sys.exit(0 if applied or pending else 2)
def write(value):
    temporary=PATH+".capgrow.tmp";open(temporary,"w",encoding="utf8").write(value);os.chmod(temporary,fmode);os.replace(temporary,PATH)
if mode=="apply":
    if applied:print("already applied");sys.exit(0)
    if not pending:print("drift");sys.exit(2)
    for old,new in PAIRS:text=text.replace(old,new)
    write(text);print("applied")
elif mode=="revert":
    if not applied:print("not applied");sys.exit(0)
    for old,new in PAIRS:text=text.replace(new,old)
    write(text);print("reverted")
