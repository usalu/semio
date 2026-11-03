"""Helpers of the dead-fields tool: removes `reliabilityClass` and the kind `change-reliability-class` from an artifact. Every edit asserts the number of matches it expects."""
import os, re, shutil, sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/rc-threading")
import rc_lib
from rc_lib import ART, SUBSET, rd, wr, CHANGED

KIND = "change-reliability-class"


def sub(path, pattern, repl, count=1, flags=0):
    text = rd(path)
    found = len(re.findall(pattern, text, flags))
    assert found == count, (path[-80:], pattern[:90], "expected", count, "found", found)
    wr(path, re.sub(pattern, repl, text, flags=flags))


def drop_lines(path, pattern, count=1):
    sub(path, pattern, "", count, re.M)


def rmtree(path):
    assert os.path.isdir(path), path
    for root, _, files in os.walk(path):
        for name in files:
            CHANGED.append(os.path.join(root, name))
    if not rc_lib.DRY:
        shutil.rmtree(path)


def value_end(text, i):
    while text[i] in " \t\n":
        i += 1
    if text[i] in "{[":
        depth, in_str = 0, False
        while True:
            c = text[i]
            if in_str:
                if c == "\\":
                    i += 1
                elif c == '"':
                    in_str = False
            elif c == '"':
                in_str = True
            elif c in "{[":
                depth += 1
            elif c in "}]":
                depth -= 1
                if depth == 0:
                    return i + 1
            i += 1
    if text[i] == '"':
        i += 1
        while text[i] != '"':
            i += 2 if text[i] == "\\" else 1
        return i + 1
    m = re.compile(r"[^,}\]\s]+").match(text, i)
    return m.end()


def cut(text, ks, ve):
    """Removes the member or element spanning [ks, ve) with its separating comma, whole line when it stands alone."""
    after = re.match(r"[ \t]*,", text[ve:])
    line_start = text.rfind("\n", 0, ks) + 1
    alone = text[line_start:ks].strip() == ""
    if after:
        end = ve + after.end()
        tail = re.match(r"[ \t]*\n", text[end:]) if alone else re.match(r"[ \t]*", text[end:])
        return text[: line_start if alone else ks] + text[end + (tail.end() if tail else 0):]
    head = text[:ks].rstrip()
    assert head.endswith(","), text[max(0, ks - 40):ks]
    return head[:-1] + text[ve:]


def drop_key(text, key):
    """Removes every `"key": value` member of a JSON text (pretty or compact) together with its separating comma; returns (text, count)."""
    count = 0
    while True:
        m = re.search(r'"%s"\s*:' % re.escape(key), text)
        if not m:
            return text, count
        text = cut(text, m.start(), value_end(text, m.end()))
        count += 1


def drop_objects(path, needle, count):
    """Removes every JSON object that directly holds the member text `needle`."""
    text = rd(path)
    found = 0
    while needle in text:
        at = text.index(needle)
        depth, i = 0, at
        while True:
            i -= 1
            if text[i] == "}":
                depth += 1
            elif text[i] == "{":
                if depth == 0:
                    break
                depth -= 1
        text = cut(text, i, value_end(text, i))
        found += 1
    assert found == count, (path[-80:], needle, "expected", count, "found", found)
    wr(path, text)


def drop_json_key(path, key, count=1):
    text, found = drop_key(rd(path), key)
    assert found == count, (path[-80:], key, "expected", count, "found", found)
    wr(path, text)


def drop_required(path, key="reliabilityClass", count=1):
    text = rd(path)
    text, found = re.subn(r',\s*"%s"(?=\s*\])' % key, "", text)
    assert found == count, (path[-80:], "required", found)
    wr(path, text)


def sweep_fixtures(root):
    """Removes the `reliabilityClass` member from every json fixture below root."""
    changed = 0
    for base, _, files in os.walk(root):
        for name in files:
            if not name.endswith(".json"):
                continue
            path = os.path.join(base, name)
            text = rd(path)
            if "reliabilityClass" not in text:
                continue
            new, n = drop_key(text, "reliabilityClass")
            assert n and "reliabilityClass" not in new.replace("newReliabilityClass", ""), path[-100:]
            wr(path, new)
            changed += 1
    return changed


def finish(dry):
    for p in CHANGED:
        if os.path.exists(p) and p in rc_lib.CACHE:
            pass
    print(("DRY " if dry else "APPLIED ") + str(len(set(CHANGED))), "files")


def drop_dsl_token(path):
    text = rd(path)
    new, n = re.subn(r"^reliability-class=\w+\n|[ ]reliability-class=\w+(?=[ \n])", "", text, flags=re.M)
    assert n == 1, (path[-80:], n)
    wr(path, new)
