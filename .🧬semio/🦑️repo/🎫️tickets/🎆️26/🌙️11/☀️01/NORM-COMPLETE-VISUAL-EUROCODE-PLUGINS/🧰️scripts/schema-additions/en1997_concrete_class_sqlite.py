"""en1997 `piles[].concreteClass` in the SQLite codec (sql, rs, ts, admission), idempotent; the contract numbers are replayed by `en1997_concrete_class_contract.py`."""
import re
Q = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot"
TOKENS = ["notGiven", "c1215", "c1620", "c2025", "c2530", "c3037", "c3545", "c4050", "c4555", "c5060"]
VARIANTS = ["NotGiven", "C1215", "C1620", "C2025", "C2530", "C3037", "C3545", "C4050", "C4555", "C5060"]


def sub(path, old, new, marker):
    t = open(path, encoding="utf8").read()
    if marker in t:
        return
    assert t.count(old) == 1, (path[-40:], old[:70], t.count(old))
    open(path, "w", encoding="utf8").write(t.replace(old, new))


quoted = ",".join("'%s'" % t for t in TOKENS)
sub(Q + "/🗄️.sql", " set_per_blow REAL, elastic_compression REAL, hammer_efficiency REAL,\n", " set_per_blow REAL, elastic_compression REAL, hammer_efficiency REAL, concrete_class TEXT NOT NULL CHECK(concrete_class IN (%s)),\n" % quoted, "concrete_class TEXT")
rs = Q + "/🦀️.rs"
sub(rs, "use crate::{SoilLayer, SpreadFoundation, FoundationLoadCase, Pile, PileTestProfile,", "use crate::{SoilLayer, SpreadFoundation, FoundationLoadCase, Pile, PileConcreteClass, PileTestProfile,", "Pile, PileConcreteClass")
enc = ",".join("PileConcreteClass::%s=>\"%s\"" % (v, t) for v, t in zip(VARIANTS, TOKENS))
dec = ",".join("\"%s\"=>Ok(PileConcreteClass::%s)" % (t, v) for v, t in zip(VARIANTS, TOKENS))
sub(rs, "fn checkpoint(c:&mut SqliteSnapshotControl", "fn concrete_class(a:PileConcreteClass)->&'static str{match a{%s}}\nfn read_concrete_class(a:&str)->Result<PileConcreteClass, ValueError>{match a{%s,_=>Err(invalid(\"EN1997 pile concrete class differs\"))}}\nfn checkpoint(c:&mut SqliteSnapshotControl" % (enc, dec), "fn read_concrete_class")
sub(rs, "Cell::Real(v.hammer_efficiency)],PILE)?", "Cell::Real(v.hammer_efficiency),Cell::Text(concrete_class(v.concrete_class))],PILE)?", "concrete_class(v.concrete_class)")
sub(rs, 'entities(d,"en1997_pile",29,PILE,c)', 'entities(d,"en1997_pile",30,PILE,c)', 'entities(d,"en1997_pile",30,')
sub(rs, "hammer_efficiency:v.real(28)?,test_profiles}", "hammer_efficiency:v.real(28)?,concrete_class:read_concrete_class(v.text(29)?)?,test_profiles}", "read_concrete_class(v.text(29)")
ts = Q + "/🟦️.ts"
members = "|".join('"%s"' % t for t in TOKENS)
cond = "&&".join('s!=="%s"' % t for t in TOKENS)
sub(ts, "function reliabilityClass(s:string)", 'function pileConcreteClass(s:string):%s{if(%s)throw Error("EN1997 pile concrete class differs");return s as %s}\nfunction reliabilityClass(s:string)' % (members, cond, members), "function pileConcreteClass")
sub(ts, '"elasticCompression","hammerEfficiency","testProfiles"]);collection(v.testProfiles)', '"elasticCompression","hammerEfficiency","concreteClass","testProfiles"]);collection(v.testProfiles)', '"hammerEfficiency","concreteClass"')
sub(ts, "v.elasticCompression,v.hammerEfficiency],PILE,o)", "v.elasticCompression,v.hammerEfficiency,pileConcreteClass(v.concreteClass)],PILE,o)", "pileConcreteClass(v.concreteClass)]")
sub(ts, "entities(t[4]!,29,PILE,o)", "entities(t[4]!,30,PILE,o)", "entities(t[4]!,30,PILE,o)")
sub(ts, "hammerEfficiency:readBinary64(v,28,PILE),testProfiles})", "hammerEfficiency:readBinary64(v,28,PILE),concreteClass:pileConcreteClass(artifactSqliteText(v,29)),testProfiles})", "concreteClass:pileConcreteClass(artifactSqliteText")
adm = Q + "/🛂️admission/🦀️.rs"
sub(adm, "Real(23),Real(24),Real(25)])?,PILE)?;", "Real(23),Real(24),Real(25),Enumeration(26,&[%s])])?,PILE)?;" % ",".join('"%s"' % t for t in TOKENS), "Enumeration(26,")
sub(adm, "for(index,a)in n::list(n::field(v,26)?)?", "for(index,a)in n::list(n::field(v,27)?)?", "n::field(v,27)")
