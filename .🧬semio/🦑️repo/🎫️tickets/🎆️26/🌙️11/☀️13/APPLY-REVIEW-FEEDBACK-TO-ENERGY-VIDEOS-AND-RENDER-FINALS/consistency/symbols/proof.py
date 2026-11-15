"""📊 Final grep proof: count every forbidden token across the energy tree, listing the files that still hold one."""
import collections
import os
import re
import sys

ROOT = r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy"
SKIP = {".venv", "site-packages", "__pycache__", ".ruff_cache"}
EXT = {".py", ".md", ".json", ".ts"}
QD = "\u0307"
TOKENS = {
    "mathrm{I} index (upper-case I as internal-gains index)": r"\\mathrm\{\{?I(?:,[a-z]+)?\}",
    "Q_I / Q̇_I": r"Q" + QD + r"?_\{?I\b(?!_)",
    "Phi_V / Phi_T": r"\\Phi_\{?\\?m?a?t?h?r?m?\{?[VT]\b|Φ_[VT]\b",
    "Q̇_L / Q̇_K (dot Q L, K)": r"Q" + QD + r"_[LK]\b|\\dot\{Q\}_\{?\\?m?a?t?h?r?m?\{?[LK]\b",
    "F_sh / F_V / F_f": r"F_\{?\\?m?a?t?h?r?m?\{?(?:sh|V|f)\b",
    "R_ges": r"R_\{?\\?m?a?t?h?r?m?\{?ges",
    "vartheta (shown)": r"ϑ|\\vartheta",
    "Delta T": r"ΔT|\\Delta\s?T\b",
    "G_t20/15 variants": r"G_t20/15|G_\{\\mathrm\{t20|G_\{\\mathrm\{t\}\}|\bGt20/15",
    "bare G_t (not G_{t,20/15})": r"\bG_t\b(?!,)|G_\{t\}",
    "F_{Gt} upright": r"F_\{\\mathrm\{Gt\}\}",
    "e_P": r"\be_\{?P\b",
    "e_{\\mathrm{E}}": r"e_\{\\mathrm\{E\}\}",
    "Pettenkoferwert": r"Pettenkoferwert",
    "I_S,max": r"I_S,max|I_\{S,max",
    "Q_Licht / Q_Pers / Q_Geraete": r"Q" + QD + r"?_(?:Licht|Pers|Geräte)",
    "lights up / leuchtet in narration": r"each facade lights up",
}
FIXTURE_HINT = ("fonts", "lexicon", "guard", "tex")


def main():
    counts = collections.Counter()
    where = collections.defaultdict(collections.Counter)
    pats = {k: re.compile(v) for k, v in TOKENS.items()}
    for d, ds, fs in os.walk(ROOT):
        ds[:] = [x for x in ds if x not in SKIP]
        for f in fs:
            if os.path.splitext(f)[1] not in EXT:
                continue
            path = os.path.join(d, f)
            try:
                text = open(path, encoding="utf-8").read()
            except Exception:
                continue
            rel = os.path.relpath(path, ROOT)
            for key, pat in pats.items():
                n = len(pat.findall(text))
                if n:
                    counts[key] += n
                    where[key][rel[:90]] += n
    sys.stdout.reconfigure(encoding="utf-8")
    for key in TOKENS:
        print(f"{counts[key]:4d}  {key}")
        for rel, n in where[key].most_common(8):
            print(f"        {n:3d}  {rel}")


main()
