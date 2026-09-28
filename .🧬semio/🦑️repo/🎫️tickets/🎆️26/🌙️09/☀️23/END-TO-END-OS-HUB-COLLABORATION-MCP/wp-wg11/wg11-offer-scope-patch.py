#!/usr/bin/env python3
"""🛰️ WG11 session 14c — window-3 T3 joint set with G12: the wgpu shell asks for its agent-bridge offer WITH its scope.

G12 made bridge offers scoped (rendezvous v2): the dev server serves a hub gateway's offer only to the shell open on that hub and
space whose human delegated the agent; a request with no scope gets local offers only. React's ShellHost passes `offerScope`
(hub origin, open space, the human's own live delegations from `GET /auth/agent-delegations`). Both wgpu page entries
(`🚀️browser-boot`, `🎬️renderer-boot`) poll through `watchAgentBridgeOffer` with no scope, so a hub agent could no longer reach
any wgpu shell. The scope's inputs live in the Rust shell (the session capability, the open space) — the page realm has no
token — so the shell computes the scope and ONLY its three values cross into the page (coordinator constraint).

Set:
  kernel (`semio-framework-os-kernel`): directory client `live_agent_delegation_principals` + pure `live_agent_principals`
    (NEW `📇️directory/🔌️client/🤖️agent-delegations/🦀️.rs`, the Rust twin of `🤖️delegations/🟦️.ts` parse + ShellHost's filter);
    shared vectors NEW `📇️directory/🤖️delegations/🧫️fixtures/📋️agent-delegation-list.json` (Rust client unit law + TS law).
  renderer (`semio-framework-os-renderer-wgpu`): `AgentBridgeOfferScopeV1` + `AgentBridgeOfferScopeOwnerV1` (sign-out / space
    switch retire the scope at once, a stale listing is dropped, the listing is re-read on the discovery max interval so a
    revoked or expired delegation leaves it), `pump_agent_bridge_offer_scope` on the directory pump, the published scope +
    wasm export `dumpAgentBridgeOfferScope` (JSON of the three values or `null`); law in hub-projection-workspace.
  os TS: `watchAgentBridgeOffer` option `offerScope` (re-read before every poll, like React's hook) +
    `agentBridgeOfferScopeFromJsonV1`; transport probe `agent-bridge-scope`; frame worker answers it from the export;
    both page entries pass the scope; AgentBridge + AgentDelegations vitest laws.

Dry run by default; `--write` backs every edited file up under `wp-wg11/w3-backup/offer-scope/` and applies (every anchor asserted
exactly once; refuses when a new file exists); `--revert` restores the backups and removes the new files.
"""

import difflib
import json
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
OS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules"
ENGINE = OS / "📺️renderer/🧑‍🎨engine"
WGPU = ENGINE / "🎯️targets/🧊️wgpu"
CLIENT = OS / "📇️directory/🔌️client/🦀️.rs"
CLIENT_LAWS = OS / "📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs"
DELEGATIONS_RS = OS / "📇️directory/🔌️client/🤖️agent-delegations/🦀️.rs"
VECTORS = OS / "📇️directory/🤖️delegations/🧫️fixtures/📋️agent-delegation-list.json"
SHELL = ENGINE / "🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
SHELL_LAWS = ENGINE / "🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs"
OFFER_TS = ENGINE / "🧱️elements/🔗️AgentBridge/🛰️offer/🟦️.ts"
BRIDGE_LAWS = ENGINE / "🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts"
DELEGATION_LAWS = ENGINE / "🧱️elements/🤖️AgentDelegations/🧪️tests/🧩️component/🟦️.tsx"
TRANSPORT_TS = WGPU / "🚚️browser-frame-transport/🟦️.ts"
WORKER_TS = WGPU / "🎞️frame-worker/🟦️.ts"
BROWSER_BOOT_TS = WGPU / "🚀️browser-boot/🟦️.ts"
RENDERER_BOOT_TS = WGPU / "🎬️renderer-boot/🟦️.ts"
BACKUP = Path(__file__).resolve().parent / "w3-backup/offer-scope"


def row(delegation_id, revoked=False, expires=5000, **overrides):
    value = {"delegationId": delegation_id, "agentLabel": f"agent {delegation_id}", "spaceId": "space-a", "audience": "edit", "createdAtMs": 100, "expiresAtMs": expires, "revoked": revoked}
    value.update(overrides)
    return value


VECTOR_FILE = {
    "schema": "semio.os.agent-delegation-list-vectors/v1",
    "description": "GET /auth/agent-delegations?space=<id> answers -> the live agent principals a shell names in its agent-bridge offer scope: parseAgentDelegationListV1 (🤖️delegations/🟦️.ts; a malformed row is dropped, never the listing) followed by ShellHost's offer-scope filter (not revoked, expiresAtMs > nowMs), principal = agent:<delegationId>. Answered by the TypeScript law (🤖️AgentDelegations component tests) and the Rust twin's law (📇️directory/🔌️client unit tests, live_agent_principals).",
    "cases": [
        {"name": "live rows name their agents in listing order", "nowMs": 1000, "body": {"schema": "semio.hub.auth.agent-delegation-list/v1", "delegations": [row("d-2"), row("d-1", audience="read")]}, "principals": ["agent:d-2", "agent:d-1"]},
        {"name": "a revoked delegation names no agent", "nowMs": 1000, "body": {"schema": "semio.hub.auth.agent-delegation-list/v1", "delegations": [row("d-1"), row("d-2", revoked=True)]}, "principals": ["agent:d-1"]},
        {"name": "an expired delegation names no agent", "nowMs": 5000, "body": {"schema": "semio.hub.auth.agent-delegation-list/v1", "delegations": [row("d-1", expires=5000), row("d-2", expires=5001)]}, "principals": ["agent:d-2"]},
        {
            "name": "a malformed row is dropped, never the listing",
            "nowMs": 1000,
            "body": {"schema": "semio.hub.auth.agent-delegation-list/v1", "delegations": [row("d-1", audience="admin"), row("d-2", createdAtMs=1.5), row("", agentLabel="x"), row("d-4", agentLabel="line\nbreak"), row("d-5")]},
            "principals": ["agent:d-5"],
        },
        {"name": "a body that is not a listing names none", "nowMs": 1000, "bodyText": "<html>proxy error</html>", "principals": []},
    ],
}

AGENT_DELEGATIONS_RS = '''//! 🤖️ The asking human's own agent delegations in one space — `GET /auth/agent-delegations?space=<id>` — read as the live agent
//! principals a shell names when it asks the agent-bridge supervisor for its scoped offer (ticket 26/09/23, G12 × WG11
//! session 14c). The Rust twin of `📇️directory/🤖️delegations/🟦️.ts` (`parseAgentDelegationListV1`, `agentPrincipalIdV1`) plus
//! ShellHost's offer-scope filter; both answer `🤖️delegations/🧫️fixtures/📋️agent-delegation-list.json`.

use super::{encode_url_component, DirectoryClient, DirectoryClientError, DirectoryTransport, HttpMethod};
use crate::os_pack::json::Value;
use semio_framework_async::OperationContext;

/// 📏️ The hub's own listing page bound (`AGENT_DELEGATION_PAGE_MAX`); rows past it are never read.
pub const AGENT_DELEGATION_PAGE_MAX: usize = 256;
/// 📏️ The listing answer's byte bound, checked before it is decoded.
pub const AGENT_DELEGATION_LIST_MAX_BYTES: usize = 256 * 1024;
const AGENT_LABEL_MAX_UNITS: usize = 128;
const IDENTIFIER_MAX_UNITS: usize = 256;
const SAFE_INTEGER_MAX: u64 = 9_007_199_254_740_991;

/// 🤖️ `semio_hub::auth::agent::agent_principal_id`'s twin: the actor string of one delegation's agent.
pub fn agent_principal_id(delegation_id: &str) -> String {
    format!("agent:{delegation_id}")
}

/// 🔤️ A bounded, control-free, non-empty text field, measured in UTF-16 units as the TypeScript twin measures it.
fn bounded_text<'a>(row: &'a Value, key: &str, max: usize) -> Option<&'a str> {
    row.get(key).and_then(Value::as_str).filter(|value| !value.is_empty() && value.encode_utf16().count() <= max && !value.chars().any(char::is_control))
}

/// ⏱️ A safe-integer millisecond field (`Number.isSafeInteger`).
fn safe_ms(row: &Value, key: &str) -> Option<i64> {
    row.get(key).and_then(Value::as_i64).filter(|value| value.unsigned_abs() <= SAFE_INTEGER_MAX)
}

/// 📥️ The principals of the live delegations one listing answer names at `now_ms`, in listing order: a malformed row is
/// dropped, never the listing, and a revoked or expired row names no principal — exactly what ShellHost's `offerScope` keeps.
pub fn live_agent_principals(body: &str, now_ms: i64) -> Vec<String> {
    let Ok(value) = crate::os_pack::json::parse(body) else { return Vec::new() };
    let Some(rows) = value.get("delegations").and_then(Value::as_array) else { return Vec::new() };
    rows.iter()
        .take(AGENT_DELEGATION_PAGE_MAX)
        .filter_map(|row| {
            let delegation_id = bounded_text(row, "delegationId", IDENTIFIER_MAX_UNITS)?;
            bounded_text(row, "agentLabel", AGENT_LABEL_MAX_UNITS)?;
            bounded_text(row, "spaceId", IDENTIFIER_MAX_UNITS)?;
            safe_ms(row, "createdAtMs")?;
            let expires_at_ms = safe_ms(row, "expiresAtMs")?;
            matches!(row.get("audience").and_then(Value::as_str), Some("read" | "edit")).then_some(())?;
            let revoked = row.get("revoked").and_then(Value::as_bool) == Some(true);
            (!revoked && expires_at_ms > now_ms).then(|| agent_principal_id(delegation_id))
        })
        .collect()
}

impl<T: DirectoryTransport> DirectoryClient<T> {
    /// 🤖️ The live agent principals the asking human delegated in `space_id` at `now_ms` — one bounded
    /// `GET /auth/agent-delegations?space=<id>` read with this client's own session capability, which never leaves it.
    pub async fn live_agent_delegation_principals(&self, ctx: &OperationContext, space_id: &str, now_ms: i64) -> Result<Vec<String>, DirectoryClientError> {
        if space_id.is_empty() || space_id.encode_utf16().count() > IDENTIFIER_MAX_UNITS || space_id.chars().any(char::is_control) {
            return Err(DirectoryClientError::Decode("agent delegation listing names no space".into()));
        }
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let path = format!("/auth/agent-delegations?space={}", encode_url_component(space_id));
        let response = self.transport.http(ctx, HttpMethod::Get, &self.url(&path), bearer, None).await?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        match response.status {
            200 => {}
            401 => return Err(DirectoryClientError::Unauthorized),
            status => return Err(DirectoryClientError::Http { status, body: String::new() }),
        }
        if response.body.len() > AGENT_DELEGATION_LIST_MAX_BYTES {
            return Err(DirectoryClientError::Decode("agent delegation listing exceeds its bound".into()));
        }
        let body = String::from_utf8(response.body).map_err(|error| DirectoryClientError::Decode(error.to_string()))?;
        Ok(live_agent_principals(&body, now_ms))
    }
}
'''

CLIENT_EDITS = [
    (
        '''//#region 🧩️ExecutionTargetModule
#[path = "🧩️execution-target-module/🦀️.rs"]''',
        '''//#region 🤖️AgentDelegations
#[path = "🤖️agent-delegations/🦀️.rs"]
pub mod agent_delegations;
//#endregion 🤖️AgentDelegations

//#region 🧩️ExecutionTargetModule
#[path = "🧩️execution-target-module/🦀️.rs"]''',
    )
]

CLIENT_LAW = r'''
/// 🤖️ The Rust twin answers the shared delegation-list vectors (`🤖️delegations/🧫️fixtures/📋️agent-delegation-list.json`) the
/// TypeScript parser + ShellHost filter answer: a malformed row is dropped, never the listing; revoked or expired rows name no agent.
#[test]
fn live_agent_principals_answer_the_shared_delegation_list_vectors() {
    let vectors = crate::os_pack::json::parse(include_str!("../../../🤖️delegations/🧫️fixtures/📋️agent-delegation-list.json")).expect("the delegation vectors parse");
    let cases = vectors.get("cases").and_then(crate::os_pack::json::Value::as_array).expect("cases");
    assert!(!cases.is_empty());
    for case in cases {
        let name = case.get("name").and_then(crate::os_pack::json::Value::as_str).expect("name");
        let body = match case.get("body") {
            Some(body) => crate::os_pack::json::to_string(body),
            None => case.get("bodyText").and_then(crate::os_pack::json::Value::as_str).expect("bodyText").to_string(),
        };
        let now_ms = case.get("nowMs").and_then(crate::os_pack::json::Value::as_i64).expect("nowMs");
        let expected: Vec<String> = case.get("principals").and_then(crate::os_pack::json::Value::as_array).expect("principals").iter().map(|principal| principal.as_str().expect("principal").to_string()).collect();
        assert_eq!(agent_delegations::live_agent_principals(&body, now_ms), expected, "{name}");
    }
}
'''

SHELL_SCOPE = '''//#region 🛰️AgentBridgeOfferScope
/// 🎯️ Who this shell is for the agent-bridge supervisor's scoped offer (ticket 26/09/23, G12 × WG11 session 14c): the hub it
/// signed in to, the space it is in and the agents ITS human delegated there — ShellHost's `offerScope`. Only these three
/// values cross into the page realm (`dumpAgentBridgeOfferScope`); the session capability that lists them never does.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentBridgeOfferScopeV1 {
    pub hub_origin: String,
    pub space_id: String,
    pub agent_principal_ids: Vec<String>,
}

/// ⏱️ How often a standing scope re-reads its delegations — the offer watcher's own maximum discovery interval
/// (`BRIDGE_DISCOVERY_MAX_INTERVAL_MS`), so a revoked or expired delegation leaves the scope within one poll of it.
pub const AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS: f64 = 30_000.0;

/// 🔁️ Keeps [`AgentBridgeOfferScopeV1`] true to the shell: a sign-out or a space switch retires the scope AT ONCE (one space's
/// agents are never offered to another), a listing answered for a target the shell has since left is dropped, and the
/// listing is re-read every [`AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS`] while the target stands.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AgentBridgeOfferScopeOwnerV1 {
    target: Option<(String, String)>,
    scope: Option<AgentBridgeOfferScopeV1>,
    next_listing_at_ms: f64,
    listing: bool,
}

impl AgentBridgeOfferScopeOwnerV1 {
    /// 🎯️ Follows the shell's current `(hubOrigin, spaceId)`; answers the target whose listing is due now, if any.
    pub fn observe(&mut self, target: Option<(String, String)>, now_ms: f64) -> Option<(String, String)> {
        if self.target != target {
            self.target = target;
            self.scope = None;
            self.listing = false;
            self.next_listing_at_ms = now_ms;
        }
        if self.listing || now_ms < self.next_listing_at_ms {
            return None;
        }
        let due = self.target.clone()?;
        self.listing = true;
        Some(due)
    }

    /// 📥️ Settles the listing asked for `target`: its live principals become the scope, a failed listing leaves no scope
    /// (local offers only, as ShellHost's rejected `offerScope`), and an answer for a target the shell has left is dropped.
    pub fn settle(&mut self, target: &(String, String), principals: Option<Vec<String>>, now_ms: f64) {
        if self.target.as_ref() != Some(target) {
            return;
        }
        self.listing = false;
        self.next_listing_at_ms = now_ms + AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS;
        self.scope = principals.map(|agent_principal_ids| AgentBridgeOfferScopeV1 { hub_origin: target.0.clone(), space_id: target.1.clone(), agent_principal_ids });
    }

    /// 🎯️ The standing scope, if the shell is signed in and in a space whose listing answered.
    pub fn scope(&self) -> Option<&AgentBridgeOfferScopeV1> {
        self.scope.as_ref()
    }

    /// 📜️ The scope as the page realm reads it: the three values, or `null`.
    pub fn page_json(&self) -> String {
        self.scope.as_ref().and_then(|scope| serde_json::to_string(scope).ok()).unwrap_or_else(|| "null".to_string())
    }
}

thread_local! {
    static PUBLISHED_AGENT_BRIDGE_OFFER_SCOPE: std::cell::RefCell<String> = std::cell::RefCell::new("null".to_string());
}

/// 📤️ Publishes the shell's scope for the page realm's offer watcher.
fn publish_agent_bridge_offer_scope(json: String) {
    PUBLISHED_AGENT_BRIDGE_OFFER_SCOPE.with(|slot| *slot.borrow_mut() = json);
}

/// 🛰️ The scope the page's offer watcher asks the supervisor with (`🔗️AgentBridge/🛰️offer` `agentBridgeOfferScopeFromJsonV1`):
/// `{hubOrigin, spaceId, agentPrincipalIds}` or `null` — read-only, and nothing else of the session crosses.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = dumpAgentBridgeOfferScope)]
pub fn dump_agent_bridge_offer_scope() -> String {
    PUBLISHED_AGENT_BRIDGE_OFFER_SCOPE.with(|slot| slot.borrow().clone())
}
//#endregion 🛰️AgentBridgeOfferScope

//#region 🌉️AgentBridgeConfigDoor'''

SHELL_EDITS = [
    (
        '''    pub agent_bridge: crate::agent_bridge_door::AgentBridgeTransport,
''',
        '''    pub agent_bridge: crate::agent_bridge_door::AgentBridgeTransport,
    /// 🛰️ The scope this shell asks the agent-bridge supervisor with ([`AgentBridgeOfferScopeOwnerV1`]).
    pub agent_bridge_offer_scope: AgentBridgeOfferScopeOwnerV1,
''',
    ),
    (
        '''            agent_bridge: crate::agent_bridge_door::AgentBridgeTransport::new(),
''',
        '''            agent_bridge: crate::agent_bridge_door::AgentBridgeTransport::new(),
            agent_bridge_offer_scope: AgentBridgeOfferScopeOwnerV1::default(),
''',
    ),
    (
        '''        changed |= self.pump_hub_check_in().await;
''',
        '''        changed |= self.pump_hub_check_in().await;
        self.pump_agent_bridge_offer_scope().await;
''',
    ),
    (
        '''    /// 🎬️ One bounded agent-bridge turn, folded into the same 100 ms `PumpSync` slot the directory
    /// lane already rides. Answers whether the chrome must repaint.''',
        '''    /// 🛰️ One bounded offer-scope turn on the directory pump: follows sign-in and the space the shell is in, re-reads this
    /// human's own live delegations when due (one `GET /auth/agent-delegations`, [`AgentBridgeOfferScopeOwnerV1`]) and
    /// publishes the scope for the page realm.
    async fn pump_agent_bridge_offer_scope(&mut self) {
        let target = self.identity.as_ref().zip(self.open_space_id.as_ref()).filter(|(_, space_id)| !space_id.is_empty()).map(|(identity, space_id)| (identity.hub_base_url.trim_end_matches('/').to_string(), space_id.clone()));
        if let Some(due) = self.agent_bridge_offer_scope.observe(target, chrome_now_ms()) {
            let principals = match self.directory_client.clone() {
                Some(client) => client.live_agent_delegation_principals(&self.directory_command_ctx(), &due.1, i64::try_from(Self::directory_now_ms()).unwrap_or(i64::MAX)).await.ok(),
                None => None,
            };
            self.agent_bridge_offer_scope.settle(&due, principals, chrome_now_ms());
        }
        publish_agent_bridge_offer_scope(self.agent_bridge_offer_scope.page_json());
    }

    /// 🎬️ One bounded agent-bridge turn, folded into the same 100 ms `PumpSync` slot the directory
    /// lane already rides. Answers whether the chrome must repaint.''',
    ),
    ("//#region 🌉️AgentBridgeConfigDoor", SHELL_SCOPE),
]

SHELL_LAW = r'''/// 🛰️ The offer scope stays true to the shell (G12 × WG11): a signed-in shell in a space names its human's live agents once the
/// listing answers; a space switch or a sign-out retires the scope AT ONCE; a listing answered for a target the shell left is
/// dropped; the listing is re-read on the discovery max interval, so a revoked delegation leaves the scope; only the three
/// values reach the page.
#[test]
fn the_agent_bridge_offer_scope_follows_sign_in_space_and_revocation() {
    let hub = "http://127.0.0.1:7800".to_string();
    let (space_a, space_b) = ((hub.clone(), "space-a".to_string()), (hub.clone(), "space-b".to_string()));
    let mut owner = AgentBridgeOfferScopeOwnerV1::default();
    assert_eq!((owner.observe(None, 0.0), owner.page_json()), (None, "null".to_string()), "no scope before sign-in");
    let due = owner.observe(Some(space_a.clone()), 10.0).expect("a new target lists at once");
    assert_eq!(owner.observe(Some(space_a.clone()), 11.0), None, "one listing in flight at a time");
    owner.settle(&due, Some(vec!["agent:d-1".into(), "agent:d-2".into()]), 20.0);
    assert_eq!(owner.page_json(), r#"{"hubOrigin":"http://127.0.0.1:7800","spaceId":"space-a","agentPrincipalIds":["agent:d-1","agent:d-2"]}"#);
    assert_eq!(owner.observe(Some(space_a.clone()), 20.0 + AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS - 1.0), None, "a standing scope is not re-listed early");
    let refresh = owner.observe(Some(space_a.clone()), 20.0 + AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS).expect("the listing is re-read on schedule");
    owner.settle(&refresh, Some(vec!["agent:d-2".into()]), 30_100.0);
    assert_eq!(owner.scope().map(|scope| scope.agent_principal_ids.clone()), Some(vec!["agent:d-2".to_string()]), "a revoked delegation leaves the scope");
    let switched = owner.observe(Some(space_b.clone()), 30_200.0).expect("a space switch lists the new space at once");
    assert_eq!(owner.page_json(), "null", "a space switch retires the old space's agents at once");
    owner.settle(&space_a, Some(vec!["agent:d-9".into()]), 30_300.0);
    assert_eq!(owner.page_json(), "null", "a listing for a space the shell left is dropped");
    owner.settle(&switched, None, 30_400.0);
    assert_eq!(owner.page_json(), "null", "a failed listing leaves local offers only");
    let retry = owner.observe(Some(space_b.clone()), 30_400.0 + AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS).expect("a failed listing is retried on schedule");
    owner.settle(&retry, Some(Vec::new()), 60_500.0);
    assert_eq!(owner.page_json(), r#"{"hubOrigin":"http://127.0.0.1:7800","spaceId":"space-b","agentPrincipalIds":[]}"#);
    assert_eq!(owner.observe(None, 60_600.0), None);
    assert_eq!(owner.page_json(), "null", "a sign-out retires the scope at once");
}

'''
SHELL_LAW_ANCHOR = '''#[cfg(not(target_arch = "wasm32"))]
fn remote_of(shell: &ShellState) -> String {'''

OFFER_EDITS = [
    (
        '''/** 🔎️ Asks the local supervisor for the offer a shell of `scope` may dial.''',
        '''/** 🎯️ Reads the scope a wgpu shell publishes for its page (`dumpAgentBridgeOfferScope`): exactly `{hubOrigin, spaceId,
 * agentPrincipalIds}` of non-empty strings, else `null` (the shell is in no hub space and dials only local offers). */
export function agentBridgeOfferScopeFromJsonV1(json: string | null): AgentBridgeOfferScopeV1 {
  if (json === null) return null;
  let value: unknown;
  try {
    value = JSON.parse(json);
  } catch {
    return null;
  }
  if (typeof value !== "object" || value === null || Array.isArray(value)) return null;
  const record = value as Record<string, unknown>;
  if (Object.keys(record).sort().join(",") !== "agentPrincipalIds,hubOrigin,spaceId") return null;
  const { hubOrigin, spaceId, agentPrincipalIds } = record;
  if (typeof hubOrigin !== "string" || hubOrigin.length === 0 || typeof spaceId !== "string" || spaceId.length === 0) return null;
  if (!Array.isArray(agentPrincipalIds) || !agentPrincipalIds.every((agent) => typeof agent === "string" && agent.length > 0)) return null;
  return { hubOrigin, spaceId, agentPrincipalIds: agentPrincipalIds as string[] };
}

/** 🔎️ Asks the local supervisor for the offer a shell of `scope` may dial.''',
    ),
    (
        '''/** 🛰️ Polls the supervisor's offer on the shared schedule and publishes every CHANGE — an offer appearing, changing or
 * going away — the page-realm loop both wgpu page entries run (`🚀️browser-boot` forwards it to its frame Worker,
 * `🎬️renderer-boot` hands it to the page-mounted renderer). Returns the stop. */
export function watchAgentBridgeOffer(
  publish: (offer: AgentBridgeConfig | null) => void,
  options: { readonly endpoint?: string; readonly fetchImpl?: BridgeOfferFetch; readonly setTimer?: (run: () => void, delayMs: number) => unknown; readonly clearTimer?: (handle: unknown) => void } = {},
): () => void {''',
        '''/** 🛰️ Polls the supervisor's offer on the shared schedule and publishes every CHANGE — an offer appearing, changing or
 * going away — the page-realm loop both wgpu page entries run (`🚀️browser-boot` forwards it to its frame Worker,
 * `🎬️renderer-boot` hands it to the page-mounted renderer). `offerScope` is re-read before every poll, exactly like React's
 * hook: the shell's hub, space and its human's agents (a failed read is a shell in no hub space). Returns the stop. */
export function watchAgentBridgeOffer(
  publish: (offer: AgentBridgeConfig | null) => void,
  options: {
    readonly endpoint?: string;
    readonly fetchImpl?: BridgeOfferFetch;
    readonly setTimer?: (run: () => void, delayMs: number) => unknown;
    readonly clearTimer?: (handle: unknown) => void;
    readonly offerScope?: (signal: AbortSignal) => Promise<AgentBridgeOfferScopeV1>;
  } = {},
): () => void {''',
    ),
    (
        '''    const discovered = await fetchAgentBridgeConfig(options.endpoint ?? AGENT_BRIDGE_OFFER_ENDPOINT, options.fetchImpl, abort.signal);
    if (stopped) return;''',
        '''    const scope = await (options.offerScope?.(abort.signal) ?? Promise.resolve(null)).catch(() => null);
    if (stopped) return;
    const discovered = await fetchAgentBridgeConfig(options.endpoint ?? AGENT_BRIDGE_OFFER_ENDPOINT, options.fetchImpl, abort.signal, scope);
    if (stopped) return;''',
    ),
]

TRANSPORT_EDITS = [
    (
        '''export type BrowserFrameIntrospectionProbe = "structure" | "frame-stats" | "accessibility" | "mesh-stats" | "chrome";''',
        '''export type BrowserFrameIntrospectionProbe = "structure" | "frame-stats" | "accessibility" | "mesh-stats" | "chrome" | "agent-bridge-scope";''',
    ),
    (
        ''' * elements, so the tree has to cross this seam as data (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export type BrowserFrameIntrospectionProbe''',
        ''' * elements, so the tree has to cross this seam as data (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 *
 * 🛰️ `agent-bridge-scope` is production too: the shell's agent-bridge offer scope (hub origin, space, its human's agent
 * principals — never the session capability) for the page's offer watcher (ticket 26/09/23, G12 × WG11 session 14c). */
export type BrowserFrameIntrospectionProbe''',
    ),
]

WORKER_EDITS = [
    (
        '''  dumpChrome?: (windowId?: string) => string;
''',
        '''  dumpChrome?: (windowId?: string) => string;
  /** 🛰️ The shell's agent-bridge offer scope (`{hubOrigin, spaceId, agentPrincipalIds}` or `null`) for the page's offer
   * watcher — production, not a probe; the session capability stays in this isolate. */
  dumpAgentBridgeOfferScope?: () => string;
''',
    ),
    (
        '''  const hook = message.probe === "structure" ? bindings.dumpStructure : message.probe === "accessibility" ? bindings.dumpAccessibility : message.probe === "mesh-stats" ? bindings.dumpMeshStats : message.probe === "chrome" ? bindings.dumpChrome : bindings.dumpFrameStats;''',
        '''  const hook = message.probe === "structure" ? bindings.dumpStructure : message.probe === "accessibility" ? bindings.dumpAccessibility : message.probe === "mesh-stats" ? bindings.dumpMeshStats : message.probe === "chrome" ? bindings.dumpChrome : message.probe === "agent-bridge-scope" ? bindings.dumpAgentBridgeOfferScope : bindings.dumpFrameStats;''',
    ),
]

BROWSER_BOOT_EDITS = [
    (
        '''import { watchAgentBridgeOffer } from "../../../🧱️elements/🔗️AgentBridge/🛰️offer/🟦️.ts";''',
        '''import { agentBridgeOfferScopeFromJsonV1, watchAgentBridgeOffer } from "../../../🧱️elements/🔗️AgentBridge/🛰️offer/🟦️.ts";''',
    ),
    (
        '''  const stopAgentBridgeOffer = watchAgentBridgeOffer((offer) => transport.setHostAgentBridge(offer));''',
        '''  const stopAgentBridgeOffer = watchAgentBridgeOffer((offer) => transport.setHostAgentBridge(offer), { offerScope: async () => agentBridgeOfferScopeFromJsonV1(await transport.introspect("agent-bridge-scope")) });''',
    ),
]

RENDERER_BOOT_EDITS = [
    (
        '''import { watchAgentBridgeOffer } from "../../../🧱️elements/🔗️AgentBridge/🛰️offer/🟦️.ts";''',
        '''import { agentBridgeOfferScopeFromJsonV1, watchAgentBridgeOffer } from "../../../🧱️elements/🔗️AgentBridge/🛰️offer/🟦️.ts";''',
    ),
    (
        '''    semioWgpuSetAgentBridgeConfig?: (url: string, admissionProof: string) => void;
    uploadIconAtlas?''',
        '''    semioWgpuSetAgentBridgeConfig?: (url: string, admissionProof: string) => void;
    dumpAgentBridgeOfferScope?: () => string;
    uploadIconAtlas?''',
    ),
    (
        '''  const stopAgentBridgeOffer = watchAgentBridgeOffer((offer) => rendererModule.semioWgpuSetAgentBridgeConfig?.(offer?.url ?? "", offer?.admissionProof ?? ""));''',
        '''  const stopAgentBridgeOffer = watchAgentBridgeOffer((offer) => rendererModule.semioWgpuSetAgentBridgeConfig?.(offer?.url ?? "", offer?.admissionProof ?? ""), {
    offerScope: async () => agentBridgeOfferScopeFromJsonV1(rendererModule.dumpAgentBridgeOfferScope?.() ?? null),
  });''',
    ),
]

BRIDGE_LAW_ANCHOR = '''  it("refuses a poisoned offer as no offer at all", async () => {'''
BRIDGE_LAW = '''  it("asks with the shell's scope, re-read before every poll, and reads the wgpu shell's published scope exactly", async () => {
    const { watchAgentBridgeOffer, agentBridgeOfferScopeFromJsonV1, agentBridgeOfferPathV1 } = await import("../../🛰️offer/🟦️.ts");
    const scopes = [
      { hubOrigin: "http://127.0.0.1:7800", spaceId: "space-a", agentPrincipalIds: ["agent:d-1"] },
      null,
      { hubOrigin: "http://127.0.0.1:7800", spaceId: "space-b", agentPrincipalIds: [] },
    ] as const;
    const asked: string[] = [];
    let reads = 0;
    const pending: (() => void)[] = [];
    const stop = watchAgentBridgeOffer(() => {}, {
      fetchImpl: async (input) => {
        asked.push(input);
        return { ok: true, status: 200, json: async () => agentBridgeOfferAnswerV1(null) };
      },
      offerScope: async () => scopes[Math.min(reads++, scopes.length - 1)] ?? null,
      setTimer: (run) => {
        pending.push(run);
        return pending.length;
      },
      clearTimer: () => {},
    });
    for (let poll = 0; poll < scopes.length; poll += 1) {
      await vi.waitFor(() => expect(asked.length).toBe(poll + 1));
      if (poll + 1 < scopes.length) pending[poll]!();
    }
    stop();
    expect(asked).toEqual(scopes.map((scope) => agentBridgeOfferPathV1(AGENT_BRIDGE_OFFER_ENDPOINT, scope)));
    expect(agentBridgeOfferScopeFromJsonV1(JSON.stringify(scopes[0]))).toEqual(scopes[0]);
    for (const refused of [null, "null", "not json", "[]", JSON.stringify({ hubOrigin: "http://127.0.0.1:7800", spaceId: "space-a" }), JSON.stringify({ ...scopes[0], token: "secret" }), JSON.stringify({ ...scopes[0], agentPrincipalIds: [7] }), JSON.stringify({ ...scopes[0], spaceId: "" })]) {
      expect(agentBridgeOfferScopeFromJsonV1(refused)).toBeNull();
    }
  });

'''

DELEGATION_IMPORT_ANCHOR = '''} from "../../../../../../📇️directory/🤖️delegations/🟦️.ts";
'''
DELEGATION_IMPORT_NEW = '''} from "../../../../../../📇️directory/🤖️delegations/🟦️.ts";
import delegationListVectors from "../../../../../../📇️directory/🤖️delegations/🧫️fixtures/📋️agent-delegation-list.json";
'''
DELEGATION_LAW = '''
/** 🤖️ The shared delegation-list vectors the Rust twin (`📇️directory/🔌️client` `live_agent_principals`) answers too: the
 * parser drops a malformed row, never the listing, and ShellHost's offer-scope filter keeps only live delegations. */
describe("agent delegation list vectors", () => {
  for (const vector of delegationListVectors.cases) {
    it(vector.name, () => {
      const body = "body" in vector ? JSON.stringify(vector.body) : vector.bodyText;
      const live = parseAgentDelegationListV1(body).filter((delegation) => !delegation.revoked && delegation.expiresAtMs > vector.nowMs);
      expect(live.map((delegation) => delegation.agentPrincipalId)).toEqual(vector.principals);
    });
  }
});
'''


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


NEW_FILES = {DELEGATIONS_RS: AGENT_DELEGATIONS_RS, VECTORS: json.dumps(VECTOR_FILE, ensure_ascii=False, indent=2) + "\n"}


def plans():
    read = lambda path: path.read_text(encoding="utf-8")
    client, client_laws, shell, shell_laws = read(CLIENT), read(CLIENT_LAWS), read(SHELL), read(SHELL_LAWS)
    offer, bridge_laws, delegation_laws = read(OFFER_TS), read(BRIDGE_LAWS), read(DELEGATION_LAWS)
    transport, worker, browser_boot, renderer_boot = read(TRANSPORT_TS), read(WORKER_TS), read(BROWSER_BOOT_TS), read(RENDERER_BOOT_TS)
    return [
        (CLIENT, client, replaced(CLIENT, client, CLIENT_EDITS)),
        (CLIENT_LAWS, client_laws, client_laws.rstrip("\n") + "\n" + CLIENT_LAW),
        (SHELL, shell, replaced(SHELL, shell, SHELL_EDITS)),
        (SHELL_LAWS, shell_laws, replaced(SHELL_LAWS, shell_laws, [(SHELL_LAW_ANCHOR, SHELL_LAW + SHELL_LAW_ANCHOR)])),
        (OFFER_TS, offer, replaced(OFFER_TS, offer, OFFER_EDITS)),
        (BRIDGE_LAWS, bridge_laws, replaced(BRIDGE_LAWS, bridge_laws, [(BRIDGE_LAW_ANCHOR, BRIDGE_LAW + BRIDGE_LAW_ANCHOR)])),
        (DELEGATION_LAWS, delegation_laws, replaced(DELEGATION_LAWS, delegation_laws, [(DELEGATION_IMPORT_ANCHOR, DELEGATION_IMPORT_NEW)]).rstrip("\n") + "\n" + DELEGATION_LAW),
        (TRANSPORT_TS, transport, replaced(TRANSPORT_TS, transport, TRANSPORT_EDITS)),
        (WORKER_TS, worker, replaced(WORKER_TS, worker, WORKER_EDITS)),
        (BROWSER_BOOT_TS, browser_boot, replaced(BROWSER_BOOT_TS, browser_boot, BROWSER_BOOT_EDITS)),
        (RENDERER_BOOT_TS, renderer_boot, replaced(RENDERER_BOOT_TS, renderer_boot, RENDERER_BOOT_EDITS)),
    ]


def main():
    if "--revert" in sys.argv:
        for path in [CLIENT, CLIENT_LAWS, SHELL, SHELL_LAWS, OFFER_TS, BRIDGE_LAWS, DELEGATION_LAWS, TRANSPORT_TS, WORKER_TS, BROWSER_BOOT_TS, RENDERER_BOOT_TS]:
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        DELEGATIONS_RS.unlink(missing_ok=True)
        if DELEGATIONS_RS.parent.exists():
            DELEGATIONS_RS.parent.rmdir()
        VECTORS.unlink(missing_ok=True)
        print("REVERTED: 11 files restored from backups, 2 new files removed")
        return
    for path in NEW_FILES:
        if path.exists():
            sys.exit(f"{path.relative_to(ROOT)} exists — landed already")
    write = "--write" in sys.argv
    planned = plans()
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    for path, content in NEW_FILES.items():
        print(f"\n+++ NEW {path.relative_to(ROOT)} ({len(content)} chars)")
    if write:
        for path, before, _ in planned:
            backup = BACKUP / path.relative_to(ROOT)
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.write_text(after, encoding="utf-8")
        for path, content in NEW_FILES.items():
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(planned)} files edited, {len(NEW_FILES)} new (crates: semio-framework-os-kernel, semio-framework-os-renderer-wgpu native + wasm32; os TS)")


if __name__ == "__main__":
    main()
