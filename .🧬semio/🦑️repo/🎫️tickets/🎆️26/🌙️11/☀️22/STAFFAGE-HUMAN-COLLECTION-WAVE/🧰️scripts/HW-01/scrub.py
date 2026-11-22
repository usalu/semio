"""🧽️ Removes from a bilingual base text every clause (split at '; ') and sentence (split at '. ') that names a part the derived asset
no longer has; `drop` is (English pattern, German pattern)."""
import re


def _clean(s, pat):
    out = []
    for sentence in re.split(r"(?<=\.) (?=[A-ZÄÖÜ0-9])", s):
        parts = [c for c in sentence.split("; ") if not re.search(pat, c, re.I)]
        if parts:
            c = "; ".join(parts)
            out.append(c if c.endswith(".") else c + ".")
    return " ".join(out)


def scrub(text, drop):
    return {"en": _clean(text["en"], drop[0]), "de": _clean(text["de"], drop[1])}
