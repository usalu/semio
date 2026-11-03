import json, sys
ev = json.load(open(sys.argv[1], encoding="utf-8"))
for e in ev:
    print(e[0], e[1].split("\\")[-1][:28], e[2], e[3])
    if e[3] == "Edit":
        print("   OLD:", e[4]["old_string"][:150].replace("\n", "\n"))
