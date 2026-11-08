"""🧱 Emits third-party-blessed EN 1991 catalogue grid rows (wind profile, snow shapes, traffic dynamics) from the sympy/scipy closed forms; usage: print rows, append to the catalogue feature by hand once."""
import importlib.util, pathlib
B = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any")
spec = importlib.util.spec_from_file_location("cf", B / "🔮️oracles/🧮️closed-forms/🐍️.py"); cf = importlib.util.module_from_spec(spec); spec.loader.exec_module(cf)
rows = []
def add(slug, entry, annex, inputs, keys, tol="1e-6", sins=None):
    n = {k: v for k, v in inputs.items() if not isinstance(v, str)}
    s = {k: v for k, v in inputs.items() if isinstance(v, str)}
    got = cf.derive(entry, annex, n, s, lambda r, a: 2.0)
    text = ";".join(sorted([f"{k}={v:g}" for k, v in n.items()] + [f"{k}:{v}" for k, v in s.items()]))
    for k in keys:
        rows.append(f"      | {slug}-{k.lower().replace('_','-')} | {entry} | {annex} | {text} | {k} | {got[k]:.12g} | {tol} |")
for t in range(5):
    for z in (1.0, 5.0, 10.0, 25.0, 60.0, 150.0):
        add(f"oracle-e14-05-t{t}-z{z:g}", "E14.05", "en", {"z": z, "terrain": float(t), "v_b": 26.0, "rho": 1.25, "c_o": 1.0}, ["c_r", "I_v", "q_p"])
add("oracle-e14-05-oro", "E14.05", "en", {"z": 40.0, "terrain": 3.0, "v_b": 24.0, "rho": 1.226, "c_o": 1.15}, ["k_r", "v_m", "q_p"])
for p in (0.5, 0.2, 0.1, 0.02, 0.01, 0.005): add(f"oracle-e14-06-p{p:g}", "E14.06", "en", {"p": p}, ["c_prob"])
for a in (0, 10, 25, 30, 35, 45, 59, 60, 75): add(f"oracle-e13-03-a{a}", "E13.03", "de", {"alpha": float(a)}, ["mu1"])
for hb in ((2, 12), (1, 20), (3, 10)): add(f"oracle-e13-05-h{hb[0]}-b{hb[1]}", "E13.05", "de", {"h": float(hb[0]), "b": float(hb[1])}, ["mu3"])
for al in (0, 15, 30, 45, 60): add(f"oracle-e13-08-a{al}", "E13.08", "de", {"s": 1200.0, "b": 5.0, "alpha": float(al)}, ["F_s"], "1e-9")
for s in (400.0, 900.0, 2000.0): add(f"oracle-e13-07-s{s:g}", "E13.07", "de", {"s": s}, ["s_e"], "1e-9")
for v, y in ((0.2, 10.0), (0.4, 100.0), (0.4, 5.0)): add(f"oracle-e13-10-v{v:g}-n{y:g}", "E13.10", "de", {"v": v, "n_years": y}, ["factor"], "2e-5")
for z in (1.0, 2.0, 3.0, 4.0): add(f"oracle-e14-01-zone{z:g}", "E14.01", "de", {"zone": z}, ["q_b"], "2e-4")
for ph in (0.02, 0.1, 0.25, 0.4): add(f"oracle-e14-03-phi{ph:g}", "E14.03", "de", {"phi": ph, "s": 1.5}, ["c_o"])
for l in (2.0, 4.0, 10.0, 25.0, 60.0, 120.0): add(f"oracle-e2-16-l{l:g}", "E2.16", "de", {"l_phi": l}, ["phi_2", "phi_3"])
for r in (150.0, 200.0, 800.0, 1500.0, 2000.0): add(f"oracle-e2-08-r{r:g}", "E2.08", "de", {"q_v": 900000.0, "radius": r}, ["Q_tk"], "1e-9")
for sp_ in (5.0, 20.0, 60.0, 200.0): add(f"oracle-e2-12-span{sp_:g}", "E2.12", "de", {"span": sp_, "width": 3.0}, ["q_fk"])
for L in (3.0, 10.0, 25.0, 60.0, 400.0): add(f"oracle-e2-07-l{L:g}", "E2.07", "de", {"loaded_length": L}, ["Q_lk"], "1e-9")
for v in (100.0, 120.0, 160.0, 250.0): add(f"oracle-e2-17-v{v:g}", "E2.17", "de", {"v_kmh": v, "radius": 1500.0, "l_f": 30.0, "q_vk": 1e6}, ["f", "Q_tk"], "1e-6")
for h in ("HC1", "HC2", "HC3", "HC4"): add(f"oracle-e3-01-{h.lower()}", "E3.01", "de", {"hoist_class": h, "v_h": 0.4}, ["phi_2"], "1e-9")
print("\n".join(rows))
