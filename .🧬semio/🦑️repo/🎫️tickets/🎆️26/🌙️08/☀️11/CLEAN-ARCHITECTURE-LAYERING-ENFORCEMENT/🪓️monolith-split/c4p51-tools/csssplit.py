"""🎨️ Splits the quiz-react stylesheet into contiguous section children under 🎨️styles and rewrites the root as the header plus the ordered @import list (c4p51).

usage: csssplit.py [--apply]   (reads the snapshot c4p51-snap/f1.css; refuses when the live root differs from it)
"""
import os, sys, json
R, F, G, Q = (os.environ[k] for k in ("R", "F", "G", "Q"))
SNAP = G + "/c4p51-snap/f1.css"
ROOT = Q + "/🎯️targets/⚛️react/🎨️.css"
BASE = Q + "/🎯️targets/⚛️react/🎨️styles"
text = open(SNAP, encoding="utf-8").read()
lines = text.split("\n")
assert lines[-1] == ""
lines = lines[:-1]
CHILDREN = [
    (8, "🔤️text-and-glyphs", "The learner's text size as the root font size, word wrapping and the monochrome emoji glyphs."),
    (46, "🎞️icon-motion", "Icons stand still until their host is reached for: the looping microanimations and their keyframes."),
    (174, "🏠️chrome-and-home", "Card titles, the touch target size, the navbar title, the home card layer, the first-visit screen and the basics of the adaptive layouts."),
    (317, "🧩️task-layouts", "The head of a run and the layouts of the classification, sorting and matching tasks, each taken for the room its card has."),
    (616, "🗃️record-tables", "Tables that fold into lists in a card too narrow for their columns."),
    (722, "🏁️results-and-preferences", "The parts of a result, the preferences, the ways to appear, the segmented choice, the leaderboard row and the notices."),
    (872, "🤏️drag-and-spider", "The pointer drag states and the spider diagram's paint."),
    (939, "👥️peer-marks", "The pointers, focus frames and name labels of the other learners on the page."),
    (993, "📊️plots-and-crowd", "The plot columns, the figures of a task, what everyone answered, the score bins, their hover wash, the heading link of a card."),
    (1219, "⚖️footer-and-forced-colors", "The footer, reduced motion and the forced-colours (Windows high contrast) rules."),
    (1290, "🎚️challenge-keys", "The keys and guesses of the challenges, with their forced-colours rules."),
]
header = lines[:6]
out = {}
for i, (start, folder, doc) in enumerate(CHILDREN):
    end = CHILDREN[i + 1][0] - 1 if i + 1 < len(CHILDREN) else len(lines)
    body = lines[start - 1 : end]
    while body and body[-1].strip() == "":
        body.pop()
    emoji = folder[: folder.index("️")]
    out[folder] = "/* %s %s */\n\n%s\n" % (emoji, doc, "\n".join(body))
imports = "\n".join('@import "./🎨️styles/%s/🎨️.css";' % folder for _, folder, _ in CHILDREN)
root = "\n".join(header) + "\n\n" + imports + "\n"
sequence = [l for folder in out for l in out[folder].split("\n")[2:]]
expected = lines[7:]
a = [l for l in sequence if l.strip()]
b = [l for l in expected if l.strip()]
assert a == b, "non-blank line sequence differs"
for folder, t in out.items():
    print("%-30s %4d lines" % (folder, t.count("\n")))
print("root", root.count("\n"))
if "--apply" in sys.argv:
    assert open(ROOT, encoding="utf-8").read() == text, "root changed since the snapshot"
    for folder, t in out.items():
        target = BASE + "/" + folder + "/🎨️.css"
        assert not os.path.exists(target)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "w", encoding="utf-8").write(t)
    open(ROOT, "w", encoding="utf-8").write(root)
    json.dump([f for _, f, _ in CHILDREN], open(os.environ["T"] + "/css-children.json", "w", encoding="utf-8"), ensure_ascii=False)
    print("applied")
