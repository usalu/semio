import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "groups"))
from _gen import emit, live
HERE = os.path.dirname(os.path.abspath(__file__))
R = "🧰️framework/🛍️products/🦑️repo/README.md"
text = live(R)
en_old = "(`createNativeDirectoryMeterV1`; `meter` in the directory schema)."
de_old = "(`createNativeDirectoryMeterV1`; `meter` im Verzeichnis-Schema)."
assert text.count(en_old) == 1 and text.count(de_old) == 1
en_new = en_old + " Measurements share one asynchronous, cancellable walk per partition (at most two walks however many commits arrive at once, none blocks the event loop), a vanished entry counts as freed space, and the entry bound follows the envelope instead of a constant (`usage` in the directory schema; the dependency image already holds about 950,000 entries). Space queries below a partition root go through `nativeStatfsV1` and `nativeStatfsSyncV1`, which answer the declared envelope minus the shared measured usage instead of the host disk that merely stores the directories; Windows host paths read the free space of their volume without `statfs`."
de_new = de_old + " Messungen teilen sich einen asynchronen, abbrechbaren Durchlauf je Partition (höchstens zwei Durchläufe, wie viele Commits auch gleichzeitig eintreffen, keiner blockiert die Ereignisschleife), ein verschwundener Eintrag zählt als freigegebener Platz, und die Eintragsgrenze folgt dem Umfang statt einer Konstanten (`usage` im Verzeichnis-Schema; das Abhängigkeits-Image hält bereits rund 950.000 Einträge). Platzabfragen unterhalb einer Partitionswurzel laufen über `nativeStatfsV1` und `nativeStatfsSyncV1`, die den deklarierten Umfang abzüglich der geteilten gemessenen Belegung liefern statt der Host-Platte, die die Verzeichnisse nur speichert; Windows-Hostpfade lesen den freien Platz ihres Volumes ohne `statfs`."
extra = [
 ("The adapter never names a sandbox strategy.", "The adapter never names a host-specific sandbox strategy: hermetic actions list `sandboxed`, which Bazel resolves to the best sandbox the host registers (`darwin-sandbox`, `linux-sandbox`, `processwrapper-sandbox`); naming `linux-sandbox` itself is refused by name in an unprivileged container."),
 ("so the adapter never names a sandbox strategy.", "so the adapter names no host-specific sandbox strategy (hermetic actions list the alias `sandboxed`; a platform-specific name that is not registered on the host is refused, as the macOS binary showed for `linux-sandbox`)."),
 ("Der Adapter nennt nie eine Sandbox-Strategie.", "Der Adapter nennt nie eine hostspezifische Sandbox-Strategie: Hermetische Aktionen führen `sandboxed` auf, das Bazel auf die beste auf dem Host registrierte Sandbox abbildet (`darwin-sandbox`, `linux-sandbox`, `processwrapper-sandbox`); `linux-sandbox` selbst wird in einem unprivilegierten Container beim Namen abgelehnt."),
]
for old, new in extra:
    assert text.count(old) == 1, old
emit("g13", "README: directory partition capacity measurement, the virtual statfs and the portable hermetic strategy names, English and German", [], [(R, en_old, en_new), (R, de_old, de_new)] + [(R, old, new) for old, new in extra], [], os.path.join(HERE, "groups"))
print("g13 written")
