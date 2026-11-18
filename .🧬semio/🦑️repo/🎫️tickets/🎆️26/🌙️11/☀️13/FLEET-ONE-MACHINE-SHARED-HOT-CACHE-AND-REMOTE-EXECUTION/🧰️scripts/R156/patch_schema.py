import sys,json
def once(t,old,new):
    n=t.count(old)
    if n!=1: raise SystemExit("anchor %d: %s"%(n,old[:80]))
    return t.replace(old,new)
def patch(t):
    t=once(t,'"ownerFactsMemoEntries": 16384\n','"ownerFactsMemoEntries": 16384,\n        "sealStalePaths": 4096\n')
    en_old='"repairRounds": "The most times one attempt of a graph read repeats only the inference after a source it read changed, with the owner closure kept, before the attempt starts over.",'
    en_new='"repairRounds": "The most rounds a graph read follows the tree after a source it read, the membership or the owner closure changed (only the changed owners are observed or inferred again) before it seals with the records it has and says how many observed rows changed later.",'
    de_old='"repairRounds": "Wie oft ein Versuch eines Graph-Lesens nur die Inferenz wiederholt, nachdem sich eine gelesene Quelle änderte, bei behaltenem Eigentümerabschluss, bevor der Versuch von vorn beginnt.",'
    de_new='"repairRounds": "Wie viele Runden ein Graph-Lesen dem Baum folgt, nachdem sich eine gelesene Quelle, die Mitgliedschaft oder der Eigentümerabschluss änderte (nur die geänderten Eigentümer werden neu beobachtet oder abgeleitet), bevor es mit den vorhandenen Datensätzen versiegelt und sagt, wie viele beobachtete Zeilen sich später änderten.",'
    t=once(t,en_old,en_new);t=once(t,de_old,de_new)
    t=once(t,'"ownerFactsMemoEntries": "The most owner admission observations one process keeps to answer a repeated attempt without observing the owner again."\n','"ownerFactsMemoEntries": "The most owner admission observations one process keeps to answer a repeated attempt without observing the owner again.",\n          "sealStalePaths": "The most changed observed rows a sealed read keeps to name the owners that are stale in the next read."\n')
    t=once(t,'"ownerFactsMemoEntries": "Wie viele Zulassungsbeobachtungen von Eigentümern ein Prozess behält, um einen wiederholten Versuch ohne neue Beobachtung des Eigentümers zu beantworten."\n','"ownerFactsMemoEntries": "Wie viele Zulassungsbeobachtungen von Eigentümern ein Prozess behält, um einen wiederholten Versuch ohne neue Beobachtung des Eigentümers zu beantworten.",\n          "sealStalePaths": "Wie viele geänderte beobachtete Zeilen ein versiegeltes Lesen behält, um die Eigentümer zu nennen, die im nächsten Lesen veraltet sind."\n')
    json.loads(t);return t
if __name__=="__main__":
    open(sys.argv[2],"w",encoding="utf8").write(patch(open(sys.argv[1],encoding="utf8").read()))
