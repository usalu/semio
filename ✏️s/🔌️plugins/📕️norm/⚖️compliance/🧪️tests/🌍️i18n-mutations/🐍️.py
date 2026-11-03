"""🌍 No mutation label speaks English in its German text.

`python -I 🐍️.py` reads every `LocalizedLabel::native(en, de)` of the Eurocode mutation leaves as text and exits non-zero when the DE text holds a lowercase ASCII word that equals a word of the leaf's kind or `new*` field name (camelCase, snake_case, kebab-case) or is a listed English word. Capitalised ASCII words are German nouns or cognates and pass.

@see ../🌐️i18n-sinks/🐍️.py the sink scan this one follows.
"""
import pathlib, re, sys

NORM = pathlib.Path(__file__).resolve().parents[3]
PAIR = re.compile(r'LocalizedLabel::native\("((?:[^"\\]|\\.)*)", "((?:[^"\\]|\\.)*)"\)')
FIELD = re.compile(r"pub (new_[a-z0-9_]+):")
COGNATE = set("parallel moment detail form element system position radius ring normal relative operative station isolator block first net tests gamma alpha delta sigma outer wall".split())
ENGLISH = set("""kind member source name permanent height label backfill weld destabilizing stabilizing shape activity face material
direction class process length stress pressure movement friction surcharge governing layer stem thickness embedment vertical
horizontal concrete variable total pore structure bolts fabrication exponent deck change width depth angle area force load
ratio factor value type level limit strength diameter spacing number count slope bearing check design group deg bend of the
and with for from this that when then than per its each set insert remove add update replace""".split())


def words(name):
    return {w for w in re.split(r"[-_ ]+|(?<=[a-z0-9])(?=[A-Z])", name) if len(w) > 3}


def leaks(de, names):
    tokens = {t for t in re.findall(r"[^\W\d_]+", de) if t.isascii() and t[0].islower()}
    return sorted((tokens & ENGLISH) | {w for w in tokens & names if w not in COGNATE})


def main():
    bad = []
    files = sorted(NORM.glob("🗿️artifacts/*/🏅️standards/*/🪆️subsets/*/🧬️schema/🧬️mutations/*/🦀️.rs"))
    for rs in files:
        text = rs.read_text(encoding="utf-8")
        if "MutationKind<" not in text:
            continue
        names = {w.lower() for w in words(rs.parent.name.split("change-", 1)[-1]) | {x for f in FIELD.findall(text) for x in words(f)}}
        for en, de in PAIR.findall(text):
            found = leaks(de, names)
            if found:
                bad.append("%s: DE %r leaks %s" % (rs.parent.parent.parent.parent.parent.parent.parent.name + "/" + rs.parent.name, de, found))
    print("\n".join(bad) if bad else "ok: no mutation label leaks English into German (%d leaves)" % len(files))
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
