#!/usr/bin/env python3
"""🚦️ H13 session 14c: a rate-limited route family answers its 429 with a typed body, never an empty one.

Found live (two-client e2e on 2155): the directory-command bucket (burst 60, 10/s) refused the fixture's 100-space burst with a
body-less 429; the test JSON-parsed the empty body. Every non-auth rate-limited family (directory-command, invite-redemption,
socket-grant, agent-command) now answers `RateLimitRefusalV1` (schema-first in `🔐️auth/🧬️schema`: schema, code
`rate-limited`, the class, `retryAfterMs`, an en + de message) with `retry-after` and `no-store`; credential sign-in keeps its
`AuthErrorV1`. Fixture `🔐️auth/🧫️fixtures/🚦️rate-limit-refusal-v1` (valid bodies per class + near misses) is checked by the owned
draft-07 validator (Rust unit law `the_rate_limit_refusal_is_the_declared_schema_body`) and by Ajv (TS oracle); bin law
`a_directory_command_burst_is_refused_with_a_typed_rate_limit_body`; the fixture dir is registered in the taxonomy.
Idempotent; `--dry-run` writes nothing.
"""
import json
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SCHEMA = ROOT / "🌎️hub/🔐️auth/🧬️schema/🔣️.json"
RATE = ROOT / "🌎️hub/🔐️auth/🚦️rate-limit/🦀️.rs"
AUTH_TESTS = ROOT / "🌎️hub/🔐️auth/🧪️tests/🔬️unit/🦀️.rs"
FIXTURE = ROOT / "🌎️hub/🔐️auth/🧫️fixtures/🚦️rate-limit-refusal-v1/🔣️.json"
BOOTSTRAP = ROOT / "🌎️hub/🏗️bootstrap/🦀️.rs"
BIN_TESTS = ROOT / "🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"
ORACLE = ROOT / "🌎️hub/🧪️tests/🚧️hostile-input/🟦️.ts"
TAXONOMY = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"

EN = "Too many requests in a short time. Wait a moment, then try again."
DE = "Zu viele Anfragen in kurzer Zeit. Bitte einen Moment warten und es dann erneut versuchen."

SCHEMA_DEFS = {
    "RateLimitRefusalMessageV1": {
        "description": "The notice a rate-limited request carries, in English and German; the wait itself is `retryAfterMs`.",
        "const": {"en": EN, "de": DE},
    },
    "RateLimitRefusalV1": {
        "description": "The body of the `429` a rate-limited route family (every `AuthRateLimitClassV1` except `auth`, whose sign-in keeps `AuthErrorV1`) answers when its token bucket is empty: the class that refused, how long until one request is admitted again (also the `retry-after` header, rounded up to seconds), and the notice. Nothing was executed; the client resends the same request after the wait.",
        "x-semio-formats": ["🔣️jsonschema", "🦀️rust"],
        "type": "object",
        "additionalProperties": False,
        "required": ["schema", "code", "class", "retryAfterMs", "message"],
        "properties": {
            "schema": {"const": "semio.hub.rate-limit-refusal/v1"},
            "code": {"const": "rate-limited"},
            "class": {"$ref": "#/$defs/AuthRateLimitClassV1"},
            "retryAfterMs": {"type": "integer", "minimum": 1, "maximum": 9007199254740991},
            "message": {"$ref": "#/$defs/RateLimitRefusalMessageV1"},
        },
    },
}

FIXTURE_DOC = {
    "schema": "semio.hub.rate-limit-refusal-fixture/v1",
    "description": "What a hub answers when a rate-limited route family's token bucket is empty (`RateLimitRefusalV1`, `🔐️auth/🧬️schema`): `valid` holds one body per non-auth class with the wait the limiter computes for an empty bucket of that class (its `costMs`); `invalid` are near misses a client must not accept as the typed refusal (Rust owned draft-07 validator and Ajv).",
    "valid": [
        {"schema": "semio.hub.rate-limit-refusal/v1", "code": "rate-limited", "class": "directory-command", "retryAfterMs": 100, "message": {"en": EN, "de": DE}},
        {"schema": "semio.hub.rate-limit-refusal/v1", "code": "rate-limited", "class": "invite-redemption", "retryAfterMs": 6000, "message": {"en": EN, "de": DE}},
        {"schema": "semio.hub.rate-limit-refusal/v1", "code": "rate-limited", "class": "socket-grant", "retryAfterMs": 200, "message": {"en": EN, "de": DE}},
        {"schema": "semio.hub.rate-limit-refusal/v1", "code": "rate-limited", "class": "agent-command", "retryAfterMs": 250, "message": {"en": EN, "de": DE}},
    ],
    "invalid": [
        {"schema": "semio.hub.rate-limit-refusal/v1", "code": "rate-limited", "class": "directory-command", "retryAfterMs": 0, "message": {"en": EN, "de": DE}},
        {"schema": "semio.hub.rate-limit-refusal/v1", "code": "rate-limited", "class": "uploads", "retryAfterMs": 100, "message": {"en": EN, "de": DE}},
        {"schema": "semio.hub.rate-limit-refusal/v1", "code": "rate-limited", "class": "directory-command", "retryAfterMs": 100, "message": {"en": EN}},
        {"schema": "semio.hub.rate-limit-refusal/v1", "code": "unavailable", "class": "directory-command", "retryAfterMs": 100, "message": {"en": EN, "de": DE}},
        {"schema": "semio.hub.rate-limit-refusal/v1", "code": "rate-limited", "class": "directory-command", "message": {"en": EN, "de": DE}},
        {"schema": "semio.hub.rate-limit-refusal/v1", "code": "rate-limited", "class": "directory-command", "retryAfterMs": 100, "message": {"en": EN, "de": DE}, "route": "/directory/commands"},
    ],
}

RATE_OLD = """/// 📐️ One class's admitted burst and sustained cost."""
RATE_NEW = f"""/// 🏷️ `RateLimitRefusalV1.schema`.
pub const RATE_LIMIT_REFUSAL_SCHEMA: &str = "semio.hub.rate-limit-refusal/v1";

/// 🗣️ `RateLimitRefusalMessageV1`: the notice of a rate-limited request, en and de.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub struct RateLimitRefusalMessageV1 {{
    pub en: &'static str,
    pub de: &'static str,
}}

/// 🗣️ The one `RateLimitRefusalMessageV1` every refusal carries.
pub const RATE_LIMIT_REFUSAL_MESSAGE: RateLimitRefusalMessageV1 = RateLimitRefusalMessageV1 {{ en: "{EN}", de: "{DE}" }};

/// 🚦️ `RateLimitRefusalV1`: the body of the `429` a non-auth rate-limited route family answers when its bucket is empty — the
/// class, the wait until one request is admitted again, the notice; the client resends the same request after the wait.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitRefusalV1 {{
    pub schema: &'static str,
    pub code: &'static str,
    pub class: &'static str,
    pub retry_after_ms: u64,
    pub message: RateLimitRefusalMessageV1,
}}

impl RateLimitRefusalV1 {{
    pub fn new(class: RateLimitClassV1, retry_after_ms: u64) -> Self {{
        Self {{ schema: RATE_LIMIT_REFUSAL_SCHEMA, code: "rate-limited", class: class.as_str(), retry_after_ms: retry_after_ms.max(1), message: RATE_LIMIT_REFUSAL_MESSAGE }}
    }}
}}

/// 📐️ One class's admitted burst and sustained cost."""

AUTH_LAW = r'''
/// 🚦️ The Rust rate-limit refusal is exactly `RateLimitRefusalV1`: every non-auth class's body validates against the declared
/// schema (owned draft-07 validator), its notice is the declared en + de const, and the fixture's valid bodies validate while
/// every near miss (no wait, an unknown class, a missing language, another code, no `retryAfterMs`, an extra field) does not.
#[test]
fn the_rate_limit_refusal_is_the_declared_schema_body() {
    let document: serde_json::Value = serde_json::from_str(SCHEMA_MODULE).expect("hub.auth schema module");
    let validator = structural("RateLimitRefusalV1");
    assert_eq!(serde_json::to_value(RATE_LIMIT_REFUSAL_MESSAGE).unwrap(), document["$defs"]["RateLimitRefusalMessageV1"]["const"]);
    for class in RATE_LIMIT_CLASSES.into_iter().filter(|class| *class != RateLimitClassV1::Auth) {
        let body = serde_json::to_value(RateLimitRefusalV1::new(class, u64::from(class.policy().cost_ms))).unwrap();
        assert!(validator.is_valid_json(&body.to_string()), "{} refusal {body} validates", class.as_str());
    }
    assert_eq!(RateLimitRefusalV1::new(RateLimitClassV1::DirectoryCommand, 0).retry_after_ms, 1, "a refusal always names a positive wait");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚦️rate-limit-refusal-v1/🔣️.json")).unwrap();
    for body in fixture["valid"].as_array().unwrap() {
        assert!(validator.is_valid_json(&body.to_string()), "valid fixture body {body}");
        let class = RATE_LIMIT_CLASSES.into_iter().find(|class| class.as_str() == body["class"].as_str().unwrap()).unwrap();
        assert_eq!(&serde_json::to_value(RateLimitRefusalV1::new(class, u64::from(class.policy().cost_ms))).unwrap(), body, "the Rust body for {} is the fixture's", class.as_str());
    }
    for body in fixture["invalid"].as_array().unwrap() {
        assert!(!validator.is_valid_json(&body.to_string()), "near miss {body} must not validate");
    }
}
'''

MIDDLEWARE_OLD = """            _ => {
                state.note("server.rate-limit", TraceOutcome::Refused, class.as_str());
                let mut response = StatusCode::TOO_MANY_REQUESTS.into_response();
                if let Ok(value) = axum::http::HeaderValue::from_str(&RateLimitDecisionV1::Refused { retry_after_ms }.retry_after_secs().to_string()) {
                    response.headers_mut().insert(axum::http::header::RETRY_AFTER, value);
                }
                response
            }"""
MIDDLEWARE_NEW = """            _ => {
                state.note("server.rate-limit", TraceOutcome::Refused, class.as_str());
                let mut response = (StatusCode::TOO_MANY_REQUESTS, Json(semio_hub::auth::rate_limit::RateLimitRefusalV1::new(class, retry_after_ms))).into_response();
                response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
                if let Ok(value) = axum::http::HeaderValue::from_str(&RateLimitDecisionV1::Refused { retry_after_ms }.retry_after_secs().to_string()) {
                    response.headers_mut().insert(axum::http::header::RETRY_AFTER, value);
                }
                response
            }"""

BIN_ANCHOR = "/// 🔬️ Signing out revokes the credential-minted session durably:"
BIN_LAW = r'''/// 🚦️ A directory-command burst beyond its bucket is refused with the typed `RateLimitRefusalV1` body (class, a positive wait,
/// the en + de notice), `retry-after` and `no-store` — never an empty 429 a client cannot read.
#[tokio::test]
async fn a_directory_command_burst_is_refused_with_a_typed_rate_limit_body() {
    let (state, _clock) = credential_sign_in_state().await;
    let addr = spawn_server(state).await;
    let burst = RateLimitClassV1::DirectoryCommand.policy().burst;
    for attempt in 0..burst {
        assert_ne!(raw_http_request(addr, "POST", "/directory/commands", &[("content-type", "application/json")], b"{}").await.status, 429, "attempt {attempt} is within the burst");
    }
    let refused = raw_http_request(addr, "POST", "/directory/commands", &[("content-type", "application/json")], b"{}").await;
    assert_eq!(refused.status, 429);
    let headers = refused.headers.to_lowercase();
    assert!(headers.contains("retry-after:") && headers.contains("no-store"), "{}", refused.headers);
    let body = json_body(&refused);
    assert_eq!(body["schema"], semio_hub::auth::rate_limit::RATE_LIMIT_REFUSAL_SCHEMA);
    assert_eq!(body["code"], "rate-limited");
    assert_eq!(body["class"], "directory-command");
    assert!(body["retryAfterMs"].as_u64().is_some_and(|milliseconds| milliseconds > 0));
    assert_eq!(body["message"], serde_json::to_value(semio_hub::auth::rate_limit::RATE_LIMIT_REFUSAL_MESSAGE).unwrap());
}

'''

ORACLE_OLD = """  it("the fixture's oversized vector exceeds every declared body limit", () => {"""
ORACLE_NEW = """  it("a rate-limited request's body is a valid RateLimitRefusalV1 for every non-auth class, and no near miss is", () => {
    const auth = read(hubRoot, "🔐️auth", "🧬️schema", "🔣️.json");
    const refusals = read(hubRoot, "🔐️auth", "🧫️fixtures", "🚦️rate-limit-refusal-v1", "🔣️.json");
    const validate = compileDef(auth, "RateLimitRefusalV1");
    expect(new Set(refusals.valid.map((body: any) => body.class))).toEqual(new Set(auth.$defs.AuthRateLimitClassV1.enum.filter((name: string) => name !== "auth")));
    for (const body of refusals.valid) expect(validate(body), JSON.stringify(validate.errors)).toBe(true);
    expect(refusals.invalid.length).toBeGreaterThanOrEqual(6);
    for (const body of refusals.invalid) expect(validate(body), JSON.stringify(body)).toBe(false);
  });

  it("the fixture's oversized vector exceeds every declared body limit", () => {"""


def main() -> int:
    dry = "--dry-run" in sys.argv
    texts = {path: path.read_text(encoding="utf-8") for path in [SCHEMA, RATE, AUTH_TESTS, BOOTSTRAP, BIN_TESTS, ORACLE, TAXONOMY]}
    pending, problems = [], 0

    def edit(path, old, new, label):
        nonlocal problems
        if new in texts[path]:
            print(f"applied already: {label}")
            return
        if texts[path].count(old) != 1:
            print(f"PROBLEM ({texts[path].count(old)} matches): {label}")
            problems += 1
            return
        texts[path] = texts[path].replace(old, new, 1)
        pending.append(label)
        print(f"{'would apply' if dry else 'apply'}: {label}")

    schema = json.loads(texts[SCHEMA])
    if "RateLimitRefusalV1" not in schema["$defs"]:
        schema["$defs"].update(SCHEMA_DEFS)
        texts[SCHEMA] = json.dumps(schema, indent=2, ensure_ascii=False) + "\n"
        pending.append("schema")
        print(f"{'would add' if dry else 'add'}: schema RateLimitRefusalV1 + RateLimitRefusalMessageV1")
    edit(RATE, RATE_OLD, RATE_NEW, "rate-limit: RateLimitRefusalV1")
    edit(AUTH_TESTS, "use super::rate_limit::{HubRateLimiterV1, RateLimitClassV1, RateLimitClockV1, RateLimitDecisionV1, RateLimitSubjectV1, RATE_LIMIT_CLASSES};", "use super::rate_limit::{HubRateLimiterV1, RateLimitClassV1, RateLimitClockV1, RateLimitDecisionV1, RateLimitRefusalV1, RateLimitSubjectV1, RATE_LIMIT_CLASSES, RATE_LIMIT_REFUSAL_MESSAGE};", "auth unit imports")
    if "fn the_rate_limit_refusal_is_the_declared_schema_body" not in texts[AUTH_TESTS]:
        edit(AUTH_TESTS, "\nconst SCHEMA_MODULE: &str", AUTH_LAW + "\nconst SCHEMA_MODULE: &str", "auth unit law")
    edit(BOOTSTRAP, MIDDLEWARE_OLD, MIDDLEWARE_NEW, "middleware typed 429")
    if "fn a_directory_command_burst_is_refused_with_a_typed_rate_limit_body" not in texts[BIN_TESTS]:
        edit(BIN_TESTS, BIN_ANCHOR, BIN_LAW + BIN_ANCHOR, "bin law")
    edit(ORACLE, ORACLE_OLD, ORACLE_NEW, "TS oracle case")
    taxonomy = texts[TAXONOMY]
    if '"🚦️rate-limit-refusal-v1"' not in taxonomy:
        edit(TAXONOMY, '        "⏳️transient-apply-refusal-v1"\n', '        "⏳️transient-apply-refusal-v1",\n        "🚦️rate-limit-refusal-v1"\n', "taxonomy fixture member")
    fixture_text = json.dumps(FIXTURE_DOC, indent=2, ensure_ascii=False) + "\n"
    fixture_current = FIXTURE.read_text(encoding="utf-8") if FIXTURE.exists() else None
    if fixture_current != fixture_text:
        pending.append("fixture")
        print(f"{'would write' if dry else 'write'}: fixture {FIXTURE.parent.name}")
    if problems:
        print(f"{problems} problems — nothing written")
        return 1
    if not dry:
        for path, text in texts.items():
            if text != path.read_text(encoding="utf-8"):
                path.write_text(text, encoding="utf-8")
        if fixture_current != fixture_text:
            FIXTURE.parent.mkdir(parents=True, exist_ok=True)
            FIXTURE.write_text(fixture_text, encoding="utf-8")
    print(f"{'dry run: ' if dry else ''}{len(pending)} pending")
    return 0


if __name__ == "__main__":
    sys.exit(main())
