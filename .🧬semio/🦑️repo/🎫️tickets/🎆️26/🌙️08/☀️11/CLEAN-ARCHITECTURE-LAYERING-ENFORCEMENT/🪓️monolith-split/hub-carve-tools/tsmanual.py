#!/usr/bin/env python3
"""🖊️ The call sites of script.ts whose law lists are variables: arrays of `[owning package, law]` pairs grouped by `hubLawGroups`."""
import re, sys, json
sys.path.insert(0, __import__('os').path.dirname(__import__('os').path.abspath(__file__)))
import tsrepoint

tsrepoint.MAPPING = '/tmp/hubw/law-mapping.final.json'
BYOLD, BYNAME = tsrepoint.load()

GROUPS_HELPER = '''
/** 🗺️ One exact-law group per owning crate for laws written as `[package, law]` pairs, in order of first appearance; `--features integration-fixtures` reaches only the crates that declare it. */
function hubLawGroups(laws: ReadonlyArray<readonly [string, string]>, cargoArgs: readonly string[] = []): ExactCargoLawGroup[] {
  const owners = new Map<string, string[]>();
  for (const [owner, law] of laws) owners.set(owner, [...(owners.get(owner) ?? []), law]);
  const fixturePackages: ReadonlySet<string> = new Set(%FIXTURE_PACKAGES%);
  return [...owners].map(([owner, ls]) => {
    const feature = cargoArgs.indexOf("--features");
    const args = feature >= 0 && cargoArgs[feature + 1] === "integration-fixtures" && !fixturePackages.has(owner) ? cargoArgs.filter((_, index) => index !== feature && index !== feature + 1) : cargoArgs;
    return { package: owner, target: owner === "semio-hub" ? { kind: "bin" as const, name: "os-hub" } : { kind: "lib" as const }, ...(args.length ? { cargoArgs: args } : {}), laws: ls };
  });
}
'''


def pair_for(lit, kind):
    loc = tsrepoint.locate(lit, kind, BYOLD, BYNAME)
    if loc is None:
        raise KeyError(lit)
    nc, np_ = loc
    return tsrepoint.pkg_of(nc), (lit if '::' not in lit else np_)


def pairs_array(text, kind):
    lits = tsrepoint.literal_strings(text)
    assert lits is not None, text[:60]
    return '[' + ', '.join('["%s", "%s"]' % pair_for(l, kind) for l in lits) + ']'


def json_pair(lit, kind):
    return '["%s", "%s"]' % pair_for(lit, kind)


def sub_once(s, old, new):
    assert s.count(old) == 1, (s.count(old), old[:80])
    return s.replace(old, new, 1)


def array_after(s, anchor):
    """The literal `[...]` that starts at the first `[` after `anchor` (bracket-matched)."""
    i = s.index(anchor)
    j = s.index('[', i + len(anchor) - 1)
    k = tsrepoint.match_brace(s, j, '[', ']')
    return i, j, k


def convert_array(s, anchor, kind):
    i, j, k = array_after(s, anchor)
    return s[:j] + pairs_array(s[j:k + 1], kind) + s[k + 1:]


def convert_push(s, anchor, kind):
    """`name.push("a", "b")` calls: every literal argument becomes a pair."""
    i = s.index(anchor)
    j = s.index('(', i)
    k = tsrepoint.match_brace(s, j, '(', ')')
    args = re.findall(r'"((?:[^"\\]|\\.)*)"', s[j:k + 1])
    return s[:j + 1] + ', '.join('["%s", "%s"]' % pair_for(a, kind) for a in args) + s[k:]


def patch(s):
    s = sub_once(s, '        { package: "semio-hub", target: { kind: "lib", name: "semio_hub" }, laws: suffixes },\n', '        ...hubLawGroups(suffixes),\n')
    s = convert_enclosing(s, '"gis_inference_sqlite_ledger_executes_neutral_traces_with_private_first_terminal_wins"', 'lib')
    s = convert_array(s, 'const libraryLaws = [', 'lib')
    s = convert_array(s, 'const routeLaws = [', 'bin')
    s = sub_once(s, 'if (mode === "--process") routeLaws.push("check_in_process_fixture_emits_verified_gis_ledger_and_catalog");', 'if (mode === "--process") routeLaws.push(%s);' % json_pair('check_in_process_fixture_emits_verified_gis_ledger_and_catalog', 'bin'))
    s = sub_once(s, '          { package: "semio-hub", target: { kind: "lib", name: "semio_hub" }, laws: libraryLaws },\n          { package: "semio-hub", target: { kind: "bin", name: "os-hub" }, cargoArgs: ["--features", "integration-fixtures"], laws: routeLaws },\n', '          ...hubLawGroups(libraryLaws),\n          ...hubLawGroups(routeLaws, ["--features", "integration-fixtures"]),\n')
    # directory command receipt
    s = convert_enclosing(s, '"directory_command_authority_revalidates_after_durable_revocation_before_fence"', 'bin')
    s = sub_once(s, '        { package: "semio-hub", target: { kind: "bin" as const, name: "os-hub" }, laws: phase === "invites" ? laws.slice(4, 8) : laws.slice(0, 8) },\n', '        ...hubLawGroups(phase === "invites" ? laws.slice(4, 8) : laws.slice(0, 8)),\n')
    s = sub_once(s, '[{ package: "semio-hub", target: { kind: "bin" as const, name: "os-hub" }, cargoArgs: ["--all-features"], laws: laws.slice(8) }]', 'hubLawGroups(laws.slice(8), ["--all-features"])')
    # space artifact creation
    s = convert_enclosing(s, '"artifact_authority::creation::tests::creation_contract_matches_neutral_intents_and_ready_only_coordinates"', 'lib')
    s = convert_push(s, 'sqliteLaws.push("directory::tests::memory_projection_is_atomic', 'lib')
    s = convert_push(s, 'sqliteLaws.push("directory::tests::artifact_chunk_cas_sqlite_and_filesystem_restart', 'lib')
    s = convert_enclosing(s, '"directory::postgres::creation_tests::genesis_commit_ack_and_locked_expiry_are_atomic_postgres"', 'lib')
    s = convert_enclosing(s, '"directory::neo4j::creation_tests::genesis_commit_ack_and_locked_expiry_are_atomic_neo4j"', 'lib')
    s = sub_once(s, '        { package: "semio-hub", target: { kind: "lib" }, laws: backend.laws },\n', '        ...hubLawGroups(backend.laws),\n')
    # check in
    s = convert_enclosing(s, '"check_in_advances_the_active_checkpoint_to_the_named_head_and_cold_opens_from_it"', 'bin')
    s = sub_once(s, 'if (phase === "process") laws.push("check_in_process_fixture_emits_verified_gis_ledger_and_catalog");', 'if (phase === "process") laws.push(%s);' % json_pair('check_in_process_fixture_emits_verified_gis_ledger_and_catalog', 'bin'))
    s = sub_once(s, '          {\n            package: "semio-hub",\n            target: { kind: "bin", name: "os-hub" },\n            cargoArgs: ["--features", "integration-fixtures"],\n            laws,\n          },\n', '          ...hubLawGroups(laws, ["--features", "integration-fixtures"]),\n')
    # directory event page
    s = convert_enclosing(s, '"directory_event_page_v1_route_scans_raw_holes_bounds_canonical_receipt_and_visibility"', 'bin')
    s = sub_once(s, '          { package: "semio-hub", target: { kind: "bin", name: "os-hub" }, cargoArgs: ["--all-features"], laws },\n', '          ...hubLawGroups(laws, ["--all-features"]),\n')
    # admin presence target recovery
    law = 'admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen'
    s = sub_once(s, 'groups: [{ package: "semio-hub", target: { kind: "bin", name: "os-hub" }, laws: [law] }],progress', 'groups: hubLawGroups([%s]),progress' % json_pair(law, 'bin').replace('"%s"' % law, 'law'))
    # presence lease
    s = convert_array(s, 'const hubLaws = [', 'bin')
    s = sub_once(s, 'const hubGroup = { package: "semio-hub", target: { kind: "bin" as const, name: "os-hub" }, cargoArgs: ["--all-features"], laws: hubLaws };', 'const hubGroups = hubLawGroups(hubLaws, ["--all-features"]);')
    s = sub_once(s, 'laws: ["presence_roster_fixed_maximum_plus_one_returns_the_exact_rejected_owner"] }, hubGroup] : [hubGroup];', 'laws: ["presence_roster_fixed_maximum_plus_one_returns_the_exact_rejected_owner"] }, ...hubGroups] : hubGroups;')
    return s


def enclosing_array(s, pos):
    d = 0
    i = pos
    while i > 0:
        i -= 1
        if s[i] == ']':
            d += 1
        elif s[i] == '[':
            if d == 0:
                return i
            d -= 1
    raise ValueError


def convert_enclosing(s, unique_text, kind):
    assert s.count(unique_text) == 1, unique_text
    pos = s.index(unique_text)
    j = enclosing_array(s, pos)
    k = tsrepoint.match_brace(s, j, '[', ']')
    return s[:j] + pairs_array(s[j:k + 1], kind) + s[k + 1:]


def add_helper(s):
    helper = GROUPS_HELPER.replace('%FIXTURE_PACKAGES%', json.dumps(json.load(open('/tmp/hubw/fixture-packages.json')), ensure_ascii=False))
    anchor = '/** 🧪️ The shared fixtures and the unit laws'
    assert s.count(anchor) == 1
    return s.replace(anchor, helper.lstrip('\n') + '\n' + anchor, 1)


if __name__ == '__main__':
    s = open(sys.argv[1]).read()
    s = patch(s)
    s = add_helper(s)
    open(sys.argv[2], 'w').write(s)
