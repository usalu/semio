"""Traffic groups layer B: SQLite (ddl, rs, ts, admission, tests, contract) and the empty `trafficGroups` in every JSON fixture."""
from tg_lib import *
import sqlite3
from tg_a import put
DDL = "CREATE TABLE %s(id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1990_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, group_key TEXT NOT NULL CHECK(group_key IN (%s)), variable_reference TEXT NOT NULL);" % (TABLE, ",".join("'%s'" % k for k in KEYS))
MATCH_W = ",".join('LoadGroup::%s=>"%s"' % (k, k) for k in KEYS)
MATCH_R = ",".join('"%s"=>Ok(LoadGroup::%s)' % (k, k) for k in KEYS)

def sql():
    edit(SQ + "/🗄️.sql", lambda t: t if TABLE in t else t.rstrip("\n") + "\n" + DDL + "\n")

def rs():
    def go(t):
        if "traffic_groups" in t:
            return t
        t = put(t, "FactorOverride, VariableAction,", "FactorOverride, LoadGroup, TrafficGroup, VariableAction,")
        write = '        for (position, row) in self.traffic_groups.iter().enumerate() {\n            out.insert_float("%s", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Text(load_group(row.group)), Cell::Text(&row.variable_id)], &[])?;\n        }\n' % TABLE
        t = put(t, "        Ok(())\n }\n fn admit_sqlite_values", write + "        Ok(())\n }\n fn admit_sqlite_values")
        t = rep(t, "SQLITE_SCHEMA,17,96,control)", "SQLITE_SCHEMA,18,96,control)")
        t = put(t, "self.cases.len(),self.factor_overrides.len()]{rows=add(rows,size)?}", "self.cases.len(),self.factor_overrides.len(),self.traffic_groups.len()]{rows=add(rows,size)?}")
        t = put(t, "        for case in &self.cases { bound.add(4096)?;", "        for row in &self.traffic_groups { bound.add(4096)?; for value in [&row.id, &row.variable_id] { bound.repeated(value.len(), 24)?; } }\n        for case in &self.cases { bound.add(4096)?;")
        t = put(t, '        let case_rows = entities(database, "en1990_case", 7, &[], control)?;\n', '        let traffic_rows = entities(database, "%s", 6, &[], control)?;\n        let case_rows = entities(database, "en1990_case", 7, &[], control)?;\n' % TABLE)
        read = '        let mut traffic_groups = Vec::new();\n        for row in ordered(traffic_rows.values().copied(), 2, control)? {\n            checkpoint(control, traffic_groups.len(), 0)?;\n            if row.integer(1)? != 1 { return Err(invalid("EN1990 traffic group has an unknown document")); }\n            traffic_groups.push(TrafficGroup { id: reconstruct_text(control, row.text(3)?)?, group: read_load_group(row.text(4)?)?, variable_id: reconstruct_text(control, row.text(5)?)? });\n        }\n'
        t = put(t, "        let result = Self { annex:", read + "        let result = Self { annex:")
        t = put(t, "cases, factor_overrides };", "cases, factor_overrides, traffic_groups };")
        return put(t, "fn ordinal(value: usize)", 'fn load_group(value: LoadGroup) -> &\'static str { match value { %s } }\nfn read_load_group(value: &str) -> Result<LoadGroup, ValueError> { match value { %s, _ => Err(invalid("EN1990 traffic load group is not owned")) } }\nfn ordinal(value: usize)' % (MATCH_W, MATCH_R))
    edit(SQ + "/🦀️.rs", go)

def ts():
    def go(t):
        if "trafficGroups" in t:
            return t
        w = ' for(let position=0;position<snapshot.trafficGroups.length;position++){const row=snapshot.trafficGroups[position]!;await write(out,"%s",BigInt(position+1),[1n,BigInt(position),row.id,row.group,row.variableId],[],options)}\n' % TABLE
        t = put(t, " return out.finish();\n}", w + " return out.finish();\n}")
        t = put(t, "const overrideRows=await entities(tables[16]!,8,FACTOR_OVERRIDE_FLOATS,options);", "const overrideRows=await entities(tables[16]!,8,FACTOR_OVERRIDE_FLOATS,options);const trafficRows=await entities(tables[17]!,6,[],options);")
        r = " const trafficGroups:TrafficGroup[]=[];for(const row of await ordered([...trafficRows.values()],options)){await checkpoint(options,trafficGroups.length,0);trafficGroups.push({id:artifactSqliteText(row,3),group:loadGroup(artifactSqliteText(row,4)),variableId:artifactSqliteText(row,5)})}\n"
        t = put(t, " const annex=artifactSqliteText(document,1);if(", r + " const annex=artifactSqliteText(document,1);if(")
        t = put(t, "testSeries,placements,cases,factorOverrides};", "testSeries,placements,cases,factorOverrides,trafficGroups};")
        t = put(t, "En1990FactorOverride as FactorOverride,", "En1990FactorOverride as FactorOverride,En1990TrafficGroup as TrafficGroup,")
        t = put(t, "async function write(out:ArtifactSqliteProjection", 'function loadGroup(value:unknown):TrafficGroup["group"]{if(%s)throw Error("EN1990 traffic load group is not owned");return value as TrafficGroup["group"]}\nasync function write(out:ArtifactSqliteProjection' % "&&".join('value!=="%s"' % k for k in KEYS))
        return t
    edit(SQ + "/🟦️.ts", go)

def admission():
    def go(t):
        if TABLE in t:
            return t
        t = rep(t, "for id in 16..=24{rows=n::add", "for id in 16..=25{rows=n::add")
        row = ' for(index,o)in n::list(n::field(root,25)?)?.iter().enumerate(){n::entity(out,"%s",1,index,&n::cells(n::record(o)?,[Text(0),Enumeration(1,&[%s]),Text(2)])?,&[])?;}\n' % (TABLE, ",".join('"%s"' % k for k in KEYS))
        t = put(t, " Ok(())\n}\n/// 🛂️ Seventeen SQL tables", row + " Ok(())\n}\n/// 🛂️ Eighteen SQL tables")
        return t.replace("SQLITE_SCHEMA,17,96", "SQLITE_SCHEMA,18,96").replace(",17,96,rows", ",18,96,rows")
    edit(SQ + "/🛂️admission/🦀️.rs", go)

def tests():
    def rs_(t):
        t = t.replace("tables.len(), 17)", "tables.len(), 18)").replace("empty.factor_overrides.clear();", "empty.factor_overrides.clear(); empty.traffic_groups.clear();") if "traffic_groups" not in t else t
        t = t.replace("source.factor_overrides.clear();", "source.factor_overrides.clear();source.traffic_groups.clear();") if "source.traffic_groups" not in t else t
        return t
    edit(SQ + "/🧪️tests/🦀️.rs", rs_)
    def ts_(t):
        t = t.replace("tables.length).toBe(17)", "tables.length).toBe(18)")
        t = t.replace("source.factorOverrides=[];", "source.factorOverrides=[];source.trafficGroups=[];") if "source.trafficGroups" not in t else t
        t = t.replace("cases:[],factorOverrides:[]}", "cases:[],factorOverrides:[],trafficGroups:[]}") if "trafficGroups:[]" not in t else t
        return t
    edit(SQ + "/🧪️tests/🟦️.ts", ts_)

def contract():
    src = rd(SEMIO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/en1990/sqlite_extent.py")
    db = sqlite3.connect(":memory:"); db.executescript(rd(SQ + "/🗄️.sql"))
    rows = db.execute("SELECT name, sql FROM sqlite_schema WHERE type='table' ORDER BY name").fetchall()
    widths = {n: len(db.execute('PRAGMA table_info("%s")' % n).fetchall()) for n, _ in rows}
    size = sum(len(n.encode()) + len(s.encode()) for n, s in rows)
    p = SQ + "/🧪️tests/🧫️fixtures/🎛️semantic.json"
    d = json.loads(rd(p)); d["schemaBytes"] = size; d["tableWidths"] = dict(sorted(widths.items()))
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    def ts(t):
        t = re.sub(r'(normSemanticContract\["schemaBytes"\]\)\.toEqual\()\d+\)', lambda m: m.group(1) + str(size) + ")", t)
        return re.sub(r'(normSemanticContract\["tableWidths"\]\)\.toEqual\()\{[^}]*\}\)', lambda m: m.group(1) + json.dumps(d["tableWidths"], separators=(",", ":")) + ")", t)
    edit(SQ + "/🧪️tests/🟦️.ts", ts)
    print("  schemaBytes", size, len(widths), "tables")

def fixtures():
    sys.path.insert(0, SEMIO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/placements-adder")
    from padd import value_end
    EMPTY = '{"removed": [], "inserted": [], "moved": [], "modified": []}'
    def insert(t):
        out, pos, hits = [], 0, 0
        for m in re.finditer(r'"factorOverrides"\s*:\s*', t):
            if m.start() < pos:
                continue
            s = m.end(); e = value_end(t, s); val = t[s:e].lstrip()
            if re.match(r'\s*,\s*"%s"' % CAMEL, t[e:e + 80]):
                continue
            new = "[]" if val.startswith("[") else (EMPTY if val.startswith("{") and '"removed"' in val else None)
            if new is None:
                continue
            ls = t.rfind("\n", 0, m.start()) + 1
            indent = re.match(r"\s*", t[ls:m.start()]).group(0) if t[ls:m.start()].strip() == "" else None
            sep = (",\n" + indent) if indent is not None else ", "
            colon = ": " if ": " in t[m.start():s] else ":"
            out.append(t[pos:e]); out.append('%s"%s"%s%s' % (sep, CAMEL, colon, new)); pos = e; hits += 1
        out.append(t[pos:])
        return "".join(out), hits
    count = 0
    for dp, dn, fn in os.walk(SUB):
        dn[:] = [d for d in dn if d not in ("target", "node_modules", "🧬️schema", "🎬️scene")]
        for f in fn:
            if not f.endswith(".json") or f == "🎛️semantic.json":
                continue
            p = os.path.join(dp, f); t = rd(p)
            if '"factorOverrides"' not in t:
                continue
            n, hits = insert(t)
            if hits:
                json.loads(n); wr(p, n); count += 1
    print("  fixtures updated:", count)

if __name__ == "__main__":
    sql(); rs(); ts(); admission(); tests(); contract(); fixtures()
