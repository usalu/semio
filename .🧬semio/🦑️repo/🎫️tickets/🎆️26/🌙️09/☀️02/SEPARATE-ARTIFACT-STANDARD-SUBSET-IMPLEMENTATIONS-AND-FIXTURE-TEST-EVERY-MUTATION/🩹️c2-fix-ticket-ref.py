import sys, subprocess

TICKET_PATH = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"

paths = [
    "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️标准" if False else "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "🧰️框架" if False else "🧰️framework/🛍️products/💻️os/🎚️config/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️标准" if False else "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
]

count = 0
for p in paths:
    with open(p, encoding="utf-8") as fh:
        s = fh.read()
    old = "See $TICKET/📓️c2-native-artifact-oracles.md."
    new = f"See {TICKET_PATH}/📓️c2-native-artifact-oracles.md."
    if old in s:
        s2 = s.replace(old, new)
        with open(p, "w", encoding="utf-8") as fh:
            fh.write(s2)
        count += 1
        print("patched", p)
    else:
        print("no-match (already patched or absent)", p)

print("total patched:", count)
