"""🚦️ G11 set (c), audit G13-P2-1: a per-agent-session budget on the volume of document commands an agent pushes into the hub.

The hub rate limiter covered four route families (sign-in, directory commands, invite redemption, socket grants); once an
agent's document socket was granted, nothing bounded how many command batches (one per committed MCP tool call) it could
push. New fifth class `agent-command` (schema-first: `AuthRateLimitClassV1` in the hub auth schema, Rust policy table):
burst 120, one per 250 ms sustained, charged per agent SESSION (the minted session id, so one delegation's parallel
processes each get their own session and a revoked session's budget dies with it). The socket PACES instead of refusing:
an agent `Commands` frame waits (at most `AGENT_COMMAND_PATIENCE_MS`, 10 s — the MCP's relay-acknowledgement wait) for its
bucket, BEFORE the socket re-reads its authority, so a revocation that lands during the wait still fences the frame; only a
frame that cannot be admitted within the patience is answered `Rejected("rate-limited: retry after N ms")`. Humans and
share tokens are never charged. Laws: the policy validates against its schema, every class is a schema enum member, the
paced admission waits exactly the bucket's refill and refuses beyond its patience.

usage: python3 g11-agent-command-budget.py [--apply]"""
import sys

HUB = "/Users/ueli/Documents/semio/🌎️hub"
LIMITER = f"{HUB}/🔐️auth/🚦️rate-limit/🦀️.rs"
SCHEMA = f"{HUB}/🔐️auth/🧬️schema/🔣️.json"
LAWS = f"{HUB}/🔐️auth/🧪️tests/🔬️unit/🦀️.rs"
BOOT = f"{HUB}/🏗️bootstrap/🦀️.rs"
H = []


def hunk(path, old, new, count=1):
    H.append((path, old, new, count))


hunk(LIMITER, """//! 🚦️ The hub's own per-principal and per-remote-address token buckets, in front of the credential
//! sign-in, directory-command, invite-redemption and socket-grant routes.""", """//! 🚦️ The hub's own per-principal and per-remote-address token buckets, in front of the credential
//! sign-in, directory-command, invite-redemption and socket-grant routes, and behind every agent session's document
//! commands (`agent-command`: the volume of committed tool calls an agent pushes through its document sockets).""")
hunk(LIMITER, """/// 🏷️ The four rate-limited route families.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RateLimitClassV1 {
    Auth,
    DirectoryCommand,
    InviteRedemption,
    SocketGrant,
}""", """/// 🏷️ The five rate-limited families: four routes and the agent document-command lane.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RateLimitClassV1 {
    Auth,
    DirectoryCommand,
    InviteRedemption,
    SocketGrant,
    AgentCommand,
}

/// 🧾️ Every class, in the schema enum's order — what the laws hold the schema and the policy table against.
pub const RATE_LIMIT_CLASSES: [RateLimitClassV1; 5] = [RateLimitClassV1::Auth, RateLimitClassV1::DirectoryCommand, RateLimitClassV1::InviteRedemption, RateLimitClassV1::SocketGrant, RateLimitClassV1::AgentCommand];""")
hunk(LIMITER, """            Self::SocketGrant => "socket-grant",
        }
    }""", """            Self::SocketGrant => "socket-grant",
            Self::AgentCommand => "agent-command",
        }
    }""")
hunk(LIMITER, """            Self::SocketGrant => RateLimitPolicyV1 { burst: 30, cost_ms: 200 },
        }""", """            Self::SocketGrant => RateLimitPolicyV1 { burst: 30, cost_ms: 200 },
            Self::AgentCommand => RateLimitPolicyV1 { burst: 120, cost_ms: 250 },
        }""")
hunk(LIMITER, """        RateLimitDecisionV1::Refused { retry_after_ms: worst }
    }
}""", """        RateLimitDecisionV1::Refused { retry_after_ms: worst }
    }

    /// ⏳️ Paces one request instead of refusing it outright: admits it as soon as every subject's bucket holds its cost,
    /// waiting through `wait(ms)` for exactly the refill each refusal names, at most `patience_ms` in total. A request
    /// the buckets cannot admit within that patience is refused with the wait it still needs.
    pub async fn admit_paced<W, F>(&self, class: RateLimitClassV1, subjects: &[RateLimitSubjectV1], patience_ms: u64, mut wait: W) -> RateLimitDecisionV1
    where
        W: FnMut(u64) -> F,
        F: std::future::Future<Output = ()>,
    {
        let mut waited_ms = 0u64;
        loop {
            match self.admit(class, subjects) {
                RateLimitDecisionV1::Admitted => return RateLimitDecisionV1::Admitted,
                RateLimitDecisionV1::Refused { retry_after_ms } if waited_ms.saturating_add(retry_after_ms) <= patience_ms => {
                    wait(retry_after_ms).await;
                    waited_ms = waited_ms.saturating_add(retry_after_ms);
                }
                refused => return refused,
            }
        }
    }
}""")
hunk(SCHEMA, """    "AuthRateLimitClassV1": {
      "enum": [
        "auth",
        "directory-command",
        "invite-redemption",
        "socket-grant"
      ]
    },""", """    "AuthRateLimitClassV1": {
      "enum": [
        "auth",
        "directory-command",
        "invite-redemption",
        "socket-grant",
        "agent-command"
      ]
    },""")
hunk(LAWS, """use super::rate_limit::{HubRateLimiterV1, RateLimitClassV1, RateLimitClockV1, RateLimitDecisionV1, RateLimitSubjectV1};""", """use super::rate_limit::{HubRateLimiterV1, RateLimitClassV1, RateLimitClockV1, RateLimitDecisionV1, RateLimitSubjectV1, RATE_LIMIT_CLASSES};""")
hunk(LAWS, """const SCHEMA_MODULE: &str = include_str!("../../🧬️schema/🔣️.json");""", """#[tokio::test]
async fn an_agent_session_is_paced_for_exactly_its_refill_and_refused_beyond_its_patience() {
    let clock = ManualClock::new(0);
    let limiter = HubRateLimiterV1::new(clock.clone());
    let session = RateLimitSubjectV1::principal("agent-session-1");
    let other = RateLimitSubjectV1::principal("agent-session-2");
    let policy = RateLimitClassV1::AgentCommand.policy();
    assert_eq!((policy.burst, policy.cost_ms), (120, 250), "burst 120, then one command batch per 250 ms");
    for batch in 0..policy.burst {
        assert!(limiter.admit(RateLimitClassV1::AgentCommand, &[session]).is_admitted(), "batch {batch} is inside the burst");
    }
    let waits = std::sync::Mutex::new(Vec::new());
    let paced = limiter
        .admit_paced(RateLimitClassV1::AgentCommand, &[session], 10_000, |ms| {
            waits.lock().unwrap().push(ms);
            clock.advance(i64::try_from(ms).unwrap());
            std::future::ready(())
        })
        .await;
    assert_eq!(paced, RateLimitDecisionV1::Admitted, "a paced batch is admitted once its bucket refilled");
    assert_eq!(*waits.lock().unwrap(), vec![u64::from(policy.cost_ms)], "it waited exactly one refill, never a guess");
    assert!(limiter.admit(RateLimitClassV1::AgentCommand, &[other]).is_admitted(), "another agent session has its own budget");
    let impatient = limiter.admit_paced(RateLimitClassV1::AgentCommand, &[session], u64::from(policy.cost_ms) - 1, |_| std::future::ready(())).await;
    assert_eq!(impatient, RateLimitDecisionV1::Refused { retry_after_ms: u64::from(policy.cost_ms) }, "a batch its patience cannot cover is refused with the wait it still needs");
    assert!(limiter.admit(RateLimitClassV1::DirectoryCommand, &[session]).is_admitted(), "the agent class charges no route family");
}

#[test]
fn every_rate_limit_class_is_a_schema_member_and_its_policy_validates() {
    let document: serde_json::Value = serde_json::from_str(SCHEMA_MODULE).expect("hub.auth schema module");
    let declared: Vec<&str> = document["$defs"]["AuthRateLimitClassV1"]["enum"].as_array().expect("class enum").iter().map(|class| class.as_str().expect("class name")).collect();
    assert_eq!(declared, RATE_LIMIT_CLASSES.iter().map(|class| class.as_str()).collect::<Vec<_>>(), "the schema enum and the Rust classes are one list, in one order");
    let validator = structural("AuthRateLimitPolicyV1");
    for class in RATE_LIMIT_CLASSES {
        let policy = class.policy();
        let row = serde_json::json!({ "class": class.as_str(), "burst": policy.burst, "costMs": policy.cost_ms });
        assert!(validator.is_valid_json(&row.to_string()), "{} policy {row} validates against AuthRateLimitPolicyV1", class.as_str());
    }
}

const SCHEMA_MODULE: &str = include_str!("../../🧬️schema/🔣️.json");""")
hunk(BOOT, """/// @emoji 📨️ Handles one decoded `ClientFrame` for an already-authenticated v1 socket session.""", """/// ⏳️ How long an agent's document `Commands` frame waits for its session's `agent-command` budget before it is refused
/// (the MCP gateway's relay-acknowledgement wait): pacing keeps an honest agent's commits whole, and the wait sits before the
/// socket re-reads its authority, so a revocation that lands while a frame waits still fences it.
const AGENT_COMMAND_PATIENCE_MS: u64 = 10_000;

/// @emoji 📨️ Handles one decoded `ClientFrame` for an already-authenticated v1 socket session.""")
hunk(BOOT, """    let (user_id, role, auth_session_id, authorization_generation, principal_kind) = match &auth {""", """    let agent_command_subject = match &auth {
        AuthOutcome::Session { session_id, session_kind, .. } if session_kind.is_agent() => Some(RateLimitSubjectV1::principal(session_id)),
        _ => None,
    };
    let (user_id, role, auth_session_id, authorization_generation, principal_kind) = match &auth {""")
hunk(BOOT, """                        let Ok((_lane, frame)) = decode_client_frame(&bytes).await else {
                            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                            break;
                        };""", """                        let Ok((_lane, frame)) = decode_client_frame(&bytes).await else {
                            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                            break;
                        };
                        if let (ClientFrame::Commands { batch_id, .. }, Some(subject)) = (&frame, agent_command_subject) {
                            let paced = state.rate_limits.admit_paced(RateLimitClassV1::AgentCommand, &[subject], AGENT_COMMAND_PATIENCE_MS, |ms| tokio::time::sleep(std::time::Duration::from_millis(ms))).await;
                            if let RateLimitDecisionV1::Refused { retry_after_ms } = paced {
                                let frontier = best_effort_frontier(&handle).await;
                                let ack = ServerFrame::Ack { batch_id: *batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: format!("rate-limited: retry after {retry_after_ms} ms"), messages: Vec::new() }) }], frontier };
                                if sender.send(encode(&ack, &document_id).await).await.is_err() {
                                    break;
                                }
                                continue;
                            }
                        }""")


def main():
    apply = "--apply" in sys.argv
    texts, misses = {}, []
    for path, old, new, count in H:
        texts.setdefault(path, open(path, encoding="utf-8").read())
        found = texts[path].count(old)
        if found != count:
            misses.append(f"{path.split('🌎️hub/')[1]}: expected {count}, found {found}: {old[:100]!r}")
            continue
        texts[path] = texts[path].replace(old, new)
    for miss in misses:
        print("MISS", miss)
    print(f"hunks {len(H)}, files {len(texts)}, misses {len(misses)}")
    if misses:
        sys.exit(1)
    if not apply:
        print("dry run clean")
        return
    for path, text in texts.items():
        open(path, "w", encoding="utf-8").write(text)
    print("applied")


main()
