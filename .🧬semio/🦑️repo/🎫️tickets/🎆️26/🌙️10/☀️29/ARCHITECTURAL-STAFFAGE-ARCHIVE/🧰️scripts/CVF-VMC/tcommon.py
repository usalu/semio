"""Shared thermal-v2 construction helpers for CV-VM. Numbers live in the asset scripts; this only shapes the JSON."""
from lib import tx

ENV_KEYS = ("airTempC", "meanRadiantTempC", "skyTempC", "groundTempC", "relativeHumidity", "windMS", "windDirectionDeg", "solarWM2", "diffuseFraction", "pressurePa")


def environment(old, skyTempC, groundTempC, diffuse=0.2):
    e = dict(old)
    e["skyTempC"] = skyTempC
    e["groundTempC"] = groundTempC
    e["diffuseFraction"] = diffuse
    return {k: e[k] for k in ENV_KEYS if k in e}


def layer(id, en, de, kind, material, thick, resistance, hc=None, vapour=None, source="authored-estimate", ref=None):
    d = {"id": id, "title": tx(en, de), "kind": kind, "material": material}
    if thick is not None:
        d["thicknessM"] = thick
    d["resistanceM2KW"] = resistance
    if vapour is not None:
        d["vapourResistanceM2PaW"] = vapour
    if hc is not None:
        d["heatCapacityJM2K"] = hc
    d["source"] = source
    if ref:
        d["reference"] = ref
    return d


def region(id, en, de, kind, model, material, area, length, geometry, sky, hc, layers=(), operating=None, source="authored-estimate", ref=None):
    r = {"id": id, "title": tx(en, de), "kind": kind, "model": model, "material": material, "areaM2": area, "characteristicLengthM": length,
         "geometry": geometry, "skyViewFactor": sky, "layers": list(layers), "heatCapacityJM2K": hc}
    if operating:
        r["operating"] = {"key": operating[0], "heatW": operating[1]}
    r["source"] = source
    if ref:
        r["reference"] = ref
    return r


def level(v, en, de):
    return {"v": v, "title": tx(en, de)}


def opkey(id, en, de, levels):
    return {"id": id, "title": tx(en, de), "levels": [level(*l) for l in levels]}


def emitter(id, kind, en, de, anchor, direction, diameter, key, flow, rise, vapour, source="authored-estimate", ref=None):
    d = {"id": id, "kind": kind, "title": tx(en, de), "anchor": anchor, "directionDeg": direction, "diameterM": diameter, "operatingKey": key,
         "volumeFlowM3S": flow, "temperatureRiseK": rise, "vapourKgPerKg": vapour, "source": source}
    if ref:
        d["reference"] = ref
    return d


def binding(regions_default, overrides, operating, emitters, contacts=None):
    """regions_default: id -> (exposed, sunlit); overrides: id -> dict of updates; contacts: id -> (fraction, tempC)."""
    out = {}
    for rid, (ex, su) in regions_default.items():
        b = {"exposedFraction": ex, "sunlitFraction": su}
        for k, v in (overrides or {}).get(rid, {}).items():
            b[k] = v
        if contacts and rid in contacts:
            b["contact"] = {"fraction": contacts[rid][0], "tempC": contacts[rid][1]}
        out[rid] = b
    return {"operating": dict(operating), "regions": out, "emitters": dict(emitters)}
