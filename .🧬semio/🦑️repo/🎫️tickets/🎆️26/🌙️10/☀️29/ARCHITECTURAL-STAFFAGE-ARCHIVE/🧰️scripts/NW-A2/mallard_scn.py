"""mallard-duck: poses (with leg inverse kinematics), clips and the thermal model"""
import math
from nw import *
from mallard_phys import *

HIP = (0.17, -0.06)
LEG = dict(thigh=0.05, shank=0.085, tarsus=0.045)
CONTACT_IN_FOOT = (0.04, -0.010)
ROOT_X = -0.18


def neck_offsets(asset, trunk_w, n1, n2, n3, hd):
    return offsets_from_world(asset, trunk_w, ["neck-1", "neck-2", "neck-3", "head"], [n1, n2, n3, hd])


def leg_offsets(asset, s, trunk_w, hip_w, target, tarsus_w, foot_w, bend=1):
    c, sn = math.cos(math.radians(foot_w)), math.sin(math.radians(foot_w))
    mtp = (target[0] - (c*CONTACT_IN_FOOT[0] - sn*CONTACT_IN_FOOT[1]), target[1] - (sn*CONTACT_IN_FOOT[0] + c*CONTACT_IN_FOOT[1]))
    ankle = (mtp[0] - LEG["tarsus"]*math.cos(math.radians(tarsus_w)), mtp[1] - LEG["tarsus"]*math.sin(math.radians(tarsus_w)))
    t1, t2 = two_link(hip_w, ankle, LEG["thigh"], LEG["shank"], bend)
    off = offsets_from_world(asset, trunk_w, [f"thigh-{s}", f"shank-{s}", f"tarsus-{s}", f"foot-{s}"], [t1, t2, tarsus_w, foot_w])
    return off


def HX(pitch):
    return trunk_hip(pitch, 0)[0]


def trunk_hip(pitch, ry):
    """world position of the hip for trunk pitch and root y (root x = ROOT_X)"""
    c, sn = math.cos(math.radians(pitch)), math.sin(math.radians(pitch))
    return (ROOT_X + c*HIP[0] - sn*HIP[1], ry + sn*HIP[0] + c*HIP[1])


def water_ry(pitch):
    return -0.17*math.sin(math.radians(pitch)) + 0.02*math.cos(math.radians(pitch))


def pose_bones(asset, pitch, ry, near, far, neck, wings=(0, 0), tail=0, tar=(-75, -75), ft=(0, 0), bend=1):
    """near/far = foot contact targets (world); neck = (n1,n2,n3,head) world angles"""
    hip = trunk_hip(pitch, ry)
    b = {"trunk": {"angleDeg": pitch}}
    b.update(neck_offsets(asset, pitch, *neck))
    b.update(leg_offsets(asset, "n", pitch, hip, near, tar[0], ft[0], bend))
    b.update(leg_offsets(asset, "f", pitch, hip, far, tar[1], ft[1], bend))
    if wings[0]: b["wing-n"] = {"angleDeg": wings[0]}
    if wings[1]: b["wing-f"] = {"angleDeg": wings[1]}
    if tail: b["tail"] = {"angleDeg": tail}
    return b


def build_scenario(asset):
    P = {}
    UP1 = trunk_hip(-66, water_ry(-66))[1] + 0.10
    # ---------------- thermal binding states
    def land(sun=0.5, ground=18, legs_c=0.12, feet_c=0.45):
        return {"body-plumage": rb(1.0, sun), "neck-plumage": rb(1.0, sun), "head-plumage": rb(1.0, sun), "bill": rb(1.0, sun),
                "wing-plumage": rb(0.5, sun*0.8), "tail-plumage": rb(1.0, sun*0.6),
                "legs-skin": rb(1.0, sun*0.5, (legs_c, ground)), "web-feet": rb(1.0, sun*0.3, (feet_c, ground))}
    def water(sun=0.5, tw=14.0, head_c=0.0, neck_c=0.0, body_c=0.42, tail_e=0.8):
        return {"body-plumage": rb(1.0 - body_c, sun, (body_c, tw), 0.25), "neck-plumage": rb(1.0 - neck_c, sun, (neck_c, tw) if neck_c else None, 0.3 if neck_c else None),
                "head-plumage": rb(1.0 - head_c, sun, (head_c, tw) if head_c else None, 0.5 if head_c else None), "bill": rb(1.0 - 0.5*head_c, sun, (0.5*head_c, tw) if head_c else None),
                "wing-plumage": rb(0.4, sun*0.7), "tail-plumage": rb(tail_e, sun*0.6),
                "legs-skin": rb(0.0, 0.0, (1.0, tw), 1.0), "web-feet": rb(0.0, 0.0, (1.0, tw), 1.0)}
    stand_neck = (56, 78, 46, -8)
    # ---------------- standing (default)
    P["standing"] = pose("standing", ("Standing alert", "Aufmerksam stehend"),
        ("Standing on both webbed feet on firm ground, body tilted 12 degrees, neck upright in a shallow S, head level with the bill horizontal, wings folded.",
         "Stehen auf beiden Schwimmfüßen auf festem Grund, Körper um 12 Grad geneigt, Hals aufrecht in flachem S, Kopf waagerecht, Flügel angelegt."),
        pose_bones(asset, 12, 0.150, (0.04, 0.0), (-0.02, 0.0), stand_neck), ["contact-n", "contact-f"], land(), {"plume-back": 1.0}, 1.5, "rest", {"x": ROOT_X})
    # ---------------- swimming (floating, paddling)
    sw_neck = (68, 96, 62, -6)
    P["swimming"] = pose("swimming", ("Swimming", "Schwimmen"),
        ("Floating on the water with the body level, the waterline cutting the hull about 0.065 m above the keel, neck in an S, feet trailing below the hull; the hull below the waterline is drawn transparent.",
         "Schwimmend auf dem Wasser, Körper waagerecht, die Wasserlinie schneidet den Rumpf etwa 0,065 m über dem Kiel, Hals im S, Füße hängen unter dem Rumpf; der Rumpf unter der Wasserlinie ist durchsichtig gezeichnet."),
        pose_bones(asset, 3, water_ry(3), (HX(3)+0.07, -0.17), (HX(3)+0.01, -0.19), sw_neck, tar=(-95, -100), ft=(-35, -30)), ["waterline"], water(), {"plume-back": 0.7}, 3.0, "swim", {"x": ROOT_X})
    # ---------------- dabbling
    db_neck = (-52, -70, -62, -50)
    P["dabbling"] = pose("dabbling", ("Dabbling", "Gründeln an der Oberfläche"),
        ("Surface-feeding: body pitched 24 degrees forward, neck stretched forward and down so that the bill sweeps just below the surface, tail raised, feet paddling gently.",
         "Seihen an der Oberfläche: Körper um 24 Grad nach vorn geneigt, Hals nach vorn unten gestreckt, sodass der Schnabel knapp unter der Wasserfläche seiht, Schwanz angehoben, Füße paddeln leicht."),
        pose_bones(asset, -24, water_ry(-24), (HX(-24)+0.0, -0.15), (HX(-24)-0.06, -0.15), db_neck, tail=18, tar=(-95, -100), ft=(-35, -30)), ["waterline"],
        water(head_c=0.35, neck_c=0.12, body_c=0.4, tail_e=1.0), {"plume-back": 0.5}, 2.2, "dabble", {"x": ROOT_X})
    P["upending"] = pose("upending", ("Upending", "Gründeln mit aufgestelltem Schwanz"),
        ("Tail-up feeding: body pitched 68 degrees forward, neck and head straight down below the surface, tail and feet out of the water.",
         "Gründeln mit aufgestelltem Schwanz: Körper um 68 Grad nach vorn geneigt, Hals und Kopf senkrecht unter der Wasseroberfläche, Schwanz und Füße über dem Wasser."),
        pose_bones(asset, -66, water_ry(-66), (HX(-66)-0.03, UP1), (HX(-66)-0.06, UP1-0.01), (-84, -90, -95, -90), tail=20, tar=(80, 80), ft=(100, 100), bend=1), ["waterline"],
        {**water(head_c=1.0, neck_c=0.8, body_c=0.3, tail_e=1.0), "legs-skin": rb(1.0, 0.4), "web-feet": rb(1.0, 0.4)}, {"plume-back": 0.3}, 2.8, "dabble", {"x": ROOT_X}, key=True)
    # wing stretch
    P["wing-stretch"] = pose("wing-stretch", ("Wing stretch", "Flügelstrecken"),
        ("Standing with both wings raised and opened 30 degrees away from the back, neck upright: the wing posture after preening or bathing.",
         "Stehend mit beiden Flügeln, die 30 Grad vom Rücken abgespreizt sind, Hals aufrecht: die Flügelhaltung nach Putzen oder Baden."),
        pose_bones(asset, 12, 0.150, (0.04, 0.0), (-0.02, 0.0), (56, 76, 46, -4), wings=(-30, -34)), ["contact-n", "contact-f"],
        {**land(), "wing-plumage": rb(1.0, 0.7)}, {"plume-back": 1.0}, 2.0, "rest", {"x": ROOT_X})

    # ---------------- waddle phases (IK): stance 0.6 of the cycle, double support 2 x 0.1
    N = 8
    S0 = 0.075
    LIFT = 0.026
    def foot_at(phase):
        """x, y of the contact target relative to the hip for a foot that touches down at phase 0"""
        ph = phase % 1.0
        if ph < 0.6:
            u = ph / 0.6
            return (S0 - 2*S0*u, 0.0, -75 + 0*u, 0.0)
        u = (ph - 0.6) / 0.4
        e = u*u*(3 - 2*u)
        return (-S0 + 2*S0*e, LIFT*math.sin(math.pi*u), -75 - 22*math.sin(math.pi*u), -18*math.sin(math.pi*u))
    walk_keys = []
    for i in range(N):
        ph = i / N
        pitch = 12 + 1.6*math.cos(4*math.pi*ph)
        hip = trunk_hip(pitch, 0.150)
        xn, yn, tn, fn = foot_at(ph)
        xf, yf, tf, ff = foot_at(ph + 0.5)
        stance_foot_y = 0.0
        near = (hip[0] + xn, yn)
        far = (hip[0] + xf, yf)
        b = pose_bones(asset, pitch, 0.150, near, far, stand_neck, tar=(tn, tf), ft=(fn, ff))
        contacts = [c for c, k in (("contact-n", ph % 1.0 < 0.6), ("contact-f", (ph + 0.5) % 1.0 < 0.6)) if k]
        reg = land()
        pid = f"waddle-{i}"
        P[pid] = pose(pid, (f"Waddle phase {i}", f"Watschelphase {i}"), (f"Key pose {i} of 8 of the waddle cycle.", f"Schlüsselpose {i} von 8 des Watschelzyklus."),
                      b, contacts, reg, {"plume-back": 1.0}, 2.6, "walk", {"x": ROOT_X}, key=True)
        walk_keys.append(pid)
    # ---------------- paddle phases
    M = 6
    paddle_keys = []
    for i in range(M):
        ph = i / M
        def foot_path(p):
            p = p % 1.0
            if p < 0.55:
                u = p/0.55
                return (0.09 - 0.14*u, -0.20 - 0.02*math.sin(math.pi*u), -95 - 6*u, -30 + 20*u)
            u = (p - 0.55)/0.45
            e = u*u*(3 - 2*u)
            return (-0.05 + 0.14*e, -0.20 + 0.03*math.sin(math.pi*u), -95 - 6*(1-e), -30 + 20*(1-e) - 25*math.sin(math.pi*u))
        xn, yn, tn, fn = foot_path(ph)
        xf, yf, tf, ff = foot_path(ph + 0.5)
        pitch = 3 + 1.0*math.sin(2*math.pi*ph)
        hip = trunk_hip(pitch, water_ry(pitch))
        near = (hip[0] + xn - 0.0, yn)
        far = (hip[0] + xf, yf)
        nk = (68 + 3*math.sin(2*math.pi*ph), 96, 62 - 2*math.sin(2*math.pi*ph), -6)
        b = pose_bones(asset, pitch, water_ry(pitch), near, far, nk, tar=(tn, tf), ft=(fn, ff))
        pid = f"paddle-{i}"
        P[pid] = pose(pid, (f"Paddle phase {i}", f"Paddelphase {i}"), (f"Key pose {i} of 6 of the paddling cycle.", f"Schlüsselpose {i} von 6 des Paddelzyklus."),
                      b, ["waterline"], water(), {"plume-back": 0.7}, 3.0, "swim", {"x": ROOT_X}, key=True)
        paddle_keys.append(pid)
    return P, walk_keys, paddle_keys, N, M


def clips(walk_keys, paddle_keys, N, M, advance):
    wd = 0.8
    def keys(ids, dur, ease="smooth"):
        return [{"t": round(dur*i/len(ids), 5), "pose": p, "ease": ease} for i, p in enumerate(ids)]
    waddle = {"id": "waddle", "title": tx("Waddling walk", "Watschelgang"), "description": tx(
        "Two-beat walk at about 0.3 m/s with a 0.8 s cycle: each foot is on the ground 60 % of the cycle, both feet share the load twice 0.08 s per cycle, the body pitches 1.6 degrees at step rate and the head stays level.",
        "Zweitakt-Gang mit etwa 0,3 m/s bei 0,8 s Zyklus: Jeder Fuß steht 60 % des Zyklus am Boden, beide Füße tragen pro Zyklus zweimal 0,08 s gemeinsam, der Körper nickt mit Schrittfrequenz um 1,6 Grad und der Kopf bleibt waagerecht."),
        "durationS": wd, "fps": 30, "loop": True, "rootMotion": "plant", "advanceM": advance,
        "keys": keys(walk_keys, wd), "contacts": [{"anchor": "contact-n", "fromS": 0, "toS": round(0.6*wd, 4)}, {"anchor": "contact-f", "fromS": round(0.5*wd, 4), "toS": wd}, {"anchor": "contact-f", "fromS": 0, "toS": round(0.1*wd, 4)}],
        "phases": [{"id": "double-support-a", "fromS": 0, "toS": round(0.1*wd, 4), "title": tx("Both feet on the ground", "Beide Füße am Boden")},
                   {"id": "near-stance", "fromS": round(0.1*wd, 4), "toS": round(0.5*wd, 4), "title": tx("Near foot carries, far foot swings", "Naher Fuß trägt, ferner Fuß schwingt")},
                   {"id": "double-support-b", "fromS": round(0.5*wd, 4), "toS": round(0.6*wd, 4), "title": tx("Both feet on the ground", "Beide Füße am Boden")},
                   {"id": "far-stance", "fromS": round(0.6*wd, 4), "toS": wd, "title": tx("Far foot carries, near foot swings", "Ferner Fuß trägt, naher Fuß schwingt")}],
        "events": [{"t": 0, "kind": "contact-on", "title": tx("Near foot touches down", "Naher Fuß setzt auf"), "target": "contact-n"},
                   {"t": round(0.6*wd, 4), "kind": "contact-off", "title": tx("Near foot lifts", "Naher Fuß hebt ab"), "target": "contact-n"},
                   {"t": round(0.5*wd, 4), "kind": "contact-on", "title": tx("Far foot touches down", "Ferner Fuß setzt auf"), "target": "contact-f"},
                   {"t": round(0.1*wd, 4), "kind": "contact-off", "title": tx("Far foot lifts", "Ferner Fuß hebt ab"), "target": "contact-f"}],
        "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.5}, {"t": round(wd/2, 4), "v": 2.7}, {"t": wd, "v": 2.5}]}]}
    pd = 0.7
    paddle = {"id": "paddle", "title": tx("Paddling swim", "Paddelnd schwimmen"), "description": tx(
        "Steady swimming with alternating foot strokes at 1.4 Hz per foot: the webbed foot spreads and pushes back on the power stroke, folds and returns forward on the recovery; the neck nods slightly. The hull floats on the waterline.",
        "Gleichmäßiges Schwimmen mit wechselnden Fußschlägen von 1,4 Hz je Fuß: Die Schwimmhaut spreizt sich und schiebt im Kraftschlag nach hinten, faltet sich und kehrt in der Erholung nach vorn zurück; der Hals nickt leicht. Der Rumpf schwimmt auf der Wasserlinie."),
        "durationS": pd, "fps": 30, "loop": True, "rootMotion": "none",
        "keys": keys(paddle_keys, pd), "contacts": [{"anchor": "waterline", "fromS": 0, "toS": pd}],
        "phases": [{"id": "power-near", "fromS": 0, "toS": round(0.385*pd, 4), "title": tx("Near foot power stroke", "Kraftschlag des nahen Fußes")},
                   {"id": "recovery-near", "fromS": round(0.385*pd, 4), "toS": pd, "title": tx("Near foot recovery", "Erholung des nahen Fußes")}],
        "events": [{"t": 0, "kind": "motion", "title": tx("Near foot starts its power stroke", "Naher Fuß beginnt den Kraftschlag")}, {"t": round(0.5*pd, 4), "kind": "motion", "title": tx("Far foot starts its power stroke", "Ferner Fuß beginnt den Kraftschlag")}],
        "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 3.0}, {"t": round(pd/2, 4), "v": 3.1}, {"t": pd, "v": 3.0}]}]}
    dd = 4.0
    dk = [(0.0, "swimming", "smooth"), (1.0, "dabbling", "smooth"), (1.8, "dabbling", "smooth"), (2.4, "upending", "smooth"), (3.0, "dabbling", "smooth"), (4.0, "swimming", "smooth")]
    dabble = {"id": "dabble", "title": tx("Dabbling and upending", "Gründeln und Kopfstand"), "description": tx(
        "A feeding cycle on the water: the duck dips from floating to surface dabbling, sweeps the bill, tips up with tail and feet out of the water for 0.6 s, returns to dabbling and floats up again; breathing slows during the dive and pauses with the nostrils under water.",
        "Ein Fresszyklus auf dem Wasser: Die Ente kippt vom Schwimmen zum Seihen an der Oberfläche, seiht mit dem Schnabel, stellt sich 0,6 s mit Schwanz und Füßen über Wasser auf den Kopf, kehrt zum Seihen zurück und richtet sich wieder auf; die Atmung verlangsamt sich beim Tauchen und pausiert mit den Nasenlöchern unter Wasser."),
        "durationS": dd, "fps": 30, "loop": True, "rootMotion": "none",
        "keys": [{"t": t, "pose": p, "ease": e} for t, p, e in dk], "contacts": [{"anchor": "waterline", "fromS": 0, "toS": dd}],
        "phases": [{"id": "tip-down", "fromS": 0, "toS": 1.0, "title": tx("Tipping forward", "Vornüberkippen")}, {"id": "sweep", "fromS": 1.0, "toS": 1.8, "title": tx("Bill sweeps below the surface", "Schnabel seiht unter der Oberfläche")},
                   {"id": "upend", "fromS": 1.8, "toS": 3.0, "title": tx("Tail-up feeding", "Gründeln mit aufgestelltem Schwanz")}, {"id": "recover", "fromS": 3.0, "toS": dd, "title": tx("Rising and floating", "Aufrichten und Schwimmen")}],
        "events": [{"t": 1.0, "kind": "contact-on", "title": tx("Bill reaches the water", "Schnabel erreicht das Wasser"), "target": "waterline"}, {"t": 2.4, "kind": "motion", "title": tx("Tail and feet out of the water", "Schwanz und Füße über Wasser")}],
        "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 3.0}, {"t": 1.0, "v": 2.2}, {"t": 2.4, "v": 2.8}, {"t": dd, "v": 3.0}]},
                   {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 2}, {"t": 0.9, "v": 3}, {"t": 3.2, "v": 2}]},
                   {"target": "emitter:plume-back", "ease": "smooth", "keys": [{"t": 0, "v": 0.7}, {"t": 2.4, "v": 0.3}, {"t": dd, "v": 0.7}]}]}
    return [waddle, paddle, dabble]


def thermal(skin):
    regs, skin = regions()
    pats = [pattern("rest", ("Resting, nostril breathing", "Ruhe, Nasenatmung"), 20, 0.014, 0.45, 0.03, 0.45, 0.07, "lasiewski-calder-1971"),
            pattern("walk", ("Walking, nostril breathing", "Gehen, Nasenatmung"), 30, 0.017, 0.46, 0.02, 0.46, 0.06, "lasiewski-calder-1971"),
            pattern("swim", ("Swimming, nostril breathing", "Schwimmen, Nasenatmung"), 34, 0.019, 0.46, 0.02, 0.46, 0.06, "prange-schmidt-nielsen-1970"),
            pattern("dabble", ("Dabbling, shallow breathing with pauses", "Gründeln, flache Atmung mit Pausen"), 12, 0.012, 0.35, 0.05, 0.35, 0.25, "lasiewski-calder-1971")]
    th = {
        "environment": {"airTempC": 15, "meanRadiantTempC": 15, "skyTempC": 5, "groundTempC": 16, "relativeHumidity": 0.6, "windMS": 0.5, "windDirectionDeg": 180, "solarWM2": 450, "diffuseFraction": 0.15, "pressurePa": 101325},
        "baseMet": 1.5,
        "body": {"species": "duck", "massKg": MASS, "skinAreaM2": round(skin, 4), "basalMetabolismW": 4.2, "metWM2": round(4.2/skin, 1), "coreSetPointC": 41.5, "skinSetPointC": 39.0, "sweating": False, "panting": True,
                 "maxSkinBloodFlowLM2H": 70, "minSkinBloodFlowLM2H": 5, "source": "authored-estimate", "reference": "lasiewski-dawson-1967"},
        "respiration": {"noseEmitter": "breath-nares", "exhaledTempC": 31, "exhaledRelativeHumidity": 0.95, "patterns": pats,
                        "motion": [{"bone": "trunk", "angleDeg": 1.2, "dy": 0.0015}], "source": "authored-estimate", "reference": "schmidt-nielsen-1970"},
        "passengers": [], "operating": [], "regions": regs,
        "emitters": [
            {"id": "breath-nares", "kind": "breath", "title": tx("Nares on the bill base (both, combined)", "Nasenlöcher am Schnabelansatz (beide zusammen)"), "anchor": "nostrils", "directionDeg": 25, "diameterM": 0.006, "source": "authored-estimate", "reference": "gupta-2010"},
            {"id": "plume-back", "kind": "plume", "title": tx("Warm-air plume above the back", "Warmluftfahne über dem Rücken"), "anchor": "back-plume", "directionDeg": 90, "diameterM": 0.16, "region": "body-plumage", "source": "authored-estimate", "reference": "scholander-1950"},
        ],
        "assumptions": [
            tx("The surface temperatures are solved, not authored: heat flows from the 41.5 degC core through tissue and the plumage layers; the plumage resistances (about 0.5 m2K/W on the body, 0.2-0.35 elsewhere) are authored estimates guided by plumage conductance data of waterbirds and apply to dry plumage, in water the contact binding takes over.",
               "Die Oberflächentemperaturen werden gelöst, nicht vorgegeben: Wärme fließt vom 41,5-Grad-Kern durch Gewebe und die Gefiederschichten; die Gefiederwiderstände (rund 0,5 m2K/W am Rumpf, 0,2-0,35 sonst) sind Schätzwerte nach Leitwertdaten von Wasservögeln und gelten für trockenes Gefieder, im Wasser übernimmt die Kontaktbindung."),
            tx("Legs and feet are bare scaled skin without layers and carry 7 % of the heat production as a controlled heat-loss path (counter-current exchange in the legs is not modelled).",
               "Beine und Füße sind unbefiederte beschuppte Haut ohne Schichten und tragen 7 % der Wärmeproduktion als geregelten Wärmeabgabeweg (Gegenstromaustausch in den Beinen wird nicht modelliert)."),
            tx("Birds breathe through the nares only; the exhaled air leaves at 31 degC and 95 % relative humidity (cooled by the nasal turbinates).",
               "Vögel atmen nur durch die Nasenlöcher; die Ausatemluft verlässt sie mit 31 Grad und 95 % relativer Feuchte (durch die Nasenmuscheln gekühlt)."),
            tx("In water the hull is in conductive contact with water at 14 degC over the submerged share of the body, legs and feet; evaporation from wet plumage is only represented by the wettedness value.",
               "Im Wasser steht der Rumpf über den eingetauchten Anteil von Körper, Beinen und Füßen in Wärmeleitkontakt mit 14-Grad-Wasser; Verdunstung aus nassem Gefieder ist nur durch den Feuchtewert abgebildet."),
        ]}
    return th, skin
