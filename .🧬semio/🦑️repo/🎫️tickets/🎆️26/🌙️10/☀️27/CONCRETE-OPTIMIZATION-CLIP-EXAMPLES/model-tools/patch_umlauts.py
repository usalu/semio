from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🔣️.json")
s = p.read_text(encoding="utf-8")
pairs = [("Zuerich", "Zürich"), ("hoehe", "höhe"), ("Hoehe", "Höhe"), ("ueber", "über"), ("Ueber", "Über"), ("Uebergangs", "Übergangs"), ("fuer", "für"), ("Waerme", "Wärme"), ("waerme", "wärme"), ("Huelle", "Hülle"),
         ("Kuehl", "Kühl"), ("kuehl", "kühl"), ("erhaelt", "erhält"), ("Traegheit", "Trägheit"), ("abfuehren", "abführen"), ("Aeusserer", "Äußerer"), ("Daemm", "Dämm"), ("daemm", "dämm"), ("Lueftung", "Lüftung"),
         ("rueckgewinnung", "rückgewinnung"), ("ausserhalb", "außerhalb"), ("Groesst", "Größt"), ("Haelfte", "Hälfte"), ("flaechen", "flächen"), ("flaeche", "fläche"), ("Kaelte", "Kälte"), ("kaelte", "kälte"),
         ("unabhaengig", "unabhängig"), ("Geruest", "Gerüst"), ("Oberflaechen", "Oberflächen"), ("Staerke", "Stärke"), ("staerke", "stärke"), ("Sued", "Süd"), ("Zielgroesse", "Zielgröße"), ("Mittagshöhen", "Mittagshöhen"),
         ("Facade", "Façade"), ("facade", "façade"), ("8 m x 6 m x 3 m", "8 m × 6 m × 3 m"), ("48271 x state", "48271 × state"), ("48271 x Zustand", "48271 × Zustand"), ("Stuetz", "Stütz"),
         ("Staerken", "Stärken"), ("Grad ", "° "), ("Winter: Sued", "Winter: Süd")]
for a, b in pairs:
    s = s.replace(a, b)
p.write_text(s, encoding="utf-8")
