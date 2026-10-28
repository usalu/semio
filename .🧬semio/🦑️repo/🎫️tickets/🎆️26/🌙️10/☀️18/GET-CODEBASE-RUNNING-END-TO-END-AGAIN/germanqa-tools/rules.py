"""🇩🇪 Rule tables of lane germanqa: ASCII fallbacks, glossary normalisations and grammar fixes for German texts (pure data + one `transform`).
Every rule is context free on the German text alone, so a catalog leaf and the rendered help snapshot that embeds it change identically."""
import re

UMLAUT_STOP = {"zuerst", "soeben", "quelle", "query", "cquery", "request", "requests", "issue", "issues", "true", "value", "unique", "queue"}

TRANSLIT_WORDS = """
Abhaengigkeiten Abstuerzen Aenderung Aenderungen Aendert Aktionsschluessels Anfuehrungszeichen Beruehren Begruendung Bestaenden Befoerderung Befoerderungsregel
Domaenen Groesse Herzschlaegen Huelle Identitaet Importaufloesungen Kapazitaetspunkt Kapazitaetszusagen Konsistenzpruefung Laeufe Laeufen Namensraeume Oeffnet
Pruefsumme Pruefsummen Pruefung Repository-Tuer Schluesselmenge Schnappschuesse Testlaeufer Bazel-Testlaeufer Tuer Urteilslaeufen Veraendert
Vordertuer Woertern aelter aelteste abzueglich aufgeraeumt aufgeloester aufgeloesten aufloesen ausdrueckliche Ausdruecke ausfuehren ausfuehrt ausgefuehrte
behaelt beruehrt beruehren befoerdern befoerdert befoerderten befoerderte duerfen eingehaengt eingehaengte enthaelt erhaelt erklaerten fuehren fuehrt fuehrten
fuenf fuer gegenueber geaendert geaenderter geprueft haelt haengt hinterlaesst hoechstens koennen laedt laesst laeuft muessen naechsten naechste prueft pruefen
regulaere regulaerer schluesselt spaetere stoeren temporaere traegt ueber uebersprungen ueberlebt ueberein ueberschreitet unabhaengigen unabhaengiges unabhaengig
unberuehrt unveraendert urspruenglichen veraendert veroeffentlichen veroeffentlicht waechst waehlt waehrend zaehlt zurueck zurueckgehalten frueheren
Konsistenzpruefung Abhaengigkeit praesentation Praesentation ueberschreitet
""".split()
SS_WORDS = {"ausserhalb": "außerhalb", "Ausserhalb": "Außerhalb", "heisst": "heißt", "massgeblich": "maßgeblich", "Groesse": "Größe", "gross": "groß", "Fussabdruck": "Fußabdruck"}


def _umlaut(word: str) -> str:
    out = re.sub(r"(?<![aeqAEQ])ue", "ü", word)
    out = re.sub(r"(?<![aeqAEQ])Ue", "Ü", out)
    return out.replace("ae", "ä").replace("Ae", "Ä").replace("oe", "ö").replace("Oe", "Ö")


TRANSLIT = {w: _umlaut(w) for w in TRANSLIT_WORDS}
TRANSLIT.update(SS_WORDS)
TRANSLIT.pop("zuerst", None)
import json as _json, os as _os
_MAP = _os.path.join(_os.path.dirname(_os.path.abspath(__file__)), "translit_map.json")
if _os.path.exists(_MAP): TRANSLIT.update(_json.load(open(_MAP, encoding="utf-8")))
_TOKEN = re.compile(r"[A-Za-zÄÖÜäöüß]+(?:-[A-Za-zÄÖÜäöüß]+)*")
_PROTECT = re.compile(r"(`[^`]*`|\{[A-Za-z_][A-Za-z0-9_.]*\}|\$\{[^}]*\}|https?://\S+|[\w./\\-]*[/\\][\w./\\-]*)")


def fix_translit(text: str) -> str:
    parts = _PROTECT.split(text)
    for i in range(0, len(parts), 2):
        def one(m):
            tok = m.group(0)
            if tok in TRANSLIT: return TRANSLIT[tok]
            if "-" in tok:
                return "-".join(TRANSLIT.get(p, p) for p in tok.split("-"))
            return tok
        parts[i] = _TOKEN.sub(one, parts[i])
    return "".join(parts)


R = []
TASK_RULES = []


def rule(pattern, repl, flags=0):
    R.append((re.compile(pattern, flags), repl))


def lit(old, new):
    R.append((re.compile(re.escape(old)), new.replace("\\", "\\\\")))


for old, new in [
    ("Closure-Inventare", "Abschluss-Inventare"), ("Closure-Inventar", "Abschluss-Inventar"), ("Quell-Closures", "Quellabschlüsse"), ("Lock-Closures", "Lock-Abschlüsse"),
    ("Closures liegen jetzt", "Abschlüsse liegen jetzt"), ("Closure-Stelle", "Abschluss-Stelle"),
    ("außerhalb der Closure darf", "außerhalb des Abschlusses darf"), ("außerhalb der erfassten Closure und", "außerhalb des erfassten Abschlusses und"),
    ("Hüllen-Inventaren", "Abschluss-Inventaren"), ("Hüllen-Inventare", "Abschluss-Inventare"),
    ("Build-Graph-Inferenz, Hüllen und Veröffentlichungen", "Build-Graph-Inferenz, Abschlüsse und Veröffentlichungen"),
    ("Zero-Touch-Hüllen…", "Zero-Touch-Abschlüsse…"), ("bei Zero-Touch-Hüllen", "bei Zero-Touch-Abschlüssen"),
    ("Hülle der Historie", "Abschluss der Historie"), ("Selbsttest der Historien-Hülle", "Selbsttest des Historien-Abschlusses"), ("gegen die Historien-Hülle", "gegen den Historien-Abschluss"),
    ("Hüllenverletzung(en)", "Abschlussverletzung(en)"), ("Gibt die Hülle jedes Ziels", "Gibt den Abschluss jedes Ziels"),
    ("die volle Executor-Hülle", "den vollen Executor-Abschluss"), ("Projekte der Hülle geprüft", "Projekte des Abschlusses geprüft"),
    ("Eingabehüllen auflösen", "Eingabeabschlüsse auflösen"), ("Eingabehüllen prüfen", "Eingabeabschlüsse prüfen"),
]: lit(old, new)

for old, new in [
    ("Garbage Collection", "Einsammeln"),
    ("im Cache-Verzeichnis deklarieren", "in der Cache-Bilanz deklarieren"), ("auf diese Bereiche des Verzeichnisses", "auf diese Bereiche der Bilanz"),
    ("dem Größenverzeichnis", "der Größenbilanz"), ("aus dem Größenverzeichnis, das", "aus der Größenbilanz, die"), ("das Größenverzeichnis zu lesen", "die Größenbilanz zu lesen"),
    ("und das Größenverzeichnis liest, das gc und --deep schreiben", "und die Größenbilanz liest, die gc und --deep schreiben"),
]: lit(old, new)

for old, new in [
    ("Der Beleg von Abbild {partition} ist nicht lesbar", "Die Quittung von Image {partition} ist nicht lesbar"),
    ("ist aber kein gültiger Beleg", "ist aber keine gültige Quittung"),
    ("Ein Beleg existiert, die Abbilddatei fehlt", "Eine Quittung existiert, die Image-Datei fehlt"),
    ("Abhängigkeits-Generationsbeleg", "Abhängigkeits-Generationsquittung"), ("Generationsbelegs", "Generationsquittung"),
    ("die der Beleg als fehlend festgehalten hatte", "die die Quittung als fehlend festgehalten hatte"),
    ("Seit dem Beleg haben", "Seit der Quittung haben"), ("Der Beleg wurde mit", "Die Quittung wurde mit"),
    ("entspricht nicht mehr dem Beleg", "entspricht nicht mehr der Quittung"), ("Der Beleg konnte nicht geprüft werden", "Die Quittung konnte nicht geprüft werden"),
    ("Der Bereitschaftsbeleg des Bereichs", "Die Bereitschaftsquittung des Bereichs"),
    ("Erzeuger-Beleg", "Erzeuger-Anspruch"), ("Bazel-Arbeitsbereiche, -Ausgaben, -Aktionen und Belege", "Bazel-Arbeitsbereiche, -Ausgaben, -Aktionen und Quittungen"),
    ("Plattform-Belegregeln", "Plattform-Nachweisregeln"), ("Testbelege", "Testnachweise"),
    ("Belege die Mitgliedschaft nach einer Lücke in den Dateiereignissen neu", "Weise die Mitgliedschaft nach einer Lücke in den Dateiereignissen neu nach"),
    ("ohne gültigen Belegeintrag", "ohne gültigen Quittungseintrag"),
]: lit(old, new)

rule(r"\bangehefteten\b", "gepinnten"); rule(r"\bangeheftete\b", "gepinnte"); rule(r"\bangehefteter\b", "gepinnter"); rule(r"\bangeheftetem\b", "gepinntem")
lit("die Generations-Anheftung des aufrufenden Builds ist keine geschlossene Anheftung", "der Generations-Pin des aufrufenden Builds ist kein geschlossener Pin")
lit("Fest eingebundene Werkzeugketten", "Gepinnte Toolchains"); lit("fest eingebundene Werkzeug-Binärdateien", "gepinnte Werkzeug-Binärdateien"); lit("Fest gehaltene Test-Orakel", "Gepinnte Test-Orakel")
for old, new in [("ausgemusterte Cache-Bytes", "stillgelegte Cache-Bytes"), ("es wird ausgemustert und", "es wird stillgelegt und"), ("einsammeln, ausmustern, Ledger", "einsammeln, stilllegen, Ledger"),
                 ("Das Ausmustern von Dateien", "Das Stilllegen von Dateien"), ("Das Ausmustern des Eingabespeichers", "Das Stilllegen des Eingabespeichers")]: lit(old, new)

for old, new in [
    ("Host-Kapazitätsfreigabe", "Rechner-Kapazitätszusage"), ("Warten auf Host-Kapazität", "Warten auf Rechnerkapazität"), ("Host- und Compiler-Kapazität", "Rechner- und Compiler-Kapazität"),
    ("Dem Host fehlen", "Dem Rechner fehlen"), ("nie der ganze Host exklusiv", "nie der ganze Rechner exklusiv"), ("beim Host werden", "beim Rechner werden"),
    ("Anteil des Host-Speichers", "Anteil des Rechnerspeichers"), ("Höchster Host-Speicher", "Höchster Rechnerspeicher"), ("kind-host: Host-Speicher", "kind-host: Rechnerspeicher"),
    ("auf diesem Host laufen mussten", "auf diesem Rechner laufen mussten"), ("für andere Hosts deklariert", "für andere Rechner deklariert"), ("die Host-Zähler", "die Rechnerzähler"),
    ("einen gesunden Host zu melden", "einen gesunden Rechner zu melden"), ("für diesen Host)", "für diesen Rechner)"), ("Sandbox der Hermetic-Tabelle für den Host:", "Sandbox der Hermetic-Tabelle für den Rechner:"),
    ("Werkzeug-Paket für den Host {host}", "Werkzeug-Paket für den Rechner {host}"), ("Toolchain-Paket für den Host {host}", "Toolchain-Paket für den Rechner {host}"), ("Installationsprogramm für {tool} fehlt für den Host {host}", "Installationsprogramm für {tool} fehlt für den Rechner {host}"),
    ("Das Host-Volume ignoriert", "Das Volume dieses Rechners ignoriert"),
]: lit(old, new)
lit("Host-Speicher", "Rechnerspeicher")
for old, new in [
    ("Maschinen-Budgets", "Rechner-Budgets"), ("Maschinen-Budget-Datei", "Rechner-Budget-Datei"), ("Maschinen-Budget", "Rechner-Budget"), ("Maschinen-Platzbudget", "Rechner-Platzbudget"),
    ("aus der Maschinendatei", "aus der Rechnerdatei"), ("Maschinen-Zulassung", "Rechner-Zulassung"), ("Datenträger-Budget der Maschine", "Datenträger-Budget des Rechners"),
    ("dieser Maschine für diesen Aufruf", "dieses Rechners für diesen Aufruf"), ("je Maschine mit --budget", "je Rechner mit --budget"), ("die ganze Maschine hinter", "den ganzen Rechner hinter"),
    ("dem Maschinenstandard", "dem Rechnerstandard"), ("auf die Maschine wartet", "auf den Rechner wartet"),
]: lit(old, new)

lit("Nutzerin", "Nutzers"); lit("Terminalfenster der Nutzers", "Terminalfenster des Nutzers")
rule(r"\bBenutzer-PATH\b", "Nutzer-PATH"); rule(r"\bBenutzer\b", "Nutzer")
rule(r"\bAgents\b", "Agenten")

rule(r"\bSnapshots\b", "Schnappschüsse"); rule(r"\bSnapshot\b", "Schnappschuss"); rule(r"Snapshot-", "Schnappschuss-"); rule(r"-Snapshots\b", "-Schnappschüsse")
lit("Bazel-Ausgabe-Schnappschuss", "Bazel-Ausgabeschnappschuss")

for old, new in [("Cargo-Bahn", "Cargo-Lane"), ("Verifikationsbahnen", "Verifikations-Lanes"), ("Cache-Bahnen", "Cache-Lanes"), ("Cargo-Unit-Cache-Bahnen", "Cargo-Unit-Cache-Lanes"),
                 ("welche Bahn ein Prozess", "welche Lane ein Prozess"), ("ruhende Bahnen", "ruhende Lanes"), ("Bahn-Dateien", "Lane-Dateien"), ("Bahn-Layout", "Lane-Layout"), ("Ihre Bahnen, Plätze und Sandboxes", "Ihre Lanes, Slots und Sandboxes")]: lit(old, new)

for old, new in [("Sitzungsplätze", "Sitzungs-Slots"), ("Kernplätze", "Kern-Slots"), ("Kapazitätsplatz", "Kapazitäts-Slot"), ("Nx-Daemon-Plätze", "Nx-Daemon-Slots"), ("Rechenplätze", "Rechen-Slots"),
                 ("gezählten Platz", "gezählten Slot"), ("nur Platzdateien liest", "nur Slot-Dateien liest"), ("Platzansprüche", "Slot-Ansprüche")]: lit(old, new)

rule(r"Datenträgerabbilder\b", "Datenträger-Images"); rule(r"Speicherabbilder\b", "Speicher-Images"); rule(r"Scratch-Abbilder\b", "Wegwerf-Images")
rule(r"Implementierungsabbild\b", "Implementierungs-Image"); rule(r"Container-Abbild\b", "Container-Image"); rule(r"Workspace-Abbild\b", "Workspace-Image")
rule(r"Abbilddateien\b", "Image-Dateien"); rule(r"Abbilddatei\b", "Image-Datei")
rule(r"\bAbbild\(er\)", "Image(s)"); rule(r"\bAbbildern\b", "Images"); rule(r"\bAbbilder\b", "Images"); rule(r"\bAbbilds\b", "Images"); rule(r"\bAbbild\b", "Image")
lit("Disk-Images angehängt sind", "Disk-Images eingehängt sind")

lit("Projekt im Projektstamm aus", "Projekt in der Projektwurzel aus"); lit("Befehl im Stamm eines Projekts aus", "Befehl in der Wurzel eines Projekts aus"); lit("im Projektstamm ohne Shell", "in der Projektwurzel ohne Shell")
lit("Hauptbuch", "Hauptbuch")
lit("gemeinsamen Ledger", "gemeinsamen Hauptbuch")

rule(r"\bPolicy-(?=[A-Za-zÄÖÜäöü])", "Richtlinien-")
rule(r"\bPolicy\b(?!\.json)", "Richtlinie")
rule(r"(?<=[A-Za-zäöü])-Policy\b", "-Richtlinie")
rule(r"Namespaces\b", "Namensräume")
for old, new in [
    ("ein älterer Beleg führt", "eine ältere Quittung führt"), ("prüft den Beleg der Abhängigkeits-Generation", "prüft die Quittung der Abhängigkeits-Generation"),
    ("Blick auf einen Beleg", "Blick auf eine Quittung"), ("anderer Beleg oder", "andere Quittung oder"), ("Gemessen am lebenden Beleg", "Gemessen an der lebenden Quittung"),
    ("sein Beleg `<key>.sync.json`", "seine Quittung `<key>.sync.json`"), ("neben den Beleg auch", "neben der Quittung auch"), ("ein gelöschter Beleg oder Zeuge", "eine gelöschte Quittung oder ein gelöschter Zeuge"),
    ("den Beleg gültig lässt", "die Quittung gültig lässt"), ("abgeleitet aus Belegen in ihren Testdateien", "abgeleitet aus Nachweisen in ihren Testdateien"), ("Der Beleg ist statisch", "Der Nachweis ist statisch"),
    ("Sitzungsplatz", "Sitzungs-Slot"),
]: lit(old, new)
for old, new in [
    ("kein Deadlock-Risiko", "kein Verklemmungsrisiko"), ("dass sein Beleg", "dass seine Quittung"), ("(den Beleg:", "(die Quittung:"), ("Schnappschuss, Beleg und Zeiger", "Schnappschuss, Quittung und Zeiger"),
    ("in den Beleg;", "in die Quittung;"), ("mit einem Beleg, der das gepinnte Archiv", "mit einer Quittung, die das gepinnte Archiv"),
    ("einen Task steuern", "eine Task steuern"), ("lässt den Task weiterlaufen", "lässt die Task weiterlaufen"),
]: lit(old, new)
lit("Kapazitätsregelung des Rechners ist ausgeschaltet", "Kapazitätswächter des Rechners ist ausgeschaltet"); lit("schaltet die Steuerung ausdrücklich ab", "schaltet den Kapazitätswächter ausdrücklich ab"); rule(r"Arbeitsprozess", "Worker-Prozess")
lit("Dateiarbeiter", "Datei-Worker"); lit("Dateioperation", "Dateivorgang"); lit("Ihre Version wurde veröffentlicht", "deine Version wurde veröffentlicht"); lit("suche {seconds}s ·", "suche {seconds} s ·"); rule(r"\b[Bb]ereite Adresse\b", "Bereitschaftsadresse"); lit("die bereite Adresse", "die Bereitschaftsadresse")
rule(r"\beines Tasks\b", "einer Task")
lit("Selbst-Deadlock", "Selbst-Verklemmung"); lit("Inkrementell-Verzeichnis", "Inkrement-Verzeichnis"); lit("kein Lease, kein Graph", "keine Lease, kein Graph"); lit("Provenienz-Belege", "Provenienz-Quittungen"); lit("Store-Belege", "Store-Quittungen")
lit("Deadlock-Zyklus", "Verklemmungszyklus"); lit("liegt in keinem Verklemmungszyklus", "liegt in keinem Verklemmungszyklus")
lit("Kompiliereinheiten", "Compile-Einheiten"); lit("Compilereinheiten", "Compiler-Einheiten")
lit("ihr Einheiten-Cache", "ihr Unit-Cache")
lit("Unit-Cache-Collector", "Unit-Cache-Sammler"); lit("Sitzungs-Collector", "Sitzungssammler"); lit("Cache-Müllsammler", "Cache-Sammler"); rule(r"\bEinsammler\b", "Sammler"); rule(r"Einsammler-", "Sammler-")
lit("Dependency-Generation", "Abhängigkeits-Generation")
rule(r"\bClaims\b", "Ansprüche"); lit("Quota-Anteil", "Quotenanteil")
rule(r"\bIds\b", "IDs"); rule(r"\bId\b", "ID")
lit("Cache-Treffer und -Fehlgriff", "Cache-Treffer und -Fehltreffer")
lit("Mikro-Commit", "Micro-Commit")
lit("Der Micro-Commit-Verb", "Das Micro-Commit-Verb")
for old, new in [("Nichts ist gestaged; die gewünschten Änderungen vor prepare stagen", "Nichts ist vorgemerkt; die gewünschten Änderungen vor prepare vormerken"),
                 ("jeden gestagten Bereich", "jeden vorgemerkten Bereich"), ("Die gestageten Pfade", "Die vorgemerkten Pfade")]: lit(old, new)
lit("der Swap unter seiner Obergrenze", "die Auslagerung unter ihrer Obergrenze"); lit("Druck {pressure}, Swap {swap} MB", "Druck {pressure}, Auslagerung {swap} MB")
lit("Die Zuteilung setzt die Parallelität", "Die Zusage setzt die Parallelität"); lit("die Zuteilung eines beendeten Prozesses", "die Zusage eines beendeten Prozesses")
lit("Geltungsbereichsgesetze", "Scope-Gesetze"); lit("MCP-Geltungsbereich", "MCP-Scope"); lit("Geltungsbereiche", "Scopes")
rule(r"\bBesitzern\b", "Eigentümern"); rule(r"\bBesitzer\b", "Eigentümer"); rule(r"Nur-Besitzer-", "Nur-Eigentümer-"); rule(r"Go-Testbesitzer", "Go-Testeigentümer")
rule(r"\bInhabers\b", "Halters"); rule(r"\bInhaber\b", "Halter"); rule(r"^Hüllen$", "Abschlüsse"); rule(r"(?<=\d )Hüllen\b", "Abschlüsse"); rule(r"\bangeheftet\b", "gepinnt"); lit("Maschinendatei", "Rechnerdatei"); lit("auf dieser Maschine", "auf diesem Rechner")
rule(r"\bBahnen\b", "Lanes"); rule(r"\bBahn\b", "Lane"); lit("Plattenbudget", "Datenträgerbudget"); lit("Plattenabschnitt", "Datenträgerabschnitt"); rule(r"(?<!-)\bSwap\b", "Auslagerung")
lit("Pin-Inhaber", "Pin-Halter"); lit("Graph-Inhaber", "Graph-Halter"); lit("Daemon-Inhaber", "Daemon-Halter"); lit("Schreiberinhaber", "Schreiberhalter"); lit("Kapazitätsinhaber", "Kapazitätshalter"); lit("ihres Inhabers", "ihres Halters"); lit("Namensraum- oder Daemon-Halter", "Namensraum- oder Daemon-Halter")
lit("Bauers", "Builders"); lit("dessen Bauer auf Kapazität wartet", "dessen Builder auf Kapazität wartet")
lit("Arbeitsplatz", "Arbeitsfläche"); lit("Artefaktpfade, Scratch und Erfassung", "Artefaktpfade, Arbeitsfläche und Erfassung"); lit("als Scratch-Wurzel", "als Wegwerf-Wurzel")
lit("Checkout-Arbeitsfläche sandboxed Testläufe", "Checkout-Arbeitsfläche sandboxbasierter Testläufe"); lit("ihr Reclaim-Stempel", "ihr Rückgewinnungsstempel")
lit("Ambiente Auflösungsbedingungen", "Umgebungsweite Auflösungsbedingungen")
lit("Der speicherzulassende rustc-Scheduler", "Der rustc-Scheduler mit Speicherzulassung")
lit("Der residente Dienst wurde nicht bereit", "Der residente Dienst ist nicht bereit geworden")
lit("Dem Worktree", "Dem Arbeitsbaum"); lit("Für dieses Worktree läuft kein", "Für diesen Arbeitsbaum läuft kein"); lit("Das Worktree", "Der Arbeitsbaum")
lit("dieses Worktrees", "dieses Arbeitsbaums"); lit("an diesen Worktree gebunden", "an diesen Arbeitsbaum gebunden"); lit("zu diesem Worktree und Schlüssel", "zu diesem Arbeitsbaum und Schlüssel"); lit("zu dem Worktree, den", "zu dem Arbeitsbaum, den"); lit("Worktree", "Arbeitsbaum")
lit("Hängen Sie es ab (hdiutil detach <Gerät>) und führen Sie den Befehl erneut aus", "Hänge es ab (hdiutil detach <Gerät>) und führe den Befehl erneut aus")
rule(r"\bFehlgriff\b", "Fehltreffer")
lit("Lease-Eigentümer-Marke", "Lease-Eigentümer-Marke")

rule(r"\bder Lock sagt\b", "der Lock sagt")
for old, new in [
    ("auf das Lease {subject}", "auf die Lease {subject}"), ("wartet auf ein Lease ({mode})", "wartet auf eine Lease ({mode})"), ("hält ein Lease ({mode})", "hält eine Lease ({mode})"),
    ("weder ein Lease noch", "weder eine Lease noch"), ("auf einem Lease der Klasse", "auf einer Lease der Klasse"), ("kein Lease hängt von ihnen ab", "keine Lease hängt von ihnen ab"),
    ("er nimmt kein Lease,", "er nimmt keine Lease,"), ("hält oder erwartet ein Lease in", "hält oder erwartet eine Lease in"), ("wartet auf ein Lease", "wartet auf eine Lease"),
    ("oder ein Lease hält", "oder eine Lease hält"), ("jedes gehaltene und erwartete Lease", "jede gehaltene und erwartete Lease"), ("nimmt das exklusive Lease", "nimmt die exklusive Lease"), ("(sein Lease", "(seine Lease"),
]: lit(old, new)
for old, new in [("Das Crate {crate} fehlt", "Die Crate {crate} fehlt"), ("Das Crate {crate} lässt", "Die Crate {crate} lässt"), ("es wird für {workspace} online geholt", "sie wird für {workspace} online geholt"), ("dem gesperrten Crate entsprechen", "der gesperrten Crate entsprechen")]: lit(old, new)
lit("jeder cachebare Task wiederhergestellt", "jede cachebare Task wiederhergestellt"); lit("Ein Task läuft nur in einer Sitzung", "Eine Task läuft nur in einer Sitzung")

rule(r"\bActions\b", "Aktionen"); rule(r"\bAction-Ergebnisse\b", "Aktionsergebnisse"); rule(r"\bAction-Ergebnis\b", "Aktionsergebnis"); rule(r"\bAction-Cache\b", "Aktions-Cache")
rule(r"-Actions\b", "-Aktionen"); rule(r"\bAction\b", "Aktion")

for old, new in [
    ("als losgelöste Wartungs-Kindprozess", "als losgelösten Wartungs-Kindprozess"),
    ("Cargo-Inkrementell-Sitzungen", "Cargo-Inkrementsitzungen"), ("Inkrementell-Sitzungen", "Inkrementsitzungen"), ("Inkrementell-Sitzung", "Inkrementsitzung"),
    ("Inkrementell-Satz/Sätze", "Inkrementsatz/-sätze"), ("Inkrementell-Budget", "Inkrement-Budget"),
    ("Plattenplatz", "Datenträgerplatz"), ("Plattenbudgets", "Datenträgerbudgets"), ("auf der Platte", "auf dem Datenträger"), ("Speicher auf Platte", "Speicher auf Datenträger"),
    ("Geburtsprozess", "Birth-Prozess"), ("Der Rejection-Transfer-Vertrag", "Der Ablehnungs-Transfer-Vertrag"),
    ("Die Aufnahme der physischen Compiler-Sperre", "Die Zulassung der physischen Compiler-Sperre"),
    ("die Arbeit kann nicht mehr nutzen", "die Arbeit kann nicht mehr davon nutzen"),
    ("nehmen für den Bericht keine Lease, der statfs", "nehmen keine Lease für den Bericht, der statfs"),
]: lit(old, new)
rule(r"(?<=[\}\d])%(?!\w)", " %")
rule(r"\bMin\.", "min"); rule(r"\bStd\.", "h"); rule(r"(?<=[\d\}]) T\.", " d")

for pat, new in [
    (r"Aufgabengraphen\b", "Task-Graphen"), (r"Aufgabengraph\b", "Task-Graph"), (r"Aufgabendefinition\b", "Task-Definition"), (r"Aufgabenpläne\b", "Task-Pläne"), (r"Aufgabenläufer\b", "Task-Läufer"),
    (r"Aufgabenmanifest\b", "Task-Manifest"), (r"Aufgabenergebnisse\b", "Task-Ergebnisse"), (r"Aufgabeneingabe\b", "Task-Eingabe"),
    (r"Orchestrierungsaufgabe\b", "Orchestrierungs-Task"), (r"Wurzelaufgaben\b", "Wurzel-Tasks"), (r"Testaufgabe\b", "Test-Task"), (r"Build-Aufgabe\b", "Build-Task"), (r"Bazel-Aufgabe\b", "Bazel-Task"),
    (r"Aufgaben\b", "Tasks"), (r"Aufgabe\b", "Task"),
]: TASK_RULES.append((re.compile(pat), new))

lit("Eingabe-Hülle", "Eingabe-Abschluss"); lit("Der Beleg der blockierten Beförderung", "Der Eintrag der blockierten Beförderung"); lit("Die Zeile des Abbildbuchs", "Die Zeile der Schnappschuss-Bilanz")
lit("Hüllen-Stempel", "Abschluss-Stempel"); lit("Hüllen-Block", "Abschluss-Block")
lit("das exklusive Start-Lease", "die exklusive Start-Lease"); lit("das Start-Lease", "die Start-Lease")

TITLE_VERB = re.compile(r"(?<=[a-zäöüß\-]) (Prüfen|Testen|Bauen|Erzeugen|Vorschauen|Installieren|Zurücksetzen|Einrichten|Ausführen|Entfernen|Starten|Auswählen|Anzeigen|Freigeben)$")


def fix_title(text: str) -> str:
    m = TITLE_VERB.search(text)
    return text[:m.start()] + " " + m.group(1)[0].lower() + m.group(1)[1:] if m else text


TASK_EXEMPT = ("🔨️modules/🎛️dashboard/", "💻️client/🛠️developer-tools/", "📋️project.json")


def task_applies(rel: str) -> bool:
    return not any(part in rel for part in TASK_EXEMPT)


_CODE = re.compile(r"(`[^`]*`)")


def _plain(text: str, is_title: bool, task: bool) -> str:
    out = fix_translit(text)
    for rx, repl in R: out = rx.sub(repl, out)
    if task:
        for rx, repl in TASK_RULES: out = rx.sub(repl, out)
        out = re.sub(r"Aufgaben(?=[a-zäöüß])", lambda m: "Task-", out)
        out = re.sub(r"Task-([a-zäöüß])", lambda m: "Task-" + m.group(1).upper(), out)
    return fix_title(out) if is_title else out


def transform(text: str, is_title: bool = False, task: bool = True) -> str:
    parts = _CODE.split(text)
    if len(parts) == 1: return _plain(text, is_title, task)
    return "".join(re.sub(r"<[^>`]*>", lambda m: fix_translit(m.group(0)), p) if i % 2 else _plain(p, is_title and i == len(parts) - 1, task) for i, p in enumerate(parts))
