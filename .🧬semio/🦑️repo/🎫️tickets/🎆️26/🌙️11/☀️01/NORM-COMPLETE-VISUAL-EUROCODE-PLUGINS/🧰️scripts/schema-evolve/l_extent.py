"""📏 Independent logical extent of a sample document: the committed SQLite fixture is projected row by row into a python `sqlite3` database created from the committed DDL (so every CHECK, NOT NULL and key constraint is enforced by sqlite itself) and measured with the queries of the framework test `independentSqliteExtent` (rows, value bytes by `typeof`, schema bytes, table widths). The projection follows column names and the schema, never an artifact."""
import collections, json, re, sqlite3, struct
from core import Names, Unsupported, camel, singular_of, snake
import l_codec

SQLITE = l_codec.SQLITE
MISSING = object()


def sample_document(ctx):
    path = ctx.layout.p(SQLITE + "/🧪️tests/🧫️fixtures/🔣️.json")
    doc = json.loads(ctx.plan.read(path)) if ctx.plan.exists(path) else None
    if isinstance(doc, dict) and isinstance(doc.get("snapshot"), dict):
        doc = doc["snapshot"]
    return doc


def independent_extent(db):
    """🔬 Python port of `independentSqliteExtent` (bun:sqlite in the framework test): integrity, row counts, value bytes by `typeof`, schema bytes and table widths."""
    if db.execute("PRAGMA integrity_check").fetchone()[0] != "ok" or db.execute("PRAGMA foreign_key_check").fetchall():
        raise Unsupported("independent database integrity")
    rows = values = schema = 0
    widths = {}
    for name, sql in db.execute("SELECT name,sql FROM sqlite_schema WHERE type='table' ORDER BY name").fetchall():
        quoted = '"' + name.replace('"', '""') + '"'
        columns = [c[1] for c in db.execute(f"PRAGMA table_info({quoted})")]
        widths[name] = len(columns)
        schema += len(name.encode()) + len(sql.encode())
        rows += db.execute(f"SELECT count(*) FROM {quoted}").fetchone()[0]
        for column in columns:
            ident = '"' + column.replace('"', '""') + '"'
            values += db.execute(f"SELECT coalesce(sum(CASE typeof({ident}) WHEN 'null' THEN 0 WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST({ident} AS BLOB)) WHEN 'blob' THEN length({ident}) END),0) FROM {quoted}").fetchone()[0]
    return {"rows": rows, "valueBytes": values, "schemaBytes": schema, "tableWidths": widths}


def fk_of(sql, table):
    m = re.search(r"CREATE TABLE\s+%s\s*\((.*?)\);" % re.escape(table), sql, re.S)
    return {col: ref for col, ref in re.findall(r"(\w+)\s+INTEGER[^,]*?REFERENCES\s+(\w+)\(id\)", m.group(1))}


def bits_of(value):
    return struct.unpack("<q", struct.pack("<d", float(value)))[0]


class Projector:
    """🧮 Rows per table: the entity walk of the schema over the sample document, inserted into a sqlite database."""

    def __init__(self, ctx, doc, sql, empty=()):
        self.ctx, self.doc, self.sql, self.empty = ctx, doc, sql, set(empty)
        self.tables = l_codec.tables_of(sql)
        self.db = sqlite3.connect(":memory:")
        self.db.executescript(sql)
        self.cols = {t: [r[1] for r in self.db.execute(f"pragma table_info({t})")] for t in self.tables}
        self.fks = {t: fk_of(sql, t) for t in self.tables}
        self.next_id = collections.Counter()
        self.unresolved = {}
        self.consumed = set()
        self.known = None
        self.seen = collections.defaultdict(dict)

    def resolve(self, name, key):
        """🔗 Row id of the logical id `key` in table `name` when it is unique, else None (NULL cell)."""
        table = self.known.get(name, {}) if self.known else {}
        ids = table.get(key, [])
        return ids[0] if len(ids) == 1 else None

    def value(self, table, col, row):
        m = re.match(r"resolved_(\w+?)_id$", col)
        if m:
            for k in (camel(m.group(1)) + "Id", camel(m.group(1)), camel(m.group(1)) + "Ref"):
                if k in row:
                    return self.resolve(m.group(1), row[k])
            return None
        key = camel(col)
        if key in row:
            return row[key]
        for prefix in sorted((k for k, v in row.items() if isinstance(v, dict)), key=len, reverse=True):
            p = snake(prefix)
            if col.startswith(p + "_") and camel(col[len(p) + 1:]) in row[prefix]:
                return row[prefix][camel(col[len(p) + 1:])]
        alias = camel(re.sub(r"_(?:reference|ref)$", "_id", col))
        if alias in row:
            return row[alias]
        used = {camel(c) for c in self.cols[table]}
        loose = [k for k, v in row.items() if not isinstance(v, (dict, list)) and k not in used and (col.endswith("_" + snake(k)) or col.startswith(snake(k) + "_"))]
        if loose:
            return row[max(loose, key=len)]
        if col in ("label_en", "label_de") and isinstance(row.get("label"), dict):
            return row["label"].get(col[-2:])
        self.unresolved.setdefault(table, set()).add(col)
        return None

    def insert(self, table, row, parent_id, ordinal, fixed_id=None):
        rid = fixed_id if fixed_id is not None else self.next_id[table] + 1
        self.next_id[table] = max(self.next_id[table], rid)
        cells = {}
        for col in self.cols[table]:
            if col == "id":
                cells[col] = rid
            elif re.search(r"_ieee754_bits$", col):
                base = cells.get(col[: -len("_ieee754_bits")])
                cells[col] = bits_of(base) if base is not None else 0
            elif re.search(r"_ieee754_class$", col):
                cells[col] = "finite"
            elif col.startswith("resolved_"):
                cells[col] = self.value(table, col, row)
            elif col in self.fks[table]:
                cells[col] = 1 if col == "document_id" else parent_id
            elif col == "ordinal":
                cells[col] = ordinal
            elif col == "logical_id":
                cells[col] = str(row.get("id", ""))
            else:
                v = self.value(table, col, row)
                if isinstance(v, (dict, list)):
                    self.unresolved.setdefault(table, set()).add(col)
                    v = None
                cells[col] = int(v) if isinstance(v, bool) else v
        names = list(cells)
        try:
            self.db.execute(f"INSERT INTO {table}({','.join(names)}) VALUES({','.join('?' * len(names))})", [cells[n] for n in names])
        except sqlite3.IntegrityError as error:
            self.unresolved.setdefault(table, set()).add(f"constraint: {error}")
        self.consumed.add(table)
        self.seen[table.split("_", 1)[1]].setdefault(row.get("id"), []).append(rid)
        return rid

    def child_table(self, parent_table, prop, entity):
        stem = parent_table.rsplit("_", 1)[0]
        for cand in (f"{stem}_{snake(singular_of(prop))}", f"{stem}_{snake(prop)}", f"{parent_table}_{snake(prop)}", f"{self.ctx.short}_{snake(prop)}", f"{parent_table}_{snake(prop[:-1])}", f"{self.ctx.short}_{snake(prop[:-1])}", f"{parent_table}_{snake(singular_of(prop))}", f"{self.ctx.short}_{snake(singular_of(prop))}", f"{self.ctx.short}_{snake(entity)}", snake(entity)):
            if cand in self.tables:
                return cand
        return None

    def walk(self, table, entity, row, parent_id, ordinal=0, fixed_id=None):
        rid = self.insert(table, row, parent_id, ordinal, fixed_id)
        for prop, node in self.ctx.defs[entity]["properties"].items():
            node = l_codec.unwrap(node)
            sub = row.get(prop)
            if node.get("type") == "array" and node.get("items", {}).get("$ref", "").startswith("#/$defs/"):
                child_entity = node["items"]["$ref"].rsplit("/", 1)[1]
                tbl = self.child_table(table, prop, child_entity)
                for k, item in enumerate(sub or []):
                    if tbl:
                        self.walk(tbl, child_entity, item, rid, k)
                    else:
                        self.unresolved.setdefault(table, set()).add(prop + "[]")
            elif "$ref" in node and self.ctx.defs.get(node["$ref"].rsplit("/", 1)[1], {}).get("type") == "object" and isinstance(sub, dict):
                child_entity = node["$ref"].rsplit("/", 1)[1]
                tbl = None if any(c.startswith(snake(prop) + "_") for c in self.cols[table]) else self.child_table(table, prop, child_entity)
                if tbl and tbl != table:
                    self.walk(tbl, child_entity, sub, rid, 0, fixed_id=rid)
        return rid

    def run(self):
        self.pass_once()
        again = Projector(self.ctx, self.doc, self.sql, self.empty)
        again.known = {name: {k: list(v) for k, v in rows.items()} for name, rows in self.seen.items()}
        again.pass_once()
        return again

    def pass_once(self):
        d = self.doc
        doc_table = f"{self.ctx.short}_document"
        self.insert(doc_table, d, 1, 0, fixed_id=1)
        for plural, entity in self.ctx.collections:
            if plural in self.empty:
                continue
            try:
                table = l_codec.table_of(self.ctx, Names(plural, entity), self.sql)
            except Unsupported:
                table = None
            if not table:
                if d.get(plural):
                    self.unresolved.setdefault("(root)", set()).add(plural)
                continue
            sup = self.fks[table].get("id")
            for k, item in enumerate(d.get(plural) or []):
                fixed = self.insert_super(sup, table, item, k) if sup else None
                self.walk(table, entity, item, 1, k, fixed_id=fixed)

    def insert_super(self, sup, table, item, ordinal):
        row = dict(item)
        row["kind"] = next((t for t in re.findall(r"'(\w+)'", re.search(r"kind[^)]*IN\s*\(([^)]*)\)", self.sql[self.sql.index("CREATE TABLE " + sup):]).group(1)) if table.endswith("_" + t + "_action") or table.endswith("_" + snake(t) + "_action")), "")
        return self.insert(sup, row, 1, ordinal)

    def extent(self):
        return independent_extent(self.db)


def measure(ctx, case_id="full", sql=None):
    doc = sample_document(ctx)
    if doc is None:
        raise Unsupported("no sample document fixture")
    empties = [p for p, _ in ctx.collections] if case_id.lower().startswith("empty") else ()
    p = Projector(ctx, doc, sql or ctx.plan.read(ctx.layout.p(SQLITE + "/🗄️.sql")), empties).run()
    return p.extent(), p


def refresh(ctx):
    """🧾 Recomputes `schemaBytes`, `tableWidths` and every case `valueBytes` from the committed DDL and sample document; refuses when a column could not be mapped (the projection is then incomplete) or when the projected row count differs from the stored one although the stored schema still matches the DDL (a stale schema lets the rows follow the projection)."""
    path = l_codec.semantic_path(ctx)
    if not ctx.plan.exists(path):
        raise Unsupported("this artifact keeps no semantic.json")
    sem = json.loads(ctx.plan.read(path))
    sql = ctx.plan.read(ctx.layout.p(SQLITE + "/🗄️.sql"))
    import l_sqlite
    stale = (sem["schemaBytes"], sem["tableWidths"]) != l_sqlite.replay(sql)
    for case in sem["cases"]:
        extent, p = measure(ctx, case["id"], sql)
        gaps = {t: sorted(c) for t, c in p.unresolved.items()}
        if extent["rows"] != case["rows"] and stale and not gaps:
            ctx.plan.note(f"case {case['id']}: stored rows {case['rows']} are stale (the DDL changed since the contract was written), projected {extent['rows']}")
            case["rows"] = extent["rows"]
        if extent["rows"] != case["rows"] or gaps:
            raise Unsupported(f"case {case['id']}: projected {extent['rows']} rows (stored {case['rows']}), unresolved {gaps}; the sample document is not fully projectable")
        ctx.plan.note(f"case {case['id']}: valueBytes {case['valueBytes']} -> {extent['valueBytes']} ({extent['rows']} rows)")
        case["valueBytes"] = extent["valueBytes"]
        sem["schemaBytes"], sem["tableWidths"] = extent["schemaBytes"], extent["tableWidths"]
    l_codec.write_contract(ctx, sem)
    ctx.plan.note(f"schemaBytes {sem['schemaBytes']}, {len(sem['tableWidths'])} tables")
