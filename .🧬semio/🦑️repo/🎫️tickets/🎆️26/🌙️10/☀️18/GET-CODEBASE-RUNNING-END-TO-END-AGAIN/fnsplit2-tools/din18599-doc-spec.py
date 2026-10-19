import sys, glob
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib_spec import *
F = glob.glob(ROOT + "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🩺️document-*/🦀️.rs")[0]
c1 = child("📏", "geometry-and-climate-checks", "Geometry and climate checks: the heated volume, the climate table, the net floor area and the element zone linkage of the document evaluation.")
c2 = child("🏠", "envelope-and-heating-checks", "Envelope and heating checks: the GEG transmission loss, the monthly heating demand and the ventilation heat recovery of the document evaluation.")
c3 = child("💡", "lighting-hot-water-and-cooling-checks", "Lighting, hot water and cooling checks: the lighting power, the heating system efficiency, the hot water, the cooling temperature and the cooling demand of the document evaluation.")
c4 = child("🤖", "automation-and-energy-checks", "Automation and energy checks: the building automation, the photovoltaic credit, the primary energy and the tabular method of the document evaluation.")
types = {"report": "CheckReport", "annex": "AnnexChoice", "derived": "BalanceDerived"}
modes = {"report": "mut", "annex": "val"}
spec = {"file": F, "function": "evaluate_document", "types": types, "modes": modes, "keep": [48, 49, 50, 57, 64, 759], "phases": [
    ph("check_heated_volume", "📏 Checks the heated gross volume against the sum of the zone net volumes.", 67, 67, c1),
    ph("check_climate_table", "🌦 Checks the derived climate table handle.", 114, 114, c1, drop_ins=["derived"]),
    ph("check_net_floor_area", "📐 Checks the net floor area against the sum of the zone areas.", 149, 149, c1),
    ph("check_element_zone_linkage", "🔗 Checks that every envelope element links to a declared zone.", 180, 180, c1),
    ph("check_transmission_loss", "🧱 Checks the GEG transmission heat loss coefficient or the mean U value.", 216, 216, c2),
    ph("check_monthly_heating_demand", "🔥 Checks the monthly heating demand against the GEG reference.", 316, 316, c2),
    ph("check_ventilation_heat_recovery", "🌬 Checks the ventilation heat recovery of parts 6 and 7.", 387, 387, c2),
    ph("check_lighting", "💡 Checks the installed lighting power against the profile limit.", 418, 418, c3),
    ph("check_heating_system_efficiency", "⚙ Checks the heating system efficiency against the minimum.", 453, 453, c3),
    ph("check_hot_water", "🚿 Checks the hot water demand and the loss allowances.", 485, 485, c3),
    ph("check_cooling_temperature", "🌡 Checks the cooling set temperature against the zone defaults.", 550, 550, c3),
    ph("check_cooling_demand", "🧊 Checks the cooling demand against the heating demand ratio.", 584, 584, c3),
    ph("check_automation", "🤖 Checks the building automation class.", 628, 628, c4),
    ph("check_primary_energy", "⚡ Checks the primary energy against the reference and the photovoltaic credit.", 656, 656, c4),
    ph("check_tabular_method", "📊 Checks the tabular method where the calculation method is applicable.", 717, 717, c4),
]}
dump(spec, sys.argv[1])
