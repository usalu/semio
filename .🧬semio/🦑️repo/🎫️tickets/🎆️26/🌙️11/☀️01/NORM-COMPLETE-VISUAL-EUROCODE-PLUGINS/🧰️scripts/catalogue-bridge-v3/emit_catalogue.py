"""Emits the cases file and the unit tests of a v3 catalogue from the EN 1993 template. Usage: python3 -I emit_catalogue.py CODE (1993|1994|1998|1999) CATALOGUE_DIR SPEC_PY ; SPEC_PY defines ENTRIES = [(id, part, clause, en, de, [inputs], fn, test_name)]"""
import pathlib, runpy, sys
code, d, spec = sys.argv[1], pathlib.Path(sys.argv[2]), runpy.run_path(sys.argv[3])
T93 = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📇️catalogue")
def q(s): return '"' + s + '"'
rows = "\n".join('    Entry { id: %s, part: %s, clause: %s, en: %s, de: %s, inputs: &[%s], eval: evaluators::%s },' % (q(i), q(p), q(c), q(e), q(g), ", ".join(q(k) for k in ins), f) for i, p, c, e, g, ins, f, _ in spec["ENTRIES"])
s = (T93 / "🦀️.rs").read_text(encoding="utf-8")
a, b = s.index("pub static ENTRIES: &[Entry] = &[\n") + len("pub static ENTRIES: &[Entry] = &[\n"), s.index("\n];\n", s.index("pub static ENTRIES"))
s = s[:a] + rows + s[b:]
s = s.replace("En1993", "En" + code).replace("EN 1993", "EN " + code).replace("en1993.", "en" + code + ".")
s = s.replace("the EN %s clause functions that no snapshot entity reaches" % code, spec["SUMMARY"])
(d / "🦀️.rs").write_text(s, encoding="utf-8")
t = (T93 / "🧪️tests/🔬️unit/🦀️.rs").read_text(encoding="utf-8")
a, b = t.index("entry_tests! {\n") + len("entry_tests! {\n"), t.index("\n}\n", t.index("entry_tests! {"))
t = t[:a] + "\n".join("    %s => %s," % (n, q(i)) for i, _, _, _, _, _, _, n in spec["ENTRIES"]) + t[b:]
t = t.replace("En1993", "En" + code).replace("C93-01", spec["ENTRIES"][0][0]).replace("p_plate", spec["MISSING"])
(d / "🧪️tests/🔬️unit").mkdir(parents=True, exist_ok=True)
(d / "🧪️tests/🔬️unit/🦀️.rs").write_text(t, encoding="utf-8")
print(len(spec["ENTRIES"]), "entries")
