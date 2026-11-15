import sys
sys.path.insert(0, ".")
exec(open("author-20-2d.py").read())
d = Doc("🪑️objects", "retractable-bollard")
for sid in ("power-unit", "motor-seam", "motor", "hose-port"):
    d.delete("50", "front", sid)
for sid in ("power-unit", "motor-seam", "motor"):
    d.delete("100", "front", sid)
t = "\n".join(d.lines)
rep = [
 ("and the left-offset power unit with motor seam, end-on motor, hose port, ram and ram rod. Plan view", "the ram and the ram rod; the power unit lies in front of the section plane B-B and is cut away. Plan view"),
 ("und die nach links versetzte Antriebseinheit mit Motornaht, quer liegendem Motor, Schlauchanschluss, Zylinder und Kolbenstange. Draufsicht", "sowie Zylinder und Kolbenstange; die Antriebseinheit liegt vor der Schnittebene B-B und ist weggeschnitten. Draufsicht"),
 ("Front view: LEDs, seals, bolts, motor, hose port, ram rod. Plan view: four bolts", "Front view: LEDs, seals, bolts, ram rod. Plan view: four bolts"),
 ("Vorderansicht: LEDs, Dichtungen, Schrauben, Motor, Schlauchanschluss, Kolbenstange. Draufsicht: vier Schrauben", "Vorderansicht: LEDs, Dichtungen, Schrauben, Kolbenstange. Draufsicht: vier Schrauben"),
 ("Front view: the hose is seen end-on as a port ring. Plan view: ram, hose and LEDs are hidden.", "Front view: power unit, motor and hose lie in front of the section plane B-B and are cut away. Plan view: ram, hose and LEDs are hidden."),
 ("Vorderansicht: der Schlauch erscheint stirnseitig als Anschlussring. Draufsicht: Zylinder, Schlauch und LEDs sind verdeckt.", "Vorderansicht: Antriebseinheit, Motor und Schlauch liegen vor der Schnittebene B-B und sind weggeschnitten. Draufsicht: Zylinder, Schlauch und LEDs sind verdeckt."),
 ("frame blocks at ground level, and the power unit with motor seam and the end-on motor, drawn left of the axis in front of the ram. Plan view", "frame blocks at ground level and the ram; the power unit lies in front of the section plane B-B and is cut away. Plan view"),
 ("Rahmenblöcke in Geländehöhe und die Antriebseinheit mit Motornaht und quer liegendem Motor, links der Achse vor dem Zylinder. Draufsicht", "Rahmenblöcke in Geländehöhe und der Zylinder; die Antriebseinheit liegt vor der Schnittebene B-B und ist weggeschnitten. Draufsicht"),
 ("Front view: reflector, beacon band, cap, motor seam, motor. Plan view: sleeve opening, motor.", "Front view: reflector, beacon band, cap. Plan view: sleeve opening, motor."),
 ("Vorderansicht: Reflexband, Leuchtband, Kappe, Motornaht, Motor. Draufsicht: Hülsenöffnung, Motor.", "Vorderansicht: Reflexband, Leuchtband, Kappe. Draufsicht: Hülsenöffnung, Motor."),
 ("Front view: hose is not drawn (it runs in the viewing direction). Plan view: ram and hose are hidden below the frame.", "Front view: power unit, motor and hose lie in front of the section plane B-B and are cut away. Plan view: ram and hose are hidden below the frame."),
 ("Vorderansicht: der Schlauch ist nicht gezeichnet (er verläuft in Blickrichtung). Draufsicht: Zylinder und Schlauch liegen verdeckt unter dem Rahmen.", "Vorderansicht: Antriebseinheit, Motor und Schlauch liegen vor der Schnittebene B-B und sind weggeschnitten. Draufsicht: Zylinder und Schlauch liegen verdeckt unter dem Rahmen."),
]
for a, b in rep:
    assert t.count(a) == 1, a
    t = t.replace(a, b)
d.lines = t.split("\n")
d.save()
print("ok")
