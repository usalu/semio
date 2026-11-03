"""🔎️ List on-screen text producers (Text, MathTex, labels, captions) with their argument kind, per modified clip file, and the language-pair constants defined in each file."""
import ast
import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio")
GEN = ROOT / ".🧬semio" / "🦑️repo" / "🎫️tickets" / "🎆️26" / "🌙️11" / "☀️01" / "HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS" / "🗑️generated" / "ct"
TEXT_FUNCS = {"Text", "MathTex", "Tex", "MarkupText", "Paragraph", "math_text", "math_label", "math_readout",
              "caption_bar", "swap_caption", "section_heading", "subtitle_text", "math_panel", "boxed_label",
              "chip", "readout", "label", "annotate", "text_label", "hand_over_chip", "de_num", "Title"}
CALLS = {"staffage", "staffage_motion", "fitted_staffage", "pendant_lamp"}


def fname(node):
    f = node.func
    return f.id if isinstance(f, ast.Name) else getattr(f, "attr", "")


def kind(arg):
    if isinstance(arg, ast.Constant) and isinstance(arg.value, str):
        return "LIT"
    if isinstance(arg, ast.JoinedStr):
        return "FSTR"
    if isinstance(arg, ast.Name):
        return f"VAR:{arg.id}"
    if isinstance(arg, ast.Subscript):
        return "SUB:" + ast.unparse(arg)[:60]
    if isinstance(arg, ast.Call):
        return "CALL:" + fname(arg)
    if isinstance(arg, ast.IfExp):
        return "IFEXP:" + ast.unparse(arg)[:70]
    if isinstance(arg, ast.BinOp):
        return "BINOP:" + ast.unparse(arg)[:70]
    return type(arg).__name__


def main():
    files = [l.strip() for l in (GEN / "staffage-files.txt").read_text(encoding="utf-8").splitlines() if l.strip()]
    out = []
    for rel in files:
        src = (ROOT / rel).read_text(encoding="utf-8")
        tree = ast.parse(src)
        calls = [n.lineno for n in ast.walk(tree) if isinstance(n, ast.Call) and fname(n) in CALLS]
        out.append(f"## {rel}  (staffage call lines: {sorted(set(calls))})")
        for node in ast.walk(tree):
            if isinstance(node, ast.Assign) and len(node.targets) == 1 and isinstance(node.targets[0], ast.Name):
                n = node.targets[0].id
                if n.endswith(("_DE", "_EN", "_de", "_en")) or n in {"LABELS", "STRINGS", "TEXT", "I18N", "COPY"} or "de" in n.lower().split("_") or "en" in n.lower().split("_"):
                    if isinstance(node.value, (ast.Dict, ast.Constant, ast.Tuple)):
                        out.append(f"  PAIR-DEF L{node.lineno} {n} = {ast.unparse(node.value)[:200]}")
        for node in ast.walk(tree):
            if isinstance(node, ast.Call) and fname(node) in TEXT_FUNCS:
                arg0 = node.args[0] if node.args else None
                if arg0 is None:
                    continue
                k = kind(arg0)
                near = any(abs(node.lineno - c) <= 40 for c in calls)
                txt = ast.unparse(arg0)[:150].replace("\n", " ")
                out.append(f"  {'NEAR' if near else '    '} L{node.lineno} {fname(node)} [{k}] {txt}")
        out.append("")
    (GEN / "onscreen.txt").write_text("\n".join(out), encoding="utf-8")
    print(len(out), "lines")


if __name__ == "__main__":
    sys.exit(main())
