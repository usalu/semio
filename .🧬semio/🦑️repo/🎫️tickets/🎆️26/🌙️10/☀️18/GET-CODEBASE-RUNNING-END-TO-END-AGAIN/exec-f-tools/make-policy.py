"""python3 make-policy.py <out.json> <ports.json> : writes the scheduling-domain policy (domains.py prefixes, contracts, ports from ports.json {domain: [paths]})"""
import sys,json,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from domains import DOMAINS
ports=json.load(open(sys.argv[2])) if len(sys.argv)>2 and os.path.exists(sys.argv[2]) else {}
CONTRACT={
 "leases":("Resource leases: who may hold a named resource, in which order and for how long. The bytes an action produces never depend on which holder ran first.","Ressourcen-Leases: wer eine benannte Ressource in welcher Reihenfolge und wie lange halten darf. Die Bytes, die eine Aktion erzeugt, haengen nie davon ab, welcher Halter zuerst lief."),
 "capacity":("Host and compiler capacity: when and how many processes may start. An action produces the same bytes whether it waited for capacity or not.","Host- und Compiler-Kapazitaet: wann und wie viele Prozesse starten duerfen. Eine Aktion erzeugt dieselben Bytes, ob sie auf Kapazitaet gewartet hat oder nicht."),
 "store-custody":("Custody of the repository cache stores and native cache images (open, reserve, collect, retire, ledger) and of the sealed graph generations. It decides where bytes live, never what they are.","Obhut der Repository-Cache-Speicher und nativen Cache-Images (oeffnen, reservieren, einsammeln, ausmustern, Ledger) und der versiegelten Graph-Generationen. Sie bestimmt, wo Bytes liegen, nie was sie sind."),
 "process-supervision":("Supervision of the process an action runs in: invocation envelope, deadlines, watchdog, termination, recorded process state, agent sessions.","Ueberwachung des Prozesses, in dem eine Aktion laeuft: Aufruf-Huelle, Fristen, Watchdog, Beendigung, aufgezeichneter Prozesszustand, Agent-Sitzungen."),
 "retry-observation":("Bounded retry of file observations that peers change under a running build; it only decides when to read again.","Begrenzte Wiederholung von Datei-Beobachtungen, die Peers waehrend eines Laufs aendern; sie entscheidet nur, wann erneut gelesen wird."),
 "progress-reporting":("Progress lines and refusal texts shown to humans in English and German; no action output is derived from them.","Fortschrittszeilen und Ablehnungstexte fuer Menschen auf Englisch und Deutsch; keine Aktionsausgabe wird aus ihnen abgeleitet."),
}
doc={"version":1,"domains":[{"id":k,"contract":{"en":CONTRACT[k][0],"de":CONTRACT[k][1]},"prefixes":v,"ports":sorted(ports.get(k,[]))} for k,v in DOMAINS.items()]}
json.dump(doc,open(sys.argv[1],"w",encoding="utf8"),ensure_ascii=False,indent=2)
open(sys.argv[1],"a").write("\n")
