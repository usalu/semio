"""Generates 1-stiffness-kinds.py from the v11 wall-poisson kind script: one parameterised clone script for the member and the building field of the effective stiffness (run: python -I 1-stiffness-kinds.py member|building)."""
import re
s = open("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/en1998-v11/3-wall-poisson-kind.py").read()


def r(a, b, n=1):
    global s
    assert s.count(a) == n, (a, s.count(a))
    s = s.replace(a, b)


i = s.index('SRC = "change-member-wall-thickness-m"')
j = s.index("def words")
hdr = '''import sys
CFG = {
    "member": dict(ent="member", src="change-member-wall-thickness-m", old=dict(snake="wall_thickness_m", kebab="wall-thickness-m", camel="wallThicknessM", pascal="WallThicknessM", en="Masonry wall thickness t_ef (0 = not a masonry wall)", de="Mauerwerksdicke t_ef (0 = keine Mauerwerkswand)"), field="effective_stiffness_ratio", tok=("member-wall-thickness-m", "MemberWallThicknessM", "member_wall_thickness_m", "members[].wallThicknessM"), group="🧱️members-wall-rows", crate="semio_s_artifact_norm_en1998_mut_members_wall_rows", agg="🧱️members", pascal_prefix="ChangeMember", snake_prefix="change_member_", base_old='"wallThicknessM": 1.5', base_new='"wallThicknessM": 0.0'),
    "building": dict(ent="building", src="change-building-setback-ratio", old=dict(snake="setback_ratio", kebab="setback-ratio", camel="setbackRatio", pascal="SetbackRatio", en="Largest set-back as share of the plan dimension below", de="Größter Rücksprung als Anteil der darunterliegenden Grundrissabmessung"), field="storey_stiffness_ratio", tok=("building-setback-ratio", "BuildingSetbackRatio", "building_setback_ratio", "buildings[].setbackRatio"), group="🏢️buildings-envelope", crate="semio_s_artifact_norm_en1998_mut_buildings_envelope", agg="🏢️buildings", pascal_prefix="ChangeBuilding", snake_prefix="change_building_", base_old='"setbackRatio": 1.5', base_new='"setbackRatio": 0.0'),
}[sys.argv[1]]
SRC = CFG["src"]
OLD = CFG["old"]
SPEC = json.load(open("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/field-adder/specs/en1998-effective-stiffness.json"))


'''
s = s[:i] + hdr + s[j:]
r('for f in SPEC["fields"]:\n    w = words', 'for f in [x for x in SPEC["fields"] if x["field"] == CFG["field"]]:\n    w = words')
r('value="0.3"', 'value="0.5"')
r('TOK = ("member-wall-thickness-m", "MemberWallThicknessM", "member_wall_thickness_m", "members[].wallThicknessM")\nGROUP = "🧱️members-wall-rows"\nCRATE = "semio_s_artifact_norm_en1998_mut_members_wall_rows"', 'TOK = CFG["tok"]\nGROUP = CFG["group"]\nCRATE = CFG["crate"]')
r('''s.replace('"wallThicknessM": 1.5', '"wallThicknessM": 0.0', 1)''', '''s.replace(CFG["base_old"], CFG["base_new"], 1)''')
r('''s.replace('"wallThicknessM": 1.5', '"wallThicknessM": null', 1)''', '''s.replace(CFG["base_old"], CFG["base_old"].split(":")[0] + ": null", 1)''')
r('bp = MUT + "/🧱️members/🦀️.rs"\n        t = rd(bp)\n        old_use', 'bp = MUT + "/" + CFG["agg"] + "/🦀️.rs"\n        t = rd(bp)\n        old_use')
r('''if "change-member-" + NEW[0]["kebab"] in text or "ChangeMember" + NEW[0]["pascal"] in text or "change_member_" + NEW[0]["snake"] in text:''', '''if "change-" + CFG["ent"] + "-" + NEW[0]["kebab"] in text or CFG["pascal_prefix"] + NEW[0]["pascal"] in text or CFG["snake_prefix"] + NEW[0]["snake"] in text:''')
r('''lambda l: " = 123;" in l and "oneof" not in l and l.startswith("    "))''', '''lambda l: bool(re.search(r" = \\d+;", l)) and "oneof" not in l and l.startswith("    "))''')
r('''if "change_member_" + NEW[0]["snake"] + " =" not in s:''', '''if CFG["snake_prefix"] + NEW[0]["snake"] + " =" not in s:''')
r('''"    ChangeMember%s change_member_%s = %d;" % (d["pascal"], d["snake"], top + 1 + k)''', '''"    %s%s %s%s = %d;" % (CFG["pascal_prefix"], d["pascal"], CFG["snake_prefix"], d["snake"], top + 1 + k)''')
r('''"record change-member-%s tag=%d\\nfield payload bytes\\n" % (d["kebab"], tag + k)''', '''"record change-%s-%s tag=%d\\nfield payload bytes\\n" % (CFG["ent"], d["kebab"], tag + k)''')
s = s.replace('"member-" + NEW[0]["kebab"] + "/"', 'CFG["ent"] + "-" + NEW[0]["kebab"] + "/"')
s = s.replace('"change-member-" + NEW[0]["kebab"]', '"change-" + CFG["ent"] + "-" + NEW[0]["kebab"]')
s = s.replace('"change-member-" + d["kebab"]', '"change-" + CFG["ent"] + "-" + d["kebab"]')
s = s.replace('"change-member-" + n["kebab"]', '"change-" + CFG["ent"] + "-" + n["kebab"]')
s = s.replace('FIX + "/✏️" + kind\n', 'FIX + "/✏️" + SHORT\n')
s = s.replace("fsrc, fdst = FIX + \"/✏️\" + SRC, FIX + \"/✏️\" + kind", "fsrc, fdst = FIX + \"/✏️\" + SRC, FIX + \"/✏️\" + SHORT")
s = s.replace("        mounts.append(kind)\n", "        mounts.append(kind)\n        SHORT = \"change-\" + CFG[\"ent\"] + \"-stiffness-ratio\"\n")
s = s.replace("\n\nmain()\n", """

def shorten():
    short, long_ = "✏️change-" + CFG["ent"] + "-stiffness-ratio", "✏️change-" + CFG["ent"] + "-" + NEW[0]["kebab"]
    for p in (ANY + "/🧪️tests/🫨️mutate-en1998-1/🐍️.py", ANY + "/🧪️tests/🫨️mutate-en1998-1/🥒️.feature", ANY + "/🧪️tests/🫨️mutate-en1998-1/🦀️.rs"):
        t = rd(p)
        if long_ in t:
            wr(p, t.replace(long_, short))
    p = ANY + "/🔮️oracles/🔣️.json"
    t = rd(p)
    old = '"mutationDirectoryName": "' + long_ + '"'
    if old in t:
        wr(p, t.replace(old, '"mutationDirectoryName": "' + short + '"'))


main()
shorten()
""")
open("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/en1998-v12/1-stiffness-kinds.py", "w").write(s)
