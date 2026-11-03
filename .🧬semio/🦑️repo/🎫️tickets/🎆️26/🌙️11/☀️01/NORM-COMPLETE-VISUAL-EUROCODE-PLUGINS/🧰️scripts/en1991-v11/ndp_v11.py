import sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp")
from ndplib import *
d = load()
def p(key, clause, symbol, rec, unit, note):
    put(d, "1991", key, clause, symbol, rec, rec, unit, note + " UNVERIFIED: spec line E4.04 marks the value [V]; to be compared with the printed text.", part="4", locator=clause, keep=True)
p("4.discharge-c-h", "5.3", "C_h", 1.15, "", "Discharge factor of the horizontal wall pressure p_he = C_h p_hf (simplified, AAC2 slender silo, flow-controlled).")
p("4.discharge-c-w", "5.3", "C_w", 1.10, "", "Discharge factor of the wall friction traction p_we = C_w p_wf (simplified, AAC2 slender silo).")
save(d); print(len(d))
