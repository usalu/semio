import json, sys

path, out = sys.argv[1], sys.argv[2]
text = open(path, encoding="utf-8").read()
schema = json.loads(text)
defs = schema["$defs"]
const = defs["Protocol"]["const"]

stamp = "semio.dependency-generation-walk/1"
if "implementation" in const:
    position = list(const).index("implementation")
    items = list(const.items())
    items[position] = ("walkStamp", stamp)
    const.clear()
    const.update(items)

classes = [c if c != "implementation-changed" else "protocol-changed" for c in const["mismatchClasses"]]
const["mismatchClasses"] = classes
defs["MismatchClass"]["enum"] = list(classes)

tiers = const["tiers"]
tiers[0] = {
    "name": "protocol",
    "cost": "O(1)",
    "covers": "the declared walk protocol stamp of this schema; never the bytes, paths or timestamps of the code that performs the walk, the installation or the verification",
}

en = {
    "stale": "The dependency generation receipt of scope {scope} is stale: {reason}",
    "absent": "No dependency generation receipt exists for scope {path}.",
    "receipt-invalid": "The dependency generation receipt {path} cannot be read or is not sealed correctly.",
    "evidence-invalid": "A sealed evidence file of the dependency generation receipt is missing or damaged: {detail}.",
    "protocol-changed": "The dependency generation protocol changed: the receipt was sealed with {detail}, this tool declares {expected}.",
    "runtime-changed": "The tool runtime changed ({detail}).",
    "runtime-changed.bun": "The Bun version changed.",
    "runtime-changed.revision": "The Bun revision changed.",
    "runtime-changed.node": "The Node version changed.",
    "runtime-changed.platform": "The platform changed.",
    "runtime-changed.arch": "The CPU architecture changed.",
    "runtime-changed.sha256": "The installer recipe or resolution profile declared in the schema changed.",
    "configuration-changed": "The installer configuration changed: {path}.",
    "conditions-open": "Ambient resolution conditions are set (NODE_OPTIONS, BUN_OPTIONS, --conditions or --preserve-symlinks).",
    "binding-changed": "The node_modules links of scope {path} changed: {detail}.",
    "ready-changed": "The ready receipt of scope {path} changed.",
    "input-changed": "An installer input changed: {path}.",
    "input-changed.bytes": "An installer input changed its content: {path}.",
    "input-changed.identity": "An installer input was replaced, moved or removed: {path}.",
    "input-changed.appeared": "An installer input appeared that the receipt recorded as missing: {path}.",
    "manifest-changed": "A workspace manifest changed: {path}.",
    "manifest-changed.bytes": "A workspace manifest changed its content: {path}.",
    "manifest-changed.unreadable": "A workspace manifest is no longer readable: {path}.",
    "membership-changed": "The workspace layout changed at {path}.",
    "membership-changed.state": "The workspace layout changed at {path}: an entry was added, replaced or removed.",
    "membership-changed.directory": "A directory of the workspace layout disappeared: {path}.",
    "membership-changed.listing": "The workspace layout changed in {path}: a package, link or directory was added or removed.",
    "membership-churn": "More than {limit} directories changed since the receipt, the first at {path}.",
    "provider-changed": "A provider observation changed: {path}.",
    "provider-changed.identity": "A provider file or directory changed: {path}.",
    "provider-changed.target": "A provider link now points elsewhere: {path}.",
    "unreadable": "The receipt could not be checked: {detail}.",
    "sha-changed": "The combined read of the scope no longer equals the receipt.",
    "seal": "The replayed walk no longer equals the completed walk.",
}
de = {
    "stale": "Der Abhängigkeits-Generationsbeleg des Bereichs {scope} ist veraltet: {reason}",
    "absent": "Für den Bereich {path} existiert kein Abhängigkeits-Generationsbeleg.",
    "receipt-invalid": "Der Abhängigkeits-Generationsbeleg {path} ist nicht lesbar oder nicht korrekt versiegelt.",
    "evidence-invalid": "Eine versiegelte Nachweisdatei des Abhängigkeits-Generationsbelegs fehlt oder ist beschädigt: {detail}.",
    "protocol-changed": "Das Protokoll der Abhängigkeitsgeneration hat sich geändert: Der Beleg wurde mit {detail} versiegelt, dieses Werkzeug deklariert {expected}.",
    "runtime-changed": "Die Werkzeug-Laufzeit hat sich geändert ({detail}).",
    "runtime-changed.bun": "Die Bun-Version hat sich geändert.",
    "runtime-changed.revision": "Die Bun-Revision hat sich geändert.",
    "runtime-changed.node": "Die Node-Version hat sich geändert.",
    "runtime-changed.platform": "Die Plattform hat sich geändert.",
    "runtime-changed.arch": "Die CPU-Architektur hat sich geändert.",
    "runtime-changed.sha256": "Das im Schema deklarierte Installationsrezept oder Auflösungsprofil hat sich geändert.",
    "configuration-changed": "Die Installer-Konfiguration hat sich geändert: {path}.",
    "conditions-open": "Ambiente Auflösungsbedingungen sind gesetzt (NODE_OPTIONS, BUN_OPTIONS, --conditions oder --preserve-symlinks).",
    "binding-changed": "Die node_modules-Verknüpfungen des Bereichs {path} haben sich geändert: {detail}.",
    "ready-changed": "Der Bereitschaftsbeleg des Bereichs {path} hat sich geändert.",
    "input-changed": "Eine Installer-Eingabe hat sich geändert: {path}.",
    "input-changed.bytes": "Eine Installer-Eingabe hat ihren Inhalt geändert: {path}.",
    "input-changed.identity": "Eine Installer-Eingabe wurde ersetzt, verschoben oder entfernt: {path}.",
    "input-changed.appeared": "Eine Installer-Eingabe ist aufgetaucht, die der Beleg als fehlend festgehalten hatte: {path}.",
    "manifest-changed": "Ein Workspace-Manifest hat sich geändert: {path}.",
    "manifest-changed.bytes": "Ein Workspace-Manifest hat seinen Inhalt geändert: {path}.",
    "manifest-changed.unreadable": "Ein Workspace-Manifest ist nicht mehr lesbar: {path}.",
    "membership-changed": "Die Workspace-Struktur hat sich bei {path} geändert.",
    "membership-changed.state": "Die Workspace-Struktur hat sich bei {path} geändert: Ein Eintrag wurde hinzugefügt, ersetzt oder entfernt.",
    "membership-changed.directory": "Ein Verzeichnis der Workspace-Struktur ist verschwunden: {path}.",
    "membership-changed.listing": "Die Workspace-Struktur hat sich in {path} geändert: Ein Paket, eine Verknüpfung oder ein Verzeichnis wurde hinzugefügt oder entfernt.",
    "membership-churn": "Seit dem Beleg haben sich mehr als {limit} Verzeichnisse geändert, das erste bei {path}.",
    "provider-changed": "Eine Anbieter-Beobachtung hat sich geändert: {path}.",
    "provider-changed.identity": "Eine Anbieter-Datei oder ein Anbieter-Verzeichnis hat sich geändert: {path}.",
    "provider-changed.target": "Eine Anbieter-Verknüpfung zeigt jetzt woandershin: {path}.",
    "unreadable": "Der Beleg konnte nicht geprüft werden: {detail}.",
    "sha-changed": "Das kombinierte Lesen des Bereichs entspricht nicht mehr dem Beleg.",
    "seal": "Der wiederholte Durchlauf entspricht nicht mehr dem abgeschlossenen Durchlauf.",
}
const["languages"] = ["en", "de"]
const["texts"] = {"en": en, "de": de}

defs["Evidence"]["properties"]["name"]["pattern"] = "^[a-f0-9]{64}\\.(frontier|execution)\\.ndjson$"
defs.pop("Implementation", None)
defs.pop("ClosureRows", None)

receipt = defs["Receipt"]
required = receipt["required"]
if "implementation" in required:
    required[required.index("implementation")] = "walkStamp"
props = receipt["properties"]
if "implementation" in props:
    items = [("walkStamp", {"type": "string", "minLength": 1, "maxLength": 128}) if key == "implementation" else (key, value) for key, value in props.items()]
    props.clear()
    props.update(items)

schema["description"] = "Schema-first contract of the content-addressed receipt that lets ordinary commands prove an unchanged dependency generation by stat and bounded reads, falling back to the complete ownership and provider walk on any mismatch. The receipt is keyed only by what determines the generation (lockfiles, manifests, installer configuration, pinned runtime, platform, topology, provider observations) and by the declared walk protocol stamp, never by the bytes of the code that performs the install or the verification."

open(out, "w", encoding="utf-8").write(json.dumps(schema, ensure_ascii=False, indent=1) + "\n")
