import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from stagelib import *
g=Group("G")
D=L[:L.rindex("/")]+"/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/"
lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
p=D+"🔣️.json"
def row(owner,ref,en,de): return f'''  {{
   "owner": "{lib}{owner}",
   "reference": "{ref}",
   "contract": {{
    "en": "{en}",
    "de": "{de}"
   }}
  }}'''
a=row("🏗️build/🏛️sealed/🚦️trampoline/🟦️.ts","../🚪️entry/🟦️.ts","The sealed trampoline only computes the path of the sealed entry and starts the promoted snapshot of it as a child process; it never imports the entry, and the entry is the repo command line, which never runs an Nx task, so the entry and everything only it reaches cannot change the bytes of a task.","Das versiegelte Sprungbrett berechnet nur den Pfad des versiegelten Einstiegs und startet dessen beförderten Schnappschuss als Kindprozess; es importiert den Einstieg nie, und der Einstieg ist die Repo-Befehlszeile, die nie eine Nx-Task ausführt, daher können der Einstieg und alles, was nur er erreicht, die Bytes einer Task nicht ändern.")
b=row("🏗️build/🧭️invocation/🔄️reload/🟦️.ts","../../🏛️sealed/🚪️entry/🟦️.ts","The reload of an invocation only computes the path of the sealed entry to find the sealed store that holds its snapshot and reads its pointer; it never imports the entry, and the entry never runs an Nx task, so the entry and everything only it reaches cannot change the bytes of a task.","Das Neuladen einer Aufrufs berechnet nur den Pfad des versiegelten Einstiegs, um den versiegelten Speicher mit seinem Schnappschuss zu finden und dessen Zeiger zu lesen; es importiert den Einstieg nie, und der Einstieg führt nie eine Nx-Task aus, daher können der Einstieg und alles, was nur er erreicht, die Bytes einer Task nicht ändern.")
a=a.replace("Das Neuladen einer Aufrufs","Das Neuladen eines Aufrufs");b=b.replace("Das Neuladen einer Aufrufs","Das Neuladen eines Aufrufs")
t=g.text(p)
i=t.index('"domains": [')
j=t.rindex("  }\n ],\n",0,i)+len("  }")
g.files[p][0]=t[:j]+",\n"+a+",\n"+b+t[j:]
g.save()
