# Complete Original Store Lifetime Gate Registration Inputs

## 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts

SHA256 751534a5b2948224e77cdba2816bedd863496d75ee74816176c628264507edcd

````text
#!/usr/bin/env bun
import { runVitestV1, readVitestPolicyV1 } from "../../../../🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { resolveTestLevel } from "../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🦀️ `@semio-tech/framework-os-kernel` task router. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { type ValidateFunction } from "ajv";
import Ajv2020 from "ajv/dist/2020.js";
import { runCargo, runRepositoryCargoTests, runRepositoryTestCommand, runRepositoryExactCargoLaws } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runNestedCargoPackageAdapter } from "../../../🦑️repo/🔨️modules/📚️library/📽️projection/🧩️package-adapter/📦️publication/🟦️.ts";
import { blake3Hex } from "../../../../🔨️modules/🔏️hash/🟦️.ts";
import { semioSchemaAjvV1 } from "../../../../🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";

/** 🧬️ A compiled owned-schema export, typed as a boolean runtime check so `assert` never narrows its validated subject to `unknown`. */
type SchemaCheck = ((data: unknown) => boolean) & Pick<ValidateFunction, "errors">;

//#region 🧬️OwnedSchemaExports
const OS_MODULE_SCHEMAS = {
  "db.engine": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧬️schema/🔣️.json",
  "db.wal": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧬️schema/🔣️.json",
  "db.storage": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧬️schema/🔣️.json",
  "db.storage.writer": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🧬️schema/🔣️.json",
  "db.compact": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🧬️schema/🔣️.json",
  "db.artifact": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧬️schema/🔣️.json",
  directory: "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json",
} as const;

/** 🧬️ Compiles one named `$defs` export of an owning `🧬️schema/` module against its draft-07 `$id`. */
function ownedExport(repoRoot: string, scope: keyof typeof OS_MODULE_SCHEMAS, exportId: string): SchemaCheck {
  const doc = JSON.parse(readFileSync(join(repoRoot, OS_MODULE_SCHEMAS[scope]), "utf8")) as { $id: string };
  const compiled = semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(doc).getSchema(`${doc.$id}#/$defs/${exportId}`);
  if (!compiled) throw new Error(`${scope} schema module publishes no export ${exportId}`);
  return compiled as ValidateFunction;
}
//#endregion 🧬️OwnedSchemaExports


function exactCargoStageEnvironments() {
  return {
    env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },
    nativeEnv: { RUST_MIN_STACK: "268435456" },
  };
}

/** ↔️ Admits portable paged traversal and independently checks array/deque ordering. */
class PagedHistoryStackScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length > 1 || (args.length && args[0] !== "--native")) throw Error("paged-history-stack-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/📸️paged-history-stack/🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/📸️paged-history-stack/🔣️.json"), "utf8"));
    const admit = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
    assert(admit(fixture), JSON.stringify(admit.errors));
    for (const vector of fixture.vectors) {
      const values = Array.from({ length: vector.pushes }, (_, i) => String(i));
      if (vector.removeLogical !== null) values.splice(vector.removeLogical, 1);
      if (vector.replacement !== null) values.push(vector.replacement);
      assert.deepEqual(values, vector.forward);
      assert.deepEqual([...values].reverse(), vector.reverse);
      assert.deepEqual(vector.directions.map((direction: string) => (direction === "front" ? values.shift() : values.pop()) ?? null), vector.expected);
      assert.deepEqual(values, vector.remaining);
    }
    const branchSchema = JSON.parse(readFileSync(join(owner, "🧬️schema/🌿️branch-provenance/🔣️.json"), "utf8"));
    const branchFixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🌿️branch-provenance/🔣️.json"), "utf8"));
    const branchAjv = semioSchemaAjvV1({ strict: true, allErrors: true });
    assert(branchAjv.compile(branchSchema)(branchFixture));
    const editCheck = branchAjv.compile(branchSchema.$defs.Edit);
    for (const vector of branchFixture.vectors) {
      assert.equal(editCheck(vector.edit), vector.valid, vector.id);
      assert.equal(Object.hasOwn(vector.edit, "line") && (vector.edit.line === null || typeof vector.edit.line === "string"), vector.valid, vector.id);
    }
    const { testCanonicalEditFixtures } = await import("../../🔨️modules/🏪️store/🧪️tests/🧵️canonical-edit/🟦️.ts");
    const { storeCanonicalEditSealerSelfTests } = await import("../../🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts");
    testCanonicalEditFixtures();
    const canonical = storeCanonicalEditSealerSelfTests();
    console.log("paged-history-canonical-oracles: " + JSON.stringify(canonical));
    if (args[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({ cwd: this.repoRoot, ...exactCargoStageEnvironments(),
        groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib", name: "semio_framework_os_kernel" }, laws: [
          "os_vcs::tests::paged_history_stack_traversal_follows_the_portable_deque_vectors",
          "os_vcs::tests::history_branch_provenance_follows_portable_required_wire_vectors",
          "os_store::component::canonical_edit::tests::edit_digest_chains_match_the_neutral_vectors_and_extend_incrementally",
          "os_store::component::canonical_edit::tests::canonical_authority_final_unicode_strings_retire_under_single_byte_grants",
        ] }], artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs: 3_600_000, listBudgetMs: 60_000, lawBudgetMs: 120_000 });
      for (const receipt of receipts) console.log("paged-history-stack-native-receipt: " + JSON.stringify(receipt));
    }
    console.log(`paged-history-stack-check: vectors=${fixture.vectors.length} independent-array/Ajv=passed`);
  }
}

/** 📜️ Verifies history-result publication and exact replay-owner retirement. */
class DatabaseHistoryCompletionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-history-completion-check accepts only --native");
    const root = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧫️fixtures/📜️history-completion");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.engine", "HistoryCompletionV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(fixture.publication.map((row: { name: string }) => row.name), ["before-poll", "before-registration", "after-pending"].flatMap(stage => ["success", "cancelled"].map(outcome => stage + "-" + outcome)));
    const { Database } = await import("bun:sqlite");
    const oracle = new Database(":memory:");
    try {
      oracle.run("CREATE TABLE handoff (payload TEXT, location TEXT, waiter INTEGER, wakes INTEGER, admission INTEGER, registry INTEGER)");
      for (const row of fixture.publication) {
        oracle.run("DELETE FROM handoff");
        const payload = row.outcome === "success" ? fixture.operations : { error: "Closed" };
        oracle.run("INSERT INTO handoff VALUES (?1, 'producer', 0, 0, 1, 1)", [JSON.stringify(payload)]);
        const snapshot = () => oracle.query("SELECT payload, location, waiter, wakes, admission, registry FROM handoff").get() as { payload: string; location: string; waiter: number; wakes: number; admission: number; registry: number };
        const publish = () => oracle.run("UPDATE handoff SET location = 'completion', wakes = wakes + waiter, waiter = 0 WHERE location = 'producer'");
        if (row.publication === "before-poll") publish();
        let firstReady = snapshot().location === "completion";
        if (!firstReady) {
          if (row.publication === "before-registration") publish();
          oracle.run("UPDATE handoff SET waiter = 1");
          firstReady = snapshot().location === "completion";
        }
        assert.equal(firstReady, row.firstReady, row.name);
        if (!firstReady) publish();
        assert.equal(oracle.run("UPDATE handoff SET location = 'consumer', waiter = 0 WHERE location = 'completion'").changes, 1, row.name);
        assert.equal(JSON.stringify(JSON.parse(snapshot().payload)) === JSON.stringify(payload), row.exactResult, row.name);
        assert.equal(snapshot().waiter === 0, row.waiterEmpty, row.name);
        assert.equal(snapshot().wakes, row.wakes, row.name);
        assert.equal(snapshot().waiter === 0, row.wakeLockReleased, row.name);
        oracle.run("UPDATE handoff SET location = 'empty', admission = 0, registry = 0 WHERE location = 'consumer'");
        assert.equal(snapshot().admission === 0, row.admissionReleased, row.name);
        assert.equal(snapshot().registry === 0, row.registryEmpty, row.name);
      }
    } finally {
      oracle.close();
    }
    console.log("database-history-completion-check: AJV=1 sqlite-publication=" + fixture.publication.length);
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [{
        package: "semio-framework-os-kernel-db",
        target: { kind: "lib", name: "db" },
        cargoArgs: ["--all-features"],
        laws: [
          "artifact_history_completion_interleavings_preserve_result_and_wake",
          "artifact_history_empty_and_two_batch_replay_are_deterministic",
          "artifact_history_empty_one_cap_plus_one_admission_returns_exact_request",
          "artifact_history_cancel_before_handoff_retires_full_reservation_before_credit_release",
          "artifact_history_public_terminal_close_releases_admission_only_after_roots_are_empty",
        ],
      }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log("database-history-completion-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);
      },
    });
    for (const receipt of receipts) console.log("database-history-completion-native-receipt: " + JSON.stringify(receipt));
  }
}

/** 📖️ Verifies catalog-root ownership independently of scalar capability opening. */
class DatabaseCatalogReadOwnershipCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-catalog-read-ownership-check accepts only --native");
    const root = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧫️fixtures/📖️catalog-read-ownership");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.engine", "CatalogReadOwnershipV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(fixture.transfers.map((row: { phase: string }) => row.phase), ["Handoff", "RetainWork", "Poll"]);
    assert.deepEqual(fixture.completion.map((row: { name: string }) => row.name), ["synchronous", "before-wake", "after-finalizer-check", "refused-finalizer"].flatMap(stage => ["pages", "backend-fault"].map(outcome => stage + "-" + outcome)));
    assert.deepEqual(fixture.recovery.map((row: { name: string }) => row.name), ["retry", "terminal-completion", "terminal-result", "spent-work"].flatMap(path => ["pages", "backend-fault"].map(outcome => path + "-" + outcome)));
    const { Database } = await import("bun:sqlite");
    const oracle = new Database(":memory:");
    try {
      oracle.run("CREATE TABLE ownership (active INTEGER, location TEXT, admission INTEGER, waiter INTEGER, releases INTEGER, payload TEXT)");
      const reset = (active: number, location: string, payload: unknown) => {
        oracle.run("DELETE FROM ownership");
        oracle.run("INSERT INTO ownership VALUES (?1, ?2, 1, 1, 0, ?3)", [active, location, JSON.stringify(payload)]);
      };
      const finalize = () => oracle.run("UPDATE ownership SET active = 4, admission = 0, releases = releases + 1 WHERE active = 0 AND location = 'empty' AND waiter = 0 AND admission = 1");
      for (const row of fixture.transfers) {
        reset(1, "stack", fixture.root);
        const close = oracle.run("UPDATE ownership SET admission = 0 WHERE active = 0 AND location = 'empty'").changes;
        assert.equal(close === 0, row.closeBlocked, row.name);
        const paused = oracle.query("SELECT admission, location FROM ownership").get() as { admission: number; location: string };
        assert.equal(paused.admission === 1, row.admissionRetained, row.name);
        assert.equal(paused.location === "stack", row.storageRetained, row.name);
        const submissions = oracle.run("UPDATE ownership SET active = 2 WHERE active = 0").changes;
        assert.equal(submissions, row.submissionsWhileActive, row.name);
        oracle.run("UPDATE ownership SET active = 0, location = 'empty', waiter = 0");
        finalize();
        assert.equal((oracle.query("SELECT admission = 0 AS empty FROM ownership").get() as { empty: number }).empty === 1, row.finalEmpty, row.name);
      }
      for (const row of fixture.completion) {
        const payload = row.outcome === "pages" ? fixture.root : { key: fixture.root.key, error: "fixture-root-fault" };
        reset(row.publication === "synchronous" ? 0 : 1, "completion", payload);
        assert.deepEqual(JSON.parse((oracle.query("SELECT payload FROM ownership").get() as { payload: string }).payload), payload, row.name);
        oracle.run("UPDATE ownership SET location = 'empty', waiter = 0");
        finalize();
        assert.equal((oracle.query("SELECT admission FROM ownership").get() as { admission: number }).admission === 1, row.admissionDuringPublication, row.name);
        const retirement = row.publication === "after-finalizer-check" || row.publication === "refused-finalizer";
        const submissions = retirement ? oracle.run("UPDATE ownership SET active = 2 WHERE active = 1").changes : 0;
        assert.equal(submissions, row.retirementSubmissions, row.name);
        oracle.run("UPDATE ownership SET active = 0 WHERE active IN (1, 2)");
        finalize();
        assert.equal(oracle.run("UPDATE ownership SET active = 2 WHERE active = 0").changes, row.lateSubmissions, row.name);
        const final = oracle.query("SELECT admission = 0 AND releases = 1 AS empty FROM ownership").get() as { empty: number };
        assert.equal(final.empty === 1, row.terminalEmpty, row.name);
      }
      oracle.run("CREATE TABLE recovery (generation INTEGER, checked_out INTEGER, admission INTEGER, deliveries INTEGER, payload TEXT)");
      for (const row of fixture.recovery) {
        oracle.run("DELETE FROM recovery");
        const payload = row.outcome === "pages" ? fixture.root : { key: fixture.root.key, error: "fixture-root-fault" };
        oracle.run("INSERT INTO recovery VALUES (1, 0, 1, 0, ?1)", [JSON.stringify(payload)]);
        if (row.path === "retry") {
          oracle.run("UPDATE recovery SET generation = generation + 1");
          assert.equal(oracle.run("UPDATE recovery SET deliveries = deliveries + 1 WHERE generation = 1").changes, row.staleRetrySubmissions, row.name);
        }
        if (row.path === "terminal-result" || row.path === "spent-work") {
          oracle.run("UPDATE recovery SET checked_out = 1");
          assert.equal(oracle.run("UPDATE recovery SET admission = 0 WHERE checked_out = 0").changes === 0, row.checkoutBlocksRetirement, row.name);
          oracle.run("UPDATE recovery SET checked_out = 0");
        } else {
          assert.equal(row.checkoutBlocksRetirement, false, row.name);
        }
        oracle.run("UPDATE recovery SET deliveries = deliveries + 1, admission = 0 WHERE checked_out = 0");
        const actual = oracle.query("SELECT deliveries, admission, payload FROM recovery").get() as { deliveries: number; admission: number; payload: string };
        assert.equal(actual.deliveries, 1, row.name);
        assert.deepEqual(JSON.parse(actual.payload), payload, row.name);
        assert.equal(actual.admission === 0, row.terminalEmpty, row.name);
      }
    } finally {
      oracle.close();
    }
    console.log("database-catalog-read-ownership-check: AJV=1 sqlite-transfers=" + fixture.transfers.length + " sqlite-completion=" + fixture.completion.length + " sqlite-recovery=" + fixture.recovery.length);
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [{
        package: "semio-framework-os-kernel-db",
        target: { kind: "lib", name: "db" },
        cargoArgs: ["--all-features"],
        laws: [
          "database_catalog_read_paused_transfers_exclude_successors_and_public_cleanup",
          "database_catalog_read_consumed_publication_preserves_exact_root_and_retires",
          "database_catalog_read_retry_and_terminal_resume_preserve_exact_root",
          "database_catalog_read_fixed_cap_plus_one_and_generation_aba",
          "database_catalog_read_success_returns_exact_storage_key_and_root",
          "database_catalog_read_controlled_wakes_coalesce_and_terminal_never_repolls",
          "database_catalog_read_publication_between_check_and_waker_registration_is_observed",
          "database_catalog_read_rejected_mount_retires_storage_and_key_on_distinct_grants",
          "database_catalog_read_cancel_stale_and_rejection_preserve_exact_storage_key",
          "database_catalog_read_terminal_result_drop_hands_back_exact_result",
        ],
      }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log("database-catalog-read-ownership-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);
      },
    });
    for (const receipt of receipts) console.log("database-catalog-read-ownership-native-receipt: " + JSON.stringify(receipt));
  }
}

/** 📬️ Verifies retained capability completion handoff at every public waiter boundary. */
class DatabaseCapabilityCompletionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-capability-completion-check accepts only --native");
    const root = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧫️fixtures/📬️capability-completion");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.engine", "CapabilityCompletionV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(
      fixture.publication.map((row: { name: string }) => row.name),
      ["before-poll", "before-registration", "after-pending"].flatMap((stage) => ["success", "fault"].map((outcome) => stage + "-" + outcome)),
    );
    assert.deepEqual(
      fixture.retirement.map((row: { name: string }) => row.name),
      ["consumed-before-wake-success", "consumed-before-wake-fault"],
    );
    assert.deepEqual(fixture.driveOwnership.map((row: { phase: string }) => row.phase), ["Handoff", "RetainWork", "Poll"]);
    assert.deepEqual(fixture.leaseCompletion.map((row: { publication: string }) => row.publication), ["synchronous", "active-publisher", "after-finalizer-check", "after-finalizer-check-refused"]);
    const { Database } = await import("bun:sqlite");
    const oracle = new Database(":memory:");
    try {
      oracle.run("CREATE TABLE handoff (id INTEGER PRIMARY KEY, completion TEXT, waiter INTEGER NOT NULL, wakes INTEGER NOT NULL, admission INTEGER NOT NULL)");
      const snapshot = () => oracle.query("SELECT completion, waiter, wakes, admission FROM handoff WHERE id = 1").get() as { completion: string | null; waiter: number; wakes: number; admission: number };
      const reset = () => {
        oracle.run("DELETE FROM handoff");
        oracle.run("INSERT INTO handoff VALUES (1, NULL, 0, 0, 1)");
      };
      const publish = (outcome: string) => oracle.run("UPDATE handoff SET completion = ?1, wakes = wakes + waiter, waiter = 0 WHERE id = 1", [outcome]);
      const consume = () => oracle.run("UPDATE handoff SET completion = NULL, waiter = 0, admission = 0 WHERE id = 1 AND completion IS NOT NULL");
      for (const row of fixture.publication) {
        reset();
        if (row.publication === "before-poll") publish(row.outcome);
        let firstReady = snapshot().completion !== null;
        if (!firstReady) {
          if (row.publication === "before-registration") publish(row.outcome);
          oracle.run("UPDATE handoff SET waiter = 1 WHERE id = 1");
          firstReady = snapshot().completion !== null;
        }
        assert.equal(firstReady, row.firstReady, row.name);
        if (!firstReady) publish(row.outcome);
        assert.equal(snapshot().completion, row.outcome, row.name);
        consume();
        assert.deepEqual(snapshot(), { completion: null, waiter: 0, wakes: row.wakes, admission: 0 }, row.name);
      }
      for (const row of fixture.retirement) {
        reset();
        oracle.run("UPDATE handoff SET waiter = 1, completion = ?1 WHERE id = 1", [row.outcome]);
        consume();
        assert.equal(snapshot().admission === 0, row.terminalBeforePublisherWake, row.name);
        oracle.run("UPDATE handoff SET wakes = wakes + waiter, waiter = 0 WHERE id = 1");
        assert.equal(snapshot().wakes, row.wakes, row.name);
      }
      oracle.run("CREATE TABLE ownership (active INTEGER NOT NULL, location TEXT NOT NULL, admission INTEGER NOT NULL, releases INTEGER NOT NULL)");
      for (const row of fixture.driveOwnership) {
        oracle.run("DELETE FROM ownership");
        oracle.run("INSERT INTO ownership VALUES (1, 'stack', 1, 0)");
        const close = () => oracle.run("UPDATE ownership SET admission = 0, releases = releases + 1 WHERE active = 0 AND location = 'empty' AND admission = 1").changes;
        assert.equal(close() === 0, row.closeBlocked, row.name);
        const paused = oracle.query("SELECT admission, location FROM ownership").get() as { admission: number; location: string };
        assert.equal(paused.admission === 1, row.admissionRetained, row.name);
        assert.equal(paused.location === "stack", row.storageRetained, row.name);
        oracle.run("UPDATE ownership SET location = 'terminal', active = 0");
        assert.equal(close(), 0, row.name);
        oracle.run("UPDATE ownership SET location = 'empty'");
        assert.equal(close(), 1, row.name);
        assert.equal(close(), 0, row.name);
        assert.equal((oracle.query("SELECT releases FROM ownership").get() as { releases: number }).releases, row.finalReleases, row.name);
      }
      for (const row of fixture.leaseCompletion) {
        oracle.run("DELETE FROM ownership");
        oracle.run("INSERT INTO ownership VALUES (?1, 'completion', 1, 0)", [row.publication === "synchronous" ? 0 : 1]);
        const checkedBeforeConsumption = row.publication.startsWith("after-finalizer-check");
        const readyAtFirstCheck = (oracle.query("SELECT location = 'empty' AS ready FROM ownership").get() as { ready: number }).ready === 1;
        oracle.run("UPDATE ownership SET location = 'empty'");
        const finalize = () => oracle.run("UPDATE ownership SET active = 4, admission = 0, releases = releases + 1 WHERE location = 'empty' AND active = 0 AND admission = 1");
        finalize();
        assert.equal((oracle.query("SELECT admission FROM ownership").get() as { admission: number }).admission === 1, row.admissionDuringPublication, row.name);
        let retirementSubmissions = 0;
        if (checkedBeforeConsumption && !readyAtFirstCheck) {
          oracle.run("UPDATE ownership SET active = active | 2 WHERE active = 1");
          oracle.run("UPDATE ownership SET active = active & ~1 WHERE active = 3");
          retirementSubmissions = (oracle.query("SELECT active = 2 AS queued FROM ownership").get() as { queued: number }).queued;
          oracle.run("UPDATE ownership SET active = 0 WHERE active = 2");
        } else {
          oracle.run("UPDATE ownership SET active = 0 WHERE active = 1");
        }
        assert.equal(retirementSubmissions, row.retirementSubmissions, row.name);
        finalize();
        assert.equal(oracle.run("UPDATE ownership SET active = 2 WHERE active = 0").changes, row.lateSubmissions, row.name);
        assert.equal((oracle.query("SELECT releases FROM ownership").get() as { releases: number }).releases, 1, row.name);
      }
    } finally {
      oracle.close();
    }
    console.log("database-capability-completion-check: AJV=1 sqlite-publication=" + fixture.publication.length + " sqlite-retirement=" + fixture.retirement.length + " sqlite-drive-ownership=" + fixture.driveOwnership.length + " sqlite-lease-completion=" + fixture.leaseCompletion.length);
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [
        {
          package: "semio-framework-os-kernel-db",
          target: { kind: "lib", name: "db" },
          cargoArgs: ["--all-features"],
          laws: [
            "database_capability_open_paused_transfer_blocks_public_close",
            "database_capability_open_lease_successors_and_active_publication_retire_once",
            "database_capability_open_completion_interleavings_preserve_result_and_wake",
            "database_capability_open_consumed_completion_retires_before_publisher_wake",
            "database_capability_open_fixed_admission_cap_plus_one_and_generation_aba",
            "database_capability_open_success_returns_exact_storage_owner_and_scalar",
            "database_capability_open_cancel_and_stale_generation_retain_exact_owner_for_public_close",
            "database_capability_open_saturation_and_shutdown_keep_retry_job_and_public_terminal",
            "database_capability_open_poll_publication_precedes_wake_rearm_at_every_boundary",
            "database_capability_open_post_ready_cancel_and_stale_retain_public_exact_result",
            "database_capability_open_rejection_take_retry_and_close_preserve_exact_storage",
            "database_capability_open_terminal_result_take_resume_and_checked_out_drop_handback",
            "database_capability_open_retry_contention_is_one_compare_exchange_per_callback",
            "database_catalog_read_publication_between_check_and_waker_registration_is_observed",
            "database_catalog_bootstrap_publication_race_and_queue_pressure_keep_exact_successor",
            "database_create_catalog_publication_check_register_recheck_has_no_lost_wake",
            "open_at_creates_a_fresh_zero_touch_database_with_an_empty_catalog",
          ],
        },
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log("database-capability-completion-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);
      },
    });
    for (const receipt of receipts) console.log("database-capability-completion-native-receipt: " + JSON.stringify(receipt));
  }
}

/** 🔐️ Checks fixed writer capabilities and retained local backend integration. */
class WalWriterAuthorityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-writer-authority-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.storage.writer", "WriterAuthorityV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    const writerDeferredWake = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔔️deferred-wake/🔣️.json"), "utf8"));
    const writerDeferredWakeSchema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔔️deferred-wake/🔣️.json"), "utf8"));
    const validateWriterDeferredWake = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(writerDeferredWakeSchema) as SchemaCheck;
    assert(validateWriterDeferredWake(writerDeferredWake), JSON.stringify(validateWriterDeferredWake.errors));
    const neutralDeferredWake = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🔨️modules/⏳️async/🔔️deferred-wake/🧫️fixtures/🔣️.json"), "utf8"));
    const neutralDeferredWakeSchema = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🔨️modules/⏳️async/🔔️deferred-wake/🧬️schema/🔣️.json"), "utf8"));
    const validateNeutralDeferredWake = semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(neutralDeferredWakeSchema).getSchema(`${neutralDeferredWakeSchema.$id}#/$defs/DeferredWakeFixture`) as SchemaCheck;
    assert(validateNeutralDeferredWake(neutralDeferredWake), JSON.stringify(validateNeutralDeferredWake.errors));
    assert.equal(neutralDeferredWake.capacity.partitions, writerDeferredWake.capacity.backendControls);
    assert.equal(neutralDeferredWake.capacity.slotsPerPartition, writerDeferredWake.capacity.writersPerBackend);
    assert.equal(neutralDeferredWake.capacity.totalWaiters, writerDeferredWake.capacity.backendControls * writerDeferredWake.capacity.writersPerBackend * writerDeferredWake.capacity.waitersPerWriter);
    {
    let queuedOwner = true;
    let faulted = true;
    let retryEpoch = 0;
    let queuedOwners = 1;
    let maximumQueuedOwners = queuedOwners;
    assert.equal(faulted && queuedOwner ? "pending" : "ready", writerDeferredWake.retryEpoch.hostileTrace[2]);
    assert.equal(writerDeferredWake.retryEpoch.readyBeforeOldSlotDrains, false);
    queuedOwner = false;
    assert.equal(faulted && queuedOwner ? "pending" : "fault-ready", writerDeferredWake.retryEpoch.hostileTrace[4]);
    faulted = false;
    retryEpoch += 1;
    queuedOwner = true;
    queuedOwners = Number(queuedOwner);
    maximumQueuedOwners = Math.max(maximumQueuedOwners, queuedOwners);
    assert.equal(retryEpoch, 1);
    assert.equal(maximumQueuedOwners, writerDeferredWake.retryEpoch.maximumQueuedOwnersPerSignal);
    assert.equal(writerDeferredWake.retryEpoch.admission, "fault-ready-after-exact-slot-drain");
    }
    for (const row of writerDeferredWake.cases) {
      assert.equal(row.expected.retainedFaults, row.activeRequested, row.id);
      assert.equal(row.expected.retainedGuards, row.activeRequested, row.id);
      assert.equal(row.expected.terminalEpochs, 0, row.id);
    }
    const writerDeferredStorageSource = readFileSync(join(owner, "..", "🦀️.rs"), "utf8");
    const writerDeferredSource = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const writerDeferredReleaseSource = readFileSync(join(owner, "🔔️release/🦀️.rs"), "utf8");
    assert(writerDeferredStorageSource.includes(`const DB_IO_BACKEND_CONTROLS: usize = ${writerDeferredWake.capacity.backendControls};`));
    assert(writerDeferredSource.includes(`const WAL_WRITER_CAPACITY: usize = ${writerDeferredWake.capacity.writersPerBackend};`));
    assert(writerDeferredReleaseSource.includes("fn request_controller(") && writerDeferredReleaseSource.includes("fn notify_faults("));
    const missingWriter = writerDeferredWake.runtimeMarkers.writer.filter((marker: string) => !writerDeferredReleaseSource.includes(marker));
    assert.deepEqual(missingWriter, [], `missing writer runtime markers: ${missingWriter.join(", ")}`);
    console.log(`[DEBUG] writer-deferred-wake-independent-oracle: AJV=1 cases=${writerDeferredWake.cases.length} capacities=64*32*1 runtime-markers=${writerDeferredWake.runtimeMarkers.writer.length}`);
    const remoteOwner = join(owner, "🧫️fixtures/🌐️remote-guard");
    const remoteFixture = JSON.parse(readFileSync(join(remoteOwner, "🔣️.json"), "utf8"));
    const validateRemote = ownedExport(this.repoRoot, "db.storage.writer", "WalWriterFenceV1");
    assert(validateRemote(remoteFixture), JSON.stringify(validateRemote.errors));
    assert.deepEqual(remoteFixture.mutations, ["create", "append", "sync", "seal", "truncateTail", "delete"]);
    for (const row of remoteFixture.postgres.lockKeys) {
      const digest = createHash("sha256").update(Buffer.concat([Buffer.from(remoteFixture.postgres.lockNamespace, "utf8"), Buffer.from([0]), Buffer.from(row.document, "utf8")])).digest();
      assert.equal(digest.readBigInt64BE(0).toString(), row.key, `postgres advisory-lock key oracle for ${row.document}`);
    }
    assert(remoteFixture.neo4j.renewEveryMs * 2 < remoteFixture.neo4j.leaseTtlMs, "a Neo4j writer lease must survive one missed renewal");
    for (const backend of ["sqlite", "postgres", "neo4j"]) assert(remoteFixture.laws.filter((law: { backends: string[] }) => law.backends.includes(backend)).length >= 4, `${backend} runs every shared fence law`);
    const sourceOf = (path: string) => readFileSync(join(owner, "..", path, "🦀️.rs"), "utf8");
    for (const [marker, path] of [
      ["pg_try_advisory_lock($1)", "🐘️postgres"],
      ["fn fenced_wal_mutation", "🐘️postgres"],
      [`"${remoteFixture.postgres.lockNamespace}"`, "🐘️postgres"],
      ["CYPHER_WAL_WRITER_FENCE", "🌐️neo4j"],
      ["fn fenced_wal_mutation", "🌐️neo4j"],
      [`WAL_WRITER_LEASE_TTL_MS: i64 = ${remoteFixture.neo4j.leaseTtlMs.toLocaleString("en-US").replaceAll(",", "_")}`, "🌐️neo4j"],
    ] as const)
      assert(sourceOf(path).includes(marker), `missing cross-process writer fence primitive ${marker} in ${path}`);
    console.log(`wal-writer-fence-oracle: AJV=1 lockKeys=${remoteFixture.postgres.lockKeys.length} laws=${remoteFixture.laws.length}`);
    const memoryOwner = join(owner, "..", "🧫️fixtures", "🧮️memory-backing");
    const memoryFixture = JSON.parse(readFileSync(join(memoryOwner, "🔣️.json"), "utf8"));
    const validateMemory = ownedExport(this.repoRoot, "db.storage", "MemoryBackingV1");
    assert(validateMemory(memoryFixture), JSON.stringify(validateMemory.errors));
    const poolUseOwner = join(owner, "..", "🧫️fixtures", "🔐️backend-pool-use");
    const poolUseFixture = JSON.parse(readFileSync(join(poolUseOwner, "🔣️.json"), "utf8"));
    const validatePoolUse = ownedExport(this.repoRoot, "db.storage", "BackendPoolUseV1");
    assert(validatePoolUse(poolUseFixture), JSON.stringify(validatePoolUse.errors));
    assert.deepEqual(
      poolUseFixture.cases.map((row: { name: string }) => row.name),
      [
        "registered-backend-blocks-shutdown",
        "task-derives-registered-pool",
        "dropped-facade-retains-use",
        "compaction-acquires-before-admission",
        "forged-kind-rejected-before-page-admission",
        "rollback-reserved-before-owner-transfer",
        "all-retirement-tiers-full-return-exact-executor",
        "committed-rollback-retains-use-until-terminal",
        "prepared-post-transfer-refusal-returns-close-owner",
      ],
    );
    const { Database } = await import("bun:sqlite");
    const openingOracle = new Database(":memory:");
    try {
      openingOracle.run("CREATE TABLE opening_owner (backend TEXT PRIMARY KEY, retained INTEGER NOT NULL, close_requested INTEGER NOT NULL)");
      for (const row of poolUseFixture.opening) {
        openingOracle.run("INSERT INTO opening_owner VALUES (?1, 1, 0)", [row.backend]);
        openingOracle.run("UPDATE opening_owner SET close_requested = 1 WHERE backend = ?1", [row.backend]);
        const requested = openingOracle.query("SELECT close_requested FROM opening_owner WHERE backend = ?1").get(row.backend) as { close_requested: number };
        assert.equal(!!requested.close_requested, row.closeRequestedBeforeRetry);
        openingOracle.run("DELETE FROM opening_owner WHERE backend = ?1 AND close_requested = 1", [row.backend]);
        const terminal = openingOracle.query("SELECT count(*) AS retained FROM opening_owner").get() as { retained: number };
        assert.equal(terminal.retained === 0, row.ledgerBaseline);
        assert.equal(terminal.retained === 0, row.poolShutdownAfterDrain);
        assert.equal(terminal.retained === 0, row.reopens);
      }
    } finally {
      openingOracle.close();
    }
    assert.deepEqual(memoryFixture.writerTable, { slots: fixture.capacity, separateBox: true });
    assert.deepEqual(memoryFixture.controllerCredit, { items: 1, controls: 1, bytesFormula: "wake-plus-two-usize" });
    assert.equal(memoryFixture.retainedPageResults, 44);
    assert.equal(memoryFixture.sameSlotReuseBeforeOldResultClose, true);
    assert.equal(memoryFixture.taskSlotReleasedWhileResultRetained, true);
    assert.equal(memoryFixture.droppedResultRetirement, "mounted-io-maintenance");
    const directoryOwner = join(owner, "..", "🧫️fixtures", "📁️directory-durability");
    const directoryFixture = JSON.parse(readFileSync(join(directoryOwner, "🔣️.json"), "utf8"));
    const validateDirectory = ownedExport(this.repoRoot, "db.storage", "DirectoryDurabilityV1");
    assert(validateDirectory(directoryFixture), JSON.stringify(validateDirectory.errors));
    const openOwner = join(owner, "..", "..", "📝️wal", "🧫️fixtures", "🚪️open-rejection");
    const openFixture = JSON.parse(readFileSync(join(openOwner, "🔣️.json"), "utf8"));
    const validateOpen = ownedExport(this.repoRoot, "db.wal", "OpenRejectionV1");
    assert(validateOpen(openFixture), JSON.stringify(validateOpen.errors));
    let openOwnerState = "exact-writer-permit";
    assert.equal(openFixture.acquisition[1].owner, openOwnerState);
    openOwnerState = "exact-writer-release";
    assert.equal(openFixture.acquisition[2].owner, openOwnerState);
    assert.equal(openFixture.acquisition[2].releaseActivation, "first-explicit-close-poll");
    const exactRelease = Symbol("exact-writer-release");
    let retainedRelease: symbol | undefined = exactRelease;
    for (const [index, close] of openFixture.explicitClose.entries()) {
      assert.equal(close.attempt, index + 1);
      assert.equal(close.openCause, "original-open-error");
      if (close.outcome === "fault") assert.equal(retainedRelease, exactRelease);
      else retainedRelease = undefined;
      assert.equal(close.sameDocumentAcquire, retainedRelease ? "conflict" : "available");
    }
    assert.equal(openFixture.droppedRejection.behavior === "transfer" && !openFixture.droppedRejection.panics ? "backend-release-cell" : "discarded", openFixture.droppedRejection.location);
    const directoryNames = new Set(["segment", "marker"]);
    directoryNames.delete("segment");
    assert.equal(!directoryNames.has("segment") && directoryNames.has("marker") ? "not-found-with-retained-marker" : "invalid", directoryFixture.deleteFaultState);
    directoryNames.delete("marker");
    assert.equal(directoryNames.size, 0);
    assert(directoryFixture.delete.indexOf("segment-delete-parent-synced") < directoryFixture.delete.indexOf("marker-deleted"));
    const pinnedWriter = { operation: fixture.controllerBinding.operation, releaseRequested: true };
    assert.equal(pinnedWriter.releaseRequested && pinnedWriter.operation !== fixture.controllerBinding.contender ? "closed" : "ok", fixture.controllerBinding.fenceBeforeCallback);
    assert.equal(pinnedWriter.operation === fixture.controllerBinding.operation ? "ok" : "closed", fixture.controllerBinding.pinnedResume);
    assert.equal(fixture.controllerBinding.tasksAddedOnRelease, 0);
    assert.equal(fixture.controllerBinding.guardFactoryCallsOnConflict, 0);
    assert.equal(fixture.controllerBinding.faultRetryPreservesKey, true);
    const pendingWriters = new Map([
      ["fault", "retained"],
      ["healthy", "retained"],
    ]);
    const faultTrace = ["fault-retained"];
    pendingWriters.delete("healthy");
    faultTrace.push("healthy-terminal");
    assert.equal(pendingWriters.get("fault"), "retained");
    pendingWriters.delete("fault");
    faultTrace.push("fault-retry-terminal");
    assert.deepEqual(faultTrace, fixture.controllerFaults.coalesced);
    assert.equal(fixture.controllerFaults.outerPanic.executorTurns, 1);
    assert.equal(fixture.controllerFaults.outerPanic.wakesPerOwner, 1);
    const followerOwners = new Set(["document"]);
    assert.equal(followerOwners.has("document") ? "conflict" : "ok", fixture.replication.occupiedFollower);
    assert.deepEqual(fixture.replication.inventoryAfterConflict, []);
    const transfer = fixture.replication.snapshotTransfer;
    const sourceSnapshot = Buffer.concat(Array.from({ length: transfer.repetitions }, () => Buffer.from(transfer.pattern)));
    const copiedSnapshot = Buffer.from(sourceSnapshot);
    assert.equal(sourceSnapshot.length, transfer.bytes);
    sourceSnapshot.fill(0);
    assert.deepEqual(
      [...copiedSnapshot],
      Array.from({ length: transfer.bytes }, (_, index) => transfer.pattern[index % transfer.pattern.length]),
    );
    assert.equal(Symbol("source-result") === Symbol("independent-input"), transfer.sameOperation);
    const clusterSource = readFileSync(join(owner, "..", "..", "🌐️cluster", "🦀️.rs"), "utf8");
    assert(clusterSource.includes("db_io_copy_page_owner(&pages)") && clusterSource.includes("close_replication_pages(&mut pages)"), "snapshot replication must retire source result before a distinct write owner");
    const walSource = readFileSync(join(owner, "..", "..", "📝️wal", "🦀️.rs"), "utf8");
    for (const marker of ["struct ArtifactWalAcquiredRejected", "enum ArtifactWalOpenRejected", "retry_open(", "retry_close(", "open_acquired(", "into_open_rejected"])
      assert(walSource.includes(marker), "missing retained WAL-open owner primitive: " + marker);
    assert(!walSource.includes("release_failed_open"), "WAL open rejection must not await and flatten its writer release");
    const artifactSource = readFileSync(join(owner, "..", "..", "🗿️artifact", "🦀️.rs"), "utf8");
    for (const marker of ["enum ArtifactEngineOpenRejected", "RetainedWal", "has_retained_writer", "Future<Output = Result<Box<ArtifactEngine>, ArtifactEngineOpenRejected>>"])
      assert(artifactSource.includes(marker), "missing engine retained-open propagation: " + marker);
    const engineSource = readFileSync(join(owner, "..", "..", "⚙️engine", "🦀️.rs"), "utf8");
    for (const marker of ["enum DatabaseDocumentOpenRejected", "Result<ArtifactHandle, DatabaseDocumentOpenRejected>", "rejected.retry_close().await"]) assert(engineSource.includes(marker), "missing database retained-open propagation: " + marker);
    for (const marker of ["enum ReplicationRejected", "ArtifactWal::open_acquired", "ReplicationRejected::WalOpen"]) assert(clusterSource.includes(marker), "missing cluster acquired-writer propagation: " + marker);
    for (const outcome of fixture.replication.releaseAfter) {
      const owner = new Set(["document"]);
      try {
        assert(["tail", "up-to-date", "snapshot", "leader-corrupt"].includes(outcome));
      } finally {
        owner.delete("document");
      }
      assert.equal(owner.size, 0);
    }
    for (const row of fixture.cases) {
      let generation = BigInt(row.firstGeneration);
      const live = new Map<string, bigint>();
      const owners = new Map<string, { document: string; generation: bigint }>();
      for (const step of row.steps) {
        let actual = "ok";
        const permit = owners.get(step.owner);
        if (step.action === "acquire") {
          if (live.has(step.document)) actual = "conflict";
          else if (generation === 0xffffffffffffffffn) actual = "exhausted";
          else {
            live.set(step.document, generation);
            owners.set(step.owner, { document: step.document, generation: generation++ });
          }
        } else if (step.backend !== 0 || permit?.document !== step.document || live.get(step.document) !== permit?.generation) actual = "fenced";
        else if (step.action === "release") live.delete(step.document);
        assert.equal(actual, step.expected, `${row.name}: ${JSON.stringify(step)}`);
      }
      assert.equal(live.size, 0, row.name);
    }
    assert.deepEqual(
      fixture.resultRetirement.terminal,
      Array.from({ length: fixture.resultRetirement.pages + 3 }, (_, index) => index === fixture.resultRetirement.pages + 2),
    );
    assert.deepEqual(
      fixture.guardRetirement.terminal,
      fixture.guardRetirement.stages.map((stage: string) => stage === "terminal"),
    );
    const rejected = new Set(Array.from({ length: fixture.backendPressure.capacity }, (_, index) => index));
    let retained = rejected.size === fixture.backendPressure.capacity;
    assert.equal(retained, fixture.backendPressure.retainedWhenFull);
    rejected.delete(0);
    if (rejected.size < fixture.backendPressure.capacity) retained = false;
    assert.equal(!retained, fixture.backendPressure.terminalAfterCapacityReturns);
    const closing = new Map([
      [fixture.fairRetirement.pinnedSlot, "pinned"],
      [fixture.fairRetirement.releasingSlot, "releasing"],
    ]);
    for (let cursor = 0; cursor < fixture.fairRetirement.maximumOpportunities; cursor++) if (closing.get(cursor) === "releasing") closing.delete(cursor);
    assert.equal(closing.has(fixture.fairRetirement.pinnedSlot), fixture.fairRetirement.pinnedRetained);
    assert.equal(!closing.has(fixture.fairRetirement.releasingSlot), fixture.fairRetirement.releasingRetired);
    for (const trace of [fixture.maintenanceFairness.continuouslyReady, fixture.maintenanceFairness.firstClassFaults])
      assert.deepEqual(
        trace,
        trace.map((_: unknown, index: number) => index % fixture.maintenanceFairness.classes.length),
      );
    let terminalEpoch = BigInt(fixture.releaseSignal.firstEpoch);
    const waits: bigint[] = [];
    for (const writer of fixture.releaseSignal.writers) {
      const required = terminalEpoch + 1n;
      assert.equal(terminalEpoch >= required, fixture.releaseSignal.requestCompletesRelease);
      terminalEpoch += 1n;
      waits.push(required);
      assert.equal(terminalEpoch.toString(), writer.terminalEpoch);
      assert.equal(
        waits.every((wait) => terminalEpoch >= wait),
        fixture.releaseSignal.oldWaitSurvivesReuse,
      );
    }
    console.log(
      `wal-writer-authority-independent-oracle: AJV=6 exact-u64=1 cases=${fixture.cases.length} mutations=${fixture.mutations.length} fence-laws=${remoteFixture.laws.length} writer-slots=${memoryFixture.writerTable.slots} retained-result=1 directory-barriers=4 wal-open-owner=1 backend-pool-use=${poolUseFixture.cases.length} physical-opening=${poolUseFixture.opening.length}`,
    );
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    for (const marker of ["struct WalWriterPermit", "struct WalWriterTable", "struct WalFileWriterGuard", "try_lock()", "checked_add(1)", "active_operation", "fn release_step"])
      assert(source.includes(marker), `missing writer capability primitive: ${marker}`);
    const explicitRelease = source.slice(source.indexOf("pub fn release(mut self)"), source.indexOf("fn request_release(&self)"));
    assert(!explicitRelease.includes("self.request_release()"), "explicit retained release must stay dormant until its first poll");
    const storageSource = readFileSync(join(owner, "..", "🦀️.rs"), "utf8");
    for (const marker of ["WalWriterTable<WalFileWriterGuard>", "fn writer_sidecar", "DbIoTask::WalWriterAcquire", "fn pin_writer_operation", "finish_operation_if_pinned", ".semio-wal-writer"])
      assert(storageSource.includes(marker), `missing filesystem writer integration: ${marker}`);
    for (const marker of [
      "pool_use: Option<Arc<WorkerPoolUse>>",
      "fn db_io_backend_admit_operation",
      "Result<Arc<WorkerPool>, DbError>",
      "pub fn submit_db_io_task(task: DbIoTask)",
      "db_io_backend_control(owner.kind, slot, generation) != control",
      "struct DbIoBackendRollbackReservation",
      "struct DbIoBackendRegistrationRejected",
      "pub enum DbStorageOpenRejected",
      "pub async fn retry_close(self) -> Result<DbError, Self>",
      "register_db_io_backend_prepared_with_use",
      "Result<DbIoBackendControl, DbIoBackendRegistrationRejected>",
      "reserved: bool",
    ])
      assert(storageSource.includes(marker), `missing backend-owned WorkerPool use boundary: ${marker}`);
    assert(!storageSource.includes("pub fn submit_db_io_task(pool:"), "DB I/O tasks must derive the exact registered backend pool");
    const registrationSource = storageSource.slice(storageSource.indexOf("pub fn register_db_io_backend("), storageSource.indexOf("fn db_io_writer_release_lane_step"));
    assert(!registrationSource.includes("let _ = db_io_park_lost_owner"), "backend registration must not discard a saturated retirement owner");
    for (const marker of ["MemoryDbIoExecutor::backing_bytes()", "checked_add(writer::release::controller_credit())"]) assert(storageSource.includes(marker), `missing memory writer backing integration: ${marker}`);
    const sqliteSource = readFileSync(join(owner, "..", "🪶️sqlite", "🦀️.rs"), "utf8");
    for (const marker of ["WalWriterTable<SqliteWalWriterGuard>", "canonical_database", "fn physical_writer_sidecar", "DbIoTask::WalWriterAcquire", "fn pin_writer_operation", "finish_operation_if_pinned", ".semio-wal-writer"])
      assert(sqliteSource.includes(marker), `missing SQLite writer integration: ${marker}`);
    const releaseSource = readFileSync(join(owner, "🔔️release/🦀️.rs"), "utf8");
    for (const marker of [
      "struct WalWriterSignalCell",
      "requested: bool",
      "fn request_release(&mut self)",
      "impl Drop for WalWriterRelease",
      "deferred_fault_waiter",
      "defer_fault_notifications",
      "suspend_controller_for_refusal",
      "deferred_wake_pending_for_test",
    ])
      assert(releaseSource.includes(marker), `missing bounded deferred refusal primitive: ${marker}`);
    assert.equal((storageSource.match(/writer::release::notify_faults/g) ?? []).length, 0, "public DB handback paths must not invoke writer wakers directly");
    const retainedFixtureLaws = readFileSync(join(owner, "..", "🧪️tests", "🔬️db-io-retained-fixtures", "🦀️.rs"), "utf8");
    assert(retainedFixtureLaws.includes("fn wal_writer_mounted_stale_controller_defers_cross_key_wake_and_fences_retry_epoch()"), "missing mounted stale-controller refusal law");
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [
        {
          package: "semio-framework-os-kernel-db",
          target: { kind: "lib", name: "db" },
          cargoArgs: ["--all-features"],
          laws: [
            "db_io_real_storage_open_drop_retires_queued_backend_and_allows_reopen",
            "db_io_real_storage_open_fault_drop_retires_registered_backend_without_retry",
            "wal_writer_table_matches_neutral_exact_scope_and_aba_rejection",
            "wal_writer_table_capacity_recycles_slots_without_reusing_generations",
            "wal_writer_file_lock_excludes_independent_instances_and_processes",
            "db_io_lost_result_lease_retains_every_page_and_final_handback",
            "wal_writer_release_retains_pinned_operation_and_faulted_guard",
            "db_io_lost_backend_retains_exact_owner_under_rejected_registry_pressure",
            "wal_writer_table_close_advances_other_guards_while_first_operation_is_pinned",
            "db_io_maintenance_rotates_ready_and_faulted_classes_without_starvation",
            "wal_writer_release_signal_preserves_exact_waits_across_writer_and_backend_reuse",
            "wal_writer_mounted_controller_fences_at_signal_and_wakes_outside_registry_without_tasks",
            "wal_writer_mounted_controller_fault_returns_exact_retry_owner_without_poisoning_other_writer",
            "wal_writer_mounted_stale_controller_defers_cross_key_wake_and_fences_retry_epoch",
            "wal_writer_mounted_controller_rerequests_after_async_executor_handback",
            "wal_writer_mounted_controller_coalesced_fault_does_not_strand_healthy_release",
            "wal_writer_mounted_controller_outer_panic_faults_waiters_once_and_stops",
            "db_io_memory_backend_heap_tables_have_exact_preflight_credit_and_terminal_return",
            "db_io_retained_page_results_survive_same_task_slot_reuse_and_return_exact_credit",
            "fs_storage_canonical_alias_writer_fences_all_six_mutations",
            "sqlite_wal_writer_real_database_alias_and_crash_are_exclusive",
            "fs_wal_directory_barriers_match_neutral_order_and_duplicate_create_is_atomic",
            "fs_wal_directory_faults_retain_seal_and_delete_order_until_explicit_retry",
            "fs_replacement_reports_failure_until_renamed_parent_is_synced",
            "fs_wal_reopen_repairs_unacknowledged_segment_namespace_before_header_ack",
            "replicate_document_fences_occupied_follower_before_inventory_or_up_to_date",
            "replicate_document_releases_follower_after_leader_replay_failure",
            "replicate_document_applies_missing_tail_commands_to_a_fresh_follower",
            "replicate_document_reports_up_to_date_once_a_follower_catches_up",
            "replicate_document_transfers_a_snapshot_when_the_follower_is_below_the_retained_floor",
            "artifact_wal_open_rejection_retains_exact_writer_for_close_or_same_owner_retry",
            "artifact_engine_create_rejection_propagates_exact_wal_release_owner",
            "database_document_mount_failure_terminalizes_authority_builder_wal_owner_before_fanout",
            "db_io_registered_backend_use_blocks_pool_shutdown_until_terminal_close",
            "db_io_backend_registration_saturation_returns_exact_executor_before_pool_use",
            "db_io_prepared_registration_failure_returns_exact_close_owner_after_submission_refusal",
            "db_io_task_uses_registered_backend_pool_not_caller_pool",
            "db_io_backend_drop_retains_pool_until_deferred_close_terminal",
            "db_io_forged_backend_kind_is_rejected_before_task_page_admission",
            "database_compaction_future_acquires_pool_use_before_admission",
          ],
        },
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log(`wal-writer-authority-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    for (const receipt of receipts) console.log(`wal-writer-authority-native-receipt: ${JSON.stringify(receipt)}`);
  }
}

/** 🤺️ The cross-process WAL writer fence laws of `db_storage::writer::fence_conformance`, one lane per backend:
 * `sqlite` on a scratch file; `postgres` and `neo4j` on the ONE shared development server claimed by
 * `os-hub-ts backend run <postgres|neo4j> -- …` (its `OS_HUB_*` environment selects the server, `SEMIO_BACKEND_CLIENT` is the
 * server's own client the laws cross-check the lock through), so a server lane runs only under that claim.
 * `wal-writer-fence-live [sqlite|postgres|neo4j]…`: without a lane, `sqlite` plus every claimed lane whose environment
 * is present. */
const WAL_WRITER_FENCE_LANES: Readonly<Record<string, Readonly<{ law: string; claimed?: string }>>> = {
  sqlite: { law: "db_storage::writer::fence_conformance::sqlite_wal_writer_fence_holds_every_shared_law" },
  postgres: { law: "db_storage::writer::fence_conformance::postgres_wal_writer_fence_holds_every_shared_law", claimed: "OS_HUB_DATABASE_URL" },
  neo4j: { law: "db_storage::writer::fence_conformance::neo4j_wal_writer_fence_holds_every_shared_law", claimed: "OS_HUB_NEO4J_URI" },
};

class WalWriterFenceLiveScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const lanes = segments.length ? segments : Object.keys(WAL_WRITER_FENCE_LANES).filter((name) => !WAL_WRITER_FENCE_LANES[name]!.claimed || process.env[WAL_WRITER_FENCE_LANES[name]!.claimed!]);
    const unknown = lanes.filter((name) => !(name in WAL_WRITER_FENCE_LANES));
    if (unknown.length) throw new Error(`wal-writer-fence-live accepts ${Object.keys(WAL_WRITER_FENCE_LANES).join(" | ")}, got ${unknown.join(",")}`);
    const unclaimed = lanes.filter((name) => WAL_WRITER_FENCE_LANES[name]!.claimed && !process.env[WAL_WRITER_FENCE_LANES[name]!.claimed!]);
    if (unclaimed.length) throw new Error(`wal-writer-fence-live ${unclaimed.join(",")} needs the claimed shared server: run it under \`os-hub-ts backend run ${unclaimed[0]} -- …\``);
    for (const lane of lanes) {
      console.log(`wal-writer-fence-live ${lane}: ${WAL_WRITER_FENCE_LANES[lane]!.law}`);
      await runCargo(["test", "--manifest-path", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml"), "--features", "sqlite,postgres,neo4j", "--lib", "--", "--exact", WAL_WRITER_FENCE_LANES[lane]!.law, "--include-ignored", "--test-threads=1"], this.root);
    }
  }
}

/** 🐘️ `postgres-round-trips-live` — the PostgreSQL storage laws of `db_storage_postgres::round_trips` on the ONE shared
 * development server claimed by `os-hub-ts backend run postgres -- …` (a list is one statement, concurrent writers wait for
 * the backend's operation slot, concurrent openers of a fresh database all find its schema). */
class PostgresRoundTripsLiveScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("postgres-round-trips-live takes no arguments");
    if (!process.env.OS_HUB_DATABASE_URL) throw new Error("postgres-round-trips-live needs the claimed shared server: run it under `os-hub-ts backend run postgres -- …`");
    await runCargo(["test", "--manifest-path", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml"), "--features", "sqlite,postgres", "--lib", "--", "db_storage_postgres::round_trips::", "--include-ignored"], this.root);
  }
}

/** 🧾️ Proves the logical commit firewall with an independent neutral grammar evaluator. */
class WalCommittedTransactionsCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-committed-transactions-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🧾️committed-transactions/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.wal", "CommittedTransactionsV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(fixture.historyProjection.expected, { entries: 1, operationIds: fixture.historyProjection.commands.map((command: any) => command.id), headSeq: fixture.historyProjection.commands.length, commitSeq: 1 });
    assert.equal(new Set(fixture.historyProjection.expected.operationIds).size, fixture.historyProjection.commands.length);
    for (const row of fixture.historyProjection.rejections) {
      const frontiers = row.records.flatMap((record: any, index: number) => (record.kind === "frontier" ? [index] : []));
      const accepted = row.records.every((record: any) => record.document === "current") && frontiers.length === 1 && frontiers[0] === row.records.length - 1;
      assert.equal(accepted, row.accepted, row.name);
    }
    for (const row of fixture.cases) {
      let next = 1n;
      let observed = false;
      let recoverAbort: string | null = null;
      const transactions: { id: string; kinds: string[] }[] = [];
      let error: string | null = null;
      try {
        for (const [index, segment] of row.segments.entries()) {
          if (index !== row.segments.length - 1 && segment.state !== "sealed") throw "corrupt";
          assert.deepEqual(
            [...segment.physicalCommitsAfter].sort((a: number, b: number) => a - b),
            segment.physicalCommitsAfter,
          );
          const frames = segment.frames.flatMap((frame: any) => Array.from({ length: frame.repeat ?? 1 }, () => frame));
          assert.equal(segment.physicalCommitsAfter.at(-1), frames.length - 1);
          let current: { id: string; kinds: string[] } | null = null;
          for (const [ordinal, frame] of frames.entries()) {
            if (frame.kind === "header") {
              if (ordinal !== 0 || current !== null) throw "corrupt";
              continue;
            }
            if (ordinal === 0) throw "corrupt";
            if (frame.kind === "begin") {
              if (current !== null || BigInt(frame.id) < next || (observed && BigInt(frame.id) !== next)) throw "corrupt";
              const payload = Buffer.alloc(8);
              payload.writeBigUInt64LE(BigInt(frame.id));
              const id = payload.readBigUInt64LE();
              if (id === 0xffffffffffffffffn) throw "sequence";
              next = id + 1n;
              observed = true;
              current = { id: id.toString(), kinds: [] };
            } else if (frame.kind === "commit" || frame.kind === "abort") {
              if (current === null || current.id !== frame.id || (frame.kind === "commit" && current.kinds.length !== frame.count)) throw "corrupt";
              if (frame.kind === "commit") transactions.push(current);
              current = null;
            } else {
              if (current === null) throw "corrupt";
              if (current.kinds.length === fixture.maximumRecords) throw "capacity";
              current.kinds.push(frame.kind);
            }
          }
          if (current !== null) {
            if (segment.state !== "active" || index !== row.segments.length - 1) throw "corrupt";
            recoverAbort = current.id;
          }
        }
      } catch (caught) {
        if (!["corrupt", "capacity", "sequence"].includes(String(caught))) throw caught;
        error = String(caught);
      }
      assert.deepEqual({ accepted: error === null, transactions: error === null ? transactions : [], nextTxId: error === null ? next.toString() : null, recoverAbort: error === null ? recoverAbort : null, error }, row.expected, row.name);
    }
    console.log(`wal-committed-transactions-independent-oracle: AJV=1 u64=1 vectors=${fixture.cases.length}`);
    const faults = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🛑️fail-stop/🔣️.json"), "utf8"));
    const validateFaults = ownedExport(this.repoRoot, "db.wal", "FailStopV1");
    assert(validateFaults(faults), JSON.stringify(validateFaults.errors));
    assert.deepEqual(
      faults.cases.filter((row: any) => row.fault !== "successorAppendError").map((row: any) => [row.fault, row.expectedPhysicalSuffix]),
      [
        ["shortAppend", "torn"],
        ["appendError", "absent"],
        ["syncError", "complete"],
      ],
    );
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(source.includes("struct WalTransactionGate") && source.includes("frames: Vec<WalRecordFrame>") && source.includes("self.frames.len() >= WAL_TRANSACTION_RECORDS_MAX") && source.includes("count > WAL_TRANSACTION_RECORDS_MAX"), "logical admission must retain frame spans (not owned decoded records) up to the writer's own transaction bound");
    assert(source.includes("WalCommittedCursor") && source.includes("WalCommittedTransaction"), "materializers need one shared borrowed committed cursor");
    assert(source.includes("enum WalVerifiedFrameStep"), "verified replay must yield after each physical frame without repeating whole-frame CRC work");
    assert(source.includes("trait WalImmutableByteSource"), "History must share the authenticated frame source without borrowing another field across polls");
    assert(source.includes("struct WalAuthenticatedSource<S>") && source.includes("source: S"), "History authentication and committed spans must retain the same immutable source owner");
    assert(source.includes("recovered_abort_tx_id") && source.includes("wal recovery abort exceeds retained segment budget"), "active recovery must durably abort within the retained segment budget");
    const history = readFileSync(join(owner, "../🗿️artifact/🦀️.rs"), "utf8");
    assert(history.includes("WalAuthenticatedSource<HistoryPageSet>") && !history.includes("struct HistoryFrameCursor"), "History must consume authenticated committed spans, with no independent frame grammar");
    for (const check of ["history envelope document differs", "history frontier document differs", "history committed frontier is not terminal"]) assert(history.includes(check), `History admission is missing ${check}`);
    const decoder = JSON.parse(readFileSync(join(owner, "🧫️fixtures/📖️retained-decoder/🔣️.json"), "utf8"));
    const validateDecoder = ownedExport(this.repoRoot, "db.wal", "RetainedDecoderV1");
    assert(validateDecoder(decoder), JSON.stringify(validateDecoder.errors));
    const { default: leb } = await import("@webassemblyjs/leb128/lib/leb.js");
    for (const row of decoder.varints) {
      const bytes = Buffer.from(row.hex, "hex");
      let value: string | null = null;
      let consumed: number | null = null;
      const terminal = bytes.findIndex((byte) => byte < 128);
      if (terminal >= 0 && terminal < 10) {
        const number = bytes.subarray(0, terminal + 1).reduceRight((value, byte) => value * 128n + BigInt(byte & 127), 0n);
        if (number <= 0xffffffffffffffffn) {
          const storage = Buffer.alloc(8);
          storage.writeBigUInt64LE(number);
          if (Buffer.from(leb.encodeUIntBuffer(storage)).equals(bytes.subarray(0, terminal + 1))) {
            value = number.toString();
            consumed = terminal + 1;
          }
        }
      }
      assert.deepEqual({ value, consumed }, { value: row.value, consumed: row.consumed }, row.name);
    }
    assert(source.includes("fn wal_read_canonical_varint"), "retained readers must reject noncanonical and overflowing u64 fields");
    console.log(`wal-retained-decoder-independent-oracle: AJV=1 LEB128=1 vectors=${decoder.varints.length}`);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib", name: "db" },
            cargoArgs: ["--all-features"],
            laws: [
              "wal_transaction_gate_matches_neutral_committed_spans",
              "wal_retained_decoder_fuel_resumes_exact_fragmented_bytes",
              "wal_retained_decoder_cancel_close_preserves_source_and_returns_owner",
              "wal_committed_cursor_cancel_resume_keeps_transaction_position",
              "wal_committed_cursor_unfinished_borrow_poison_and_cancelled_close",
              "artifact_open_ignores_neutral_aborted_command_snapshot_and_cas",
              "sync_replay_ignores_neutral_aborted_command_snapshot_and_cas",
              "cli_verify_checks_neutral_logical_commit_boundaries",
              "open_replays_the_wal_and_reconstructs_state_and_frontier_identically",
              "wal_retained_varints_match_neutral_exact_u64_and_atomic_interruption",
              "wal_committed_cursor_single_fuel_and_expired_turns_match_neutral_transactions",
              "sync_retained_reads_resume_neutral_varints_without_renewing_overall_deadline",
              "wal_immutable_source_fragmentation_matches_neutral_transactions",
              "artifact_history_replay_uses_neutral_committed_inventory_and_retires_every_owner",
              "artifact_history_replay_projects_real_committed_batch_and_cancels_owned_sources",
              "artifact_history_and_opener_reject_neutral_inner_documents_and_frontier_order",
              "wal_recovery_aborts_only_incomplete_active_transactions_idempotently",
              "wal_recovery_abort_fsync_survives_two_independent_filesystem_reopens",
              "wal_recovery_abort_faults_retry_without_duplicate_abort",
              "wal_recovery_abort_cancellation_has_one_durable_boundary",
              "wal_recovery_abort_capacity_exact_and_plus_one_preserves_source",
              "artifact_history_panic_at_each_phase_transition_retains_then_fault_retires",
              "db_compact::tests::compaction_applies_only_committed_frontier_snapshot_and_payload_effects",
            ],
          },
        ],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
        listBudgetMs: 60_000,
        lawBudgetMs: 120_000,
        progress(event) {
          console.log(`wal-committed-transactions-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      for (const receipt of receipts) console.log(`wal-committed-transactions-native-receipt: ${JSON.stringify(receipt)}`);
    }
  }
}

/** 🧹️ Proves compaction observes only logically committed WAL effects. */
class WalCommittedCompactionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-committed-compaction-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🧾️committed-effects/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.compact", "CommittedEffectsV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(
      fixture.segments.map((row: any) => row.index),
      fixture.segments.map((_: any, index: number) => index),
    );
    assert.equal(fixture.segments.at(-1).state, "active");
    const committed = fixture.segments.flatMap((segment: any) =>
      segment.transactions.filter((transaction: any) => transaction.outcome === "commit").flatMap((transaction: any) => transaction.records.map((record: any) => ({ ...record, segment: segment.index }))),
    );
    const horizons = fixture.segments.map((segment: any) => ({
      segment: segment.index,
      head: committed.filter((record: any) => record.segment === segment.index && ["frontier", "snapshot"].includes(record.kind)).reduce((head: number | null, record: any) => (head === null ? record.headSeq : Math.max(head, record.headSeq)), null),
    }));
    const highest = fixture.segments.at(-1).index;
    const deletedSegments = horizons.filter((row: any) => row.segment !== highest && row.head !== null && row.head <= fixture.floorHeadSeq).map((row: any) => row.segment);
    const deletedPayloads =
      fixture.actor.payloadReclamation === "deferred-global-reference-authority"
        ? []
        : committed
            .filter((record: any) => record.kind === "payload" && deletedSegments.includes(record.segment) && !committed.some((live: any) => live.kind === "payload" && live.payload === record.payload && !deletedSegments.includes(live.segment)))
            .map((record: any) => record.payload);
    const allPayloads = new Set(fixture.segments.flatMap((segment: any) => segment.transactions.flatMap((transaction: any) => transaction.records.filter((record: any) => record.kind === "payload").map((record: any) => record.payload))));
    assert.deepEqual(
      {
        deletedSegments: deletedSegments.length,
        deletedPayloads: new Set(deletedPayloads).size,
        remainingSegments: fixture.segments.map((row: any) => row.index).filter((index: number) => !deletedSegments.includes(index)),
        retainedPayloads: [...allPayloads].filter((payload) => !deletedPayloads.includes(payload)),
      },
      fixture.expected,
    );
    assert.deepEqual(fixture.actor, {
      priority: "command",
      writerAuthority: "retained-artifact-wal",
      activeSegmentAuthority: "artifact-wal",
      leaseBefore: ["snapshot-floor", "wal-horizon", "wal-delete"],
      indexBudgetContinuation: "same-owned-future-cooperative-yield",
      payloadReclamation: "deferred-global-reference-authority",
      queuedSubmitDuring: "pending",
      queuedSubmitAfter: "accepted",
    });
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const committedCut = source.slice(source.indexOf("async fn committed_compaction_horizons"), source.indexOf("async fn retained_compaction_under_lease"));
    assert(committedCut.includes("replay_committed_document") && committedCut.includes("close_record_step") && committedCut.includes("transaction.finish()") && committedCut.includes("close_compaction_replay"));
    assert(source.slice(source.indexOf("async fn close_compaction_replay"), source.indexOf("async fn close_compaction_owner")).includes("close_owner_step"));
    assert(!committedCut.includes("WalReplayCursor") && !committedCut.includes("replay_document"));
    const liveCut = source.slice(source.indexOf("pub async fn retained_compaction_with_wal"), source.indexOf("async fn retained_compaction_under_lease"));
    assert(liveCut.indexOf("CompactionLease::acquire") < liveCut.indexOf("SnapshotFloor"));
    assert(!liveCut.includes("acquire_writer"));
    const underLeaseCut = source.slice(source.indexOf("async fn retained_compaction_under_lease"), source.indexOf("async fn retained_compaction_snapshot"));
    assert(!underLeaseCut.includes("loop {\n            let deadline") && underLeaseCut.includes("handle.compact(&mut control).await"));
    const walSource = readFileSync(join(owner, "..", "📝️wal", "🦀️.rs"), "utf8");
    assert(walSource.includes("delete_compacted_sealed_segment"));
    const artifactSource = readFileSync(join(owner, "..", "🗿️artifact", "🦀️.rs"), "utf8");
    assert(artifactSource.includes("ArtifactMessage::Compact") && artifactSource.includes("Priority::Command"));
    const laws = readFileSync(join(owner, "🧪️tests", "🔬️unit", "🦀️.rs"), "utf8");
    assert(laws.includes("fn compaction_applies_only_committed_frontier_snapshot_and_payload_effects("));
    console.log("wal-committed-compaction-independent-oracle: abort effects excluded, global payloads retained, header-only highest preserved");
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        cargoArgs: ["--all-features"],
        groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib", name: "db" },
            laws: [
              "db_compact::tests::compaction_applies_only_committed_frontier_snapshot_and_payload_effects",
              "db_compact::tests::document_compaction_retains_shared_and_private_cas_without_global_reference_authority",
              "db_engine::tests::compact_document_uses_live_actor_writer_and_restores_submits",
            ],
          },
        ],
        progress(event) {
          console.log(`wal-committed-compaction-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`wal-committed-compaction-native-receipts: ${JSON.stringify(receipts)}`);
    }
  }
}

/** 🚪️ Proves retained database shutdown keeps exact retry owners across interruption and shared authority. */
class DatabaseShutdownCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-shutdown-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🚪️shutdown/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.engine", "ShutdownV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(fixture.phases, ["authority", "emit", "complete"]);
    assert.equal(fixture.maximumAuthorityStepsPerTurn, 1);
    for (const row of fixture.cases) {
      let authorityOwners = row.initial.authorityOwners;
      let externalAuthorityOwners = row.initial.externalAuthorityOwners;
      let closing = false;
      let cancelled = false;
      let terminal = false;
      let emitCount = 0;
      let authorityRetained = false;
      for (const action of row.trace) {
        if (action === "cancel") {
          cancelled = true;
          authorityRetained ||= authorityOwners > 0 || closing;
          continue;
        }
        if (action === "retry") {
          cancelled = false;
          continue;
        }
        if (action === "release-shared-owner") {
          externalAuthorityOwners = 0;
          continue;
        }
        if (cancelled || terminal) continue;
        if (authorityOwners > 0) {
          if (externalAuthorityOwners > 0) {
            authorityRetained = true;
          } else if (!closing) {
            closing = true;
          } else {
            closing = false;
            authorityOwners -= 1;
          }
        } else if (emitCount === 0) {
          emitCount = 1;
        } else {
          terminal = true;
        }
      }
      assert.deepEqual({ terminal, authorityRetained, emitCount }, row.expected, row.name);
    }
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const artifact = readFileSync(join(owner, "..", "🗿️artifact", "🦀️.rs"), "utf8");
    assert(source.includes("closing_authority: Option<") && source.includes("pub async fn shutdown_step(&mut self"));
    assert(source.includes("DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Authorities"));
    assert(source.includes("shutdown_emit_started") && !source.includes("shutdown_graph_complete"));
    assert(!source.includes("pub async fn shutdown(self"));
    assert(artifact.includes("pub fn shutdown_step(&self) -> bool") && artifact.includes("handoff.terminal"));
    console.log(`database-shutdown-independent-oracle: AJV=1 cases=${fixture.cases.length} retained-authority=1 terminal-ack=1`);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        cargoArgs: ["--all-features"],
        groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib", name: "db" },
            laws: [
              "db_engine::tests::database_shutdown_cancellation_preserves_exact_retry_owners",
              "db_engine::tests::database_shutdown_shared_authority_blocks_without_closing_live_handle",
            ],
          },
        ],
        progress(event) {
          console.log(`database-shutdown-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`database-shutdown-native-receipts: ${JSON.stringify(receipts)}`);
    }
  }
}

/** 🪢️ Proves one generation-fenced retained mount owner serves every concurrent document opener. */
class DocumentMountSingleFlightCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("document-mount-single-flight-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🚪️document-mount");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.engine", "DocumentMountV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    const mountedPoolUse = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔐️pool-use/🔣️.json"), "utf8"));
    const mountedPoolUseSchema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔐️pool-use/🔣️.json"), "utf8"));
    const validateMountedPoolUse = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(mountedPoolUseSchema);
    assert(validateMountedPoolUse(mountedPoolUse), JSON.stringify(validateMountedPoolUse.errors));
    for (const row of mountedPoolUse.cases) {
      const retainedUses = row.externalUses + (row.pool === "open" && (row.database === "open" || row.database === "opening-non-runnable") ? 1 : 0);
      const shutdown = retainedUses ? `busy-${retainedUses}` : "stopped";
      assert.equal(shutdown, row.expectedShutdown, row.id);
      if (row.authority === "ready") assert.equal(row.database, "open", row.id);
      if (row.database === "terminal" || row.database === "absent") assert.equal(row.databaseActivities, "closed", row.id);
    }
    const mountedEngine = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs"), "utf8");
    const mountedArtifact = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs"), "utf8");
    const mountedSync = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs"), "utf8");
    assert(mountedEngine.includes('#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;'), "missing actual database pool-use test mount");
    const engineLaws = readFileSync(join(owner, "..", "🧪️tests", "🔬️unit", "🦀️.rs"), "utf8");
    for (const marker of ["pool_use: Option<Arc<WorkerPoolUse>>", "let pool_use = pool.acquire_use()", "fn require_open_use(&self)", "self.pool_use.take()", "DatabaseRetainedActivityRejected::Closed", "DatabaseDocumentMountDriver::NonRunnable", "DatabaseShutdownBlock::Executor(kind)"]) assert(mountedEngine.includes(marker), `missing mounted pool-use marker ${marker}`);
    const catalogStateStart = mountedEngine.indexOf("struct DatabaseCreateCatalogState {");
    const catalogState = mountedEngine.slice(catalogStateStart, mountedEngine.indexOf("\n}", catalogStateStart));
    assert.equal((mountedEngine.match(/_pool_use: Arc<WorkerPoolUse>/g)?.length ?? 0) + Number(catalogState.includes("pool_use: Mutex<Option<Arc<WorkerPoolUse>>>")), 4, "every retained Database capability/catalog state must own the use cell");
    assert(mountedEngine.includes("pool_use: Mutex::new(Some(pool_use))"), "catalog publication must retain its admitted use cell");
    assert(mountedEngine.includes("self.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();"), "catalog terminal drain must release its exact admitted use cell");
    for (const marker of ["_pool_use: Arc<semio_framework_async::WorkerPoolUse>", "spawn_with_pool_use", "pool.acquire_use()"] ) assert(mountedArtifact.includes(marker), `missing authority pool-use marker ${marker}`);
    for (const marker of ["_pool_use: std::sync::Arc<semio_framework_async::WorkerPoolUse>", "let pool_use = match pool.acquire_use()"] ) assert(mountedSync.includes(marker), `missing sync-hello pool-use marker ${marker}`);
    for (const law of ["database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack", "database_worker_pool_use_is_admitted_before_the_first_storage_probe", "database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer"]) assert(engineLaws.includes(`fn ${law}(`), `missing mounted pool-use law ${law}`);
    console.log(`[DEBUG] database-mounted-pool-use-independent-oracle: AJV=1 cases=${mountedPoolUse.cases.length} retained-source-owners=3`);
    for (const row of fixture.cases) {
      let activeGeneration: bigint | undefined;
      let waiters = 0;
      let ready = false;
      let terminalCleanup = true;
      let emitting = false;
      let events = 0;
      let fanoutPrepared = false;
      let registryUnlocked = true;
      let ownerPollActive = false;
      for (const step of row.steps) {
        const generation = BigInt(step.generation);
        if (step.action === "install") {
          assert.equal(activeGeneration, undefined, row.name);
          activeGeneration = generation;
          waiters = 1;
          ready = false;
        } else if (step.action === "join") {
          assert.equal(generation, activeGeneration, row.name);
          waiters += 1;
        } else if (step.action === "cancel-waiter") {
          assert.equal(generation, activeGeneration, row.name);
          waiters -= 1;
        } else if (step.action === "catalog-published" || step.action === "shutdown-interrupted") {
          assert.equal(generation, activeGeneration, row.name);
        } else if (step.action === "emit-begin") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!emitting && events === 0, row.name);
          emitting = true;
        } else if (step.action === "emit-complete") {
          assert.equal(generation, activeGeneration, row.name);
          assert(events === 0, row.name);
          emitting = false;
          events = 1;
        } else if (step.action === "fill-waiters") {
          assert.equal(generation, activeGeneration, row.name);
          waiters = fixture.capacity.waitersPerDocument;
        } else if (step.action === "reject-over-capacity") {
          assert.equal(generation, activeGeneration, row.name);
          assert.equal(waiters, fixture.capacity.waitersPerDocument, row.name);
        } else if (step.action === "release-slot") {
          assert.equal(generation, activeGeneration, row.name);
          waiters -= 1;
        } else if (step.action === "close-fault") {
          assert.equal(generation, activeGeneration, row.name);
          terminalCleanup = false;
        } else if (step.action === "resume-cleanup") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!terminalCleanup, row.name);
        } else if (step.action === "explicit-retirement-retry") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!terminalCleanup, row.name);
        } else if (step.action === "close-terminal") {
          assert.equal(generation, activeGeneration, row.name);
          terminalCleanup = true;
          activeGeneration = undefined;
        } else if (step.action === "prepare-fanout") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!fanoutPrepared && registryUnlocked, row.name);
          fanoutPrepared = true;
          registryUnlocked = false;
        } else if (step.action === "unlock-registry") {
          assert.equal(generation, activeGeneration, row.name);
          assert(fanoutPrepared && !registryUnlocked, row.name);
          registryUnlocked = true;
        } else if (step.action === "wake-waiter") {
          assert.equal(generation, activeGeneration, row.name);
          assert(fanoutPrepared && registryUnlocked, row.name);
          fanoutPrepared = false;
        } else if (step.action === "owner-poll-active") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!ownerPollActive, row.name);
          ownerPollActive = true;
        } else if (step.action === "request-drive") {
          assert.equal(generation, activeGeneration, row.name);
          assert(ownerPollActive, row.name);
        } else if (step.action === "owner-poll-release") {
          assert.equal(generation, activeGeneration, row.name);
          assert(ownerPollActive, row.name);
          ownerPollActive = false;
        } else if (step.action === "retry") {
          assert.equal(activeGeneration, undefined, row.name);
          activeGeneration = generation;
          waiters = 1;
          events = 0;
        } else if (step.action === "ready") {
          assert.equal(generation, activeGeneration, row.name);
          assert(terminalCleanup && !emitting && events === 1, row.name);
          ready = true;
        }
      }
      assert.equal(ready ? 1 : 0, row.expected.actors, row.name);
      assert.equal(row.expected.owners, 1, row.name);
      assert.equal(events, row.expected.events, row.name);
      assert.equal(waiters, row.expected.waitersReleased, row.name);
      assert.equal(terminalCleanup, row.expected.terminalBeforeFanout, row.name);
      assert(!fanoutPrepared && registryUnlocked && !ownerPollActive, row.name);
    }
    for (const row of fixture.driverCases) {
      let state = "idle";
      let wakeRequested = false;
      let resumeRequested = false;
      let queued = 0;
      let maximumQueued = 0;
      let cleanupParked = false;
      let cleanupResumes = 0;
      for (const step of row.steps) {
        if (step === "request" || step === "request-resume") {
          wakeRequested = true;
          resumeRequested ||= step === "request-resume";
          if (state === "idle") {
            state = "queued";
            queued = 1;
            maximumQueued = Math.max(maximumQueued, queued);
          }
        } else if (step === "poll-start") {
          assert.equal(state, "queued", row.name);
          state = "polling";
          queued = 0;
          wakeRequested = false;
        } else if (step === "cleanup-fault") {
          assert.equal(state, "polling", row.name);
          cleanupParked = true;
        } else if (step === "cleanup-resume") {
          assert(cleanupParked && resumeRequested, row.name);
          cleanupParked = false;
          resumeRequested = false;
          cleanupResumes += 1;
        } else if (step === "hard-submit-rejected") {
          assert.equal(state, "queued", row.name);
          state = "non-runnable";
        } else if (step === "poll-pending") {
          assert.equal(state, "polling", row.name);
          if (wakeRequested) wakeRequested = false;
          else state = "idle";
        } else if (step === "complete") {
          state = "terminal";
        }
      }
      assert.equal(state, row.expected.state, row.name);
      assert.equal(maximumQueued, row.expected.maximumQueuedPollers, row.name);
      assert.equal(cleanupResumes, row.expected.cleanupResumes, row.name);
    }
    const source = readFileSync(join(owner, "..", "🦀️.rs"), "utf8");
    for (const marker of [
      "enum DatabaseDocumentMountSlot",
      "Opening {",
      "struct DatabaseDocumentMountOwner",
      "DatabaseDocumentMountWork",
      "DATABASE_DOCUMENT_MOUNT_WAITERS",
      "fn take_generation",
      "pub async fn ensure_document",
      "retained_mount_rejection",
      "DatabaseDocumentMountWait",
      "impl Drop for DatabaseDocumentMountWait",
    ])
      assert(source.includes(marker), `missing retained document-mount marker: ${marker}`);
    assert(!source.includes("open_artifacts: Mutex<HashMap<String, Arc<db_artifact::ArtifactAuthority>>>") && source.includes("open_artifacts: Arc<Mutex<DatabaseDocumentMountRegistry>>"));
    const retainedMount = source.slice(source.indexOf("async fn run_document_mount("), source.indexOf("async fn mount_document("));
    assert(retainedMount.indexOf("emit.emit(") >= 0 && retainedMount.indexOf("emit.emit(") < retainedMount.indexOf("Ok(DatabaseDocumentMountReply"));
    assert(!source.slice(source.indexOf("struct DatabaseDocumentMountReply"), source.indexOf("struct DatabaseDocumentMountWaiter")).includes("emission:"));
    const complete = source.slice(source.indexOf("fn complete(&self, result: Result<DatabaseDocumentMountReply, DbError>)"), source.indexOf("//#region 🔖️Database"));
    assert(complete.includes("let mut fanout: [Option<(") && complete.includes("DATABASE_DOCUMENT_MOUNT_WAITERS"));
    assert(complete.indexOf("drop(authority);") < complete.indexOf("for (reply, outcome) in fanout.into_iter().flatten()"));
    assert(complete.indexOf("self.terminal.store(true") < complete.indexOf("for (reply, outcome) in fanout.into_iter().flatten()"));
    const registryScope = complete.slice(complete.indexOf("let mut registry = registry.lock()"), complete.indexOf("for (reply, outcome) in fanout.into_iter().flatten()"));
    assert(registryScope.trimEnd().endsWith("self.terminal.store(true, Ordering::Release);\n        }"), "waiters must be woken only after the registry guard's scope has closed");
    const mountOwner = source.slice(source.indexOf("impl DatabaseDocumentMountOwner"), source.indexOf("//#region 🔖️Database"));
    assert(
      mountOwner.includes("DatabaseDocumentMountDriver::Idle") &&
        mountOwner.includes("DatabaseDocumentMountDriver::Queued") &&
        mountOwner.includes("DatabaseDocumentMountDriver::Polling") &&
        mountOwner.includes("DatabaseDocumentMountDriver::NonRunnable"),
    );
    assert(mountOwner.includes("resume_requested") && mountOwner.includes("wake_requested"));
    assert(mountOwner.includes("Arc::downgrade(self)") && !mountOwner.includes("let owner = self.clone();\n        self.submit_exact"));
    const databaseOpen = source.slice(source.indexOf("async fn open_with("), source.indexOf("fn document_engine_config("));
    assert.equal(databaseOpen.match(/acquire_use\(\)/g)?.length, 1);
    for (const marker of ["DatabaseCapabilityOpenFuture::try_prepare_with_use", "DatabaseCatalogReadFuture::try_prepare_with_use", "DatabaseCatalogBootstrapFuture::try_prepare_with_use"])
      assert(databaseOpen.includes(marker), `database open minted an untracked pool use instead of retaining ${marker}`);
    const catalogPublication = source.slice(source.indexOf("async fn publish_mount_catalog("), source.indexOf("async fn run_open_document_mount("));
    assert(catalogPublication.includes("DatabaseCreateCatalogFuture::try_prepare_with_use(pool, pool_use"));
    const catalogDriveStart = source.indexOf("fn drive_one(self: Arc<Self>, generation: u64)", source.indexOf("//#region 🔖️CreateDocumentCatalogCas"));
    const catalogDrive = source.slice(catalogDriveStart, source.indexOf("fn drive_claimed(self: &Arc<Self>, generation: u64)", catalogDriveStart));
    assert(!catalogDrive.includes("release_success()"));
    const catalogTerminal = source.slice(source.indexOf("pub struct DatabaseCreateCatalogTerminalHandle"), source.indexOf("pub fn take_database_create_catalog_terminal"));
    assert(catalogTerminal.includes("drive_explicit_close_one()"), "explicit catalog terminal cleanup must make one bounded turn without a wall-clock callback");
    const helloStart = source.indexOf("pub fn hello_retained(");
    const hello = source.slice(helloStart, source.indexOf("pub async fn hello(", helloStart));
    assert(hello.includes("DatabaseSyncHelloFuture::try_submit_with_use") && !hello.includes("DatabaseSyncHelloFuture::try_submit("));
    const requestDrive = mountOwner.slice(mountOwner.indexOf("fn request_drive(self: &Arc<Self>"), mountOwner.indexOf("fn resume_parked("));
    assert(!requestDrive.includes("self.work.try_lock()") && !requestDrive.includes("self.work.lock()"));
    const artifact = readFileSync(join(owner, "..", "..", "🗿️artifact", "🦀️.rs"), "utf8");
    for (const marker of [
      "enum ArtifactRunnerDriver",
      "RunnableIdle",
      "Queued",
      "PollingWake",
      "Parked",
      "ClosingReady",
      "ClosingPollingWake",
      "ClosingParked",
      "Terminal",
      "fn park_terminal_job",
      "struct ArtifactRunnerClosePoll",
      "struct ArtifactRunnerPoll",
      "struct ArtifactRunnerRetirementReservation",
      "WorkerMaintenanceStep::Retire",
      "impl Drop for ArtifactAuthority",
      "pub fn close(mut self) -> Result<(), Self>",
      "pub fn resume(mut self) -> Result<(), Self>",
    ]) {
      assert(artifact.includes(marker), `missing artifact terminal-authority marker: ${marker}`);
    }
    for (const marker of ["close_error", "WorkerMaintenanceStep::Fault"]) {
      assert(artifact.includes(marker), `missing retained artifact close-fault marker: ${marker}`);
    }
    const artifactLaws = readFileSync(join(owner, "..", "..", "🗿️artifact", "🧪️tests", "🔬️unit", "🦀️.rs"), "utf8");
    for (const law of [
      "artifact_engine_close_fault_retries_on_bounded_timer_backoff_until_terminal",
      "artifact_engine_close_fault_exhausts_its_budget_then_polls_only_on_readmission",
      "artifact_engine_close_fault_cancel_stops_the_timer_until_readmission",
    ]) {
      assert(artifactLaws.includes(`fn ${law}(`), `missing retained artifact close-fault law ${law}`);
    }
    const artifactSchedule = artifact.slice(artifact.indexOf("fn schedule(self: &Arc<Self>)"), artifact.indexOf("fn submit_exact(self: &Arc<Self>"));
    assert(artifactSchedule.includes("compare_exchange") && artifactSchedule.includes("ArtifactRunnerDriver::RunnableIdle as u8") && artifactSchedule.includes("ArtifactRunnerDriver::Queued as u8"));
    assert(!artifactSchedule.includes("scheduled.compare_exchange"));
    const observe = readFileSync(join(owner, "..", "..", "👁️observe", "🦀️.rs"), "utf8");
    assert(observe.includes("fn emit(&self, event: EmitEvent) -> impl Future<Output = ()> + Send;"));
    const hub = readFileSync(join(this.repoRoot, "🌎️hub/🏗️bootstrap/🦀️.rs"), "utf8");
    const ensure = hub.slice(hub.indexOf("async fn ensure_document(&self"), hub.indexOf("fn bearer("));
    assert(ensure.includes("self.db.ensure_document(id).await") && ensure.includes("rejected.retry_close().await"));
    assert(!ensure.includes("self.db.create_document") && !ensure.includes("self.db.document(id)"));
    for (const law of [
      "database_document_mount_coalesces_join_drives_without_shared_pool_starvation",
      "database_document_mount_cleanup_fault_consumes_racing_resume_request_exactly_once",
      "database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer",
    ]) {
      assert(engineLaws.includes(`fn ${law}(`), `missing exact mount driver law ${law}`);
    }
    console.log(`document-mount-single-flight-independent-oracle: AJV=1 cases=${fixture.cases.length} waiters=${fixture.capacity.waitersPerDocument} owner-futures=${fixture.capacity.ownerFuturesPerDocument}`);
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [
        {
          package: "semio-framework-os-kernel-db",
          target: { kind: "lib", name: "db" },
          cargoArgs: ["--all-features"],
          laws: [
            "db_engine::tests::database_concurrent_ensure_mounts_one_actor_and_one_writer",
            "db_engine::tests::database_published_opening_joins_without_actor_overwrite",
            "db_engine::tests::database_cancelled_ensure_waiter_does_not_cancel_mount_owner",
            "db_engine::tests::database_document_mount_failure_waiters_share_terminal_cleanup_and_retry_generation",
            "db_engine::tests::database_mount_owner_emits_before_ready_and_survives_elected_waiter_cancellation",
            "db_engine::tests::database_mount_waiter_capacity_rejects_33_and_reuses_one_cancelled_slot",
            "db_engine::tests::database_document_mount_fanout_wakes_only_after_registry_unlock_and_internal_owner_handoff",
            "db_engine::tests::database_shutdown_interrupt_retains_waiterless_opening_owner_until_ready",
            "db_engine::tests::database_document_mount_unlock_fault_parks_exact_owner_until_controlled_shutdown_resume",
            "db_engine::tests::database_document_mount_coalesces_join_drives_without_shared_pool_starvation",
            "db_engine::tests::database_document_mount_cleanup_fault_consumes_racing_resume_request_exactly_once",
            "db_engine::tests::database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack",
            "db_engine::tests::database_worker_pool_use_is_admitted_before_the_first_storage_probe",
            "db_engine::tests::database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer",
            "db_engine::tests::database_create_catalog_resolved_drop_retains_use_until_terminal_drain",
            "db_artifact::tests::artifact_runner_terminal_authority_latch_preserves_external_job_and_one_resume",
            "db_artifact::tests::artifact_runner_closing_poll_waits_for_retained_wake_before_next_turn",
            "db_artifact::tests::artifact_runner_terminal_close_returns_exact_cursor_until_retained_wake",
            "db_artifact::tests::artifact_runner_terminal_resume_refusal_returns_exact_cursor_for_close",
            "db_artifact::tests::artifact_authority_drop_transfers_parked_terminal_job_to_registered_close_owner",
            "db_artifact::tests::artifact_runner_retirement_panic_retains_exact_cursor_until_explicit_retry",
            "db_artifact::tests::artifact_engine_close_fault_retries_on_bounded_timer_backoff_until_terminal",
            "db_artifact::tests::artifact_engine_close_fault_exhausts_its_budget_then_polls_only_on_readmission",
            "db_artifact::tests::artifact_engine_close_fault_cancel_stops_the_timer_until_readmission",
            "db_artifact::tests::more_live_authorities_than_pool_maintenance_hooks_retire_through_one_shared_hook",
          ],
        },
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 180_000,
      progress(event) {
        console.log(`document-mount-single-flight-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    console.log(`document-mount-single-flight-native-receipts: ${JSON.stringify(receipts)}`);
  }
}

/** 🗺️ Proves the fixed owned Map decision envelope with independent AJV and SHA-256 oracles. */
class DurableOwnedGroupDecisionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("durable-owned-group-decision-check accepts only --native");
    const { testDurableOwnedGroupDecisionFixture } = await import("../../🔨️modules/🏪️store/🧪️tests/🗄️durable-owned-group/🟦️.ts");
    testDurableOwnedGroupDecisionFixture();
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [
          {
            package: "semio-framework-os-kernel",
            target: { kind: "lib" },
            laws: [
              "durable_group::tests::durable_owned_group_decision_matches_neutral_canonical_hash_and_bounds",
              "durable_group::tests::durable_group_journal_record_projects_edits_only_after_all_three_bound_outcomes_verify",
              "durable_group::tests::durable_store_prepared_outcome_derives_and_verifies_exact_unbound_bytes",
              "durable_group::tests::durable_owned_group_decision_rejects_forged_identity_commitment_and_capacity",
              "durable_group::tests::durable_decision_rejects_deflate_expansion_before_document_body_allocation",
              "durable_group::tests::durable_store_owned_three_member_bind_and_base_recovery_retain_exact_private_owners",
              "durable_group::tests::durable_store_private_committed_record_recovers_all_three_stores_without_reappending_journal",
              "durable_group::tests::durable_json_carriers_preserve_numeric_kinds_and_reject_control_and_resource_excess",
              "durable_group::tests::durable_store_group_journal_commit_flips_one_shared_root_then_adopts_exactly_once",
              "durable_group::tests::durable_store_group_cancellation_waits_for_trusted_absence_then_restores_all_old_roots",
              "durable_group::tests::durable_store_group_stage_error_retains_abort_owner_until_every_root_is_empty",
              "durable_group::tests::durable_store_group_uncertain_journal_error_retries_same_owner_without_rebegin_or_visibility_change",
              "durable_group::tests::durable_store_group_rejects_foreign_anchor_receipt_before_visibility_and_aborts_only_after_absence",
              "durable_group::tests::durable_map_fixed_host_slot_retains_every_live_owner_across_request_error_until_terminal_handoff",
              "durable_group::tests::durable_map_fixed_host_slot_cancellation_after_uncertain_io_waits_for_trusted_absence",
            ],
          },
        ],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
        listBudgetMs: 60_000,
        lawBudgetMs: 120_000,
        progress(event) {
          console.log(`durable-owned-group-decision-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      for (const receipt of receipts) console.log(`durable-owned-group-decision-native-receipt: ${JSON.stringify(receipt)}`);
    }
  }
}

/** 🧾️ Proves the typed Store-decision journal boundary and exact physical WAL reservation. */
class DurableGroupJournalCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("durable-group-journal-check accepts only --native");
    const artifactOwner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact");
    const fixtureOwner = join(artifactOwner, "🧫️fixtures/📓️durable-group-journal");
    const fixture = JSON.parse(readFileSync(join(fixtureOwner, "🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.artifact", "DurableGroupJournalV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.equal(new Set(fixture.cases.map((row: any) => row.id)).size, fixture.cases.length);
    for (const row of fixture.committedDecisionWitnessCases) {
      const events = row.recordKinds.filter((kind: string) => kind === "event").length;
      const expected = row.transaction === "aborted" || events === 0 ? "ignored" : row.replayDocument === "foreign" || events !== 1 || row.recordKinds.length !== 1 ? "rejected" : "witness";
      assert.equal(row.expected, expected);
    }
    assert.deepEqual(fixture.committedRecovery.authority, {
      consumer: "db-wal-witness",
      recordEscape: false,
      receiptSource: "derived",
      cancelAfterCommit: false,
      errorSurface: "retaining-fault",
      terminalHandoff: "stores-and-ack-once",
    });
    assert.equal(new Set(fixture.committedRecovery.cases.map((row: any) => row.id)).size, fixture.committedRecovery.cases.length);
    for (const row of fixture.committedRecovery.cases) {
      const expected = row.frontier === "base" ? ["complete", 3] : row.frontier === "post" ? ["already-applied", 0] : ["fault", 0];
      assert.deepEqual([row.expected, row.mutations], expected);
      assert.equal(row.returnedStoreOwners, 3);
    }
    const storeOwner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group");
    const storeFixture = JSON.parse(readFileSync(join(storeOwner, "🧫️fixtures/🔣️.json"), "utf8"));
    assert.equal(createHash("sha256").update(storeFixture.expected.unsignedJson).digest("hex"), fixture.record.decisionSha256);
    assert.equal(storeFixture.expected.anchorSha256, fixture.record.anchorSha256);
    const varintBytes = (value: number) => {
      let bytes = 1;
      while (value >= 128) {
        value = Math.floor(value / 128);
        bytes += 1;
      }
      return bytes;
    };
    const frameBytes = (payload: number) => varintBytes(payload + 2) + payload + 10;
    const fieldBytes = (value: string) => varintBytes(Buffer.byteLength(value)) + Buffer.byteLength(value);
    const successorHeaderBytes = 32 + frameBytes(fieldBytes(fixture.record.document) + 41) + 75;
    const transactionBytes = (eventBytes: number) => frameBytes(8) + frameBytes(eventBytes) + frameBytes(12) + 75;
    assert.equal(successorHeaderBytes, fixture.limits.successorHeaderBytes);
    assert.equal(transactionBytes(fixture.limits.storeEventBytes), fixture.limits.storeMaximumTransactionBytes);
    assert.equal(successorHeaderBytes + transactionBytes(fixture.limits.storeEventBytes), fixture.limits.storeMaximumSegmentBytes);
    let maximumEventBytes = fixture.limits.walSegmentBytes;
    while (successorHeaderBytes + transactionBytes(maximumEventBytes) > fixture.limits.walSegmentBytes) maximumEventBytes -= 1;
    assert.equal(maximumEventBytes, fixture.limits.maximumEventBytes);
    assert(successorHeaderBytes + transactionBytes(fixture.limits.storeEventBytes) <= fixture.limits.walSegmentBytes);
    assert(successorHeaderBytes + transactionBytes(maximumEventBytes + 1) > fixture.limits.walSegmentBytes);
    const storeSource = readFileSync(join(storeOwner, "🦀️.rs"), "utf8");
    const artifactSource = readFileSync(join(artifactOwner, "🦀️.rs"), "utf8");
    const walSource = readFileSync(join(artifactOwner, "../📝️wal/🦀️.rs"), "utf8");
    const engineSource = readFileSync(join(artifactOwner, "../⚙️engine/🦀️.rs"), "utf8");
    assert(storeSource.includes("pub struct DurableOwnedGroupJournalRecordV1") && storeSource.includes("DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&canonical_pack)"));
    assert(storeSource.includes("pub enum DurableOwnedGroupJournalAdvanceV1") && storeSource.includes("Rejected(String)"));
    const append = artifactSource.slice(artifactSource.indexOf("async fn append_durable_group_decision("), artifactSource.indexOf("pub(crate) async fn compact_retained", artifactSource.indexOf("async fn append_durable_group_decision(")));
    assert(append.includes("WalRecord::Event") && append.includes("DurabilityClass::Fsync"));
    assert(append.indexOf("preflight_submit") < append.indexOf("self.wal.submit"));
    assert(append.includes("ArtifactDurableGroupJournalAppendV1::Absent") && append.includes("ArtifactDurableGroupJournalAppendV1::Rejected"));
    const witness = artifactSource.slice(artifactSource.indexOf("struct ArtifactCommittedDurableGroupDecisionV1"), artifactSource.indexOf("//#endregion 🔖️Receipt"));
    const lowerRecovery = storeSource.slice(
      storeSource.indexOf("impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapRecoveryHostV1"),
      storeSource.indexOf("impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapRecoveryHostV1"),
    );
    assert(witness.includes("WalCommittedTransaction") && witness.includes("record_count != 1") && witness.includes("transaction.finish()?"));
    for (const marker of [
      "ArtifactCommittedDurableGroupRecoveryV1",
      "ArtifactCommittedDurableGroupRecoveryStateV1::Admitting",
      "ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault",
      "into_store_owned_recovery",
      "restore_untrusted_committed_record",
      "acknowledge_restoration(&receipt)",
      "take_terminal",
      "take_rejected_terminal",
    ])
      assert(witness.includes(marker), `missing committed Store recovery marker ${marker}`);
    assert(!witness.includes("pub fn begin_store_owned_recovery"));
    assert(!witness.includes("fn into_record(") && !witness.includes("fn record(&self)"), "a committed WAL witness has no raw record escape");
    assert(!witness.includes("fn cancel("), "committed Store recovery remains non-cancellable after WAL visibility");
    assert(
      lowerRecovery.includes("pub fn advance(&mut self, grant: super::ArtifactStoreOneItemGrant) -> DurableOwnedMapRecoveryAdvanceV1") && lowerRecovery.includes("DurableOwnedMapRecoveryAdvanceV1::Fault(error)"),
      "lower Store recovery faults must retain their exact host instead of exposing Result/? owner loss",
    );
    assert(lowerRecovery.includes("pub fn capture_snapshot(&self) -> Option<"), "lower Store recovery observation must not expose a Result/? owner-loss path");
    const sink = artifactSource.slice(artifactSource.indexOf("struct ArtifactDurableGroupJournalSinkV1"), artifactSource.indexOf("type ArtifactBuildFuture"));
    assert(sink.includes("NotSubmitted") && sink.includes("Awaiting") && sink.includes("Failed") && sink.includes("Committed") && sink.includes("terminal_is_empty"));
    assert(!sink.includes("block_on"));
    assert(walSource.includes("pub(crate) fn preflight_submit(&self, commands: &[Vec<u8>], records: &WalRecordBatch)") && walSource.includes("wal transaction exceeds readable segment"));
    assert(engineSource.includes("pub fn durable_group_journal_sink(&self, now_ms: u64)"));
    const laws = [
      "db_artifact::tests::document_authority_durable_group_journal_commits_one_exact_fsync_event",
      "db_artifact::tests::committed_durable_group_decision_accepts_only_one_exact_event_transaction",
      "db_artifact::tests::committed_durable_group_recovery_consumes_wal_witness_and_returns_exact_three_stores_on_pre_mutation_rejection",
      "db_artifact::tests::document_authority_durable_group_journal_cancellation_before_handoff_is_absent",
      "db_artifact::tests::document_authority_durable_group_journal_rejects_hash_before_mailbox",
    ];
    for (const law of laws) assert(artifactSource.includes(`fn ${law.split("::").at(-1)}(`), `missing exact native law ${law}`);
    console.log(
      `durable-group-journal-independent-oracle: AJV=1 cases=${fixture.cases.length} witnesses=${fixture.committedDecisionWitnessCases.length} recovery=${fixture.committedRecovery.cases.length} max-event=${maximumEventBytes} store-margin=${fixture.limits.walSegmentBytes - fixture.limits.storeMaximumSegmentBytes}`,
    );
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, cargoArgs: ["--all-features"], laws }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 180_000,
      progress(event) {
        console.log(`durable-group-journal-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    console.log(`durable-group-journal-native-receipts: ${JSON.stringify(receipts)}`);
  }
}

/** 🚑️ Proves exact WAL commit boundaries with independent CRC/LEB128 and schema oracles. */
class WalRecoveryCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-recovery-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🚑️recovery/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.wal", "TailOnlyRecoveryV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    const failStop = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🛑️fail-stop/🔣️.json"), "utf8"));
    const validateFailStop = ownedExport(this.repoRoot, "db.wal", "FailStopV1");
    assert(validateFailStop(failStop), JSON.stringify(validateFailStop.errors));
    assert.deepEqual(
      failStop.cases.map((row: any) => [row.name, row.fault, row.expectedPhysicalSuffix]),
      [
        ["short-append", "shortAppend", "torn"],
        ["append-error", "appendError", "absent"],
        ["sync-error", "syncError", "complete"],
        ["successor-append-error", "successorAppendError", "complete"],
      ],
    );
    const { default: crc } = await import("crc-32/crc32c.js");
    const leb = await import("@webassemblyjs/leb128");
    const { inspectRetainedSprNeutral } = await import(join(this.repoRoot, "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts"));
    const checksum = (bytes: Uint8Array) => crc.buf(bytes) >>> 0;
    const fragmented = Buffer.from(Array.from({ length: 49152 }, (_, index) => (index * 17 + 3) % 251));
    for (const row of fixture.fragmentCopies) assert.equal(checksum(fragmented.subarray(row.offset, row.offset + row.length)), row.crc32c);
    const hash = (bytes: Buffer) => Buffer.from(blake3Hex(bytes), "hex");
    const u64 = (value: number) => {
      const bytes = Buffer.alloc(8);
      bytes.writeBigUInt64LE(BigInt(value));
      return bytes;
    };
    const frame = (kind: number, payload: Buffer) => {
      const body = Buffer.concat([Buffer.from([kind, 2]), payload]);
      const size = Buffer.from(leb.encodeU32(body.length));
      const trailer = Buffer.alloc(8);
      trailer.writeUInt32LE(checksum(body));
      trailer.writeUInt32LE(size.length + body.length + 8, 4);
      return Buffer.concat([size, body, trailer]);
    };
    const header = Buffer.alloc(32);
    Buffer.from([137, 83, 80, 82, 13, 10, 26, 10]).copy(header);
    header.writeUInt16LE(1, 8);
    header.writeUInt32LE(1, 12);
    header.writeUInt32LE(checksum(header.subarray(0, 20)), 20);
    let bytes = header;
    let chain = hash(header);
    let previousOffset = 0;
    const batches = [
      [frame(64, Buffer.concat([Buffer.from([1, 100]), u64(0), Buffer.from([0])]))],
      ...fixture.commands.map((command: string, index: number) => [frame(65, u64(index + 1)), frame(68, Buffer.from(command)), frame(66, Buffer.concat([u64(index + 1), Buffer.from([1, 0, 0, 0])]))]),
    ];
    for (const [index, records] of batches.entries()) {
      const payload = Buffer.alloc(64);
      const recordsLength = records.reduce((sum: number, value: Buffer) => sum + value.length, 0);
      chain = hash(Buffer.concat([chain, ...records.map(hash)]));
      payload.writeBigUInt64LE(BigInt(index + 1));
      payload.writeBigUInt64LE(BigInt(previousOffset), 8);
      payload.writeBigUInt64LE(BigInt(recordsLength), 16);
      payload.writeUInt32LE(records.length, 24);
      chain.copy(payload, 32);
      previousOffset = bytes.length + recordsLength;
      bytes = Buffer.concat([bytes, ...records, frame(12, payload)]);
      assert.equal(bytes.length, fixture.commitEnds[index]);
    }
    for (const row of fixture.cuts) {
      const prefix = bytes.subarray(0, row.cut);
      const span = row.cut < 32 ? { end: 0, sequence: 0 } : inspectRetainedSprNeutral(prefix, checksum, hash);
      assert.equal(span.end, row.trustedEnd);
      assert.equal(Math.max(129, span.end), row.recoveredEnd);
      assert.equal(Math.max(1, span.sequence), row.nextTxId);
    }
    const expectedAccepted = new Set(["missing", "highest-sealed", "successor-empty", "successor-partial", "successor-header", "compacted-clean"]);
    assert.equal(new Set(fixture.lifecycle.map((row: any) => row.name)).size, fixture.lifecycle.length);
    for (const row of fixture.lifecycle) assert.equal(row.accepted, expectedAccepted.has(row.name), row.name);
    console.log(`wal-recovery-independent-oracle: ${fixture.cuts.length} exact CRC/hash-chain prefixes, ${fixture.lifecycle.length} lifecycle rows`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const open = source.slice(source.indexOf("pub async fn open(storage:", source.indexOf("impl ArtifactWal")), source.indexOf("pub async fn document(&self)", source.indexOf("impl ArtifactWal")));
    assert(!open.includes("delete_segment"), "recovery must not delete a committed segment");
    assert(source.includes("resume_verified") && source.includes("RetainedSprVerification"), "full verification and exact writer resume are required");
    const replayClose = source.slice(
      source.indexOf("pub fn close_owner_step(&mut self)", source.indexOf("impl<'storage, S: db_storage::WalStorage> WalReplayCursor")),
      source.indexOf("pub async fn close_step(&mut self)", source.indexOf("impl<'storage, S: db_storage::WalStorage> WalReplayCursor")),
    );
    assert(replayClose.includes("pages.close_step()") && replayClose.includes("segments.close_step()"), "replay close must retire retained page and list owners");
    assert(!replayClose.includes("control.grant()"), "terminal replay close must remain available after cancellation");
    const artifactClose = source.slice(source.indexOf("pub fn close_step(&mut self)", source.indexOf("impl ArtifactWal")), source.indexOf("//#endregion 🔖️ArtifactWal"));
    assert(artifactClose.includes("self.active.close_step()") && artifactClose.includes("self.active.terminal_is_empty()"), "artifact WAL must expose explicit terminal owner retirement");
    const segmentClose = source.slice(source.indexOf("fn close_step(&mut self)", source.indexOf("impl SegmentWriter")), source.indexOf("//#endregion 🔖️Segment"));
    assert(segmentClose.indexOf("self.writer.take()") < segmentClose.indexOf("buf.close_step()"), "segment close must relinquish the retained writer before retiring its page buffer");
    assert(segmentClose.includes("force_flush is required before close"), "segment close must reject pending records");
    const segmentFlush = source.slice(source.indexOf("async fn commit_and_flush", source.indexOf("impl SegmentWriter")), source.indexOf("async fn tip_chain_hash", source.indexOf("impl SegmentWriter")));
    assert(segmentFlush.includes("new_len != expected_len") && segmentFlush.includes("self.flushed_len = new_len"), "WAL flush must verify the exact appended length before acknowledging it");
    assert(segmentFlush.indexOf("self.flushed_len = new_len") < segmentFlush.indexOf("storage.sync"), "a failed sync must retain knowledge that its append already landed");
    assert(segmentFlush.includes("self.poison()"), "every uncertain post-commit failure must poison the live writer");
    const rotate = source.slice(source.indexOf("async fn rotate", source.indexOf("impl ArtifactWal")), source.indexOf("pub fn close_step", source.indexOf("impl ArtifactWal")));
    assert(rotate.indexOf("storage.seal") < rotate.indexOf("self.active.poison()"), "a sealed segment must poison its old live writer before successor creation");
    const laws = [
      "db_wal::tests::wal_recovery_preserves_neutral_committed_prefixes",
      "db_wal::tests::wal_recovery_matches_neutral_lifecycle_without_prefix_replacement",
      "db_wal::retained_tests::wal_replay_cancellation_remains_set_while_close_reaches_terminal_empty",
      "db_wal::retained_tests::artifact_wal_repeated_open_close_is_page_budget_neutral",
      "db_wal::retained_tests::artifact_wal_close_rejects_pending_records_and_closed_writes",
      "db_wal::retained_tests::artifact_wal_short_append_is_fail_stop_until_reopen",
      "db_wal::retained_tests::artifact_wal_append_error_is_fail_stop_until_reopen",
      "db_wal::retained_tests::artifact_wal_sync_error_is_fail_stop_until_reopen",
      "db_wal::retained_tests::artifact_wal_successor_failure_after_seal_is_fail_stop_until_reopen",
      "db_fault_testing::tests::fault_storage_fail_nth_sync_fails_once_after_the_preceding_append",
    ];
    laws.push(
      ...[
        "single_segment_write_commit_flush_recovers_cleanly",
        "group_commit_batches_until_policy_threshold_then_commits",
        "fsync_durability_forces_immediate_commit_regardless_of_policy",
        "torn_tail_is_recovered_by_truncating_only_the_uncommitted_suffix",
        "recovery_resumes_next_tx_id_and_accepts_further_submits",
        "multi_segment_rotation_chains_prev_hash_and_replay_spans_segments",
        "recovery_rejects_a_torn_non_active_sealed_segment",
        "empty_document_open_creates_a_fresh_wal",
      ].map((law) => `db_wal::tests::${law}`),
    );
    const faultStorageLawsSource = readFileSync(join(owner, "../🧪️tests/🧯️fault-storage-laws/🦀️.rs"), "utf8");
    for (const law of laws) assert((law.startsWith("db_fault_testing::") ? faultStorageLawsSource : source).includes(`fn ${law.split("::").at(-1)}(`), `missing exact native law ${law}`);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, laws }],
        progress(event) {
          console.log(`wal-recovery ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`wal-recovery-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`wal-recovery-check: ${fixture.cuts.length + fixture.lifecycle.length + fixture.fragmentCopies.length + failStop.cases.length} checks clean`);
  }
}

/** 📏️ Proves complete transaction reservations fit the shared readable storage span. */
class WalCapacityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-capacity-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/📏️capacity/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.wal", "SegmentCapacityV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    const leb = await import("@webassemblyjs/leb128");
    const frame = (payload: number) => leb.encodeU32(payload + 2).length + payload + 10;
    const txn = (payload: number) => frame(8) + frame(payload) + frame(12);
    for (const row of fixture.cases) {
      const lengths = [129];
      const segments: number[] = [];
      let pending = false;
      for (let index = 0; index < 3; index++) {
        if (lengths.at(-1)! + txn(fixture.payloadBytes) + 75 > fixture.maxSegmentBytes) {
          if (pending) lengths[lengths.length - 1] += 75;
          lengths.push(161);
          pending = false;
        }
        segments.push(lengths.length - 1);
        lengths[lengths.length - 1] += txn(fixture.payloadBytes);
        pending = true;
        if (row.durability === "fsync") {
          lengths[lengths.length - 1] += 75;
          pending = false;
        }
      }
      if (pending) lengths[lengths.length - 1] += 75;
      assert.deepEqual(segments, row.segments);
      assert.deepEqual(lengths, row.lengths);
    }
    assert.equal(129 + txn(fixture.exactPayloadBytes) + 75, fixture.maxSegmentBytes);
    assert.equal(129 + txn(fixture.oversizedPayloadBytes) + 75, fixture.maxSegmentBytes + 1);
    console.log("wal-capacity-independent-oracle: Fsync/grouped rotation and exact/one-over capacity confirmed by LEB128");
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(source.includes("fn wal_transaction_frame_bytes("), "missing transaction byte preflight before writes");
    assert(source.includes("const DEFAULT_MAX_SEGMENT_BYTES: u64 = db_storage::DB_IO_MAX_READ_BYTES;"), "WAL and storage must share one byte ceiling");
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, laws: ["db_wal::tests::wal_capacity_preflight_matches_neutral_memory_and_filesystem_boundaries"] }],
        progress(event) {
          console.log(`wal-capacity ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`wal-capacity-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log("wal-capacity-check: 6 checks clean");
  }
}

//#region 🔎️ScalarWireSource
class ScalarWireSourceScript extends BundleScript {
  async run(): Promise<void> {
    const { testScalarRecordWireFixture } = await import("../../🔨️modules/🎒️pack/🧪️tests/🔎️scalar-witness/🟦️.ts");
    testScalarRecordWireFixture();
  }
}
//#endregion 🔎️ScalarWireSource

class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

/** 🏛️ Verifies shared policy and mutation publication authority through exact native laws. */
class CanonicalArchitectureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("canonical-architecture accepts no arguments");
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [{
        package: "semio-framework-os-kernel",
        target: { kind: "lib", name: "semio_framework_os_kernel" },
        laws: [
          "plugin_module_schema_exports_match_declared_formats",
          "declared_access_policy_matches_the_language_neutral_truth_table",
          "access_policy_is_closed_by_default_and_deny_overrides_allow",
          "one_item_publication_fixture_matches_the_third_party_json_oracle",
          "artifact_snapshot_root_is_o1_and_generation_stable_until_the_next_event",
          "presence_local_read_is_o1_and_never_clones_the_payload_at_capture",
          "transient_root_is_o1_and_retains_the_exact_pre_reset_value",
          "artifact_store_batch_publication_of_one_mutation_is_the_single_item_case",
          "artifact_store_batch_publication_stages_two_hundred_mutations_into_one_ledger_slot_and_one_undo_step",
          "artifact_store_batch_cancel_mid_flight_retires_every_staged_owner_without_publishing",
          "artifact_store_one_item_digest_helper_matches_validation_and_rejects_forged_cursor_history",
          "artifact_store_one_item_stale_saturation_and_cancel_leave_root_generation_and_revision_unchanged",
          "retained_member_publication_rejects_wrong_owner_staleness_and_cancels_without_commit",
        ],
      }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log("canonical-architecture-native " + event.stage + ": " + (event.law ?? ""));
      },
    });
    console.log("canonical-architecture-native receipts=" + receipts.length);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...segments], this.root);
  }
}

class ListRecordNativeTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-list-record-boundaries accepts no arguments");await runRepositoryCargoTests(["semio-framework-os-kernel"],this.repoRoot,["--lib","--no-fail-fast","list_record_"]);}
}

class ComposedPackSchemaTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-composed-pack-schema accepts no arguments");await runRepositoryCargoTests(["semio-framework-os-kernel"],this.repoRoot,["--lib","composed_pack_schema_tests::"]);}
}

class IeeePayloadNativeTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-ieee-payload-native accepts no arguments");await runRepositoryCargoTests(["semio-framework-os-kernel"],this.repoRoot,["--lib","ieee_payload_"]);}
}

class IeeePayloadSourceTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-ieee-payload-source accepts no arguments");const root=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🔢️ieee754");await runRepositoryTestCommand(process.execPath,["x","--no-install","tsc","--project",join(root,"🧪️tests/📋️tsconfig.json")],{cwd:this.repoRoot});await runRepositoryTestCommand(process.execPath,["test",join(root,"🧪️tests/🟦️.ts")],{cwd:this.repoRoot});}
}

//#region 🧬️RetainedCloneFixtures
/** 🎭️ One `choice` arm of `🧬️retained-clone/🧫️fixtures/📦️nested/🧬️schema/🔣️.json` (`#/$defs/choice`). */
type RetainedCloneChoiceV1 =
  | { readonly kind: "unit" }
  | { readonly kind: "text"; readonly text: string }
  | { readonly kind: "nested"; readonly rows: readonly { readonly id: number; readonly text: string }[] };

/** 🎟️ The copy grant a retained-clone corpus hands one step. */
type RetainedCloneGrantV1 = { readonly maximumItems: number; readonly maximumCopyBytes: number; readonly maximumCapacityBytes: number; readonly maximumDepth: number };

/** 📦️ `🧬️retained-clone/🧫️fixtures/📦️nested` — the nested-value clone corpus its schema admits. */
type RetainedCloneNestedFixtureV1 = {
  readonly source: {
    readonly title: string;
    readonly optional: string | null;
    readonly choice: RetainedCloneChoiceV1;
    readonly choices: readonly RetainedCloneChoiceV1[];
    readonly fixedArray: readonly [number, number, number, number];
    readonly pair: readonly [string, number];
    readonly triple: readonly [string, number, string];
    readonly labels: Readonly<Record<string, string>>;
  };
  readonly payloadByteLength: number;
  readonly payloadModulo: number;
  readonly recursiveDepth: number;
  readonly grant: RetainedCloneGrantV1;
  readonly insufficientCapacityBytes: number;
  readonly immutableLease: { readonly captured: string; readonly externalAfterCapture: string; readonly expected: string };
  readonly cancellationStops: readonly number[];
};

/** 🎯️ The state an ordered-map operation row must leave behind. */
type RetainedOrderedMapExpectationV1 = { readonly found: boolean; readonly ordinal: number; readonly entryCount: number };

/** 🗺️ `🗺️ordered-map/🧫️fixtures/📦️paging` — a lookup carries no value; an insert or duplicate carries the value it offers. */
type RetainedOrderedMapOperationV1 =
  | { readonly kind: "lookup"; readonly key: string; readonly expected: RetainedOrderedMapExpectationV1 }
  | { readonly kind: "insert" | "duplicate"; readonly key: string; readonly value: string; readonly expected: RetainedOrderedMapExpectationV1 };

/** 🗺️ `🗺️ordered-map/🧫️fixtures/📦️paging` — the paging, growth and immutable-lookup corpus its schema admits. */
type RetainedOrderedMapPagingFixtureV1 = {
  readonly pageCapacity: 16;
  readonly entryCount: number;
  readonly keyPrefix: string;
  readonly valuePrefix: string;
  readonly longKeyByteLength: number;
  readonly comparisonGrant: { readonly maximumItems: number; readonly maximumBytes: number };
  readonly progressChannels: { readonly comparisonOnly: true; readonly capacityOnly: true; readonly minimumMovedItems: number };
  readonly repeatedGrowth: { readonly entryCount: 32; readonly insertions: 17; readonly keyPrefix: string; readonly valuePrefix: string; readonly expectedEntryCount: 49 };
  readonly immutableLookup: { readonly capturedTarget: string; readonly externalAfterCapture: string; readonly expectedOrdinal: number };
  readonly operations: readonly RetainedOrderedMapOperationV1[];
};

/** 🧩️ `🧩preparation/🧪️fixtures/📦️lifecycle` — one preparation outcome row. */
type RetainedClonePreparationCaseV1 = {
  readonly id: string;
  readonly kind: "success" | "rejection" | "cancel" | "stale" | "fault" | "overBudget";
  readonly initial: number;
  readonly value: number;
  readonly interruptAfterTurns: number;
  readonly expected: { readonly published: boolean; readonly value: number; readonly history: number; readonly terminalEmpty: true };
};

/** 🧩️ `🧩preparation/🧪️fixtures/📦️lifecycle` — the preparation lifecycle corpus its schema admits. */
type RetainedClonePreparationFixtureV1 = {
  readonly grant: { readonly maximumItems: 1; readonly maximumBytes: number; readonly maximumDepth: number };
  readonly largeCapacity: { readonly stringByteLength: number; readonly expectedCode: "retained-clone.step-grant-too-small" };
  readonly cases: readonly RetainedClonePreparationCaseV1[];
};

/** 📋️ `📋️paged-list/🧫️fixtures/📦️copy` — the paged-list copy corpus its schema admits. */
type RetainedPagedListCopyFixtureV1 = {
  readonly maximumEntries: 1024;
  readonly entryCount: 513;
  readonly valuePrefix: string;
  readonly grant: { readonly maximumItems: 5; readonly maximumCopyBytes: 32; readonly maximumCapacityBytes: 4096; readonly maximumDepth: 64 };
  readonly cancellationAfterEntries: 173;
  readonly expected: { readonly ordered: true; readonly sourcePreserved: true; readonly copyRequiresMultipleTurns: true; readonly closeRequiresMultipleTurns: true; readonly terminalEmpty: true };
};
//#endregion 🧬️RetainedCloneFixtures

/** 🧬️ Validates the retained-clone resource contract and neutral corpus with Ajv and the platform structured-clone oracle. */
class RetainedCloneCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length) throw new Error("retained-clone-check accepts no arguments");
    const root = join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧫️fixtures/📦️nested");
    const schema = JSON.parse(readFileSync(join(root, "🧬️schema/🔣️.json"), "utf8"));
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile<RetainedCloneNestedFixtureV1>(schema);
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(structuredClone(fixture), fixture);
    assert.equal(fixture.source.choices.length, 3);
    assert.equal(fixture.source.fixedArray.length, 4);
    assert(fixture.grant.maximumDepth >= fixture.recursiveDepth);
    const capturedLease = structuredClone(fixture.immutableLease.captured);
    let externalLease = fixture.immutableLease.captured;
    externalLease = fixture.immutableLease.externalAfterCapture;
    assert.equal(capturedLease, fixture.immutableLease.expected);
    assert.notEqual(externalLease, capturedLease);
    assert(fixture.cancellationStops.some((stop: number) => stop > fixture.payloadByteLength / fixture.grant.maximumCopyBytes));
    const mapRoot = join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🧫️fixtures/📦️paging");
    const mapSchema = JSON.parse(readFileSync(join(mapRoot, "🧬️schema/🔣️.json"), "utf8"));
    const mapFixture = JSON.parse(readFileSync(join(mapRoot, "🔣️.json"), "utf8"));
    const validateMap = new Ajv2020({ strict: true, allErrors: true }).compile<RetainedOrderedMapPagingFixtureV1>(mapSchema);
    assert(validateMap(mapFixture), JSON.stringify(validateMap.errors));
    const entries = Array.from({ length: mapFixture.entryCount }, (_, ordinal) => [`${mapFixture.keyPrefix}${ordinal.toString().padStart(4, "0")}`, `${mapFixture.valuePrefix}${ordinal}`] as [string, string]);
    const oracle = new Map(entries);
    const ordered = [...oracle.entries()].sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0);
    const lowerBound = (key: string): number => {
      let low = 0;
      let high = ordered.length;
      while (low < high) {
        const middle = low + Math.floor((high - low) / 2);
        if (ordered[middle][0] < key) low = middle + 1;
        else high = middle;
      }
      return low;
    };
    for (const operation of mapFixture.operations) {
      const ordinal = lowerBound(operation.key);
      const found = ordinal < ordered.length && ordered[ordinal][0] === operation.key;
      if (operation.kind === "insert") ordered.splice(ordinal, 0, [operation.key, operation.value]);
      assert.equal(operation.kind === "insert" ? true : found, operation.expected.found, operation.kind);
      assert.equal(ordinal, operation.expected.ordinal, operation.kind);
      assert.equal(ordered.length, operation.expected.entryCount, operation.kind);
    }
    const longKey = "k".repeat(mapFixture.longKeyByteLength);
    assert.equal(new TextEncoder().encode(longKey).byteLength, mapFixture.longKeyByteLength);
    assert(longKey < `${longKey}z`);
    assert.equal(mapFixture.progressChannels.comparisonOnly, true);
    assert.equal(mapFixture.progressChannels.capacityOnly, true);
    assert(mapFixture.progressChannels.minimumMovedItems >= 1);
    const growth = new Map(Array.from({ length: mapFixture.repeatedGrowth.entryCount }, (_, ordinal) => [`${mapFixture.repeatedGrowth.keyPrefix}${(ordinal * 2).toString().padStart(4, "0")}`, `${mapFixture.repeatedGrowth.valuePrefix}${ordinal}`]));
    for (let ordinal = 0; ordinal < mapFixture.repeatedGrowth.insertions; ordinal += 1) growth.set(`${mapFixture.repeatedGrowth.keyPrefix}${(ordinal * 2 + 1).toString().padStart(4, "0")}`, `${mapFixture.repeatedGrowth.valuePrefix}insert-${ordinal}`);
    assert.equal(growth.size, mapFixture.repeatedGrowth.expectedEntryCount);
    const capturedTarget = structuredClone(mapFixture.immutableLookup.capturedTarget);
    let externalTarget = mapFixture.immutableLookup.capturedTarget;
    externalTarget = mapFixture.immutableLookup.externalAfterCapture;
    assert.equal(lowerBound(capturedTarget), mapFixture.immutableLookup.expectedOrdinal);
    assert.notEqual(externalTarget, capturedTarget);
    const preparationRoot = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🧪️fixtures/📦️lifecycle");
    const preparationSchema = JSON.parse(readFileSync(join(preparationRoot, "🧬️schema/🔣️.json"), "utf8"));
    const preparationFixture = JSON.parse(readFileSync(join(preparationRoot, "🔣️.json"), "utf8"));
    const validatePreparation = new Ajv2020({ strict: true, allErrors: true }).compile<RetainedClonePreparationFixtureV1>(preparationSchema);
    assert(validatePreparation(preparationFixture), JSON.stringify(validatePreparation.errors));
    assert(preparationFixture.largeCapacity.stringByteLength > preparationFixture.grant.maximumBytes);
    assert.equal(preparationFixture.largeCapacity.expectedCode, "retained-clone.step-grant-too-small");
    for (const row of preparationFixture.cases) {
      let value = row.initial;
      let history = 0;
      if (row.kind === "success") {
        value = row.value;
        history = 1;
      } else if (row.kind === "stale") {
        value += 1;
        history = 1;
      }
      assert.equal(value, row.expected.value, row.id);
      assert.equal(history, row.expected.history, row.id);
      assert.equal(row.expected.published, row.kind === "success", row.id);
      assert.equal(row.expected.terminalEmpty, true, row.id);
    }
    const pagedRoot = join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧫️fixtures/📦️copy");
    const pagedSchema = JSON.parse(readFileSync(join(pagedRoot, "🧬️schema/🔣️.json"), "utf8"));
    const pagedFixture = JSON.parse(readFileSync(join(pagedRoot, "🔣️.json"), "utf8"));
    const validatePaged = new Ajv2020({ strict: true, allErrors: true }).compile<RetainedPagedListCopyFixtureV1>(pagedSchema);
    assert(validatePaged(pagedFixture), JSON.stringify(validatePaged.errors));
    const pagedValues = Array.from({ length: pagedFixture.entryCount }, (_, ordinal) => `${pagedFixture.valuePrefix}${ordinal}`);
    const pagedOracle = structuredClone(pagedValues);
    assert.deepEqual(pagedOracle, pagedValues);
    assert.equal(pagedOracle.length, pagedFixture.entryCount);
    assert(pagedFixture.entryCount <= pagedFixture.maximumEntries);
    assert(pagedFixture.cancellationAfterEntries > 0 && pagedFixture.cancellationAfterEntries < pagedFixture.entryCount);
    assert(new TextEncoder().encode(pagedOracle.join("")).byteLength > pagedFixture.grant.maximumCapacityBytes);
    assert.equal(pagedFixture.expected.ordered, true);
    assert.equal(pagedFixture.expected.sourcePreserved, true);
    assert.equal(pagedFixture.expected.copyRequiresMultipleTurns, true);
    assert.equal(pagedFixture.expected.closeRequiresMultipleTurns, true);
    assert.equal(pagedFixture.expected.terminalEmpty, true);
    console.log(`retained-clone-check: choices=${fixture.source.choices.length} payload=${fixture.payloadByteLength} cancellation=${fixture.cancellationStops.length} orderedMap=${ordered.length} growth=${growth.size} preparation=${preparationFixture.cases.length} paged=${pagedOracle.length}`);
  }
}

/** 🪪️ Executes the native outer opening-attempt wire law without broadening the browser patch contract. */
class DocumentOpeningAttemptNativeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("document-opening-attempt-native-check accepts no arguments");
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [
        {
          package: "semio-framework-os-kernel",
          target: { kind: "lib" },
          cargoArgs: ["--features", "sync"],
          laws: ["os_store::sync::tests::document_opening_attempt_wire_preserves_outer_owner_without_widening_actor_messages"],
        },
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log(`document-opening-attempt-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    console.log(`document-opening-attempt-native-receipts: ${JSON.stringify(receipts)}`);
  }
}

class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-framework-os-kernel"], this.repoRoot, ["--lib", "--features", "sync,ureq", ...rest]);
  }
}

class SnapshotNativeAdmissionTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🚪️public/🧩️ownership/🟦️.ts"), join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/🟦️.ts"), join(this.repoRoot, "🧰️framework/🔨️modules/🗣️dsl/📖️grammar/📡️literal/🧪️tests/🟦️.ts"), join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🧪️tests/🚦️controlled/🟦️.ts"), join(this.repoRoot, "🧰️framework/🔨️modules/🚪️io/🧬️schema/🔗️reference/🧪️tests/🟦️.ts")], { cwd: this.repoRoot });
    if (segments.length === 1 && segments[0] === "portable") return;
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-framework-os-kernel"], this.repoRoot, ["--test", "sqlite_snapshot_native_admission", ...rest, ...(rest.includes("--no-fail-fast") ? [] : ["--no-fail-fast"])]);
  }
}

class DirectoryRuntimeSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-directory-runtime-source accepts no arguments");
    const { testDirectoryRuntimeIdentityFixture } = await import("../../🔨️modules/📇️directory/🧪️tests/🪪️runtime-identity/🟦️.ts");
    testDirectoryRuntimeIdentityFixture();
  }
}

/** 🪪️ Proves the broker-visible identity is canonically bound to one exact server session. */
export async function directorySessionAuthorityOracle(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1");
  const schema = JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validate: SchemaCheck = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
  const contract = await import("../../🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🟦️.ts");
  for (const row of fixture.rows) {
    assert.equal(validate(row.value), row.accepted, `${row.id}: ${JSON.stringify(validate.errors)}`);
    let accepted = true;
    try { contract.parseDirectorySessionAuthorityJsonV1(JSON.stringify(row.value)); } catch { accepted = false; }
    assert.equal(accepted, row.accepted, `${row.id}: TypeScript`);
  }
  for (const row of fixture.raw) {
    let accepted = true;
    try { contract.parseDirectorySessionAuthorityJsonV1(row.source); } catch { accepted = false; }
    assert.equal(accepted, row.accepted, `${row.id}: canonical`);
  }
  for (const row of fixture.bindingGoldens) {
    const hash = createHash("sha256");
    const length = (value: string) => { const bytes = Buffer.alloc(4); bytes.writeUInt32BE(Buffer.byteLength(value)); return bytes; };
    const generation = Buffer.alloc(8);
    const expiresAt = Buffer.alloc(8);
    generation.writeBigUInt64BE(BigInt(row.authorizationGeneration));
    expiresAt.writeBigInt64BE(BigInt(row.expiresAt));
    hash.update("semio/hub/directory-event-page/session-binding/v1\0");
    hash.update(length(row.sessionId));
    hash.update(row.sessionId);
    hash.update(length(row.userId));
    hash.update(row.userId);
    hash.update(generation);
    hash.update(expiresAt);
    assert.equal(hash.digest("hex"), row.sessionBindingSha256, row.id);
  }
  for (const row of fixture.authorityLifecycle) {
    assert(["installed", "retained", "replaced", "ignored", "retired"].includes(row.outcome), `${row.id}: outcome`);
    assert.equal(row.epochDelta === 1, row.retireMountedAuthorities, `${row.id}: retirement epoch`);
    if (row.outcome === "replaced" || row.outcome === "retired") assert.equal(row.retainIndeterminateJobs, true, `${row.id}: uncertain jobs`);
    if (!row.brokerAdmissionCurrent) assert.equal(row.outcome, "ignored", `${row.id}: stale admission`);
  }
  const rust = readFileSync(join(root, "🦀️.rs"), "utf8");
  const client = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8");
  const hub = readFileSync(join(repoRoot, "🌎️hub/🏗️bootstrap/🦀️.rs"), "utf8");
  assert(rust.includes("pub struct DirectorySessionAuthorityV1") && rust.includes("parse_canonical_json"), "Rust session authority contract missing");
  assert(client.includes("DirectorySessionAuthorityV1::parse_canonical_json") && client.includes("DIRECTORY_SESSION_AUTHORITY_MAX_BYTES"), "Rust client does not enforce canonical session authority");
  assert(hub.includes("directory_event_page_session_binding_v1(&caller)") && hub.includes("Json<DirectorySessionAuthorityV1>"), "Hub session response is not bound to the existing session digest");
  return fixture.rows.length + fixture.raw.length + fixture.bindingGoldens.length + fixture.authorityLifecycle.length + 3;
}

class DirectorySessionAuthorityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-session-authority-check accepts only --native");
    const checks = await directorySessionAuthorityOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [{
          package: "semio-framework-os-kernel",
          target: { kind: "lib" },
          laws: [
            "os_directory::schema::tests::directory_session_authority_v1_matches_neutral_corpus_and_binding_goldens",
            "os_directory::client::tests::session_authority_client_preserves_canonical_binding_and_rejects_reordered_body",
            "os_directory::schema::tests::inference_current_hub_wire_preserves_required_nullable_hash",
            "os_directory::schema::tests::inference_indeterminate_lifecycle_matches_neutral_corpus",
            "os_directory::client::tests::inference_client_refuses_substituted_hub_receipt_and_page_coordinates",
          ],
        }],
      });
      console.log(`directory-session-authority-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-session-authority-check: checks=${checks} clean`);
  }
}

/** 📃️ Proves the shared event-page envelope against an independent JSON Schema and SHA-256 oracle. */
export async function directoryEventPageContractOracle(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📃️event-page-v1.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"), "utf8"));
  const validator: SchemaCheck = semioSchemaAjvV1({ strict: false, allErrors: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/DirectoryEventPageV1" });
  assert(validator(fixture.valid), JSON.stringify(validator.errors));
  assert.equal(new TextEncoder().encode(fixture.canonicalUnsigned).length, 474);
  assert.equal(createHash("sha256").update(fixture.canonicalUnsigned).digest("hex"), fixture.expectedReceiptSha256);
  const contract = await import("../../🔨️modules/📇️directory/🧬️schema/🟦️.ts");
  const parsed = await contract.parseDirectoryEventPageV1(JSON.stringify(fixture.valid));
  assert.deepEqual(parsed, fixture.valid);
  const setPath = (value: any, path: string, replacement: unknown): any => {
    const copy = structuredClone(value);
    const parts = path.split(".");
    let parent = copy;
    for (const part of parts.slice(0, -1)) parent = parent[Number.isInteger(Number(part)) ? Number(part) : part];
    parent[parts.at(-1)!] = replacement;
    return copy;
  };
  for (const hostile of fixture.hostileMutations) await assert.rejects(() => contract.parseDirectoryEventPageV1(JSON.stringify(setPath(fixture.valid, hostile.path, hostile.value))), hostile.name);
  const canonical = JSON.stringify(fixture.valid);
  await assert.rejects(() => contract.parseDirectoryEventPageV1(`${canonical} `), "trailing-byte");
  await assert.rejects(() => contract.parseDirectoryEventPageV1(canonical.replace('{"schema":', '{"schema":"duplicate","schema":')), "duplicate-key");
  const rust = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs"), "utf8");
  assert(rust.includes("pub struct DirectoryEventPageV1") && rust.includes("pub fn receipt_matches(&self) -> bool"), "Rust event-page contract missing");
  return 5 + fixture.hostileMutations.length + fixture.rawHostiles.length;
}

/** 🔌️ Proves both directory clients preserve one canonical event-page response and its bounded header. */
export async function directoryEventPageClientOracle(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📃️event-page-v1.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"), "utf8"));
  const validator: SchemaCheck = semioSchemaAjvV1({ strict: false, allErrors: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/DirectoryEventPageV1" });
  const canonical = JSON.stringify(fixture.valid);
  const accept = (raw: string, after: number) => {
    if (!Number.isSafeInteger(after) || after < 0 || new TextEncoder().encode(raw).byteLength > 65_536) throw new Error("client admission");
    const parsed = JSON.parse(raw);
    if (JSON.stringify(parsed) !== raw || !validator(parsed)) throw new Error("client admission");
    if (parsed.afterSeqExclusive !== after) throw new Error("frontier substitution");
    const { receiptSha256, ...unsigned } = parsed;
    if (createHash("sha256").update(JSON.stringify(unsigned)).digest("hex") !== receiptSha256) throw new Error("receipt substitution");
    return { canonicalJson: raw, throughSeqInclusive: parsed.throughSeqInclusive, receiptSha256: parsed.receiptSha256 };
  };
  const page = accept(canonical, 3);
  assert.equal(page.canonicalJson, canonical);
  assert.equal(page.throughSeqInclusive, fixture.valid.throughSeqInclusive);
  assert.equal(page.receiptSha256, fixture.expectedReceiptSha256);
  assert.throws(() => accept("x".repeat(65_537), 0));
  assert.throws(() => accept(canonical, 4));
  assert.throws(() => accept(canonical, -1));
  assert.throws(() => accept(`${canonical} `, 3));
  const typescript = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🟦️.ts"), "utf8");
  const method = typescript.slice(typescript.indexOf("async eventPage("), typescript.indexOf("stream(since", typescript.indexOf("async eventPage(")));
  assert(
    method.includes("response.text()") && method.includes("parseDirectoryEventPageV1(canonicalJson)") && method.includes("page.afterSeqExclusive !== after") && !method.includes("response.json()"),
    "TypeScript canonical page transport is incomplete",
  );
  assert(typescript.includes("streamAcknowledged(since:") && typescript.includes("acknowledge: (through: number)"), "TypeScript acknowledged directory frontier is missing");
  const rust = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8");
  assert(rust.includes("pub async fn event_page") && rust.includes("CanonicalDirectoryEventPageV1") && rust.includes("DIRECTORY_EVENT_PAGE_MAX_BYTES"), "Rust canonical page transport is incomplete");
  assert(rust.includes("pub fn stream_acknowledged") && rust.includes("pub fn acknowledge(&mut self, through: u64)"), "Rust acknowledged directory frontier is missing");
  assert(rust.includes("pub struct DirectoryEventPageBootstrapV1") && rust.includes("pub enum DirectoryBootstrapTransition"), "Rust directory bootstrap owner is missing");
  return 11;
}

class DirectoryEventPageContractCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-event-page-contract-check accepts only --native");
    const checks = await directoryEventPageContractOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["os_directory::schema::tests::directory_event_page_v1_matches_language_neutral_receipt_and_rejects_hostiles"] }],
        progress(event) {
          console.log(`directory-event-page-contract ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`directory-event-page-contract-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-event-page-contract-check: checks=${checks} clean`);
  }
}

class DirectoryEventPageClientCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-event-page-client-check accepts only --native");
    const checks = await directoryEventPageClientOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["os_directory::client::tests::directory_event_page_preserves_canonical_bytes_bounds_and_cancels_before_io"] }],
        progress(event) {
          console.log(`directory-event-page-client ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`directory-event-page-client-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-event-page-client-check: checks=${checks} clean`);
  }
}

/** 🧭️ Proves fetch, exact Home ACK, next-page, and live-cursor ordering independently of either shell. */
export function directoryEventPageBootstrapOracle(repoRoot: string): number {
  const trace = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🚀️event-page-bootstrap-v1.json"), "utf8"));
    const validator = ownedExport(repoRoot, "directory", "DirectoryEventPageBootstrapTraceV1");
  assert(validator(trace), JSON.stringify(validator.errors));
  let cursor = trace.initialAfter;
  let pending: any = null;
  let live = false;
  const present = (page: any): void => {
    assert(!pending && !live && page.afterSeqExclusive === cursor && page.throughSeqInclusive >= cursor);
    pending = page;
  };
  const acknowledge = (page: any, epoch = trace.bootstrapEpoch): "fetch" | "live" => {
    assert(pending && epoch === trace.bootstrapEpoch);
    for (const key of ["receiptSha256", "sessionBindingSha256", "authorizationGeneration", "throughSeqInclusive"]) assert.equal(page[key], pending[key]);
    cursor = pending.throughSeqInclusive;
    const hasMore = pending.hasMore;
    pending = null;
    live = !hasMore;
    return hasMore ? "fetch" : "live";
  };
  present(trace.pages[0]);
  assert.throws(() => present(trace.pages[1]), "page 2 before ACK");
  assert.throws(() => acknowledge({ ...trace.pages[0], receiptSha256: "d".repeat(64) }), "forged ACK");
  assert.equal(cursor, trace.initialAfter);
  assert.equal(acknowledge(trace.pages[0]), "fetch");
  present(trace.pages[1]);
  assert.throws(() => acknowledge(trace.pages[1], trace.bootstrapEpoch + 1), "stale epoch");
  assert.equal(acknowledge(trace.pages[1]), "live");
  for (const wakeup of trace.wakeups) assert(wakeup > cursor && cursor === trace.expectedSocketSince);
  const worker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"), "utf8");
  assert(worker.includes("DirectoryEventPageBootstrapV1") && worker.includes("directory-bootstrap-ack") && worker.includes("directory-event-page"), "browser worker bootstrap owner missing");
  return 11;
}

class DirectoryEventPageBootstrapCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length) throw new Error("directory-event-page-bootstrap-check accepts no arguments");
    console.log(`directory-event-page-bootstrap-check: checks=${directoryEventPageBootstrapOracle(this.repoRoot)} clean`);
  }
}

/** 🚦️ Proves every WAL backend exposes the same read-only active/sealed contract. */
export function walSegmentStateOracle(repoRoot: string): number {
  const storageRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db");
  const core = readFileSync(join(storageRoot, "🗄️storage/🦀️.rs"), "utf8");
  const sqlite = readFileSync(join(storageRoot, "🗄️storage/🪶️sqlite/🦀️.rs"), "utf8");
  const postgres = readFileSync(join(storageRoot, "🗄️storage/🐘️postgres/🦀️.rs"), "utf8");
  const neo4j = readFileSync(join(storageRoot, "🗄️storage/🌐️neo4j/🦀️.rs"), "utf8");
  const faultStorage = readFileSync(join(storageRoot, "🧪️tests/🧯️fault-storage/🦀️.rs"), "utf8");
  assert(core.includes("pub enum WalSegmentState") && core.includes("Active,") && core.includes("Sealed,"));
  assert(core.includes("async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError>"));
  assert(core.includes("WalState { backend: DbIoBackendControl") && core.includes("WalSegmentState(WalSegmentState)"));
  assert(core.includes("Err(error) if error.kind() == std::io::ErrorKind::NotFound => WalSegmentState::Active") && core.includes("Err(error) => return Err(io_err(error))"));
  assert(sqlite.includes("SELECT sealed FROM wal_segment") && sqlite.includes("Err(DbError::Corrupt") && sqlite.includes("DbIoTask::WalState"));
  assert(postgres.includes("POSTGRES_WAL_STATE_QUERY") && postgres.includes("fetch_optional") && !postgres.slice(postgres.indexOf("const POSTGRES_WAL_STATE_QUERY"), postgres.indexOf("const POSTGRES_WAL_STATE_QUERY") + 240).includes("FOR UPDATE"));
  assert(neo4j.includes("const CYPHER_WAL_STATE") && neo4j.includes("RETURN n.sealed AS sealed") && !neo4j.slice(neo4j.indexOf("const CYPHER_WAL_STATE"), neo4j.indexOf("const CYPHER_WAL_STATE") + 240).includes("bytes"));
  const faultStorageLaws = readFileSync(join(storageRoot, "🧪️tests/🧯️fault-storage-laws/🦀️.rs"), "utf8");
  assert(faultStorage.includes("async fn segment_state") && faultStorageLaws.includes("fault_storage_segment_state_is_observational_and_counter_neutral"));
  return 12;
}

class WalSegmentStateCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("wal-segment-state-check accepts only --native");
    const checks = walSegmentStateOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        cargoArgs: ["--all-features"],
        groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib" },
            laws: [
              "memory_storage_satisfies_wal_storage_laws",
              "fs_storage_satisfies_wal_storage_laws",
              "fs_storage_stale_seal_marker_does_not_resurrect_missing_segment",
              "memory_storage_db_backend_accessors_and_capabilities",
              "wal_segment_state_observes_active_sealed_and_missing_rows",
              "wal_segment_state_decoder_rejects_non_boolean_storage_values",
              "wal_segment_state_query_and_mapper_are_read_only_and_byte_neutral",
              "wal_cypher_statements_reference_the_expected_label_and_keys",
              "fault_storage_segment_state_is_observational_and_counter_neutral",
            ],
          },
        ],
        progress(event) {
          console.log(`wal-segment-state ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`wal-segment-state-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`wal-segment-state-check: checks=${checks} clean`);
  }
}

class CodecSendSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-codec-send-source accepts no arguments");
    const { testNativeCodecSendFixture } = await import("../../🔨️modules/🏪️store/🧪️tests/📦️native-codec-send/🟦️.ts");
    testNativeCodecSendFixture();
  }
}

class GroupVisibilitySourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-group-visibility-source accepts no arguments");
    const { testGroupVisibilityFixtures } = await import("../../🔨️modules/🏪️store/🧪️tests/👁️group-visibility/🟦️.ts");
    testGroupVisibilityFixtures();
  }
}

class BackboneDetachSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-backbone-detach-source accepts no arguments");
    const { testBackboneDetachFixture } = await import("../../🔨️modules/🏪️store/🧪️tests/🔗️backbone-detach/🟦️.ts");
    testBackboneDetachFixture();
  }
}

class MemberDialectSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-member-dialect-source accepts no arguments");
    const { testMemberDialectFixture } = await import("../../🔨️modules/🏪️store/🧪️tests/🗣️member-dialect/🟦️.ts");
    testMemberDialectFixture();
    const { testFixtureProjectionRetirement } = await import("../../🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/🟦️.ts");
    testFixtureProjectionRetirement();
  }
}

class MemberDialectCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { testMemberDialectFixture } = await import("../../🔨️modules/🏪️store/🧪️tests/🗣️member-dialect/🟦️.ts");
    testMemberDialectFixture();
    const { testFixtureProjectionRetirement } = await import("../../🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/🟦️.ts");
    testFixtureProjectionRetirement();
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      cargoArgs: segments,
      buildBudgetMs: 3_600_000,
      groups: [
        {
          package: "semio-framework-schema",
          target: { kind: "lib" },
          laws: [
            "artifact_composition_fields_derive_emits_expected_slot_tables",
            "artifact_composition_fields_default_to_empty_for_leaf_artifacts",
            "artifact_composition_projection_walks_aliases_nested_options_and_cancels",
            "artifact_composition_projection_real_child_alias_has_fixed_admission_bounds",
          ],
        },
        {
          package: "semio-framework-os-kernel",
          target: { kind: "lib" },
          laws: [
            "member_factory_closed_dialect_matches_neutral_admission_corpus",
            "member_factory_closed_dialect_rejects_identity_and_owner_substitution",
            "member_factory_closed_dialect_graph_admission_matches_neutral_corpus",
            "member_factory_closed_dialect_graph_sync_preserves_prior_state_on_rejection",
            "member_factory_closed_dialect_parent_projection_matches_neutral_corpus",
            "initial_child_identity_matches_neutral_coordinates_and_blake3",
          ],
        },
        {
          package: "semio-framework-plugin",
          target: { kind: "lib" },
          laws: [
            "fixture_projection_retires_exact_tree_before_return_error_or_panic",
            "member_factory_parent_snapshot_restore_matches_neutral_corpus",
            "member_factory_closed_dialect_open_failure_retains_pin_and_drains_exact_member",
            "member_factory_closed_dialect_register_rejects_pin_without_mutating_member",
            "member_factory_closed_dialect_fresh_register_and_restore_publish_exact_parent_owner",
          ],
        },
      ],
    });
    console.log(`exact member admission laws: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed across ${receipts.length} verified test executables`);
  }
}

//#region 🧩️JCO Package Adapter
class GenerateJcoPackageAdapterScript extends BundleScript {
  run(): void {
    runNestedCargoPackageAdapter(this.repoRoot, "generate");
  }
}
class PreviewGeneratedScript extends BundleScript {
  run(): void {
    runNestedCargoPackageAdapter(this.repoRoot, "preview");
  }
}
class CheckJcoPackageAdapterScript extends BundleScript {
  run(): void {
    runNestedCargoPackageAdapter(this.repoRoot, "check");
  }
}
//#endregion 🧩️JCO Package Adapter

//#region 🌪️ReopenStorm
/** 🌪️ The reopen-storm laws of the db engine: a hub restart's reconnect storm, measured where the storage lives. `unit`
 * greets two dozen grown documents at once after a reopen (bound 30 s, the growth e2e's reopen bound); `fs`, `sqlite`,
 * `postgres` and `neo4j` are the throughput laws that grow, reopen alone and reopen as a storm and bound the storm against
 * the solo reopen (`🛢️db/⚙️engine/🧫️fixtures/⏱️throughput`). `postgres`/`neo4j` run against the ONE shared development
 * server claimed by `os-hub-ts backend run <postgres|neo4j> -- …` (its `OS_HUB_*` environment selects it), so they are
 * selected only by name and `all` never includes them. */
const REOPEN_STORM_LAWS: Readonly<Record<string, Readonly<{ law: string; features: string; claimed?: string }>>> = {
  unit: { law: "db_engine::tests::long::two_dozen_grown_documents_greeted_at_once_after_a_reopen_are_welcomed_within_the_reopen_bound", features: "sqlite" },
  fs: { law: "db_engine::throughput_tests::fs_commits_and_reopen_storms_stay_within_their_throughput_bounds", features: "sqlite" },
  sqlite: { law: "db_engine::throughput_tests::sqlite_commits_and_reopen_storms_stay_within_their_throughput_bounds", features: "sqlite" },
  postgres: { law: "db_engine::throughput_tests::postgres_commits_and_reopen_storms_stay_within_their_throughput_bounds", features: "sqlite,postgres", claimed: "OS_HUB_DATABASE_URL" },
  neo4j: { law: "db_engine::throughput_tests::neo4j_commits_and_reopen_storms_stay_within_their_throughput_bounds", features: "sqlite,neo4j", claimed: "OS_HUB_NEO4J_URI" },
};

/** 🌪️ `reopen-storm-check [unit|fs|sqlite|all]…` — runs each selected law alone in its own cargo test process (in place,
 * `SEMIO_DB_ISOLATED_LAW`), streams its output, reads the storm welcome distribution it prints and publishes the
 * acceptance record. Ctrl-C stops the running law. */
class ReopenStormCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const wanted = segments.length === 0 || segments.includes("all") ? Object.keys(REOPEN_STORM_LAWS).filter((name) => !REOPEN_STORM_LAWS[name]!.claimed) : segments;
    const unknown = wanted.filter((name) => !(name in REOPEN_STORM_LAWS));
    if (unknown.length) throw new Error(`reopen-storm-check accepts ${Object.keys(REOPEN_STORM_LAWS).join(" | ")} | all, got ${unknown.join(",")}`);
    const throughput = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧫️fixtures/⏱️throughput/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.engine", "ThroughputV1");
    assert(validate(throughput), `throughput fixture: ${JSON.stringify(validate.errors)}`);
    console.log(`[reopen-storm] throughput fixture valid (ThroughputV1, AJV): storm ${throughput.storm.documents}×${throughput.storm.batches}×${throughput.storm.batchEdits}, storm/serial ≤ ${throughput.bounds.stormToSerialRatioMax}`);
    const unclaimed = wanted.filter((name) => REOPEN_STORM_LAWS[name]!.claimed && !process.env[REOPEN_STORM_LAWS[name]!.claimed!]);
    if (unclaimed.length) throw new Error(`reopen-storm-check ${unclaimed.join(",")} needs the claimed shared server: run it under \`os-hub-ts backend run ${unclaimed[0]} -- …\``);
    const { acceptanceCheckResult, publishAcceptanceCheckResult, runLawProcess } = await import("../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts");
    const startedAt = new Date();
    const measured: Record<string, number | string | boolean> = {};
    const failed: string[] = [];
    let cancelled = false;
    for (const [index, name] of wanted.entries()) {
      if (cancelled) break;
      const { law, features, claimed } = REOPEN_STORM_LAWS[name]!;
      console.log(`[reopen-storm] ${index + 1}/${wanted.length} ${name}: ${law}`);
      const { status, lines } = await runLawProcess(
        "cargo",
        ["test", "-p", "semio-framework-os-kernel-db", "--features", features, "--lib", "--no-fail-fast", "--", "--exact", law, "--nocapture", "--test-threads=1", ...(claimed ? ["--include-ignored"] : [])],
        { cwd: this.repoRoot, env: { ...process.env, CARGO_INCREMENTAL: "0", RUST_MIN_STACK: "268435456", SEMIO_DB_ISOLATED_LAW: law } },
        () => {
          cancelled = true;
        },
      );
      const passed = status === 0 && lines.some((line) => line.includes("test result: ok. 1 passed"));
      measured[`${name}Pass`] = passed;
      const storm = lines.map((line) => /storm of (\d+) in (\d+) ms: welcome p50 ([\d.]+) ms .*? max ([\d.]+) ms(?:.*solo ([\d.]+) ms)?/u.exec(line)).find((match) => match !== null);
      if (storm) Object.assign(measured, { [`${name}Documents`]: Number(storm[1]), [`${name}StormMs`]: Number(storm[2]), [`${name}WelcomeP50Ms`]: Number(storm[3]), [`${name}WelcomeMaxMs`]: Number(storm[4]) }, storm[5] === undefined ? {} : { [`${name}SoloMs`]: Number(storm[5]) });
      if (!passed) failed.push(name);
    }
    const status = cancelled ? "skipped" : failed.length === 0 ? "pass" : "fail";
    publishAcceptanceCheckResult(
      this.repoRoot,
      acceptanceCheckResult({
        check: "hub-reopen-storm",
        status,
        startedAt,
        measured: { ...measured, laws: wanted.join(","), cancelled },
        summary: {
          en: `reopen storm laws ${wanted.length - failed.length}/${wanted.length} pass (${wanted.join(", ")})${failed.length ? `; failing: ${failed.join(", ")}` : ""}${cancelled ? "; cancelled" : ""}`,
          de: `Wiederöffnungssturm-Gesetze ${wanted.length - failed.length}/${wanted.length} bestanden (${wanted.join(", ")})${failed.length ? `; fehlgeschlagen: ${failed.join(", ")}` : ""}${cancelled ? "; abgebrochen" : ""}`,
        },
      }),
    );
    if (status !== "pass") process.exitCode = 1;
  }
}
//#endregion 🌪️ReopenStorm

/** 🌐️ Checks owner removal with neutral schema vectors and independent Ajv validation. */
class DocumentHttpCheckScript extends BundleScript {
  async run(): Promise<void> {
    const fixture = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🌐️document-http/🧫️fixtures/🔣️.json"), "utf8"));
    const ajv = semioSchemaAjvV1({ strict: false });
    const declarationSchema = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧬️schema/🔣️.json"), "utf8"));
    const declared = ajv.compile(declarationSchema);
    assert(declared(fixture.neutral));
    assert(declared(fixture.secondary));
    const replyBounds=ajv.compile(fixture.replyNodeBounds.oracleSchema);
    for(const vector of fixture.replyNodeBounds.vectors) assert.equal(replyBounds(Array(vector.items).fill(null)),vector.valid);
    const validate = ajv.compile(JSON.parse(fixture.neutral.operations[0].inputSchema));
    for (const vector of fixture.vectors) assert.equal(validate(vector.value), vector.valid, vector.name);
    for (const law of ["os_directory::client::document_http::tests::owner_removal_preserves_neutral_document_transport", "os_directory::client::document_http::tests::schema_vectors_match_owned_validator", "os_directory::client::document_http::tests::decoded_replies_obey_the_same_node_bounds_as_owner_inputs"]) {
      await runRepositoryCargoTests(["semio-framework-os-kernel"], this.root, ["--lib", "--", law, "--exact", "--nocapture"]);
    }
    await runRepositoryCargoTests(["semio-framework-os-kernel"], this.root, ["--lib", "--", "os_directory::client::tests::document_http_transport_preserves_scope_bounds_and_owner_decode", "--exact"]);
    console.log("document-http: Ajv vectors, removal/reinstall, scope, bounds and owner decoding passed");
  }
}


const router = new ScriptRouter(import.meta.dir)
  .register("check", CheckScript)
  .register("retained-clone-check", RetainedCloneCheckScript)
  .register("test", TestScript)
  .register("test-list-record-boundaries", ListRecordNativeTestScript)
  .register("test-composed-pack-schema", ComposedPackSchemaTestScript)
  .register("test-ieee-payload-native", IeeePayloadNativeTestScript)
  .register("test-ieee-payload-source", IeeePayloadSourceTestScript)
  .register("canonical-architecture", CanonicalArchitectureScript)
  .register("document-opening-attempt-native-check", DocumentOpeningAttemptNativeCheckScript)
  .register("test-scalar-wire-source", ScalarWireSourceScript)
  .register("generate-jco-package-adapter", GenerateJcoPackageAdapterScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-jco-package-adapter", CheckJcoPackageAdapterScript)
  .register("test-native", NativeTestScript)
  .register("test-snapshot-native-admission", SnapshotNativeAdmissionTestScript)
  .register("test-directory-runtime-source", DirectoryRuntimeSourceScript)
  .register("directory-session-authority-check", DirectorySessionAuthorityCheckScript)
  .register("directory-event-page-contract-check", DirectoryEventPageContractCheckScript)
  .register("directory-event-page-client-check", DirectoryEventPageClientCheckScript)
  .register("directory-event-page-bootstrap-check", DirectoryEventPageBootstrapCheckScript)
  .register("wal-segment-state-check", WalSegmentStateCheckScript)
  .register("test-codec-send-source", CodecSendSourceScript)
  .register("test-group-visibility-source", GroupVisibilitySourceScript)
  .register("test-backbone-detach-source", BackboneDetachSourceScript)
  .register("test-member-dialect-source", MemberDialectSourceScript)
  .register("member-dialect-check", MemberDialectCheckScript);

router.register("document-http-check", DocumentHttpCheckScript);
router.register("paged-history-stack-check", PagedHistoryStackScript);
router.register("wal-recovery-check", WalRecoveryCheckScript);
router.register("wal-capacity-check", WalCapacityCheckScript);
router.register("wal-committed-transactions-check", WalCommittedTransactionsCheckScript);
router.register("wal-writer-authority-check", WalWriterAuthorityCheckScript);
router.register("wal-writer-fence-live", WalWriterFenceLiveScript);
router.register("postgres-round-trips-live", PostgresRoundTripsLiveScript);
router.register("database-history-completion-check", DatabaseHistoryCompletionCheckScript);
router.register("database-catalog-read-ownership-check", DatabaseCatalogReadOwnershipCheckScript);
router.register("database-capability-completion-check", DatabaseCapabilityCompletionCheckScript);
router.register("wal-committed-compaction-check", WalCommittedCompactionCheckScript);
router.register("database-shutdown-check", DatabaseShutdownCheckScript);
router.register("document-mount-single-flight-check", DocumentMountSingleFlightCheckScript);
router.register("durable-owned-group-decision-check", DurableOwnedGroupDecisionCheckScript);
router.register("durable-group-journal-check", DurableGroupJournalCheckScript);
router.register("reopen-storm-check", ReopenStormCheckScript);

/** 🧱️ Runs the retained owned fixture law against its independent oracle. */
class DirectoryLeaseFixtureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-directory-lease-fixture accepts no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🔏️document-execution-target-lease-v1/🟦️.ts")], { cwd: this.repoRoot, env: process.env, budgetMs: 15_000, throwOnFailure: true });
  }
}

router.register("test-directory-lease-fixture", DirectoryLeaseFixtureScript);


/** 🧱️ Verifies actual OS composition and full original DSL/diagnostic source-law preservation. */
class DslArchitectureProofScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-dsl-architecture accepts no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧱️architecture/🟦️.ts"), join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧬️record-floor/🟦️.ts")], {cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true});
  }
}

router.register("test-dsl-architecture", DslArchitectureProofScript);

await runScriptMain(router, { defaultCommand: "check" });

````

## 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json

SHA256 dd74444f26a02be084c5f5abd75b3d1ee3a8e6fa8e548eeecc8670ce10ba4a92

````text
{
  "name": "@semio-tech/framework-os-kernel",
  "root": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
  "sourceRoot": "🧰️framework/🛍️products/💻️os",
  "projectType": "library",
  "tags": [
    "lang:rust",
    "role:framework",
    "family:os-kernel"
  ],
  "targets": {
    "retained-clone-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun 📜️script.ts retained-clone-check",
        "cwd": "{projectRoot}"
      },
      "outputs": []
    },
    "document-opening-attempt-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts document-opening-attempt-native-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "database-history-completion-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts database-history-completion-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "database-history-completion-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts database-history-completion-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "database-catalog-read-ownership-check": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun ./📜️script.ts database-catalog-read-ownership-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "database-catalog-read-ownership-native-check": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun ./📜️script.ts database-catalog-read-ownership-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "database-capability-completion-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts database-capability-completion-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "database-capability-completion-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts database-capability-completion-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "reopen-storm-check": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun ./📜️script.ts reopen-storm-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "database-shutdown-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts database-shutdown-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "database-shutdown-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts database-shutdown-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "document-mount-single-flight-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts document-mount-single-flight-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "document-mount-single-flight-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts document-mount-single-flight-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "durable-owned-group-decision-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts durable-owned-group-decision-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "durable-owned-group-decision-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts durable-owned-group-decision-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "durable-group-journal-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts durable-group-journal-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "durable-group-journal-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts durable-group-journal-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "wal-committed-compaction-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts wal-committed-compaction-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "wal-committed-compaction-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts wal-committed-compaction-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "wal-committed-transactions-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts wal-committed-transactions-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "wal-committed-transactions-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts wal-committed-transactions-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "wal-writer-authority-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts wal-writer-authority-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "wal-writer-authority-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts wal-writer-authority-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "wal-writer-fence-live": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun ./📜️script.ts wal-writer-fence-live",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "wal-capacity-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts wal-capacity-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "wal-capacity-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts wal-capacity-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "wal-recovery-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts wal-recovery-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "wal-recovery-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts wal-recovery-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "directory-session-authority-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts directory-session-authority-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "directory-session-authority-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts directory-session-authority-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "directory-event-page-contract-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts directory-event-page-contract-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      }
    },
    "directory-event-page-contract-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts directory-event-page-contract-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      }
    },
    "directory-event-page-client-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts directory-event-page-client-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "directory-event-page-client-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts directory-event-page-client-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "directory-event-page-bootstrap-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts directory-event-page-bootstrap-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "wal-segment-state-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts wal-segment-state-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "wal-segment-state-native-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts wal-segment-state-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "test-directory-runtime-source": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun ./📜️script.ts test-directory-runtime-source",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
        "forwardAllArgs": true
      }
    },
    "test-codec-send-source": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun ./📜️script.ts test-codec-send-source",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
        "forwardAllArgs": true
      }
    },
    "test-backbone-detach-source": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun ./📜️script.ts test-backbone-detach-source",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
        "forwardAllArgs": true
      }
    },
    "test-group-visibility-source": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun ./📜️script.ts test-group-visibility-source",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
        "forwardAllArgs": true
      }
    },
    "member-dialect-check": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts member-dialect-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "test-member-dialect-source": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts test-member-dialect-source",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "test-native": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun ./📜️script.ts test-native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
        "forwardAllArgs": true
      }
    },
    "test-snapshot-native-admission": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun ./📜️script.ts test-snapshot-native-admission",
        "cwd": "{projectRoot}",
        "forwardAllArgs": true
      }
    },
    "test-scalar-wire-source": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun ./📜️script.ts test-scalar-wire-source",
        "cwd": "{projectRoot}"
      }
    },
    "generate-jco-package-adapter": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts generate-jco-package-adapter",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "preview-generated": {
      "executor": "nx:run-commands",
      "cache": true,
      "outputs": [],
      "options": {
        "command": "bun ./📜️script.ts preview-generated",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      }
    },
    "check-jco-package-adapter": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "command": "bun ./📜️script.ts check-jco-package-adapter",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "check": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun 📜️script.ts check",
        "forwardAllArgs": true,
        "cwd": "{projectRoot}"
      }
    },
    "test": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun 📜️script.ts test",
        "forwardAllArgs": true,
        "cwd": "{projectRoot}"
      }
    },
    "canonical-architecture": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun ./📜️script.ts canonical-architecture",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "outputs": []
    },
    "test-list-record-boundaries": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun ./📜️script.ts test-list-record-boundaries",
        "cwd": "{projectRoot}"
      }
    },
    "test-ieee-payload-native": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun ./📜️script.ts test-ieee-payload-native",
        "cwd": "{projectRoot}"
      }
    },
    "test-ieee-payload-source": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun ./📜️script.ts test-ieee-payload-source",
        "cwd": "{projectRoot}"
      }
    },
    "paged-history-stack-check": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun ./📜️script.ts paged-history-stack-check",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "metadata": {
        "workspaceCommand": [
          "paged-history-stack-check"
        ]
      }
    },
    "paged-history-stack-native-check": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun ./📜️script.ts paged-history-stack-check --native",
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
      },
      "metadata": {
        "workspaceCommand": [
          "paged-history-stack-check",
          "--native"
        ]
      }
    },
    "document-http-check": {
      "executor": "nx:run-commands",
      "cache": false,
      "dependsOn": [],
      "inputs": [
        "nativeTestSources",
        "^nativeTestSources"
      ],
      "outputs": [],
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts document-http-check"
      },
      "metadata": {
        "workspaceCommand": true
      }
    },
    "test-directory-lease-fixture": {
      "executor": "nx:run-commands",
      "cache": false,
      "dependsOn": [],
      "inputs": [
        "default",
        "^default",
        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🔏️document-execution-target-lease-v1/🟦️.ts",
        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json"
      ],
      "outputs": [],
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-directory-lease-fixture"
      }
    },
    "test-composed-pack-schema": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun ./📜️script.ts test-composed-pack-schema",
        "cwd": "{projectRoot}",
        "forwardAllArgs": true
      }
    },
    "test-dsl-architecture": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-dsl-architecture"
      }
    }
  },
  "namedInputs": {
    "default": [
      "{projectRoot}/**/*",
      "{workspaceRoot}/🧰️framework/🔨️modules/🔏️hash/🟦️.ts"
    ]
  }
}

````

## .vscode/🧩️launch.seed.jsonc

SHA256 1baac36c27eed9b5b75142efc1b0a724a7cd22a5612f1adca60a7807efba6587

````text
{
  "version": "0.2.0",
  "configurations": [
{"name":"⚖️gate🖨️raster🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/raster-raster-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.756}},
{"name":"⚖️gate🖨️raster🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/raster-raster-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.757}},
{"name":"⚖️gate🖨️raster🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/raster-raster-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.758}},

{"name":"⚖️gate🎪️playground🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/demonstrator-playground:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.752}},
{"name":"⚖️gate🎪️playground🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/demonstrator-playground:build --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.762}},
{"name":"⚖️gate🎪️playground🟦️consumers","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/demonstrator-playground:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.763}},

{"name":"⚖️gate🎪️playground🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/demonstrator-playground-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.749}},
{"name":"⚖️gate🎪️playground🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/demonstrator-playground-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.75}},
{"name":"⚖️gate🎪️playground🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/demonstrator-playground-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.751}},

{"name":"⚖️gate🖍️draw🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/draw-drawing-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.753}},
{"name":"⚖️gate🖍️draw🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/draw-drawing-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.754}},
{"name":"⚖️gate🖍️draw🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/draw-drawing-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.755}},

{"name":"⚖️gate🕸️dag🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/dag-dag:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.746}},
{"name":"⚖️gate🕸️dag🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/dag-dag:build --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.747}},
{"name":"⚖️gate🕸️dag🟦️consumers","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/dag-dag:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.748}},

{"name":"⚖️gate♻️rewriting🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/trinity-rewriting:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.74}},
{"name":"⚖️gate♻️rewriting🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/trinity-rewriting:build --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.741}},
{"name":"⚖️gate♻️rewriting🟦️consumers","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/trinity-rewriting:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.742}},
{"name":"⚖️gate🕸️dag🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/dag-dag-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.743}},
{"name":"⚖️gate🕸️dag🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/dag-dag-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.744}},
{"name":"⚖️gate🕸️dag🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/dag-dag-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.745}},

{"name":"⚖️gate♻️rewriting🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/trinity-rewriting-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.737}},
{"name":"⚖️gate♻️rewriting🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/trinity-rewriting-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.738}},
{"name":"⚖️gate♻️rewriting🪶️sqlite🟦️contract","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/trinity-rewriting-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.739}},

{"name":"⚖️gate📜️imperative📜️procedure🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/imperative-procedure-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.731}},
{"name":"⚖️gate📜️imperative📜️procedure🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/imperative-procedure-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.732}},
{"name":"⚖️gate📜️imperative📜️procedure🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/imperative-procedure-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.733}},
{"name":"⚖️gate📖️playbook📖️playbook🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/playbook-playbook-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.734}},
{"name":"⚖️gate📖️playbook📖️playbook🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/playbook-playbook-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.735}},
{"name":"⚖️gate📖️playbook📖️playbook🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/playbook-playbook-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.736}},
{"name":"⚖️gate🪶️sqlite🦀️parent-baselines","type":"node-terminal","request":"launch","command":"bun nx run-many --targets=test-snapshot-sqlite-native --projects=@semio-tech/block-2d-rs,@semio-tech/block-3d-rs,@semio-tech/block-5d-rs,@semio-tech/puzzle-3d-rs,@semio-tech/process-process3d-rs,@semio-tech/writer-writer-rs,@semio-tech/mathematical-equation-rs,@semio-tech/gis-gisterrain-rs,@semio-tech/gis-gismap-rs,@semio-tech/trinity-jack-rs,@semio-tech/layout-layout-rs,@semio-tech/procedural-generation2d-rs,@semio-tech/procedural-generation3d-rs,@semio-tech/imperative-procedure-rs,@semio-tech/playbook-playbook-rs,@semio-tech/puzzle-5d-rs,@semio-tech/trinity-rewriting-rs,@semio-tech/dag-dag-rs --parallel=1 --output-style=stream --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.73}},
{"name":"⚖️gate🔌️jack🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/trinity-jack:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.727}},
{"name":"⚖️gate🔌️jack🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/trinity-jack:build --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.728}},
{"name":"⚖️gate🔌️jack🟦️consumers","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/trinity-jack:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.729}},
{"name":"⚖️gate🔱️jack🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/trinity-jack-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.724}},
{"name":"⚖️gate🔱️jack🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/trinity-jack-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.725}},
{"name":"⚖️gate🔱️jack🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/trinity-jack-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.726}},
{"name":"⚖️gate🗄️stdio🎒️zip🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-zip-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.719}},
{"name":"⚖️gate🗄️stdio🎒️zip🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-zip-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.72}},
{"name":"⚖️gate🗄️stdio🎒️zip🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-zip-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.721}},
{"name":"⚖️gate🗄️stdio🎒️zip🦀️whole","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-zip-rs:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.722}},
{"name":"⚖️gate🌊️flow🚦️journal","type":"node-terminal","request":"launch","command":"bun nx run semio-framework-os-flow-core:test --filter-expr 'test(snapshot_io_prerequisite_flow_gesture_journal)' --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.723}},
{"name":"⚖️gate🏭️process🧊️3d🟦️consumers","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/process-process3d-rs:check-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.718}},
{"name":"⚖️gate✒️writer🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/writer-writer-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.705}},
{"name":"⚖️gate✒️writer🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/writer-writer-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.706}},
{"name":"⚖️gate✒️writer🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/writer-writer:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.707}},
{"name":"⚖️gate➗️equation🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/mathematical-equation-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.708}},
{"name":"⚖️gate➗️equation🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/mathematical-equation-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.709}},
{"name":"⚖️gate➗️equation🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/mathematical-equation:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.71}},
{"name":"⚖️gate🏔️terrain🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/gis-gisterrain-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.711}},
{"name":"⚖️gate🏔️terrain🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/gis-gisterrain-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.712}},
{"name":"⚖️gate🏔️terrain🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/gis-gisterrain:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.713}},
{"name":"⚖️gate🗺️map🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/gis-gismap-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.714}},
{"name":"⚖️gate🗺️map🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/gis-gismap-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.715}},
{"name":"⚖️gate🗺️map🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/gis-gismap-js:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.716}},
{"name":"⚖️gate✒️writer➗️equation🌍️gis🟦️consumers","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/writer-writer:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.717}},
    {"name":"⚖️gate🏭️process🧊️3d🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/process-process3d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.702}},
    {"name":"⚖️gate🏭️process🧊️3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/process-process3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.703}},
    {"name":"⚖️gate🏭️process🧊️3d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/process-process3d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.704}},
    {"name":"⚖️gate🌀️procedural◻️2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/procedural-generation2d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.7}},
    {"name":"⚖️gate🌀️procedural🧊️3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/procedural-generation3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.701}},
    {"name":"⚖️gate🌀️generation2d🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/procedural-generation2d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.697}},
    {"name":"⚖️gate🧊️generation3d🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/procedural-generation3d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.698}},
    {"name":"⚖️gate🌀️generation🟦️consumers","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/procedural-generation2d:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.699}},
    {"name":"⚖️gate🧱️block🖐️5d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/block-5d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.696}},
    {"name":"⚖️gate🧱️block🧊️3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/block-3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.695}},
    {"name":"⚖️gate🧱️block◻️2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/block-2d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.694}},
    {"name":"⚖️gate🧩️puzzle🧊️3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.693}},
    {"name":"⚖️gate🧩️puzzle🧊️3d🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-3d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.692}},

    {"name":"⚖️test-snapshot-sqlite📏️layout🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/layout-layout-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.691}},
    {"name":"⚖️test-snapshot-sqlite-source📏️layout🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/layout-layout-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.69109999999995}},
    {"name":"⚖️test-snapshot-sqlite-native📏️layout🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/layout-layout-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6912}},
    {"name":"⚖️test-quick📖️pdf🌊️structural🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-pdf-rs:test --skip-nx-cache -- quick --lib lossless_structural_flow_law_bachelor_thesis_snapshot_mutation_diff_io_and_inverse --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6904}},
    {"name":"⚖️sqlite-source🧱️🖐️5d","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/block-5d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.688}},
    {"name":"⚖️sqlite-public🧱️🖐️5d","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/block-5d-rs:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.689}},
    {"name":"⚖️test-quick📕️xlsx🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-xlsx-rs:test --skip-nx-cache -- quick --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6901}},
    {"name":"⚖️test-quick📜️docx🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-docx-rs:test --skip-nx-cache -- quick --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6902}},
    {"name":"⚖️test-quick📽️pptx🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-pptx-rs:test --skip-nx-cache -- quick --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6903}},
    {"name":"⚖️gate🏗️fem🧊️3d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d:build --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.685}},
    {"name":"⚖️gate🏗️fem🧊️3d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d:check --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.686}},
    {"name":"⚖️gate🏗️fem🧊️3d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.687}},
    {"name":"⚖️gate🧩️puzzle🧊️3d🪶️sqlite🔍️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-3d-rs:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.684}},
    {"name":"⚖️gate🧩️puzzle🧊️3d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-3d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.683}},
    {"name":"⚖️gate🧩️puzzle🖐️5d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-5d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.682}},
    {"name":"⚖️gate🧩️puzzle🖐️5d🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-5d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.681}},
    {"name":"⚖️gate🏗️fem🧊️3d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.679}},
    {"name":"⚖️gate🏗️fem🧊️3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.678}},
    {"name":"⚖️gate🏗️fem🧊️3d🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.677}},
    {"name":"⚖️gate🏗️fem◻️2d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d:build --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.674}},
    {"name":"⚖️gate🏗️fem◻️2d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d:check --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.675}},
    {"name":"⚖️gate🏗️fem◻️2d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.676}},
    {"name":"⚖️gate🏗️fem◻️2d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.673}},
    {"name":"⚖️gate🏗️fem◻️2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.672}},
    {"name":"⚖️gate🏗️fem◻️2d🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.671}},
    {"name":"⚖️gate🧩️puzzle🖐️5d🪶️sqlite🔍️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-5d-rs:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.670}},
    {"name":"⚖️gate🧩️puzzle🖐️5d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-5d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.669}},
    {"name":"⚖️gate📸️remodel📸️remodeling🪶️sqlite🔍️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/remodel-remodeling-rs:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.668}},
    {
      "name": "⚖️gate📸️remodel📸️remodeling🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:test-snapshot-sqlite --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.65
      }
    },
    {
      "name": "⚖️gate📸️remodel📸️remodeling🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:test-snapshot-sqlite-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.65099999999995
      }
    },
    {
      "name": "⚖️gate📸️remodel📸️remodeling🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:test-snapshot-sqlite-source --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.652
      }
    },
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.623}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.624}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.625}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🏗️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.626}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🔍️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.627}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🧪️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:test --skip-nx-cache -- quick --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.628}},
    {"name":"⚖️gate🔋️energy🔋️model🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model:build --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.629}},
    {"name":"⚖️gate🔋️energy🔋️model🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model:check --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.63}},
    {"name":"⚖️gate🔋️energy🔋️model🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.631}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.632}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.633}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.634}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.635}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.636}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.637}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.638}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.639}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.64}},
    {"name":"⚖️gate🀄️wfc◻️2d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.641}},
    {"name":"⚖️gate🀄️wfc◻️2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.642}},
    {"name":"⚖️gate🀄️wfc◻️2d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.643}},
    {"name":"⚖️gate🀄️wfc🧊️3d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.644}},
    {"name":"⚖️gate🀄️wfc🧊️3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.645}},
    {"name":"⚖️gate🀄️wfc🧊️3d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.646}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.653}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.654}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.655}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.656}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.657}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.658}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.659}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.66}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.661}},
    {"name":"⚖️gate🀄️wfc◻️2d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.662}},
    {"name":"⚖️gate🀄️wfc◻️2d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.663}},
    {"name":"⚖️gate🀄️wfc◻️2d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.664}},
    {"name":"⚖️gate🀄️wfc🧊️3d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.665}},
    {"name":"⚖️gate🀄️wfc🧊️3d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.666}},
    {"name":"⚖️gate🀄️wfc🧊️3d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.667}},
    {"name":"⚖️gate📋️forms📋️forms🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/forms-forms-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.619}},
    {"name":"⚖️gate📋️forms📋️forms🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/forms-forms-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.62}},
    {"name":"⚖️gate📋️forms📋️forms🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/forms-forms-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.621}},
    {"name":"⚖️gate📋️forms📋️forms🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/forms-js:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.622}},
    {"name":"⚖️gate🗄️stdio🧿️semio🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-semio-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.596}},
    {"name":"⚖️gate🗄️stdio🧿️semio🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-semio-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.597}},
    {"name":"⚖️gate🗄️stdio🧿️semio🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-semio-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.598}},
    {"name":"⚖️gate🗄️stdio🧿️semio🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-semio:test","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.599}},

    {"name":"⚖️gate📐️cad📐️cad🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/cad-cad-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.592}},
    {"name":"⚖️gate📐️cad📐️cad🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/cad-cad-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.59299999999996}},
    {"name":"⚖️gate📐️cad📐️cad🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/cad-cad-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.594}},
    {"name":"⚖️gate📐️cad📐️cad🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/cad-cad-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.59499999999997}},
    {"name":"⚖️gate🗄️stdio🖊️dwg🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-dwg-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.588}},
    {"name":"⚖️gate🗄️stdio🖊️dwg🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-dwg-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.589}},
    {"name":"⚖️gate🗄️stdio🖊️dwg🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-dwg-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.59000000000003}},
    {"name":"⚖️gate🗄️stdio🖊️dwg🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-dwg:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.591}},
    {"name":"⚖️gate🔁️workflow🔁️workflow🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-workflow-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.584}},
    {"name":"⚖️gate🔁️workflow🔁️workflow🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-workflow-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.585}},
    {"name":"⚖️gate🔁️workflow🔁️workflow🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-workflow-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.586}},
    {"name":"⚖️gate🔁️workflow🔁️workflow🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-workflow-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.587}},
    {"name":"⚖️gate🔁️workflow🏃️run🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-run-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.58}},
    {"name":"⚖️gate🔁️workflow🏃️run🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-run-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.58099999999996}},
    {"name":"⚖️gate🔁️workflow🏃️run🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-run-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.582}},
    {"name":"⚖️gate🔁️workflow🏃️run🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-run-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.58299999999997}},
    {"name":"⚖️gate💠️lowpoly🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/lowpoly-lowpoly-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.576}},
    {"name":"⚖️gate💠️lowpoly🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/lowpoly-lowpoly-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.577}},
    {"name":"⚖️gate💠️lowpoly🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/lowpoly-lowpoly-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.578}},
    {"name":"⚖️gate💠️lowpoly🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/lowpoly-lowpoly:test","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.579}},
    {"name":"⚖️gate🧩️puzzle◻️2d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-2d-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.572}},
    {"name":"⚖️gate🧩️puzzle◻️2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-2d-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.573}},
    {"name":"⚖️gate🧩️puzzle◻️2d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-2d-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.574}},
    {"name":"⚖️gate🧩️puzzle◻️2d🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-2d-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.575}},
    {"name":"⚖️gate💡️reasoning🔌️wires🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/reasoning-wires-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.568}},
    {"name":"⚖️gate💡️reasoning🔌️wires🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/reasoning-wires-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.569}},
    {"name":"⚖️gate💡️reasoning🔌️wires🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/reasoning-wires-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.57}},
    {"name":"⚖️gate💡️reasoning🔌️wires🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/reasoning-wires:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.571}},
    {"name":"⚖️gate🪐️space🪐️space🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-space-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.564}},
    {"name":"⚖️gate🪐️space🪐️space🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-space-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.565}},
    {"name":"⚖️gate🪐️space🪐️space🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-space-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.566}},
    {"name":"⚖️gate🪐️space🪐️space🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-space:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.567}},
    {"name":"⚖️gate🪵️sourcing🗂️curation🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sourcing-curation-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.56}},
    {"name":"⚖️gate🪵️sourcing🗂️curation🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sourcing-curation-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.561}},
    {"name":"⚖️gate🪵️sourcing🗂️curation🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sourcing-curation-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.562}},
    {"name":"⚖️gate🪵️sourcing🗂️curation🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sourcing-curation-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.563}},
    {"name":"⚖️gate🪐️space🏠️home🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-home-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.55}},
    {"name":"⚖️gate🪐️space🏠️home🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-home-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.551}},
    {"name":"⚖️gate🪐️space🏠️home🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-home-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.552}},
    {
      "name": "⚖️gate🪐️space🏠️home🪶️sqlite🟦️public",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/space-home:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.553
      }
    },
    {
      "name": "⚖️gate🏛️architect🏛️program🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.6
      }
    },
    {
      "name": "⚖️gate🏛️architect🏛️program🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.601
      }
    },
    {
      "name": "⚖️gate🏛️architect🏛️program🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.60200000000003
      }
    },
    {
      "name": "⚖️gate🏛️architect🏛️program🪶️sqlite🏭️public",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:verify-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.603
      }
    },
    {
      "name": "⚖️gate🗄️stdio📼️avi🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-avi-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.604
      }
    },
    {
      "name": "⚖️gate🗄️stdio📼️avi🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-avi-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.60499999999996
      }
    },
    {
      "name": "⚖️gate🗄️stdio📼️avi🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-avi-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.606
      }
    },
    {
      "name": "⚖️gate🗄️stdio💬️bcf🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-bcf-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.60699999999997
      }
    },
    {
      "name": "⚖️gate🗄️stdio💬️bcf🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-bcf-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.608
      }
    },
    {
      "name": "⚖️gate🗄️stdio💬️bcf🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-bcf-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.609
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎵️mp3🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp3-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.60999999999996
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎵️mp3🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp3-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.611
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎵️mp3🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp3-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.61199999999997
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎥️mp4🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp4-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.613
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎥️mp4🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp4-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.614
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎥️mp4🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp4-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.615
      }
    },
    {
      "name": "⚖️gate🗄️stdio🌦️epw🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-epw-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.616
      }
    },
    {
      "name": "⚖️gate🗄️stdio🌦️epw🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-epw-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.61699999999996
      }
    },
    {
      "name": "⚖️gate🗄️stdio🌦️epw🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-epw-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.618
      }
    },
    {"name": "⚖️test-controlled-encoding🌱️value🦀️", "type": "node-terminal", "request": "launch", "command": "bun nx run @semio-tech/value-rs:test-controlled-encoding", "cwd": "${workspaceFolder}", "presentation": {"group": "4_gate", "order": 900.033208}},
    {"name":"⚖️gate🎥️shooting🎥️shooting🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/shooting-shooting-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.543}},
    {"name":"⚖️gate🎥️shooting🎥️shooting🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/shooting-shooting-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.540}},
    {"name":"⚖️gate🎥️shooting🎥️shooting🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/shooting-shooting-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.541}},
    {"name":"⚖️gate🎥️shooting🎥️shooting🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/shooting-shooting-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.542}},
    {"name":"⚖️gate🎞️animate🎬️presentation🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/animate-presentation-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.530}},
    {"name":"⚖️gate🎞️animate🎬️presentation🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/animate-presentation-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.531}},
    {"name":"⚖️gate🎞️animate🎬️presentation🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/animate-presentation-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.532}},
    {"name":"⚖️gate🎬️sequence🎬️sequence🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sequence-sequence:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.523}},
    {"name":"⚖️gate🎬️sequence🎬️sequence🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sequence-sequence-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.52}},
    {"name":"⚖️gate🎬️sequence🎬️sequence🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sequence-sequence-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.521}},
    {"name":"⚖️gate🎬️sequence🎬️sequence🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sequence-sequence-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.522}},
    {"name":"⚖️gate🗄️stdio🔺️stl🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-stl-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.5}},
    {"name":"⚖️gate🗄️stdio🔺️stl🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-stl-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.501}},
    {"name":"⚖️gate🗄️stdio🔺️stl🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-stl-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.502}},
    {"name":"⚖️gate🗄️stdio🗽️obj🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-obj-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.503}},
    {"name":"⚖️gate🗄️stdio🗽️obj🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-obj-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.504}},
    {"name":"⚖️gate🗄️stdio🗽️obj🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-obj-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.505}},
    {"name":"⚖️gate🗄️stdio🧱️ply🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-ply-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.506}},
    {"name":"⚖️gate🗄️stdio🧱️ply🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-ply-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.507}},
    {"name":"⚖️gate🗄️stdio🧱️ply🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-ply-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.508}},
    {"name":"⚖️gate🌿️vcs🌿️vcs🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/vcs-js:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.494}},
    {"name":"⚖️gate🌿️vcs🌿️vcs🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/vcs-vcs-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.491}},
    {"name":"⚖️gate🌿️vcs🌿️vcs🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/vcs-vcs-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.492}},
    {"name":"⚖️gate🌿️vcs🌿️vcs🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/vcs-vcs-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.493}},
    {"name":"⚖️gate📕️norm🏭️vdi3805🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.465}},
    {"name":"⚖️gate📕️norm🏭️vdi3805🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.466}},
    {"name":"⚖️gate📕️norm🏭️vdi3805🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.467}},
    {"name":"⚖️build📕️norm🏭️vdi3805🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.468}},
    {"name":"⚖️check📕️norm🏭️vdi3805🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.469}},
    {"name":"⚖️test📕️norm🏭️vdi3805🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.47}},
    {"name":"⚖️gate📕️norm🧬️contract🌱️bytes🖥️bounded","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-artifact-contract-rs:test-byte-property","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.471}},
    {"name":"⚖️gate📕️norm🫨️en1998🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.453}},
    {"name":"⚖️gate📕️norm🫨️en1998🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.454}},
    {"name":"⚖️gate📕️norm🫨️en1998🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.455}},
    {"name":"⚖️build📕️norm🫨️en1998🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.456}},
    {"name":"⚖️check📕️norm🫨️en1998🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.457}},
    {"name":"⚖️test📕️norm🫨️en1998🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.458}},
    {"name":"⚖️gate📕️norm🧩️en1994🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.447}},
    {"name":"⚖️gate📕️norm🧩️en1994🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.448}},
    {"name":"⚖️gate📕️norm🧩️en1994🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.449}},
    {"name":"⚖️build📕️norm🧩️en1994🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.45}},
    {"name":"⚖️check📕️norm🧩️en1994🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.451}},
    {"name":"⚖️test📕️norm🧩️en1994🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.452}},
    {"name":"⚖️gate📕️norm🧱️din4108🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.417}},
    {"name":"⚖️gate📕️norm🧱️din4108🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.418}},
    {"name":"⚖️gate📕️norm🧱️din4108🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.419}},
    {"name":"⚖️build📕️norm🧱️din4108🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.420}},
    {"name":"⚖️check📕️norm🧱️din4108🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.421}},
    {"name":"⚖️test📕️norm🧱️din4108🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.422}},
    {"name":"⚖️gate📕️norm⚡️din18599🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.435}},
    {"name":"⚖️gate📕️norm⚡️din18599🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.436}},
    {"name":"⚖️gate📕️norm⚡️din18599🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.437}},
    {"name":"⚖️build📕️norm⚡️din18599🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.438}},
    {"name":"⚖️check📕️norm⚡️din18599🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.439}},
    {"name":"⚖️test📕️norm⚡️din18599🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.440}},

    {
      "name": "⚖️build📕️norm🌬️din16798🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.414
      }
    },
    {
      "name": "⚖️check📕️norm🌬️din16798🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.415
      }
    },
    {
      "name": "⚖️test📕️norm🌬️din16798🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.416
      }
    },
    {
      "name": "⚖️gate📕️norm🌬️din16798🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.411
      }
    },
    {
      "name": "⚖️gate📕️norm🌬️din16798🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.412
      }
    },
    {
      "name": "⚖️gate📕️norm🌬️din16798🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.413
      }
    },
    {
      "name": "⚖️build📕️norm🪵️en1995🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.402
      }
    },
    {
      "name": "⚖️check📕️norm🪵️en1995🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.403
      }
    },
    {
      "name": "⚖️test📕️norm🪵️en1995🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.404
      }
    },
    {
      "name": "⚖️gate📕️norm🪵️en1995🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.399
      }
    },
    {
      "name": "⚖️gate📕️norm🪵️en1995🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.4
      }
    },
    {
      "name": "⚖️gate📕️norm🪵️en1995🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.401
      }
    },
    {
      "name": "⚖️build📕️norm🪨️en1996🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.396
      }
    },
    {
      "name": "⚖️check📕️norm🪨️en1996🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.397
      }
    },
    {
      "name": "⚖️test📕️norm🪨️en1996🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.398
      }
    },
    {
      "name": "⚖️gate📕️norm🪨️en1996🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.393
      }
    },
    {
      "name": "⚖️gate📕️norm🪨️en1996🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.394
      }
    },
    {
      "name": "⚖️gate📕️norm🪨️en1996🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.395
      }
    },
    {
      "name": "⚖️build📕️norm🏋️en1991🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.39
      }
    },
    {
      "name": "⚖️check📕️norm🏋️en1991🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.391
      }
    },
    {
      "name": "⚖️test📕️norm🏋️en1991🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.392
      }
    },
    {
      "name": "⚖️gate📕️norm🏋️en1991🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.387
      }
    },
    {
      "name": "⚖️gate📕️norm🏋️en1991🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.388
      }
    },
    {
      "name": "⚖️gate📕️norm🏋️en1991🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.389
      }
    },
    {
      "name": "⚖️gate📕️norm⚖️en1990🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.381
      }
    },
    {
      "name": "⚖️gate📕️norm⚖️en1990🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.382
      }
    },
    {
      "name": "⚖️gate📕️norm⚖️en1990🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.383
      }
    },
    {
      "name": "⚖️build📕️norm⚖️en1990🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.384
      }
    },
    {
      "name": "⚖️check📕️norm⚖️en1990🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.385
      }
    },
    {
      "name": "⚖️test📕️norm⚖️en1990🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.386
      }
    },
    {
      "name": "🦑️Repo 📋️native owner command 🧪️policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-native-owner-command-policy --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05751
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🖱️UI 🦀️native 🧭️command dispatch",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:test-command-dispatch --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05752
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🖱️UI 🦀️native 🧭️command types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:check-command-types --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05753
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "◻️2D 🧮️compute 🧪️portable schema",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/2d-compute:test-schema --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05754
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "◻️2D 🧮️compute 🧭️contract types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/2d-compute:check-types --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05755
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧪️test◻️2d🧮️compute📍️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/2d-compute:test-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05787
      }
    },
    {
      "name": "🧪️test◻️2d🧮️compute🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-2d:test-compute --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057871
      }
    },
    {
      "name": "🧪️test◻️2d🧮️compute🔏️keys",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-2d:test-compute-keys --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057872
      }
    },
    {
      "name": "🌐️UI locale 🦀️native owner laws",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-locale-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05808
      }
    },
    {
      "name": "🧪️test🖱️ui🧊️feature-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:test-feature-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-ui-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05812
      }
    },
    {
      "name": "⚖️check↔️paged-history-stack🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:paged-history-stack-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1814
      }
    },
    {
      "name": "⚖️check↔️paged-history-stack🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:paged-history-stack-native-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1815
      }
    },
    {
      "name": "⚖️check🔐️hub-auth-client🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-auth-client-rs:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1811
      }
    },
    {
      "name": "⚖️test🔐️hub-auth-client🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-auth-client-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1812
      }
    },
    {
      "name": "⚖️test-source🔐️hub-auth-client🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-auth-client-rs:test-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1813
      }
    },
    {
      "name": "⚖️check👁️source-watch🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:source-watch-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.181
      }
    },
    {
      "name": "⚖️verify🔗️puzzle-browser-contribution🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-puzzle-composition-tests:verify-browser-contribution",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1809
      }
    },
    {
      "name": "⚖️check📍️distribution-output🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:distribution-output-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1808
      }
    },
    {
      "name": "⚖️test-component-owners📇️registry🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-component-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1807
      }
    },
    {
      "name": "⚖️test-artifact-kind🧰️framework🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-rs:test-artifact-kind",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05665
      }
    },
    {
      "name": "⚖️test-artifact-kind-source🧰️framework🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-rs:test-artifact-kind-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.056651
      }
    },
    {
      "name": "⚖️check🖍️draw-guest-instance🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-plugin:guest-instance-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2647
      }
    },
    {
      "name": "⚖️check🧩️hub-component-codecs🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:component-codec-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2648
      }
    },
    {
      "name": "⚖️check⏱️hub-gis-codec-budget🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:component-codec-budget-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2649
      }
    },
    {
      "name": "⚖️check🧫️host-fixture🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin-host-fixture:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2638
      }
    },
    {
      "name": "⚖️build🧫️host-fixture-component🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin-host-fixture:component-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2639
      }
    },
    {
      "name": "⚖️check🧫️host-owned-instance🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin-host:owned-instance-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.264
      }
    },
    {
      "name": "⚖️check🗒️note-snapshot-guest🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/note-note-rs:verify-snapshot-guest",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2641
      }
    },
    {
      "name": "⚖️check🗒️note-sqlite-snapshot-guest🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/note-note-rs:verify-sqlite-snapshot-guest",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2642
      }
    },
    {
      "name": "⚖️check🗺️gis-mcp-inference-source🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:inference-bridge-source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2643
      }
    },
    {
      "name": "⚖️check🗺️gis-mcp-inference-process🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:inference-bridge-process-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2644
      }
    },
    {
      "name": "🧑‍💻mcp🌍️gis2d-stdio-dev",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:mcp-gis2d-stdio-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 332.8
      }
    },
    {
      "name": "🧑‍💻mcp🌍️gis2d-stdio-release",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:mcp-gis2d-stdio-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 332.81
      }
    },
    {
      "name": "🧑‍💻mcp🌍️gis2d-http-dev",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:mcp-gis2d-http-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 332.82
      }
    },
    {
      "name": "🧑‍💻mcp🌍️gis2d-http-release",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:mcp-gis2d-http-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 332.83
      }
    },
    {
      "name": "⚖️browser-test📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:browser-test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️contract-check🎮️native-host🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/playground-native-host:contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️source-check📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️check📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️test📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️wasm📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:wasm",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-codec-oracle🌍️gis🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-plugin:native-codec-oracle",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-codec-oracle🌿️vcs🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/vcs-plugin:native-codec-oracle",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️source-check💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-check💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:native-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️check💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-build💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:native-build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-build-release💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:native-build-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️check🗺️gis-inference-presentation🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:inference-presentation-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2634
      }
    },
    {
      "name": "⚖️check🗺️gis-inference-mcp🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-inference-mcp",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2635
      }
    },
    {
      "name": "⚖️check🗺️gis-inference-native-service🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-inference-native-service",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2636
      }
    },
    {
      "name": "⚖️check🧩️mcp-installed-service🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp-rs:installed-service-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2637
      }
    },
    {
      "name": "⚖️publish🗄️stdio-publication🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-publication:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.32
      }
    },
    {
      "name": "⚖️test🗄️stdio-publication🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-publication:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.32
      }
    },
    {
      "name": "⚖️build🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️check🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️test🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️canonical-architecture🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️deletion-proof🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:deletion-proof",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️flow-add-widget-retained-check🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:add-widget-retained-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️flow-child-edit-check🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:child-edit-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️flow-child-identity-check🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:child-identity-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️flow-test-source🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:test-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-architect-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:verify-architect-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-cad-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cad-cad-rs:verify-cad-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-curation-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/sourcing-curation-rs:verify-curation-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-dag-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/dag-dag-rs:verify-dag-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-drawing-canvas-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-drawing-rs:verify-drawing-canvas-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-assembly-physical-owners🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-assembly-physical-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-live-visual-publication🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-live-visual-publication",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-mesh-preparation-owners🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-mesh-preparation-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-numerical-microcursor🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-numerical-microcursor",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-numerical-page-owners🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-numerical-page-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-pcg-publication-owners🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-pcg-publication-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-scalar-owners-native🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-scalar-owners-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem2d-window-config-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem2d-window-config-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem3d-numerical-child-native🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/fem-3d-rs:verify-fem3d-numerical-child-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem3d-window-config-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem3d-window-config-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-flow-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:verify-flow-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-forms-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/forms-forms-rs:verify-forms-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-forms-try-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-forms-composition-tests:verify-forms-try-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-framework-flow-physical-retirement🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-flow-flow-rs:verify-framework-flow-physical-retirement",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-framework-ui-protocol-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-space-composition-tests:verify-framework-ui-protocol-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-generation2d-window-camera-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/procedural-generation2d-rs:verify-generation2d-window-camera-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-generation3d-document-io🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/procedural-generation3d-rs:verify-generation3d-document-io",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-generation3d-preview-window-transient🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/procedural-generation3d-rs:verify-generation3d-preview-window-transient",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-gis-map-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-gis-map-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-gis-terrain-window-config🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gisterrain-rs:verify-gis-terrain-window-config",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-home-host-panel-owner🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/space-home-rs:verify-home-host-panel-owner",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-jack-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-jack-rs:verify-jack-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-jack-query-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-jack-rs:verify-jack-query-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-layout-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/layout-layout-rs:verify-layout-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-layout-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/layout-layout-rs:verify-layout-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-map-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-map-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-norm-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-norm-composition-tests:verify-norm-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-norm-results-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-norm-composition-tests:verify-norm-results-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-note-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-note-composition-tests:verify-note-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-note-empty-config-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/note-note-rs:verify-note-empty-config-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-playbook-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/playbook-playbook-rs:verify-playbook-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-presentation-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/animate-presentation-rs:verify-presentation-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-procedure-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/imperative-procedure-rs:verify-procedure-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-program-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:verify-program-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-puzzle-fill-policy-self-tests🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-puzzle-composition-tests:verify-puzzle-fill-policy-self-tests",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-remodel-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:verify-remodel-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-rewriting-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-rewriting-rs:verify-rewriting-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-rewriting-map-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-rewriting-rs:verify-rewriting-map-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-rewriting-window-config🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-rewriting-rs:verify-rewriting-window-config",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-sequence-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/sequence-sequence-rs:verify-sequence-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-stdio-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-semio-rs:verify-stdio-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-terrain-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gisterrain-rs:verify-terrain-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-wires-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/reasoning-wires-rs:verify-wires-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-wires-window-transient🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/reasoning-wires-rs:verify-wires-window-transient",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-writer-window-state🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/writer-writer-rs:verify-writer-window-state",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️owned-script-routes🦑️repo🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-owned-script-routes",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️installed-service🧰️os🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os:installed-service-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2631
      }
    },
    {
      "name": "⚖️inference🗺️gismap🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:inference-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2632
      }
    },
    {
      "name": "⚖️inference-browser🗺️gismap🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:cold-document-pair-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2633
      }
    },
    {
      "name": "⚖️browser-dock-widgets⚛️react🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- run react \"${input:wgpuDockReactServe}\" --suite widgets --output \"${workspaceFolder}/${input:wgpuDockArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2641
      }
    },
    {
      "name": "⚖️browser-dock-widgets🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- run wgpu \"${input:wgpuDockServe}\" --suite widgets --output \"${workspaceFolder}/${input:wgpuDockArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2642
      }
    },
    {
      "name": "⚖️browser-embedded-acceptance🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-embedded-acceptance -- --configuration \"${workspaceFolder}/${input:wgpuEmbeddedConfiguration}\" --output \"${workspaceFolder}/${input:wgpuEmbeddedArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2643
      }
    },
    {
      "name": "🧪️test🖍️draw🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.4
      }
    },
    {
      "name": "🧪️test🖍️draw🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-drawing-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.5
      }
    },
    {
      "name": "🔍️check🖍️draw🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-drawing-rs:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.6
      }
    },
    {
      "name": "📚️deps print tex",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:deps-tex",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -36
      }
    },
    {
      "name": "📥️deps print compiler",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:deps-tectonic",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -37
      }
    },
    {
      "name": "🔄️watch logo",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:logo-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 19
      }
    },
    {
      "name": "▶️repo coordinator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator:dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 20
      }
    },
    {
      "name": "🚀️start repo coordinator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator:start",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 21
      }
    },
    {
      "name": "📥️styling Python dependencies",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-py:deps",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -45
      }
    },
    {
      "name": "📥️styling .NET dependencies",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-dotnet:deps",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -44
      }
    },
    {
      "name": "📥️energy oracle Python dependencies",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/energy-oracle-py:deps",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -45
      }
    },
    {
      "name": "⚙️setup",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:setup",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -50
      }
    },
    {
      "name": "▶️start",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:start",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.9
      }
    },
    {
      "name": "🏭️generate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:generate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.8
      }
    },
    {
      "name": "🧹lint",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:lint",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.7
      }
    },
    {
      "name": "🎨format",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:format",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.6
      }
    },
    {
      "name": "🧪️test",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.5
      }
    },
    {
      "name": "📦️build",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.4
      }
    },
    {
      "name": "🚢️publish",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.3
      }
    },
    {
      "name": "🧰️prepare",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:prepare",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49
      }
    },
    {
      "name": "⚙️setup🪟️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:setup-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -48
      }
    },
    {
      "name": "⚙️setup🐙️git",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:setup-git",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -47
      }
    },
    {
      "name": "📥️deps javascript",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-javascript",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -46
      }
    },
    {
      "name": "📥️deps python",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-python",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -45
      }
    },
    {
      "name": "📥️deps cargo",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-cargo",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -44
      }
    },
    {
      "name": "📥️deps go",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-go",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -43
      }
    },
    {
      "name": "📥️deps dotnet",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-dotnet",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -42
      }
    },
    {
      "name": "📥️deps cpp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-cpp",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -41
      }
    },
    {
      "name": "📥️deps browsers",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-browsers",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -40
      }
    },
    {
      "name": "📥️deps wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-wasm",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -39
      }
    },
    {
      "name": "📥️deps Trunk",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-trunk",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -38.75
      }
    },
    {
      "name": "📥️deps wasm optimizer",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-wasm-opt",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -38.5
      }
    },
    {
      "name": "📥️deps tools",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-tools",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -38
      }
    },
    {
      "name": "🧪️test⚡️cache-command-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-cache-command-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": -11.5
      }
    },
    {
      "name": "⚖️gate🦀️cargo🧾️build-dir-provenance",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:cargo-provenance-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": -11.4
      }
    },
    {
      "name": "🚦️ci baseline",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:ci-baseline",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -39
      }
    },
    {
      "name": "🦑️mcp dev",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun ./📜️script.ts dev mcp stdio client",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -4
      }
    },
    {
      "name": "⌨️gemini",
      "type": "node-terminal",
      "request": "launch",
      "command": "gemini --yolo",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "1_keyboard",
        "order": 10
      }
    },
    {
      "name": "⌨️kiro",
      "type": "node-terminal",
      "request": "launch",
      "command": "kiro-cli chat --trust-all-tools",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "1_keyboard",
        "order": 20
      }
    },
    {
      "name": "🖱️f3d",
      "type": "node-terminal",
      "request": "launch",
      "command": "f3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "2_mouse",
        "order": 10
      }
    },
    {
      "name": "🖱️gitkraken",
      "type": "node-terminal",
      "request": "launch",
      "command": "gitkraken --path \"${workspaceFolder}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "2_mouse",
        "order": 20
      }
    },
    {
      "name": "🖱️mcpinspector",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp",
      "env": {
        "CLIENT_PORT": "6274",
        "SERVER_PORT": "6277"
      },
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "2_mouse",
        "order": 30
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://[^\\s]+:6274)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🎛️dashboard",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:run",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1
      }
    },
    {
      "name": "🛠️dev🎛️dashboard🌀daemon▶️start",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:daemon -- start",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1.1
      }
    },
    {
      "name": "🛠️dev🎛️dashboard🌀daemon📎attach",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:daemon -- attach",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1.2
      }
    },
    {
      "name": "🛠️dev🎛️dashboard🌊️workflow",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:workflow",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1.3
      }
    },
    {
      "name": "🛠️dev🎛️dashboard🌳️command-tree",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts run command-tree --dump-tree",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1.4
      }
    },
    "@generated:cad:react",
    "@generated:cad:wgpu",
    {
      "name": "🛠️dev📐️cad🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- cad",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 10.2
      }
    },
    {
      "name": "🛠️dev📐️cad🧩️concrete🌲️forest⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- cad fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "CAD_JS_RENDERER_PLAY_PORT": "6020",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 20
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6020)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📐️cad🧩️concrete🌲️forest🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- cad fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "CAD_JS_RENDERER_PLAY_PORT": "6120",
        "SEMIO_RENDERER": "wgpu"
      },
      "presentation": {
        "group": "3_dev",
        "order": 20.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6120)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📐️cad🧩️concrete🌲️forest🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- cad",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 20.2
      }
    },
    "@generated:dag:react",
    "@generated:dag:wgpu",
    {
      "name": "🛠️dev🌳️dag🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- dag",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 140.2
      }
    },
    "@generated:mathematical:react",
    "@generated:mathematical:wgpu",
    {
      "name": "🛠️dev🧮️mathematical🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- mathematical",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 141.2
      }
    },
    "@generated:architect:react",
    "@generated:architect:wgpu",
    {
      "name": "🛠️dev🏛️architect🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- architect",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 142.2
      }
    },
    "@generated:flow:react",
    "@generated:flow:wgpu",
    {
      "name": "🛠️dev🌊️flow🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- flow",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 150.2
      }
    },
    "@generated:imperative:react",
    "@generated:imperative:wgpu",
    {
      "name": "🛠️dev⚙️imperative🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- imperative",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 155.2
      }
    },
    "@generated:sequence:react",
    "@generated:sequence:wgpu",
    {
      "name": "🛠️dev📜️sequence🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- sequence",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 156.2
      }
    },
    "@generated:lowpoly:react",
    "@generated:lowpoly:wgpu",
    {
      "name": "🛠️dev🔷️lowpoly🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- lowpoly",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 157.2
      }
    },
    "@generated:layout:react",
    "@generated:layout:wgpu",
    {
      "name": "🛠️dev📄️layout🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- layout",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 158.2
      }
    },
    "@generated:gis2d:react",
    "@generated:gis2d:wgpu",
    {
      "name": "🛠️dev🌐️gis📍️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- gis2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 160.2
      }
    },
    "@generated:gis3d:react",
    "@generated:gis3d:wgpu",
    {
      "name": "🛠️dev🌐️gis⛰️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- gis3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 160.5
      }
    },
    "@generated:animate:react",
    "@generated:animate:wgpu",
    {
      "name": "🛠️dev🎬️animateplay🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- animate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 170.2
      }
    },
    {
      "name": "🛠️dev🖨️print📊️viz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:watch-viz",
      "cwd": "${workspaceFolder}"
    },
    {
      "name": "🛠️dev🖨️print",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:watch",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 175
      }
    },
    "@generated:generation2d:react",
    "@generated:generation2d:wgpu",
    {
      "name": "🛠️dev🔧️procedural🩻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- generation2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 180.2
      }
    },
    "@generated:generation3d:react",
    "@generated:generation3d:wgpu",
    {
      "name": "🛠️dev🔧️procedural🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- generation3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 190.2
      }
    },
    "@generated:process3d:react",
    "@generated:process3d:wgpu",
    {
      "name": "🛠️dev🪚️process🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- process3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 195.2
      }
    },
    "@generated:sourcing:react",
    "@generated:sourcing:wgpu",
    {
      "name": "🛠️dev🛒️sourcing🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- sourcing",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 196.2
      }
    },
    "@generated:bitmap:react",
    "@generated:bitmap:wgpu",
    {
      "name": "🛠️dev🀄️wfc🖼️bitmap🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- bitmap",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 201.2
      }
    },
    "@generated:grid2d:react",
    "@generated:grid2d:wgpu",
    {
      "name": "🛠️dev🀄️wfc🔲️grid2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- grid2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 202.2
      }
    },
    "@generated:wfc2d:react",
    "@generated:wfc2d:wgpu",
    {
      "name": "🛠️dev🀄️wfc◻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- wfc2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 203.2
      }
    },
    "@generated:grid3d:react",
    "@generated:grid3d:wgpu",
    {
      "name": "🛠️dev🀄️wfc🧱️grid3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- grid3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 204.2
      }
    },
    "@generated:wfc3d:react",
    "@generated:wfc3d:wgpu",
    {
      "name": "🛠️dev🀄️wfc🧊️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- wfc3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 205.2
      }
    },
    {
      "name": "🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- procedural 3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6018",
        "SEMIO_RENDERER": "react",
        "SEMIO_DEFAULT_EXAMPLE": "hexagonal-mushroom-column"
      },
      "presentation": {
        "group": "3_dev",
        "order": 200
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6018)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- procedural 3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6118",
        "SEMIO_RENDERER": "wgpu",
        "SEMIO_DEFAULT_EXAMPLE": "hexagonal-mushroom-column"
      },
      "presentation": {
        "group": "3_dev",
        "order": 200.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6118)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🔧️procedural🏙️3d👁️viewer⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- procedural 3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6018",
        "SEMIO_RENDERER": "react",
        "SEMIO_APP_ROLE": "viewer"
      },
      "presentation": {
        "group": "3_dev",
        "order": 200.2
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6018)",
        "uriFormat": "%s/?plugin=generation3d"
      }
    },
    {
      "name": "🛠️dev🔧️procedural🏙️3d👁️viewer🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- procedural 3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6118",
        "SEMIO_RENDERER": "wgpu",
        "SEMIO_APP_ROLE": "viewer"
      },
      "presentation": {
        "group": "3_dev",
        "order": 200.3
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6118)",
        "uriFormat": "%s/?plugin=generation3d&role=viewer"
      }
    },
    {
      "name": "🛠️dev📽️projektetage",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-praesentation-projektetage:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "PRAESENTATION_PROJEKTETAGE_PORT": "6050"
      },
      "presentation": {
        "group": "3_dev",
        "order": 210
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6050)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev♻️mit-bestand📚️bericht",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-bericht:watch",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 211
      }
    },
    "@generated:aggregator:react",
    {
      "name": "🛠️dev♻️mit-bestand🧺️demonstrator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-demonstrator:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "MIT_BESTAND_DEMONSTRATOR_PORT": "6029",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 213
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6029)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "♻️activate🏚️mitbestand🎪️demonstrator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-demonstrator:activate-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.1
      }
    },
    {
      "name": "🖥️serve🏚️mitbestand🎪️demonstrator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-demonstrator:serve",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.2
      }
    },
    {
      "name": "🛠️dev🏢️semio-tech🎡️play",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TECH_PLAY_PORT": "6033",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 213.3
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6033)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "♻️activate🏢️semio-tech🎡️play",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:activate-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.4
      }
    },
    {
      "name": "🖥️serve🏢️semio-tech🎡️play",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:serve",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.5
      }
    },
    {
      "name": "🛠️dev🎓️teaching🏛️architecture❓️quiz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "TEACHING_ARCHITECTURE_QUIZ_PORT": "6061",
        "PROCTOR_PORT": "8791"
      },
      "presentation": {
        "group": "3_dev",
        "order": 213.6
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6061)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🎓️teaching🛂️proctor",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "PROCTOR_PORT": "8791"
      },
      "presentation": {
        "group": "3_dev",
        "order": 213.61
      }
    },
    {
      "name": "🔁️rebuild🎓️teaching🛂️proctor",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:rebuild",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.611
      }
    },
    {
      "name": "🧪️test❓️quiz🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/quiz:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.63
      }
    },
    {
      "name": "🧪️test❓️quiz⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/quiz-react:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.64
      }
    },
    {
      "name": "🧪️test❓️quiz🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/quiz-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.65
      }
    },
    {
      "name": "🧪️test🎓️teaching🛂️proctor🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.66
      }
    },
    {
      "name": "🧪️test🎓️teaching🏛️architecture❓️quiz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.67
      }
    },
    {
      "name": "🛠️dev❓️quiz⚛️react🪁️typecheck",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/quiz-react:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.68
      }
    },
    "@generated:generator:react",
    "@generated:koordinator:react",
    "@generated:aussuchen:react",
    "@generated:bearbeiten:react",
    "@generated:verfolgen:react",
    "@generated:puzzle2d:react",
    "@generated:puzzle2d:wgpu",
    {
      "name": "🛠️dev🧩️puzzle🩻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 220.2
      }
    },
    "@generated:puzzle3d:react",
    "@generated:puzzle3d:wgpu",
    {
      "name": "🛠️dev🧩️puzzle🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 230.2
      }
    },
    {
      "name": "🛠️dev🧩️puzzle🏙️3d🎛️concrete🌲️forest⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 3d fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_3D_PLAY_PORT": "6013",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 240
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6013)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle🏙️3d🎛️concrete🌲️forest🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 3d fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_3D_PLAY_PORT": "6113",
        "SEMIO_RENDERER": "wgpu"
      },
      "presentation": {
        "group": "3_dev",
        "order": 240.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6113)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle🏙️3d🎛️concrete🌲️forest🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 240.2
      }
    },
    "@generated:puzzle5d:react",
    "@generated:puzzle5d:wgpu",
    {
      "name": "🛠️dev🧩️puzzle👯️5d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle5d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 250.2
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️concrete🌲️forest⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 5d fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_5D_PLAY_PORT": "6014",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 260
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6014)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️concrete🌲️forest🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 5d fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_5D_PLAY_PORT": "6114",
        "SEMIO_RENDERER": "wgpu"
      },
      "presentation": {
        "group": "3_dev",
        "order": 260.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6114)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️concrete🌲️forest🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle5d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 260.2
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️capsule🌙️dream⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 5d",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_5D_PLAY_PORT": "6015",
        "SEMIO_RENDERER": "react",
        "PLAYGROUND_LOCKED_EXAMPLE_ID": "capsule-dream",
        "SEMIO_DEFAULT_EXAMPLE": "capsule-dream"
      },
      "presentation": {
        "group": "3_dev",
        "order": 261
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6015)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️capsule🌙️dream🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 5d",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_5D_PLAY_PORT": "6115",
        "SEMIO_RENDERER": "wgpu",
        "PLAYGROUND_LOCKED_EXAMPLE_ID": "capsule-dream",
        "SEMIO_DEFAULT_EXAMPLE": "capsule-dream"
      },
      "presentation": {
        "group": "3_dev",
        "order": 261.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6115)",
        "uriFormat": "%s"
      }
    },
    "@generated:block2d:react",
    "@generated:block2d:wgpu",
    {
      "name": "🛠️dev🧱️block🩻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- block2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 261.2
      }
    },
    "@generated:block3d:react",
    "@generated:block3d:wgpu",
    {
      "name": "🛠️dev🧱️block🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- block3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 262.2
      }
    },
    "@generated:block5d:react",
    "@generated:block5d:wgpu",
    {
      "name": "🛠️dev🧱️block👯️5d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- block5d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 263.2
      }
    },
    "@generated:reasoning-wires:react",
    "@generated:reasoning-wires:wgpu",
    {
      "name": "🛠️dev🧠️reasoning🔗️wires🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- reasoning-wires",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 270.2
      }
    },
    {
      "name": "🛠️dev🧰️repo🤖️mcp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp repo",
      "env": {
        "MCP_AUTO_OPEN_ENABLED": "false",
        "MCP_PROXY_AUTH_TOKEN": "repo-mcp-token"
      },
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://[^\\s]+:6274/\\?MCP_PROXY_AUTH_TOKEN=[^\\s]+)",
        "uriFormat": "%s&transport=stdio&serverCommand=cargo&serverArgs=run&serverArgs=--release&serverArgs=-p&serverArgs=semio-framework-repo-cli&serverArgs=--&serverArgs=mcp&MCP_PROXY_FULL_ADDRESS=http://127.0.0.1:6277"
      }
    },
    {
      "name": "🛠️dev🧰️repo🤖️mcp⌨️cursor",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp stdio cursor",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.1
      }
    },
    {
      "name": "🛠️dev🧰️repo⌨️cli🦀️rust",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:run --args=\"--help\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.15
      }
    },
    {
      "name": "🛠️dev🧰️repo⌨️cli🦀️semio-repo",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:repo --args=\"--help\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.16
      }
    },
    {
      "name": "🛠️dev🧰️repo🔌️mcp🦀️rust",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:mcp",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.17
      }
    },
    {
      "name": "🛠️dev🧰️repo⌨️client",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:run",
      "env": {
        "SEMIO_REPO_IMPLEMENTATION": "go",
        "GOWORK": "${workspaceFolder}/go.work"
      },
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.25
      }
    },
    {
      "name": "🛠️build🧰️repo🔌️mcp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo-mcp:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.4
      }
    },
    {
      "name": "🛠️dev🧰️repo🖥️coordinator🐹️go",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator-go:dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.5
      }
    },
    {
      "name": "🛠️dev🧰️repo🖥️coordinator🦀️rust",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator-rs:run",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.6
      }
    },
    {
      "name": "🛠️dev🧰️repo🖥️coordinator🟦️typescript",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator:dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.7
      }
    },
    "@generated:shooting:react",
    "@generated:shooting:wgpu",
    {
      "name": "🛠️dev📸️shooting🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- shooting",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 290.2
      }
    },
    {
      "name": "🛠️dev📸️shooting🎛️base⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- shooting fixture base-icon",
      "cwd": "${workspaceFolder}",
      "env": {
        "SHOOTING_PLAY_PORT": "6019",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 300
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6019)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📸️shooting🎛️base🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- shooting fixture base-icon",
      "cwd": "${workspaceFolder}",
      "env": {
        "SHOOTING_PLAY_PORT": "6119",
        "SEMIO_RENDERER": "wgpu"
      },
      "presentation": {
        "group": "3_dev",
        "order": 300.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6119)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📸️shooting🎛️base🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- shooting",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 300.2
      }
    },
    {
      "name": "🛠️dev📖️storybook",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 310
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🧩️puzzle◻️2d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-puzzle-2d",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 340
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🖱️ui",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-ui",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 350
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🎨️styling",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-styling",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 460
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🧩️puzzle🧊️3d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-puzzle-3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 470
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🧩️puzzle🖐️5d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-puzzle-5d",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 480
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🛠️framework",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-framework",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 490
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🛠️framework🔌️hosts",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-framework-hosts",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 400
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🛠️framework🖥️os",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-framework-os",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 410
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook♾️infinite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-infinite",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 420
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook📐️cad",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-cad",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 430
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🎬️animate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-animate",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 450
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    "@generated:trinity-jack:react",
    "@generated:trinity-jack:wgpu",
    {
      "name": "🛠️dev🔺️trinity🃏️jack🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- trinity",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 360.2
      }
    },
    {
      "name": "🛠️dev🔺️trinity🃏️jack🦀️shell",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-jack-shell:run -- trinity/fixture/nakagin-capsule-tower.trinity.json \"MATCH (a:Piece) RETURN a.name\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 370
      }
    },
    "@generated:trinity-rewriting:react",
    "@generated:trinity-rewriting:wgpu",
    {
      "name": "🛠️dev🔺️trinity♻️rewriting🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- trinity-rewriting",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 380.2
      }
    },
    "@generated:forms:react",
    "@generated:forms:wgpu",
    {
      "name": "🛠️dev📋️forms🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- forms",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 385.2
      }
    },
    {
      "name": "🧪️test🖨️raster🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/raster-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.7
      }
    },
    {
      "name": "🧪️test🖨️raster🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/raster-raster-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.8
      }
    },
    {
      "name": "🧪️test🔲️pixels🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/pixels:test-typescript",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.9
      }
    },
    {
      "name": "🧪️test🔲️pixels🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/pixels:test-rust",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387
      }
    },
    "@generated:raster:react",
    "@generated:raster:wgpu",
    {
      "name": "🛠️dev🖼️raster🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- raster",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.2
      }
    },
    "@generated:vcs:react",
    "@generated:vcs:wgpu",
    {
      "name": "🛠️dev🗄️vcs🎛️play🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- vcs",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 385.2
      }
    },
    "@generated:s:react",
    "@generated:s:wgpu",
    "@generated:s:users",
    {
      "name": "🛠️dev🖥️s⚛️react📦️served",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- s served",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6070",
        "SEMIO_PLUGIN": "s",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 386.05
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6070)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🖥️s🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- s",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.1
      }
    },
    {
      "name": "🛠️dev🦀️os-plugins",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:plugin",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.5
      }
    },
    {
      "name": "🛠️dev🦀️os-plugins📏️size",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:plugin -- size",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.6
      }
    },
    {
      "name": "🛠️dev🦀️os-plugins🧫️scale-fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:generate-scale-fixture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.7
      }
    },
    {
      "name": "🛠️dev🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):8787)",
        "uriFormat": "%s/admin"
      }
    },
    {
      "name": "🛠️dev🗄️os-hub🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-postgres",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev-postgres/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.002
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):8787)",
        "uriFormat": "%s/admin"
      }
    },
    {
      "name": "🛠️dev🗄️os-hub🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-neo4j",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev-neo4j/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.003
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):8787)",
        "uriFormat": "%s/admin"
      }
    },
    {
      "name": "🛠️dev🐳️hub-backends⬆️up",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-up -- all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.004
      }
    },
    {
      "name": "🛠️dev🐳️hub-backends🩺️status",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-status -- all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.005
      }
    },
    {
      "name": "🛠️dev🐳️hub-backends⬇️down",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-down -- all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.006
      }
    },
    {
      "name": "🛠️dev🗄️os-hub🎫️local",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:local-hub-owner",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_HUB_URL": "http://127.0.0.1:8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.01
      }
    },
    {
      "name": "🛠️dev🗄️os-hub🛡️admin",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-admin:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_URL": "http://127.0.0.1:8787"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.05
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):8790)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🪐️space⚛️react🔒local-only",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- s",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6070",
        "SEMIO_PLUGIN": "s",
        "SEMIO_RENDERER": "react",
        "S_LOCAL_ONLY": "1"
      },
      "presentation": {
        "group": "3_dev",
        "order": 386.25
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6070)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🔐️os-secure-suite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-secure-suite",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "S_OS_PORT": "6066",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.06
      }
    },
    {
      "name": "🛠️dev🔐️os-secure-native🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-secure-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/",
        "SEMIO_PLUGIN": "s"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.061
      }
    },
    {
      "name": "🛠️dev🔐️os-secure-mcp🌉️stdio",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-secure-mcp",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.062
      }
    },
    {
      "name": "🛠️dev🔐️os-secure-admin🛡️browser",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-secure-admin",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.063
      }
    },
    {
      "name": "🛠️dev🤝️os-collab-e2e",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-collaboration",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.064
      }
    },
    {
      "name": "🛠️dev🪐️os-s🩺️cold-boot",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:cold-boot-check-s",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.065
      }
    },
    {
      "name": "🛠️dev🪐️os-s🔭️foreign-kind",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:s-host-foreign-kind-s -- http://127.0.0.1:6070/ --tag launch raster dag block=s.block.block2d@1/*#editor block=s.block.block3d@1/*#editor block=s.block.block5d@1/*#editor",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.066
      }
    },
    {
      "name": "🛠️dev🪐️os-s🤏️pinch-a11y",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:s-host-pinch-diagram-contrast-s -- http://127.0.0.1:6070/ --tag launch",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.067
      }
    },
    {
      "name": "🛠️dev🌉️os-mcp🧵️stdio",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp stdio os",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.05
      }
    },
    {
      "name": "🛠️dev🌉️os-mcp🌐️http",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp http os",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.06
      }
    },
    {
      "name": "🛠️dev🌉️os-mcp🤝️client-e2e",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp:client-e2e",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.07
      }
    },
    {
      "name": "🛠️dev⏳️async🛌️worker-parking-check",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-async-rs:worker-parking-native-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.071
      }
    },
    {
      "name": "🖱️mcpinspector🌉️os",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x @modelcontextprotocol/inspector --config .mcp.json --server semio",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "2_mouse",
        "order": 31
      }
    },
    {
      "name": "🛠️dev🕸️os-run",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:os -- run",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.1
      }
    },
    "@generated:draw:react",
    "@generated:draw:wgpu",
    {
      "name": "🛠️dev✏️draw🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- draw",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.2
      }
    },
    "@generated:note:react",
    "@generated:note:wgpu",
    {
      "name": "🛠️dev📝️note🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- note",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 388.2
      }
    },
    "@generated:writer:react",
    "@generated:writer:wgpu",
    {
      "name": "🛠️dev✍️writer🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- writer",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.2
      }
    },
    "@generated:remodel:react",
    "@generated:remodel:wgpu",
    {
      "name": "🛠️dev🏺️remodel🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- remodel",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 389.2
      }
    },
    {
      "name": "🛠️dev🖱️ui🪁️typecheck",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-react:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 390
      }
    },
    {
      "name": "🛠️dev🧰️framework🪁️typecheck",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 390.1
      }
    },
    {
      "name": "🛠️dev💻️os🪁️typecheck",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 390.2
      }
    },
    "@generated:fem2d:react",
    "@generated:fem2d:wgpu",
    {
      "name": "🛠️dev🏗️fem🩻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- fem2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 392.2
      }
    },
    "@generated:fem3d:react",
    "@generated:fem3d:wgpu",
    {
      "name": "🛠️dev🏗️fem🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- fem3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 392.5
      }
    },
    {
      "name": "🛠️dev🧊️wgpu🖥️native🚢️release",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native-release -- s",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "0_dev",
        "order": 121.4
      }
    },
    {
      "name": "🛠️dev♻️mit-bestand📋️zwischenbericht",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-bericht:watch-zwischenbericht",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "0_dev",
        "order": 392
      }
    },
    {
      "name": "🛠️dev♻️mit-bestand📑️forschungsbericht",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-bericht:watch-forschungsbericht",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "0_dev",
        "order": 392.001
      }
    },
    {
      "name": "🛠️dev♻️mit-bestand kompaktbericht",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-bericht:watch-kompaktbericht",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "0_dev",
        "order": 392.002
      }
    },
    {
      "name": "📦️build✏️s",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 9
      }
    },
    {
      "name": "📦️build🖥️s",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-s-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 10
      }
    },
    {
      "name": "📦️build📐️cad",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-cad-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 10
      }
    },
    {
      "name": "📦️build🏢️semio-tech🎡️play",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11
      }
    },
    {
      "name": "⚖️gate🏢️semio-tech🎡️play🎭️e2e",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:test-e2e",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11
      }
    },
    {
      "name": "📦️build🎓️teaching🏛️architecture❓️quiz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.1
      }
    },
    {
      "name": "🚚️publish🎓️teaching🏛️architecture❓️quiz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.15
      }
    },
    {
      "name": "📦️build🎓️teaching🛂️proctor",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.2
      }
    },
    {
      "name": "📦️build🎓️teaching🏛️architecture❓️quiz🐳️docker-image",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:docker-image-build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.3
      }
    },
    {
      "name": "🚚️publish🎓️teaching🏛️architecture❓️quiz🐳️docker-image",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:docker-image-publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.35
      }
    },
    {
      "name": "✅️check🎓️teaching🏛️architecture❓️quiz📚️catalog",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11.1
      }
    },
    {
      "name": "✅️check🎓️teaching🛂️proctor📚️catalog",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11.15
      }
    },
    {
      "name": "⚖️gate🎓️teaching🏛️architecture❓️quiz🐳️docker-image",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:docker-image-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11.2
      }
    },
    {
      "name": "⚖️gate🎓️teaching🏛️architecture❓️quiz🐳️docker-stack",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:docker-stack-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11.25
      }
    },
    {
      "name": "📦️build🧩️puzzle🏙️3d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-puzzle3d-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 140
      }
    },
    {
      "name": "📦️build🧩️puzzle👯️5d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-puzzle5d-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 150
      }
    },
    {
      "name": "📦️build📸️shooting",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-shooting-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 180
      }
    },
    {
      "name": "📦️check🧊️wgpu🔒️lockfile",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:lockfile-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.145
      }
    },
    {
      "name": "📦️generate🧊️wgpu🚀️boot",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:generate-browser-boot",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.146
      }
    },
    {
      "name": "📦️check🧊️wgpu🌐️browser",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:check-browser-worker",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.147
      }
    },
    {
      "name": "📦️check⚛️react🔐️hub-sign-in",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-react:hub-sign-in-spaces-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1475
      }
    },
    {
      "name": "📦️test🧊️wgpu🖱️ui",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:test-wgpu-engine",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.148
      }
    },
    {
      "name": "📦️test🧊️wgpu📺️renderer",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:test-wgpu-unit",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.149
      }
    },
    {
      "name": "📦️test🧊️wgpu♾️infinite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-infinite:test-wgpu-world-terrain",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.15
      }
    },
    {
      "name": "📦️build🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.159
      }
    },
    {
      "name": "📦️build-dev🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:build-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16
      }
    },
    {
      "name": "🚚️publish🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1601
      }
    },
    {
      "name": "📦️generate🌐️gis🧬️native-codec-projection",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-plugin:native-codec-projection",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16013
      }
    },
    {
      "name": "📦️generate🗄️stdio🧬️native-codec-projection",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:native-codec-projection",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16014
      }
    },
    {
      "name": "🛫️preflight-catalog🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:trusted-catalog-preflight --packages all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.160145
      }
    },
    {
      "name": "🚚️publish-catalog🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:trusted-catalog-bootstrap --packages all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16015
      }
    },
    {
      "name": "🧊️check-guest-framework🔌️plugin-registry",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:guest-framework-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.160155
      }
    },
    {
      "name": "🔁️rebuild-all🔌️plugin-registry",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:rebuild-all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16016
      }
    },
    {
      "name": "📦️build-release🌉️os-mcp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp-rs:build-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1602
      }
    },
    {
      "name": "🚚️publish🌉️os-mcp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp-rs:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1603
      }
    },
    {
      "name": "📦️test🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.161
      }
    },
    {
      "name": "📦️test🗄️os-hub♾️all-features",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:test-all-features",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.162
      }
    },
    {
      "name": "📦️test🖥️server",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-server-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1625
      }
    },
    {
      "name": "📦️test🖥️server🟦️typescript",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-server:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1626
      }
    },
    {
      "name": "📦️check🗄️os-hub🚀️launch",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:local-bootstrap-launch-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.163
      }
    },
    {
      "name": "📦️check🤖️generated🔬️corruption",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-generated-corruption",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.164
      }
    },
    {
      "name": "📦️check🗄️os-hub🟦️types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.165
      }
    },
    {
      "name": "📦️check🦑️repo🟦️types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.166
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️root-clean-scaffold-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-clean-scaffold-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.167
      }
    },
    {
      "name": "🧬schema🗺️surface🏛️abstraction🧪source-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-surface-abstraction-law-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.168
      }
    },
    {
      "name": "🧬schema🗿artifact🧪root-artifact-schema-law-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-artifact-schema-law-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.169
      }
    },
    {
      "name": "🧬schema💡️inference🧪source-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-inference-law-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.17
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️root-schema-field-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-schema-field-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.171
      }
    },
    {
      "name": "🧹clean🦑️repo🧪️source-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-repo-source-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.172
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🎚️vitest-configuration-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-vitest-configuration-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.173
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🎚️tool-configuration-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-tool-configuration-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.174
      }
    },
    {
      "name": "🧹clean🧩️taxonomy📦️package-body-policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-package-body-policy",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.175
      }
    },
    {
      "name": "📦️test🥾️bootstrap🪟️cross-platform",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-cross-platform-bootstrap",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.176
      }
    },
    {
      "name": "📦️test🏃️process🪓️tree-termination",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-process-tree-termination",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.177
      }
    },
    {
      "name": "📦️test🪟️windows🧭️command-paths",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-windows-command-paths",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1775
      }
    },
    {
      "name": "📦️test🦀️cargo🧾️provenance",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-cargo-provenance",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.178
      }
    },
    {
      "name": "🧹clean🦀️cargo🧾️provenance",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:cargo-provenance-repair",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.179
      }
    },
    {
      "name": "📦️check🗿️taxonomy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-taxonomy-report",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.165
      }
    },
    {
      "name": "📦️check🗿️taxonomy🧩️implementation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-taxonomy-implementation-report",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.166
      }
    },
    {
      "name": "📦️check🔒️dependencies📃️literal-external",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-dependencies-literal-external",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.167
      }
    },
    {
      "name": "🏛️check🧩️canonical-architecture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 899.99
      }
    },
    {
      "name": "📦️check🧅️layering",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-layering",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.168
      }
    },
    {
      "name": "📦️check🔌️plugin-registry",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.169
      }
    },
    {
      "name": "📦️generate🖨️print📊️viz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:generate-viz",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1689
      }
    },
    {
      "name": "📦️generate📕️norm🪨️en1996🖼️example-assets",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996-rs:regenerate-example-assets",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16915
      }
    },
    {
      "name": "📦️generate🧬️surface-schema",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:surface-schema",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1691
      }
    },
    {
      "name": "📦️check🧬️surface-schema",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:surface-schema-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1692
      }
    },
    {
      "name": "📦️generate✨️dsl-derive🧬️mutation-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/dsl-derive-rs:generate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1693
      }
    },
    {
      "name": "📦️check✨️dsl-derive🧬️mutation-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/dsl-derive-rs:check-generated",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1694
      }
    },
    {
      "name": "📦️check🧹️fixture-sweep",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fixture-sweep-rs:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.17
      }
    },
    {
      "name": "📦️check🌊️flow-composition",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-flow-composition-rs:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1701
      }
    },
    {
      "name": "📦️check🌐️semio-session",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-spatial-kernel-semio-session-rs:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1702
      }
    },
    {
      "name": "📦️check📡️channel-version",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:channel-version-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.171
      }
    },
    {
      "name": "🛠️dev📡️channel-version🏭️generate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:channel-version-generate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.172
      }
    },
    {
      "name": "📦️wasm🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:wasm",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 500.81
      }
    },
    {
      "name": "📦️wasm-release🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:wasm-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 500.811
      }
    },
    {
      "name": "⚖️gate🧱️hub-foundations📐️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:foundation-source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10755
      }
    },
    {
      "name": "⚖️gate🧭️local-relay📛️admission",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:local-relay-routing-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10756
      }
    },
    {
      "name": "⚖️gate🔐️hub-auth🤝️live-sign-in",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:live-sign-in-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10757
      }
    },
    {
      "name": "⚖️gate🔐️hub-auth🧊️wgpu-live-journey",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:hub-live-journey-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🔐️hub-auth🧊️wgpu-live-creation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:hub-live-creation-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🔐️hub-auth🧊️wgpu-live-collaboration",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:hub-live-collaboration-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🧊️wgpu⏯️native-guest-journey",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native-guest-journey-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🚨️capability-audit",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp-rs:capability-audit-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🤖️live-agent-loop",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:live-agent-loop-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10758
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🤖️live-agent-loop🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:live-agent-loop-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_MCP_LIVE_LOCALE": "de"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.107582
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🤝️hub-edit-durability",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:hub-edit-durability-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107585
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🤝️hub-agent-participant",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:hub-agent-participant-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10759
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp💬️agent-reply",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:agent-reply-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107595
      }
    },
    {
      "name": "🧹clean🧩️taxonomy❄️frozen-markdown-coordinates",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-frozen-markdown-coordinates --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.18
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🕰️historical-json-source-encoding",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-historical-json-source-encoding --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.19
      }
    },
    {
      "name": "⚖️gate🌊️flow🌐️startup",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-flow-core:test-browser",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.8
      }
    },
    {
      "name": "⚖️gate🌊️flow⏱️consumed-clock",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-flow-core:test-browser-clock",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.81
      }
    },
    {
      "name": "⚖️gate🌊️flow🏷️browser-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-flow-core:test-browser-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.82
      }
    },
    {
      "name": "📦️preview🤖️flow-browser-package",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-flow-core:preview-generated",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 407.83
      }
    },
    {
      "name": "📦️build🗄️stdio",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 407.84
      }
    },
    {
      "name": "📦️check🗄️stdio",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 407.85
      }
    },
    {
      "name": "⚖️gate🗄️stdio🧩️composition",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.86
      }
    },
    {
      "name": "⚖️gate🗄️stdio🛂️package-contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:package-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.87
      }
    },
    {
      "name": "⚖️gate🗄️stdio🕸️package-graph",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:package-graph",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.88
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️editing🧬️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-artifact-contract-rs:test -- --lib",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.89
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️editors🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run-many --target=test --projects=\"@semio-tech/stdio-*-rs\" --exclude=@semio-tech/stdio-artifact-contract-rs --parallel=2 -- --features component-app-assembly --lib editor",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.9
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️catalog🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:test -- --test editor_catalog",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.91
      }
    },
    {
      "name": "⚖️gate🗄️stdio🚢️shipped-fleet🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:test -- --test shipped_fleet",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.915
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️editing🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-snapshot-editing-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.92
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎬️lanes🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-scene-rs:test -- --lib lanes_preserve_complete_unicode_documents",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.93
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎬️lanes🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-scene-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.94
      }
    },
    {
      "name": "⚖️gate🗄️stdio🪟️kits🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:test -- window_kits_tests",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.95
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️component🌐️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:editor-component-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.96
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️fixture🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:test -- editor-catalog-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.97
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️launch🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test -- 🧪️tests/🚀️launch/🟦️.ts",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.98
      }
    },
    {
      "name": "⚖️gate🌱️value💾️resident",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/value-resident:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.32
      }
    },
    {
      "name": "⚖️gate🌱️value💾️resident🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/value-resident-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.33
      }
    },
    {
      "name": "⚖️gate🌱️value💾️resident🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/value-resident-rs:check-wasm --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.34
      }
    },
    {
      "name": "⚖️gate🖱️ui🖥️host🔣️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/ui-host-rs:test-source --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.35
      }
    },
    {
      "name": "⚖️gate🖱️ui🖥️host🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/ui-host-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.36
      }
    },
    {
      "name": "⚖️gate🖱️ui🖥️host🦀️check",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/ui-host-rs:check --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.37
      }
    },
    {
      "name": "⚖️gate🖱️ui🖥️host🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/ui-host-rs:check-wasm --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.38
      }
    },
    {
      "name": "⚖️gate🔌️plugin🦀️lib",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.39
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️artifact-support",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-support --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.06
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🏺️historical-package-owner-identity",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-historical-package-owner-identity --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.07
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧲️rust-physical-reference-context",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-physical-reference-context --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.08
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️cli-cancellation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-taxonomy-cli-cancellation --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.09
      }
    },
    {
      "name": "🧹clean🧩️taxonomy💠️inventory-artifact-shards",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-inventory-artifact-shards --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.1
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️root-script-compiler",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-script-compiler",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.11
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🔎️json-reference-owner-lookup",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-json-reference-owner-lookup --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.12
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🚧️cargo-discovery-exclusions",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-cargo-discovery-exclusions --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.13
      }
    },
    {
      "name": "🧹clean🧩️taxonomy💥️nested-cargo-collision-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-nested-cargo-collision-authority --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.14
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🌐️registry-import-language",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-registry-import-language --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.15
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🛫️preflight-reference-basis",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-preflight-reference-basis --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.16
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🛤️typescript-path-collection",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-typescript-path-collection --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.17
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🗺️testing-readme-coordinates",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-testing-readme-coordinates --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.192
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🥤️rust-finite-target-consumption",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-finite-target-consumption --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}/rust-finite-target-consumption"
      },
      "presentation": {
        "group": "4_gate",
        "order": 410.193
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️artifact-source-residue",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-source-residue --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.194
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🔖️readme-current-source-revision",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-readme-current-source-revision --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.195
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🚚️readme-move-source-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-readme-move-source-authority --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.197
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️artifact-source-commit",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-source-commit --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.198
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🫙️artifact-empty-facet-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-empty-facet-authority --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.199
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🟢️readme-current-source-activation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-readme-current-source-activation --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.204
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🪶️artifact-empty-facet-authoring",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-empty-facet-authoring --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.205
      }
    },
    {
      "name": "🧹clean🧩️taxonomy👀️readme-reviewed-fixture-inputs",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-readme-reviewed-fixture-inputs --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.206
      }
    },
    {
      "name": "🧹clean🧩️taxonomy♻️taxonomy-pattern-compiler-reuse",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-taxonomy-pattern-compiler-reuse --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.207
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🔤️taxonomy-leading-grapheme",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-taxonomy-leading-grapheme --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.209
      }
    },
    {
      "name": "🧹clean🧩️taxonomy📈️reference-coordinate-progress",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-reference-coordinate-progress --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.21
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🎯️draw-destination-observation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-draw-destination-observation --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.211
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧾️registry-catalog-gitlink-boundary",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-registry-catalog-gitlink-boundary --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.2152
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🎯️cargo-target-discovery-skip",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-cargo-target-discovery-skip --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.2154
      }
    },
    {
      "name": "⚖️gate🔌️socket-grant📐️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:socket-grant-command-source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.09999
      }
    },
    {
      "name": "⚖️gate🔌️socket-grant🛡️server",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:socket-grant-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.1
      }
    },
    {
      "name": "⚖️gate📌️check-in🧬️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:check-in-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.11
      }
    },
    {
      "name": "⚖️gate📌️check-in🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:check-in-native-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.12
      }
    },
    {
      "name": "⚖️gate📌️check-in🌉️process",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:check-in-process-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.13
      }
    },
    {
      "name": "⚖️gate🔐️wal-writer-fence🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:wal-writer-fence-live -- sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.15
      }
    },
    {
      "name": "⚖️gate🔐️wal-writer-fence🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- postgres -- bun nx run @semio-tech/framework-os-kernel:wal-writer-fence-live -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.151
      }
    },
    {
      "name": "⚖️gate🔐️wal-writer-fence🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- neo4j -- bun nx run @semio-tech/framework-os-kernel:wal-writer-fence-live -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.152
      }
    },
    {
      "name": "⚖️gate🗂️directory-live-lanes🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- postgres -- bun nx run os-hub:directory-live-lanes -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.153
      }
    },
    {
      "name": "⚖️gate🗂️directory-live-lanes🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- neo4j -- bun nx run os-hub:directory-live-lanes -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.154
      }
    },
    {
      "name": "⚖️gate🤝️two-client-document🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:two-client-e2e -- sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.16
      }
    },
    {
      "name": "⚖️gate🤝️two-client-document🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:two-client-e2e -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.17
      }
    },
    {
      "name": "⚖️gate🤝️two-client-document🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:two-client-e2e -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.18
      }
    },
    {
      "name": "⚖️gate📈️document-growth🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:document-growth-e2e -- sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.19
      }
    },
    {
      "name": "⚖️gate📈️document-growth🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:document-growth-e2e -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2
      }
    },
    {
      "name": "⚖️gate📈️document-growth🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:document-growth-e2e -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.21
      }
    },
    {
      "name": "⚖️gate🎯️repo-goal",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test-domain:acceptance-goal",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.22
      }
    },
    {
      "name": "⚖️browser-dock-contract🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.26
      }
    },
    {
      "name": "⚖️browser-dock-acceptance⚛️react🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- run react \"${input:wgpuDockReactServe}\" --output \"${workspaceFolder}/${input:wgpuDockArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.261
      }
    },
    {
      "name": "⚖️browser-dock-acceptance🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- run wgpu \"${input:wgpuDockServe}\" --output \"${workspaceFolder}/${input:wgpuDockArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.262
      }
    },
    {
      "name": "⚖️browser-media-acceptance🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-media-acceptance -- --serve \"${input:wgpuMediaServe}\" --locale ${input:wgpuMediaLocale} --output \"${workspaceFolder}/${input:wgpuMediaArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.263
      }
    },
    {
      "name": "⚖️parity-journey🧑‍💻dev🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:parity-journey -- --renderer ${input:rendererParityRenderer} --react-serve \"${input:wgpuDockReactServe}\" --wgpu-serve \"${input:wgpuDockServe}\" --locale ${input:wgpuMediaLocale} --output \"${workspaceFolder}/${input:rendererParityJourneyArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.264
      }
    },
    {
      "name": "⚖️gate🎯️repo-goal🔗️hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test-domain:acceptance-goal -- --hub ${input:acceptanceHubUrl} --serve ${input:acceptanceServeUrl} --local-serve ${input:acceptanceLocalServeUrl} --users ${input:acceptanceUsers}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.221
      }
    },
    {
      "name": "⚖️gate🎯️repo-goal📋️plan",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test-domain:acceptance-plan",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.23
      }
    },
    {
      "name": "⚖️gate🧮️program-matrix⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:program-matrix -- --serve http://127.0.0.1:6070/ --tag launch-en --locale en --roles editor,viewer",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.24
      }
    },
    {
      "name": "⚖️gate🧮️program-matrix⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:program-matrix -- --serve http://127.0.0.1:6070/ --tag launch-de --locale de --roles editor,viewer",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.25
      }
    },
    {
      "name": "⚖️gate⏯️tool-run⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:tool-run-matrix -- --serve http://127.0.0.1:6070/ --tag launch-en --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.251
      }
    },
    {
      "name": "⚖️gate⏯️tool-run⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:tool-run-matrix -- --serve http://127.0.0.1:6070/ --tag launch-de --locale de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.252
      }
    },
    {
      "name": "⚖️gate🗂️hub-document-sweep⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:hub-document-sweep -- --serve http://127.0.0.1:6071/ --hub ${input:acceptanceHubUrl} --tag launch-en --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.253
      }
    },
    {
      "name": "⚖️gate🗂️hub-document-sweep⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:hub-document-sweep -- --serve http://127.0.0.1:6071/ --hub ${input:acceptanceHubUrl} --tag launch-de --locale de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.254
      }
    },
    {
      "name": "⚖️gate🚪️io-matrix⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:io-matrix -- --serve http://127.0.0.1:6070/ --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.255
      }
    },
    {
      "name": "⚖️gate🚪️io-matrix⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:io-matrix -- --serve http://127.0.0.1:6070/ --locale de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.256
      }
    },
    {
      "name": "⚖️gate👥️two-human⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:two-human -- --hub ${input:acceptanceHubUrl} --serve ${input:acceptanceServeUrl} --users ${input:acceptanceUsers} --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.26
      }
    },
    {
      "name": "⚖️gate👥️two-human⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:two-human -- --hub ${input:acceptanceHubUrl} --serve ${input:acceptanceServeUrl} --users ${input:acceptanceUsers} --locale de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.27
      }
    },
    {
      "name": "⚖️gate🔀️connection-budget⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:connection-budget -- ${input:acceptanceServeUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.275
      }
    },
    {
      "name": "⚖️gate💤️idle-budget⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:idle-budget -- ${input:acceptanceServeUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.276
      }
    },
    {
      "name": "⚖️gate🫧️memory-soak⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:memory-soak -- ${input:acceptanceServeUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.277
      }
    },
    {
      "name": "⚖️gate⏱️interaction-latency⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:interaction-latency -- ${input:acceptanceServeUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.278
      }
    },
    {
      "name": "⚖️gate⏪️time-travel⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:time-travel -- --serve http://127.0.0.1:6012/ --renderer react --locales en,de --chords en,de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2781
      }
    },
    {
      "name": "⚖️gate⏪️time-travel🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:time-travel -- --serve http://127.0.0.1:6112/ --renderer wgpu --locales en,de --chords en,de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2782
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🧩️plugin-coverage",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:plugin-coverage-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.28
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🚶️user-path",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:user-path-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_MCP_HUB_ORIGIN": "${input:acceptanceHubUrl}",
        "S_OS_MCP_LIVE_SHELL_URL": "${input:acceptanceServeUrl}",
        "S_OS_MCP_LIVE_LOCALE": "en"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.29
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🚶️user-path🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:user-path-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_MCP_HUB_ORIGIN": "${input:acceptanceHubUrl}",
        "S_OS_MCP_LIVE_SHELL_URL": "${input:acceptanceServeUrl}",
        "S_OS_MCP_LIVE_LOCALE": "de"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🛡️security",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:security-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_MCP_HUB_ORIGIN": "${input:acceptanceHubUrl}",
        "OS_HUB_ADMIN_CAPABILITY_FILE": "${input:acceptanceHubAdminCapability}"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.301
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp💼️inference-quartet",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:inference-quartet-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_MCP_HUB_ORIGIN": "${input:acceptanceHubUrl}"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.302
      }
    },
    {
      "name": "⚖️gate💾️hub-backup-restore",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backup-restore-drill",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️gate🛑️hub-graceful-shutdown",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:shutdown-drill",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.315
      }
    },
    {
      "name": "⚖️gate🧠️hub-residency",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:residency-watch -- --hub ${input:acceptanceHubUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.32
      }
    },
    {
      "name": "⚖️gate🌅️hub-boot-watch",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:boot-watch -- --restarts 1",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.325
      }
    },
    {
      "name": "⚖️gate🏷️hub-freshness",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:hub-freshness -- --hub ${input:acceptanceHubUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️gate🤖️hub-agent-ceiling",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:agent-ceiling-check -- --hub ${input:acceptanceHubUrl} --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.335
      }
    },
    {
      "name": "⚖️gate🌪️reopen-storm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:reopen-storm-check -- all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.331
      }
    },
    {
      "name": "⚖️gate🌪️reopen-storm🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- postgres -- bun nx run @semio-tech/framework-os-kernel:reopen-storm-check -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.332
      }
    },
    {
      "name": "⚖️gate🌪️reopen-storm🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- neo4j -- bun nx run @semio-tech/framework-os-kernel:reopen-storm-check -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.333
      }
    },
    {
      "name": "⚖️gate🚧️production-placeholders",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- production-placeholders",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.34
      }
    },
    {
      "name": "⚖️gate🎛️command-reachability",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity commands",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.35
      }
    },
    {
      "name": "⚖️gate🪆️composed-child-refs",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- composed-child-refs",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.42
      }
    },
    {
      "name": "⚖️gate🚫️history-closure",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- history-closure",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.425
      }
    },
    {
      "name": "⚖️gate🎯️mutation-outcome-law🧪️planted",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:test-outcome-law-gate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.427
      }
    },
    {
      "name": "⚖️gate⚡️interactivity",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.43
      }
    },
    {
      "name": "⚖️gate⚡️interactivity🎯️tool-jobs",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity tool-jobs",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.44
      }
    },
    {
      "name": "⚖️gate⚡️interactivity🧭️apps",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity apps",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.45
      }
    },
    {
      "name": "⚖️gate⚡️interactivity🧭️apps🎛️actions",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity apps --actions",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.46
      }
    },
    {
      "name": "⚖️gate📦️dependencies",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- dependencies",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.47
      }
    },
    {
      "name": "⚖️gate📦️dependencies0️⃣",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- dependencies literal-external",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.48
      }
    },
    {
      "name": "⚖️gate🧿️semio✉️base🔬️carrier-reproduce",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/probe-stdio-semio-v1-base:carrier-reproduce",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.49
      }
    },
    {
      "name": "⚖️gate🧪️test🏭️inventory",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test-domain:test-inventory",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.5
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test-plugin-publication-source-ownership📚️library🟦️",
      "command": "bun nx run @semio-tech/repo-lib:test-plugin-publication-source-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.04
      }
    },
    {
      "name": "⚖️gate🔌️plugin📇️catalog🧬️contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-catalog-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05667
      }
    },
    {
      "name": "⚖️gate🔌️plugin🧾️schema-owner🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:schema-document-authority-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05668
      }
    },
    {
      "name": "⚖️gate🔁️graph revision📚️repo",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-graph-revision",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1806
      }
    },
    {
      "name": "⚖️gate🪪️installation identity🧰️framework",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework:installation-identity-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1805
      }
    },
    {
      "name": "📥️deps javascript lock",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-js-lock",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -43.8
      }
    },
    {
      "name": "⚖️gate📦️javascript dependency commands",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-js-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1804
      }
    },
    {
      "name": "📥️deps cargo lock",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-cargo-lock",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -43.9
      }
    },
    {
      "name": "⚖️gate🔌️plugin📦️deployment🧬️contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-deployment-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05669
      }
    },
    {
      "name": "⚖️gate🔌️plugin🚀️launch🏷️name🧬️contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-launch-name-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0567
      }
    },
    {
      "name": "🧰️framework 🏃️process 🧭️routing 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-routing",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05671
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 💻️os 🔖️channel-version 🏭️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:channel-version-ownership-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05672
      }
    },
    {
      "name": "🧰️framework 💻️os 🎮️playground ⭐️default 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-playground-default-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05673
      }
    },
    {
      "name": "🧰️framework 💻️os 🔖️channel version 📣️contributions 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:channel-version-contributions-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05674
      }
    },
    {
      "name": "🛡️styling verification contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-tokens:test-verification-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05675
      }
    },
    {
      "name": "📏️styling relative sizing",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-tokens:test-relative-sizing",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05676
      }
    },
    {
      "name": "🎥️video container providers",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:verify-video-container-providers",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05677
      }
    },
    {
      "name": "🗒️note artifact document contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/note-note-js:canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05678
      }
    },
    {
      "name": "🧰️framework modules 🛍️product dependency direction",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-framework-module-product-direction",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05679
      }
    },
    {
      "name": "🧰️framework 🦑️repo ⚡️cache owner policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-cache-policy",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.056795
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🟦️workspace 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:bun-contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568099999999
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 🔍️members",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:members-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568199999999
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 📣️members",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:members-write",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05683
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 🛠️prepare",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:prepare",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05684
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🟦️workspace 📦️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-js-all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568499999999
      }
    },
    {
      "name": "🌎️hub 🗄️stdio 🧩️composition 🛠️prepare",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:composition-prepare",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568599999999
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 📥️runtime 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:runtime-contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05687
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 🧩️capability 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:capability-contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05688
      }
    },
    {
      "name": "✏️s 🧿️semio 🧩️composition 🛠️prepare",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-semio-rs:composition-prepare",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05689
      }
    },
    {
      "name": "✏️s 🔊️wav 🧩️architecture 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-wav-rs:canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0569
      }
    },
    {
      "name": "🎭️styling color primitives",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-tokens:test-color-primitives",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05696
      }
    },
    {
      "name": "🌎️hub 🗄️stdio 📼️artifact 🧪️removal",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:parent-removal-check -- \"${workspaceFolder}/${input:stdioRemovalSnapshot}\"",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}"
      },
      "presentation": {
        "group": "4_gate",
        "order": 900.05691
      }
    },
    {
      "name": "🌎️hub 🗄️stdio 📼️artifact 🧪️tests-removal",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:parent-removal-test-check -- \"${workspaceFolder}/${input:stdioRemovalSnapshot}\"",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}"
      },
      "presentation": {
        "group": "4_gate",
        "order": 900.05692
      }
    },
    {
      "name": "🧹clean🧬️schema🧩️subset-contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-schema:test-subset-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05693
      }
    },
    {
      "name": "🧹clean🥽️mesh🛡️transport-contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:test-mesh-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05694
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️root-taxonomy-workflow-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-taxonomy-workflow-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05695
      }
    },
    {
      "name": "🧰️framework 🏃️process 📦️artifact files 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-artifact-files",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05697
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️ Framework Process Test Budgets",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/framework-process:test-budget",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "repo-gate",
        "order": 900.056975
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧪️ Framework Test Adapter Ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/framework-test:test-adapter-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "repo-gate",
        "order": 900.056976
      }
    },
    {
      "name": "🪪️ Framework Identity Grapheme",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/framework-identity:test-grapheme",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "repo-gate",
        "order": 900.056977
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:identityContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🎛️owned execution 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-owned-execution",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05698
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🪓️tree termination 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-process-tree-termination",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05699
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🦑️repo 🎨️workspace styling 📏️pixel policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-styling-pixels",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05701
      }
    },
    {
      "name": "🦑️repo 🎨️workspace styling 🎭️color policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-styling-colors",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05702
      }
    },
    {
      "name": "🗄️stdio 🦛️Semio 🧬️conversion definition contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-semio-rs:conversion-definition-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}/conversion-definition"
      },
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05703
      }
    },
    {
      "name": "🗄️stdio 🧊️GLTF 🧬️canonical architecture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-gltf-rs:canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05704
      }
    },
    {
      "name": "🧰️framework 🏃️process ⏱️execution budgets",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-execution-budget",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05705
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🧪️bounded test command",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-test-command",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05706
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🦑️repo 🧬️native input vocabulary",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-native-input-vocabulary",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05707
      }
    },
    {
      "name": "🗄️stdio 🧊️GLTF 🧬️native schema identity",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-gltf-rs:native-schema-check -- \"${input:stdioRemovalSnapshot}\"",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}/gltf-native-schema"
      },
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05708
      }
    },
    {
      "name": "🧰️framework 🖼️assets 🗺️tile proxy 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:test-tile-proxy-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05709
      }
    },
    {
      "name": "🧪️framework🏃️process🦀️cargo🧭️driver",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-cargo-driver",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05711
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧪️framework🖼️assets🧭️dispatch🛂️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:test-dispatch-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05712
      }
    },
    {
      "name": "🖥️OS 🎭️actor transport 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os:test-actor-transport",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.0571
      }
    },
    {
      "name": "🖥️OS 🎬️media transport 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os:test-media-transport",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.057101
      }
    },
    {
      "name": "🦑️Repo ⚙️native source ownership 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-native-source-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.057102
      }
    },
    {
      "name": "🕸️Graph 🛂️manifest 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-graph:test-manifest-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05713
      },
      "env": {
        "SEMIO_TEST_ARTIFACTS_DIR": "${workspaceFolder}/${input:graphContractArtifacts}"
      }
    },
    {
      "name": "🖋️SVG 🎥️video 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:test-svg-video-contract",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACTS_DIR": "${workspaceFolder}/${input:svgVideoArtifacts}"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05719
      }
    },
    {
      "name": "🏃️Process 🔒️resource leases 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-resource-leases",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.0572
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🏃️Process 🧪️Vitest 🧪️driver contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-vitest-driver",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05721
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧬️Schema 🏷️entity ownership 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-schema:test-entity-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05714
      }
    },
    {
      "name": "🦑️Repo 🏷️entity kinds 🏭️generate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-client:generate-entity-kinds",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05715
      }
    },
    {
      "name": "🦑️Repo 🏷️entity kinds 👁️preview",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-client:preview-generated",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05716
      }
    },
    {
      "name": "🦑️Repo 🏷️entity kinds ✅️check",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-client:check-entity-kinds",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05717
      }
    },
    {
      "name": "🦑️Repo 🏷️entity kinds 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-client:test-entity-kinds",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05718
      }
    },
    {
      "name": "🪧️Logo 🎬️video export",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:logo",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACTS_DIR": "${workspaceFolder}/${input:svgVideoArtifacts}"
      },
      "presentation": {
        "group": "2_build",
        "order": 206.061
      }
    },
    {
      "name": "🏃️Process 📦️artifact publication 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-artifact-publication",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05722
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🏃️Process 📋️owner context 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-owner-context",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05723
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🗄️stdio 🧾️JSON 🧬️native schema identity",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-json-rs:native-schema-check -- \"${input:stdioRemovalSnapshot}\"",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}/json-native-schema"
      },
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05724
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 🧭️router ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-ui-router-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05725
      }
    },
    {
      "name": "🧰️framework 🏃️process 🎯️exact Cargo 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-exact-cargo-laws",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05731
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🦀️native artifacts 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-native-artifacts",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05732
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🌐️Wasm build 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-wasm-build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05733
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🦑️Repo 🦀️dependency direction 🧱️gate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-cargo-dependency-direction",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05739
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}/cargo-direction"
      }
    },
    {
      "name": "🦑️Repo 🦀️dependency direction 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-cargo-dependency-direction",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05738
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}/cargo-direction"
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 🧱️primitives",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:check-ui-primitives",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05726
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 🌐️chrome i18n",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:check-chrome-i18n",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05727
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 📖️Storybook development",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:ui-storybook-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05728
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 📖️Storybook build",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:ui-storybook-build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05729
      }
    },
  
    {
      "name": "🦑️Repo 🧱️fixture law ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-fixture-law-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05734
      }
    },
  
    {
      "name": "📇️Directory 🔏️lease fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:test-directory-lease-fixture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05735
      }
    },
  
    {
      "name": "🌉️MCP ✅️approval fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp:test-approval-request-fixture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05736
      }
    },
  
    {
      "name": "🌊️Flow 🏷️slider labels fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-flow-flow-rs:test-slider-labels-fixture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05737
      }
    },
    {
      "name": "🌐️Locale 🏷️localized label fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-kernel:test-localized-label",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.0574
      }
    },
    {
      "name": "🦑️Repo 🌐️locale law ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-locale-law-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05741
      }
    },
    {
      "name": "🧬️Schema 🧺️mutation leaf registration 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-schema:test-mutation-leaf-registration",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05744
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🌱️Value 🧩️neutral owner 🧪️products absent",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-value:test-neutral-owner --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05745
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🌱️Value 🦀️native 🧪️test",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/value-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05746
      }
    },
    {
      "name": "🦑️Repo 🦀️physical source 🧪️portable policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-direction --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05747
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🦑️Repo 🦀️physical source 🛡️live direction",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-rust-source-direction --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05748
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "⚖️test🧬️validator🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-validator-rs:test-neutral-owner --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05749
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "⚖️document-http-check💻️os🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:document-http-check --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.0575
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🖱️UI 🌐️locale 🧪️strict portable contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-locale:test-contract --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}/ui-locale"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05756
      }
    },
    {
      "name": "⚖️check🖱️ui🌐️locale🟦️types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-locale:check-types --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}/ui-locale"
      },
      "presentation": {
        "group": "9_clean_architecture",
        "order": 901.05756
      }
    },
    {
      "name": "🦑️Repo 🧱️policy 🚀️fresh authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-dependency-policy-bootstrap --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05758
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🔗️rust-binding",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-binding --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05759
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🔗️rust-binding🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-binding-native --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0576
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🦀️source🧾️attributes",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-attributes --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05761
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🦀️source🏘️scopes",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-scopes --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05762
      }
    },
    {
      "name": "🧪️test🦑️repo🧪️test🚷️discovery-boundaries",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test:test-discovery-boundaries --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05763
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🦀️source🕸️graph",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-graph --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05764
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧬️mutation🛂️scaffolding",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-mutation-authority-scaffolding --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05765
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧬️mutation🛂️inventory",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-mutation-authority-inventory --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05766
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧬️mutation🛂️reachability",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-mutation-authority-reachability --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05767
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧬️mutation🛂️type-origin",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-mutation-authority-type-origin --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05768
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library📥️inference",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-nx-project-inference --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05769
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library📥️inference🔁️revision",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-nx-project-inference-revision --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0577
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library📥️inference🔍️imports",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-nx-project-inference-imports --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05771
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧱️rust-source-direction🔗️participation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-participation --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05772
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧾️serialization🔣️json",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-canonical-json --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05773
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧹️normalization🏗️source-services",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-source-services --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05774
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧱️rust-source-direction🪵️root",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-roots --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05775
      }
    },
    {
      "name": "🧪️test🧰️framework🎠️kernel🫧️transient🧬️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-transient:test-contract --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05776
      }
    },
    {
      "name": "🧪️test🧰️framework🫧️transient📦️retained",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-transient:test-retained --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05777
      }
    },
    {
      "name": "🧪️test🧰️framework🧬️schema✅️validator🧩️shape",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-validator-ts:test-shape --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05778
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio🏭️scalar-schema🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run-many --targets=test-snapshot-sqlite-source --projects=@semio-tech/stdio-pdf-rs,@semio-tech/stdio-dxf-rs,@semio-tech/stdio-jpg-rs --parallel=1 --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05779
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio🏭️scalar-schema🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run-many --targets=test-snapshot-sqlite-native --projects=@semio-tech/stdio-pdf-rs,@semio-tech/stdio-dxf-rs,@semio-tech/stdio-jpg-rs --parallel=1 --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057791
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio🏭️scalar-schema🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run-many --targets=test-snapshot-sqlite --projects=@semio-tech/stdio-pdf-rs,@semio-tech/stdio-dxf-rs,@semio-tech/stdio-jpg-rs --parallel=1 --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057792
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio🖊️dwg🎛️controlled-metadata",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-dwg-rs:test-controlled-metadata --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0578
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio📖️pdf♻️recursive-retirement",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-pdf-rs:test-recursive-retirement --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05781
      }
    },
    {
      "name": "🧪️test🦑️repo🧱️rust🧫️fixture-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-fixture-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05782
      }
    },
    {
      "name": "🧪️test🧰️framework💻️os🔌️plugin🤝️cooperative-host",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:cooperative-host-check --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/cooperative-host"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05783
      }
    },
    {
      "name": "🧪️test🧰️framework💻️os🔌️plugin🤝️cooperative-host🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:cooperative-host-native-check --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/cooperative-host"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057831
      }
    },
    {
      "name": "🧪️test🌱️value🛬️portable",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/value-rs:test-portable --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05784
      }
    },
    {
      "name": "🧪️test✏️s🖊️dwg🏗️grammar-shape🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-dwg-rs:test-grammar-shape --skip-nx-cache -- source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05785
      }
    },
    {
      "name": "🧪️test✏️s🖊️dwg🏗️grammar-shape🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-dwg-rs:test-grammar-shape --skip-nx-cache -- native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057851
      }
    },
    {
      "name": "🧪️test✏️s🖊️dwg🏗️grammar-shape",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-dwg-rs:test-grammar-shape --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057852
      }
    },
    {
      "name": "🧪️test🦑️repo🔐️pool-use-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-pool-use-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/pool-use-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05786
      }
    },
    {
      "name": "🧪️test🦑️repo🔔️deferred-wake-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-deferred-wake-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/deferred-wake-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05788
      }
    },
    {
      "name": "🧪️test🦑️repo📍️rust-family-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-family-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework/captured-family"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05789
      }
    },
    {
      "name": "🧪️test🦑️repo📍️rust-family-ownership🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-family-ownership-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework/captured-family"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057891
      }
    },
    {
      "name": "🧪️test🧬️schema🧱️neutrality",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-schema:test-neutrality --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0579
      }
    },
    {
      "name": "🧪️test🗄️stdio🔮️oracle🧩️composition",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-composition --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio-oracle"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05791
      }
    },
    {
      "name": "🧪️test🗄️stdio🔮️oracle🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-native-oracles --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio-oracle-native"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05792
      }
    },
    {
      "name": "🧪️test🧬️schema📇️registry🦀️test-neutrality",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-registry-rs:test-neutrality --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05793
      }
    },
    {
      "name": "🧪️test🧬️schema📶️state🦀️test",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-state-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05794
      }
    },
    {
      "name": "🧪️test🧬️schema📶️state🦀️test-quick",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-state-rs:test-quick --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05795
      }
    },
    {
      "name": "🧪️test🧬️schema📶️state🦀️test-long",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-state-rs:test-long --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05796
      }
    },
    {
      "name": "🧪️test🧬️schema📶️state🦀️test-exhaustive",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-state-rs:test-exhaustive --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05797
      }
    },
    {
      "name": "🧪️test🧬️schema🧩️composition🦀️test",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-composition-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05798
      }
    },
    {
      "name": "🧪️test🧬️schema🧩️composition🦀️test-quick",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-composition-rs:test-quick --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05799
      }
    },
    {
      "name": "🧪️test🧬️schema🧩️composition🦀️test-long",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-composition-rs:test-long --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.058
      }
    },
    {
      "name": "🧪️test🧬️schema🧩️composition🦀️test-exhaustive",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-composition-rs:test-exhaustive --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05801
      }
    },
    {
      "name": "🧪️test📕️norm🧾️definition🧱️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-artifact-contract-rs:test-definition-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/norm-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05802
      }
    },
    {
      "name": "🧪️test🧩️puzzle🧵️retained📍️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-puzzle-retained-test:test-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-puzzle-retained"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05803
      }
    },
    {
      "name": "🧪️test🧩️puzzle🧵️retained🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-puzzle-retained-test:test-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-puzzle-retained"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05804
      }
    },
    {
      "name": "🧪️test🪐️space🧫️fixture📍️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-space-fixture-sources-test:test-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-space-fixture-sources"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05805
      }
    },
    {
      "name": "🧪️test🪐️space🧫️fixture🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-space-fixture-sources-test:test-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-space-fixture-sources"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05806
      }
    },
    {
      "name": "🧪️test🔌️plugin🏗️fixture🧬️interfaces",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:test-fixture-channel-interfaces --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/fixture-channel-interfaces"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05807
      }
    },
    {
      "name": "🧪️test📽️pptx📐️transform🔣️wire",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-pptx-rs:test-transform-wire --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/pptx-transform-wire"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05809
      }
    },
    {
      "name": "🧪️test🛠️tool🕸️rows🧬️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-tool-machine-rs:test-node-graph-row-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/node-graph-row-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05810
      }
    },
    {
      "name": "🧪️test🗄️stdio🔮️oracle🖊️drawing-reader",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-drawing-reader --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio-drawing"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05811
      }
    },
    {
      "name": "🧪️test🌱️value🏷️type🧬️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/value-rs:test-type-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/value-type-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05813
      }
    },
    {
      "name": "🧪️test🖱️ui🎚️ring📬️press",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:test-control-commit --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-ui-neutrality/ring-press"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05815
      }
    },
    {
      "name": "🧪️test🗣️dsl🧱️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-dsl-rs:test-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-lexical/ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05814
      }
    },
    {
      "name": "🧪️test📚️compiler🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-compiler-rs:test-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-lexical/compiler-native"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0582
      }
    },
    {
      "name": "🧪️test🗣️dsl🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-dsl-rs:test-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-lexical/dsl-native"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05819
      }
    },
    {
      "name": "🧪️test🧵️worker-cell🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:test-worker-cell --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-worker/portable-launch-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05816
      }
    },
    {
      "name": "🧪️test🌎️hub🧫️private-reader",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-private-reader --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/private-reader-preservation"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05817
      }
    },
    {
      "name": "🧪️test🌎️hub🖊️drawing🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-native-drawing-reader --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-worker/lower-drawing-native-artifacts",
        "CARGO_TARGET_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-worker/lower-drawing-target",
        "CARGO_BUILD_BUILD_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-worker/lower-drawing-target/intermediate"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05818
      }
    },
    {
      "name": "🧪️test🌱️value🛬️decode🧩️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/value-rs:test-decode-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/decode-test-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05821
      }
    },
    {
      "name": "🧪️test⚠️diagnostic🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-diagnostic-rs:test-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-lexical/diagnostic-native"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05822
      }
    },
    {
      "name": "🧪️test🗣️dsl💻️composition",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:test-dsl-architecture --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-lexical/higher-composition"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05824
      }
    },
    {
      "name": "🧪️test🗣️dsl🧫️reference",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-dsl-rs:test-reference --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-lexical/lower-reference"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05825
      }
    },
    {
      "name": "🧪️test🧠️neural🏷️type🧩️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/neural-engine-rs:test-type-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/value-type-proof"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05826
      }
    },
    {
      "name": "🧪️test🌎️hub🏷️type🧩️preservation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-type-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/value-type-proof"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05827
      }
    },
    {
      "name": "🧪️test🎒️pack🔤️json🧩️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-pack-json-rs:test-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05828
      }
    },
    {
      "name": "🧪️test🎒️pack🔤️json🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-pack-json-rs:test-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05829
      }
    },
    {
      "name": "🧪️test🧬️schema📄️source🔗️closure",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-source-closure-ts:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0584
      }
    },
    {
      "name": "🧪️test🎒️pack🔤️json📥️decode",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/pack-json-decode-ts:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05841
      }
    },
    {
      "name": "🧪️test🧬️schema📄️source🔗️closure🚮️absence",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-source-closure-ts:test-absence --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05842
      }
    },
    {
      "name": "🧪️test📚️compiler📖️syntax🦀️rust",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/compiler-syntax-rust-ts:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05843
      }
    },
    {
      "name": "🧪️test📚️compiler📖️syntax🦀️rust🚮️absence",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/compiler-syntax-rust-ts:test-absence --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05844
      }
    },
    {
      "name": "🧪️test🗣️dsl🧬️record🧩️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-dsl-record-rs:test-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.0583
      }
    },
    {
      "name": "🧪️test🗣️dsl🧬️record🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-dsl-record-rs:test-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05831
      }
    }
  ],
  "compounds": [
    {
      "name": "🧭️compound🖥️s⚛️react🌉️os-mcp",
      "configurations": [
        "🛠️dev🌉️os-mcp🌐️http",
        "🛠️dev🪐️space⚛️react"
      ],
      "stopAll": true,
      "presentation": {
        "group": "3_dev",
        "order": 386.16
      }
    },
    {
      "name": "🧭️compound🖥️s⚛️react🗄️os-hub",
      "configurations": [
        "🛠️dev🗄️os-hub",
        "🛠️dev🪐️space⚛️react"
      ],
      "stopAll": true,
      "presentation": {
        "group": "3_dev",
        "order": 386.15
      }
    },
    {
      "name": "🧭️compound🖥️s👥️users🗄️os-hub",
      "configurations": [
        "🛠️dev🗄️os-hub",
        "🛠️dev🪐️space👤️1⚛️react",
        "🛠️dev🪐️space👤️2⚛️react"
      ],
      "stopAll": true,
      "presentation": {
        "group": "3_dev",
        "order": 386.16
      }
    },
    {
      "name": "🧭️compound🎓️teaching🏛️architecture❓️quiz🛂️proctor",
      "configurations": [
        "🛠️dev🎓️teaching🛂️proctor",
        "🛠️dev🎓️teaching🏛️architecture❓️quiz"
      ],
      "stopAll": true,
      "presentation": {
        "group": "3_dev",
        "order": 213.62
      }
    }
  ],
  "inputs": [
    {
      "id": "wgpuEmbeddedConfiguration",
      "type": "promptString",
      "description": "Ticket input configuration for actual embedded renderer acceptance",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️embedded-browser/🧊️wgpu.json"
    },
    {
      "id": "wgpuEmbeddedArtifacts",
      "type": "promptString",
      "description": "Ticket directory for embedded browser receipts",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/embedded-browser"
    },
    {
      "id": "wgpuDockReactServe",
      "type": "promptString",
      "description": "React renderer host URL",
      "default": "http://127.0.0.1:7300/"
    },
    {
      "id": "wgpuDockServe",
      "type": "promptString",
      "description": "WGPU renderer host URL",
      "default": "http://127.0.0.1:7301/?plugin=puzzle3d"
    },
    {
      "id": "wgpuDockArtifacts",
      "type": "promptString",
      "description": "Ticket directory for Dock acceptance receipts",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/dock-browser"
    },
    {
      "id": "wgpuMediaServe",
      "type": "promptString",
      "description": "WGPU stdio media viewer host URL",
      "default": "http://127.0.0.1:7303/?plugin=stdio-wav"
    },
    {
      "id": "wgpuMediaLocale",
      "type": "pickString",
      "description": "Explicit acceptance language",
      "options": [
        "en",
        "de"
      ],
      "default": "en"
    },
    {
      "id": "wgpuMediaArtifacts",
      "type": "promptString",
      "description": "Ticket directory for media acceptance receipts",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/media-browser"
    },
    {
      "id": "rendererParityRenderer",
      "type": "pickString",
      "description": "Renderer comparison",
      "options": [
        "paired",
        "react",
        "wgpu"
      ],
      "default": "paired"
    },
    {
      "id": "rendererParityJourneyArtifacts",
      "type": "promptString",
      "description": "Ticket directory for shell interaction receipts",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/shell-interaction"
    },
    {
      "id": "nativeScaleRegistry",
      "type": "promptString",
      "description": "Native scale registry JSON path",
      "default": "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🤖️generated/📇️registry/🔣️.json"
    },
    {
      "id": "gisVerification",
      "type": "pickString",
      "description": "GIS verification",
      "options": [
        "imports",
        "watcher",
        "map",
        "graph"
      ],
      "default": "map"
    },
    {
      "id": "nxCacheTicket",
      "type": "promptString",
      "description": "Active ticket directory for Nx diagnostics"
    },
    {
      "id": "artifactPackageProject",
      "type": "promptString",
      "description": "Artifact package Nx project, for example @semio-tech/stdio-pdf-rs"
    },
    {
      "id": "artifactPackageTarget",
      "type": "pickString",
      "description": "Artifact package target",
      "options": [
        "build",
        "check",
        "test"
      ]
    },
    {
      "id": "catalogFreshBuildRoot",
      "type": "promptString",
      "description": "Absolute fresh plugin catalog build root"
    },
    {
      "id": "acceptanceHubUrl",
      "type": "promptString",
      "description": "Hub under acceptance (origin)",
      "default": "http://127.0.0.1:8787"
    },
    {
      "id": "acceptanceServeUrl",
      "type": "promptString",
      "description": "s React serve joined to that hub",
      "default": "http://127.0.0.1:6070/"
    },
    {
      "id": "acceptanceLocalServeUrl",
      "type": "promptString",
      "description": "Local-only s React serve (every plugin loaded; launch row 🛠️dev🪐️space⚛️react🔒local-only)",
      "default": "http://127.0.0.1:6070/"
    },
    {
      "id": "acceptanceHubAdminCapability",
      "type": "promptString",
      "description": "The hub launcher's admin-capability.json (0600) for gates that read the hub's connection census",
      "default": ""
    },
    {
      "id": "acceptanceUsers",
      "type": "promptString",
      "description": "Optional JSON file {\"users\":[{\"email\",\"password\"}…]} with the hub's test users (empty: each gate's development users)",
      "default": ""
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️ownership🌎️hub🧩️compositions🟦️",
      "command": "bun nx run @semio-tech/hub-compositions:ownership",
      "cwd": "${workspaceFolder}",
      "presentation": { "group": "4_gate", "order": 900.06 }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "🧪️native🌎️hub🧩️compositions🦀️",
      "command": "bun nx run @semio-tech/hub-compositions:native",
      "cwd": "${workspaceFolder}",
      "presentation": { "group": "4_gate", "order": 900.07 }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️check🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.08
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.09
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test-quick🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:test-quick",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test-long🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:test-long",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.11
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test-exhaustive🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:test-exhaustive",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.12
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️mutation-verb-vocabulary-check🧬️mutation-verbs🟦️",
      "command": "bun nx run @semio-tech/framework-os:mutation-verb-vocabulary-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.13
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️verify-inference-client🧬️gis-gismap🦀️",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-inference-client",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.14
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️verify-inference-client-native🧬️gis-gismap🦀️",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-inference-client-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.15
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️contract-check📦️component-deployment🟦️",
      "command": "bun nx run @semio-tech/component-deployment-contract:contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.16
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️generation3d-semantic-wire-source🧊️procedural🦀️",
      "command": "bun nx run @semio-tech/procedural-generation3d-rs:semantic-wire-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.17
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️generation3d-semantic-wire-native🧊️procedural🦀️",
      "command": "bun nx run @semio-tech/procedural-generation3d-rs:semantic-wire-check -- native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1701
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️services-mcp-composition-source🧩️s🟦️",
      "command": "bun nx run @semio-tech/s-services-native:mcp-composition-source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.18
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️services-mcp-untrusted-content🧩️s🟦️",
      "command": "bun nx run @semio-tech/s-services-native:untrusted-content-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1801
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️services-fixture-ownership🧩️s🦀️",
      "command": "bun nx run @semio-tech/s-services-native:fixture-ownership-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1802
      }
    }
,
    {
      "name": "⚖️check-wgpu-boot-cache-inputs🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:check-boot-cache-inputs --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1803
      }
    },
    {
      "id": "stdioRemovalArtifacts",
      "type": "promptString",
      "description": "Ticket output directory containing the retained copied workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio"
    },
    {
      "id": "stdioRemovalSnapshot",
      "type": "promptString",
      "description": "Retained workspace copy with only AVI absent, relative to the workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/parent-avi-removal"
    },
    {
      "id": "graphContractArtifacts",
      "type": "promptString",
      "description": "Ticket-owned graph contract artifacts directory relative to workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/graph-contract"
    },
    {
      "id": "svgVideoArtifacts",
      "type": "promptString",
      "description": "Ticket-owned SVG video artifacts directory relative to workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/svg-video"
    },
    {
      "id": "processContractArtifacts",
      "type": "promptString",
      "description": "Ticket-owned framework process contract output directory relative to workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/process-contract"
    },
    {
      "id": "identityContractArtifacts",
      "type": "promptString",
      "description": "Ticket-owned identity contract output directory relative to workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/grapheme-removal"
    }
  ],

  // 🎮️devLaunchers — per-playground-variant dev-launcher metadata (not part of the generated
  // output); see 🚀️launch/🟦️.ts readSeed() for the exact split contract this marker line supports.
  "devLaunchers": {
    "aggregator": {
      "namePrefix": "♻️mit-bestand🧩️puzzle🧊️3d",
      "order": 212
    },
    "animate": {
      "namePrefix": "🎞️animate🎬️presentation",
      "order": 170,
      "wgpuOrder": 170.1
    },
    "architect": {
      "namePrefix": "🏛️architect🏛️program",
      "order": 142,
      "wgpuOrder": 142.1
    },
    "aussuchen": {
      "namePrefix": "♻️mit-bestand🪵️sourcing🗂️curation",
      "order": 216
    },
    "bearbeiten": {
      "namePrefix": "♻️mit-bestand🏭️process🧊️process3d",
      "order": 217
    },
    "block2d": {
      "namePrefix": "🧱️block◻️2d",
      "order": 261,
      "wgpuOrder": 261.1
    },
    "block3d": {
      "namePrefix": "🧱️block🧊️3d",
      "order": 262,
      "wgpuOrder": 262.1
    },
    "block5d": {
      "namePrefix": "🧱️block🖐️5d",
      "order": 263,
      "wgpuOrder": 263.1
    },
    "cad": {
      "namePrefix": "📐️cad",
      "order": 10,
      "wgpuOrder": 10.1
    },
    "dag": {
      "namePrefix": "🕸️dag",
      "order": 140,
      "wgpuOrder": 140.1
    },
    "draw": {
      "namePrefix": "🖍️draw🖍️drawing",
      "order": 387,
      "wgpuOrder": 387.1
    },
    "fem2d": {
      "namePrefix": "🏗️fem◻️2d",
      "order": 392,
      "wgpuOrder": 392.1
    },
    "fem3d": {
      "namePrefix": "🏗️fem🧊️3d",
      "order": 392.3,
      "wgpuOrder": 392.4
    },
    "flow": {
      "namePrefix": "🌊️flow",
      "order": 150,
      "wgpuOrder": 150.1
    },
    "forms": {
      "namePrefix": "📋️forms",
      "order": 385,
      "wgpuOrder": 385.1
    },
    "generator": {
      "namePrefix": "♻️mit-bestand🌀️procedural🧊️generation3d",
      "order": 214
    },
    "gis2d": {
      "namePrefix": "🌍️gis🗺️gismap",
      "order": 160,
      "wgpuOrder": 160.1
    },
    "gis3d": {
      "namePrefix": "🌍️gis🏔️gisterrain",
      "order": 160.3,
      "wgpuOrder": 160.4
    },
    "imperative": {
      "namePrefix": "📜️imperative📜️procedure",
      "order": 155,
      "wgpuOrder": 155.1
    },
    "koordinator": {
      "namePrefix": "♻️mit-bestand📐️cad",
      "order": 215
    },
    "layout": {
      "namePrefix": "📏️layout",
      "order": 158,
      "wgpuOrder": 158.1
    },
    "lowpoly": {
      "namePrefix": "💠️lowpoly",
      "order": 157,
      "wgpuOrder": 157.1
    },
    "mathematical": {
      "namePrefix": "➗️mathematical➗️equation",
      "order": 141,
      "wgpuOrder": 141.1
    },
    "note": {
      "namePrefix": "🗒️note",
      "order": 388,
      "wgpuOrder": 388.1
    },
    "bitmap": {
      "namePrefix": "🀄️wfc🖼️bitmap",
      "order": 201,
      "wgpuOrder": 201.1
    },
    "grid2d": {
      "namePrefix": "🀄️wfc🔲️grid2d",
      "order": 202,
      "wgpuOrder": 202.1
    },
    "wfc2d": {
      "namePrefix": "🀄️wfc◻️2d",
      "order": 203,
      "wgpuOrder": 203.1
    },
    "grid3d": {
      "namePrefix": "🀄️wfc🧱️grid3d",
      "order": 204,
      "wgpuOrder": 204.1
    },
    "wfc3d": {
      "namePrefix": "🀄️wfc🧊️3d",
      "order": 205,
      "wgpuOrder": 205.1
    },
    "generation2d": {
      "namePrefix": "🌀️procedural🌀️generation2d",
      "order": 180,
      "wgpuOrder": 180.1
    },
    "generation3d": {
      "namePrefix": "🌀️procedural🧊️generation3d",
      "order": 190,
      "wgpuOrder": 190.1
    },
    "process3d": {
      "namePrefix": "🏭️process🧊️process3d",
      "order": 195,
      "wgpuOrder": 195.1
    },
    "puzzle2d": {
      "namePrefix": "🧩️puzzle◻️2d",
      "order": 220,
      "wgpuOrder": 220.1
    },
    "puzzle3d": {
      "namePrefix": "🧩️puzzle🧊️3d",
      "order": 230,
      "wgpuOrder": 230.1
    },
    "puzzle5d": {
      "namePrefix": "🧩️puzzle🖐️5d",
      "order": 250,
      "wgpuOrder": 250.1
    },
    "raster": {
      "namePrefix": "🖨️raster",
      "order": 386,
      "wgpuOrder": 386.1
    },
    "reasoning-wires": {
      "namePrefix": "💡️reasoning🔌️wires",
      "order": 270,
      "wgpuOrder": 270.1
    },
    "remodel": {
      "namePrefix": "📸️remodel📸️remodeling",
      "order": 389,
      "wgpuOrder": 389.1
    },
    "s": {
      "namePrefix": "🪐️space",
      "order": 386.2,
      "wgpuOrder": 386,
      "env": {
        "S_HUB_URL": "http://127.0.0.1:8787",
        "S_DATA_DIR": "${workspaceFolder}/.🧬semio/🔗space/s-dev"
      },
      "users": {
        "namePrefixPattern": "🖥️s👤️{N}",
        "emailPattern": "user{N}@semio.dev",
        "env": {
          "S_HUB_URL": "http://127.0.0.1:8787",
          "S_DATA_DIR": "${workspaceFolder}/.🧬semio/🔗space/s-user{N}"
        }
      }
    },
    "sequence": {
      "namePrefix": "🎬️sequence",
      "order": 156,
      "wgpuOrder": 156.1
    },
    "shooting": {
      "namePrefix": "🎥️shooting",
      "order": 290,
      "wgpuOrder": 290.1
    },
    "sourcing": {
      "namePrefix": "🪵️sourcing🗂️curation",
      "order": 196,
      "wgpuOrder": 196.1
    },
    "trinity-jack": {
      "namePrefix": "🔱️trinity🔌️jack",
      "order": 360,
      "wgpuOrder": 360.1
    },
    "trinity-rewriting": {
      "namePrefix": "🔱️trinity♻️rewriting",
      "order": 380,
      "wgpuOrder": 380.1
    },
    "vcs": {
      "namePrefix": "🌿️vcs",
      "order": 385,
      "wgpuOrder": 385.1
    },
    "verfolgen": {
      "namePrefix": "♻️mit-bestand🌍️gis🗺️gismap",
      "order": 218
    },
    "writer": {
      "namePrefix": "✒️writer",
      "order": 387,
      "wgpuOrder": 387.1
    }
  },
  // 📋️projectLaunchers — how every declared `📋️project.json` target becomes a launch row (not part of the generated
  // output); see 🚀️launch/🟦️.ts `renderProjectTargetLaunchers` for the contract. A curated row above that runs a target wins.
  "projectLaunchers": {
    "familyMinimumProjects": 3,
    "familyEmoji": "📋️",
    "fallbackClass": "run",
    "classes": [
      {
        "id": "dev",
        "emoji": "🛠️",
        "group": "3_dev",
        "orderBase": 900,
        "tokens": [
          "dev",
          "serve",
          "start",
          "watch",
          "activate",
          "open",
          "launch",
          "inspect",
          "demo",
          "playground",
          "attach"
        ]
      },
      {
        "id": "build",
        "emoji": "📦️",
        "group": "4_build",
        "orderBase": 900,
        "tokens": [
          "build",
          "package",
          "wasm",
          "publish",
          "release",
          "bundle",
          "generate",
          "generator",
          "typegen",
          "codegen",
          "preview",
          "deps",
          "fonts",
          "prepare",
          "materialize",
          "install",
          "compile",
          "sign",
          "deploy",
          "bootstrap",
          "clean",
          "prune",
          "restage",
          "rebuild",
          "setup",
          "format",
          "fix",
          "regenerate"
        ]
      },
      {
        "id": "gate",
        "emoji": "⚖️",
        "group": "4_gate",
        "orderBase": 900,
        "tokens": [
          "test",
          "check",
          "verify",
          "lint",
          "typecheck",
          "oracle",
          "native",
          "e2e",
          "probe",
          "audit",
          "census",
          "bench",
          "parity",
          "law",
          "laws",
          "drill",
          "smoke",
          "conformance",
          "contract",
          "validate",
          "scan",
          "report",
          "doctor",
          "discover",
          "inventory",
          "metrics"
        ]
      },
      {
        "id": "run",
        "emoji": "▶️",
        "group": "3_dev",
        "orderBase": 950,
        "tokens": [
          "run"
        ]
      }
    ],
    "languageSegments": {
      "🦀️rust": "🦀️",
      "🟦️typescript": "🟦️",
      "🐍️python": "🐍️"
    },
    "transparentSegments": [
      "📦️packages"
    ],
    "skipDirectories": [
      "node_modules",
      "dist",
      "target",
      "temp",
      "pkg",
      "storybook-static",
      "🤖️generated",
      "🗑️generated",
      "🎫️tickets"
    ]
  }
}

````