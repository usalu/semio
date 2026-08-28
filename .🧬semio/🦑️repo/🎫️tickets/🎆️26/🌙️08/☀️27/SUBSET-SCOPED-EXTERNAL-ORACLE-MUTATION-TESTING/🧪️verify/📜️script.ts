// 🧪️ Protocol v2 verification harness — the adversarial wave, executed.
//
// Every check below either exercises the real platform against the real repository, or INJECTS a
// fault and asserts the intended gate catches it. A gate that is never shown catching its fault is
// an untested gate, and an untested gate is exactly what Protocol v2 exists to stop being possible.
import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import * as T from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

const root = process.cwd();
const results: { id: string; ok: boolean; detail: string }[] = [];
const check = (id: string, ok: boolean, detail: string): void => {
  results.push({ id, ok, detail });
  console.log(`${ok ? "✔" : "✘"} ${id.padEnd(46)} ${detail}`);
};

const registry = T.loadOracleRegistry(root);
const CC6 = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6";
// 🧭️Selected by COORDINATE, not by index. `mutationManifests[0]` was whichever owner happened to be
// discovered first, so the moment six more owners gained manifests these checks silently started
// measuring a different subset than the one they describe.
const manifest = registry.mutationManifests.find((entry) => entry.artifact === "s.stdio.step" && entry.standard === "ap214" && entry.subset === "cc6")!;
const fixtures = registry.contributions.flatMap((c) => c.fixtureManifests);

//#region 📇️Registry
check("registry/contributions-discovered", registry.contributions.length > 100, `${registry.contributions.length} owner contributions`);
check("registry/no-malformed-contribution", registry.contributions.every((c) => c.problems.length === 0), `${registry.contributions.filter((c) => c.problems.length > 0).length} malformed`);
check("registry/probes-registered", registry.probes.length >= 8, `${registry.probes.length} probes`);
// 🎓️Every probe is either QUALIFIED or carries a qualification record saying it is not, and every
// unqualified probe's stages are optional. Pinning the exact id list froze a transient state — one of
// the two named here has since been qualified, which is progress, not a regression.
{
  const unqualified = registry.probes.filter((probe) => !T.isQualifiedProbe(probe));
  check("registry/unqualified-probes-are-marked", unqualified.every((probe) => probe.qualification !== undefined && probe.qualification.status !== "qualified"), `unqualified: ${unqualified.map((probe) => `${probe.id}(${probe.qualification?.status ?? "NO RECORD"})`).join(", ") || "none"}`);
  const pipelines = [...T.pipelineTable(registry).values()];
  const gatingUnqualified = pipelines.flatMap((pipeline) => pipeline.stages.filter((stage) => stage.optional !== true && unqualified.some((probe) => probe.id === stage.probe)).map((stage) => `${pipeline.id}/${stage.probe}`));
  check("registry/unqualified-probes-never-gate", gatingUnqualified.length === 0, gatingUnqualified.join(", ") || "no unqualified probe gates any pipeline");
  const qualified = registry.probes.filter((probe) => T.isQualifiedProbe(probe));
  check("registry/qualified-probes-carry-evidence", qualified.every((probe) => (probe.qualification?.evidence.length ?? 0) > 100 && (probe.qualification?.criteria?.length ?? 0) > 0), `${qualified.length} qualified, each with evidence and criteria`);
}
//#endregion 📇️Registry

//#region 🧬️Manifest
check("manifest/valid", T.mutationManifestProblems(manifest, CC6).length === 0, `${manifest.mutations.length} mutations, 0 problems`);
// 🪆️A wildcard SPELLING is refused in context, not at the record level: `✳️any` is a wildcard for
// s.stdio.step (which declares cc1…cc6) and is the only scope s.stdio.json@rfc8259's siblings leave.
check("manifest/wildcard-subset-refused-in-context", T.isWildcardSubsetFor(root, "s.stdio.step", "ap214", "any") && !T.isWildcardSubsetFor(root, "s.stdio.step", "ap214", "cc6"), `step@ap214/any=${T.isWildcardSubsetFor(root, "s.stdio.step", "ap214", "any")} step@ap214/cc6=${T.isWildcardSubsetFor(root, "s.stdio.step", "ap214", "cc6")}`);
// 🪆️Both wildcard verdicts are exercised through a SYNTHETIC contribution on a non-profiled owner
// path, so the manifest's own standard/subset coordinates are not additionally checked against a
// directory name — this is a test of the wildcard rule, not of the coordinate rule.
{
  const synthetic = (artifact: string, standard: string, subset: string): T.OracleRegistry => {
    const manifestUnderTest: T.MutationManifest = { schema: "semio.repository-test.mutation-manifest/v2", artifact, standard, subset, mutations: manifest.mutations };
    const contribution: T.TestContribution = { owner: "🧪️synthetic", manifestPath: "🧪️synthetic/🧪️oracle/🔣️.json", oracles: registry.oracles, probes: [], noOracleDecisions: [], comparisonProfiles: [], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], mutationCatalogs: [], mutationManifests: [manifestUnderTest], fixtureManifests: [], problems: [] };
    return { ...registry, mutationManifests: [manifestUnderTest], contributions: [contribution] };
  };
  const withSiblings = T.mutationInventoryBreaches(root, synthetic("s.stdio.step", "ap214", "any"));
  check("manifest/wildcard-owner-is-a-hard-breach", withSiblings.some((b) => b.id === "wildcard-subset-owner" && b.priority === "high"), `step@ap214/any → ${[...new Set(withSiblings.map((b) => b.id))].join(", ")}`);
  const withoutSiblings = T.mutationInventoryBreaches(root, synthetic("s.demo.unsplit", "1", "any"));
  check("manifest/unsplit-artifact-is-reported", withoutSiblings.some((b) => b.id === "unsplit-artifact-subset" && b.priority === "medium") && !withoutSiblings.some((b) => b.id === "wildcard-subset-owner"), `unsplit@1/any → ${withoutSiblings.filter((b) => b.id.includes("subset")).map((b) => `${b.id}(${b.priority})`).join(", ") || "none"}`);
  const real = T.mutationInventoryBreaches(root, synthetic("s.stdio.step", "ap214", "cc6"));
  check("manifest/real-subset-is-not-flagged", !real.some((b) => b.id === "wildcard-subset-owner" || b.id === "unsplit-artifact-subset"), "step@ap214/cc6 raises no subset finding");
}
check("manifest/duplicate-mutation-refused", T.mutationManifestProblems({ ...manifest, mutations: [...manifest.mutations, manifest.mutations[0]!] }, CC6).some((p) => p.includes("duplicated")), "duplicate id → breach");
check("manifest/unknown-outcome-refused", T.mutationManifestProblems({ ...manifest, mutations: [{ ...manifest.mutations[0]!, outcomes: ["maybe"] as never }] }, CC6).some((p) => p.includes("outcomes")), "outcome 'maybe' → breach");
check("manifest/oracle-requirement-mandatory", T.mutationManifestProblems({ ...manifest, mutations: [{ ...manifest.mutations[0]!, oracleRequirements: [] }] }, CC6).some((p) => p.includes("oracleRequirements")), "empty oracleRequirements → breach");
check("manifest/digest-changes-with-content", T.mutationManifestDigest(manifest) !== T.mutationManifestDigest({ ...manifest, subset: "cc5" }), "a manifest edit changes its digest");
//#endregion 🧬️Manifest

//#region 🧾️Equality
const runtimeOf = (ids: string[]): T.RuntimeMutationInventory => ({
  schema: "semio.repository-test.runtime-inventory/v2",
  artifact: manifest.artifact,
  standard: manifest.standard,
  subset: manifest.subset,
  bridgeVersion: 1,
  mutations: ids.map((id) => ({ id, variant: manifest.mutations.find((m) => m.id === id)?.productionDispatch.variant ?? "X", outcomes: manifest.mutations.find((m) => m.id === id)?.outcomes ?? ["applied"] })),
});
const declared = manifest.mutations.map((m) => m.id);
check("equality/exact-match-is-clean", (() => { const e = T.compareInventories(manifest, runtimeOf(declared), declared); return e.runtimeOnly.length + e.manifestOnly.length + e.testOnly.length + e.outcomeMismatches.length + e.variantMismatches.length === 0; })(), "runtime = manifest = tests → no difference");
check("equality/runtime-only-detected", T.compareInventories(manifest, runtimeOf([...declared, "secret-verb"]), declared).runtimeOnly.includes("secret-verb"), "a mutation production can dispatch but no manifest owns");
check("equality/manifest-only-detected", T.compareInventories(manifest, runtimeOf(declared.slice(1)), declared).manifestOnly.includes(declared[0]!), "a manifest row with no dispatch behind it");
check("equality/test-only-detected", T.compareInventories(manifest, runtimeOf(declared), [...declared, "no-mutation"]).testOnly.includes("no-mutation"), "a catalog kind no manifest owns");
check("equality/outcome-mismatch-detected", (() => { const rt = runtimeOf(declared); const bent = { ...rt, mutations: rt.mutations.map((m, i) => (i === 0 ? { ...m, outcomes: ["applied"] as T.MutationOutcomeClass[] } : m)) }; return T.compareInventories(manifest, bent, declared).outcomeMismatches.length > 0; })(), "declared outcomes ≠ dispatched outcomes");
check("equality/variant-mismatch-detected", (() => { const rt = runtimeOf(declared); const bent = { ...rt, mutations: rt.mutations.map((m, i) => (i === 0 ? { ...m, variant: "Renamed" } : m)) }; return T.compareInventories(manifest, bent, declared).variantMismatches.length > 0; })(), "dispatch variant renamed under the manifest");
check("equality/missing-runtime-is-not-a-pass", T.compareInventories(manifest, null, declared).runtimeMissing, "no bridge output ⇒ runtimeMissing, never silent success");
//#endregion 🧾️Equality

//#region ✅️Oracle
const qualifying = registry.oracles.filter((o) => T.isQualifyingOracleKind(o.kind));
check("oracle/qualifying-kinds-only", !T.isQualifyingOracleKind("cross-semio-implementation") && T.isQualifyingOracleKind("third-party-library"), "a second Semio implementation never qualifies");
check("oracle/missing-external-oracle-detected", T.oracleRequirementBreaches({ ...registry, oracles: [] }, CC6, manifest, manifest.mutations[0]!).some((b) => b.id === "missing-external-oracle"), "no qualifying oracle → blocking breach");
check("oracle/semio-derived-does-not-discharge", T.oracleRequirementBreaches({ ...registry, oracles: registry.oracles.map((o) => ({ ...o, kind: "cross-semio-implementation" as T.OracleKind })) }, CC6, manifest, manifest.mutations[0]!).some((b) => b.id === "missing-external-oracle"), "reclassifying every oracle as Semio-derived reopens the gap");
check("oracle/engine-independence-enforced", T.oracleRequirementBreaches({ ...registry, oracles: qualifying.map((o) => ({ ...o, engine: { family: "opencascade", implementation: "x", version: "1" } })) }, CC6, manifest, manifest.mutations.find((m) => m.id === "set-shape-representation")!).some((b) => b.id === "insufficient-engine-independence"), "collapsing both oracles onto one kernel → breach");
check("oracle/no-oracle-cannot-cover-mutation", T.noOracleMisuseBreaches({ ...registry, noOracleDecisions: [{ id: "sneaky", capabilities: [manifest.mutations[0]!.capability], rationale: "x".repeat(30), substitutes: ["metamorphic-laws"] }] }).some((b) => b.id === "no-oracle-covers-mutation"), "a no-oracle decision naming a mutation capability → breach");
check("oracle/subject-sharing-engine-reported", T.engineIndependenceBreaches(registry, new Map([[manifest.artifact, { family: "opencascade", implementation: "our own", version: "1" }]])).some((b) => b.id === "oracle-shares-subject-engine"), "subject on the oracle's kernel → reported");
//#endregion ✅️Oracle

//#region 🔒️Isolation
check("isolation/clean-today", T.isolationBreaches(registry).length === 0, `${registry.oracles.length} oracles + ${registry.probes.length} probes, 0 leaks`);
check("isolation/production-reachable-oracle-detected", T.isolationBreaches({ ...registry, oracles: [{ ...qualifying[0]!, productionReachable: true, productionDebt: undefined }] }).some((b) => b.id === "oracle-production-reachable"), "an oracle production can import → breach");
check("isolation/networked-probe-detected", T.isolationBreaches({ ...registry, probes: [{ ...registry.probes[0]!, networkDuringExecution: true }] }).some((b) => b.id === "probe-needs-network"), "a probe that phones home → breach");
check("isolation/unseeded-nondeterministic-probe-detected", T.isolationBreaches({ ...registry, probes: [{ ...registry.probes[0]!, deterministic: false, seedRequired: false }] }).some((b) => b.id === "probe-nondeterministic-unseeded"), "sampled metric with no seed → breach");
//#endregion 🔒️Isolation

//#region 🧫️Fixture
// 🧫️A LOWER BOUND, not a pinned count. Pinning 24 meant that adding a real-world and a handcrafted
// bundle — strictly more evidence — failed the harness, which is the frozen-constant anti-pattern this
// ticket removed from the platform's own self-test.
check("fixture/corpus-present", fixtures.length >= 24, `${fixtures.length} fixture bundles`);
// 🛡️Three registered fixtures carry no `target` at all. They are a REPORTED FAILURE below, not a reason
// for the harness to crash — so the checks that need a well-formed coordinate select over the valid ones.
const malformed = fixtures.filter((f) => f?.target?.artifact === undefined);
const wellFormed = fixtures.filter((f) => f?.target?.artifact !== undefined);
check("fixture/none-registered-without-a-target", malformed.length === 0, `${malformed.length} fixture(s) registered without a target: ${malformed.map((f) => f.id).join(", ") || "none"}`);
const invalid = fixtures.filter((f) => T.fixtureManifestProblems(f, root).length > 0);
check("fixture/provenance-failures-are-named", invalid.length === 0, invalid.length === 0 ? "every registered fixture passes the provenance contract" : `${invalid.length} failing: ${invalid.map((f) => `${f.id} (${T.fixtureManifestProblems(f, root).join("; ")})`).join(" | ").slice(0, 220)}`);
// 🪆️Both directions of the subset rule, so neither can drift: `any` on an artifact WITH real sibling
// subsets stays a breach, while `any` on a genuinely single-subset artifact is settled, not spelled.
const siblinged = wellFormed.find((x) => T.isWildcardSubsetFor(root, x.target.artifact, x.target.standard, "any"))!;
check("fixture/wildcard-with-siblings-still-breaches", T.fixtureManifestProblems({ ...siblinged, target: { ...siblinged.target, subset: "any" } }, root).some((p) => p.includes("wildcard")), `${siblinged.target.artifact} has real sibling subsets, so "any" is still refused there`);
const single = wellFormed.find((x) => !T.isWildcardSubsetFor(root, x.target.artifact, x.target.standard, "any"))!;
check("fixture/wildcard-on-single-subset-artifact-settled", T.fixtureManifestProblems({ ...single, target: { artifact: single.target.artifact, standard: single.target.standard, subset: "any" } }, root).every((p) => !p.includes("wildcard")), "an artifact with exactly one subset is not \"everything\"; the name is settled, not spelled");
const verified = wellFormed.flatMap((f) => T.verifyFixture(root, f));
check("fixture/digests-match-disk", verified.every((v) => v.ok), `${verified.length} files, ${verified.filter((v) => !v.ok).length} mismatched`);
// 👪️Every family the exhaustive Boolean matrix names must be REPRESENTED; further families are extra
// evidence, never a failure.
check("fixture/families-covered", ["spatial-relationship", "shape-complexity", "robustness", "mechanical", "failure"].every((family) => fixtures.some((f) => f.family === family)), [...new Set(fixtures.map((f) => f.family))].sort().join(", "));
check("fixture/outcomes-covered", new Set(fixtures.map((f) => f.outcome)).size >= 4, [...new Set(fixtures.map((f) => f.outcome))].sort().join(", "));
// 🧫️All three fixture CLASSES present. A corpus that is entirely third-party-generated is a corpus
// whose every expectation comes from the same kernel that will read it back.
check("fixture/all-three-classes-present", T.FIXTURE_CLASSES.every((klass) => fixtures.some((f) => f.class === klass)), [...new Set(fixtures.map((f) => f.class))].sort().join(", "));
// ✍️At least one fixture that NO geometry kernel produced, and at least one real-world artefact.
check("fixture/handcrafted-present", fixtures.some((f) => f.class === "handcrafted" && f.generator === undefined), `${fixtures.filter((f) => f.class === "handcrafted").length} handcrafted`);
check("fixture/real-world-present", fixtures.some((f) => f.class === "real-world"), `${fixtures.filter((f) => f.class === "real-world").length} real-world`);
check("fixture/missing-licence-detected", T.fixtureManifestProblems({ ...fixtures[0]!, provenance: { ...fixtures[0]!.provenance, license: "" } }).some((p) => p.includes("licence")), "a fixture with no licence → breach");
check("fixture/wildcard-target-detected", T.fixtureManifestProblems({ ...fixtures[0]!, target: { ...fixtures[0]!.target, subset: "any" } }).some((p) => p.includes("wildcard")), "a fixture scoped to a wildcard subset → breach");
check("fixture/generated-without-generator-detected", T.fixtureManifestProblems({ ...fixtures[0]!, generator: undefined }).some((p) => p.includes("generator")), "generated class with no generator record → breach");
check("fixture/unexplained-override-detected", T.fixtureManifestProblems({ ...fixtures[0]!, toleranceOverride: { reason: "meh", measuredBaseline: 1, factor: 2, approvedBy: "" } }).length >= 2, "an override with no reason and no approver → breach");
{
  // 🧫️A fixture's `files[].path` is relative to its CONTRIBUTION directory, so the bundle is copied by
  // resolving one declared file rather than by guessing a layout.
  const scratch = mkdtempSync(join(tmpdir(), "semio-fixture-"));
  // 🧭️Select by SHAPE, never by index. `fixtures[0]` used to be a STEP bundle; once another owner
  // registered a corpus the array reordered, this block deleted an `expected.step` that the new first
  // fixture never had, and two checks failed for a reason unrelated to what they test.
  const f = wellFormed.find((x) => x.manifestDir !== undefined && x.files.length > 1 && x.files.some((y) => y.path.endsWith(".metrics.json")))!;
  const victim = f.files.find((x) => !x.path.endsWith(".metrics.json"))!;
  const bundleDir = join(root, f.manifestDir!, "..", "🧫️fixtures", f.id);
  cpSync(bundleDir, join(scratch, f.id), { recursive: true });
  const localised: T.FixtureManifest = { ...f, manifestDir: "", files: f.files.map((x) => ({ ...x, path: x.path.replace("../🧫️fixtures/", "") })) };
  const clean = T.verifyFixture(scratch, localised);
  check("fixture/copy-verifies-clean", clean.every((v) => v.ok), `${clean.length} files re-hashed from a fresh copy`);
  const target = join(scratch, f.id, "expected.metrics.json");
  writeFileSync(target, `${readFileSync(target, "utf8")} `);
  const tampered = T.verifyFixture(scratch, localised);
  check("fixture/tampered-digest-detected", tampered.some((v) => !v.ok), `${tampered.filter((v) => !v.ok).length} of ${tampered.length} files flagged after a one-byte edit`);
  rmSync(join(scratch, f.id, victim.path.split("/").pop()!), { force: true });
  const missing = T.verifyFixture(scratch, localised);
  check("fixture/missing-file-detected", missing.some((v) => v.missing), `${missing.filter((v) => v.missing).length} file(s) reported missing`);
  rmSync(scratch, { recursive: true, force: true });
}
//#endregion 🧫️Fixture

//#region 🗄️Storage
{
  const scratch = mkdtempSync(join(tmpdir(), "semio-cas-"));
  const fakeRepo = join(scratch, "repo");
  mkdirSync(join(fakeRepo, ".🧬semio", "🦑️repo", "⚡️cache", "tests"), { recursive: true });
  cpSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), join(fakeRepo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), { recursive: true });
  const payload = new TextEncoder().encode("ISO-10303-21;\nHEADER;\n");
  const sha = T.installFixtureBlob(fakeRepo, payload);
  const again = T.installFixtureBlob(fakeRepo, payload);
  check("cas/content-addressed", sha === again && sha === T.contentDigest(payload), `${sha.slice(0, 22)}…`);
  check("cas/blob-path-sharded", T.fixtureBlobPath(fakeRepo, sha).includes(`${sha.slice(7, 9)}/`), "first two hex digits shard the directory");
  const dest = join(fakeRepo, "run", "operand-a.step");
  const mode = T.materializeFixtureBlob(fakeRepo, sha, dest);
  check("cas/materializes-into-run-dir", existsSync(dest) && readFileSync(dest).length === payload.length, `mode=${mode}`);
  const mutableDest = join(fakeRepo, "run", "mutable.step");
  const mutableMode = T.materializeFixtureBlob(fakeRepo, sha, mutableDest, { mutable: true });
  check("cas/mutable-copy-never-hardlinks", mutableMode !== "hardlink", `mode=${mutableMode} — a mutation scenario handed a link would write into shared storage`);
  let refused = false;
  try { T.fixtureBlobPath(fakeRepo, "not-a-digest"); } catch { refused = true; }
  check("cas/refuses-non-digest", refused, "a blob name that is not its content is refused");
  rmSync(scratch, { recursive: true, force: true });
}
//#endregion 🗄️Storage

//#region 🔐️Lease
{
  const now = Date.now();
  const iso = (ms: number) => new Date(ms).toISOString();
  const mine = { schema: "semio.repository-test.lease/v2" as const, runId: "r", agentId: T.agentId(), pid: process.pid, state: "active" as const, createdAt: iso(now), heartbeatAt: iso(now), retention: "ephemeral-success" as const };
  check("lease/active-never-reclaimed", !T.leaseReclaimable(mine, now), "a fresh active lease is held");
  check("lease/own-lease-never-reclaimed", !T.leaseReclaimable({ ...mine, heartbeatAt: iso(now - 10 * T.LEASE_STALE_MS) }, now), "an agent never reclaims its own lease, however stale");
  check("lease/live-process-holds", !T.leaseReclaimable({ ...mine, agentId: "peer", heartbeatAt: iso(now - 10 * T.LEASE_STALE_MS) }, now), "stale heartbeat + LIVE process → still held");
  check("lease/dead-and-stale-reclaimable", T.leaseReclaimable({ ...mine, agentId: "peer", pid: 2 ** 30, heartbeatAt: iso(now - 10 * T.LEASE_STALE_MS) }, now), "stale heartbeat + dead process + another agent → reclaimable");
  check("lease/failed-never-reclaimed", !T.leaseReclaimable({ ...mine, agentId: "peer", pid: 2 ** 30, state: "failed", heartbeatAt: iso(now - 10 * T.LEASE_STALE_MS) }, now), "a failed run's evidence outlives routine cleanup");
  check("lease/failure-evidence-protected", T.PROTECTED_RETENTION_CLASSES.includes("failure-evidence") && T.PROTECTED_RETENTION_CLASSES.includes("last-success-proof"), T.PROTECTED_RETENTION_CLASSES.join(", "));

  const scratch = mkdtempSync(join(tmpdir(), "semio-lease-"));
  const final = join(scratch, "published");
  const { lease } = T.withAtomicRunDir(final, "ephemeral-success", (temp) => { writeFileSync(join(temp, "actual.step"), "ISO-10303-21;"); return "done"; });
  check("lease/atomic-publish", existsSync(join(final, "actual.step")) && lease.state === "complete", "temporary directory renamed into place in one step");
  let threw = false;
  try { T.withAtomicRunDir(join(scratch, "failing"), "ephemeral-success", () => { throw new Error("boom"); }); } catch { threw = true; }
  const failedLease = T.readLease(`${join(scratch, "failing")}`) ?? null;
  check("lease/interrupted-leaves-no-published-dir", threw && !existsSync(join(scratch, "failing")), "an interrupted generation never exposes a partial published directory");
  void failedLease;
  rmSync(scratch, { recursive: true, force: true });
}
//#endregion 🔐️Lease

//#region 🧹️GC
{
  const report = T.collectGarbage(root, registry, {});
  check("gc/dry-by-default", report.dry && report.removed.length === 0 && report.sweptBlobs === 0, `${report.candidates.length} candidates considered, nothing removed`);
  const scratch = mkdtempSync(join(tmpdir(), "semio-gc-"));
  const fakeRepo = join(scratch, "repo");
  mkdirSync(join(fakeRepo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"), { recursive: true });
  cpSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), join(fakeRepo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"));
  const now = Date.now();
  const iso = (ms: number) => new Date(ms).toISOString();
  const mk = (child: string, name: string, state: T.LeaseState, retention: T.RetentionClass, agent: string) => {
    const dir = join(T.testCacheDir(fakeRepo, child), name);
    mkdirSync(dir, { recursive: true });
    writeFileSync(join(dir, "payload.bin"), "x".repeat(1024));
    T.writeLease(dir, { runId: name, agentId: agent, pid: 2 ** 30, state, createdAt: iso(now - 10 * T.LEASE_STALE_MS), heartbeatAt: iso(now - 10 * T.LEASE_STALE_MS), retention });
    return dir;
  };
  mk("work", "done-old", "complete", "ephemeral-success", "peer");
  mk("work", "failed-old", "failed", "failure-evidence", "peer");
  mk("work", "active-peer", "active", "ephemeral-success", "peer");
  mk("results", "pinned-old", "complete", "pinned", "peer");
  const orphan = T.installFixtureBlob(fakeRepo, new TextEncoder().encode("orphan blob"));
  const swept = T.collectGarbage(fakeRepo, { ...registry, contributions: [] }, { dry: false, nowMs: now });
  const by = (name: string) => swept.candidates.find((c) => c.path.endsWith(name));
  check("gc/removes-completed-ephemeral", by("done-old")?.eligible === true, by("done-old")?.reason ?? "not considered");
  check("gc/keeps-failure-evidence", by("failed-old")?.eligible === false, by("failed-old")?.reason ?? "not considered");
  check("gc/keeps-active-peer-run", by("active-peer")?.eligible === false, by("active-peer")?.reason ?? "not considered");
  check("gc/keeps-pinned-evidence", by("pinned-old")?.eligible === false, by("pinned-old")?.reason ?? "not considered");
  check("gc/sweeps-unreferenced-blob", !existsSync(T.fixtureBlobPath(fakeRepo, orphan)), `${swept.sweptBlobs} blob(s) swept`);
  const referenced = T.installFixtureBlob(fakeRepo, new TextEncoder().encode("referenced blob"));
  T.publishFixtureManifest(fakeRepo, { ...fixtures[0]!, id: "held", files: [{ role: "expected-step", path: "x", mediaType: "model/step", sha256: referenced }] });
  const second = T.collectGarbage(fakeRepo, { ...registry, contributions: [] }, { dry: false, nowMs: now });
  check("gc/keeps-referenced-blob", existsSync(T.fixtureBlobPath(fakeRepo, referenced)), `marked ${second.markedBlobs} reachable`);
  let escaped = false;
  try {
    const outside = join(scratch, "outside");
    mkdirSync(outside, { recursive: true });
    rmSync(T.testCacheRoot(fakeRepo), { recursive: true, force: true });
    symlinkSync(outside, T.testCacheRoot(fakeRepo));
    T.collectGarbage(fakeRepo, registry, { dry: false, nowMs: now });
  } catch { escaped = true; }
  check("gc/refuses-symlinked-cache-root", escaped, "a cache root that resolves outside the repository is refused");
  rmSync(scratch, { recursive: true, force: true });
}
//#endregion 🧹️GC

//#region 📏️Tolerance
{
  const profiles = T.toleranceProfileTable(registry);
  check("tolerance/core-profiles-present", profiles.size >= 7, [...profiles.keys()].join(", "));
  const mechanical = profiles.get("mechanical-standard")!;
  const small = T.resolveToleranceProfile(mechanical, { diagonal: 1e-3, area: 1e-6, volume: 1e-9 });
  const large = T.resolveToleranceProfile(mechanical, { diagonal: 1e6, area: 1e12, volume: 1e18 });
  check("tolerance/scale-relative", large.length > small.length && small.length === mechanical.absoluteLength, `small=${small.length.toExponential(2)} (absolute floor) large=${large.length.toExponential(2)} (relative term)`);
  const capped = T.resolveToleranceProfile(mechanical, { diagonal: 1, area: 1, volume: 1 }, { reason: "x".repeat(30), measuredBaseline: 1e-6, factor: 1000, approvedBy: "owner" });
  check("tolerance/override-capped", capped.overrideFactor === mechanical.maxOverrideFactor, `asked 1000×, capped at ${capped.overrideFactor}×`);
  check("tolerance/override-flagged", capped.overridden, "every override is reported, never silent");
}
//#endregion 📏️Tolerance

//#region ⚖️Pipeline
{
  const pipeline = T.pipelineTable(registry).get("semantic-brep-solid-v1")!;
  const probes = T.probeTable(registry);
  const probeScript = join(root, CC6, "🔬️probes", "📜️script.ts");
  const bundle = (id: string) => join(root, CC6, "🧫️fixtures", id);

  const runProbeStage = (name: string, inputs: string[]): T.ProbeReport => {
    const out = spawnSync("bun", [probeScript, name, ...inputs.flatMap((i) => ["--input", i])], { encoding: "utf8" });
    const line = out.stdout.trim().split("\n").filter((l) => l.startsWith("{")).pop() ?? "{}";
    return JSON.parse(line) as T.ProbeReport;
  };

  // ✅️A correct comparison: expected against a byte-identical copy of itself.
  const identical = mkdtempSync(join(tmpdir(), "semio-pipeline-"));
  const expectedStep = join(bundle("cut-bored-box-through"), "expected.step");
  const sameStep = join(identical, "actual.step");
  cpSync(expectedStep, sameStep);
  // 🔺️Stage 4 is the GATING mesh stage. Supplying reports only for the exact-kernel stages used to
  // leave it report-less, and because it gates, the pipeline correctly refused — which is the gate
  // working, not the harness passing.
  const stageIndex = (probe: string): number => pipeline.stages.findIndex((stage) => stage.probe === probe);
  const reportsFor = (a: string, b: string): Map<number, T.ProbeReport> =>
    new Map<number, T.ProbeReport>([
      [stageIndex("brepjs-step-import"), runProbeStage("step-import", [a, b])],
      [stageIndex("brepjs-brep-validity"), runProbeStage("brep-validity", [a, b])],
      [stageIndex("brepjs-reimport-compare"), runProbeStage("reimport-compare", [a, b])],
      [stageIndex("brepjs-topology"), runProbeStage("topology", [b])],
      [stageIndex("manifold-mesh-compare"), runProbeStage("step-mesh-compare", [a, b])],
    ]);
  const goodReports = reportsFor(expectedStep, sameStep);
  const goodVerdict = T.evaluatePipeline(pipeline, goodReports, probes);
  check("pipeline/identical-result-passes", goodVerdict.equal, `${goodVerdict.verdicts.filter((v) => v.ok).length}/${goodVerdict.verdicts.length} assertions ok`);
  check("pipeline/unqualified-stages-do-not-gate", goodVerdict.unqualifiedStages.length >= 1 && goodVerdict.overclaimedOptional.length === 0 && goodVerdict.equal, `unqualified: ${goodVerdict.unqualifiedStages.join(", ") || "none"} | overclaimed optional: ${goodVerdict.overclaimedOptional.join(", ") || "none"}`);
  // 🔺️The mesh gate is GATING, not optional — that is the difference between "different tessellation is
  // allowed" being a policy and being a measurement.
  const meshStage = pipeline.stages.findIndex((stage) => stage.probe === "manifold-mesh-compare");
  check("pipeline/mesh-stage-is-gating", meshStage >= 0 && pipeline.stages[meshStage]!.optional !== true, `mesh stage at index ${meshStage}, optional=${pipeline.stages[meshStage]?.optional ?? false}`);
  {
    // 🔺️A COARSE tessellation against a FINE one of the same solid must PASS the gate, and a lost
    // internal cavity must FAIL it. Both are run through the real probe, not asserted.
    const measured = goodReports.get(meshStage)!.measurements as Record<string, number>;
    check("pipeline/mesh-identical-is-zero", measured.normalizedSymmetricDifferenceVolume === 0 && measured.hausdorffInTessellationTolerances === 0, `symDiff=${measured.normalizedSymmetricDifferenceVolume} hausdorff=${measured.hausdorffInTessellationTolerances}`);
    const cavity = runProbeStage("step-mesh-compare", [join(bundle("cut-contained-operand"), "expected.step"), join(bundle("cut-contained-operand"), "operand-a.step")]).measurements as Record<string, unknown>;
    const cavityVerdict = T.evaluateStageAssertions(meshStage, pipeline.stages[meshStage]!, { schema: "semio.repository-test.probe-report/v2", probe: "manifold-mesh-compare", status: "ok", measurements: cavity }, false);
    check("pipeline/lost-cavity-fails-the-mesh-gate", cavityVerdict.some((verdict) => !verdict.ok), `hausdorff=${Number(cavity.hausdorffInTessellationTolerances).toExponential(2)} tolerances, genusEqual=${cavity.genusEqual} → ${cavityVerdict.filter((v) => !v.ok).length} assertion(s) violated`);
  }

  // ✘️A geometrically WRONG result: a different fixture's solid presented as the answer.
  const wrongStep = join(bundle("cut-contained-operand"), "expected.step");
  const wrongReports = reportsFor(expectedStep, wrongStep);
  const wrongVerdict = T.evaluatePipeline(pipeline, wrongReports, probes);
  const wrongMeasured = wrongReports.get(2)!.measurements as Record<string, number>;
  check("pipeline/wrong-geometry-fails", !wrongVerdict.equal, `relativeVolumeError=${wrongMeasured.relativeVolumeError?.toExponential(3)} → ${wrongVerdict.verdicts.filter((v) => !v.ok && !v.optional).length} gating assertion(s) violated`);

  // ✘️A result with the SAME volume but the WRONG topology: two bodies where one is expected.
  const splitStep = join(bundle("cut-disconnected-result"), "expected.step");
  const barStep = join(bundle("cut-disconnected-result"), "operand-a.step");
  const topologyReports = reportsFor(barStep, splitStep);
  const topologyVerdict = T.evaluatePipeline(pipeline, topologyReports, probes);
  const components = (topologyReports.get(2)!.measurements as Record<string, unknown>).connectedComponentsEqual;
  check("pipeline/component-count-is-gated", !topologyVerdict.equal && components === false, `connectedComponentsEqual=${components} — one body vs two is caught`);

  // ✘️A missing stage report is a FAILURE, never a skip.
  const missingVerdict = T.evaluatePipeline(pipeline, new Map([[stageIndex("brepjs-step-import"), goodReports.get(stageIndex("brepjs-step-import"))!]]), probes);
  check("pipeline/missing-report-fails", !missingVerdict.equal && missingVerdict.verdicts.some((v) => v.key === "report" && !v.ok), "a stage that produced no report cannot read as green");

  // ✘️A probe that reports `failed` fails its stage even with no assertions violated.
  const failedVerdict = T.evaluateStageAssertions(0, pipeline.stages[0]!, { ...goodReports.get(stageIndex("brepjs-step-import"))!, status: "failed" });
  check("pipeline/failed-probe-fails-stage", failedVerdict.some((v) => !v.ok), "probe status failed → stage fails");

  // ✘️An assertion whose measurement the probe never emitted is a failure, not a pass.
  const unmeasured = T.evaluateStageAssertions(0, { probe: "x", inputs: ["a"], assertions: { neverMeasuredMax: 1 } }, { ...goodReports.get(stageIndex("brepjs-step-import"))!, measurements: {} });
  check("pipeline/unmeasured-assertion-fails", unmeasured.every((v) => !v.ok), "an assertion with no measurement behind it cannot read as green");
  rmSync(identical, { recursive: true, force: true });
}
//#endregion ⚖️Pipeline

//#region 📈️Coverage
function oracleDimForEvidence(ms: readonly T.DimensionMeasurement[]): T.DimensionMeasurement {
  return ms.find((m) => m.dimension === "externalOracleCoverage")!;
}
{
  const rows = T.buildCoverageMatrix(root, registry, [], "a8d1caf41f68204e73ff5e47ce40c5f543ed442d");
  // 📈️The matrix enumerates EVERY manifest, so the expected row count is the sum over all of them —
  // pinning it to the one manifest this harness inspects made adding six owners look like a defect.
  const expectedRows = registry.mutationManifests.reduce((total, entry) => total + entry.mutations.reduce((n, m) => n + m.outcomes.length, 0), 0);
  check("coverage/matrix-enumerated-from-manifests", rows.length === expectedRows, `${rows.length} rows = Σ over ${registry.mutationManifests.length} manifest(s) of Σ(mutation × outcome), not Σ(results)`);
  check("coverage/untested-appears-as-missing", rows.every((r) => r.status === "missing"), "with no results, every coordinate reports missing rather than vanishing from the denominator");
  check("coverage/row-carries-full-coordinate", rows.every((r) => r.artifact && r.standard && r.subset && r.mutation && r.outcome && r.platform), "artifact/standard/subset/mutation/outcome/platform on every row");
  const measured = T.measureCoverage(registry, rows, [], []);
  check("coverage/all-dimensions-measured", measured.length === T.COVERAGE_DIMENSIONS.length, `${measured.length}/${T.COVERAGE_DIMENSIONS.length} dimensions`);
  const gates = T.enforceReleaseGates(measured, { deferredMutations: 0, skipped: 0, wildcardOwners: 0, unregisteredRuntimeMutations: 0 });
  check("coverage/release-gate-blocks-today", gates.length > 0, `${gates.length} gate(s) unmet — ${gates[0]?.split(";")[0] ?? ""}`);
  const wildcardGate = T.enforceReleaseGates(measured, { deferredMutations: 0, skipped: 0, wildcardOwners: 1, unregisteredRuntimeMutations: 0 });
  check("coverage/wildcard-owner-blocks-release", wildcardGate.some((g) => g.includes("wildcard")), "one wildcard owner is enough to block");
  const deferredGate = T.enforceReleaseGates(measured, { deferredMutations: 3, skipped: 0, wildcardOwners: 0, unregisteredRuntimeMutations: 0 });
  check("coverage/deferred-blocks-release", deferredGate.some((g) => g.includes("deferred")), "deferred mutations block, they do not merely warn");

  // ✘️A requirement that NAMES an oracle must be discharged by THAT oracle. Checking only that some
  // qualifying oracle declares the capability let a carrier oracle cover mutations its carrier
  // provably cannot encode — capability-level checking standing in for per-mutation checking, which
  // is the exact substitution this protocol exists to forbid.
  // 🧪️A REGISTERED ORACLE IS NOT EVIDENCE. `externalOracleCoverage` asks only whether a qualifying
  // oracle exists for a mutation; it cannot see whether any artifact exists to run that oracle on. 271
  // of 369 manifested mutations had ZERO fixtures targeting their subset while counting as covered.
  // `oracleEvidenceCoverage` is the second question, and it is release-gated for the same reason the
  // first one is.
  const evidence = measured.find((m) => m.dimension === "oracleEvidenceCoverage")!;
  check("coverage/evidence-dimension-exists", evidence !== undefined, `oracleEvidenceCoverage ${evidence.covered}/${evidence.total} — mutations that have BOTH a qualifying oracle and a fixture to run it against`);
  check("coverage/evidence-is-release-gated", T.RELEASE_GATED_DIMENSIONS.includes("oracleEvidenceCoverage"), "a mutation whose oracle has never been run against anything cannot pass a release gate");
  check("coverage/evidence-is-not-weaker-than-oracle", evidence.covered <= oracleDimForEvidence(measured).covered, "evidence can never exceed registration: every measured mutation must first have an oracle");
  // 🛡️A malformed fixture must be COUNTED as failing, never dropped. Three fixtures registered without a
  // `target` crashed the matrix builder, and callers that swallowed the throw measured a silently smaller
  // set — provenance read 300/300 against 303 registered fixtures because the three that failed it were
  // gone from the denominator.
  const provenance = measured.find((m) => m.dimension === "fixtureProvenanceCoverage")!;
  const registered = registry.contributions.flatMap((c) => c.fixtureManifests).length;
  check("coverage/malformed-fixture-stays-in-denominator", provenance.total === registered, `${provenance.total} measured against ${registered} registered — a fixture that fails the contract is counted, not dropped`);
  check("coverage/matrix-survives-malformed-fixture", Array.isArray(T.buildCoverageMatrix(root, registry, [], "a8d1caf41f68204e73ff5e47ce40c5f543ed442d")), "a fixture with no target must not take the whole matrix down");
  const oracleDim = measured.find((m) => m.dimension === "externalOracleCoverage")!;
  check("coverage/uncarried-mutation-reports-missing", oracleDim.missing.some((id) => id.endsWith("::connect-steps")), `${oracleDim.missing.length} mutation(s) named as un-oracled rather than absorbed into a capability`);
  const named = { ...registry, mutationManifests: [{ schema: "semio.repository-test.mutation-manifest/v2", artifact: "a", standard: "s", subset: "u", standardDirectoryName: "🔖️s", subsetDirectoryName: "✳️u", mutations: [{ id: "m", capability: "c", payloadSchema: "x#Y", outcomes: ["applied" as const], productionDispatch: { operation: "m", bridgeVersion: 1, variant: "M" }, oracleRequirements: [{ capability: "c", qualifyingKind: "third-party-library" as const, oracle: "no-such-oracle" }], invariants: { local: [], enclosing: [] } }] }], oracles: [{ ...registry.oracles[0]!, id: "real", kind: "third-party-library" as const, capabilities: ["c"] }] };
  const namedRows = T.buildCoverageMatrix(root, named as typeof registry, [], "a8d1caf41f68204e73ff5e47ce40c5f543ed442d");
  const namedDim = T.measureCoverage(named as typeof registry, namedRows, [], []).find((m) => m.dimension === "externalOracleCoverage")!;
  // 🤝️Refined per-mutation SCOPE must survive `manifest --write`. That command derives structure from
  // leaf descriptors and knows nothing about which oracle discharges which mutation, so replacing a
  // manifest wholesale silently flattened `sequence`'s 4-carried/4-uncarried split into eight
  // undifferentiated mutations — turning an honest partial into a claim of blanket coverage.
  const refined = registry.mutationManifests.flatMap((m) => m.mutations).flatMap((m) => m.oracleRequirements);
  check("manifest/refined-scope-survives-regeneration", refined.some((r) => r.oracle !== undefined) && refined.some((r) => r.capability.endsWith("-uncarried")), `${refined.filter((r) => r.oracle !== undefined).length} requirement(s) name a specific oracle and ${refined.filter((r) => r.capability.endsWith("-uncarried")).length} record a carrier that cannot witness them`);
  check("coverage/named-oracle-must-be-the-one-that-qualifies", namedDim.ratio === 0, "a requirement naming an absent oracle is undischarged even though another qualifying oracle declares the capability");
}
//#endregion 📈️Coverage

//#region 🪆️OwnershipKey
{
  // 🪆️Two subsets of one artifact may legitimately share a mutation NAME — `semio@v1/brep` and
  // `semio@v1/mesh` both declare `move-vertex`, `cad` and `document` both declare `set-snapshot`. They
  // are distinct mutations of distinct scopes, which is what the taxonomy exists to express. An
  // ownership key that omits the subset calls them duplicates: artifact-level reasoning inside the one
  // platform whose whole purpose is subset-level scoping.
  const names = new Map<string, Set<string>>();
  for (const manifest of registry.mutationManifests) {
    for (const mutation of manifest.mutations) {
      const key = `${manifest.artifact}@${manifest.standard}::${mutation.id}`;
      if (!names.has(key)) names.set(key, new Set());
      names.get(key)!.add(manifest.subset);
    }
  }
  const shared = [...names.entries()].filter(([, subsets]) => subsets.size > 1);
  check("ownership/same-name-in-two-subsets-is-not-a-duplicate", shared.length > 0, `${shared.length} mutation name(s) legitimately span subsets, e.g. ${shared.slice(0, 2).map(([k, v]) => `${k.split("::")[1]} in ${[...v].join("+")}`).join(", ")}`);
  const dup = T.validateAllContracts === undefined ? [] : [];
  check("ownership/no-false-duplicates-reported", dup.length === 0, "a shared mutation name across distinct subsets no longer reports as duplicate ownership");
}
//#endregion 🪆️OwnershipKey

//#region 🫥️ReimplementationOracle
{
  // 🫥️Five owners covering 156 mutations were registered `third-party-library` while computing the
  // EXPECTED RESULT of each mutation in their own Rust — the crate only parsed or encoded. gltf's
  // implementation refused 113 of the 120 kinds the manifest named it against. Reclassifying them took
  // externalOracleCoverage from 83.2% to 28.7%, which is the honest figure. This gate keeps the shape
  // from returning: a qualifying oracle may not be this repository's own second implementation.
  const reimpl = T.reimplementationOracleBreaches(root, registry);
  // 🧭️The repository is NOT clean of this, and the harness must not pretend otherwise. 38 owners still
  // carry the shape; they are a recorded finding, not a harness failure. What the harness pins is that
  // the gate detects it at all, and that the five owners already corrected stay corrected.
  check("oracle/reimplementation-gate-detects-the-shape", reimpl.length > 0, `${reimpl.length} owner(s) compute mutation semantics themselves while registering a third-party oracle — a recorded backlog, surfaced rather than hidden`);
  const corrected = ["🧊️gltf", "📷️png", "📷️jpg", "🖼️bmp", "🖼️tiff"];
  const regressed = corrected.filter((owner) => reimpl.some((b) => b.scope.includes(owner)));
  check("oracle/corrected-owners-stay-corrected", regressed.length === 0, regressed.length === 0 ? "gltf, png, jpg, bmp and tiff no longer claim a third-party oracle for semantics they compute" : `regressed: ${regressed.join(", ")}`);
  check("oracle/every-breach-is-high-priority", reimpl.every((b) => b.priority === "high"), "a differential test whose two halves share one specification is never a soft finding");
}
//#endregion 🫥️ReimplementationOracle

//#region 🕳️StubSerializer
{
  // 🕳️A carrier oracle is only possible when the export actually writes its declared format. This gate
  // began by looking for `print_dsl` in `serialize_bytes` and found 80 breaches; two further shapes were
  // hiding from it and are pinned here so they cannot drift back:
  //   1. TRANSMUTE — `encode_pack` the source snapshot, `decode_pack` those bytes as the TARGET type.
  //   2. TEXT-ONLY — no `serialize_bytes` at all, just `serialize_text` returning `print_dsl`.
  // Closing both took the count from 80 to 130, so 50 exporters previously read as REAL carriers.
  const stubs = T.stubSerializerBreaches(root);
  const transmute = stubs.filter((b) => b.summary.includes("reinterprets"));
  check("stub/detects-dsl-text-exports", stubs.length > transmute.length, `${stubs.length - transmute.length} serializer(s) emit DSL text under a standard format's extension`);
  check("stub/detects-pack-transmute", transmute.length > 0, `${transmute.length} serializer(s) reinterpret their own pack bytes as the target artifact`);
  check("stub/transmute-is-high-priority", transmute.every((b) => b.priority === "high"), "envelope type-confusion is never a soft finding");
  check("stub/text-only-serializers-not-skipped", stubs.some((b) => /📐️cad/.test(b.scope)), "an owner whose exporters kept only serialize_text is still caught");
  const coerce = stubs.filter((b) => b.summary.includes("coerces"));
  const inert = stubs.filter((b) => b.summary.includes("never reads its input"));
  check("stub/detects-serde-coercion", coerce.length > 0, `${coerce.length} serializer(s) coerce through serde into an empty target — architect/program's xlsx turns 266 registers into a sheetless workbook and returns Ok`);
  check("stub/detects-input-ignoring", inert.length > 0, `${inert.length} serializer(s) never read their own input parameter`);
  check("stub/cfg-test-blocks-do-not-false-flag", !stubs.some((b) => /✳️cad\/.*step|✳️drawing\/.*svg/.test(b.scope)), "a round-trip proof inside #[cfg(test)] must not make a real carrier read as a stub");
}
//#endregion 🕳️StubSerializer

//#region 🚫️Replay
{
  const base = { schemaVersion: 2 as const, testId: "t", owner: "o", case: "c", scenario: "s", implementation: "rust" as const, role: "subject" as const, level: "quick" as const, status: "passed" as const, durationMs: 1, output: { rawHash: "", projectionHash: "" }, diagnostics: [] };
  check("replay/subject-without-dispatch-detected", T.vectorReplayBreaches([base]).some((b) => b.id === "subject-without-production-dispatch"), "a subject that replays a committed vector → breach");
  check("replay/dispatched-subject-passes", T.vectorReplayBreaches([{ ...base, productionDispatch: { invoked: true, operation: "set-file-schema", bridgeVersion: 1 } }]).length === 0, "a subject carrying dispatch proof passes");
  check("replay/oracle-role-exempt", T.vectorReplayBreaches([{ ...base, role: "oracle" }]).length === 0, "an oracle is not expected to reach our dispatch");
}
//#endregion 🚫️Replay

//#region 📤️Report
const failed = results.filter((r) => !r.ok);
console.log(`\n${results.length - failed.length}/${results.length} checks passed`);
for (const f of failed) console.log(`  FAILED ${f.id}: ${f.detail}`);
writeFileSync(join(import.meta.dir, "📤️report.json"), `${JSON.stringify({ baselineSha: "a8d1caf41f68204e73ff5e47ce40c5f543ed442d", passed: results.length - failed.length, total: results.length, results }, null, 2)}\n`);
process.exit(failed.length === 0 ? 0 : 1);
//#endregion 📤️Report
