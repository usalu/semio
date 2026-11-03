"""Runs the shared vocabulary generator with extra unit tokens. Usage: python3 -I gen_inputs.py OUT_DIR SOURCE_TSV ARTIFACT"""
import pathlib, sys
GEN = pathlib.Path(__file__).resolve().parent.parent / "catalogue-bridge-gen-inputs.py"
EXTRA = {"per_m": ("PerMetre", "1/m"), "mm6": ("MillimetreSixth", "mm⁶"), "g0": ("StandardGravity", "g₀"), "kNm_": ("KilonewtonMetre", "kN·m"), "N_mm2": ("NewtonPerSquareMillimetre", "N/mm²"), "kg_m3": ("KilogramPerCubicMetre", "kg/m³"), "kN_m": ("KilonewtonPerMetre", "kN/m"), "cm": ("Centimetre", "cm"), "cm2": ("SquareCentimetre", "cm²"), "cm3": ("CubicCentimetre", "cm³"), "cm4": ("CentimetreFourth", "cm⁴"), "pct": ("Percent", "%"), "kN_m2": ("KilonewtonPerSquareMetre", "kN/m²"), "N_mm3": ("NewtonPerMillimetre", "N/mm")}
src = GEN.read_text(encoding="utf-8")
src = src.replace("UNITS = {", "UNITS = {" + "".join(f'"{k}": "{v[0]}", ' for k, v in EXTRA.items()), 1)
src = src.replace("SYMBOLS = {", "SYMBOLS = {" + "".join(f'"{k}": "{v[1]}", ' for k, v in EXTRA.items()), 1)
sys.argv[0] = str(GEN)
exec(compile(src, str(GEN), "exec"), {"__file__": str(GEN), "__name__": "__main__"})
