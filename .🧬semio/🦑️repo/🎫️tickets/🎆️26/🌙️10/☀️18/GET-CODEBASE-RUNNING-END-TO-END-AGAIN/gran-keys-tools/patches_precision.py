# 🪝 Anchored edits of the input precision engine, its test and the assembly hook: a Cargo read carries the resolution identity marker of the crates it names (locked slice, workspace inheritance, toolchain) instead of the raw root Cargo files, so a root manifest edit no longer re-keys the root laws that run Cargo.
import json
from patchlib import chain

def engine(text):
    return chain(text, [
        ("host-type", "  readonly cargoToolchain: readonly string[];\n};", "  readonly cargoToolchain: readonly string[];\n  readonly cargoIdentity?: (crateDirectory: string) => unknown;\n};"),
        ("result-type", "readonly unresolved: readonly string[]; readonly reads:", "readonly unresolved: readonly string[]; readonly markers?: readonly unknown[]; readonly reads:"),
        ("unique-json", "const bytes = (left: string, right: string): number =>", "const uniqueJson = (values: readonly unknown[]): unknown[] => [...new Map(values.map((value) => [JSON.stringify(value), value])).values()];\nconst bytes = (left: string, right: string): number =>"),
        ("directory-sig", 'host: Pick<InputPrecisionHostV1, "cargoRoots" | "cargoSources" | "cargoToolchain">): {patterns: string[]; files: string[]} {\n  const patterns: string[] = [], files: string[] = [];\n  for (const directory of directories) {\n', 'host: Pick<InputPrecisionHostV1, "cargoRoots" | "cargoSources" | "cargoToolchain" | "cargoIdentity">): {patterns: string[]; files: string[]; markers: unknown[]} {\n  const patterns: string[] = [], files: string[] = [], markers: unknown[] = [];\n  for (const directory of directories) {\n    const marker = host.cargoIdentity?.(directory);\n    if (marker !== undefined) markers.push(marker);\n'),
        ("directory-return", "files: unique(files)};\n}", "files: unique(files), markers: uniqueJson(markers)};\n}"),
        ("packages-sig", 'host: Pick<InputPrecisionHostV1, "readText" | "cargoManifests" | "cargoRoots" | "cargoSources" | "cargoToolchain">): {patterns: string[]; files: string[]} {', 'host: Pick<InputPrecisionHostV1, "readText" | "cargoManifests" | "cargoRoots" | "cargoSources" | "cargoToolchain" | "cargoIdentity">): {patterns: string[]; files: string[]; markers: unknown[]} {'),
        ("derive-state", "const patterns: string[] = [], groups: string[] = [], files: string[] = [], unresolved: string[] = [],", "const patterns: string[] = [], groups: string[] = [], files: string[] = [], markers: unknown[] = [], unresolved: string[] = [],"),
        ("cargo-read", "for (const crate of [packages.length ? cratePatternsV1(packages, host) : {patterns: [], files: []}, directories.length ? crateDirectoryPatternsV1(directories, host) : {patterns: [], files: []}]) {\n        patterns.push(...crate.patterns);\n        files.push(...crate.files);", "for (const crate of [packages.length ? cratePatternsV1(packages, host) : {patterns: [], files: [], markers: []}, directories.length ? crateDirectoryPatternsV1(directories, host) : {patterns: [], files: [], markers: []}]) {\n        patterns.push(...crate.patterns);\n        files.push(...crate.files);\n        markers.push(...crate.markers);"),
        ("script-read-empty", "const crates = facts.crates.length ? cratePatternsV1(facts.crates, host) : {patterns: [], files: []}, crateDirs = resolved.resolved.filter((path) => host.exists(`${path}/Cargo.toml`)), directories = crateDirs.length ? crateDirectoryPatternsV1(crateDirs, host) : {patterns: [], files: []};\n      patterns.push(...crates.patterns, ...directories.patterns);", "const crates = facts.crates.length ? cratePatternsV1(facts.crates, host) : {patterns: [], files: [], markers: []}, crateDirs = resolved.resolved.filter((path) => host.exists(`${path}/Cargo.toml`)), directories = crateDirs.length ? crateDirectoryPatternsV1(crateDirs, host) : {patterns: [], files: [], markers: []};\n      patterns.push(...crates.patterns, ...directories.patterns);\n      markers.push(...crates.markers, ...directories.markers);"),
        ("dispatch-read", "named = crates.length ? cratePatternsV1(crates, host) : {patterns: [], files: []}, directories = crateDirectories.length ? crateDirectoryPatternsV1(crateDirectories, host) : {patterns: [], files: []};\n      patterns.push(...named.patterns, ...directories.patterns);", "named = crates.length ? cratePatternsV1(crates, host) : {patterns: [], files: [], markers: []}, directories = crateDirectories.length ? crateDirectoryPatternsV1(crateDirectories, host) : {patterns: [], files: [], markers: []};\n      patterns.push(...named.patterns, ...directories.patterns);\n      markers.push(...named.markers, ...directories.markers);"),
        ("owner-read", "const crate = crateDirectoryPatternsV1([route.root], host);\n        patterns.push(...crate.patterns);\n        files.push(...crate.files);", "const crate = crateDirectoryPatternsV1([route.root], host);\n        patterns.push(...crate.patterns);\n        files.push(...crate.files);\n        markers.push(...crate.markers);"),
        ("derive-return", "unresolved: unique(unresolved), reads};", "unresolved: unique(unresolved), ...(markers.length ? {markers: uniqueJson(markers)} : {}), reads};"),
    ])

TEST_TAIL = '''

test("a host that carries the resolution identity adds one marker per named crate and keeps every other derived input", async () => {
  const host = {...fixtureHost(), cargoToolchain: [] as string[], cargoIdentity: (directory: string) => ({env: `SEMIO_CARGO_LOCK_${directory.replace(/\\W/gu, "_")}`})};
  let withMarkers = 0;
  for (const [index, row] of fixture.derive.entries()) {
    const rule = selectRuleV1(document, row.identity as any)!;
    if (rule.evidence === "assumed") continue;
    const derived = await deriveInputsV1(document, rule, row.identity as any, host), expected = frozen.derive[index];
    expect(derived.patterns, JSON.stringify(row.identity)).toEqual(expected.patterns.filter((pattern: string) => pattern !== "{workspaceRoot}/rust-toolchain.toml"));
    expect(derived.files).toEqual(expected.files);
    if (derived.markers?.length) {
      withMarkers++;
      expect(new Set(derived.markers.map((marker) => JSON.stringify(marker))).size).toBe(derived.markers.length);
    }
  }
  expect(withMarkers).toBeGreaterThan(0);
});
'''
def test(text):
    if TEST_TAIL.strip()[:80] in text: return text, False
    return text.rstrip("\n") + "\n" + TEST_TAIL, True

def assembly_markers(text):
    return chain(text, [
        ("assembly-inputs", "...derived.patterns, ...derived.groups, ...intern(derived.files.map((file) => `{workspaceRoot}/${file}`))]", "...derived.patterns, ...derived.groups, ...(derived.markers ?? []), ...intern(derived.files.map((file) => `{workspaceRoot}/${file}`))]"),
        ("assembly-identity", "    cargoToolchain: POLICY.toolchains.cargo.files,\n", "    cargoToolchain: POLICY.toolchains.cargo.files,\n    cargoIdentity: (directory) => (readToml(join(workspaceRoot, directory, \"Cargo.toml\")).package ? loaded.cargoLockInput(directory ? `${directory}/Cargo.toml` : \"Cargo.toml\", workspaceRoot) : undefined),\n"),
    ])
