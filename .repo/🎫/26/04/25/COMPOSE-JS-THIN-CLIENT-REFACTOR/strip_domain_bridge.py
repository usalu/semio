# Remove diff bridges and asKitInstance-based exports; strip geometry+copy+paste block (1-based line numbers).
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
INDEX = ROOT / "compose/js/index.ts"


def main() -> None:
    lines = INDEX.read_text(encoding="utf-8").splitlines(keepends=True)
    n = len(lines)
    if n < 8000:
        raise SystemExit(f"unexpectedly short file: {n} lines")

    def cut(a1: int, b1: int) -> None:
        """1-based inclusive a1, b1; delete that range."""
        del lines[a1 - 1 : b1]
        print(f"cut 1-based [{a1}, {b1}] -> removed {b1 - a1 + 1} lines, now {len(lines)} total")

    # Apply in reverse line order (highest first) to preserve indices
    # Run after re-counting: use markers instead

    s = INDEX.read_text(encoding="utf-8")
    if "asKitInstance(kit).copyDesignOp" in s:
        a = s.index("export const copyDesign = (kit: KitLike,")
        b = s.index("export const PASTE_DESIGN_ANCHORING_KINDS")
        s = s[:a] + s[b:]
        print("removed copyDesign + leading comment block to PASTE_DESIGN_ANCHORING_KINDS")
    if "asKitInstance(kit).pasteDesignOp" in s:
        a = s.index("export const pasteDesign = (kit: KitLike,")
        b = s.index("// #endregion", a)
        s = s[:a] + s[b:]

    if "asKitInstance(kit).fixPieceInDesignDiff" in s:
        a = s.index("export const fixPieceInDesign =")
        b = s.index("\n", a) + 1
        s = s[:a] + s[b:]
        print("removed fixPieceInDesign")
    if "asKitInstance(kit).removePiecesAndConnectionsFromDesignOp" in s:
        a = s.index("export const removePiecesAndConnectionsFromDesign =")
        b = s.index("\n", a) + 1
        s = s[:a] + s[b:]
        print("removed removePiecesAndConnectionsFromDesign")

    for needle in [
        "export const getTypeDiff = (before: Type, after: Type): TypeDiff",
        "export const getPieceDiff = (before: Piece, after: Piece): PieceDiff",
        "export const getDesignDiff = (before: Design, after: Design): DesignDiff",
    ]:
        if needle in s:
            a = s.index(needle)
            b = a
            while b < len(s) and s[b] != ";":
                b += 1
            b += 1
            if b < len(s) and s[b] == "\n":
                b += 1
            s = s[:a] + s[b:]

    for name in [
        "applyTypeDiff",
        "mergeTypeDiff",
        "inverseTypeDiff",
        "inversePieceDiff",
        "mergePieceDiff",
        "applyPieceDiff",
        "mergeDesignDiff",
        "inverseDesignDiff",
    ]:
        pat = f"export const {name} ="
        if pat in s:
            a = s.index(pat)
            b = a
            depth = 0
            if "=>" in s[a : a + 200]:
                while b < len(s) and s[b] != ";":
                    b += 1
                b += 1
            else:
                while b < len(s):
                    if s[b] == "{":
                        depth += 1
                    elif s[b] == "}" and depth:
                        depth -= 1
                    b += 1
                    if depth == 0 and s[b - 1] == "}" and "export const" in s[a : b + 1]:
                        break
            if b < len(s) and s[b] == "\n":
                b += 1
            s = s[:a] + s[b:]
            print("removed", name)

    if "export const deletePiecesAndConnectionsInDesign = " in s:
        a = s.index("export const deletePiecesAndConnectionsInDesign =")
        b = s.index("\n", a) + 1
        s = s[:a] + s[b:]

    if "const connectionPlacementTranslationBasis = (parentConnector: Connector)" in s:
        a = s.index("const connectionPlacementTranslationBasis = (parentConnector: Connector)")
        # delete through line before PASTE_DESIGN or copyDesign
        b = s.find("export const PASTE_DESIGN_ANCHORING_KINDS", a)
        if b < 0:
            b = s.find("export const copyDesign =", a)
        if b < 0:
            raise SystemExit("marker after geometry block not found")
        s = s[:a] + s[b:]
        print("removed connectionPlacement/geometry run up to PASTE_DESIGN or copyDesign")

    if "\n * Live {@link KitImpl} or wire {@link KitData}" in s:
        a = s.index("/**\n * Live {@link KitImpl} or wire {@link KitData}")
        b = s.index("\n", a + 1)
        b = s.index("\n", b + 1) + 1
        s = s[:a] + s[b:]

    INDEX.write_text(s, encoding="utf-8")
    print("done strip_domain_bridge, bytes", len(s))


if __name__ == "__main__":
    main()
