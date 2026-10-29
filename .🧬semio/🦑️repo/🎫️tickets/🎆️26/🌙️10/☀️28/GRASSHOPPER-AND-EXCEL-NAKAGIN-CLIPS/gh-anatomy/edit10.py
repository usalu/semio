import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)
rep("Komponenten zeigen Zustände. Grau ist in Ordnung. Bekommt Z eine Zahl und einen Text, wird sie orange, eine Warnung: Eine Box entsteht, der zweite Platz bleibt null.", "Grau ist in Ordnung. Bekommt Z eine Zahl und einen Text, wird sie orange, eine Warnung: Eine Box entsteht, der zweite Platz bleibt null.")
rep("Kommt nichts Brauchbares an, wird sie rot, ein Fehler: Eine Verbindung macht aus dem Text tall keine Höhe. Man behebt das erste Problem weiter vorn.", "Ohne Brauchbares wird sie rot, ein Fehler: Aus dem Text tall macht keine Verbindung eine Höhe. Das erste Problem zuerst beheben.")
rep("Eine Series mit der Schrittweite 2,75 Meter erzeugt eine Liste: neun Höhen in einem Ast {0}, Indizes 0 bis 8, je eine Kapsel auf neun Geschossen.", "Eine Series mit Schrittweite 2,75 Meter erzeugt eine Liste: neun Höhen in einem Ast {0}, Indizes 0 bis 8, je eine Kapsel pro Geschoss.")
rep("Ein typischer Fehler ist, einen Datenbaum wie eine lange Liste zu lesen. Der Index 1 allein passt zu allen 27 Ästen von Turm A und nennt 27 Kapseln.", "Typischer Fehler: einen Datenbaum wie eine lange Liste lesen. Der Index 1 allein passt zu allen 27 Ästen von Turm A: 27 Kapseln.")
rep("Plugin. Die englischen Wörter stehen daneben.", "Plugin. Englisch steht daneben.")
rep("Und Plugin. Die", "Und Plugin. Die") if False else None
open(p, "w", encoding="utf-8").write(s)
