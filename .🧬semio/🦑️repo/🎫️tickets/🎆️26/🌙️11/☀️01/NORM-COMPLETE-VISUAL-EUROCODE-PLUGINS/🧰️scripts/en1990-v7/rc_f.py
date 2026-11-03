"""Task 1 layer F: SQLite codec (sql, rs, ts, admission, tests, contract) of the reliability class text column; K_FI column dropped."""
from lib import *
import sqlite3

def sql(t):
    if "k_fi_declared" not in t:
        return t
    t = rep(t, "reliability_class INTEGER NOT NULL CHECK(reliability_class>=0 AND reliability_class<=255)", "reliability_class TEXT NOT NULL CHECK(reliability_class IN ('RC1','RC2','RC3'))", count=t.count("reliability_class INTEGER NOT NULL CHECK(reliability_class>=0 AND reliability_class<=255)"))
    t = rep(t, " k_fi_declared REAL,", "", count=t.count(" k_fi_declared REAL,"))
    t = rep(t, " k_fi_declared_ieee754_bits INTEGER NOT NULL, k_fi_declared_ieee754_class TEXT NOT NULL,", "", count=t.count(" k_fi_declared_ieee754_bits INTEGER NOT NULL, k_fi_declared_ieee754_class TEXT NOT NULL,"))
    return t
edit(SQ + "/🗄️.sql", sql)

def rs(t):
    if "k_fi_declared" not in t:
        return t
    t = rep(t, "Cell::Integer(i64::from(self.reliability_class)), ", "Cell::Text(reliability(self.reliability_class)), ")
    t = rep(t, "Cell::Real(self.k_fi_declared), ", "")
    t = rep(t, "reliability_class: u8::try_from(document.integer(6)?).map_err(|error| invalid(error.to_string()))?, ", "reliability_class: read_reliability(document.text(6)?)?, ")
    t = rep(t, "k_fi_declared: document.real(12)?, beta_computed: document.real(13)?,", "beta_computed: document.real(12)?,")
    t = rep(t, "FloatColumn::Binary64(9), FloatColumn::Binary64(12), FloatColumn::Binary64(13)];", "FloatColumn::Binary64(9), FloatColumn::Binary64(12)];")
    t = rep(t, '"en1990_document", 14, DOCUMENT_FLOATS', '"en1990_document", 13, DOCUMENT_FLOATS')
    t = rep(t, "use crate::{AccidentalAction, BridgeSls, FatigueSpectrum, ImportanceClass, Member, MemberEffect, PermanentAction, ReliabilityAnalysis,", "use crate::{AccidentalAction, BridgeSls, FatigueSpectrum, ImportanceClass, Member, MemberEffect, PermanentAction, ReliabilityAnalysis, ReliabilityClass,")
    t = rep(t, "fn ordinal(value: usize)", 'fn reliability(value: ReliabilityClass) -> &\'static str { match value { ReliabilityClass::RC1 => "RC1", ReliabilityClass::RC2 => "RC2", ReliabilityClass::RC3 => "RC3" } }\nfn read_reliability(value: &str) -> Result<ReliabilityClass, ValueError> { match value { "RC1" => Ok(ReliabilityClass::RC1), "RC2" => Ok(ReliabilityClass::RC2), "RC3" => Ok(ReliabilityClass::RC3), _ => Err(invalid("EN1990 reliability class is not owned")) } }\nfn ordinal(value: usize)')
    return t
edit(SQ + "/🦀️.rs", rs)

def ts(t):
    if "kFiDeclared" not in t:
        return t
    t = rep(t, "u8(snapshot.reliabilityClass),", "snapshot.reliabilityClass,")
    t = rep(t, "snapshot.kFiDeclared,", "")
    t = rep(t, 'reliabilityClass:readU8(document,6),', 'reliabilityClass:reliability(artifactSqliteText(document,6)),')
    t = rep(t, "kFiDeclared:readBinary64(document,12,DOCUMENT_FLOATS),betaComputed:readBinary64(document,13,DOCUMENT_FLOATS),", "betaComputed:readBinary64(document,12,DOCUMENT_FLOATS),")
    t = rep(t, '|"kFiDeclared"|"betaComputed"', '|"betaComputed"')
    t = rep(t, "kFiDeclared:Binary64;", "")
    t = rep(t, "{index:9,width:64},{index:12,width:64},{index:13,width:64}]", "{index:9,width:64},{index:12,width:64}]")
    t = rep(t, "En1990ImportanceClass as ImportanceClass}", "En1990ImportanceClass as ImportanceClass,En1990ReliabilityClass as ReliabilityClass}")
    t = rep(t, "async function write(out:ArtifactSqliteProjection", 'function reliability(value:unknown):ReliabilityClass{if(value!=="RC1"&&value!=="RC2"&&value!=="RC3")throw Error("EN1990 reliability class is not owned");return value}\nasync function write(out:ArtifactSqliteProjection')
    return t
edit(SQ + "/🟦️.ts", ts)

def adm(t):
    if "Real(11),Real(12)])" not in t:
        return t
    t = rep(t, "Unsigned(4,u8::MAX as u64),Unsigned(5,u8::MAX as u64),Unsigned(6,u8::MAX as u64),Real(7),Real(8),Text(9),Text(10),Real(11),Real(12)])", 'Unsigned(4,u8::MAX as u64),Enumeration(5,&["RC1","RC2","RC3"]),Unsigned(6,u8::MAX as u64),Real(7),Real(8),Text(9),Text(10),Real(11)])')
    dec = lambda m: str(int(m.group(0)) - 1)
    t = re.sub(r"(?<=n::field\(root,)\d+(?=\))", dec, t)
    t = re.sub(r"(\d+)\.\.=(\d+)", lambda m: "%d..=%d" % (int(m.group(1)) - 1, int(m.group(2)) - 1), t)
    t = re.sub(r"(?<=\()1[3-6](?=,\"(?:permanent|variable|accidental|seismic)\")", dec, t)
    t = re.sub(r"(?<=[{,])1[3-6](?==>out\.insert_key_float)", dec, t)
    return t
edit(SQ + "/🛂️admission/🦀️.rs", adm)

edit(SQ + "/🧪️tests/🦀️.rs", lambda t: t.replace(" snapshot.k_fi_declared = value;", ""))
def tts(t):
    t = t.replace("kFiDeclared:binary64(f.kFiDeclared),", "").replace("s.kFiDeclared=value;", "")
    return t
edit(SQ + "/🧪️tests/🟦️.ts", tts)
edit(SQ + "/🧪️tests/🧫️fixtures/🛂️invalid-u8.json", lambda t: t.replace('"consequenceClass","reliabilityClass","designWorkingLifeCategory"', '"consequenceClass","designWorkingLifeCategory"'))

import importlib.util
spec = importlib.util.spec_from_file_location("ext", SEMIO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/en1990/sqlite_extent.py")
