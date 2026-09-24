"""📌️ One-off, pass 2: the check-in-check gate, the renamed native fixture law, and remaining upload references."""
p = "/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts"
s = open(p, encoding="utf-8").read()
def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:120], s.count(old))
    s = s.replace(old, new)
def replace_between(start_marker, end_marker, new):
    global s
    a = s.index(start_marker)
    b = s.index(end_marker, a)
    s = s[:a] + new + s[b:]
s = s.replace("checkpoint_publication_process_fixture_emits_verified_gis_pair_and_catalog", "check_in_process_fixture_emits_verified_gis_ledger_and_catalog")
s = s.replace('join(artifactRoot, "checkpoint-publication-process-fixture", "data")', 'join(artifactRoot, "check-in-process-fixture", "data")')
rep('''  const profile: LocalProfile = { profileId: "checkpoint-publication-process", subject: "checkpoint-publication-process-author", displayName: "Checkpoint Publication Author", allowedClientClasses: ["native", "mcp"] };''', '''  const profile: LocalProfile = { profileId: "check-in-process", subject: "check-in-process-author", displayName: "Check In Author", allowedClientClasses: ["native", "mcp"] };''')
rep('''clientInfo: { name: "semio-checkpoint-publication-oracle", version: "1" }''', '''clientInfo: { name: "semio-check-in-oracle", version: "1" }''')
rep('''    retained.forEach((bytes) => bytes.fill(0));
    await finishLocalHub(run).catch((error) => cleanupErrors.push(error));''', '''    await finishLocalHub(run).catch((error) => cleanupErrors.push(error));''')
rep('''      initialCheckpoint: "normal-checkpoint-publication",''', '''      initialCheckpoint: "hub-materialized-check-in",''')
rep('''async function readGisMapProcessCheckpoint(''', '''/** 🧯️ The hub authority's own canonical pair ceiling (`AUTHORITY_MAX_PAIR_BYTES`). */
const GIS_MAP_PROCESS_PAIR_MAX_BYTES = 64 * 1024 * 1024;

async function readGisMapProcessCheckpoint(''')
replace_between("async function proveCheckpointPublicationCommandV1(repoRoot: string): Promise<number> {", "function proveSpaceArtifactCreationContractV1(", '''/** 📌️ Check In contract oracle: every language-neutral vector through Ajv (third party) and the
 * TypeScript twin, then the source boundary the command depends on end to end. */
async function proveDocumentCheckInV1(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📌️document-check-in-v1/🧫️fixtures/🔣️.json"), "utf8"));
  const validateRequest = hubSchemaExport(repoRoot, "schema://os.directory/DocumentCheckInV1");
  const validateStatus = hubSchemaExport(repoRoot, "schema://os.directory/DocumentCheckInStatusV1");
  let checks = 0;
  const parseJson = (source: string): unknown => {
    try {
      return JSON.parse(source);
    } catch {
      return undefined;
    }
  };
  for (const row of fixture.valid.requests) {
    if (!validateRequest(parseJson(row.source)) || parseDocumentCheckInV1(row.source) === null) throw new Error(`check-in valid request refused: ${row.name}`);
    checks++;
  }
  for (const row of fixture.valid.statuses) {
    if (!validateStatus(parseJson(row.source)) || parseDocumentCheckInStatusV1(row.source) === null) throw new Error(`check-in valid status refused: ${row.name}`);
    checks++;
  }
  for (const row of fixture.invalid.requests) {
    if (Boolean(validateRequest(parseJson(row.source))) !== row.schemaValid || parseDocumentCheckInV1(row.source) !== null) throw new Error(`check-in invalid request differs: ${row.name}`);
    checks++;
  }
  for (const row of fixture.invalid.statuses) {
    if (Boolean(validateStatus(parseJson(row.source))) !== row.schemaValid || parseDocumentCheckInStatusV1(row.source) !== null) throw new Error(`check-in invalid status differs: ${row.name}`);
    checks++;
  }
  const hub = readFileSync(join(repoRoot, "🌎️hub/🏗️bootstrap/🦀️.rs"), "utf8");
  const job = readFileSync(join(repoRoot, "🌎️hub/🗿️artifact-authority/📌️check-in/🦀️.rs"), "utf8");
  const ledger = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs"), "utf8");
  const store = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"), "utf8");
  const wit = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit"), "utf8");
  for (const [name, body, markers] of [
    ["hub", hub, ["/spaces/{space_id}/documents/{document_id}/check-ins", "/spaces/{space_id}/documents/{document_id}/check-ins/{request_id}/cancel", "artifact_ledger_tail", "claim_or_read_checkpoint_publication"]],
    ["job", job, ["TrustedArtifactReplayCodec", "materialize_check_in", "DocumentCheckInRefusalV1"]],
    ["ledger", ledger, ["pub async fn artifact_ledger_tail"]],
    ["store", store, ["pub async fn replay_envelopes_onto_pair", "ingest_remote"]],
    ["wit", wit, ["replay-envelopes: async func"]],
  ] as const) {
    for (const marker of markers) if (!body.includes(marker)) throw new Error(`check-in ${name} boundary is missing ${marker}`);
    checks++;
  }
  if (hub.includes("checkpoint-publications")) throw new Error("the client upload route must stay deleted");
  console.log(`document-check-in-oracle: requests=${fixture.valid.requests.length}+${fixture.invalid.requests.length} statuses=${fixture.valid.statuses.length}+${fixture.invalid.statuses.length} ajv=2 typescript=2 boundaries=5`);
  return checks;
}

''')
rep('''  const current = JSON.parse(readFileSync(join(base, "../🌱️artifact-genesis-v1/📤️current.json"), "utf8"));
  const validateCurrent = hubSchemaExport(repoRoot, "schema://os.directory/CheckpointPublicationCommandV1");
  for (const row of current.cases) {
    let accepted = false;
    try { parseCheckpointPublicationCommandV1(JSON.stringify(row.command)); accepted = true; } catch {}
    if (accepted !== row.accepted || Boolean(validateCurrent(row.command)) !== row.accepted) throw new Error(`genesis publication current differs: ${row.id}`);
  }
  console.log(`[DEBUG] genesis publication current: TypeScript=${current.cases.length} AJV=1; no Directory lineage or backend transaction executed`);
''', '')
replace_between("class CheckpointPublicationCheckScript extends BundleScript {", "class DirectoryEventPageV1CheckScript extends BundleScript {", '''/** 📌️ `check-in-check source|native|process`: contract oracle, in-process hub laws, and the MCP process. */
class CheckInCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native", "process"].includes(phase)) throw new Error("check-in-check accepts source, native, or process");
    const checks = await proveDocumentCheckInV1(this.repoRoot);
    if (phase === "native" || phase === "process") {
      const laws = [
        "tests::check_in_advances_the_active_checkpoint_to_the_named_head_and_cold_opens_from_it",
        "tests::check_in_refuses_stale_unknown_and_foreign_heads_and_is_author_owned",
        "tests::check_in_is_idempotent_per_request_and_cancellable",
      ];
      if (phase === "process") laws.push("tests::check_in_process_fixture_emits_verified_gis_ledger_and_catalog");
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [
          {
            package: "semio-hub",
            target: { kind: "bin", name: "os-hub" },
            cargoArgs: ["--no-default-features", "--features", phase === "process" ? "sqlite,integration-fixtures,native-artifact-execution" : "sqlite,native-artifact-execution"],
            laws,
          },
        ],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: buildBudgetMs(),
        listBudgetMs: 60_000,
        lawBudgetMs: 180_000,
        progress(event) {
          console.log(`check-in-${phase} ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      for (const receipt of receipts) console.log(`check-in-${phase}-receipt: ${JSON.stringify(receipt)}`);
    }
    if (phase === "process") {
      const nativeEnv = { ...process.env, RUST_MIN_STACK: "268435456" };
      runCargo(["build", "--manifest-path", "Cargo.toml", "-p", "semio-hub", "--bin", "os-hub", "--no-default-features", "--features", "sqlite,native-artifact-execution"], this.repoRoot, nativeEnv);
      runCmd("bun", ["nx", "run", "@semio-tech/framework-os-mcp-rs:build", "--skip-nx-cache"], { cwd: this.repoRoot, env: nativeEnv, ...orchestratorBudgetOpts() });
      await proveCheckInMcpProcess(this.repoRoot, this.root);
      console.log("check-in-process: real GIS Map ledger edit -> hub-materialized Check In -> credential-FD MCP scoped checkpoint resource passed");
    }
    console.log(`check-in-check: checks=${checks} phase=${phase}`);
  }
}

''')
rep('''  .register("checkpoint-publication-check", CheckpointPublicationCheckScript)''', '''  .register("check-in-check", CheckInCheckScript)''')
open(p, "w", encoding="utf-8").write(s)
print("ok")
