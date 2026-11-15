import sys

path = sys.argv[1]
lines = open(path, encoding="utf-8", newline="").read().split("\n")
for number, var in ((501, "h_t"), (505, "h_v")):
    assert "label_number(" + var in lines[number - 1], lines[number - 1]
    lines[number - 1] = (
        '                    rf"{label_number(' + var + '.get_value())}\\,\\frac{{\\mathrm{{W}}}}{{\\mathrm{{K}}}}",'
    )
open(path, "w", encoding="utf-8", newline="").write("\n".join(lines))
