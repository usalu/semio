#!/usr/bin/env python3
"""🖊️ The raw cargo invocations of script.ts that name the retired `semio-hub` lib or bin test targets."""
import re, sys, json
sys.path.insert(0, __import__('os').path.dirname(__import__('os').path.abspath(__file__)))
import tsrepoint, tsmanual
from tsmanual import sub_once, pair_for, BYNAME

TEST_PACKAGES = None


def home(name, kind):
    """(package, new law path) of the law whose old path ends with `name`."""
    return pair_for(name, kind)


def target_for(name, kind):
    pkg, _ = home(name, kind)
    return ['"-p"', '"%s"' % pkg, '"--lib"'] if pkg != 'semio-hub' else ['"--bin"', '"os-hub"']


def regex_pass(s):
    def suffix_target(m):
        kind = 'bin' if 'bin' in m.group(1) else 'lib'
        return 'target: [%s], suffix: "%s"' % (', '.join(target_for(m.group(2), kind)), m.group(2))
    s = re.sub(r'target: \[("--bin", "os-hub"|"--lib")\], suffix: "(\w+)"', suffix_target, s)

    def exact_law(m):
        kind = 'bin' if 'bin' in m.group(1) else 'lib'
        return 'exactLaw([%s], "%s")' % (', '.join(target_for(m.group(2), kind)), m.group(2))
    s = re.sub(r'exactLaw\(\[("--bin", "os-hub"|"--lib")\], "(\w+)"\)', exact_law, s)
    # catalogTarget
    pkg, _ = home('verified_trusted_catalog_document_open_generation_and_resolution_are_exact', 'lib')
    s = sub_once(s, 'const catalogTarget = ["-p", "semio-hub", "--lib"];', 'const catalogTarget = ["-p", "%s", "--lib"];' % pkg)
    # runCargo with a law variable that was an os-hub bin law
    var_pkg = {}
    for m in re.finditer(r'const (\w+Law) = exactLaw\(\["-p", "([^"]+)", "--lib"\]', s):
        var_pkg[m.group(1)] = m.group(2)

    def run_cargo(m):
        v = m.group(1)
        return '"--all-features", "-p", "%s", "--lib", %s,' % (var_pkg[v], v) if v in var_pkg else m.group(0)
    s = re.sub(r'"--all-features", "--bin", "os-hub", (\w+Law),', run_cargo, s)
    return s


def patch(s):
    s = regex_pass(s)
    s = sub_once(s, 'runRepositoryCargoTests(["semio-hub"], this.repoRoot, this.invocation, ["--lib","--features","integration-fixtures",filter', 'runRepositoryCargoTests(["semio-hub-component-codec-laws"], this.repoRoot, this.invocation, ["--lib","--features","integration-fixtures",filter')
    s = sub_once(s, 'runRepositoryCargoTests(["semio-hub"], this.repoRoot, this.invocation, allFeatures', 'runRepositoryCargoTests(HUB_TEST_PACKAGES, this.repoRoot, this.invocation, allFeatures')
    s = sub_once(s, 'const path = join(root, "🗑️generated", "test-artifacts");', 'const path = join(root, "../../🧪️tests/🗂️artifact-root/📦️packages/🦀️rust/🗑️generated/test-artifacts");')
    # live lanes
    s = sub_once(s, 'Readonly<Record<string, Readonly<{ filter: string; claimed: string; threads: number }>>> = {\n  postgres: { filter: "directory::postgres", claimed: "OS_HUB_DATABASE_URL", threads: 2 },\n  neo4j: { filter: "directory::neo4j", claimed: "OS_HUB_NEO4J_URI", threads: 1 },\n};', 'Readonly<Record<string, Readonly<{ filters: ReadonlyArray<readonly [string, string]>; claimed: string; threads: number }>>> = {\n  postgres: { filters: [["semio-hub-directory-backend-laws", "postgres_tests"], ["semio-hub-directory-laws", "postgres_creation_tests"]], claimed: "OS_HUB_DATABASE_URL", threads: 2 },\n  neo4j: { filters: [["semio-hub-directory-backend-laws", "neo4j_tests"], ["semio-hub-directory-laws", "neo4j_creation_tests"]], claimed: "OS_HUB_NEO4J_URI", threads: 1 },\n};')
    s = sub_once(s, '      const { filter, threads } = DIRECTORY_LIVE_LANES[lane]!;\n      runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", filter, "--", "--include-ignored", `--test-threads=${threads}`], this.root);', '      const { filters, threads } = DIRECTORY_LIVE_LANES[lane]!;\n      for (const [owner, filter] of filters) runCargo(["test", "--manifest-path", "Cargo.toml", "-p", owner, "--lib", filter, "--", "--include-ignored", `--test-threads=${threads}`], this.root);')
    # hostile input
    hp = {
        'draws': ('hostile_generative::long::hostile_draws_reproduce_the_language_neutral_vectors', 'hostile_draws_reproduce_the_language_neutral_vectors'),
        'coverage': ('the_hostile_input_fixture_covers_every_registered_route',) * 2,
        'refusals': ('every_route_answers_hostile_input_with_a_typed_signed_refusal',) * 2,
        'generative': ('hostile_generative::long::every_route_and_socket_survives_generated_hostile_input', 'every_route_and_socket_survives_generated_hostile_input'),
    }
    new = 'const HOSTILE_INPUT_LAWS = {\n' + ''.join('  %s: ["%s", "%s"],\n' % (k, pair_for(v[1], 'bin')[0], v[0]) for k, v in hp.items()) + '} as const;'
    s = re.sub(r'const HOSTILE_INPUT_LAWS = \{.*?\} as const;', lambda m: new, s, count=1, flags=re.S)
    s = sub_once(s, 'const law = (name: string, seed?: number): Promise<{ status: number; lines: string[] }> =>\n        runLawProcess("cargo", ["test", "--manifest-path", "Cargo.toml", "--bin", "os-hub", "--no-fail-fast", "--", "--exact", `tests::${name}`,', 'const law = ([owner, name]: readonly [string, string], seed?: number): Promise<{ status: number; lines: string[] }> =>\n        runLawProcess("cargo", ["test", "--manifest-path", "Cargo.toml", "-p", owner, "--lib", "--no-fail-fast", "--", "--exact", `tests::${name}`,')
    # cas
    s = sub_once(s, 'runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--lib", "artifact_chunk_cas", ...segments]', 'runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "semio-hub-artifact-authority", "-p", "semio-hub-directory-laws", "--lib", "artifact_chunk_cas", ...segments]')
    s = sub_once(s, 'runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub", "artifact_cas_maintenance", ...segments]', 'runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "%s", "--lib", "artifact_cas_maintenance", ...segments]' % pair_for('artifact_cas_maintenance_checkpoint_reaches_tail_after_sixteen_requests', 'bin')[0])
    # canonical pair
    s = sub_once(s, 'runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "canonical_pair", "--", "--test-threads=1"], this.root, env);', 'runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-hub-lag-rebootstrap", "--lib", "canonical_pair", "--", "--test-threads=1"], this.root, env);')
    s = sub_once(s, 'runCargo(["test", "--manifest-path", "Cargo.toml", "--bin", "os-hub", "canonical_pair_route", "--", "--test-threads=1"], this.root, env);', 'runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "%s", "--lib", "canonical_pair_route", "--", "--test-threads=1"], this.root, env);' % pair_for('canonical_pair_route_rejects_non_path_and_ambiguous_headers_before_work', 'bin')[0])
    # space public boundary
    pkg = pair_for('space_public_boundary_public_event_route_denies_raw_directory_events', 'bin')[0]
    s = sub_once(s, 'const target = ["test", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"];', 'const target = ["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "%s", "--lib"];' % pkg)
    # trusted publication native
    s = sub_once(s, 'runRepositoryCargoTests(["semio-hub"], this.repoRoot, this.invocation, ["--no-fail-fast", "--lib", "trusted_publication"', 'runRepositoryCargoTests(["semio-hub-trusted-catalog-contract-laws", "semio-hub-trusted-catalog-laws"], this.repoRoot, this.invocation, ["--no-fail-fast", "--lib", "trusted_publication"')
    return s


def live_journey(s):
    first = 'local_bootstrap::tests::local_bootstrap_idle_listener_survives_until_admission_and_admitted_frame_is_deadline_bounded'
    s = tsmanual.convert_enclosing(s, '"%s"' % first, 'lib')
    s = sub_once(s, '    for (const law of laws) {\n      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--lib", law, "--", "--list"]', '    for (const [owner, law] of laws) {\n      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "-p", owner, "--lib", law, "--", "--list"]')
    s = sub_once(s, 'runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", law, "--", "--exact", "--test-threads=1"], this.root);', 'runCargo(["test", "--manifest-path", "Cargo.toml", "-p", owner, "--lib", law, "--", "--exact", "--test-threads=1"], this.root);')
    return s


def add_constants(s):
    idents = [l.strip() for l in open('/tmp/hubw/list.txt') if l.strip()]
    names = sorted({i.replace('_', '-') for i in idents} | {'semio-hub', 'semio-hub-component-codec-laws'})
    const = '/** 🧪️ Every hub package that owns tests: the `os-hub` verbs that run the whole suite select these. */\nconst HUB_TEST_PACKAGES: readonly string[] = [\n' + ''.join('  "%s",\n' % n for n in names) + '];\n\n'
    anchor = '/** 🗺️ One exact-law group per owning crate'
    assert s.count(anchor) == 1
    return s.replace(anchor, const + anchor, 1)


if __name__ == '__main__':
    s = open(sys.argv[1]).read()
    s = patch(s)
    s = live_journey(s)
    s = add_constants(s)
    open(sys.argv[2], 'w').write(s)
