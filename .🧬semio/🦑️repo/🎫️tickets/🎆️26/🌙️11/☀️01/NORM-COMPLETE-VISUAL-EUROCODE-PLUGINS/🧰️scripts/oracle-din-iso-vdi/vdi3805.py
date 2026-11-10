"""VDI 3805 sheet 2 / 3 independent cross-checks: the EN 442 logarithmic mean excess temperature and radiator power law with a root-solved operating point (scipy brentq), the valve square law with the Kell water density, and the R5 progression of the minimum kvs table."""
import math

from scipy import optimize

KVS_TABLE = {10: 0.16, 15: 0.25, 20: 0.40, 25: 0.63, 32: 1.00, 40: 1.60, 50: 2.50, 65: 4.00, 80: 6.30, 100: 10.0, 125: 16.0, 150: 25.0, 200: 40.0, 250: 63.0, 300: 100.0}
DELTA_T_RUST = 49.83
CP_WATER = 4.19


def lmtd(flow, ret, room):
    return (flow - ret) / math.log((flow - room) / (ret - room))


def kell_density(t):
    return (999.83952 + 16.945176 * t - 7.9870401e-3 * t ** 2 - 46.170461e-6 * t ** 3 + 105.56302e-9 * t ** 4 - 280.54253e-12 * t ** 5) / (1 + 16.879850e-3 * t)


def vectors():
    dt_n = lmtd(75.0, 65.0, 20.0)
    radiators = []
    for phi_n, n, flow, ret, room in [(1200.0, 1.3, 75.0, 65.0, 20.0), (1200.0, 1.3, 55.0, 45.0, 20.0), (1200.0, 1.3, 70.0, 55.0, 20.0), (1800.0, 1.25, 45.0, 35.0, 20.0), (800.0, 1.33, 60.0, 50.0, 22.0)]:
        dt = lmtd(flow, ret, room)
        phi = phi_n * (dt / dt_n) ** n
        phi_rust = phi_n * (dt / DELTA_T_RUST) ** n
        m = phi / (CP_WATER * 1000.0 * (flow - ret))
        solved = optimize.brentq(lambda tr: phi_n * (lmtd(flow, tr, room) / dt_n) ** n - m * CP_WATER * 1000.0 * (flow - tr), room + 0.5, flow - 0.01) if abs(flow - ret) > 0 else ret
        radiators.append({"standardOutputW": phi_n, "n": n, "flowC": flow, "returnC": ret, "roomC": room, "deltaTLogK": dt, "outputW": phi, "rustOutputW": phi_rust, "rustRelativeDelta": phi_rust / phi - 1, "massFlowKgS": m, "returnSolvedC": solved})
    valves = []
    for kvs, q, t in [(1.0, 1.0, 20.0), (0.63, 0.4, 70.0), (2.5, 1.8, 80.0), (4.0, 5.0, 60.0)]:
        dp_nominal = (q / kvs) ** 2
        rho = kell_density(t)
        dp_density = (q / kvs) ** 2 * rho / 1000.0
        valves.append({"kvsM3H": kvs, "flowM3H": q, "waterC": t, "dropBarRustLaw": dp_nominal, "waterDensityKgM3": rho, "dropBarDensityCorrected": dp_density, "rustRelativeDelta": dp_nominal / dp_density - 1})
    authority = []
    for kvs, q, rest in [(1.0, 0.5, 0.15), (0.63, 0.3, 0.05), (2.5, 2.0, 0.3)]:
        dp100 = (q / kvs) ** 2
        authority.append({"kvsM3H": kvs, "designFlowM3H": q, "dropRestBar": rest, "dropValveBar": dp100, "authority": dp100 / (dp100 + rest)})
    r5 = [{"dn": dn, "tableKvs": v, "r5": 10 ** (round(5 * math.log10(v)) / 5), "matchesR5": abs(10 ** (round(5 * math.log10(v)) / 5) - v) / v < 0.05} for dn, v in KVS_TABLE.items()]
    return {"oracle": "scipy brentq, EN 442 log-mean excess, Kell water density", "deltaTNominalK": dt_n, "deltaTNominalRustK": DELTA_T_RUST, "deltaTRelative": DELTA_T_RUST / dt_n - 1, "radiators": radiators, "valves": valves, "authority": authority, "kvsTableR5": r5}


def divergences(doc):
    out = [{"id": "vdi3805.deltaT-nominal", "clause": "EN 442-2 / VDI 3805-3 4.2 (75/65/20 C)", "inputs": "75/65/20", "rust": doc["deltaTNominalRustK"], "reference": doc["deltaTNominalK"], "relative": doc["deltaTRelative"], "note": "rounded constant; EN 442 itself prints 49.83, informational"}]
    out += [{"id": f"vdi3805.valve-density.{v['kvsM3H']}-{v['waterC']}C", "clause": "VDI 2073 / IEC 60534-2-1 (kvs defined for water of density 1000 kg/m3)", "inputs": f"kvs {v['kvsM3H']} m3/h, Q {v['flowM3H']} m3/h, water {v['waterC']} C", "rust": v["dropBarRustLaw"], "reference": v["dropBarDensityCorrected"], "relative": v["rustRelativeDelta"], "note": "diagram drop law ignores the water density, informational"} for v in doc["valves"] if abs(v["rustRelativeDelta"]) > 0.01]
    return out
