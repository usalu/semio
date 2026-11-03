import json, re
from decimal import Decimal

_EXP = re.compile(r"(?<![\w\"])-?\d+(?:\.\d+)?[eE][-+]?\d+(?![\w\"])")


def dumps(obj):
    text = json.dumps(obj, ensure_ascii=False, indent=1)
    return _EXP.sub(lambda m: format(Decimal(m.group(0)), "f"), text) + "\n"


def read(path):
    return json.loads(open(path, encoding="utf-8", newline="").read())


def write(path, obj):
    import os, tempfile
    text = dumps(obj)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(os.path.abspath(path)), suffix=".tmp")
    try:
        with os.fdopen(fd, "w", encoding="utf-8", newline="") as f:
            f.write(text)
        os.replace(tmp, path)
    except BaseException:
        if os.path.exists(tmp):
            os.remove(tmp)
        raise


if __name__ == "__main__":
    import sys
    raw = open(sys.argv[1], encoding="utf-8", newline="").read()
    print(dumps(json.loads(raw)) == raw)
