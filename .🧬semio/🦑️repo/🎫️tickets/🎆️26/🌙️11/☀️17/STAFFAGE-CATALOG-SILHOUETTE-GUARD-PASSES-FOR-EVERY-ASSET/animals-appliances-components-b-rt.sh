#!/bin/zsh
# usage: rt.sh slug scales view "shape=solids;shape=solids" [tol]
H="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET"
python3 - "$@" <<'PY'
import sys, importlib.util
H = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET/"
s = importlib.util.spec_from_file_location("r", H + "animals-appliances-components-b-retrace.py"); r = importlib.util.module_from_spec(s); s.loader.exec_module(r)
slug, scales, view, spec = sys.argv[1:5]; tol = sys.argv[5] if len(sys.argv) > 5 else "1.2"
d = r.l.doc(slug)
for item in spec.split(";"):
    sid, so = item.split("=")
    for sc in scales.split(","):
        try: r.retrace(d, slug, sc, view, sid, so, tol)
        except (LookupError, KeyError) as e: print("skip", sc, view, sid, repr(e)[:90].replace("\\n", " "))
d.commit()
PY
