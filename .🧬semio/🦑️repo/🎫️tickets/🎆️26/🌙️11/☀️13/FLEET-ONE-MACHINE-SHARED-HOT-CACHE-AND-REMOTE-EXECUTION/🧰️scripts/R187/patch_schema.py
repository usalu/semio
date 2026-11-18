import json,sys,re
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R187"
t=open(S+"/base/ds.json").read()
def once(a,b):
    global t
    assert t.count(a)==1,(a,t.count(a));t=t.replace(a,b)
once('          "renameClass": 10,\n','          "renameClass": 10,\n          "renameExClass": 65,\n')
once('        "shareWrite": 2,\n','        "shareWrite": 2,\n        "shareDelete": 4,\n')
once('          "3221225927": "STATUS_PATH_NOT_COVERED"\n','          "3221225927": "STATUS_PATH_NOT_COVERED",\n          "3221225475": "STATUS_INVALID_INFO_CLASS",\n          "3221225485": "STATUS_INVALID_PARAMETER",\n          "3221225474": "STATUS_NOT_IMPLEMENTED",\n          "3221225761": "STATUS_CANNOT_DELETE",\n          "3221226051": "STATUS_USER_MAPPED_FILE"\n')
words={
 "3221225506":{"en":"access denied: a program outside this command keeps a file or folder below the source open without allowing the move, or a read-only mark or missing rights forbid it","de":"Zugriff verweigert: ein Programm ausserhalb dieses Befehls hält eine Datei oder einen Ordner unterhalb der Quelle offen und erlaubt das Verschieben nicht, oder eine Schreibschutz-Markierung oder fehlende Rechte verbieten es"},
 "3221225539":{"en":"sharing violation: a program keeps the source or a file below it open without allowing the move","de":"Freigabeverletzung: ein Programm hält die Quelle oder eine Datei darunter offen und erlaubt das Verschieben nicht"},
 "3221225558":{"en":"delete pending: the source is already being deleted and a program still holds it open","de":"Löschen ausstehend: die Quelle wird bereits gelöscht und ein Programm hält sie noch offen"},
 "3221225761":{"en":"cannot delete: a program keeps a file below the source open or mapped","de":"Löschen nicht möglich: ein Programm hält eine Datei unterhalb der Quelle offen oder eingeblendet"},
 "3221226051":{"en":"a file below the source is mapped into the memory of a running program","de":"eine Datei unterhalb der Quelle ist in den Speicher eines laufenden Programms eingeblendet"}}
const={
 "version":1,
 "methods":["posix-ex","classic"],
 "flags":{"posixSemantics":2,"ignoreReadonlyAttribute":64},
 "fallbackStatuses":[3221225475,3221225659,3221225485,3221225474],
 "transientStatuses":[3221225506,3221225539,3221225558,3221225761,3221226051],
 "initialMilliseconds":50,"growth":2,"capMilliseconds":2000,"budgetMilliseconds":60000,
 "reportEveryMilliseconds":5000,"holderClass":47,"maximumHolders":8,"maximumHolderUnits":120,
 "statusWords":words,
 "texts":{
  "en":{
   "foreign":"The folder {source} could not be moved to {target}: Windows answered {status} ({statusName}: {words}) on each of {attempts} attempts over {waited} ms. Holders: {holders}. Close the program that has it open (Resource Monitor, tab CPU, Associated Handles, search for the folder name) or let the antivirus scan finish; the next run tries again.",
   "own":"The folder {source} could not be moved to {target}: this command itself still holds {count} open handle(s) at or below it. It must close them before the move; this is a defect of the command, not of the machine.",
   "unknown":"not determined (Windows does not name the holder of a folder tree)"},
  "de":{
   "foreign":"Der Ordner {source} konnte nicht nach {target} verschoben werden: Windows antwortete {status} ({statusName}: {words}) bei jedem von {attempts} Versuchen über {waited} ms. Halter: {holders}. Das Programm schliessen, das ihn offen hält (Ressourcenmonitor, Reiter CPU, Zugeordnete Handles, nach dem Ordnernamen suchen) oder den Virenscan abwarten; der nächste Lauf versucht es erneut.",
   "own":"Der Ordner {source} konnte nicht nach {target} verschoben werden: dieser Befehl selbst hält noch {count} offene(s) Handle(s) auf oder unterhalb davon. Er muss sie vor dem Verschieben schliessen; das ist ein Fehler des Befehls, nicht des Rechners.",
   "unknown":"nicht ermittelt (Windows nennt den Halter eines Ordnerbaums nicht)"}}}
block='    "RelativeRenameSettlement": {\n      "const": '+json.dumps(const,ensure_ascii=False,indent=2).replace("\n","\n      ")+'\n    },\n'
once('    "RelativeWindowsFailureNames": {','    "RelativeWindowsFailureNames": {') 
i=t.index('    "RelativeWindowsFailureNames": {')
t=t[:i]+block+t[i:]
json.loads(t)
open(S+"/stage/ds.json","w").write(t)
print("ok",len(t))
