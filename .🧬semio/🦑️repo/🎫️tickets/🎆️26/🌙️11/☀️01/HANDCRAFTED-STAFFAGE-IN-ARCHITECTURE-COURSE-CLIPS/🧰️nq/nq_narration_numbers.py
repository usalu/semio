"""🔢 Find en/de number mismatches in architecture clip narration via ast (py) and json walks."""

import ast
import json
import re
import sys
from pathlib import Path

ROOT = Path("C:/git/semio/🎓️teaching/🏛️architecture")
TOKEN = re.compile(r"\d{1,3}(?:[   ]\d{3})+|\d+(?:[.,]\d+)*")
THOUSANDS_DE = re.compile(r"\d{1,3}(?:\.\d{3})+")
THOUSANDS_EN = re.compile(r"\d{1,3}(?:,\d{3})+")
TOKENS = re.compile(r"[^\W\d_]+|[,;:.!?()–—]")
EN_UNITS = {w: (v,) for v, w in enumerate("zero one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen".split())}
EN_UNITS.update({w: (v,) for v, w in zip(range(20, 100, 10), "twenty thirty forty fifty sixty seventy eighty ninety".split())})
EN_UNITS.update({"hundred": (100,), "thousand": (1000,), "million": (10**6,), "billion": (10**9,), "trillion": (10**12,), "a": "ART", "one": "ART"})
DE_PARTS = {w: (v,) for w, v in {"null": 0, "eins": 1, "zwei": 2, "drei": 3, "vier": 4, "fünf": 5, "sechs": 6, "sieben": 7, "acht": 8, "neun": 9, "zehn": 10, "elf": 11, "zwölf": 12, "dreizehn": 13, "vierzehn": 14, "fünfzehn": 15, "sechzehn": 16, "siebzehn": 17, "achtzehn": 18, "neunzehn": 19, "zwanzig": 20, "dreißig": 30, "vierzig": 40, "fünfzig": 50, "sechzig": 60, "siebzig": 70, "achtzig": 80, "neunzig": 90, "hundert": 100, "tausend": 1000, "million": 10**6, "millionen": 10**6, "milliarde": 10**9, "milliarden": 10**9, "billion": 10**12, "billionen": 10**12}.items()}
DE_PARTS.update({"und": (), "ein": (1,)})
DE_KEYS = sorted(DE_PARTS, key=len, reverse=True)
DE_ARTICLES = {"ein", "eine", "einen", "einem", "einer", "eines"}
SCALES = {(100,), (1000,), (10**6,), (10**9,), (10**12,)}


def de_parts(word: str):
    """🔢 Number parts of a German compound word such as fünfundzwanzig, or None."""
    if word in DE_ARTICLES:
        return "ART"
    if word in DE_PARTS:
        return DE_PARTS[word]
    if word.endswith("mal") and len(word) > 3 and isinstance(de_parts(word[:-3]), tuple):
        return de_parts(word[:-3])
    for key in DE_KEYS:
        if word.startswith(key) and len(key) < len(word):
            rest = de_parts(word[len(key):])
            if isinstance(rest, tuple):
                return DE_PARTS[key] + rest
    return None


def en_parts(word: str):
    """🔢 Number parts of an English number word, or None."""
    return EN_UNITS.get(word)


def combine(vals: list[int]) -> float:
    """🔢 Value of a run of number parts such as fifteen hundred or six hundred thousand."""
    total = cur = 0
    for v in vals:
        if v == 100:
            cur = (cur or 1) * 100
        elif v >= 1000:
            total += (cur or 1) * v
            cur = 0
        else:
            cur += v
    return float(total + cur)


def word_numbers(text: str, lang: str) -> list[float]:
    """🔢 Numbers written as words, with runs grouped and English point decimals."""
    words = [w.lower() for w in TOKENS.findall(text)]
    parts = [(en_parts(w) if lang == "en" else de_parts(w)) for w in words]
    parts = [None if p in ((), None) else p for p in parts]
    point = "point" if lang == "en" else "komma"
    out, i, n = [], 0, len(words)
    while i < n:
        vals, j = [], i
        while j < n:
            p = parts[j]
            if p == "ART":
                if j + 1 < n and (parts[j + 1] in SCALES or words[j + 1] == point) or vals and 20 <= vals[-1] < 100 and words[j] == "one":
                    vals.append(1)
                else:
                    break
            elif words[j] == "and" and vals and j + 1 < n and isinstance(parts[j + 1], tuple) and lang == "en":
                pass
            elif p is None:
                break
            else:
                vals.extend(p)
            j += 1
        value = combine(vals) if vals else None
        if value is not None and j < n and words[j] == point:
            k, digits = j + 1, ""
            while k < n and (parts[k] == "ART" and words[k] == "one" or isinstance(parts[k], tuple) and len(parts[k]) == 1 and parts[k][0] < 10):
                digits += "1" if parts[k] == "ART" else str(parts[k][0])
                k += 1
            if digits:
                value += float("0." + digits)
                j = k
        if value is not None:
            out.append(value)
        i = max(j, i + 1)
    return out


def numbers(text: str, lang: str) -> list[float]:
    """🔢 Digit and word numbers in a string, German decimal commas and thousand dots normalised."""
    out = []
    text = text.replace("{,}", ",").replace("\\,", "")
    for tok in TOKEN.findall(text):
        tok = re.sub(r"[   ]", "", tok)
        if lang == "de" and THOUSANDS_DE.fullmatch(tok):
            tok = tok.replace(".", "")
        elif lang == "en" and THOUSANDS_EN.fullmatch(tok):
            tok = tok.replace(",", "")
        if len(re.findall(r"[.,]", tok)) > 1:
            out.extend(float(part) for part in re.split(r"[.,]", tok))
        else:
            out.append(float(tok.replace(",", ".")))
    return sorted(out + word_numbers(text, lang))


def drop_ones(values: list[float]) -> list[float]:
    """🔢 Values without 1s, since English one is mostly an article or pronoun."""
    return [v for v in values if v != 1.0]


def const_str(node) -> str | None:
    """🔤 String value of a constant node, else None."""
    return node.value if isinstance(node, ast.Constant) and isinstance(node.value, str) else None


def py_pairs(path: Path):
    """🔗 Bilingual pairs in a python module: tr(en, de), {en, de} dicts, (key, en, de) narration tuples."""
    tree = ast.parse(path.read_text(encoding="utf-8"), filename=str(path))
    narration = {id(sub) for node in ast.walk(tree) if isinstance(node, ast.Assign) and any(getattr(t, "id", None) == "NARRATION" for t in node.targets) for sub in ast.walk(node.value)}
    for node in ast.walk(tree):
        if isinstance(node, ast.Call) and getattr(node.func, "id", None) == "tr" and len(node.args) >= 2:
            en, de = const_str(node.args[0]), const_str(node.args[1])
            if en is not None and de is not None:
                yield node.lineno, "tr", en, de
        elif isinstance(node, ast.Dict):
            keyed = {const_str(k): const_str(v) for k, v in zip(node.keys, node.values) if k is not None}
            if keyed.get("en") is not None and keyed.get("de") is not None:
                yield node.lineno, "dict", keyed["en"], keyed["de"]
        elif isinstance(node, ast.Tuple) and len(node.elts) == 3 and id(node) in narration:
            parts = [const_str(e) for e in node.elts]
            if all(p is not None for p in parts):
                yield node.lineno, "narration", parts[1], parts[2]


def json_pairs(node, trail=""):
    """🔗 Bilingual pairs in a json document: objects with string en and de values."""
    if isinstance(node, dict):
        if isinstance(node.get("en"), str) and isinstance(node.get("de"), str):
            yield trail, "json", node["en"], node["de"]
        for key, value in node.items():
            yield from json_pairs(value, f"{trail}/{key}")
    elif isinstance(node, list):
        for index, value in enumerate(node):
            yield from json_pairs(value, f"{trail}[{index}]")


def clip_files(suffix: str):
    """📂 Clip-related source files of one suffix below the architecture root."""
    for path in sorted(ROOT.rglob(f"*{suffix}")):
        rel = path.relative_to(ROOT).parts
        if "__pycache__" not in rel and any("clip" in part for part in rel):
            yield path


def main() -> int:
    """🚀 Print every pair with en and de numbers, mismatches flagged."""
    sys.stdout.reconfigure(encoding="utf-8")
    total = 0
    mismatches = []
    for path in clip_files(".py"):
        for line, kind, en, de in py_pairs(path):
            total += 1
            en_n, de_n = numbers(en, "en"), numbers(de, "de")
            if drop_ones(en_n) != drop_ones(de_n):
                mismatches.append((path.relative_to(ROOT).as_posix(), line, kind, en_n, de_n, en, de))
    for path in clip_files(".json"):
        for trail, kind, en, de in json_pairs(json.loads(path.read_text(encoding="utf-8"))):
            total += 1
            en_n, de_n = numbers(en, "en"), numbers(de, "de")
            if drop_ones(en_n) != drop_ones(de_n):
                mismatches.append((path.relative_to(ROOT).as_posix(), trail, kind, en_n, de_n, en, de))
    print(f"pairs={total} mismatches={len(mismatches)}")
    for file, where, kind, en_n, de_n, en, de in mismatches:
        print(f"\n{file}:{where} [{kind}]\n  en {en_n}: {en}\n  de {de_n}: {de}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
