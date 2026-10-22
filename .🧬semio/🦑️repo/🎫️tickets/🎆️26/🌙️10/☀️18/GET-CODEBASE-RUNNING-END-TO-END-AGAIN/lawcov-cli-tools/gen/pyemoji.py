PY_EMOJI = r'''
RANGES = [(0xA9, 0xA9), (0xAE, 0xAE), (0x203C, 0x203C), (0x2049, 0x2049), (0x2122, 0x2122), (0x2139, 0x2139), (0x2194, 0x21AA), (0x231A, 0x23FF), (0x24C2, 0x24C2), (0x25AA, 0x25FE), (0x2600, 0x27BF), (0x2934, 0x2935), (0x2B05, 0x2B55), (0x3030, 0x3030), (0x303D, 0x303D), (0x3297, 0x3299), (0x1F000, 0x1FAFF)]


def pict(ch):
    cp = ord(ch)
    return any(a <= cp <= b for a, b in RANGES)


def lead(text):
    i, n = 0, len(text)
    out = 0
    if i >= n or not pict(text[i]):
        return ""
    i += 1
    while True:
        if i < n and text[i] == "️":
            i += 1
        elif i + 1 < n and text[i] == "‍" and pict(text[i + 1]):
            i += 2
        else:
            break
    return text[:i]


def lead_cluster_seq(text):
    total = ""
    while True:
        c = lead(text[len(total):])
        if not c:
            return total
        total += c
'''
