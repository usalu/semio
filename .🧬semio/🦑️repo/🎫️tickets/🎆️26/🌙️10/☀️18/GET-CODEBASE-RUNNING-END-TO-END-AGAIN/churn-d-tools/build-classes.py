"""Builds the refusal class table of the churn schema: for every class the files that carry its messages are discovered with ripgrep (fixed strings, no tests), so the schema names real sites. usage: build-classes.py <schema.json>"""
import json, subprocess, sys

def sites(message):
    out = subprocess.run(["rg", "-l", "-F", "--glob", "!**/node_modules/**", "--glob", "!**/🧪️tests/**", "--glob", "!**/🗑️generated/**", "--glob", "*.ts", "--glob", "*.mjs", "--glob", "*.cjs", "--glob", "*.json", message, "🧰️framework"], capture_output=True, text=True).stdout.split("\n")
    return sorted(line for line in out if line and not line.endswith("refusals.json") and not any(part.startswith("\U0001f9ea") and part.endswith("tests") for part in line.split("/")))

stale = [
    "Stale loaded Source revision reader; restart required", "Stale loaded exact Source reader; restart required", "Stale loaded inventory implementation; restart required",
    "Stale loaded Graph consumer implementation; restart required", "Stale loaded Graph reader policy; restart required", "Stale loaded selected Rust graph assembly; restart required",
    "Stale loaded repository Store implementation; restart required", "Stale loaded Graph closure implementation; restart required", "Stale loaded Rust receipt reader; restart required",
    "Stale loaded Cargo configuration reader; restart required", "Stale loaded Rust descriptor publication controller; restart required", "Stale loaded Rust descriptor reader; restart required",
    "Stale inference inventory reuse implementation; restart required", "Stale loaded observation implementation; restart required",
]
rows = [
    ("source-read", "reobserve", "read", ["concurrentEditV1("], [
        "Source revision descriptor", "Source revision ancestry changed", "Source revision shortened during capture", "Source revision grew beyond admitted EOF",
        "Current physical bytes changed before transfer", "Current directory roster changed", "Cargo Source identity changed during read"],
     "A Source file or directory another process edits is read again when its identity, length or roster moved during the read.",
     "Eine Quelldatei oder ein Verzeichnis, das ein anderer Prozess bearbeitet, wird erneut gelesen, wenn sich Identität, Länge oder Verzeichnisliste während des Lesens änderten."),
    ("source-owner-generation", "reobserve", "snapshot", ["javascriptDependencyGenerationMovedV1("], ["Current shared Source-owner generation changed during consumption", "Current Source-owner execution changed during consumption", "Current Source-owner binding changed during consumption"],
     "The step that consumed a pinned Source-owner generation takes a new snapshot of it and consumes again, because a peer edited an owner meanwhile.",
     "Der Schritt, der eine angeheftete Source-Owner-Generation verbraucht hat, nimmt einen neuen Schnappschuss und verbraucht sie erneut, weil ein Peer inzwischen einen Owner bearbeitete."),
    ("print-source", "reobserve", "snapshot", ["concurrentEditV1("], ["Consumed Print Source membership changed", "Consumed Print Source bytes changed", "Print descriptor identity changed", "Print file changed while reading", "Print file shortened"],
     "A template, bibliography, style or font a print build reads is edited while it is read or compiled: the file is read again, and a document whose consumed Source moved is staged and compiled again as a whole.",
     "Eine Vorlage, ein Literaturverzeichnis, ein Stil oder eine Schrift, die ein Print-Build liest, wird beim Lesen oder Übersetzen bearbeitet: die Datei wird erneut gelesen, und ein Dokument, dessen verbrauchte Quelle sich bewegte, wird als Ganzes neu bereitgestellt und übersetzt."),
    ("dependency-input", "reobserve", "snapshot", ["concurrentEditV1("], ["Original Bun dependency input changed during installation", "Original Bun dependency input appeared during installation"],
     "A manifest or lock a dependency step staged is edited before the step used it: the step stages it again.",
     "Ein Manifest oder eine Lock-Datei, die ein Abhängigkeitsschritt bereitgestellt hat, wird bearbeitet, bevor der Schritt sie nutzt: der Schritt stellt sie erneut bereit."),
    ("cargo-ready-inputs", "reobserve", "snapshot", ["cargoInputsMovedV1("], ["Original Cargo ready inputs changed during consumption", "Original Cargo ready inputs changed before consumption", "Original Cargo fetched registry changed during consumption", "Original Cargo fetched registry changed before consumption"],
     "First-party Cargo manifests edited while a step consumed their proven readiness: the step reads the readiness again and consumes it again.",
     "Eigene Cargo-Manifeste, die bearbeitet wurden, während ein Schritt ihre belegte Bereitschaft verbrauchte: der Schritt liest die Bereitschaft erneut und verbraucht sie erneut."),
    ("bun-ownership", "reobserve", "snapshot", ["edited("], [
        "Bun current manifest changed during verification", "Bun current manifest changed during descriptor admission", "Bun current manifest changed during publication",
        "Bun current ownership policy changed during verification", "Bun current workspace membership changed during verification", "Bun current ownership changed during verification"],
     "A workspace manifest, the ownership policy or the membership edited while the Bun ownership custody was open ends that custody; the owning step opens a new one.",
     "Ein Workspace-Manifest, die Ownership-Policy oder die Mitgliedschaft wurde bearbeitet, während die Bun-Ownership-Verwahrung offen war; der besitzende Schritt öffnet eine neue."),
    ("cargo-workspace-topology", "reobserve", "snapshot", ["concurrentEditV1("], ["Original Cargo workspace topology changed"],
     "A Cargo manifest edited between two observations of one workspace selection is selected again as a whole.",
     "Ein Cargo-Manifest, das zwischen zwei Beobachtungen einer Workspace-Auswahl bearbeitet wurde, wird als Ganzes neu ausgewählt."),
    ("compilation-copy", "reobserve", "snapshot", ["concurrentEditV1("], ["Compilation source changed during descriptor admission", "Compilation copy ancestor changed"],
     "A compiler output replaced while it is retained is observed and copied again.",
     "Eine Compilerausgabe, die während der Aufbewahrung ersetzt wird, wird erneut beobachtet und kopiert."),
    ("retries-exhausted", "reobserve", "build", [], [],
     "A narrower layer used all its attempts on an edit that kept happening: the wider layer, such as the lane that runs the actions, runs the step again, bounded; only an edit that persists through the widest layer fails.",
     "Eine engere Schicht hat alle Versuche für eine Bearbeitung verbraucht, die nicht aufhörte: die weitere Schicht, etwa die Bahn, die die Aktionen ausführt, führt den Schritt erneut aus, begrenzt; nur eine Bearbeitung, die durch die weiteste Schicht anhält, schlägt fehl."),
    ("stale-loaded-implementation", "reexec", None, ["staleImplementationV1(", "BUILD_AUTHORITY_CHANGED"], stale,
     "A reader this process loaded was edited on disk: the outermost command starts again with the current implementation, bounded; a pinned invocation never asks for it.",
     "Ein von diesem Prozess geladener Leser wurde auf der Platte bearbeitet: der äußerste Befehl startet mit der aktuellen Implementierung neu, begrenzt; eine angeheftete Ausführung verlangt das nie."),
    ("escaped-concurrent-edit", "reexec", None, [], [],
     "A typed concurrent edit that left every layer of the process (a failure inside a tracked unit closes its owners, so nothing in the process can run the step again): the outermost command waits 2/8/30 s and starts again with a new snapshot, bounded, EN/DE announced; a command inside another only yields exit 75.",
     "Eine getypte gleichzeitige Bearbeitung, die jede Schicht des Prozesses verlassen hat (ein Fehler in einer verfolgten Einheit schließt deren Besitzer, nichts im Prozess kann den Schritt wiederholen): der äußerste Befehl wartet 2/8/30 s und startet mit einem neuen Schnappschuss erneut, begrenzt, EN/DE gemeldet; ein Befehl in einem anderen meldet nur Exit 75."),
    ("moved-format-stamp", "reexec", None, [], ["This command runs an outdated implementation", "Dieser Befehl führt eine veraltete Implementierung aus"],
     "A declared format stamp moved: the holder yields the shared resource without producing and the outermost command restarts, bounded.",
     "Ein deklarierter Formatstempel wechselte: der Halter gibt die geteilte Ressource ohne Erzeugnis frei und der äußerste Befehl startet neu, begrenzt."),
    ("half-edited-module", "reexec", None, [], [],
     "A module of the implementation failed to parse, resolve or export while a peer was editing it: the outermost command waits and starts again, bounded.",
     "Ein Modul der Implementierung ließ sich nicht parsen, auflösen oder exportieren, während ein Peer es bearbeitete: der äußerste Befehl wartet und startet neu, begrenzt."),
    ("lockfile-frozen", "fatal", None, [], [],
     "A workspace member was added or changed without regenerating its lock file: a real inconsistency, fixed by `bun repo dependencies lock --scope <scope>`, never by retrying.",
     "Ein Workspace-Mitglied wurde hinzugefügt oder geändert, ohne die Lock-Datei zu erzeugen: eine echte Inkonsistenz, behoben mit `bun repo dependencies lock --scope <Bereich>`, nie durch Wiederholen."),
    ("own-output-or-store-custody", "fatal", None, ["Error("], ["Actual compilation destination changed", "Exact current Store Source fence changed"],
     "A private output, image, lease or store this process itself owns changed under it: a real inconsistency of the infrastructure, never a peer edit of a Source.",
     "Eine private Ausgabe, ein Image, eine Lease oder ein Store, den dieser Prozess selbst besitzt, änderte sich unter ihm: eine echte Inkonsistenz der Infrastruktur, nie eine Peer-Bearbeitung einer Quelle."),
]
classes = []
for identifier, disposition, layer, markers, messages, en, de in rows:
    row = {"id": identifier, "disposition": disposition}
    if layer:
        row["layer"] = layer
    row["markers"] = markers
    row["messages"] = [{"text": text, "sites": sites(text)} for text in messages]
    row["describe"] = {"en": en, "de": de}
    classes.append(row)
    for entry in row["messages"]:
        print(("OK  " if entry["sites"] else "MISS"), identifier, "|", entry["text"][:70], "|", len(entry["sites"]))
document = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "https://semio.tech/framework/filesystem/observation/churn/refusals",
    "title": "Refusal classes of concurrent edits",
    "description": "Every refusal a build or run raises when something it reads changed under it, classified once: `reobserve` (a peer edited a Source: the layer that owns the snapshot takes a new one and runs the step again, bounded, EN/DE progress, cancellable), `reexec` (the implementation the process loaded is outdated: the outermost command starts again with the current one, bounded) or `fatal` (a real inconsistency: never retried). `markers` name the constructors a throw site of the class must use; `messages` name the texts and the files that raise them. A fatal class never uses a marker of another class.",
    "definitions": {
        "Disposition": {"enum": ["reobserve", "reexec", "fatal"]},
        "Table": {"const": {"version": 1, "constructors": ["concurrentEditV1(", "concurrentEditWireV1(", "edited(", "staleImplementationV1(", "BUILD_AUTHORITY_CHANGED", "javascriptDependencyGenerationMovedV1(", "cargoInputsMovedV1("], "lookbehind": 480, "classes": classes}},
    },
}
with open(sys.argv[1], "w", encoding="utf-8") as handle:
    json.dump(document, handle, ensure_ascii=False, indent=2)
    handle.write("\n")
