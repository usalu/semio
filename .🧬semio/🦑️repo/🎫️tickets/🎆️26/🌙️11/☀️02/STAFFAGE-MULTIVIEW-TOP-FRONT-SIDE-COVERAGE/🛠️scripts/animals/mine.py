import json, sys
d = json.load(open(sys.argv[1], encoding="utf-8"))
mine = set("dairy-cow domestic-cat goat hedgehog herring-gull horse labrador-dog mallard-duck pig pigeon red-fox red-squirrel roe-deer sheep laptop mvhr-unit pendant-lamp room-thermostat server-rack shower-head split-ac-indoor-unit thermostatic-radiator-valve toaster tower-server vacuum-cleaner kitchen-extractor-hood".split())
n = 0
for k, e in d["assets"].items():
    if k.split("/")[1] in mine:
        for i in e["issues"]:
            n += 1; print(k, i["code"], i["path"], i["message"][:150])
print("total", n)
