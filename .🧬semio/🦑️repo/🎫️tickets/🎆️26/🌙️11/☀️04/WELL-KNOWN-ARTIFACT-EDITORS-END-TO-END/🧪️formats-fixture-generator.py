import codecs, json, sys
bodies = ["﻿positions { }", "  \tindented", "\n\nafter blank lines", "\r\nwindows first line", "", "trailing \r\n", "﻿\n  mixed", "äöü \U0001F3E0"]
preamble = "semio gis.gismap.dsl v1\n"
framing = []
for body in bodies:
    wire = codecs.encode(preamble + body, "utf-8")
    assert codecs.decode(wire, "utf-8") == preamble + body
    assert codecs.decode(wire, "utf-8-sig") == (preamble + body) if not preamble.startswith("﻿") else True
    framing.append({"body": body, "wireHex": wire.hex()})
reserved = ["CON", "PRN", "AUX", "NUL"] + [f"{p}{n}" for p in ("COM", "LPT") for n in list(range(1, 10)) + ["\u00b9", "\u00b2", "\u00b3"]]
names = [
    {"name": "Plan: A/B?", "fallback": "2d.note", "stem": "Plan_ A_B_"},
    {"name": "  Ground floor.  ", "fallback": "2d.note", "stem": "Ground floor"},
    {"name": "", "fallback": "", "stem": "document"},
]
for r in reserved:
    names.append({"name": r.lower(), "fallback": "k", "stem": "_" + r.lower()})
    names.append({"name": r + ".final", "fallback": "k", "stem": "_" + r + ".final"})
envelope_ids = []
for token, component, version in [("gis.gismap", "dsl", 1), ("stdio.semio.graph", "dsl", 2), ("stdio.semio", "pack", 1), ("stdio.semio.brep", "pack", 11), ("a.b.c.d", "op", 65535)]:
    plugin, artifact = token.rsplit(".", 1)
    envelope_ids.append({"id": token, "component": component, "version": version, "line": f"semio {token}.{component} v{version}", "plugin": plugin, "artifact": artifact})
json.dump({"envelopeIds": envelope_ids, "framing": framing, "preamble": preamble, "stems": names, "reservedDeviceNames": reserved}, sys.stdout, ensure_ascii=True, indent=1)
