import pathlib

ASSETS = pathlib.Path(r"c:\git\compose\compose\assets")
PRED = pathlib.Path(r"c:\git\compose\compose\client\schema\json\design-prediction.json")


def patch_file(p: pathlib.Path) -> bool:
    t = p.read_text(encoding="utf-8")
    nt = t.replace('"connected":', '"parent":').replace('"connecting":', '"child":')
    if p == PRED:
        nt = nt.replace('"connected"', '"parent"').replace('"connecting"', '"child"')
    if nt != t:
        p.write_text(nt, encoding="utf-8", newline="\n")
        return True
    return False


def main() -> None:
    n = 0
    for p in ASSETS.rglob("*.json"):
        if patch_file(p):
            n += 1
    if PRED.is_file() and patch_file(PRED):
        n += 1
    print("updated", n, "files")


if __name__ == "__main__":
    main()
