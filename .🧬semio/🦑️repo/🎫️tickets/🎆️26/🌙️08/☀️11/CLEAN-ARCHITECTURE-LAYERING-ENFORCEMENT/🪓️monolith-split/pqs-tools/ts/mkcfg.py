import os
F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
V = "️"
T = 'const T = (emoji, slug, doc, ranges) => ({ emoji, folder: emoji + "%s" + slug, doc, ranges });\n' % V
tests = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/❓️quiz/🧪️tests"
orig = F + "/pqs-tools/ts/orig/"
def header(name, emoji):
    ls = open(orig + name, encoding="utf8").read().split("\n")
    end = next(i for i, l in enumerate(ls) if l.strip() == "*/")
    first = ls[0]
    assert first.startswith("/** " + emoji + V + " "), first
    inner = [first[len("/** " + emoji + V + " "):]] + ls[1:end]
    return end + 1, "\n".join(inner) + "\n"
import json
hg_lines, hg_doc = header("home-grid.tsx", "🏠")
pc_lines, pc_doc = header("pet-companions.tsx", "🐾")
hg = T + '''export default {
  source: "%(orig)shome-grid.tsx",
  isTsx: true,
  mode: "module",
  ext: ".tsx",
  blankHeader: %(hgn)d,
  testsDir: "%(tests)s",
  helperFolder: "🛠%(V)shome-grid-kit",
  helper: { emoji: "🛠%(V)s", doc: "Shared fixtures, stylesheet readers and the home harness of the layered-home suites (the topics sit in the sibling folders `🏠%(V)shome-grid`, `🎫%(V)squiz-card-challenges`, `🥇%(V)shome-leaderboards` and `📱%(V)shome-text-and-viewport`)." },
  topics: [
    T("🏠", "home-grid", %(hgdoc)s, [[354, 732]]),
    T("🎫", "quiz-card-challenges", "The challenge on the quiz cards and pages of the layered home.", [[734, 918]]),
    T("🥇", "home-leaderboards", "The leaderboard of the new contract on the layered home and its four leaderboards with their categories.", [[920, 1154]]),
    T("📱", "home-text-and-viewport", "The layered home in the learner's text size, the opened page and the short viewport.", [[1156, 1246]]),
  ],
};
''' % dict(orig=orig, tests=tests, V=V, hgdoc=json.dumps(hg_doc, ensure_ascii=False), hgn=hg_lines)
pc = T + '''export default {
  source: "%(orig)spet-companions.tsx",
  isTsx: true,
  mode: "module",
  ext: ".tsx",
  blankHeader: %(pcn)d,
  testsDir: "%(tests)s",
  helperFolder: "🪛%(V)spet-companions-kit",
  helper: { emoji: "🪛%(V)s", doc: "Shared doubles, motion preference, render targets and screen sessions of the pet-companion suites (the topics sit in the sibling folders `🐾%(V)spet-companions`, `🎪%(V)spets-on-stage` and `🎛%(V)spet-allowances`)." },
  splits: { 310: {} },
  topics: [
    T("🐾", "pet-companions", %(pcdoc)s, [[311, 434], [436, 496], [498, 517], [519, 730]]),
    T("🎪", "pets-on-stage", "Pet companions: the pets on stage and the client around them.", [[732, 879], [881, 1089]]),
    T("🎛", "pet-allowances", "Pet companions: what the learner allows, the hand and the topics, and play with the pets.", [[1091, 1199], [1201, 1303], [1305, 1395]]),
  ],
};
''' % dict(orig=orig, tests=tests, V=V, pcdoc=json.dumps(pc_doc, ensure_ascii=False), pcn=pc_lines)
open("cfg-hg.mjs", "w", encoding="utf8").write(hg)
open("cfg-pc.mjs", "w", encoding="utf8").write(pc)
