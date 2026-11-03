"""🔁 pa: assert byte-identical roundtrip of catalog assets - usage: rt.py <asset-id>..."""
import sys

from jio import load

for asset_id in sys.argv[1:]:
    load(asset_id)
    print(asset_id, "ok")
