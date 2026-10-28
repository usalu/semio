"""🐍️ Independent retention table: keeps the current image and the `keepNewest` newest by age (ties by identity), retires the rest. Reads {images, current, keepNewest} on stdin; prints {retire}."""
import json, sys

request = json.load(sys.stdin)
ordered = sorted(request["images"], key=lambda row: (-row["createdAt"], row["identity"]))
kept = {request["current"]} | {row["identity"] for row in ordered[: request["keepNewest"]]}
print(json.dumps({"retire": sorted(row["identity"] for row in ordered if row["identity"] not in kept)}))
