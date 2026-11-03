import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
SPEC = "en1994-en1999-target.md"
UNV = "recalled, NOT checked against the printed standard (the repo holds no copy); verification is a human double-key"
def row(key, part, clause, symbol, v, unit, note, evidence):
    i = put(d, "1999", key, clause, symbol, v, v, unit, note, part=part)
    d[i]["source"]["document"] = f"DIN EN 1999-{part}/NA"
    d[i]["specRef"] = "wave3 en1999 norm"
    d[i]["evidence"] = evidence
S5 = "EN 1999-1-5 §"
row("shell-sigma-x-factor", "1-5", S5 + "6 (axial, medium cylinder)", "0.605", 0.605, "", "σ_x,Rcr = 0.605·E·C_x·t/r of a medium-length cylinder (C_x = 1)", f"Given by the project spec ({SPEC} A-70: sigma_x,Rcr = 0.605 E t / r; example E2-11 254.1 MPa); the printed value is still to be double-keyed.")
row("shell-sigma-theta-factor", "1-5", S5 + "7 (circumferential, medium cylinder)", "0.92", 0.92, "", "σ_θ,Rcr = 0.92·E·(t/r)/ω of a medium-length cylinder under external pressure (C_θ = 1)", f"The factor is not in the project spec (A-70 names the circumferential case only as 'analogous'); {UNV}.")
row("shell-tau-factor", "1-5", S5 + "8 (shear, medium cylinder)", "0.75", 0.75, "", "τ_Rcr = 0.75·E·(t/r)/√ω of a medium-length cylinder in torsion or transverse shear (C_τ = 1)", f"The factor is not in the project spec; {UNV}.")
for key, sym, v, what in (("axial-beta", "β_x", 0.60, "plastic range factor of χ_x"), ("axial-eta", "η_x", 1.0, "plastic range exponent of χ_x"), ("axial-lambda-0", "λ̄_x0", 0.20, "squash limit slenderness of χ_x"),
                         ("hoop-shear-beta", "β_θ,τ", 0.60, "plastic range factor of χ_θ and χ_τ"), ("hoop-shear-eta", "η_θ,τ", 1.0, "plastic range exponent of χ_θ and χ_τ"), ("hoop-shear-lambda-0", "λ̄_θ0,τ0", 0.40, "squash limit slenderness of χ_θ and χ_τ")):
    row(f"shell-chi-{key}", "1-5", S5 + "5.3 (buckling reduction)", sym, v, "", f"Shell buckling reduction χ = 1 (λ̄ ≤ λ̄0), 1 − β·((λ̄ − λ̄0)/(λ̄p − λ̄0))^η, α/λ̄² (λ̄ ≥ λ̄p): {what}", f"The project spec (A-70) names the three-branch χ without these constants; {UNV}.")
for key, sym, v, what in (("k-base", "k_x,θ base", 1.25, "constant of the exponents k_x = k_θ = 1.25 + 0.75·χ"), ("k-slope", "k_x,θ slope", 0.75, "slope of the exponents k_x = k_θ = 1.25 + 0.75·χ"),
                         ("ktau-base", "k_τ base", 1.75, "constant of the exponent k_τ = 1.75 + 0.25·χ_τ"), ("ktau-slope", "k_τ slope", 0.25, "slope of the exponent k_τ = 1.75 + 0.25·χ_τ")):
    row(f"shell-interaction-{key}", "1-5", S5 + "8 (interaction of membrane stresses)", sym, v, "", f"Shell interaction (σ_x/σ_x,Rd)^k_x − k_i·(σ_x/σ_x,Rd)(σ_θ/σ_θ,Rd) + (σ_θ/σ_θ,Rd)^k_θ + (τ/τ_Rd)^k_τ: {what}", f"The project spec (A-70) names 'interaction exponents (cl. 8)' without values; {UNV}.")
row("sheet-web-shear-factor", "1-4", "EN 1999-1-4 §6.1.5 (web shear buckling)", "0.35", 0.35, "", "λ̄_w = factor·(s_w/t)·√(f_o/E) of an unstiffened sheeting web", f"Same factor the project spec gives for plate-girder webs (A-25, EN 1999-1-1 6.7.4: lambda_w = 0.35 (h_w/t_w) sqrt(f_o/E)); the EN 1999-1-4 rule for sheeting webs is not in the spec ('per lambda_w [T]') and is applied here by analogy; {UNV}.")
row("sheet-web-shear-limit", "1-4", "EN 1999-1-4 §6.1.5 (web shear buckling)", "λ̄_w,lim", 0.83, "", "ρ_v = 1 for λ̄_w ≤ limit, limit/λ̄_w above, of an unstiffened sheeting web", f"Same limit the repo applies to plate-girder webs (EN 1999-1-1 6.7.4 rho_v = 0.83/lambda_w); the EN 1999-1-4 value for sheeting webs is not in the spec and is applied by analogy; {UNV}.")
S2 = "EN 1999-1-2 §4.3.4 (protected members)"
row("fire-protected-denominator", "1-2", S2, "k_1", 3.0, "", "Temperature rise of a protected member Δθ_a = [λ_p/d_p·(A_p/V)/(c_a·ρ_a)]·(θ_g − θ_a)/(1 + φ/k_1)·Δt − (e^{φ/k_2} − 1)·Δθ_g: the divisor k_1 of φ", f"The project spec (A-51) gives only φ = (c_p·ρ_p/(c_a·ρ_a))·d_p·A_p/V; the divisor 3 is the EN 1993-1-2 4.2.5.2 structure recalled for the aluminium rule; {UNV}.")
row("fire-protected-exponent", "1-2", S2, "k_2", 10.0, "", "Temperature rise of a protected member: the divisor k_2 of φ in the exponential term e^{φ/k_2} − 1", f"The project spec (A-51) gives only φ; the divisor 10 is the EN 1993-1-2 4.2.5.2 structure recalled for the aluminium rule; {UNV}.")
save(d)
print(len(d))
