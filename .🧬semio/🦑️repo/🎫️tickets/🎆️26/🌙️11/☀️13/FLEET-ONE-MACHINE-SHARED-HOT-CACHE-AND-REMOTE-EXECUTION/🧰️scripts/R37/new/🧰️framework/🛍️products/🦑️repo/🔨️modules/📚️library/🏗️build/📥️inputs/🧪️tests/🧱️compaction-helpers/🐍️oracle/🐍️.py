"""Independent model of the helper selection of a compacted group: the helper list resolved against the inputs directory, kept when it lies inside the checkout and no prefix of the scheduling table covers it."""
import json,posixpath,sys
request=json.load(sys.stdin)
prefixes=[prefix for domain in request["table"]["domains"] for prefix in domain["prefixes"]]
kept=[]
for helper in request["helpers"]:
    path=posixpath.normpath(posixpath.join(request["directory"],helper))
    if path.startswith("../") or path==".." or posixpath.isabs(path):continue
    if any(path.startswith(prefix) for prefix in prefixes):continue
    kept.append(path)
json.dump(kept,sys.stdout)
