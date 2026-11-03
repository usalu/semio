import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
def p(key, clause, symbol, value, unit, note, part):
    put(d, "1991", key, clause, symbol, value, value, unit, note, part=part)
B = "Recalled from the standard, not checked against the print; "
p("8.parapet-open-add", "Table 8.1", "d open parapet per side", 0.3, "m", B + "added to the deck depth per side of an open parapet or open safety barrier", "1-4")
p("8.traffic-road-add", "8.3.1", "d road traffic", 2.0, "m", B + "deck depth increment of road traffic", "1-4")
p("8.traffic-rail-add", "8.3.1", "d rail traffic", 4.0, "m", B + "deck depth increment of railway traffic", "1-4")
p("8.cfx0-narrow", "Fig. 8.3", "c_f,x0 at b/d_tot <= 0.5", 2.4, "", B + "digitised upper plateau of the figure", "1-4")
p("8.cfx0-wide", "Fig. 8.3", "c_f,x0 at b/d_tot >= 4", 1.3, "", B + "digitised lower plateau of the figure", "1-4")
p("8.cfx0-b-narrow", "Fig. 8.3", "b/d_tot upper plateau limit", 0.5, "", B + "linear interpolation between the plateaus replaces the curve", "1-4")
p("8.cfx0-b-wide", "Fig. 8.3", "b/d_tot lower plateau limit", 4.0, "", B + "linear interpolation between the plateaus replaces the curve", "1-4")
p("8.cfz", "8.3.3", "c_f,z", 0.9, "", B + "recommended vertical force coefficient without wind tunnel data", "1-4")
p("8.longitudinal", "8.3.4", "F_Wy/F_Wx", 0.25, "", B + "plate girders; 0.5 for trusses is not modelled", "1-4")
p("1-4.cscd-simple-height", "6.2", "h limit c_s c_d = 1", 15.0, "m", B + "buildings below this height need no dynamic procedure", "1-4")
p("1-4.vortex-onset", "E.1.3.1", "v_crit/v_m onset factor", 1.25, "", B + "vortex shedding is investigated when v_crit <= 1.25 v_m", "1-4")
p("1-4.kinematic-viscosity", "E.1.3.1", "nu air", 1.5e-5, "m2/s", B + "kinematic viscosity of air for the Reynolds number", "1-4")
p("1-7.lateral-ratio", "Table 4.1", "F_dy/F_dx", 0.5, "", B + "transverse impact force as share of the driving-direction force", "1-7")
p("1-7.forklift-factor", "4.5", "F_d,x/W forklift", 5.0, "", B + "equivalent static force as multiple of the loaded forklift weight", "1-7")
p("1-7.impact-height-min", "4.3", "impact height min", 0.5, "m", B + "lowest application height on columns and walls", "1-7")
p("1-7.impact-height-max", "4.3", "impact height max", 1.5, "m", B + "highest application height on columns and walls", "1-7")
p("1-7.gas-base", "Annex D", "p_d base", 3.0, "kN/m2", B + "constant part of the internal gas explosion design pressure", "1-7")
p("1-7.gas-vent-coefficient", "Annex D", "p_d vent term", 0.04, "kN/m2", B + "coefficient of the (A_v/V)^-2 term", "1-7")
p("1-7.gas-volume-min", "Annex D", "V min", 5.0, "m3", B + "validity range of the Annex D formulas", "1-7")
p("1-7.gas-volume-max", "Annex D", "V max", 1000.0, "m3", B + "validity range of the Annex D formulas", "1-7")
p("2.lm3-axle-spacing", "Annex A", "LM3 axle line spacing", 1.5, "m", B + "spacing of the axle lines of the standard special vehicles", "2")
p("2.lm3-vehicle-width", "Annex A", "LM3 vehicle width", 3.0, "m", B + "width over the wheels of the standard special vehicles", "2")
p("1-3.sliding-share", "5.3.6", "sliding share", 0.5, "", B + "share of the upper roof load that slides onto the lower roof when the upper pitch exceeds 15 degrees", "1-3")
p("1-5.omega-n", "6.1.5", "omega_N", 0.35, "", B + "combination factor of the uniform temperature range", "1-5")
p("1-5.omega-m", "6.1.5", "omega_M", 0.75, "", B + "combination factor of the linear temperature difference", "1-5")
heat = {1: 18.0, 2: 15.0, 3: 10.0, 4: 15.0, 5: 15.0}
cool = {1: 13.0, 2: 18.0, 3: 5.0, 4: 8.0, 5: 8.0}
names = {1: "steel deck", 2: "composite deck", 3: "concrete box girder", 4: "concrete slab", 5: "concrete beam"}
for t in range(1, 6):
    p(f"1-5.dtm-heat.{t}", "Table 6.1", f"dT_M,heat {names[t]}", heat[t], "K", B + "linear vertical temperature difference, heating", "1-5")
    p(f"1-5.dtm-cool.{t}", "Table 6.1", f"dT_M,cool {names[t]}", cool[t], "K", B + "linear vertical temperature difference, cooling", "1-5")
ks_heat = [0.7, 1.6, 1.0, 0.7, 0.7]
ks_cool = [0.9, 1.1, 1.0, 1.2, 1.2]
for s in range(5):
    p(f"1-5.ksur-heat.{s}", "Table 6.2", f"k_sur heating surfacing {s}", ks_heat[s], "", B + "steel deck surfacing factor (0 none, 1 waterproofed, 2 50 mm, 3 100 mm, 4 150 mm); the ballast row is not registered", "1-5")
    p(f"1-5.ksur-cool.{s}", "Table 6.2", f"k_sur cooling surfacing {s}", ks_cool[s], "", B + "steel deck surfacing factor (0 none, 1 waterproofed, 2 50 mm, 3 100 mm, 4 150 mm); the ballast row is not registered", "1-5")
p("1-5.ksur-other", "Table 6.2", "k_sur other decks", 1.0, "", "Not tabulated in the spec for composite and concrete decks: the base values of Table 6.1 stay unmodified", "1-5")
save(d); print(len(d))
