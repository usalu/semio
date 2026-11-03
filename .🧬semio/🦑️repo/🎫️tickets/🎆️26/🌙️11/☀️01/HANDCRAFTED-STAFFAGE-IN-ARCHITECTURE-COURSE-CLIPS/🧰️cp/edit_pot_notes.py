"""🍲 Notes of the pot: replace the illustrative 38 W sentence by the consistent budget."""
from pathlib import Path

F = Path(__file__).resolve().parents[8] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/cooking-pot/🔣️.json"
s = F.read_bytes().decode("utf-8")
pairs = [
    (
        "Heating 1 kg of water by 80 K takes 335 kJ and evaporating it 2257 kJ (Cengel). The net heating of the water surface node (38 W) is an illustrative figure chosen so that the surface reaches the boiling point at steady state; the burner power itself is carried by the hob asset.",
        "Heating 1 kg of water by 80 K takes 335 kJ and evaporating it 2257 kJ (Cengel). The 2 kg of water need about 660 kJ for 79 K: the hob delivers 1.2 kW into the pot (gas-hob), so the open pot boils after about 11 minutes, including the evaporation losses of the warm surface.",
    ),
    (
        "Die Erwärmung von 1 kg Wasser um 80 K braucht 335 kJ, das Verdampfen 2257 kJ (Cengel). Die Nettoerwärmung des Wasseroberflächenknotens (38 W) ist ein Anschauungswert, der so gewählt ist, dass die Oberfläche im stationären Zustand den Siedepunkt erreicht; die Brennerleistung selbst trägt das Kochfeld-Asset.",
        "Die Erwärmung von 1 kg Wasser um 80 K braucht 335 kJ, das Verdampfen 2257 kJ (Cengel). Die 2 kg Wasser brauchen für 79 K rund 660 kJ: das Kochfeld liefert 1,2 kW in den Topf (gas-hob), sodass der offene Topf nach etwa 11 Minuten kocht, einschließlich der Verdunstungsverluste der warmen Oberfläche.",
    ),
]
for old, new in pairs:
    assert s.count(old) == 1, old[:40]
    s = s.replace(old, new)
F.write_bytes(s.encode("utf-8"))
