import re, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any")
T = ROOT / "🧬️schema/💡️inferences/⚖️checks/📜️ndp-tables/🦀️.rs"
s = T.read_text()
rx = re.compile(r'    psi_entry\("([^"]+)", ("[^"]*"), ("[^"]*"), psi\(([-\d.]+), ([-\d.]+), ([-\d.]+)\), psi\(([-\d.]+), ([-\d.]+), ([-\d.]+)\), ("[^"]*")\),\n')
rows = rx.findall(s)
assert len(rows) == 31, len(rows)
d = load()
for key, le, ld, e0, e1, e2, d0, d1, d2, table in rows:
    for n, (en, de) in enumerate([(e0, d0), (e1, d1), (e2, d2)]):
        put(d, "1990", f"psi.{key}.{n}", f"Table {table.strip(chr(34))}", f"ψ{n} {key}", float(en), float(de), "", f"ψ{n} of {le.strip(chr(34))}: EN {en} / DE {de}")
save(d)
s = rx.sub(lambda m: f'    psi_entry("{m[1]}", {m[2]}, {m[3]}, {m[10]}),\n', s)
s = s.replace('''/// 📊️ One ψ table row: category key, EN-recommended row, NA-DE row, source table and verification state.
#[derive(Clone, Copy, Debug)]
pub struct PsiEntry {
    pub key: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub en: PsiRow,
    pub de: PsiRow,
    pub table: &'static str,
}

const fn psi_entry(key: &'static str, label_en: &'static str, label_de: &'static str, en: PsiRow, de: PsiRow, table: &'static str) -> PsiEntry {
    PsiEntry { key, label_en, label_de, en, de, table }
}
''', '''/// 📊️ One ψ table row: category key, labels and source table; the values are the registry entries `ndp.en1990.psi.<key>.<0|1|2>`.
#[derive(Clone, Copy, Debug)]
pub struct PsiEntry {
    pub key: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub table: &'static str,
}

const fn psi_entry(key: &'static str, label_en: &'static str, label_de: &'static str, table: &'static str) -> PsiEntry {
    PsiEntry { key, label_en, label_de, table }
}

impl PsiEntry {
    fn id(&self, index: u8) -> String {
        format!("ndp.en1990.psi.{}.{index}", self.key)
    }

    /// 🔢️ The ψ0, ψ1, ψ2 row of the annex, read from the shared registry.
    pub fn row(&self, annex: AnnexChoice) -> PsiRow {
        PsiRow { psi_0: ndp(annex, &self.id(0)), psi_1: ndp(annex, &self.id(1)), psi_2: ndp(annex, &self.id(2)) }
    }

    /// 🛡️ Whether all three registry values are verified against the printed standard.
    pub fn verified(&self) -> bool {
        (0..3).all(|index| semio_s_plugin_norm_compliance::ndp_is_verified(&self.id(index)))
    }
}
''')
s = s.replace("psi_entry_of(category).map(|entry| if annex == AnnexChoice::De { entry.de } else { entry.en })", "psi_entry_of(category).map(|entry| entry.row(annex))")
s = s.replace("const fn psi(psi_0: f64, psi_1: f64, psi_2: f64) -> PsiRow {\n    PsiRow { psi_0, psi_1, psi_2 }\n}\n\n", "")
T.write_text(s)
def edit(p, a, b):
    t = p.read_text(); assert t.count(a) == 1, (p, a); p.write_text(t.replace(a, b))
edit(ROOT / "🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🦀️.rs", "let row = if c.annex == AnnexChoice::De { entry.de } else { entry.en };", "let row = entry.row(c.annex);")
edit(ROOT / "🧬️schema/🧪️tests/⚖️compliance/🦀️.rs", "assert!(psi_is_ordered(entry.en) && psi_is_ordered(entry.de), \"{}\", entry.key);", "assert!(psi_is_ordered(entry.row(AnnexChoice::En)) && psi_is_ordered(entry.row(AnnexChoice::De)), \"{}\", entry.key);")
edit(ROOT / "🎬️scene/⚓️actions/🦀️.rs", "is_some_and(|e| !e.verified)", "is_some_and(|e| !e.verified())")
