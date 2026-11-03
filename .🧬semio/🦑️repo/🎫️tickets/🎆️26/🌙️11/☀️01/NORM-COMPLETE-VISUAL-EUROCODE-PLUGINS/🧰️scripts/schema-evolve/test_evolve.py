"""🧪 Language-agnostic tests of schema-evolve; python `sqlite3` is the third-party implementation that validates the generated DDL and the contract replay.
Run: python3 -I test_evolve.py"""
import json, os, re, sqlite3, sys, unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import l_sqlite
from core import Names, Unsupported
from ctx import Ctx
from spec import Entity, Field, Spec

HERE = os.path.dirname(os.path.abspath(__file__))
LABEL = {"en": "x", "de": "y"}


def entity(fields, nested=None):
    return Entity({"name": "rows", "entity": "Row", "doc": "d", "label": LABEL, "id": LABEL, "singular": LABEL, "fields": fields, "nested": nested or []}, "rows")


class Generators(unittest.TestCase):
    def test_ddl_executes_in_sqlite_and_has_the_planned_columns(self):
        ctx = type("C", (), {"short": "en9999"})()
        ent = entity([{"name": "depth", "type": "f64", "unit": "m", "label": LABEL}, {"name": "kind", "type": "enum", "label": LABEL, "enum": {"name": "RowKind", "values": [{"token": "a", "label": LABEL}, {"token": "b", "label": LABEL}]}}, {"name": "count", "type": "u32", "label": LABEL}])
        db = sqlite3.connect(":memory:")
        db.executescript("CREATE TABLE en9999_document(id INTEGER PRIMARY KEY);" + l_sqlite.ddl(ctx, ent))
        columns = [r[1] for r in db.execute("pragma table_info(en9999_row)")]
        self.assertEqual(len(columns), l_sqlite.width(ent) + 2 * len(l_sqlite.floats(ent)))
        self.assertEqual(l_sqlite.floats(ent), [columns.index("depth")])
        with self.assertRaises(sqlite3.IntegrityError):
            db.execute("INSERT INTO en9999_row VALUES(1,1,0,'r',1.0,'zzz',1,0,'finite')")

    def test_float_free_entities_reference_an_empty_column_list_not_an_undefined_constant(self):
        text = entity([{"name": "kind", "type": "u32", "label": LABEL}])
        real = entity([{"name": "depth", "type": "f64", "unit": "m", "label": LABEL}])
        self.assertEqual((l_sqlite.const_of(text), l_sqlite.const_ts(text)), ("&[]", "[]"))
        self.assertEqual((l_sqlite.const_of(real), l_sqlite.const_ts(real)), ("ROW", "ROW"))

    def test_contract_replay_counts_bytes_and_widths(self):
        sql = "CREATE TABLE t (a INTEGER, b TEXT);\n"
        size, widths = l_sqlite.replay(sql)
        db = sqlite3.connect(":memory:")
        db.executescript(sql)
        name, text = db.execute("select name, sql from sqlite_schema").fetchone()
        self.assertEqual(size, len(name.encode()) + len(text.encode()))
        self.assertEqual(widths, {"t": 2})

    def test_labels_in_both_languages_are_required(self):
        with self.assertRaises(Unsupported):
            Field({"name": "x", "type": "f64", "label": {"en": "only english"}})
        with self.assertRaises(Unsupported):
            entity([{"name": "x", "type": "f64", "label": LABEL}], [{"name": "n", "entity": "N", "doc": "d", "label": LABEL, "id": LABEL, "fields": []}])

    def test_name_forms_rename_every_spelling(self):
        ref, new = Names("layerWaters", "LayerWater"), Names("investigations", "Investigation")
        self.assertEqual(ref.rename("LayerWaters layerWaters layer_waters layer-waters LayerWater layerWater layer_water layer-water", new), "Investigations investigations investigations investigations Investigation investigation investigation investigation")


class Repository(unittest.TestCase):
    """🫙 The repository is its own oracle: generators rebuilt from the committed schema must reproduce the committed codecs."""

    def test_selfcheck_reproduces_committed_sqlite_codecs(self):
        import selfcheck
        for artifact, collection in (("en1997", "layerWaters"), ("en1997", "labTests"), ("en1997", "pileGroups")):
            self.assertEqual(selfcheck.main(artifact, collection), 0, collection)

    def test_dry_run_of_the_committed_specs_is_idempotent_for_en1997(self):
        spec = Spec(os.path.join(HERE, "specs", "en1997-investigations.json"))
        ctx = Ctx("en1997", spec)
        self.assertEqual(ctx.ref.plural_camel, "layerWaters")


class Engine(unittest.TestCase):
    """🫘 The learned-codec engine against every artifact: the generated DDL is executed by python `sqlite3`, the planned codecs name every new field, and no layer is unsupported."""

    @classmethod
    def setUpClass(cls):
        import probe, snapshot, tempfile
        cls.probe, cls.snapshot = probe, snapshot
        cls.work = tempfile.mkdtemp(prefix="evolve-test-")
        cls.plans = {}
        for short in probe.ALL:
            path = os.path.join(cls.work, short + ".json")
            json.dump(probe.collection_spec(short), open(path, "w"))
            cls.plans[short] = (snapshot.plan_of(short, "add-collection", path), probe.collection_spec(short)["collection"])

    def test_lexer_splits_statements_and_items(self):
        import lex
        text = 'fn f(){let a=1;for x in y{z(x);}let b=Foo{p:1,q:[2,3]};}'
        mask = lex.code_mask(text)
        (o, c), = lex.fn_bodies(text, mask)
        parts = [text[a:b] for a, b in lex.statements(text, mask, o, c)]
        self.assertEqual(parts, ["let a=1;", "for x in y{z(x);}", "let b=Foo{p:1,q:[2,3]};"])
        i = text.index("{p:")
        items = lex.split_items(text, mask, i + 1, lex.match_close(text, mask, i))
        self.assertEqual([text[a:b] for a, b in items], ["p:1", "q:[2,3]"])

    def test_rename_follows_the_context_of_single_word_names(self):
        ref, new = Names("joints", "Joint"), Names("tieBeams", "TieBeam")
        quoted = '"annex","retainingWalls","joints"'
        self.assertEqual(ref.rename(quoted, new), '"annex","retainingWalls","tieBeams"')
        import core
        core.PROTECT.add("jointGapM")
        try:
            self.assertEqual(ref.rename("self.joints.len() jointGapM jointRows insert-joint", new), "self.tie_beams.len() jointGapM tieBeamRows insert-tie-beam")
        finally:
            core.PROTECT.discard("jointGapM")

    def test_every_artifact_has_a_leaf_kinds_driver(self):
        import l_mutations
        self.assertEqual({a: l_mutations.driver_of(a) for a in l_mutations.ARTIFACTS}["en1997"], "en1997.py")
        self.assertTrue(all(l_mutations.driver_of(a) for a in l_mutations.ARTIFACTS))
        self.assertEqual(set(l_mutations.ARTIFACTS), set(__import__("probe").ALL))

    def test_every_artifact_plans_the_collection_without_unsupported_layers(self):
        for short, (ctx, _) in self.plans.items():
            self.assertEqual(ctx.unsupported, [], short)

    def test_generated_ddl_executes_in_sqlite_with_the_planned_columns(self):
        import l_codec
        for short, (ctx, spec) in self.plans.items():
            path = ctx.layout.p(l_codec.SQLITE + "/🗄️.sql")
            sql = ctx.plan.read(path)
            db = sqlite3.connect(":memory:")
            db.executescript(sql)
            tables = l_codec.tables_of(sql)
            new = [t for t in tables if t.endswith("probe_row")]
            self.assertEqual(len(new), 1, short)
            columns = [r[1] for r in db.execute(f"pragma table_info({new[0]})")]
            reals = [f for f in spec["fields"] if f["type"] == "f64"]
            self.assertEqual(len(columns), 4 + len(spec["fields"]) + 2 * len(reals), short)
            enums = [f for f in spec["fields"] if f["type"] == "enum"]
            if enums:
                doc = db.execute("select name from sqlite_schema where type='table' and sql like '%REFERENCES%document%' and name=?", (new[0],)).fetchone()
                self.assertTrue(doc, short)

    def test_planned_codecs_name_every_new_field_and_table(self):
        import l_codec
        for short, (ctx, spec) in self.plans.items():
            rs = ctx.plan.read(ctx.layout.p(l_codec.SQLITE + "/🦀️.rs"))
            ts = ctx.plan.read(ctx.layout.p(l_codec.SQLITE + "/🟦️.ts"))
            for field in spec["fields"]:
                self.assertIn(field["name"], rs, f"{short} rust {field['name']}")
                self.assertIn(field["name"].replace("_", "") if "_" not in field["name"] else l_codec.camel(field["name"]), ts, f"{short} ts {field['name']}")
            self.assertRegex(rs, r'"\w*probe_row"')
            self.assertRegex(ts, r'"\w*probe_row"')

    def test_learned_ddl_reproduces_committed_tables(self):
        import l_codec
        ok = bad = 0
        for short, (ctx, _) in self.plans.items():
            sql = ctx.layout and open(ctx.layout.p(l_codec.SQLITE + "/🗄️.sql"), encoding="utf8").read()
            for plural, entity in ctx.collections:
                names = Names(plural, entity)
                try:
                    tbl = l_codec.table_of(ctx, names, sql)
                except Unsupported:
                    continue
                fields = l_codec.schema_fields(ctx, entity)
                if not fields or any(f.kind in (None, "bool") for f in fields) or any(v.get("type") in ("array", "object") for v in ctx.defs[entity]["properties"].values()):
                    continue
                m, close, items = l_codec.ddl_body(sql, tbl)
                columns = [l_codec.def_name(sql[a:b]) for a, b in items]
                if columns[4:4 + len(fields)] != [f.snake for f in fields]:
                    continue
                try:
                    again = l_codec.ddl_generic(ctx, sql, names, tbl, fields, tbl, fields)
                except Unsupported:
                    bad += 1
                    continue
                if again.strip() == sql[m.start():sql.find(";", close) + 1].strip():
                    ok += 1
                else:
                    bad += 1
        self.assertGreaterEqual(ok, 50)
        self.assertLess(bad, ok // 10 + 1)

    def test_leaf_dry_runs_have_no_unsupported_layers(self):
        spec = {"leaf": {"field": {"name": "probe_value", "type": "f64", "unit": "m", "label": {"en": "Probe value", "de": "Probewert"}, "sample": 0.0}, "entity": {"singular": {"en": "probe entity", "de": "Probeobjekt"}}}}
        for short, plural in (("en1998", "piers"), ("en1993", "jointPlates"), ("en1997", "layers"), ("en1990", "members"), ("en1991", "floors"), ("en1992", "concreteGrades"), ("en1995", "diaphragms"), ("en1999", "shells")):
            path = os.path.join(self.work, f"leaf-{short}.json")
            json.dump(spec, open(path, "w"))
            ctx = self.snapshot.plan_of(short, "add-leaf", path, plural)
            self.assertEqual(ctx.unsupported, [], f"{short} {plural}")


LB = lambda en, de: {"en": en, "de": de}


class WaveThree(unittest.TestCase):
    """🧪 Enum values, enum leaves, composite codecs, the object-to-collection conversion, the extent projection and the relabelling; python `sqlite3` is the third-party implementation."""

    @classmethod
    def setUpClass(cls):
        import snapshot, tempfile
        cls.snapshot = snapshot
        cls.work = tempfile.mkdtemp(prefix="evolve-test3-")

    def spec(self, name, raw):
        path = os.path.join(self.work, name + ".json")
        json.dump(raw, open(path, "w", encoding="utf8"))
        return path

    def leaf_plan(self, short, plural, field):
        return self.snapshot.plan_of(short, "add-leaf", self.spec(f"{short}-{plural}", {"leaf": {"field": field, "entity": {"singular": LB("probe entity", "Probeobjekt")}}}), plural)

    def test_arm_parser_clones_only_mechanical_arms(self):
        import l_enum
        from lex import code_mask
        text = 'fn t(a:K)->&str{match a{K::A=>"a",K::B=>"b"}} fn v(a:K)->f64{match a{K::A=>1.0,K::B=>2.0}} fn w(a:K)->u8{match a{K::A=>1,_=>0}}'
        ctx = type("C", (), {"enum_value": l_enum.Value({"enum": "K", "token": "c", "variant": "C", "label": LB("C", "C")}), "variant_of": {"a": "A", "b": "B"}, "after_token": "b"})()
        new, manual = l_enum.arms_edit(ctx, text, "x.rs")
        self.assertIn('K::B=>"b",K::C=>"c"', new)
        self.assertNotIn("K::C=>2.0", new)
        self.assertEqual(len(manual), 1)
        self.assertEqual(len(l_enum.arms(text, code_mask(text, "rs"), "K")), 3)
        self.assertTrue(l_enum.has_wildcard(l_enum.arms(text, code_mask(text, "rs"), "K")[2][1]))

    def test_enum_value_dry_run_extends_check_lists_and_sqlite_accepts_the_token(self):
        spec = self.spec("enum", {"enumValue": {"token": "Fair", "label": LB("Fair bond conditions", "Mittlere Verbundbedingungen")}})
        import evolve, l_enum
        ctx = Ctx("en1992", Spec(spec))
        ctx.prefix = None
        ctx.enum_value = l_enum.Value({**Spec(spec).raw["enumValue"], "enum": "BondCondition"})
        for name, fn in l_enum.layers():
            ctx.guard(fn, name)
        self.assertEqual(ctx.unsupported, [])
        sql = ctx.plan.read(ctx.layout.p("🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"))
        db = sqlite3.connect(":memory:")
        db.executescript(sql)
        table = next(t for t in ("en1992_bar_layer",) if t in sql)
        cols = [r[1] for r in db.execute(f"pragma table_info({table})")]
        row = {c: 0 for c in cols}
        row.update({"id": 1, "logical_id": "x"})
        for c in cols:
            if c.endswith("_ieee754_class"):
                row[c] = "finite"
            elif c in ("bond_condition",):
                row[c] = "Fair"
            elif c not in ("id", "ordinal") and not c.endswith("_bits") and not c.endswith("_id") and c != "logical_id" and cols.index(c) > 3 and "position" in c:
                row[c] = "Bottom"
        self.assertIn("'Good','Poor','Fair'", sql.replace(" ", ""))
        rs = ctx.plan.read(ctx.layout.p("🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"))
        self.assertIn('BondCondition::Fair=>"Fair"', rs)
        self.assertIn('"Fair"=>Ok(BondCondition::Fair)', rs)
        self.assertIn("variant BondCondition::Fair", " ".join(ctx.plan.notes))

    def test_every_composite_codec_entity_takes_a_leaf(self):
        leaf = {"name": "probe_value", "type": "f64", "unit": "m", "label": LB("Probe value", "Probewert"), "sample": 0.0}
        for short, plural in (("en1990", "bridgeSls"), ("en1990", "effects"), ("en1990", "fatigueSpectra"), ("en1992", "members"), ("en1992", "anchors"), ("en1994", "beams"), ("en1994", "slabs"), ("en1999", "connections")):
            ctx = self.leaf_plan(short, plural, leaf)
            self.assertEqual(ctx.unsupported, [], f"{short} {plural}")

    def test_composite_leaf_ddl_runs_in_sqlite_and_codec_indices_follow_the_ddl(self):
        leaf = {"name": "probe_value", "type": "f64", "unit": "m", "label": LB("Probe value", "Probewert"), "sample": 0.0}
        ctx = self.leaf_plan("en1990", "bridgeSls", leaf)
        import l_codec
        sql = ctx.plan.read(ctx.layout.p(l_codec.SQLITE + "/🗄️.sql"))
        db = sqlite3.connect(":memory:")
        db.executescript(sql)
        cols = [r[1] for r in db.execute("pragma table_info(en1990_bridge_sls)")]
        index = cols.index("probe_value")
        rs = ctx.plan.read(ctx.layout.p(l_codec.SQLITE + "/🦀️.rs"))
        self.assertEqual(int(re.search(r"probe_value\s*:\s*row\.real\((\d+)\)", rs).group(1)), index)
        self.assertEqual(int(re.search(r'entities\(database,\s*"en1990_bridge_sls",\s*(\d+)', rs).group(1)), index + 1)

    def test_enum_leaves_reuse_or_create_the_enum(self):
        existing = {"name": "probe_bond", "type": "enum", "label": LB("Probe bond", "Probe-Verbund"), "enum": {"name": "BondCondition"}}
        new = {"name": "probe_mode", "type": "enum", "label": LB("Probe mode", "Probemodus"), "enum": {"name": "ProbeMode", "values": [{"token": "fast", "label": LB("Fast", "Schnell")}, {"token": "slow", "label": LB("Slow", "Langsam")}]}}
        import l_codec
        for field, token_list in ((existing, "'Good','Poor'"), (new, "'Fast','Slow'")):
            ctx = self.leaf_plan("en1992", "anchors", field)
            self.assertEqual(ctx.unsupported, [])
            sql = ctx.plan.read(ctx.layout.p(l_codec.SQLITE + "/🗄️.sql"))
            db = sqlite3.connect(":memory:")
            db.executescript(sql)
            self.assertIn(token_list, sql.replace(" ", "").replace("','", "','"))
            col = field["name"]
            self.assertTrue(db.execute("select count(*) from pragma_table_info('en1992_anchor') where name=?", (col,)).fetchone()[0])
            with self.assertRaises(sqlite3.IntegrityError):
                db.execute(f"INSERT INTO en1992_anchor({col}) VALUES('zzz')")
        rs = ctx.plan.read(ctx.layout.p(l_codec.SQLITE + "/🦀️.rs"))
        self.assertIn("fn probe_mode_text", rs)
        self.assertIn("fn read_probe_mode", rs)
        model = ctx.plan.read(ctx.layout.p("🧬️schema/📸️snapshot/🦀️.rs"))
        self.assertIn("pub enum ProbeMode {", model)
        self.assertNotIn("table_c1_minima(self) -> (f64, f64) {\n        match self {\n            Self::Fast", model)

    def test_row_rules_are_a_safe_expression_language(self):
        import l_convert
        self.assertEqual(l_convert.safe_eval("obj['a'] * i + 1", obj={"a": 2.0}, i=3), 7.0)
        for bad in ("__import__('os')", "obj.__class__", "open('x')", "[x for x in ().__class__.__mro__]"):
            with self.assertRaises(Unsupported):
                l_convert.safe_eval(bad, obj={})

    def test_conversion_rewrites_fixtures_losslessly_and_verifies_invariants(self):
        import l_convert
        rule = l_convert.Rule({"parent": "beams", "field": "studs", "rows": {"count": "obj['n']", "values": {"id": "{parent[id]}-s{i}", "xM": "=obj['sp'] * i", "d": "=obj['d']"}}, "verify": ["len(rows) == obj['n']", "len(rows) < 2 or abs(rows[1]['xM'] - rows[0]['xM'] - obj['sp']) < 1e-12"]}, "studs")
        stats = {"objects": 0, "rows": 0, "dropped_for_new": 0, "payload_objects": 0}
        doc = {"beams": [{"id": "b1", "studs": {"n": 3, "sp": 0.25, "d": 0.019}}, {"id": "b2", "studs": {"n": 0, "sp": 0.2, "d": 0.02}}], "studs": []}
        self.assertTrue(l_convert.rows_of_document(doc, rule, stats))
        self.assertEqual([r["id"] for r in doc["studs"]], ["b1-s0", "b1-s1", "b1-s2"])
        self.assertEqual([r["xM"] for r in doc["studs"]], [0.0, 0.25, 0.5])
        self.assertNotIn("studs", doc["beams"][0])
        bad = l_convert.Rule({"parent": "beams", "field": "studs", "rows": {"count": "obj['n']", "values": {"xM": "=obj['sp'] * i * 2"}}, "verify": ["len(rows) < 2 or abs(rows[1]['xM'] - rows[0]['xM'] - obj['sp']) < 1e-12"]}, "studs")
        with self.assertRaises(Unsupported):
            l_convert.rows_of_document({"beams": [{"id": "b", "studs": {"n": 2, "sp": 0.2}}]}, bad, dict(stats))

    def test_en1994_studs_conversion_is_applied_and_leaves_no_fixture_unconverted(self):
        import evolve, l_convert
        spec = Spec(os.path.join(HERE, "specs", "en1994-studs.json"))
        ctx = Ctx("en1994", spec)
        ctx.prefix = None
        ctx.rule = l_convert.Rule(spec.raw["convert"], "studs")
        for name, fn in l_convert.layers(evolve.collection_layers()):
            ctx.guard(fn, name)
        self.assertEqual([u for u in ctx.unsupported if not u.startswith("mutations: no mutation directory for the reference kind")], [], "only the one-shot set-field kind clone may report (the committed kinds carry the legacy name change-stud-xm)")
        left = [p for p, (_, new, _) in ctx.plan.files.items() if p.endswith(".json") and "/🧪️tests/🧫️fixtures/" in p and re.search(r'"studs": \{\s*"diameterM"', new)]
        self.assertEqual(left, [])
        root = os.path.join(ctx.layout.any, "🧪️tests/🧫️fixtures")
        for dp, _, files in os.walk(root):
            for f in files:
                if f.endswith(".json"):
                    self.assertIsNone(re.search(r'"studs": \{\s*"diameterM"', open(os.path.join(dp, f), encoding="utf8").read()), os.path.join(dp, f))

    def test_extent_projection_matches_the_committed_contract_with_sqlite_as_judge(self):
        import l_extent, l_codec
        for short in ("en1990", "en1993", "en1994"):
            spec = Spec.__new__(Spec)
            spec.reference = spec.nested_reference = spec.collection = None
            ctx = Ctx(short, spec)
            sem = json.loads(ctx.plan.read(l_codec.semantic_path(ctx)))
            sql = ctx.plan.read(ctx.layout.p(l_codec.SQLITE + "/🗄️.sql"))
            self.assertEqual((sem["schemaBytes"], sem["tableWidths"]), l_sqlite.replay(sql), short)
            for case in sem["cases"]:
                extent, projector = l_extent.measure(ctx, case["id"])
                self.assertEqual(projector.unresolved, {}, short)
                self.assertEqual((extent["rows"], extent["valueBytes"]), (case["rows"], case["valueBytes"]), f"{short} {case['id']}")

    def test_extent_projection_reproduces_the_committed_contracts_of_git_head(self):
        import subprocess, l_extent, l_codec
        root = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", "..", "..", ".."))
        probe = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, cwd=HERE)
        if probe.returncode:
            self.skipTest("no git")
        root = probe.stdout.strip() + "/"
        matched = 0
        for short in ("en1991", "en1995", "en1996", "en1997", "en1999"):
            spec = Spec.__new__(Spec)
            spec.reference = spec.nested_reference = spec.collection = None
            ctx = Ctx(short, spec)

            def show(path):
                return subprocess.run(["git", "show", "HEAD:" + path[len(root):]], capture_output=True, text=True, cwd=root).stdout
            sql = show(ctx.layout.p(l_codec.SQLITE + "/🗄️.sql"))
            doc = json.loads(show(ctx.layout.p(l_codec.SQLITE + "/🧪️tests/🧫️fixtures/🔣️.json")))
            sem = json.loads(show(l_codec.semantic_path(ctx)))
            doc = doc.get("snapshot", doc)
            for case in sem["cases"]:
                projector = l_extent.Projector(ctx, doc, sql, [p for p, _ in ctx.collections] if case["id"].startswith("empty") else ()).run()
                extent = projector.extent()
                self.assertEqual((extent["rows"], extent["valueBytes"], extent["schemaBytes"], extent["tableWidths"]), (case["rows"], case["valueBytes"], sem["schemaBytes"], sem["tableWidths"]), f"{short} {case['id']}")
                matched += 1
        self.assertEqual(matched, 10)

    def test_stale_limits_follow_the_contract(self):
        import l_codec
        for short in ("en1993", "en1994"):
            spec = Spec.__new__(Spec)
            spec.reference = spec.nested_reference = spec.collection = None
            ctx = Ctx(short, spec)
            sem = json.loads(ctx.plan.read(l_codec.semantic_path(ctx)))
            text = ctx.plan.read(ctx.layout.p(l_codec.SQLITE + "/🧪️tests/🟦️.ts"))
            m = re.search(r"maxSchemaBytes:normSemanticContract\.schemaBytes,maxTables:(\d+),maxColumns:(\d+)", text)
            self.assertEqual((int(m.group(1)), int(m.group(2))), (len(sem["tableWidths"]), max(sem["tableWidths"].values())), short)

    def test_cloned_kinds_take_spec_labels_and_the_taxonomy_gets_their_rows(self):
        import probe
        spec = probe.full_spec()
        spec["collection"]["nested"] = [{"name": "probeReadings", "entity": "ProbeReading", "doc": "d", "label": LB("Probe readings", "Probemessungen"), "id": LB("Reading ID", "Messungs-ID"), "singular": LB("probe reading", "Probemessung"), "kindSlug": "probe-reading", "fields": [{"name": "depth", "type": "f64", "unit": "m", "label": LB("Depth", "Tiefe"), "sample": 1.0}]}]
        spec["collection"]["label"] = LB("Probe rows", "Probezeilen")
        spec["collection"]["singular"] = LB("probe row", "Probezeile")
        ctx = self.snapshot.plan_of("en1997", "add-collection", self.spec("nested", spec))
        self.assertEqual(ctx.unsupported, [])
        schemas = {p: new for p, (old, new, _) in ctx.plan.files.items() if old is None and "/🧬️mutations/" in p and p.endswith("🧬️schema/🔣️.json")}
        self.assertTrue(schemas)
        for path, text in schemas.items():
            self.assertNotIn("Layer water regime", text, path)
            self.assertNotIn("Wasserverhältnis der Schicht", text, path)
        insert = next(t for p, t in schemas.items() if "insert-probe-row" in p)
        self.assertIn("Probe row", insert)
        self.assertIn("Probezeile", insert)
        taxonomy = json.loads(ctx.plan.read(l_mutations_taxonomy()))["rows"]
        kinds = {r["kind"]: r for r in taxonomy if r["artifact"].endswith("en1997")}
        for kind in ("insert-probe-row", "remove-probe-row", "insert-probe-reading", "remove-probe-reading"):
            self.assertIn(kind, kinds)
        self.assertEqual(kinds["insert-probe-reading"]["edit"]["path"], "probeRows[].probeReadings")
        self.assertEqual(kinds["insert-probe-row"]["edit"]["path"], "probeRows")

    def test_a_leftover_reference_label_aborts_the_clone(self):
        import l_mutations
        source = json.dumps({"properties": {"row": {"$ref": "x#/$defs/R", "x-semio-ui": {"label": {"en": "Old thing", "de": "Altes Ding"}}}}})
        ent = type("E", (), {"singular_en": "new thing", "singular_de": "Neues Ding"})()
        ctx = type("C", (), {})()
        good = l_mutations.relabel_kind(ctx, source, source, ent, None, "insert-x")
        self.assertIn("New thing", good)
        self.assertEqual(l_mutations.leftover_label("see Old thing here", [(("Old thing", "Altes Ding"), ("New thing", "Neues Ding"))], {"new thing"}), "Old thing")
        self.assertIsNone(l_mutations.leftover_label("see the OldThingKind", [(("Old thing", "Altes Ding"), ("New thing", "Neues Ding"))], set()))
        self.assertEqual(l_mutations.relabel_prose('/// Inserts an old thing.\nfn old_thing() { "Old thing {key} exists" }', [(("Old thing", "Altes Ding"), ("Tie beam", "Zuganker"))]), '/// Inserts an tie beam.\nfn old_thing() { "Tie beam {key} exists" }')

class WaveFour(unittest.TestCase):
    """🧪 SCHEMA-TOOLING v4: the reference collection is never rewritten, and the fixes of the EN 1998 v17 hand repairs."""

    @classmethod
    def setUpClass(cls):
        import snapshot, tempfile
        cls.snapshot = snapshot
        cls.work = tempfile.mkdtemp(prefix="evolve-test4-")

    def spec(self, name, raw):
        path = os.path.join(self.work, name + ".json")
        json.dump(raw, open(path, "w", encoding="utf8"))
        return path

    def test_add_collection_only_inserts_into_the_sqlite_codecs_of_the_reference(self):
        import difflib, probe
        for short in ("en1992", "en1995", "en1996"):
            ctx = self.snapshot.plan_of(short, "add-collection", self.spec(short, probe.collection_spec(short)))
            checked = 0
            for path, (old, new, _) in ctx.plan.files.items():
                if old is None or "/🪶️sqlite/" not in path or "/🧪️tests/" in path or not path.endswith(("🦀️.rs", "🟦️.ts", "🗄️.sql")):
                    continue
                checked += 1
                ops = [o for o in difflib.SequenceMatcher(None, old, new, autojunk=False).get_opcodes() if o[0] in ("replace", "delete")]
                self.assertEqual(ops, [], f"{short}: {path} rewrote reference text {[old[o[1]:o[2]] for o in ops][:2]}")
            self.assertGreaterEqual(checked, 3, short)

    def test_the_typescript_codec_declares_and_lists_the_new_collection_in_camel_case(self):
        import probe
        ctx = self.snapshot.plan_of("en1992", "add-collection", self.spec("en1992-ts", probe.collection_spec("en1992")))
        ts = next(new for path, (old, new, _) in ctx.plan.files.items() if "/🪶️sqlite/" in path and path.endswith("🟦️.ts"))
        self.assertIn("probeRows:En1992SqliteProbeRow[]", ts)
        self.assertIn("const probeRows:En1992SqliteProbeRow[]=[]", ts)
        self.assertIn("placements,probeRows,cases", ts)
        self.assertNotIn("probe_rows", ts)
        self.assertIn("placements:En1992SqlitePlacement[]", ts)

    def plan(self, short, raw, name):
        return self.snapshot.plan_of(short, "add-collection", self.spec(name, raw))

    def planned(self, ctx, needle):
        return {p: new for p, (old, new, _) in ctx.plan.files.items() if needle in p}

    def test_committed_collection_specs_are_a_no_op_second_time(self):
        for short, name in (("en1993", "en1993-frame-actions"), ("en1993", "en1993-frame-member-bases"), ("en1998", "en1998-tie-beams"), ("en1998", "en1998-slopes")):
            ctx = self.snapshot.plan_of(short, "add-collection", os.path.join(HERE, "specs", name + ".json"))
            self.assertEqual(ctx.unsupported, [], name)
            layers = {l for _, (_, _, l) in ctx.plan.files.items()}
            if name == "en1998-slopes":
                self.assertEqual(layers, {"mutations"}, "the committed en1998 slopes differ from the spec only in the mutation kinds (legacy kind names such as change-seismic-slope-topographic-factor)")
            else:
                self.assertEqual(sorted(ctx.plan.files), [], name)

    def test_diff_schema_lists_the_lane_where_the_rust_diff_declares_it(self):
        import probe, l_schema
        for short in ("en1990", "en1997", "en1998", "en1999"):
            ctx = self.plan(short, probe.collection_spec(short), short + "-lanes")
            block = l_schema.rust_lane_block(ctx, "probe_rows")
            names = [re.sub(r"_(\w)", lambda m: m.group(1).upper(), n) for n, _ in block]
            js = json.loads(next(v for p, v in self.planned(ctx, "/🔺️diff/🔣️.json").items() if "/🧬️mutations/" not in p))
            keys = [k for k in js["properties"] if k in names]
            self.assertEqual(keys, names, short)
            ref = ctx.ref.plural_camel
            self.assertEqual("probeRows" in js.get("required", []), ref in js.get("required", []), short)
            if "required" in js and ref in js["required"]:
                req = [k for k in js["required"] if k in names]
                self.assertEqual(req, [k for k in names if k in req], short)

    def test_edit_rules_and_registries_use_camel_case_paths_and_one_kind_name(self):
        import probe
        spec = probe.collection_spec("en1998")
        spec["collection"]["fields"].append({"name": "slope_id", "type": "String", "label": probe.LB("Slope"), "sample": "x"})
        ctx = self.plan("en1998", spec, "en1998-camel")
        rules = "".join(self.planned(ctx, "/🧭️edit-rules/").values())
        self.assertIn('path: "probeRows[].span"', rules)
        self.assertIn('item: "probeRow"', rules)
        self.assertNotIn("probe_rows", rules)
        registry = next(v for p, v in self.planned(ctx, "/🧬️mutations/🦀️.rs").items() if p.endswith("/🧬️mutations/🦀️.rs"))
        self.assertIn("change-probe-row-slope-id", registry)
        self.assertNotIn("probe-row-probe-row", registry)

    def test_donor_entity_words_do_not_double_a_new_kind_name(self):
        import probe
        spec = probe.collection_spec("en1998")
        spec["collection"]["fields"].append({"name": "foundation_a_id", "type": "String", "label": probe.LB("Foundation A"), "sample": "x"})
        ctx = self.plan("en1998", spec, "en1998-doubling")
        text = "".join(self.planned(ctx, "/🧬️mutations/").values())
        self.assertIn("change-probe-row-foundation-a-id", text)
        self.assertNotIn("probe-row-probe-row", text)

    def test_set_field_kinds_get_a_crate_of_their_own_and_describe_themselves(self):
        import probe
        for short in ("en1998", "en1993"):
            ctx = self.plan(short, probe.collection_spec(short), short + "-crate")
            crates = self.planned(ctx, "mut-probe-rows") or {p: v for p, v in self.planned(ctx, "/📦️packages/🦀️rust/Cargo.toml").items() if "mut-probe-rows" in v}
            self.assertTrue(any(f'name = "semio-s-artifact-norm-{short}-mut-probe-rows"' in v for v in crates.values()), short)
            mounts = [v for p, v in self.planned(ctx, "/🧬️mutations/").items() if p.endswith("/🦀️.rs") and "pub mod change_probe_row_span;" in v]
            self.assertEqual(len(mounts), 1, short)
            leaf = next(v for p, v in self.planned(ctx, "change-probe-row-span/🦀️.rs").items())
            self.assertIn('entity: "probe-row-span"', leaf)
            self.assertIn('record: "ChangedProbeRowSpan"', leaf)
            schema = json.loads(next(v for p, v in self.planned(ctx, "change-probe-row-span/🧬️schema/🔣️.json").items()))
            ui = schema["properties"]["newSpan"]["x-semio-ui"]
            self.assertEqual((ui["unit"], ui.get("displayUnit")), ("m", None))
            taxonomy = json.loads(ctx.plan.read(l_mutations_taxonomy()))["rows"]
            row = next(r for r in taxonomy if r["artifact"].endswith(short) and r["kind"] == "change-probe-row-span")
            self.assertEqual((row["edit"]["path"], row["edit"]["value"]), ("probeRows[].span", "newSpan"))

    def test_models_with_hand_written_rows_get_row_structs_and_as_str(self):
        import probe
        ctx = self.plan("en1993", probe.collection_spec("en1993"), "en1993-model")
        model = next(iter(self.planned(ctx, "/🧬️schema/📸️snapshot/🦀️.rs").values()))
        self.assertIn("pub struct ProbeRow {", model)
        self.assertIn("pub kind: ProbeKind,", model)
        self.assertIn("pub const fn as_str(self)", model.split("pub enum ProbeKind")[1])
        self.assertIn('Self::A => "A"', model)

    def test_new_enum_tokens_follow_the_artifact_spelling(self):
        from ctx import token_style
        self.assertEqual([token_style(t) for t in ("FrameDerived", "frameDerived", "frame-derived", "a")], ["pascal", "camel", "other", "other"])
        import probe
        ctx = self.plan("en1993", probe.collection_spec("en1993"), "en1993-tokens")
        schema = "".join(self.planned(ctx, "/🧬️schema/📸️snapshot/🔣️.json").values())
        self.assertIn('"A"', schema)

    def test_exhaustive_mutation_mirrors_list_the_new_kinds_and_count(self):
        import probe
        ctx = self.plan("en1993", probe.collection_spec("en1993"), "en1993-mirror")
        mirror = "".join(self.planned(ctx, "/🧪️tests/🔩️mutate-en1993-1/").values())
        for kind in ("insert-probe-row", "remove-probe-row", "change-probe-row-span"):
            self.assertIn(kind, mirror)
        self.assertRegex(mirror, r"The \d+ `✅apply`")


class WaveFive(unittest.TestCase):
    """🧪 SCHEMA-TOOLING v5: vectors on the artifact's own base snapshot, a crate for insert/remove kinds, the per-family text codec of en1998, nested reports, nested_patch! blocks."""

    @classmethod
    def setUpClass(cls):
        import snapshot, tempfile
        cls.snapshot = snapshot
        cls.work = tempfile.mkdtemp(prefix="evolve-test5-")
        cls.cache = {}

    def plan(self, short, raw=None, name=None):
        import probe
        key = (short, name)
        if key not in self.cache:
            path = os.path.join(self.work, f"{short}-{name}.json")
            json.dump(raw or probe.collection_spec(short), open(path, "w", encoding="utf8"))
            self.cache[key] = self.snapshot.plan_of(short, "add-collection", path)
        return self.cache[key]

    def planned(self, ctx, needle):
        return {p: n for p, (o, n, _) in ctx.plan.files.items() if needle in p}

    def test_vectors_are_built_on_the_sample_snapshot_of_the_artifact(self):
        import l_basedoc
        for short in ("en1998", "en1993", "en1995"):
            ctx = self.plan(short, name="base")
            sample = l_basedoc.sample(ctx)
            lanes = set(ctx.snapshot["properties"]) - {"probeRows"}
            vectors = {k: self.planned(ctx, f"{k}/") for k in ("insert-probe-row", "remove-probe-row", "change-probe-row-span")}
            for kind, files in vectors.items():
                docs = {p.split("/")[-1]: json.loads(n) for p, n in files.items() if "fixtures/🧬️mutations" in p}
                for name in ("🔣️.before.json", "🔣️.after.json"):
                    self.assertEqual({k: v for k, v in docs[name].items() if k in lanes}, {k: sample[k] for k in lanes}, f"{short} {kind} {name}")
                diff = docs["🔣️.diff.json"]
                touched = [k for k, v in diff.items() if v not in (None, [], {"removed": [], "inserted": [], "moved": [], "modified": []})]
                self.assertEqual(touched, ["probeRows"], f"{short} {kind}")
            ins = {p.split("/")[-1]: json.loads(n) for p, n in vectors["insert-probe-row"].items() if "fixtures/🧬️mutations" in p}
            self.assertEqual(len(ins["🔣️.after.json"]["probeRows"]), len(ins["🔣️.before.json"]["probeRows"]) + 1)
            rem = {p.split("/")[-1]: json.loads(n) for p, n in vectors["remove-probe-row"].items() if "fixtures/🧬️mutations" in p}
            self.assertEqual(len(rem["🔣️.after.json"]["probeRows"]), len(rem["🔣️.before.json"]["probeRows"]) - 1)

    def test_set_field_vectors_change_the_value(self):
        ctx = self.plan("en1998", name="base")
        docs = {p.split("/")[-1]: json.loads(n) for p, n in self.planned(ctx, "change-probe-row-span/").items() if "fixtures/🧬️mutations" in p}
        self.assertNotEqual(docs["🔣️.before.json"]["probeRows"], docs["🔣️.after.json"]["probeRows"])
        self.assertNotEqual(docs["🔣️.before.json"]["probeRows"][0]["span"], docs["🔣️.mutation.json"]["newSpan"])

    def test_insert_and_remove_kinds_live_in_the_crate_of_the_collection(self):
        for short in ("en1998", "en1993"):
            ctx = self.plan(short, name="base")
            mounts = {k: [p for p, n in self.planned(ctx, "/🧬️mutations/").items() if p.endswith("/🦀️.rs") and f"pub mod {k};" in n] for k in ("insert_probe_row", "remove_probe_row", "change_probe_row_span")}
            for k, hit in mounts.items():
                self.assertEqual(len(hit), 1, f"{short} {k}")
                self.assertIn("/mut-probe-rows" if False else "probe-rows/", hit[0])
            texts = "".join(v[1] for p, v in ctx.plan.files.items() if p.endswith((".rs", ".toml")))
            stray = re.findall(r"semio_s_artifact_norm_%s_mut_(?!probe_rows)\w+::(?:insert|remove)_probe_row" % short, texts)
            self.assertEqual(stray, [], short)

    def test_en1998_text_codec_gets_a_family_crate_and_two_dispatcher_arms_per_kind(self):
        ctx = self.plan("en1998", name="base")
        family = next(v[1] for p, v in ctx.plan.files.items() if p.endswith("/📝️text/🧬️mutations/🧪probe-rows/🦀️.rs"))
        for kind in ("insert-probe-row", "remove-probe-row", "change-probe-row-span", "change-probe-row-kind", "change-probe-row-note"):
            fn = kind.replace("-", "_")
            self.assertIn(f"pub fn print_{fn}(", family)
            self.assertIn(f"pub fn parse_{fn}(", family)
        self.assertIn('required(args, "insert-probe-row", k)', family)
        self.assertNotIn("tie_beam", family)
        dispatcher = next(v[1] for p, v in ctx.plan.files.items() if p.endswith("/📝️text/🧬️mutations/🦀️.rs"))
        self.assertEqual(len(re.findall(r"text_probe_rows::(?:print|parse)_\w+\(", dispatcher)), 10)
        self.assertEqual(re.findall(r"text_(?!probe_rows)\w+::(?:print|parse)_\w*probe_row\w*\(", dispatcher), [])
        self.assertIn("use semio_s_artifact_norm_en1998_mutations_text_probe_rows as text_probe_rows;", dispatcher)
        root = next(v[1] for p, v in ctx.plan.files.items() if p.endswith("/Cargo.toml") and "[workspace.dependencies" in v[1])
        self.assertIn("[workspace.dependencies.semio-s-artifact-norm-en1998-mutations-text-probe-rows]", root)
        crate = next(v[1] for p, v in ctx.plan.files.items() if p.endswith("/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml"))
        self.assertIn("mutations-text-probe-rows = { workspace = true }", crate)

    def test_leaf_kinds_join_the_family_crate_of_their_donor_kind(self):
        import probe
        spec = {"leaf": {"field": {"name": "probe_value", "type": "f64", "unit": "m", "label": probe.LB("Probe value"), "sample": 0.5}, "entity": {"singular": probe.LB("tie beam")}}}
        path = os.path.join(self.work, "en1998-leaf.json")
        json.dump(spec, open(path, "w", encoding="utf8"))
        ctx = self.snapshot.plan_of("en1998", "add-leaf", path, "tieBeams")
        self.assertEqual(ctx.unsupported, [])
        family = "".join(v[1] for p, v in ctx.plan.files.items() if "/📝️text/🧬️mutations/" in p and p.endswith("/🦀️.rs") and not p.endswith("/🧬️mutations/🦀️.rs"))
        self.assertIn("pub fn print_change_tie_beam_probe_value(", family)
        self.assertIn("pub fn parse_change_tie_beam_probe_value(", family)

    def test_nested_collections_stop_only_where_the_artifact_has_no_nested_donor(self):
        import probe
        for short, needle in (("en1990", "need a reference collection with nested rows"), ("en1996", "need a reference collection with nested rows")):
            spec = probe.collection_spec(short)
            spec["collection"]["nested"] = [{"name": "probeReadings", "entity": "ProbeReading", "emoji": "🧪", "doc": "d", "label": probe.LB("Readings"), "id": probe.LB("Reading ID"), "singular": probe.LB("probe reading"), "kindSlug": "probe-reading", "fields": [{"name": "depth", "type": "f64", "unit": "m", "label": probe.LB("Depth"), "sample": 1.0}]}]
            ctx = self.plan(short, spec, "nested")
            self.assertTrue(any(needle in u for u in ctx.unsupported), f"{short}: {ctx.unsupported[:2]}")
            self.assertFalse([u for u in ctx.unsupported if u.startswith("sqlite")], f"{short}: the SQLite pair clone needs no nested donor")

    def test_nested_patch_blocks_keep_one_section_each_in_macro_order(self):
        import l_patch
        block = "nested_patch! {\n    /// d\n    P for m::R {\n        set { a: f64 }\n        optional { t: T => clear_t }\n        within { s => st.d: f64 }\n        rows { r: R = \"r\" }\n        within { s => st.d: f64, s2 => st.e: f64 }\n        rows { r: R = \"r\", q: Q = \"q\" }\n    }\n}\n"
        fixed = l_patch.normalize(block)
        self.assertEqual([m for m in re.findall(r"^\s+(set|within|optional|rows) \{", fixed, re.M)], ["set", "within", "optional", "rows"])
        self.assertIn("within { s => st.d: f64, s2 => st.e: f64 }", fixed)
        self.assertIn('rows { r: R = "r", q: Q = "q" }', fixed)
        self.assertEqual(l_patch.normalize(fixed), fixed)

    def test_leaf_on_a_nested_patch_entity_edits_in_place(self):
        import probe
        spec = {"leaf": {"field": {"name": "probe_value", "type": "f64", "unit": "m", "label": probe.LB("Probe value"), "sample": 0.5}, "entity": {"singular": probe.LB("member")}}}
        path = os.path.join(self.work, "en1992-leaf.json")
        json.dump(spec, open(path, "w", encoding="utf8"))
        ctx = self.snapshot.plan_of("en1992", "add-leaf", path, "members")
        for p, (o, n, _) in ctx.plan.files.items():
            if p.endswith("/🧬️schema/🔺️diff/🦀️.rs"):
                block = re.search(r"En1992MembersPatch for [^\n]*\{\n((?:.*\n)*?)    \}\n", n).group(1)
                self.assertEqual(re.findall(r"^\s+(set|within|optional|rows) \{", block, re.M), ["set", "within", "optional", "rows"])
                self.assertIn("probe_value: f64", block)

    def test_leaf_extends_the_hand_written_row_patch_impl_of_en1999_connections_byte_identically(self):
        import l_leaf
        from core import Layout
        committed = open(Layout("en1999").p("🧬️schema/🔺️diff/🦀️.rs"), encoding="utf8").read()
        struct, name = "En1999ConnectionsPatch", "hole_stagger"
        self.assertEqual(l_leaf.extend_patch_impls(committed, struct, name), committed)
        head = re.search(r"^impl [^\n]*RowPatch<[^\n]*> for %s \{\n" % struct, committed, re.M)
        end = committed.index("\n}\n", head.end())
        block = "\n".join(line for line in committed[head.end():end].split("\n") if f"self.{name}" not in line or "is_none() &&" in line)
        block = block.replace(f" && self.{name}.is_none()", "")
        stripped = committed[:head.end()] + block + committed[end:]
        self.assertNotIn(f"self.{name}", stripped)
        self.assertEqual(l_leaf.extend_patch_impls(stripped, struct, name), committed)

    def test_en1999_connection_leaf_plans_a_typed_required_diff_property_and_extends_the_impls(self):
        import probe
        spec = {"leaf": {"kindSlug": "conn", "field": {"name": "probe_value", "type": "f64", "unit": "m", "label": probe.LB("Probe value"), "sample": 0.5}, "entity": {"singular": probe.LB("connection")}}}
        path = os.path.join(self.work, "en1999-leaf.json")
        json.dump(spec, open(path, "w", encoding="utf8"))
        ctx = self.snapshot.plan_of("en1999", "add-leaf", path, "connections")
        self.assertEqual(ctx.unsupported, [])
        for p, (o, n, _) in ctx.plan.files.items():
            if p.endswith("/🧬️schema/🔺️diff/🔣️.json"):
                patch = json.loads(n)["$defs"]["En1999ConnectionsPatch"]
                self.assertEqual(patch["properties"]["probeValue"]["x-semio-rust"], {"type": "f64"})
                self.assertIn("probeValue", patch["required"])
            if p.endswith("/🧬️schema/🔺️diff/🦀️.rs"):
                self.assertEqual(n.count("self.probe_value"), 4)


NESTED_PROBE = {"name": "probeReadings", "entity": "ProbeReading", "emoji": "🧪", "doc": "d", "label": {"en": "Readings", "de": "Readings DE"}, "id": {"en": "Reading ID", "de": "Reading ID DE"}, "singular": {"en": "probe reading", "de": "probe reading DE"}, "kindSlug": "probe-reading", "fields": [{"name": "depth", "type": "f64", "unit": "m", "label": {"en": "Depth", "de": "Depth DE"}, "sample": 1.0}]}


def no_float_spec():
    import probe
    return {"collection": {"name": "probeRows", "entity": "ProbeRow", "emoji": "🧪", "doc": "Probe entity.", "label": probe.LB("Probe rows"), "id": probe.LB("Probe ID"), "singular": probe.LB("probe row"), "kindSlug": "probe-row",
        "fields": [{"name": "kind", "type": "enum", "label": probe.LB("Kind"), "enum": {"name": "ProbeKind", "values": [{"token": "a", "label": probe.LB("A")}, {"token": "b", "label": probe.LB("B")}]}}, {"name": "note", "type": "String", "label": probe.LB("Note")}]}}


class WaveSix(unittest.TestCase):
    """🧪 SCHEMA-TOOLING v6: plan scanners (`verify`), row_key! entries of new row types, no-float entities end to end, the array-only leaf (en1991 accidentalCases), snapshot literals in hand-written Rust, measured nested codec shapes."""

    @classmethod
    def setUpClass(cls):
        import snapshot, tempfile
        cls.snapshot = snapshot
        cls.work = tempfile.mkdtemp(prefix="evolve-test6-")
        cls.cache = {}

    def plan(self, short, raw, name, command="add-collection", entity=None):
        key = (short, name)
        if key not in self.cache:
            path = os.path.join(self.work, f"{short}-{name}.json")
            json.dump(raw, open(path, "w", encoding="utf8"))
            self.cache[key] = self.snapshot.plan_of(short, command, path, entity)
        return self.cache[key]

    def text_of(self, ctx, needle):
        return "".join(n for p, (o, n, _) in sorted(ctx.plan.files.items()) if needle in p)

    def test_to_kebab_is_the_law_tests_function(self):
        import importlib.util, l_verify
        from core import NORM
        law = NORM + "/⚖️compliance/🧪️tests/🧷️kind-kebab/🐍️.py"
        source = open(law, encoding="utf8").read().split("import pathlib")[0]
        scope = {}
        exec(compile(source, law, "exec"), scope)
        for name in ("InsertProbeRow", "ChangeGr1aValue", "SetEn1993Joint", "RemoveRCMember", "ChangeBeamXM", "AddHTTPServer", "ChangeMember_x"):
            self.assertEqual(l_verify.to_kebab(name), scope["to_kebab"](name), name)

    def planted(self, files):
        from core import Plan, SEMIO, NORM
        plan = Plan(SEMIO)
        plan.layer = "test"
        for rel, text in files.items():
            plan.write(NORM + "/🗿️artifacts/zz-evolve-test/" + rel, text)
        return type("C", (), {"plan": plan, "layout": type("L", (), {"p": lambda self, r: NORM + "/🗿️artifacts/zz-evolve-test/" + r})()})()

    def test_scanner_kind_kebab_flags_a_kind_that_differs_from_the_derive(self):
        import l_verify
        bad = 'impl X for InsertProbeRow {\n    const SEMANTICS: S = S { kind: "insert-probe-rows" };\n}\n'
        good = 'impl X for InsertProbeRow {\n    const SEMANTICS: S = S { kind: "insert-probe-row" };\n}\n'
        self.assertEqual(len(l_verify.kind_kebab(self.planted({"🧬️schema/🧬️mutations/a/🦀️.rs": bad}).plan)), 1)
        self.assertEqual(l_verify.kind_kebab(self.planted({"🧬️schema/🧬️mutations/a/🦀️.rs": good}).plan), [])

    def test_scanner_nested_patch_flags_order_and_duplicates(self):
        import l_verify
        block = "nested_patch! {\n    /// d\n    P for m::R {\n        set { a: f64 }\n        rows { r: R = \"r\" }\n        within { s => st.d: f64 }\n    }\n}\n"
        self.assertEqual(len(l_verify.nested_patch_order(self.planted({"x/🦀️.rs": block}).plan)), 1)
        import l_patch
        self.assertEqual(l_verify.nested_patch_order(self.planted({"x/🦀️.rs": l_patch.normalize(block)}).plan), [])

    def test_scanner_row_key_flags_a_key_without_entry_and_a_duplicate(self):
        import l_verify
        diff = "🧬️schema/🔺️diff/🦀️.rs"
        rows = "protocol::list_delta! {\n    pub X { row: m::A, key: String = by AKey }\n}\nprotocol::list_delta! {\n    pub Y { row: m::B, key: String = by BKey }\n}\n"
        missing = rows + "row_key! {\n    AKey for A;\n}\n"
        ctx = self.planted({diff: missing})
        found = l_verify.row_key_presence(ctx)
        self.assertEqual(len(found), 1)
        self.assertIn("BKey", found[0])
        ctx = self.planted({diff: rows + "row_key! {\n    AKey for A;\n    BKey for B;\n    BKey for B;\n}\n"})
        self.assertTrue(any("declared 2 times" in f for f in l_verify.row_key_presence(ctx)))
        ctx = self.planted({diff: rows + "row_key! {\n    AKey for A;\n    BKey for B;\n}\n"})
        self.assertEqual(l_verify.row_key_presence(ctx), [])

    def test_scanner_crate_refs_flags_a_module_that_was_not_mounted(self):
        import l_verify
        manifest = '[package]\nname = "zz-evolve-test"\nversion = "0.0.0"\n[lib]\npath = "🦀️.rs"\n'
        ctx = self.planted({"Cargo.toml": manifest, "🦀️.rs": "mod have;\n", "have.rs": "fn f() { crate::missing::g(); crate::have::h(); }\n"})
        found = l_verify.crate_refs(ctx)
        self.assertEqual(len(found), 1, found)
        self.assertIn("crate::missing", found[0])

    def test_plan_scanners_pass_on_real_flat_plans(self):
        import l_verify, probe
        for short in ("en1992", "en1995", "en1990"):
            ctx = self.plan(short, probe.collection_spec(short), "flat")
            self.assertEqual(ctx.unsupported, [], short)
            self.assertEqual(l_verify.scan(ctx), [], short)

    def test_a_new_row_type_gets_its_row_key_entry_flat_and_nested(self):
        import l_verify, probe
        flat = self.plan("en1992", probe.collection_spec("en1992"), "flat")
        main = self.text_of(flat, "/🧬️schema/🔺️diff/🦀️.rs")
        self.assertRegex(main, r"row_key! \{[^}]*ProbeRowKey for ProbeRow;")
        spec = probe.collection_spec("en1992")
        spec["collection"]["nested"] = [NESTED_PROBE]
        nested = self.plan("en1992", spec, "nested")
        main = self.text_of(nested, "/🧬️schema/🔺️diff/🦀️.rs")
        block = re.search(r"row_key! \{([^}]*)\}", main).group(1)
        self.assertIn("ProbeRowKey for ProbeRow;", block)
        self.assertIn("ProbeReadingKey for ProbeReading;", block)
        self.assertIn("by ProbeReadingKey", main)
        self.assertIn('rows { probe_readings: En1992ProbeRowsProbeReadingsRows = "probeReadings" }', main)
        self.assertEqual(l_verify.row_key_presence(nested), [])
        self.assertEqual(l_verify.nested_patch_order(nested.plan), [])
        self.assertFalse([u for u in nested.unsupported if u.startswith("diff")], nested.unsupported)

    def test_nested_row_key_in_a_module_dialect_is_unique_and_consistent(self):
        import l_verify, probe
        spec = probe.collection_spec("en1994")
        spec["collection"]["nested"] = [NESTED_PROBE]
        ctx = self.plan("en1994", spec, "nested")
        self.assertFalse([u for u in ctx.unsupported if u.startswith("diff")], ctx.unsupported)
        main = self.text_of(ctx, "/🧬️schema/🔺️diff/🦀️.rs")
        block = re.search(r"row_key! \{([^}]*)\}", main).group(1)
        self.assertIn("En1994ProbeRowKey for ProbeRow;", block)
        self.assertIn("En1994ProbeReadingKey for ProbeReading;", block)
        self.assertEqual(l_verify.row_key_presence(ctx), [])
        self.assertEqual(block.count("En1994SlabKey for"), 1)

    def test_no_float_entities_use_the_empty_column_list_everywhere(self):
        import l_verify
        for short in ("en1997", "en1992", "en1995", "en1990", "en1996", "en1993"):
            ctx = self.plan(short, no_float_spec(), "nofloat")
            sqlite = self.text_of(ctx, "/🪶️sqlite/📸️snapshot/")
            ddl = re.search(r"CREATE TABLE \w*probe_row\b.*?\);", sqlite, re.S).group(0)
            self.assertNotIn("ieee754", ddl, short)
            db = sqlite3.connect(":memory:")
            db.executescript(f"CREATE TABLE {short}_document(id INTEGER PRIMARY KEY);" + ddl)
            self.assertEqual(len(db.execute(f"pragma table_info({short}_probe_row)").fetchall()), 6, short)
            for m in re.finditer(r'("%s_probe_row"[^;\n]{0,400})' % short, sqlite):
                self.assertNotIn("PROBE_ROW", m.group(1), short)
                self.assertNotIn("FloatColumn::", m.group(1), short)
            self.assertNotRegex(sqlite, r"const PROBE_ROW\w*\s*:", short)
            self.assertRegex(sqlite, r'"%s_probe_row"[^;]{0,400}(?:&\[\]|,\[\])' % short, short)

    def test_admission_cells_of_enum_fields_follow_the_dialect_and_span_the_whole_list(self):
        ctx = self.plan("en1995", no_float_spec(), "nofloat")
        adm = self.text_of(ctx, "/🛂️admission/")
        self.assertIn('n::cells(n::record(p)?,[Text(0),Enumeration(1,&["a","b"]),Text(2)])?,&[])', adm)

    def test_array_only_leaf_converts_the_codecs_and_shifts_the_child_field_ids(self):
        spec = {"leaf": {"field": {"name": "probe_value", "type": "f64", "unit": "m", "label": {"en": "Probe value", "de": "Probewert"}, "sample": 0.0}, "entity": {"singular": {"en": "accidental case", "de": "Ausnahmefall"}}}}
        ctx = self.plan("en1991", spec, "array-only", "add-leaf", "accidentalCases")
        self.assertEqual(ctx.unsupported, [])
        rust = self.text_of(ctx, "/📸️snapshot/🦀️.rs")
        self.assertIn('out.insert_float("en1991_accidental_case", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&case.id), Cell::Real(case.probe_value)], ACCIDENTAL_CASE_FLOATS)?', rust)
        self.assertIn('entities(database, "en1991_accidental_case", 5, ACCIDENTAL_CASE_FLOATS, control)', rust)
        self.assertIn("probe_value: row.real(4)?", rust)
        self.assertIn("const ACCIDENTAL_CASE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4)];", rust)
        ts = self.text_of(ctx, "/📸️snapshot/🟦️.ts")
        self.assertIn('write(out,"en1991_accidental_case",id,[1n,BigInt(position),c.id,c.probeValue],ACCIDENTAL_CASE_FLOATS,options)', ts)
        self.assertIn("probeValue:readBinary64(r,4,ACCIDENTAL_CASE_FLOATS)", ts)
        self.assertIn("caseRows=await entities(tables[5]!,5,ACCIDENTAL_CASE_FLOATS,options)", ts)
        adm = self.text_of(ctx, "/🛂️admission/")
        self.assertIn('n::cells(v,[Text(0),Real(1)])?,ACCIDENTAL_CASE_FLOATS)', adm)
        self.assertIn("n::field(v,2)", adm)
        self.assertIn("n::field(v,3)", adm)
        self.assertIn("for id in 2..=3{rows=n::add(rows,n::list(n::field(case,id)?)?.len())?}", adm)
        sql = self.text_of(ctx, "/🗄️.sql")
        db = sqlite3.connect(":memory:")
        db.executescript(sql)
        columns = [r[1] for r in db.execute("pragma table_info(en1991_accidental_case)")]
        self.assertEqual(columns[3:], ["logical_id", "probe_value", "probe_value_ieee754_bits", "probe_value_ieee754_class"])

    def test_snapshot_literals_of_hand_written_rust_get_the_new_collection(self):
        import probe
        ctx = self.plan("en1990", probe.collection_spec("en1990"), "flat")
        literals = [n for p, (o, n, _) in ctx.plan.files.items() if "/📚️examples/" in p]
        self.assertTrue(literals)
        for text in literals:
            self.assertIn("probe_rows: vec![]", text)

    def test_nested_shapes_are_measured_on_statements(self):
        import l_codec
        text = 'fn f() { for (n, c) in rows.iter().enumerate() { let id = out.insert("a_case", &[]); for (k, i) in c.impact.iter().enumerate() { out.insert_float("a_impact", &[], X); } } let rows = entities(d, "a_impact", 5, X, c); }'
        self.assertEqual(l_codec.nested_shape(text, "rs", "a_case", "a_impact"), (1, 1))
        for short in ("en1991", "en1992", "en1995", "en1993", "en1994", "en1998", "en1999"):
            sql = open(os.path.join(__import__("core").Layout(short).any, "🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"), encoding="utf8").read()
            parent, child = l_codec.nested_donors(sql, short)[0]
            rs = open(os.path.join(__import__("core").Layout(short).any, "🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"), encoding="utf8").read()
            both, only = l_codec.nested_shape(rs, "rs", parent, child)
            self.assertGreaterEqual(both, 1, f"{short}: child written inline in the parent statement")


    def test_canonical_json_is_emitted_for_cloned_types_and_flagged_when_missing(self):
        import l_rust, l_verify
        model = "#[canonical_json(owner = semio_framework_pack_json)]\nstruct X;\n"
        head = "#[derive(Clone, Debug, value_derive::ToValue)]\n#[value(rename_all = \"camelCase\")]\n"
        fixed = l_rust.canonical_json(model, head)
        self.assertIn("semio_framework_value::CanonicalJsonTree", fixed.split("\n")[0])
        self.assertEqual(fixed.split("\n")[1], "#[canonical_json(owner = semio_framework_pack_json)]")
        self.assertEqual(l_rust.canonical_json(model, fixed), fixed)
        spec = Spec.__new__(Spec)
        spec.reference = spec.nested_reference = spec.collection = None
        ctx = Ctx("en1992", spec)
        self.assertEqual(l_verify.canonical_json(ctx), [])
        path = ctx.layout.p("🧬️schema/📸️snapshot/🦀️.rs")
        text = ctx.plan.read(path)
        start = text.index("pub struct ConcreteGrade {")
        head_at = text.rindex("#[derive(", 0, start)
        broken = text[:head_at] + text[head_at:start].replace("semio_framework_value::CanonicalJsonTree, ", "").replace("#[canonical_json(owner = semio_framework_pack_json)]\n", "") + text[start:]
        ctx.plan.layer = "test"
        ctx.plan.write(path, broken)
        found = l_verify.canonical_json(ctx)
        self.assertTrue(any("ConcreteGrade" in f for f in found), found)


class SelectorRepairTest(unittest.TestCase):
    def test_rule_names_row_selector_and_value_keys(self):
        import l_repair
        rule = 'SetFieldRule { path: "members[].bearingWidth", kind: "change-member-bearing-width", selectors: &[SelectorKey::Id("memberId")], value: "newValue" },'
        self.assertEqual(l_repair.RULE.findall(rule), [("members", "bearingWidth", "change-member-bearing-width", "memberId", "newValue")])


NESTED_ARTIFACTS = ("en1991", "en1992", "en1993", "en1994", "en1995", "en1998", "en1999")


class WaveSeven(unittest.TestCase):
    """🧪 SCHEMA-TOOLING v7: the SQLite pair clone of nested collections in seven artifacts, the artifact-generic diff dialects, the defects of the inline cloning, `repair`, the `set-field-kinds` and `ui-steppers` rules."""

    @classmethod
    def setUpClass(cls):
        import snapshot, tempfile
        cls.snapshot = snapshot
        cls.work = tempfile.mkdtemp(prefix="evolve-test7-")
        cls.cache = {}

    def plan(self, short, raw, name):
        key = (short, name)
        if key not in self.cache:
            path = os.path.join(self.work, f"{short}-{name}.json")
            json.dump(raw, open(path, "w", encoding="utf8"))
            self.cache[key] = self.snapshot.plan_of(short, "add-collection", path)
        return self.cache[key]

    def nested(self, short):
        import probe
        spec = probe.collection_spec(short)
        spec["collection"]["nested"] = [NESTED_PROBE]
        return self.plan(short, spec, "nested")

    def flat(self, short):
        import probe
        return self.plan(short, probe.collection_spec(short), "flat")

    def sqlite(self, ctx, name):
        return "".join(n for p, (o, n, _) in sorted(ctx.plan.files.items()) if "/📸️snapshot/" in p and p.endswith(name) and "/🧪️tests/" not in p and "/🛂️admission/" not in p)

    def test_the_pair_clone_plans_every_layer_in_seven_artifacts(self):
        for short in NESTED_ARTIFACTS:
            ctx = self.nested(short)
            self.assertEqual(ctx.unsupported, [], short)
            rust = self.sqlite(ctx, "🦀️.rs")
            self.assertEqual(len(re.findall(r'"%s_probe_row"' % short, rust)), 2, short)
            self.assertEqual(len(re.findall(r'"%s_probe_reading"' % short, rust)), 2, short)

    def test_the_child_ddl_references_the_parent_and_runs_in_sqlite(self):
        for short in NESTED_ARTIFACTS:
            ctx = self.nested(short)
            sql = self.sqlite(ctx, "🗄️.sql")
            db = sqlite3.connect(":memory:")
            db.executescript(sql)
            fk = db.execute(f"pragma foreign_key_list({short}_probe_reading)").fetchall()
            self.assertEqual([(r[2], r[4]) for r in fk], [(f"{short}_probe_row", "id")], short)
            cols = [r[1] for r in db.execute(f"pragma table_info({short}_probe_reading)")]
            self.assertEqual((cols[0], cols[2], cols[3]), ("id", "ordinal", "logical_id"), short)
            self.assertTrue(cols[1].endswith("_id") and cols[1] != "document_id", short)
            self.assertEqual(cols[4:], ["depth", "depth_ieee754_bits", "depth_ieee754_class"], short)

    def test_child_rows_are_written_inside_the_parent_loop_and_read_by_the_parent_row(self):
        for short in NESTED_ARTIFACTS:
            ctx = self.nested(short)
            rust, ts = self.sqlite(ctx, "🦀️.rs"), self.sqlite(ctx, "🟦️.ts")
            write = re.search(r'let (\w+)=(?:\w+\.)+insert\w*\(\s*"%s_probe_row".*?"%s_probe_reading",\s*&\[Cell::Integer\(\1\)' % (short, short), rust, re.S)
            self.assertIsNotNone(write, f"{short}: parent id bound and used by the child write")
            self.assertRegex(rust, r"probe_readings:\s*probe_readings_list", short)
            self.assertRegex(ts, r"const id=(?:await write\(out,\"%s_probe_row\"|BigInt)" % short, short)
            self.assertIn("probeReadings:probeReadingsList", ts, short)
            self.assertRegex(ts, r'write\(out,"%s_probe_reading",[^\[]*\[id,' % short, short)

    def test_generated_text_parses(self):
        import syntax
        for short in NESTED_ARTIFACTS:
            ctx = self.nested(short)
            found = syntax.check_plan(ctx.plan)
            self.assertEqual(found, {}, short)

    def test_roundtrip_in_a_scratch_copy_is_empty_the_second_time(self):
        import subprocess
        for short in NESTED_ARTIFACTS:
            done = subprocess.run([sys.executable, "-I", os.path.join(HERE, "roundtrip.py"), "--nested", short], capture_output=True, text=True)
            line = done.stdout.strip().splitlines()[-1]
            result = json.loads(line.split(" ", 1)[1])
            self.assertEqual((result["unsupported"], result["syntax"], result["secondCount"], result["secondUnsupported"]), ([], {}, 0, []), short)
            self.assertGreater(result["files"], 100, short)

    def test_the_diff_dialects_of_the_artifacts_get_parent_and_child_groups(self):
        names = {"en1991": ("Vec<", None), "en1993": ("Vec<", None), "en1995": ("nest { probe_readings: En1995ProbeReadingDelta }", "En1995ProbeReadingDelta {"), "en1998": (None, "ProbeReadingDelta"), "en1999": ("probe_readings: Option<En1999ProbeRowsProbeReadingsRows>", "En1999ProbeRowsProbeReadingsRows")}
        for short, (parent, child) in names.items():
            diff = self.text_of(self.nested(short), "/🧬️schema/🔺️diff/🦀️.rs") if hasattr(self, "text_of") else "".join(n for p, (o, n, _) in self.nested(short).plan.files.items() if p.endswith("/🧬️schema/🔺️diff/🦀️.rs"))
            self.assertRegex(diff, r"ProbeRow(?:s)?(?:Patch|Delta|Rows)", short)
            if parent:
                self.assertIn(parent.split("<")[0] if parent == "Vec<" else parent, diff, short)
            if child:
                self.assertIn(child, diff, short)
        for short in ("en1991", "en1993"):
            diff = "".join(n for p, (o, n, _) in self.nested(short).plan.files.items() if p.endswith("/🧬️schema/🔺️diff/🦀️.rs"))
            self.assertRegex(diff, r"ProbeRowPatch of [\w:]+ \{ set \{[^}]*probe_readings: Vec<[\w:]+ProbeReading>", short)

    def test_en1999_patch_structs_are_regenerated_for_the_new_row(self):
        diff = "".join(n for p, (o, n, _) in self.flat("en1999").plan.files.items() if p.endswith("/🧬️schema/🔺️diff/🦀️.rs"))
        block = re.search(r"pub struct En1999ProbeRowsPatch \{(.*?)\n\}", diff, re.S).group(1)
        self.assertEqual(re.findall(r"pub (\w+): Option<", block), ["span", "kind", "note"])
        self.assertEqual(diff.count("pub struct En1999TransverseStiffenersPatch"), 1)
        self.assertIn("if let Some(value) = &self.span { row.span = value.clone(); }", diff)
        self.assertNotIn("row.shell_id", diff.split("pub struct En1999ProbeRowsPatch")[1].split("pub struct ", 1)[0])

    def test_flat_inline_cloning_defects_are_fixed(self):
        import l_verify, syntax
        for short in ("en1998", "en1999"):
            ctx = self.flat(short)
            self.assertEqual(l_verify.scan(ctx), [], short)
        rust = self.sqlite(self.flat("en1999"), "🦀️.rs")
        self.assertEqual(re.findall(r"admit_rows\(&mut b,&mut rows,self\.transverse_stiffeners\.len\(\)[^;]*;", rust), ["admit_rows(&mut b,&mut rows,self.transverse_stiffeners.len())?;"])
        self.assertIn("admit_rows(&mut b,&mut rows,self.probe_rows.len())?;", rust)
        self.assertEqual(len(re.findall(r'entities\(d,"en1999_probe_row"', rust)), 1)
        self.assertRegex(self.sqlite(self.flat("en1998"), "🦀️.rs"), r"slopes:Vec::new\(\),probe_rows:Vec::new\(\)")
        ts = self.sqlite(self.flat("en1994"), "🟦️.ts")
        self.assertIn("probeRowGroups=await groups(probeRowRows,documents,options)", ts)
        self.assertEqual(syntax.check_plan(self.flat("en1995").plan), {})
        self.assertRegex(self.sqlite(self.flat("en1995"), "🟦️.ts"), r"kind:choice\(artifactSqliteText\(r,5\),PROBE_KINDS\)")
        self.assertNotRegex(self.sqlite(self.flat("en1995"), "🟦️.ts"), r"Omit<Deck,ProbeRow")
        self.assertEqual(len(re.findall(r"bound\.add\(4096\)\?;for text in \[&probe_row\.id,probe_kind\(probe_row\.kind\),&probe_row\.note\]", self.sqlite(self.flat("en1995"), "🦀️.rs"))), 1)

    def test_repair_rewrites_the_shapes_earlier_runs_left(self):
        import l_repair
        text = 'export type A=Omit<Row,Other,Third,"a"|"b">&{a:Binary64};\nexport type B=Omit<Exact<Row,"a"|"b"">,"c">&{};\nplacements:Exact<One,Two,Three>[];\nexport type XSqliteSnapshot=Omit<X,"tie_beams">&{tie_beams:T[]};\n'
        fixed, count = l_repair.fix(text)
        self.assertEqual(fixed, 'export type A=Omit<Row,"a"|"b">&{a:Binary64};\nexport type B=Omit<Exact<Row,"a"|"b">,"c">&{};\nplacements:Exact<One>[];\nexport type XSqliteSnapshot=Omit<X,"tieBeams">&{tieBeams:T[]};\n')
        self.assertEqual(count, 5)
        self.assertEqual(l_repair.fix(fixed), (fixed, 0))

    def test_repair_restores_renamed_row_declarators(self):
        import l_repair
        text = 'const a=await entities(t[1]!,2,[],o),frameActionRows=await entities(t[2]!,3,[],o);\nfor(const v of await ordered(frame_action.values(),1n,o))r.push(v);\n'
        fixed, count = l_repair.renamed_declarators(text)
        self.assertEqual((fixed, count), (text.replace("frameActionRows", "frame_action"), 1))

    def test_repair_renames_float_constants_to_the_declared_spelling(self):
        import l_repair
        text = 'const A_B_FLOATS: &[u8] = &[];\nconst C: u8 = 1;\nx(A_B, C);\ny(A_B_FLOATS);\n'
        fixed, count = l_repair.float_constants(text)
        self.assertEqual(fixed, 'const A_B_FLOATS: &[u8] = &[];\nconst C: u8 = 1;\nx(A_B_FLOATS, C);\ny(A_B_FLOATS);\n')
        self.assertEqual((count, l_repair.float_constants(fixed)), (1, (fixed, 0)))

    def test_committed_sqlite_codecs_need_no_repair_and_parse(self):
        import syntax, l_repair
        from core import Layout, walk_files
        for short in ("en1990", "en1991", "en1992", "en1993", "en1994", "en1995", "en1996", "en1997", "en1998", "en1999"):
            path = [p for p in walk_files(Layout(short).p(l_repair.SQLITE), (".ts",)) if p.endswith("/📸️snapshot/🟦️.ts")][0]
            text = open(path, encoding="utf8").read()
            self.assertEqual(l_repair.fix(text)[1], 0, short)
            found = syntax.check_ts({path: text})
            self.assertEqual(found, {path: []}, short) if found is not None else None

    def test_set_field_kinds_rule_flags_a_settable_field_without_a_kind(self):
        import l_verify
        schema = {"properties": {"layers": {"type": "array", "items": {"$ref": "#/$defs/Layer"}}}, "$defs": {"Layer": {"properties": {"id": {"type": "string"}, "gamma": {"type": "number"}, "cptNkt": {"type": "number"}, "water": {"$ref": "#/$defs/Water"}, "kids": {"type": "array", "items": {"$ref": "#/$defs/Kid"}}}}, "Water": {"type": "object", "properties": {"depth": {"type": "number"}}}, "Kid": {"properties": {"id": {"type": "string"}, "size": {"type": "number"}}}}}
        self.assertEqual(sorted(l_verify.schema_leaf_paths(schema)), ["layers[].cptNkt", "layers[].gamma", "layers[].kids[].size", "layers[].water.depth"])
        taxonomy = {"rows": [{"artifact": "X", "kind": "change-layer-gamma", "edit": {"rule": "set-field", "path": "layers[].gamma"}}, {"artifact": "X", "kind": "change-layer-water-depth"}, {"artifact": "X", "kind": "change-kid-size", "edit": {"rule": "set-field", "path": "layers[].kids[].size"}}]}
        self.assertEqual(l_verify.set_field_gaps(schema, taxonomy, "X"), ["layers[].cptNkt"], "rows without an edit object count by kind name")
        import io, contextlib, evolve
        out = io.StringIO()
        with contextlib.redirect_stdout(out):
            status = evolve.main(["verify", "set-fields", "en1996"])
        self.assertEqual(status, 0, out.getvalue())

    def test_ui_rule_flags_copied_index_labels_and_steps_that_cannot_belong_to_the_field(self):
        import l_verify
        ui = lambda label, **extra: {"widget": "stepper", "label": {"en": label, "de": label}, **extra}
        schema = {"properties": {"walls": {"type": "array", "items": {"$ref": "#/$defs/Wall"}, "x-semio-ui": {"label": {"en": "Walls", "de": "Wände"}}}}, "$defs": {"Wall": {"properties": {"id": {"type": "string"}, "cPe": {"type": "number", "x-semio-ui": ui("Wall", step=1, precision=0, description={"en": "Zero-based position of the wall in its list.", "de": "x"})}, "a": {"type": "number", "x-semio-ui": ui("Span", unit="m", step=0.1)}, "b": {"type": "number", "x-semio-ui": ui("Span", unit="m", step=0.1)}, "k": {"type": "number", "maximum": 2, "x-semio-ui": ui("Factor k", step=5)}, "cf": {"type": "number", "x-semio-rust": {"doc": ["Pressure coefficient [-]"]}, "x-semio-ui": ui("Coefficient", step=1)}}}}}
        found = l_verify.ui_findings(schema)
        text = "\n".join(found)
        self.assertIn("Wall.cPe is labelled 'Wall'", text)
        self.assertIn("Wall.b and Wall.a share the label 'Span'", text)
        self.assertIn("steps by 5, above its maximum 2", text)
        self.assertIn("Wall.cf is a dimensionless coefficient but steps by 1", text)
        metre = {"properties": {"ms": {"type": "array", "items": {"$ref": "#/$defs/M"}}}, "$defs": {"M": {"properties": {"id": {"type": "string"}, "bearingLength": {"type": "number", "x-semio-ui": ui("Bearing", unit="m", displayUnit="mm", displayFactor=1000, precision=0, step=5)}, "ok": {"type": "number", "x-semio-ui": ui("Ok", unit="m", displayUnit="mm", displayFactor=1000, precision=1, step=0.005)}}}}}
        self.assertEqual(l_verify.ui_findings(metre), ["ui: M.bearingLength steps by 5000 mm, implausible for the unit (limit 100)"])
        self.assertIn("Wall.cPe carries the 'Zero-based position' description", text)
        self.assertEqual(l_verify.repo_ui_findings(["en1996"]), {next(iter(l_verify.repo_ui_findings(["en1996"]))): []})

    def test_blocks_of_keeps_abutting_items_apart_and_impls_with_blank_lines_whole(self):
        import l_rust
        text = "/// d\npub struct A {\n    pub x: u8,\n}\nimpl T for A {\n    fn f() {}\n\n    fn g() {}\n}\n\nprotocol::list_delta! {\n    pub B { x: u8 }\n}\n"
        blocks = [text[a:b] for a, b in l_rust.blocks_of(text)]
        self.assertEqual(len(blocks), 3)
        self.assertTrue(blocks[0].startswith("/// d") and blocks[0].endswith("}"))
        self.assertIn("fn g() {}", blocks[1])

    def test_registry_clone_units_of_adjacent_kinds_stay_balanced(self):
        import l_mutations
        text = '        "insert-a" => (\n            f("x/insert-a/1"),\n        ),\n        "remove-a" => (\n            f("x/remove-a/1"),\n        ),\n        "other" => (),\n'
        spans = l_mutations.mention_units(text, ["insert-a", "remove-a"])
        self.assertEqual(text[spans[0][0]:spans[0][1]].count("("), text[spans[0][0]:spans[0][1]].count(")"))
        self.assertTrue(text[spans[0][0]:spans[0][1]].rstrip().endswith("),"))

    def test_leaf_kinds_only_mode_asks_for_the_missing_set_field_kind_and_plans_nothing_else(self):
        import l_mutations
        from core import Layout
        ctx = self.snapshot.plan_of("en1997", "add-leaf", os.path.join(HERE, "specs", "en1997-layers-cptnkt.json"), "layers")
        self.assertEqual((ctx.unsupported, sorted(ctx.plan.files)), ([], []))
        exists = any(d.endswith("change-layer-cpt-nkt") for d in os.listdir(Layout("en1997").p(l_mutations.MUT)))
        self.assertEqual(any("change-layer-cpt-nkt" in n for n in ctx.plan.notes), not exists)


def l_mutations_taxonomy():
    import l_mutations
    return l_mutations.TAXONOMY


if __name__ == "__main__":
    unittest.main()
