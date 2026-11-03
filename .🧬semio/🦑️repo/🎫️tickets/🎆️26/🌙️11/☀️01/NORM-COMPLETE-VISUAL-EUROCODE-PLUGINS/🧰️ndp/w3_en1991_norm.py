import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
def p(part, key, clause, symbol, rec, unit, note, line):
    put(d, "1991", key, clause, symbol, rec, rec, unit, note + f" UNVERIFIED: {line}; compare with the printed text before use.", part=part, locator=clause, keep=True)
S19 = "spec line E2.19 (derailment design situations I and II; the spec marks the values [V] and calls the situation I line load lateral)"
p("2", "2.derail-i-line-load", "6.7.1", "Q_A1d", 45000.0, "N/m", "Line load of derailment design situation I.", S19)
p("2", "2.derail-length", "6.7.1", "L derailment", 20.0, "m", "Loaded length of the derailment line load.", S19)
R19 = "EN 1991-2 6.7.1 recalled; the spec line states values for situation I only ('etc.')"
p("2", "2.derail-ii-factor", "6.7.1", "Q_A2d factor", 1.4, "", "Situation II line load as this factor times the LM71 distributed load alpha q_vk.", R19)
p("2", "2.derail-offset-factor", "6.7.1", "offset factor", 1.5, "", "Largest distance of the derailment line load from the track axis as a multiple of the track gauge s.", R19)
p("2", "2.derail-gauge", "6.7.1", "track gauge s", 1.435, "m", "Standard track gauge s used for the offset limit.", R19)
S20 = "spec line E2.17 (rail horizontal actions: traction Q_lak = 33 kN/m L_a,b up to 1000 kN, braking Q_lbk = 20 kN/m L_a,b up to 6000 kN, nosing Q_sk = 100 kN alpha) as the characteristic bases of the Table 6.11 groups (E2.20)"
p("2", "2.rail-traction-line", "6.5.3", "Q_lak per metre", 33000.0, "N/m", "Traction force per metre of influence length.", S20)
p("2", "2.rail-traction-cap", "6.5.3", "Q_lak cap", 1000000.0, "N", "Upper bound of the traction force.", S20)
p("2", "2.rail-braking-line", "6.5.3", "Q_lbk per metre", 20000.0, "N/m", "Braking force per metre of influence length.", S20)
p("2", "2.rail-braking-cap", "6.5.3", "Q_lbk cap", 6000000.0, "N", "Upper bound of the braking force.", S20)
p("2", "2.rail-nosing", "6.5.2", "Q_sk", 100000.0, "N", "Nosing force before the classification factor alpha.", S20)
S12 = "spec line E2.12 (service vehicle Q_sv: two axles of 40 kN, axle spacing 3.0 m, wheel contact 0.20 x 0.20 m; the spec carries question marks)"
p("2", "2.footbridge-service-axle", "5.6.3", "Q_sv axle", 40000.0, "N", "Axle load of the footbridge service vehicle.", S12)
p("2", "2.footbridge-service-spacing", "5.6.3", "Q_sv axle spacing", 3.0, "m", "Axle spacing of the footbridge service vehicle.", S12)
p("2", "2.footbridge-service-wheel", "5.6.3", "Q_sv wheel contact", 0.2, "m", "Square wheel contact side of the footbridge service vehicle.", S12)
D = "EN 14491 / VDI 3673 vent sizing after Bartknecht recalled; the spec line E4.07 states no dust explosion value (EN 1991-4 refers to the dust explosion pressure p_max)"
p("4", "4.dust-vent-c1", "EN 14491", "vent coefficient 1", 3.264e-5, "", "A_v = [c1 p_max K_St p_red^e1 + c2 (p_stat - 0.1) p_red^e2] V^e3 with pressures in bar, K_St in bar m/s, V in m3 and A_v in m2: coefficient c1.", D)
p("4", "4.dust-vent-e1", "EN 14491", "vent exponent 1", -0.569, "", "Exponent e1 of p_red in the K_St term of the vent area.", D)
p("4", "4.dust-vent-c2", "EN 14491", "vent coefficient 2", 0.27, "", "Coefficient c2 of the static opening pressure term of the vent area.", D)
p("4", "4.dust-vent-e2", "EN 14491", "vent exponent 2", -0.5, "", "Exponent e2 of p_red in the static opening pressure term of the vent area.", D)
p("4", "4.dust-vent-e3", "EN 14491", "vent exponent 3", 0.753, "", "Exponent e3 of the volume V in the vent area.", D)
p("4", "4.dust-vent-pstat-offset", "EN 14491", "p_stat offset", 10000.0, "Pa", "Offset 0.1 bar subtracted from the static opening pressure in the vent area.", D)
for key, symbol, value, unit, what in (("kst-min", "K_St lower bound", 10.0, "", "Lower bound of K_St in the validity range of the vent area formula (bar m/s)"), ("kst-max", "K_St upper bound", 300.0, "", "Upper bound of K_St (bar m/s)"), ("pmax-min", "p_max lower bound", 500000.0, "Pa", "Lower bound of the maximum explosion pressure"), ("pmax-max", "p_max upper bound", 1000000.0, "Pa", "Upper bound of the maximum explosion pressure"), ("volume-min", "V lower bound", 0.1, "m3", "Lower bound of the volume"), ("volume-max", "V upper bound", 10000.0, "m3", "Upper bound of the volume"), ("pstat-min", "p_stat lower bound", 10000.0, "Pa", "Lower bound of the static opening pressure"), ("pstat-max", "p_stat upper bound", 100000.0, "Pa", "Upper bound of the static opening pressure"), ("pred-min", "p_red lower bound", 10000.0, "Pa", "Lower bound of the reduced explosion pressure"), ("pred-max", "p_red upper bound", 200000.0, "Pa", "Upper bound of the reduced explosion pressure")):
    p("4", f"4.dust-{key}", "EN 14491", symbol, value, unit, what + " in the validity range of the vent area formula.", D)
save(d); print(len(d))
