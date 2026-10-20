# 🪝 Anchored text edits: each old text must occur exactly once in the CURRENT live text; an edit whose new text is already present is a no-op (idempotent).
def once(text, old, new, label):
    if new in text and (old not in text or old in new): return text, False
    if text.count(old) != 1: raise SystemExit("anchor %s: %d occurrence(s), expected 1" % (label, text.count(old)))
    return text.replace(old, new), True
def chain(text, edits):
    changed = False
    for label, old, new in edits:
        text, c = once(text, old, new, label); changed = changed or c
    return text, changed
