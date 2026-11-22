"""🌀 Vacuum cleaner: the suction motor (bypass motor-fan unit) and the dust bag as real parts inside the canister of the thermal list, at every scale."""
from rework_16_lib import load, save, put, revolve, thermal, decide

doc = load("vacuum-cleaner")
AXIS, ORIGIN = [1, 0, 0], [0, 0.118, 0]
MOTOR = {
    "50": [[-0.212, 0], [-0.212, 0.03], [-0.204, 0.044], [-0.15, 0.044], [-0.15, 0.05], [-0.138, 0.05], [-0.138, 0.066], [-0.132, 0.068], [-0.094, 0.068], [-0.088, 0.062], [-0.088, 0.03], [-0.08, 0.03], [-0.08, 0]],
    "100": [[-0.212, 0], [-0.212, 0.044], [-0.15, 0.044], [-0.138, 0.05], [-0.138, 0.068], [-0.088, 0.068], [-0.088, 0.03], [-0.08, 0]],
    "200": [[-0.212, 0], [-0.212, 0.044], [-0.14, 0.044], [-0.14, 0.068], [-0.085, 0.068], [-0.085, 0]],
    "500": [[-0.21, 0], [-0.21, 0.06], [-0.085, 0.06], [-0.085, 0]],
}
BAG = {
    "50": [[-0.05, 0], [-0.05, 0.05], [-0.036, 0.066], [0.0, 0.074], [0.11, 0.074], [0.16, 0.066], [0.19, 0.046], [0.2, 0.02], [0.2, 0]],
    "100": [[-0.05, 0], [-0.05, 0.05], [-0.03, 0.07], [0.12, 0.074], [0.17, 0.062], [0.2, 0.03], [0.2, 0]],
    "200": [[-0.05, 0], [-0.05, 0.06], [0.0, 0.074], [0.15, 0.074], [0.2, 0.04], [0.2, 0]],
    "500": [[-0.05, 0], [-0.05, 0.07], [0.16, 0.07], [0.2, 0.04], [0.2, 0]],
}
for scale, volume in doc["volumes"].items():
    put(volume["thermal"], thermal(revolve("th-motor", "body", "tube-steel", MOTOR[scale], AXIS, ORIGIN, tone=0.2), "body-shell", inner=True), after="th-shell")
    put(volume["thermal"], thermal(revolve("th-dust-bag", "body", "hose-pvc", BAG[scale], AXIS, ORIGIN, tone=0.75), "body-shell", inner=True), after="th-motor")
    decide(volume, "Thermal: the heat source is a real part inside the canister, the bypass motor-fan unit (88 mm motor can, 136 mm fan stage) in the rear chamber on the canister axis in front of the exhaust grille; the 148 mm dust bag fills the front chamber behind the hose socket. Both show only in the cut.", "Thermik: Die Wärmequelle ist ein echtes Bauteil im Kessel, die Motor-Gebläse-Einheit (Motorbecher 88 mm, Gebläsestufe 136 mm) in der hinteren Kammer auf der Kesselachse vor dem Abluftgitter; der Staubbeutel von 148 mm füllt die vordere Kammer hinter dem Schlauchstutzen. Beide sind nur im Schnitt sichtbar.")
save("vacuum-cleaner", doc)
