#!/usr/bin/env python3
"""🗂️ S19 one-off codemod (test-only, rule 22): ONE Flow/Note action-cohort census, schema-first.

Measured 2026-09-29 (LW1 `s14-lw1-logs/s19-flow-nextest-1.txt` + local runs): three readers disagreed on `routeCount` —
flow's Rust law (routes + `frameworkOwnedRoutes`, totals pinned 34/35), Note's TS law (command rows only, no
framework-owned) and flow's `action-cohort-audit` (rows = routes ∪ framework-owned) — and both fixtures had drifted from
source: Flow lacked `setActiveExample` + `setContributions` and still named an `artifact` lane (the scene moved to the
content child), Note still classified 27 verbs batch-only (every Note verb is retained since 09-17) and carried the removed
materialization-cursor block. The census is now defined ONCE in the schema: `routeCount` = the owner's own command rows,
each classified in exactly one group; framework-reserved actions are never app routes (`frameworkOwnedRoutes` is gone).
Groups are derived from source truth (command table order, publication-contract lanes, manifest classification). Every
reader reads that one rule. Idempotent. usage: s19-action-cohort.py <root>"""
import json
import os
import re
import sys

root = sys.argv[1]
FLOW = "✏️s/🔌️plugins/🌊️flow"
NOTE = "✏️s/🔌️plugins/🗒️note"
SCHEMA = f"{FLOW}/🎬️action-cohort/🧬️schema/🔣️.json"
FLOW_FIXTURE = f"{FLOW}/🧫️fixtures/🎬️action-cohort/🔣️.json"
NOTE_FIXTURE = f"{NOTE}/🧫️fixtures/🧪️action-cohort/🔣️.json"
FLOW_SOURCE = f"{FLOW}/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
NOTE_SOURCE = f"{NOTE}/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
NOTE_RETAINED = f"{NOTE}/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs"
FLOW_LAW = f"{FLOW}/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
FLOW_AUDIT = f"{FLOW}/📦️packages/🟦️typescript/📜️script.ts"
NOTE_LAW = f"{NOTE}/🧪️tests/🧭️action-cohort/🟦️.ts"
LANE = {"Artifact": "artifact", "WindowConfig": "config", "WindowTransient": "transient", "HostOnly": "host-only", "Child": "child", "Draft": "draft", "Presence": "presence"}
STATUS = {"Migrated": "migrated", "BatchOnlyPendingRewrite": "batch-only-pending-rewrite"}
STALE_NOTE_LAWS = ("exactArtifactScalarReducer", "gridSpacingInvariantPreserved", "artifactPreparationRejectsOtherMutations")


def read(rel):
    return open(os.path.join(root, rel), encoding="utf-8").read()


def write(rel, text):
    path = os.path.join(root, rel)
    if open(path, encoding="utf-8").read() != text:
        open(path, "w", encoding="utf-8").write(text)
        print(f"edited {rel}")


def replace(rel, old, new):
    text = read(rel)
    if new and new in text:
        return
    if old not in text:
        assert new == "" or new in text, f"{rel}: neither anchor nor result present: {old[:90]!r}"
        return
    assert text.count(old) == 1, f"{rel}: anchor x{text.count(old)}: {old[:90]!r}"
    write(rel, text.replace(old, new))


def census(source, contracts, fixture):
    rows = re.findall(r'^\s*"([^"]+)" as "[^"]+" =>', source, re.M)
    classification = dict(re.findall(r'\.action_interactive_job\("([^"]+)",\s*(?:semio_framework_plugin::)?InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)', source))
    lanes = {tool: [LANE[lane] for lane in re.findall(r"ArtifactToolPublicationLane::(\w+)", declared)] for tool, declared in re.findall(r'ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[([^\]]*)\] \}', contracts)}
    assert sorted(rows) == sorted(classification) == sorted(lanes), "command rows, manifest classifications and publication contracts must name the same routes"
    blockers = {(group["status"], tuple(group["lanes"])): group.get("blocker") for group in fixture["groups"]}
    groups = {}
    for route in rows:
        key = (STATUS[classification[route]], tuple(lanes[route]))
        groups.setdefault(key, []).append(route)
    out = []
    for (status, lane_list), routes in groups.items():
        group = {"status": status, "lanes": list(lane_list), "routes": routes}
        if status == "batch-only-pending-rewrite":
            blocker = blockers.get((status, lane_list))
            assert blocker, f"batch-only group {lane_list} needs its blocker"
            group["blocker"] = blocker
        out.append(group)
    retained = [route for group in out if group["status"] == "migrated" for route in group["routes"]]
    return len(rows), retained, out


def rewrite_fixture(rel, source, contracts, note):
    fixture = json.loads(read(rel))
    count, retained, groups = census(source, contracts, fixture)
    rebuilt = {}
    for key, value in fixture.items():
        if key == "frameworkOwnedRoutes" or (note and key == "materialization"):
            continue
        rebuilt[key] = value
    rebuilt["routeCount"], rebuilt["retainedRoutes"], rebuilt["groups"] = count, retained, groups
    if note:
        rebuilt["laws"] = {name: value for name, value in fixture["laws"].items() if name not in STALE_NOTE_LAWS}
    write(rel, json.dumps(rebuilt, ensure_ascii=False, indent=2) + "\n")


rewrite_fixture(FLOW_FIXTURE, read(FLOW_SOURCE), read(FLOW_SOURCE), False)
rewrite_fixture(NOTE_FIXTURE, read(NOTE_SOURCE), read(NOTE_RETAINED), True)

schema = json.loads(read(SCHEMA))
cohort = schema["$defs"]["ActionCohort"]
cohort["required"] = [name for name in cohort["required"] if name != "frameworkOwnedRoutes"]
cohort["properties"].pop("frameworkOwnedRoutes", None)
cohort["properties"].pop("materialization", None)
cohort["properties"]["routeCount"] = {
    "description": "The census: the number of command rows the owner's own command table declares. Every row is classified in exactly one group, so routeCount equals the number of routes across groups; framework-reserved actions are never app routes and never appear in a cohort.",
    "type": "integer",
    "minimum": 1,
}
write(SCHEMA, json.dumps(schema, ensure_ascii=False, indent=2) + "\n")

replace(FLOW_LAW, '''/// 🗂️ Serde is the independent Rust JSON oracle for the language-agnostic Flow/Note route census.
#[test]
fn action_cohort_fixtures_match_the_exact_route_census() {
    let flow: Value = serde_json::from_str(include_str!("../../../../../../../../../🧫️fixtures/🎬️action-cohort/🔣️.json")).expect("Flow action-cohort fixture must be valid JSON");
    let note: Value = serde_json::from_str(include_str!("../../../../../../../../../../🗒️note/🧫️fixtures/🧪️action-cohort/🔣️.json")).expect("Note action-cohort fixture must be valid JSON");
    for (fixture, owner, total, framework_owned) in [(&flow, "FlowPlayApp", 34_u64, 0_usize), (&note, "NotePlayApp", 35_u64, 1_usize)] {
        assert_eq!(fixture["owner"], owner);
        assert_eq!(fixture["routeCount"].as_u64(), Some(total));
        assert_eq!(fixture["frameworkOwnedRoutes"].as_array().map(Vec::len), Some(framework_owned));
        let routes: Vec<&str> = fixture["groups"].as_array().expect("groups").iter().flat_map(|group| group["routes"].as_array().expect("routes")).map(|route| route.as_str().expect("route id")).collect();
        let retained: Vec<&str> = fixture["retainedRoutes"].as_array().expect("retained routes").iter().map(|route| route.as_str().expect("retained route id")).collect();
        let migrated: Vec<&str> = fixture["groups"].as_array().expect("groups").iter().filter(|group| group["status"] == "migrated").flat_map(|group| group["routes"].as_array().expect("routes")).map(|route| route.as_str().expect("route id")).collect();
        assert_eq!(retained, migrated, "the retained route index must exactly name the migrated groups");
        let mut unique = routes.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(routes.len() + framework_owned, total as usize);
        assert_eq!(unique.len(), routes.len());
    }
}''', '''/// 🗂️ Serde is the independent Rust JSON oracle for the language-agnostic Flow/Note route census, read exactly as the
/// cohort schema (`🎬️action-cohort/🧬️schema` `routeCount`) defines it: the owner's own command rows, each classified in
/// exactly one group. Flow's census is additionally the exact row set of its typed command table.
#[test]
fn action_cohort_fixtures_match_the_exact_route_census() {
    let flow: Value = serde_json::from_str(include_str!("../../../../../../../../../🧫️fixtures/🎬️action-cohort/🔣️.json")).expect("Flow action-cohort fixture must be valid JSON");
    let note: Value = serde_json::from_str(include_str!("../../../../../../../../../../🗒️note/🧫️fixtures/🧪️action-cohort/🔣️.json")).expect("Note action-cohort fixture must be valid JSON");
    for (fixture, owner) in [(&flow, "FlowPlayApp"), (&note, "NotePlayApp")] {
        assert_eq!(fixture["owner"], owner);
        let routes: Vec<&str> = fixture["groups"].as_array().expect("groups").iter().flat_map(|group| group["routes"].as_array().expect("routes")).map(|route| route.as_str().expect("route id")).collect();
        let retained: Vec<&str> = fixture["retainedRoutes"].as_array().expect("retained routes").iter().map(|route| route.as_str().expect("retained route id")).collect();
        let migrated: Vec<&str> = fixture["groups"].as_array().expect("groups").iter().filter(|group| group["status"] == "migrated").flat_map(|group| group["routes"].as_array().expect("routes")).map(|route| route.as_str().expect("route id")).collect();
        assert_eq!(retained, migrated, "{owner}: the retained route index must exactly name the migrated groups");
        let mut unique = routes.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), routes.len(), "{owner}: every route is classified in exactly one group");
        assert_eq!(fixture["routeCount"].as_u64(), Some(routes.len() as u64), "{owner}: routeCount is exactly the classified command rows");
    }
    let mut classified: Vec<&str> = flow["groups"].as_array().expect("groups").iter().flat_map(|group| group["routes"].as_array().expect("routes")).map(|route| route.as_str().expect("route id")).collect();
    classified.sort_unstable();
    let mut declared = FlowCommand::TOOL_JOB_IDS.to_vec();
    declared.sort_unstable();
    assert_eq!(classified, declared, "the Flow census names exactly FlowCommand's rows");
}''')

replace(FLOW_AUDIT, "  frameworkOwnedRoutes: string[];\n", "")
replace(FLOW_AUDIT, "  materialization?: Record<string, boolean | number>;\n", "")
replace(FLOW_AUDIT, '''    && classified.length === fixture.routeCount - fixture.frameworkOwnedRoutes.length
    && new Set(classified).size === classified.length
    && new Set(fixture.frameworkOwnedRoutes).size === fixture.frameworkOwnedRoutes.length
''', '''    && classified.length === fixture.routeCount
    && new Set(classified).size === classified.length
''')
replace(FLOW_AUDIT, '''      && source.includes("fn build_artifact_store_one_item_preparation_factory()")
      && source.includes("fn build_config_store_one_item_preparation_factory()")
      && exact(publicationRows(retainedContracts), fixture.retainedRoutes)''', '''      && source.includes("fn build_artifact_store_one_item_preparation_factory()")
      && retainedSource.includes("bounded_config_store_one_item_preparation_factory::<NoteSnapshot, crate::op::NoteMutation>")
      && exact(publicationRows(retainedContracts), fixture.retainedRoutes)''')
replace(FLOW_AUDIT, "  return exact(commandRows(source), [...classified, ...fixture.frameworkOwnedRoutes])\n", "  return exact(commandRows(source), classified)\n")
replace(FLOW_AUDIT, '''  const schema = await Bun.file(resolve(pluginRoot, "🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs")).text();
  return fixture.globals.length === 0 && !schema.includes("static NEXT: AtomicU64");''', '''  const schema = await Bun.file(resolve(pluginRoot, "🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs")).text();
  return fixture.globals.length === 0 && !schema.includes("static NEXT") && !schema.includes("AtomicU64") && schema.includes("pub struct NoteIdOwner");''')
replace(FLOW_AUDIT, '''      { ...fixtures[0]!, routeCount: 35 },
      { ...fixtures[1]!, groups: fixtures[1]!.groups.map((group, index) => index === 3 ? { ...group, lanes: ["host-only", "artifact"] } : group) },''', '''      { ...fixtures[0]!, routeCount: fixtures[0]!.routeCount + 1 },
      { ...fixtures[1]!, groups: fixtures[1]!.groups.map((group, index) => index === 0 ? { ...group, lanes: ["host-only", "artifact"] } : group) },''')
replace(FLOW_AUDIT, "; frameworkDelegated=${scope === \"all\" ? 1 : 0}; globals=", "; globals=")

replace(NOTE_LAW, "type Fixture = { routeCount: number; retainedRoutes: string[]; frameworkOwnedRoutes: string[]; groups: Group[]; globals: unknown[]; scanThenMonolithRoutes: string[]; laws: Record<string, boolean> };",
        "type Fixture = { routeCount: number; retainedRoutes: string[]; groups: Group[]; globals: unknown[]; scanThenMonolithRoutes: string[]; laws: Record<string, boolean> };")
replace(NOTE_LAW, "  expect(commands.length).toBe(fixture.routeCount);\n", "  expect(commands.length).toBe(fixture.routeCount);\n  expect(classified.length).toBe(fixture.routeCount);\n")
replace(NOTE_LAW, "  expect(fixture.frameworkOwnedRoutes).toEqual([]);\n", "")
replace(NOTE_LAW, '''  expect(retainedSource).toContain("authority.prepare_one_item(edit");
  expect(retainedSource).toContain("NOTE_MATERIALIZATION_STRING_CHUNK_BYTES: usize = 1_024");
  expect(retainedSource).toContain("local_owner::<semio_s_artifact_stdio_semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot>()");
  expect(retainedSource).toContain("child.with_local_owner(owner)");
  expect(retainedSource).toContain("text_child_materialization_preserves_present_typed_owner");
  expect(retainedSource).toContain("text_child_materialization_preserves_absent_owner");
  expect(retainedSource).toContain("text_child_materialization_cancellation_retires_partial_metadata");
  expect(retainedSource).toContain("struct NoteSnapshotMaterializationCursor");
  expect(retainedSource).toContain("struct NoteBlockMaterializationCursor");
  expect(retainedSource).toContain("struct NoteArtifactLinkMaterializationCursor");
  expect(retainedSource).toContain(".range::<str, _>((std::ops::Bound::Excluded(last_key.as_str()), std::ops::Bound::Unbounded))");
  expect(retainedSource).toContain("snapshot_materialization_copies_every_nested_owner_and_preserves_typed_text_arc");
  expect(retainedSource).toContain("snapshot_materialization_preserves_absent_typed_owner");
  expect(retainedSource).toContain("snapshot_materialization_cancellation_during_nested_metadata_reaches_terminal_emptiness");
  expect(retainedSource).toContain("struct NoteRootScalarPreparation");
  expect(retainedSource).toContain("NoteMutation::ChangeGridVisible");
  expect(retainedSource).toContain("NoteMutation::ChangeGridSpacing");
  expect(retainedSource).toContain("Note one-item Artifact preparation admits only exact retained root-scalar mutations on the document lane");
  expect(retainedSource).toContain("if self.cursor + 1 < self.units.len()");
  expect(retainedSource).toContain("snapshot_materialization_rejects_stale_operation_authority_and_retires");
  expect(retainedSource).toContain("root_scalar_preflight_admits_only_exact_valid_document_mutations");
''', '''  expect(retainedSource).toContain("bounded_config_store_one_item_preparation_factory::<NoteSnapshot, crate::op::NoteMutation>");
  expect(retainedSource).toContain("if self.cursor + 1 < self.units.len()");
''')
replace(NOTE_LAW, "  expect(fixture.groups.every((group) => group.routes.every((route) => manifests.get(route) === group.status))).toBe(true);\n",
        "  const classification = { \"migrated\": \"Migrated\", \"batch-only-pending-rewrite\": \"BatchOnlyPendingRewrite\" } as const;\n  expect(fixture.groups.every((group) => group.routes.every((route) => manifests.get(route) === classification[group.status]))).toBe(true);\n")
replace(NOTE_LAW, "  expect(proofSource).toContain('factory: \"BoundedFirstStepCommandJobFactory\"');\n", "  expect(proofSource).toContain('factory: \"NoteCommandJobFactory\"');\n")
