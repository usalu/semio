"""Shared helpers of catalogue-bridge v2: immediate, anchor-asserting edits and the EN 1995 template access. Run with /home/deck/semio/.venv/bin/python -I."""
import glob, json, os, re, shutil
SEMIO = "/home/deck/semio"
NORM = SEMIO + "/✏️s/🔌️plugins/📕️norm"
ARTS = NORM + "/🗿️artifacts"
OWNERS = SEMIO + "/📋️build-owners.json"
TAXONOMY = NORM + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
ANY = "/🏅️standards/🔖️1/🪆️subsets/✳️any"
LOG = []


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, s):
    assert len(p[len(SEMIO) + 1:].encode()) < 256, "path too long: " + p
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)
    LOG.append(p)


def sub(p, old, new, count=1, once=True):
    s = rd(p)
    assert old in s, "anchor missing in %s: %r" % (p, old[:100])
    if once and count == 1:
        assert s.count(old) == 1, "anchor not unique in %s (%d): %r" % (p, s.count(old), old[:100])
    wr(p, s.replace(old, new, count if count else -1))


def sub_if(p, old, new):
    s = rd(p)
    if old in s and new not in s:
        wr(p, s.replace(old, new, 1))
        return True
    return False


def subre(p, pat, repl, flags=re.S, count=1):
    s = rd(p)
    n, k = re.subn(pat, repl, s, count=count, flags=flags)
    assert k >= 1, "pattern missing in %s: %r" % (p, pat[:100])
    wr(p, n)


def art_root(short):
    return next(ARTS + "/" + d for d in os.listdir(ARTS) if d.endswith(short))


class Art:
    def __init__(self, short):
        self.short = short
        self.root = art_root(short)
        self.emoji = os.path.basename(self.root)[: -len(short)]
        self.any = self.root + ANY
        self.mut = self.any + "/🧬️schema/🧬️mutations"
        self.schema = self.any + "/🧬️schema"
        self.snap = self.schema + "/📸️snapshot"
        self.diff = self.schema + "/🔺️diff"
        self.fixd = self.any + "/🧪️tests/🧫️fixtures/🧬️mutations"
        self.P = short.replace("en", "En", 1)
        self.crate = "semio-s-artifact-norm-%s-" % short
        self.rel_mut = os.path.relpath(self.mut, self.root)


T = Art("en1995")
LEAF_DIRS = {"insert-case": "➕️insert-case", "remove-case": "➖️remove-case", "set-case-number": "🔢️set-case-number", "set-case-text": "🔤️set-case-text", "change-case-label-en": "🏷️change-case-label-en", "change-case-label-de": "🏷️change-case-label-de", "change-case-entry-id": "📇️change-case-entry-id"}
KINDS = list(LEAF_DIRS)
RECORD = {k: "".join(w.capitalize() for w in k.split("-")) for k in KINDS}
MODULE = {k: k.replace("-", "_") for k in KINDS}
