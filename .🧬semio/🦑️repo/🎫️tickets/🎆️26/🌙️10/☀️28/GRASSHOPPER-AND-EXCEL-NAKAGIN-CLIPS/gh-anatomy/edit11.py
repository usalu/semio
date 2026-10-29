import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)
rep("Jede Komponente hat einen Namen und einen Kurznamen, der Canvas zeigt den Kurznamen. Eingänge liegen links, Ausgänge rechts, je mit einem Griff.", "Jede Komponente hat Namen und Kurznamen; der Canvas zeigt den Kurznamen. Eingänge liegen links, Ausgänge rechts, je mit einem Griff.")
rep("Components show states. Grey is fine. Give Z one number and one text and it turns orange, a warning: one box is built, the second slot stays null.", "Components show states. Grey is fine. Give Z a number and a text: it turns orange, a warning. One box is built, the second slot stays null.")
open(p, "w", encoding="utf-8").write(s)
