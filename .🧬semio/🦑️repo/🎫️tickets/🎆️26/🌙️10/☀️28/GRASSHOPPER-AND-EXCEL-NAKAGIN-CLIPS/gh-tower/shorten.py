"""✂️ Shorten three German tower clauses to two caption lines in the scene and in the German transcript."""
import re
from pathlib import Path

ROOT = Path("C:/git/semio/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip")
NEW = {
    "Orient bildet den lokalen": "Orient bildet den lokalen Tür-Port auf den rekonstruierten Welt-Port ab. Das Ergebnis ist die Körper-Ebene der Kapsel, gerichtet angeordnet.",
    "Ohne die Geschossmasken": "Ohne die Geschossmasken enthält das regelmäßige Raster unbelegte Positionen. Das Muster entspricht nicht dem dokumentierten Gebäude.",
    "Die Hülle ist der erste": "Die Hülle ist der erste Konstruktionsschritt. Benannte Baugruppen tragen Erschließung, Versorgung und Fassadendetails durchs Gebäude.",
}
scene = next((ROOT / "🗼️tower").glob("*.py"))
transcript = ROOT / "🗣️transcript.de.md"
for path, pattern in ((scene, r'^(\s+)"({start}[^"]*)"(\],?)$'), (transcript, r"^({start}.*)$")):
    text = path.read_bytes().decode("utf-8")
    for start, replacement in NEW.items():
        expression = re.compile(pattern.replace("{start}", re.escape(start)), re.M)
        if path == scene:
            text, count = expression.subn(lambda m: f'{m.group(1)}"{replacement}"{m.group(3)}', text)
        else:
            text, count = expression.subn(lambda m: replacement, text)
        print(path.name, start, count)
    with path.open("r+b") as handle:
        data = text.encode("utf-8")
        handle.seek(0)
        handle.write(data)
        handle.truncate()
