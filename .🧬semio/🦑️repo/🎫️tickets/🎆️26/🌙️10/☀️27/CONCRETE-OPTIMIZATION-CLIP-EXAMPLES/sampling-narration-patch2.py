"""Second narration pass: shorter limits_4 and balanced caption lines."""
from pathlib import Path

path = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir()) / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎲️sampling/🐍️.py"
text = path.read_text(encoding="utf-8")
swaps = [("For this south façade, nine simulations found a good design, not a proven optimum.", "Nine simulations found a good south façade design, no proven optimum."),
         ("Für diese Südfassade fanden neun Simulationen einen guten Entwurf, aber kein bewiesenes Optimum.", "Neun Simulationen fanden für diese Südfassade einen guten Entwurf, kein bewiesenes Optimum."),
         ("This vertical strip is one window-share band and holds exactly one design.", "This vertical strip is one window-share band with exactly one design."),
         ("Jede weitere Stufe vervielfacht die Simulationen: hier zwölfmal unser Budget.", "Jede weitere Stufe vervielfacht die Simulationen: hier das Zwölffache unseres Budgets."),
         ("Leere Box: Keine Ziehung kombiniert großen Fensteranteil mit tiefem Vordach.", "Leere Box: Kein Entwurf mit großem Fensteranteil und tiefem Vordach."),
         ("Jeder beste Entwurf ist der beste von neun, doch keiner erreicht das Optimum.", "Jeder beste Entwurf ist der beste von neun, keiner erreicht das Optimum.")]
for old, new in swaps:
    assert old in text, old
    text = text.replace(old, new)
path.write_text(text, encoding="utf-8")
