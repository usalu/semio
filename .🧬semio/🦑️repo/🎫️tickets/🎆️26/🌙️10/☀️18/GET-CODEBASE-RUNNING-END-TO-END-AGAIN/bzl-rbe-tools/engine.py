# shared by mkoverlay.py, stamp.py and apply.py: edit application and hashing.
import hashlib, os, sys
sys.dont_write_bytecode = True
ROOT = "/Users/ueli/Documents/semio"
TOOLS = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, TOOLS)
from edits import EDITS
from targets import NEW


def sha(text):
    return hashlib.sha256(text.encode("utf8")).hexdigest()


def read(path):
    with open(path, encoding="utf8") as handle:
        return handle.read()


def apply_one(text, edit):
    count = text.count(edit["old"])
    if count != 1:
        raise ValueError(f'{edit["file"]}: anchor found {count} times: {edit["old"][:90]!r}')
    return text.replace(edit["old"], edit["new"])


def edited_files():
    order = []
    for edit in EDITS:
        if edit["file"] not in order:
            order.append(edit["file"])
    return order
