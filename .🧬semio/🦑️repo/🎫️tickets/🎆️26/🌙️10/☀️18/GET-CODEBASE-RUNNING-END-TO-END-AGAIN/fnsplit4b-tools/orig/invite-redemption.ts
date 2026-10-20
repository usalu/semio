/** 🧭️ Invite redemption transaction and ordered publication gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/process/cargo-laws";
import { type ExactCargoLawGroup } from "@semio-tech/process/cargo-laws";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { BundleScript } from "@semio-tech/process/routing";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { orderedDirectoryPublicationOracle, readHubDirectorySource } from "../../📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts";
import { readDirectoryBackendSource, readHubBinSource, readNeo4jDirectorySource } from "../../🧪️testing/📖️source-readers/🟦️.ts";

type InviteRedemptionFixture = {
  readonly schema: "semio.hub.invite-redemption-transaction/v1";
  readonly limits: { readonly credentialBytes: 4096; readonly identifierBytes: 4096; readonly maximumContenders: 3 };
  readonly receiptRelease: readonly { readonly name: string; readonly outcome: "rejected-before-commit" | "appended" | "uncertain"; readonly disposition: "pending" | "completed"; readonly digest: "exact" | "different"; readonly release: boolean }[];
  readonly backendOrders: readonly { readonly name: string; readonly first: "archive" | "member"; readonly between: "invite" | "archive"; readonly accepted: boolean; readonly appended: number }[];
  readonly authorityRaces: readonly { readonly name: string; readonly revocation: "session" | "archive" | "delete"; readonly order: readonly ("hint" | "revoke" | "fence")[]; readonly status: number; readonly appended: number }[];
  readonly spaceStates: readonly {
    readonly name: string;
    readonly spaceState: "writable" | "archived" | "deleted";
    readonly inviteRole: "author" | "spectator";
    readonly accepted: "none" | "same" | "other";
    readonly status: number;
    readonly appended: number;
    readonly membershipRole: "none" | "author" | "spectator";
  }[];
  readonly vectors: readonly {
    readonly name: string;
    readonly initial: "fresh" | "accepted-same" | "accepted-other" | "revoked" | "expired" | "missing" | "missing-user" | "missing-space" | "corrupt-marker" | "corrupt-event";
    readonly concurrent: boolean;
    readonly calls: readonly {
      readonly kind: "redeem" | "revoke" | "append-forged" | "restart" | "rebuild";
      readonly user: "same" | "other" | "missing";
      readonly actor: "exact" | "different";
      readonly credential: "exact" | "wrong-selector" | "wrong-secret";
      readonly failure: "none" | "after-marker" | "after-event" | "after-projection";
    }[];
    readonly expected: {
      readonly outcomes: readonly string[];
      readonly returnedEventIds: readonly (string | null)[];
      readonly marker: { readonly acceptedAt: number | null; readonly acceptedEventId: string | null };
      readonly events: number;
      readonly memberships: number;
      readonly publications: number;
      readonly replayEvents: number;
      readonly revoked: boolean;
    };
  }[];
  readonly hostiles: readonly { readonly name: string; readonly mutation: "raw-capability" | "client-space" | "client-space-state" | "client-role" | "client-event-id" | "unknown-field" | "oversized-identifier" }[];
};

/** 🎟️ Evaluates the neutral one-transaction invite state machine independently of every backend. */
export async function proveInviteRedemptionTransaction(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/📇️directory/🧪️tests/🧫️fixtures/🎟️invite-redemption-transaction-v1");
  const client = await import(join(repoRoot, "🧰️framework/🔨️modules/📇️directory/🟦️.ts"));
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as InviteRedemptionFixture;
  const acceptanceMarker = hubSchemaExport(repoRoot, "schema://hub.directory/InviteAcceptanceMarkerV1");
  if (fixture.schema !== "semio.hub.invite-redemption-transaction/v1") throw new Error("invite redemption transaction fixture schema drift");
  if (Object.keys(fixture).sort().join(",") !== "authorityRaces,backendOrders,hostiles,limits,receiptRelease,schema,spaceStates,vectors") throw new Error("invite redemption transaction fixture envelope drift");
  if (fixture.limits.credentialBytes !== 4096 || fixture.limits.identifierBytes !== 4096 || fixture.limits.maximumContenders !== 3) throw new Error("invite redemption transaction bound drift");
  if (fixture.vectors.length < 16 || fixture.vectors.length > 32 || fixture.spaceStates.length !== 7 || fixture.authorityRaces.length !== 4 || fixture.backendOrders.length !== 2 || fixture.receiptRelease.length !== 4) throw new Error("invite redemption transaction table sizes drifted");
  const inviteHostiles = ["raw-capability", "client-space", "client-role", "client-event-id", "unknown-field", "oversized-identifier", "client-space-state"];
  if (fixture.hostiles.length < 4 || fixture.hostiles.length > 12 || new Set(fixture.hostiles.map(({ mutation }) => mutation)).size !== inviteHostiles.length || fixture.hostiles.some(({ mutation }) => !inviteHostiles.includes(mutation)))
    throw new Error("invite redemption transaction hostile inventory drift");
  if (new Set(fixture.vectors.map(({ name }) => name)).size !== fixture.vectors.length) throw new Error("invite redemption fixture names are not unique");
  for (const vector of fixture.vectors) {
    if (!acceptanceMarker(vector.expected.marker)) throw new Error(`invite redemption acceptance marker is not the owned marker contract: ${vector.name}`);
    if (vector.calls.length < 1 || vector.calls.length > fixture.limits.maximumContenders || vector.expected.outcomes.length !== vector.calls.length || vector.expected.returnedEventIds.length !== vector.calls.length)
      throw new Error(`invite redemption call/outcome arity differs: ${vector.name}`);
  }
  if (acceptanceMarker({ ...fixture.vectors[0]!.expected.marker, acceptedBy: "forged" }) || acceptanceMarker({ acceptedAt: -1, acceptedEventId: null })) throw new Error("invite redemption marker contract admitted a forged marker");

  const { Database } = await import("bun:sqlite");
  const oracle = new Database(":memory:");
  try {
    for (const row of fixture.spaceStates) {
      const expectedStatus = row.spaceState === "deleted" ? 401 : row.accepted === "same" ? 200 : row.accepted === "other" ? 409 : row.spaceState === "archived" && row.inviteRole === "author" ? 401 : 200;
      const appended = expectedStatus === 200 && row.accepted === "none" ? 1 : 0;
      const membershipRole = row.spaceState === "deleted" || (row.accepted !== "same" && appended === 0) ? "none" : row.spaceState === "archived" ? "spectator" : row.inviteRole;
      const sql = oracle
        .query("SELECT CASE WHEN ?1 = 'deleted' THEN 401 WHEN ?2 = 'same' THEN 200 WHEN ?2 = 'other' THEN 409 WHEN ?1 = 'archived' AND ?3 = 'author' THEN 401 ELSE 200 END AS status")
        .get(row.spaceState, row.accepted, row.inviteRole) as { status: number };
      if (sql.status !== expectedStatus || row.status !== expectedStatus || row.appended !== appended || row.membershipRole !== membershipRole) throw new Error(`invite state oracle differs for ${row.name}`);
    }

    for (const row of fixture.receiptRelease) {
      const release = row.outcome === "rejected-before-commit" && row.disposition === "pending" && row.digest === "exact";
      const sql = oracle.query("SELECT CASE WHEN ?1='rejected-before-commit' AND ?2='pending' AND ?3='exact' THEN 1 ELSE 0 END AS release").get(row.outcome, row.disposition, row.digest) as { release: number };
      if (release !== row.release || Boolean(sql.release) !== release) throw new Error(`receipt release oracle differs for ${row.name}`);
    }
    for (const row of fixture.backendOrders) {
      const accepted = row.first === "archive";
      const sql = oracle.query("SELECT CASE WHEN ?1 = 'archive' THEN 1 ELSE 0 END AS accepted").get(row.first) as { accepted: number };
      if (row.accepted !== accepted || sql.accepted !== Number(accepted) || row.appended !== Number(accepted) || (row.between === "invite") !== accepted) throw new Error(`invite backend order oracle differs for ${row.name}`);
      const event = (seq: number, body: unknown) => ({ seq, id: `archive-event-${seq}`, hlc: { physicalMs: seq, logical: 0 }, actor: { kind: "system", id: "archive-oracle" }, recordedAtMs: seq, body });
      const before = client.foldAll(client.emptyDirectoryReadModel(), [
        event(1, { kind: "space.created", spaceId: "archive-space", name: "Archive", spaceKind: "studio", visibility: "private", ownerUserId: "owner" }),
        event(2, { kind: "member.upserted", spaceId: "archive-space", userId: "owner", role: "author" }),
        event(3, { kind: row.between === "invite" ? "invite.redeemed" : "member.upserted", spaceId: "archive-space", userId: "member", inviteId: "archive-invite", role: row.between === "invite" ? "author" : "spectator" }),
      ]);
      before.spaces.get("archive-space").view.role = "author";
      const archived = client.fold(before, event(4, { kind: "space.archived", spaceId: "archive-space" })).spaces.get("archive-space");
      oracle.exec(
        "CREATE TABLE IF NOT EXISTS archive_members(user_id TEXT PRIMARY KEY, role TEXT); DELETE FROM archive_members; INSERT INTO archive_members VALUES ('owner','author'),('member','author'); UPDATE archive_members SET role='spectator' WHERE role='author'",
      );
      const expected = oracle.query("SELECT role FROM archive_members ORDER BY user_id").all() as { role: string }[];
      if (
        archived.view.kind !== "archive" ||
        archived.view.role !== "spectator" ||
        JSON.stringify(archived.members.map((member: { role: string }) => member.role)) !== JSON.stringify(expected.map(({ role }) => role)) ||
        before.spaces.get("archive-space").view.role !== "author"
      )
        throw new Error(`archive client fold differs from SQLite oracle for ${row.name}`);
      console.log(`directory-archive-fold case=${row.name} members=2 role=spectator immutable=1`);
    }

    for (const row of fixture.authorityRaces) {
      const revoke = row.order.indexOf("revoke");
      const fence = row.order.indexOf("fence");
      const status = revoke < fence ? 401 : 200;
      const sql = oracle.query("SELECT CASE WHEN ?1 < ?2 THEN 401 ELSE 200 END AS status").get(revoke, fence) as { status: number };
      if (row.order[0] !== "hint" || row.status !== status || sql.status !== status || row.appended !== Number(status === 200)) throw new Error(`invite authority race oracle differs for ${row.name}`);
    }
  } finally {
    oracle.close();
  }
  for (const vector of fixture.vectors) {
    const accepted = vector.initial === "accepted-same" || vector.initial === "accepted-other" || vector.initial === "corrupt-marker" || vector.initial === "corrupt-event";
    let acceptedAt: number | null = accepted ? 100 : null;
    let acceptedEventId: string | null = vector.initial === "corrupt-marker" ? null : vector.initial === "corrupt-event" ? "event-missing" : accepted ? "event-0" : null;
    let event = vector.initial === "accepted-same" ? { id: "event-0", user: "same", recordedAt: 100 } : vector.initial === "accepted-other" ? { id: "event-0", user: "other", recordedAt: 100 } : null;
    let membership = event ? 1 : 0;
    let publications = 0;
    let revoked = vector.initial === "revoked";
    const recordExists = vector.initial !== "missing";
    const userExists = vector.initial !== "missing-user";
    const spaceExists = vector.initial !== "missing-space";
    const outcomes: string[] = [];
    const returnedEventIds: (string | null)[] = [];
    for (const call of vector.calls) {
      if (call.kind === "restart") {
        outcomes.push("restarted");
        returnedEventIds.push(null);
        continue;
      }
      if (call.kind === "rebuild") {
        membership = event ? 1 : 0;
        outcomes.push("rebuilt");
        returnedEventIds.push(null);
        continue;
      }
      if (call.kind === "append-forged") {
        outcomes.push("append-denied");
        returnedEventIds.push(null);
        continue;
      }
      if (call.kind === "revoke") {
        if (acceptedAt !== null || revoked) outcomes.push("conflict");
        else {
          revoked = true;
          outcomes.push("revoked");
        }
        returnedEventIds.push(null);
        continue;
      }
      if (!recordExists || call.actor !== "exact" || call.credential !== "exact") {
        outcomes.push("unauthorized");
        returnedEventIds.push(null);
        continue;
      }
      if ((acceptedAt === null) !== (acceptedEventId === null)) {
        outcomes.push("backend-error");
        returnedEventIds.push(null);
        continue;
      }
      if (acceptedAt !== null) {
        if (!event || event.id !== acceptedEventId || event.recordedAt !== acceptedAt) {
          outcomes.push("backend-error");
          returnedEventIds.push(null);
        } else if (event.user !== call.user) {
          outcomes.push("conflict");
          returnedEventIds.push(null);
        } else {
          outcomes.push("already-committed");
          returnedEventIds.push(event.id);
        }
        continue;
      }
      if (!userExists || !spaceExists || call.user === "missing") {
        outcomes.push("unauthorized");
        returnedEventIds.push(null);
        continue;
      }
      if (revoked || vector.initial === "expired") {
        outcomes.push("conflict");
        returnedEventIds.push(null);
        continue;
      }
      if (call.failure !== "none") {
        outcomes.push("backend-error");
        returnedEventIds.push(null);
        continue;
      }
      acceptedAt = 101;
      acceptedEventId = "event-1";
      event = { id: "event-1", user: call.user, recordedAt: 101 };
      membership = 1;
      publications += 1;
      outcomes.push("newly-committed");
      returnedEventIds.push(event.id);
    }
    const actual = { outcomes, returnedEventIds, marker: { acceptedAt, acceptedEventId }, events: event ? 1 : 0, memberships: membership, publications, replayEvents: event ? 1 : 0, revoked };
    if (JSON.stringify(actual) !== JSON.stringify(vector.expected)) throw new Error(`invite redemption independent model differs for ${vector.name}: ${JSON.stringify(actual)}`);
  }
  const redemptionCall = hubSchemaExport(repoRoot, "schema://hub.directory/InviteRedemptionCallV1");
  for (const vector of fixture.vectors) for (const call of vector.calls) if (!redemptionCall(call)) throw new Error(`invite redemption call is not the owned call contract: ${vector.name}`);
  for (const hostile of fixture.hostiles) {
    const call = structuredClone(fixture.vectors[0]!.calls[0]!) as unknown as Record<string, unknown>;
    if (hostile.mutation === "raw-capability") call.rawCapability = "forbidden";
    else if (hostile.mutation === "client-space") call.spaceId = "client-space";
    else if (hostile.mutation === "client-space-state") call.spaceState = "writable";
    else if (hostile.mutation === "client-role") call.role = "author";
    else if (hostile.mutation === "client-event-id") call.eventId = "client-event";
    else if (hostile.mutation === "unknown-field") {
      const candidate = { ...(fixture as unknown as Record<string, unknown>), unknown: true };
      if (Object.keys(candidate).sort().join(",") === "authorityRaces,backendOrders,hostiles,limits,receiptRelease,schema,spaceStates,vectors") throw new Error(`invite redemption envelope admitted hostile ${hostile.name}`);
      continue;
    } else {
      if (acceptanceMarker({ acceptedAt: 1, acceptedEventId: "x".repeat(4097) })) throw new Error(`invite redemption marker admitted hostile ${hostile.name}`);
      continue;
    }
    if (redemptionCall(call)) throw new Error(`invite redemption call contract admitted hostile ${hostile.name}`);
  }
  const directory = readHubDirectorySource(repoRoot);
  const sqlite = readDirectoryBackendSource(repoRoot, "🪶️sqlite");
  const postgres = readDirectoryBackendSource(repoRoot, "🐘️postgres");
  const neo4j = readNeo4jDirectorySource(repoRoot);
  const body = (source: string, signature: string): string => {
    const offset = source.indexOf(signature);
    const start = source.indexOf("{", offset);
    let depth = 0;
    for (let index = start; offset >= 0 && index < source.length; index += 1) {
      if (source[index] === "{") depth += 1;
      else if (source[index] === "}" && --depth === 0) return source.slice(start + 1, index);
    }
    return "";
  };
  const ordered = (source: string, values: readonly string[]): boolean => values.every((value, index) => source.indexOf(value) >= 0 && (index === 0 || source.indexOf(values[index - 1]!) < source.indexOf(value)));
  const sourceClosed = (shared: string, sq: string, pg: string, neo: string): boolean => {
    const service = body(shared, "pub async fn redeem_invite(");
    const sqliteClaim = body(sq, "async fn redeem_invite_atomic(");
    const postgresClaim = body(pg, "async fn redeem_invite_atomic(");
    const neoClaim = body(neo, "async fn redeem_invite_atomic(");
    return (
      shared.includes("pub accepted_event_id: Option<String>") &&
      shared.includes("InviteRedemptionCommit::AlreadyCommitted") &&
      ordered(service, ["let mut clock = self.write.lock().await", "let hlc = clock.tick()", "self.dir.redeem_invite_atomic", "if let InviteRedemptionCommit::NewlyCommitted { event } = &committed", "publish_persisted_locked", "Ok(committed)"]) &&
      !service.includes("InviteRedemptionCommit::AlreadyCommitted") &&
      sq.includes("CHECK ((accepted_at IS NULL) = (accepted_event_id IS NULL))") &&
      sqliteClaim.includes("TransactionBehavior::Immediate") &&
      ordered(sqliteClaim.slice(sqliteClaim.indexOf("SET accepted_at = ?2")), ["SET accepted_at = ?2, accepted_event_id = ?3", "persist_event_with_identity", "self.project(&tx", "tx.commit()"]) &&
      pg.includes("CHECK ((accepted_at IS NULL) = (accepted_event_id IS NULL))") &&
      postgresClaim.includes("WHERE singleton FOR UPDATE") &&
      postgresClaim.includes("WHERE selector = $1 FOR UPDATE") &&
      ordered(postgresClaim.slice(postgresClaim.indexOf("SET accepted_at = $2")), ["SET accepted_at = $2, accepted_event_id = $3", "INSERT INTO hub_directory_event", "self.project(&mut tx", "tx.commit()"]) &&
      pg.includes("CREATE TEMP TABLE hub_rebuild_space_invite ON COMMIT DROP") &&
      pg.includes("accepted_at, accepted_event_id FROM hub_rebuild_space_invite") &&
      neoClaim.includes("SET c.claimNonce") &&
      neoClaim.includes("SET i.claimNonce") &&
      ordered(neoClaim.slice(neoClaim.indexOf("SET i.acceptedAt = $accepted_at")), ["SET i.acceptedAt = $accepted_at, i.acceptedEventId = $event_id", "CREATE (e:DirectoryEvent", "self.project(&mut txn", "txn.commit()"]) &&
      [sq, pg, neo].every((source) => source.includes("DirectoryEventBody::ArtifactCheckpointPublished { .. } | DirectoryEventBody::InviteRedeemed { .. }")) &&
      [sq, pg, neo].every((source) => source.includes("accepted_at") && source.includes("accepted_event_id"))
    );
  };
  if (!sourceClosed(directory, sqlite, postgres, neo4j)) throw new Error("invite redemption production transaction fence is incomplete");
  const sourceHostiles = [
    [directory.replace("pub accepted_event_id: Option<String>", ""), sqlite, postgres, neo4j],
    [directory.replace("if let InviteRedemptionCommit::NewlyCommitted { event } = &committed", "if let InviteRedemptionCommit::AlreadyCommitted { event } = &committed"), sqlite, postgres, neo4j],
    [
      directory,
      sqlite.replace(
        "transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;\n        let accepted_at_ms",
        "transaction_with_behavior(rusqlite::TransactionBehavior::Deferred).map_err(backend)?;\n        let accepted_at_ms",
      ),
      postgres,
      neo4j,
    ],
    [directory, sqlite, postgres.replaceAll("FOR UPDATE", ""), neo4j],
    [directory, sqlite, postgres.replace("CREATE TEMP TABLE hub_rebuild_space_invite ON COMMIT DROP", ""), neo4j],
    [directory, sqlite, postgres, neo4j.replaceAll("claimNonce", "claimRead")],
  ];
  sourceHostiles.forEach((hostile, index) => {
    if (sourceClosed(hostile[0]!, hostile[1]!, hostile[2]!, hostile[3]!)) throw new Error(`invite redemption source oracle admitted removed transaction fence ${index}`);
  });

  const hub = readHubBinSource(repoRoot);
  const stateAuthorityClosed = (shared: string, sq: string, pg: string, neo: string, server: string): boolean => {
    const preflight = body(shared, "pub(crate) fn invite_redemption_preflight(");
    const hint = body(shared, "pub(crate) fn verify_invite_redemption_scope_hint(");
    const route = body(server, "async fn post_redeem_invite(");
    return (
      ordered(preflight, ["space_state == InviteRedemptionSpaceStateV1::Missing", "record.accepted_at.is_some()", "space_state == InviteRedemptionSpaceStateV1::Archived && record.role == SpaceRole::Author"]) &&
      hint.includes("actor_user_id(actor).ok() != Some(user_id)") &&
      hint.includes("!constant_time_digest_eq(&record.secret_digest, &capability.secret_digest())") &&
      body(sq, "async fn redeem_invite_atomic(").includes("SELECT kind FROM hub_space WHERE id = ?1") &&
      body(pg, "async fn redeem_invite_atomic(").includes("SELECT kind FROM hub_space WHERE id = $1") &&
      body(neo, "async fn redeem_invite_atomic(").includes("RETURN s.kind AS kind") &&
      [sq, pg, neo].every((source) => body(source, "async fn redeem_invite_atomic(").includes("InviteRedemptionSpaceStateV1::from_kind")) &&
      [sq, pg, neo].every((source) => body(source, "async fn invite_redemption_scope_hint(").includes("verify_invite_redemption_scope_hint(record.as_ref(), capability, actor, user_id)")) &&
      ordered(route, [
        "state.directory.invite_redemption_scope_hint",
        "false).await",
        "state.socket_binding_gates.acquire_bindings",
        "revalidate_directory_caller",
        "state.directory_service.redeem_invite",
        "InviteRedemptionCommit::NewlyCommitted",
        "invalidate_directory_event_authority",
        "Ok(DirectoryJson",
      ]) &&
      ["SocketBindingKeyV1::User(user.user_id.clone())", "SocketBindingKeyV1::Session(user.session_id.clone())", "SocketBindingKeyV1::DirectorySpaceAuthority", "SocketBindingKeyV1::Membership"].every((key) => route.includes(key)) &&
      route.split("revalidate_directory_caller(&state, &user)").length === 3 &&
      !route.includes("drop(_authority)") &&
      !route.includes("get_role")
    );
  };
  if (!stateAuthorityClosed(directory, sqlite, postgres, neo4j, hub)) throw new Error("invite state or ingress authority topology is incomplete");
  const route = body(hub, "async fn post_redeem_invite(");
  const stateHostiles = [
    [directory.replace("space_state == InviteRedemptionSpaceStateV1::Archived && record.role == SpaceRole::Author", "false"), sqlite, postgres, neo4j, hub],
    [directory, sqlite.replaceAll("SELECT kind FROM hub_space WHERE id = ?1", "SELECT id FROM hub_space WHERE id = ?1"), postgres, neo4j, hub],
    [directory, sqlite, postgres.replaceAll("SELECT kind FROM hub_space WHERE id = $1", "SELECT id FROM hub_space WHERE id = $1"), neo4j, hub],
    [directory, sqlite, postgres, neo4j.replaceAll("RETURN s.kind AS kind", "RETURN s.id AS kind"), hub],
    [directory.replace("!constant_time_digest_eq(&record.secret_digest, &capability.secret_digest())", "false"), sqlite, postgres, neo4j, hub],
    [directory, sqlite, postgres, neo4j, hub.replace(route, route.replace("SocketBindingKeyV1::User(user.user_id.clone())", "SocketBindingKeyV1::User(String::new())"))],
  ];
  for (const [index, candidate] of stateHostiles.entries()) {
    if (stateAuthorityClosed(candidate[0]!, candidate[1]!, candidate[2]!, candidate[3]!, candidate[4]!)) throw new Error(`invite state authority oracle admitted removed fence ${index}`);
  }
  const archiveClosed = (shared: string, sq: string, pg: string, neo: string): boolean => {
    const execute = body(shared, "pub async fn execute_idempotent(");
    const appendTail = execute.slice(execute.indexOf("self.dir.append_decided_events"));
    const invariant = body(shared, "pub(crate) fn directory_projection_rejection_v1(");
    const decider = shared.slice(shared.indexOf("DirectoryCommand::ArchiveSpace { space_id } =>"), shared.indexOf("DirectoryCommand::DeleteSpace { space_id } =>"));
    return (
      !decider.includes("list_members") &&
      decider.includes("Ok(single(") &&
      invariant.includes("space_kind.is_none()") &&
      invariant.includes('Some("archive")') &&
      invariant.includes("DirectoryEventBody::MemberUpserted") &&
      invariant.includes("DirectoryEventBody::InviteRedeemed") &&
      ordered(appendTail, ["self.dir.append_decided_events", "await?", "DirectoryAppendOutcomeV1::RejectedBeforeCommit", "release_directory_command_receipt(&claim.actor_user_id, &claim.request_id, &claim.command_sha256)"]) &&
      [sq, pg, neo].every((source) => {
        const append = body(source, "async fn append_decided_events(");
        const release = body(source, "async fn release_directory_command_receipt(");
        const project = body(source, source === sq ? "fn project(" : "async fn project(");
        return (
          ordered(append, ["projection_rejection", ".rollback()", "RejectedBeforeCommit", ".commit()"]) &&
          project.includes("projection_rejection") &&
          project.includes("'spectator'") &&
          project.includes("'author'") &&
          release.includes("command_sha256") &&
          release.includes("pending") &&
          release.includes("changed != 1")
        );
      })
    );
  };
  if (!archiveClosed(directory, sqlite, postgres, neo4j)) throw new Error("archive transaction or rollback-confirmed receipt authority is incomplete");
  const archiveHostiles = [
    [directory.replace('space_kind == Some("archive")', "false"), sqlite, postgres, neo4j],
    [directory, sqlite.replaceAll("DirectoryAppendOutcomeV1::RejectedBeforeCommit", "DirectoryAppendOutcomeV1::Appended"), postgres, neo4j],
    [directory, sqlite, postgres.replaceAll(".rollback().await", ".commit().await"), neo4j],
    [directory, sqlite, postgres, neo4j.replaceAll("changed != 1", "changed < 0")],
    [directory.replace("self.dir.append_decided_events(&decision.events).await?", "self.dir.append_decided_events(&decision.events).await.unwrap_or_default()"), sqlite, postgres, neo4j],
  ];
  for (const [index, candidate] of archiveHostiles.entries()) if (archiveClosed(candidate[0]!, candidate[1]!, candidate[2]!, candidate[3]!)) throw new Error(`archive authority oracle admitted removed fence ${index}`);
  const checks =
    fixture.vectors.length + fixture.spaceStates.length + fixture.authorityRaces.length + fixture.backendOrders.length + fixture.receiptRelease.length + fixture.hostiles.length + sourceHostiles.length + stateHostiles.length + archiveHostiles.length;
  console.log(`invite-redemption-transaction-oracle: AJV=1 vectors=${fixture.vectors.length} hostiles=${fixture.hostiles.length} source-hostiles=${sourceHostiles.length}`);
  return checks;
}

export class InviteRedemptionTransactionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && !["--native", "--postgres", "--neo4j"].includes(segments[0]!))) throw new Error("invite-redemption-transaction-check accepts only --native, --postgres, or --neo4j");
    const checks = await proveInviteRedemptionTransaction(this.repoRoot);
    const lawGroups: ExactCargoLawGroup[] =
      segments[0] === "--postgres"
        ? [{ package: "semio-hub-directory-backend-laws", target: { kind: "lib" }, laws: ["postgres_tests::invite_redemption_claim_matches_neutral_contract"] }]
        : segments[0] === "--neo4j"
          ? [{ package: "semio-hub-directory-backend-laws", target: { kind: "lib" }, laws: ["neo4j_tests::invite_redemption_claim_matches_neutral_contract"] }]
          : segments[0] === "--native"
            ? [
                {
                  package: "semio-hub-directory-laws",
                  target: { kind: "lib" },
                  laws: ["unit::invite_redemption_sqlite_claim_is_exactly_once_across_concurrency_restart_and_rebuild", "unit::invite_redemption_projection_failure_rolls_back_claim_event_and_membership", "unit::invite_redemption_commit_and_publication_precede_the_next_directory_command", "unit::invite_archive_projection_serializes_independent_service_decisions"],
                },
              ]
            : [];
    if (lawGroups.length > 0) {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: lawGroups, progress(event) {
          console.log(`invite-redemption-transaction ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        } });
      console.log(`invite-redemption-transaction-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`invite-redemption-transaction-check: checks=${checks} mode=${segments[0] ?? "source"}`);
  }
}

export class DirectoryOrderedPublicationCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-ordered-publication-check accepts only --native");
    const checks = orderedDirectoryPublicationOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, ...exactCargoStageEnvironments(), groups: [
          {
            package: "semio-hub-directory-laws",
            target: { kind: "lib" },
            laws: ["unit::directory_append_and_live_broadcast_share_one_writer_guard_and_projection_order"],
          },
        ], progress(event) {
          console.log(`directory-ordered-publication ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        } });
      console.log(`directory-ordered-publication-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-ordered-publication-check: checks=${checks} clean`);
  }
}
