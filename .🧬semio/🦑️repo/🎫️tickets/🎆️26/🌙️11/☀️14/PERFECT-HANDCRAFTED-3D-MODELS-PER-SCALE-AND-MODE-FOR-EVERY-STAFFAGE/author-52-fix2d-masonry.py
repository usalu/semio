import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-52-asset.py")).read())


def f(a):
    for sh in a["representations"]["500"]["geometry"]["elevation"]:
        if sh["id"] == "field":
            assert "M 0.875 0.91667 L 2.125 0.91667 L 2.125 2.25 L 0.875 2.25 Z" in sh["d"]
            sh["d"] = sh["d"].replace("M 0.875 0.91667 L 2.125 0.91667 L 2.125 2.25 L 0.875 2.25 Z", "M 0.875 0.91667 L 0.875 2.25 L 2.125 2.25 L 2.125 0.91667 Z")


patch("masonry-wall-with-lintel", f)
