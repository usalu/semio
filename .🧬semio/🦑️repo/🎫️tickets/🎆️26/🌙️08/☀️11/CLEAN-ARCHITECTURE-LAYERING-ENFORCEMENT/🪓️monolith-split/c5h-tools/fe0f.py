import sys

FE0F = "️"


def pictographic(char: str) -> bool:
    code = ord(char)
    return 0x1F300 <= code <= 0x1FAFF or 0x2600 <= code <= 0x27BF or 0x2B50 <= code <= 0x2B55 or code in (0x2B1B, 0x2B1C, 0x231A, 0x231B, 0x23E9, 0x23EA, 0x23EB, 0x23EC, 0x23F0, 0x23F3)


for path in sys.argv[1:]:
    text = open(path, encoding="utf-8").read()
    out = []
    for index, char in enumerate(text):
        out.append(char)
        if pictographic(char) and text[index + 1 : index + 2] != FE0F:
            out.append(FE0F)
    fixed = "".join(out)
    if fixed != text:
        open(path, "w", encoding="utf-8").write(fixed)
        print("normalised", path)
