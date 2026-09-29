# 🐍️ Section F of `c12-jack-query-document-patch.py`: `set-query` joins the declared catalog and the Rust/Python differential
# (`🔌️mutate-jack-1`): the completeness gate requires a `mutate-` and an `inverse-` scenario for every catalog kind, and the
# structural-correspondence law compares the enum with the catalog.
CASE_DIR = ANY + "🧪️tests/🔌️mutate-jack-1/"
FEATURE = CASE_DIR + "🥒️.feature"
PYTHON = CASE_DIR + "🐍️.py"
SUBJECT = CASE_DIR + "🦀️.rs"
ORACLES = ANY + "🔮️oracles/🔣️.json"
CORRESPONDENCE = MUT + "🧪️tests/🔬️structural-correspondence/🦀️.rs"
SET_QUERY_PAYLOAD = '{"mutation":"setQuery","value":"MATCH (a:Piece) RETURN a.name"}'


def feature_rows(text):
    lines = text.split("\n")
    out = []
    for line in lines:
        out.append(line)
        stripped = line.strip()
        if stripped.startswith("| remove-data-property  | {") and "set-query" not in text:
            width = len(line) - len(line.rstrip())
            cells = line.split("|")
            row = "|".join([cells[0], " set-query".ljust(len(cells[1])), (" " + SET_QUERY_PAYLOAD).ljust(len(cells[2])), cells[3]])
            out.append(row)
        elif stripped.startswith("| remove-data-property | noop    |") and "| set-query            | applied |" not in text:
            cells = line.split("|")
            values = [" set-query", " applied", " 🔎️set-query", " 🔎️replaces-the-query", " 🧩️capsule-stack.scene.json"]
            out.append("|".join([cells[0]] + [value.ljust(len(cell)) for value, cell in zip(values, cells[1:6])] + cells[6:]))
    return "\n".join(out)


feature_text = read(FEATURE)
if feature_text is not None and "set-query" not in feature_text:
    files[FEATURE] = feature_rows(feature_text)
    plan.append("edit    feature set-query rows")
else:
    plan.append("present feature set-query rows")
edit(FEATURE, """  The committed vectors were KEPT, not replaced.""", """  The NINTH kind, `set-query`, joined when the Jack query became document content (2026-09-29): it edits the document's
  query, never the scene, so both implementations project the `query` member too, and its committed vector is the first
  POSITIVE one — its `verdict` is `applied`, and the vector must move the document to exactly the committed after-snapshot.

  The committed vectors were KEPT, not replaced.""", "feature ninth kind prose")

edit(PYTHON, 'MEMBERS = ("schema", "name", "camera", "nodes", "edges", "rootNodeId")', 'MEMBERS = ("schema", "name", "camera", "nodes", "edges", "rootNodeId", "query")', "python members")
edit(PYTHON, 'KINDS = ("create-node", "delete-node", "create-edge", "delete-edge", "rename-node", "move-node", "change-data-property", "remove-data-property")',
     'KINDS = ("create-node", "delete-node", "create-edge", "delete-edge", "rename-node", "move-node", "change-data-property", "remove-data-property", "set-query")', "python kinds")
edit(PYTHON, '''    "remove-data-property": "removeDataProperty",
}''', '''    "remove-data-property": "removeDataProperty",
    "set-query": "setQuery",
}''', "python tags")
edit(PYTHON, '''        properties[mutation["key"]] = copy.deepcopy(mutation["new_value"])
    else:''', '''        properties[mutation["key"]] = copy.deepcopy(mutation["new_value"])
    elif kind == "set-query":
        if result.get("query", "") == mutation["value"]:
            return result, True
        result["query"] = mutation["value"]
    else:''', "python apply set-query")
edit(PYTHON, '''    properties = bag(document, mutation["entity"], "inverse of %s" % kind)''', '''    if kind == "set-query":
        return {"mutation": TAGS[kind], "value": document.get("query", "")}
    properties = bag(document, mutation["entity"], "inverse of %s" % kind)''', "python inverse set-query")
edit(PYTHON, '''        applied, noop = apply_mutation(before, mutation)
        if not noop:
            raise AssertionError("spec-vector-%s: the committed vector declares an accepted no-op, but the mutation moved the scene" % kind)''',
     '''        applied, noop = apply_mutation(before, mutation)
        if verdict == "applied":
            if noop:
                raise AssertionError("spec-vector-%s: the committed vector declares an applied change, but the mutation was a no-op" % kind)
        elif not noop:
            raise AssertionError("spec-vector-%s: the committed vector declares an accepted no-op, but the mutation moved the scene" % kind)''', "python applied verdict")
replace_every(PYTHON, "assembly scene and all eight of", "assembly scene and all nine of", "python prose nine kinds")
replace_every(PYTHON, "— the eight verbs and their positional", "— the nine verbs and their positional", "python prose nine verbs")
replace_every(PYTHON, "* the eight committed specification vectors", "* the nine committed specification vectors", "python prose nine vectors")
replace_every(PYTHON, "would make all eight rows", "would make all nine rows", "python prose nine rows")
replace_every(PYTHON, "    All eight are NEGATIVE, so the feature's `verdict` column states which of the two refusals each one", "    Eight of the nine are NEGATIVE and `set-query`'s is applied, so the feature's `verdict` column states which answer each one", "python prose verdicts")
replace_every(FEATURE, "  all eight typed mutations, written in Python from", "  all nine typed mutations, written in Python from", "feature prose nine kinds")
replace_every(FEATURE, "(the eight verbs and their argument", "(the nine verbs and their argument", "feature prose nine verbs")
replace_every(FEATURE, "and from the eight committed", "and from the nine committed", "feature prose nine vectors")
replace_every(SUBJECT, "carrier and of all eight typed", "carrier and of all nine typed", "subject prose nine kinds")
replace_every(SUBJECT, "The projection carries both scenes, so all eight", "The projection carries both scenes, so all nine", "subject prose nine rows")
replace_every(SUBJECT, "content child. All eight are NEGATIVE, so the feature's", "content child. Eight of the nine are NEGATIVE and `set-query`'s is `applied`, so the feature's", "subject prose verdicts")

edit(SUBJECT, 'const KINDS: &[&str] = &["create-node", "delete-node", "create-edge", "delete-edge", "rename-node", "move-node", "change-data-property", "remove-data-property"];',
     'const KINDS: &[&str] = &["create-node", "delete-node", "create-edge", "delete-edge", "rename-node", "move-node", "change-data-property", "remove-data-property", "set-query"];', "subject kinds")
edit(SUBJECT, '    const MEMBERS: &[&str] = &["schema", "name", "camera", "nodes", "edges", "rootNodeId"];', '    const MEMBERS: &[&str] = &["schema", "name", "camera", "nodes", "edges", "rootNodeId", "query"];', "subject members")
edit(SUBJECT, "            if applied.content.child_id == base.content.child_id {", "            if applied.content.child_id == base.content.child_id && applied.query == base.query {", "subject mutate observability")
edit(SUBJECT, "            if current.content.child_id == base.content.child_id {", "            if current.content.child_id == base.content.child_id && current.query == base.query {", "subject inverse observability")
edit(SUBJECT, '''                "noop" => {
                    if !raised.iter().all(|(code, _)| code == "mutation.no-op") {
                        return Err(format!("spec-vector-{kind}: the committed vector declares an accepted no-op, but the implementation raised {raised:?}"));
                    }
                }''', '''                "noop" => {
                    if !raised.iter().all(|(code, _)| code == "mutation.no-op") {
                        return Err(format!("spec-vector-{kind}: the committed vector declares an accepted no-op, but the implementation raised {raised:?}"));
                    }
                }
                "applied" => {
                    if !raised.is_empty() {
                        return Err(format!("spec-vector-{kind}: the committed vector declares an applied change, but the implementation raised {raised:?}"));
                    }
                }''', "subject applied verdict")


edit(SUBJECT, """//! **What the two roles each hold.** The cross-language projection is the six members BOTH committed
//! serializations of a jack scene carry — `schema`, `name`, `camera`, `nodes`, `edges` and
//! `rootNodeId`. The composed""", """//! **What the two roles each hold.** The cross-language projection is the seven members BOTH committed
//! serializations of a jack scene carry — `schema`, `name`, `camera`, `nodes`, `edges`, `rootNodeId`
//! and the document's `query`. The composed""", "subject prose seven members")
edit(SUBJECT, "    /// 📤️ What parity compares: the six members both committed serializations of a jack scene carry.", "    /// 📤️ What parity compares: the seven members both committed serializations of a jack scene carry.", "subject projection prose seven members")
edit(PYTHON, """  `schema`, `name`, `manifestId`, `manifest`, `camera`, `nodes`, `edges` and `rootNodeId`; a `Node`""", """  `schema`, `name`, `manifestId`, `manifest`, `camera`, `nodes`, `edges`, `rootNodeId` and `query`; a `Node`""", "python prose snapshot members")
edit(PYTHON, """**A gap in the evidence, likewise reported.** ALL EIGHT committed vectors are NEGATIVE: three
rejections and five accepted no-ops. Not one of them exercises a mutation that actually changes the
scene, so the accepting direction of this entire vocabulary had no committed evidence before this
case's real-document scenarios.""", """**A gap in the evidence, likewise reported.** ALL EIGHT committed SCENE vectors are NEGATIVE: three
rejections and five accepted no-ops. Not one of them exercises a mutation that actually changes the
scene, so the accepting direction of the scene vocabulary had no committed evidence before this
case's real-document scenarios. The ninth, `set-query`'s, is the first positive one: it replaces the
document's query and leaves the scene alone.""", "python prose negative scene vectors")
edit(PYTHON, """The committed carrier writes `schema`, `name`, `manifestId`, `camera`, `nodes`, `edges` and
`rootNodeId`; the committed""", """The committed carrier writes `schema`, `name`, `manifestId`, `camera`, `nodes`, `edges`,
`rootNodeId` and `query`; the committed""", "python prose carrier members")
edit(PYTHON, """the six members above.""", """the seven members above.""", "python prose seven members")
edit(PYTHON, 'What parity compares: the six members both committed serializations carry.', 'What parity compares: the seven members both committed serializations carry.', "python projection prose seven members")


def oracle_registry(record):
    oracle = record["oracles"][0]
    oracle["rationale"] = oracle["rationale"].replace("all eight typed mutations", "all nine typed mutations")
    oracle["nativeSecondImplementation"]["fixtureCoverage"]["vectors"] = 9
    catalog = record["mutationCatalogs"][0]
    if "set-query" not in catalog["kinds"]:
        catalog["kinds"].append("set-query")
        catalog["vectors"].append({"mutationId": "set-query", "sourceMutationDirectoryName": "🔎️set-query", "mutationDirectoryName": "🔎️set-query", "scenarios": [{"id": "replaces-the-query", "directoryName": "🔎️replaces-the-query"}]})
    mutations = record["mutationManifests"][0]["mutations"]
    if not any(entry["id"] == "set-query" for entry in mutations):
        mutations.append({"id": "set-query", "capability": "jack-1-mutate", "payloadSchema": "🧬️.schema.json", "outcomes": ["applied", "no-op"], "productionDispatch": {"operation": "set-query", "bridgeVersion": 1, "variant": "SetQuery"}, "oracleRequirements": [{"capability": "jack-1-mutate", "qualifyingKind": "verified-native-second-implementation"}]})
    return record


def copy_of(record):
    return json.loads(json.dumps(record))


json_fixture(ORACLES, lambda record: oracle_registry(copy_of(record)), "oracle registry set-query")

CORRESPONDENCE_ANCHOR = '    assert!(!catalog_kinds.contains(&"set-snapshot"));\n'
correspondence_text = read(CORRESPONDENCE)
if correspondence_text is not None and 'let kind = "set-query";' not in correspondence_text:
    start = correspondence_text.index('    {\n        let kind = "rename-node";')
    end = correspondence_text.index("    }\n", correspondence_text.index('direct owner {directory} must correspond to the JSON catalog");', start)) + len("    }\n")
    block = correspondence_text[start:end].replace('"rename-node"', '"set-query"').replace('"RenameNode"', '"SetQuery"').replace('"✏️rename-node"', '"🔎️set-query"').replace("let binary_tag = 4;", "let binary_tag = 8;").replace('join("✏️rename-node")', 'join("🔎️set-query")')
    assert correspondence_text.count(CORRESPONDENCE_ANCHOR) == 1 and "rename" not in block
    files[CORRESPONDENCE] = correspondence_text.replace(CORRESPONDENCE_ANCHOR, block + CORRESPONDENCE_ANCHOR)
    plan.append("edit    correspondence set-query block")
else:
    plan.append("present correspondence set-query block")
