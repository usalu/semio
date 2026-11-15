import re
import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip\🔁️recap\🐍️.py")
src = ROOT.read_text(encoding="utf-8")

LISTS = {
"Beat1_Bilanzgrenze": [
 ("intro", "In three videos we calculated single heat flows. Now we assemble them into one balance.",
  "In drei Videos haben wir einzelne Wärmeströme berechnet. Jetzt fügen wir sie zu einer Bilanz zusammen."),
 ("huelle", "A balance needs a boundary: the thermal envelope.", "Eine Bilanz braucht eine Grenze: die thermische Hülle."),
 ("fluesse", "Everything that crosses it over the year is an energy flow we can add up.",
  "Alles, was sie übers Jahr kreuzt, ist ein Energiefluss, den wir aufsummieren."),
 ("erhaltung", "Energy is conserved — so at a constant indoor temperature, everything that flows out must be replaced.",
  "Energie bleibt erhalten: Bei konstanter Innentemperatur muss alles ersetzt werden, was hinausfließt."),
 ("transmission", r"Through walls, roof, windows and floor, heat leaves by transmission, the energy $Q_T$.",
  r"Durch Wände, Dach, Fenster und Boden verlässt Wärme das Gebäude als Transmission, die Energie $Q_T$."),
 ("lueftung", r"With the exhaust air, heat leaves as ventilation loss $Q_V$.",
  r"Mit der Abluft verlässt Wärme das Gebäude als Lüftungsverlust $Q_V$."),
 ("sonne", r"The sun brings heat in for free: the solar gains $Q_S$.", r"Die Sonne bringt kostenlos Wärme herein: die solaren Gewinne $Q_S$."),
 ("intern", r"People, appliances and lighting add internal gains $Q_i$.", r"Personen, Geräte und Beleuchtung liefern interne Gewinne $Q_i$."),
 ("rest", "What the free gains don't cover, the heating has to deliver.", "Was die freien Gewinne nicht decken, muss die Heizung liefern."),
 ("bedarf", r"That remainder is the heating demand $Q_h$ — the useful energy the heating must hand to the room.",
  r"Dieser Rest ist der Heizwärmebedarf $Q_h$ — die Nutzenergie, die die Heizung an den Raum abgeben muss."),
],
"Beat2_Verlustseite": [
 ("intro", "The loss side first — and it is exactly the two mechanisms the Heating and Ventilation videos derived.",
  "Zuerst die Verlustseite — genau die beiden Mechanismen aus den Videos Heizen und Lüftung."),
 ("transmission", r"Transmission: the heat flow $\Phi$ through each surface, as load $\dot{Q}_T$, times time gives the energy $Q_T$.",
  r"Transmission: Der Wärmestrom $\Phi$ durch jede Fläche, als Last $\dot{Q}_T$, mal Zeit ergibt die Energie $Q_T$."),
 ("u", r"The U-value $U$ states how many watts flow through one square metre per kelvin: here $0.24\,\mathrm{W/(m^2K)}$.",
  r"Der U-Wert $U$ gibt an, wie viele Watt je Quadratmeter und Kelvin fließen: hier $0{,}24\,\mathrm{W/(m^2K)}$."),
 ("a", r"The area $A$ tells how large the element is — here the exterior wall with $209\,\mathrm{m^2}$.",
  r"Die Fläche $A$ sagt, wie groß das Bauteil ist — hier die Außenwand mit $209\,\mathrm{m^2}$."),
 ("dt", r"The temperature difference $\Delta\theta = \theta_i - \theta_e$ between inside and outside — here $10\,\mathrm{K}$ in a transition month.",
  r"Die Temperaturdifferenz $\Delta\theta = \theta_i - \theta_e$ zwischen innen und außen — hier $10\,\mathrm{K}$ in einem Übergangsmonat."),
 ("t", r"And $t$ is the time: one month has $744\,\mathrm{h}$.", r"Und $t$ ist die Zeit: Ein Monat hat $744\,\mathrm{h}$."),
 ("wand", r"The exterior wall: $209\,\mathrm{m^2}\cdot 0.24\,\mathrm{W/(m^2K)}\cdot 10\,\mathrm{K}$ is about $500\,\mathrm{W}$.",
  r"Die Außenwand: $209\,\mathrm{m^2}\cdot 0{,}24\,\mathrm{W/(m^2K)}\cdot 10\,\mathrm{K}$ ergeben rund $500\,\mathrm{W}$."),
 ("dach", r"The roof: $89\,\mathrm{m^2}$ at $0.20\,\mathrm{W/(m^2K)}$ — well insulated, so only about $180\,\mathrm{W}$.",
  r"Das Dach: $89\,\mathrm{m^2}$ mit $0{,}20\,\mathrm{W/(m^2K)}$ — gut gedämmt, daher nur etwa $180\,\mathrm{W}$."),
 ("fenster", r"The windows: only $30\,\mathrm{m^2}$, but $1.10\,\mathrm{W/(m^2K)}$ — that is about $330\,\mathrm{W}$.",
  r"Die Fenster: nur $30\,\mathrm{m^2}$, aber $1{,}10\,\mathrm{W/(m^2K)}$ — das sind etwa $330\,\mathrm{W}$."),
 ("boden", r"The floor: $80\,\mathrm{m^2}$ at $0.30\,\mathrm{W/(m^2K)}$, and the ground stays $10\,\mathrm{K}$ below the room all year — $240\,\mathrm{W}$.",
  r"Der Boden: $80\,\mathrm{m^2}$ mit $0{,}30\,\mathrm{W/(m^2K)}$, und das Erdreich bleibt das ganze Jahr $10\,\mathrm{K}$ kälter als der Raum — $240\,\mathrm{W}$."),
 ("bruecken", "Thermal bridges — edges, corners and joints — let additional heat escape.",
  "Wärmebrücken — Kanten, Ecken und Anschlüsse — lassen zusätzlich Wärme entweichen."),
 ("zuschlag", r"We add them as a lump surcharge $\Delta U_{\mathrm{WB}}$ on every U-value.",
  r"Wir erfassen sie pauschal als Zuschlag $\Delta U_{\mathrm{WB}}$ auf jeden U-Wert."),
 ("wb_wert", r"That is $0.05\,\mathrm{W/(m^2K)}$ when a thermal-bridge check is available, more without one.",
  r"Mit Wärmebrückennachweis sind es $0{,}05\,\mathrm{W/(m^2K)}$, ohne Nachweis deutlich mehr."),
 ("lueftung", r"Ventilation, as the Ventilation video built it, carries the second stream out: the load $\dot{Q}_V$.",
  r"Die Lüftung, wie das Lüftungs-Video sie aufgebaut hat, trägt den zweiten Strom hinaus: die Last $\dot{Q}_V$."),
 ("luftwechsel", r"The air change rate $n$ states how often the room air is exchanged per hour — here $0.5\,\mathrm{h^{-1}}$.",
  r"Der Luftwechsel $n$ gibt an, wie oft pro Stunde die Raumluft ausgetauscht wird — hier $0{,}5\,\mathrm{h^{-1}}$."),
 ("volumen", r"The air volume $V$ of the building — $440\,\mathrm{m^3}$ — is what gets exchanged.",
  r"Das Luftvolumen $V$ des Gebäudes — $440\,\mathrm{m^3}$ — wird dabei ausgetauscht."),
 ("waermekap", r"$\rho\,c_p = 0.34\,\mathrm{Wh/(m^3K)}$ is the heat capacity of air per cubic metre; it turns the air flow into heat per kelvin.",
  r"$\rho\,c_p = 0{,}34\,\mathrm{Wh/(m^3K)}$ ist die Wärmekapazität der Luft je Kubikmeter; sie macht aus dem Luftstrom Wärme je Kelvin."),
 ("dichte", r"It belongs to $\rho \approx 1.2\,\mathrm{kg/m^3}$ at $20\,^{\circ}\mathrm{C}$; colder air is denser, $1.27\,\mathrm{kg/m^3}$ at $5\,^{\circ}\mathrm{C}$, but the standard keeps the fixed value.",
  r"Sie gehört zu $\rho \approx 1{,}2\,\mathrm{kg/m^3}$ bei $20\,^{\circ}\mathrm{C}$; kältere Luft ist dichter, $1{,}27\,\mathrm{kg/m^3}$ bei $5\,^{\circ}\mathrm{C}$, die Norm rechnet aber mit dem festen Wert."),
 ("lueft_block", r"With $\Delta\theta$ and $t$ as before, this gives the second block of the losses, $Q_V$.",
  r"Mit $\Delta\theta$ und $t$ wie zuvor ergibt sich der zweite Block der Verluste, $Q_V$."),
 ("senke_teile", r"Transmission $Q_T$ covers four surfaces plus thermal bridges; ventilation $Q_V$ is a block of its own.",
  r"Die Transmission $Q_T$ umfasst vier Flächen und die Wärmebrücken, die Lüftung $Q_V$ bildet einen eigenen Block."),
 ("summe", r"Together they form the loss side of the balance, $Q_l = Q_T + Q_V$ — all the heat the building loses.",
  r"Zusammen bilden sie die Verlustseite der Bilanz, $Q_l = Q_T + Q_V$ — alle Wärme, die das Gebäude verliert."),
],
"Beat3_Gewinnseite": [
 ("intro", "Now the other pan of the scale: the heat the building gets for free.", "Jetzt die andere Waagschale: die Wärme, die das Gebäude kostenlos bekommt."),
 ("solar", r"Solar gains $Q_S$: sunlight enters the building through the glazing.", r"Solare Gewinne $Q_S$: Sonnenstrahlung dringt durch die Verglasung ins Gebäude."),
 ("fenster", r"The window area $A$ — here $30\,\mathrm{m^2}$ — decides how much radiation enters.",
  r"Die Fensterfläche $A$ — hier $30\,\mathrm{m^2}$ — bestimmt, wie viel Strahlung hereinkommt."),
 ("einstrahlung", r"The irradiation $I_s$ on that facade supplies the energy per square metre: here $65\,\mathrm{kWh/m^2}$ in this spring month.",
  r"Die Einstrahlung $I_s$ auf diese Fassade liefert die Energie je Quadratmeter: hier $65\,\mathrm{kWh/m^2}$ in diesem Frühjahrsmonat."),
 ("gwert", r"The total solar energy transmittance $g_\perp$ says which share of the radiation passes the glass — here $0.6$.",
  r"Der Gesamtenergiedurchlassgrad $g_\perp$ sagt, welcher Anteil der Strahlung durch das Glas gelangt — hier $0{,}6$."),
 ("fs", r"The shading factor $F_S$ accounts for overhangs and neighbouring buildings — here $0.7$.",
  r"Der Verschattungsfaktor $F_S$ berücksichtigt Überhänge und Nachbargebäude — hier $0{,}7$."),
 ("weitere", r"Sun protection $F_C$, frame share $F_F$ and oblique incidence $F_W$ we set to $1$ in this example.",
  r"Sonnenschutz $F_C$, Rahmenanteil $F_F$ und schrägen Einfall $F_W$ setzen wir im Beispiel gleich $1$."),
 ("solar_block", r"Multiplied, that is the solar heat gain $Q_S = 819\,\mathrm{kWh}$.", r"Multipliziert ergibt das den solaren Wärmegewinn $Q_S = 819\,\mathrm{kWh}$."),
 ("intern", r"Internal gains $Q_i$: people, appliances and lighting all give off heat.", r"Interne Gewinne $Q_i$: Personen, Geräte und Beleuchtung geben Wärme ab."),
 ("q_i", r"DIN V 4108-6 assumes $q_i = 5\,\mathrm{W/m^2}$ for residential use.", r"DIN V 4108-6 setzt für Wohnnutzung $q_i = 5\,\mathrm{W/m^2}$ an."),
 ("a_n", r"That applies per square metre of useful floor area $A_N$ — here $60\,\mathrm{m^2}$.",
  r"Das gilt je Quadratmeter Nutzfläche $A_N$ — hier $60\,\mathrm{m^2}$."),
 ("t_i", r"Over the $744\,\mathrm{h}$ of the month this adds up to $Q_i = 223\,\mathrm{kWh}$.",
  r"Über die $744\,\mathrm{h}$ des Monats ergibt das $Q_i = 223\,\mathrm{kWh}$."),
 ("eta", "But not every free kilowatt-hour helps.", "Doch nicht jede freie Kilowattstunde hilft."),
 ("eta_h", r"The utilisation factor $\eta$ depends on the ratio $\gamma = Q_g / Q_l$ of gains to losses.",
  r"Der Ausnutzungsgrad $\eta$ hängt vom Verhältnis $\gamma = Q_g / Q_l$ aus Gewinnen und Verlusten ab."),
 ("wert", r"Here it is $\eta \approx 0.80$: that share of the free heat is actually used.",
  r"Hier ist $\eta \approx 0{,}80$: Diesen Anteil der freien Wärme nutzt das Gebäude tatsächlich."),
 ("ueberhitzung", "Gains that arrive when no heat is needed only overheat the room.", "Gewinne, die ohne Wärmebedarf anfallen, überhitzen nur den Raum."),
 ("bilanz", r"The usable gains $\eta \cdot Q_g$ cover part of the losses $Q_l$ — the heating adds the rest.",
  r"Die nutzbaren Gewinne $\eta \cdot Q_g$ decken einen Teil der Verluste $Q_l$ — den Rest liefert die Heizung."),
 ("bilanz_qh", r"That remainder is the heating demand $Q_h = Q_l - \eta \cdot Q_g$ — useful energy at the room.",
  r"Dieser Rest ist der Heizwärmebedarf $Q_h = Q_l - \eta \cdot Q_g$ — Nutzenergie am Raum."),
],
"Beat4_WinterUndSommer": [
 ("intro", "The same balance runs all year — only the two pans change height.", "Dieselbe Bilanz läuft das ganze Jahr — nur die beiden Waagschalen ändern ihre Höhe."),
 ("januar", r"In January it is cold outside, $\Delta\theta = 20\,\mathrm{K}$, and the sun stands low.",
  r"Im Januar ist es draußen kalt, $\Delta\theta = 20\,\mathrm{K}$, und die Sonne steht tief."),
 ("januar_schalen", r"So the loss pan $Q_l$ is high and the gain pan is small — almost all gains are used, $\eta \approx 0.97$.",
  r"Deshalb ist die Verlustschale $Q_l$ hoch und die Gewinnschale klein — fast alle Gewinne werden genutzt, $\eta \approx 0{,}97$."),
 ("januar_luecke", r"The gap between them is the heating demand $Q_h$.", r"Die Lücke dazwischen ist der Heizwärmebedarf $Q_h$."),
 ("juli", r"In July it flips: the sun stands high and the temperature difference $\Delta\theta$ shrinks to $6\,\mathrm{K}$.",
  r"Im Juli kippt es: Die Sonne steht hoch, und die Temperaturdifferenz $\Delta\theta$ schrumpft auf $6\,\mathrm{K}$."),
 ("juli_schalen", r"The losses shrink and count only with $\eta_c \approx 0.61$, while sun and internal loads deliver more heat.",
  r"Die Verluste schrumpfen und zählen nur mit $\eta_c \approx 0{,}61$, während Sonne und interne Lasten mehr Wärme liefern."),
 ("juli_ueberschuss", r"Now the gains exceed the usable losses $\eta_c \cdot Q_l$, and the surplus opens a gap on the other side.",
  r"Jetzt übersteigen die Gewinne die nutzbaren Verluste $\eta_c \cdot Q_l$, und der Überschuss öffnet eine Lücke auf der anderen Seite."),
 ("juli_kuehlen", r"That surplus is the cooling demand $Q_c = Q_g - \eta_c \cdot Q_l$ — heat that has to be removed.",
  r"Dieser Überschuss ist der Kühlbedarf $Q_c = Q_g - \eta_c \cdot Q_l$ — Wärme, die wir abführen müssen."),
 ("beide", "Same building, same physics, opposite sign.", "Gleiches Gebäude, gleiche Physik, umgekehrtes Vorzeichen."),
 ("groessen", r"Heating counted the annual demand $Q_h$, cooling the design load $\dot{Q}_C$ — two quantities, one envelope.",
  r"Heizen zählte den Jahresbedarf $Q_h$, Kühlen die Auslegungslast $\dot{Q}_C$ — zwei Größen, eine Hülle."),
],
}


def render(items):
    out = ["    NARRATION = ["]
    for key, en, de in items:
        out.append(f'        ("{key}",')
        out.append(f'         r"{en}",')
        out.append(f'         r"{de}"),')
    out.append("    ]")
    return "\n".join(out)


for cls, items in LISTS.items():
    i = src.index(f"class {cls}(Scene):")
    a = src.index("    NARRATION = [", i)
    b = src.index("\n    ]", a) + len("\n    ]")
    src = src[:a] + render(items) + src[b:]
ROOT.write_text(src, encoding="utf-8")
print("ok")
