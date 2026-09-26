#!/usr/bin/env python3
"""🔌️ WG7 S12-1b follow-up: a link shortage shorter than the bound must relink (measured run s12e: a 45 s cut expired A's link).

Defect: the retry schedule ignored the bound, so after the capped attempt at since+31.5 s the next one was due at since+61.5 s,
while `Tick` expired the link at since+60 s — every cut longer than ~31.5 s expired although the hub was back long before the
bound. New rule, one for the kernel, the TS twin, the schema and the fixture:

- a retry is never scheduled past the bound: `retryAtMs = min(nowMs + backoffMs, sinceMs + shortageBoundMs)`, so the last
  attempt of every shortage runs AT the bound;
- only a FAILED attempt at or past the bound expires the link;
- `Tick` expires only at the hard ceiling `sinceMs + shortageBoundMs + reconnectMaxMs` (an attempt that never answers);
- `expiresAtMs` is that ceiling.

Guest-linked kernel file: apply ONLY after the coordinator lifts rule 20. Default is a dry run printing unified diffs;
`--apply` writes. Every replacement asserts its anchor occurs exactly once.
"""

import difflib
import json
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
KERNEL = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs"
TWIN = ROOT / "🧰️framework/🛍️products/💻️os/🟦️.ts"
SCHEMA = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧬️schema/document-link-shortage/🔣️.json"
FIXTURE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/document-link-shortage-v1/🔣️.json"
RUST_LAW = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️document-link-shortage/🦀️.rs"

KERNEL_EDITS = [
    (
        """/// @emoji 🔌️ The capped doubling reconnect and the longest shortage one hub document's link rides out
/// (`🧬️schema/document-link-shortage/🔣️.json` `$defs/Policy`). The bound is twice the backoff cap, so at
/// least two capped reconnect attempts fall inside it.""",
        """/// @emoji 🔌️ The capped doubling reconnect and the longest shortage one hub document's link rides out
/// (`🧬️schema/document-link-shortage/🔣️.json` `$defs/Policy`). The bound is twice the backoff cap, so at
/// least two capped reconnect attempts fall inside it, and no retry is ever scheduled past it: the last
/// attempt of every shortage runs AT the bound. An attempt that never answers ends the link at the
/// ceiling, the bound plus one capped backoff.""",
    ),
    (
        """            (Self::Linked, DocumentLinkEvent::Failed { now_ms }) => Self::Unlinked { since_ms: now_ms, backoff_ms: policy.reconnect_min_ms, retry_at_ms: now_ms.saturating_add(policy.reconnect_min_ms) },
            (Self::Unlinked { since_ms, backoff_ms, .. }, DocumentLinkEvent::Failed { now_ms }) => {
                if now_ms.saturating_sub(since_ms) >= policy.shortage_bound_ms {
                    return Self::Expired { since_ms, at_ms: now_ms };
                }
                let backoff_ms = backoff_ms.saturating_mul(2).clamp(policy.reconnect_min_ms, policy.reconnect_max_ms);
                Self::Unlinked { since_ms, backoff_ms, retry_at_ms: now_ms.saturating_add(backoff_ms) }
            }
            (Self::Unlinked { since_ms, .. }, DocumentLinkEvent::Tick { now_ms }) if now_ms.saturating_sub(since_ms) >= policy.shortage_bound_ms => Self::Expired { since_ms, at_ms: now_ms },""",
        """            (Self::Linked, DocumentLinkEvent::Failed { now_ms }) => Self::Unlinked { since_ms: now_ms, backoff_ms: policy.reconnect_min_ms, retry_at_ms: policy.retry_at(now_ms, now_ms, policy.reconnect_min_ms) },
            (Self::Unlinked { since_ms, backoff_ms, .. }, DocumentLinkEvent::Failed { now_ms }) => {
                if now_ms.saturating_sub(since_ms) >= policy.shortage_bound_ms {
                    return Self::Expired { since_ms, at_ms: now_ms };
                }
                let backoff_ms = backoff_ms.saturating_mul(2).clamp(policy.reconnect_min_ms, policy.reconnect_max_ms);
                Self::Unlinked { since_ms, backoff_ms, retry_at_ms: policy.retry_at(since_ms, now_ms, backoff_ms) }
            }
            (Self::Unlinked { since_ms, .. }, DocumentLinkEvent::Tick { now_ms }) if now_ms >= policy.ceiling_at(since_ms) => Self::Expired { since_ms, at_ms: now_ms },""",
    ),
    (
        """    /// ⏳️ When an unlinked link expires unless it relinks first.
    pub fn expires_at_ms(self, policy: &DocumentLinkShortagePolicy) -> Option<u64> {
        match self {
            Self::Unlinked { since_ms, .. } => Some(since_ms.saturating_add(policy.shortage_bound_ms)),
            _ => None,
        }
    }""",
        """    /// ⏳️ The latest instant an unlinked link can live: its ceiling. A failed attempt at the bound ends it earlier.
    pub fn expires_at_ms(self, policy: &DocumentLinkShortagePolicy) -> Option<u64> {
        match self {
            Self::Unlinked { since_ms, .. } => Some(policy.ceiling_at(since_ms)),
            _ => None,
        }
    }""",
    ),
    (
        """/// 🔌️ The policy every shell drives""",
        """impl DocumentLinkShortagePolicy {
    /// 🔁️ When the next attempt of a shortage that began at `since_ms` is due: one backoff after `now_ms`, never past the bound.
    pub fn retry_at(&self, since_ms: u64, now_ms: u64, backoff_ms: u64) -> u64 {
        now_ms.saturating_add(backoff_ms).min(since_ms.saturating_add(self.shortage_bound_ms))
    }

    /// ⏳️ The instant a shortage that began at `since_ms` ends although no attempt answered: the bound plus one capped backoff.
    pub fn ceiling_at(&self, since_ms: u64) -> u64 {
        since_ms.saturating_add(self.shortage_bound_ms).saturating_add(self.reconnect_max_ms)
    }
}

/// 🔌️ The policy every shell drives""",
    ),
]

TWIN_EDITS = [
    (
        """    if (link.kind === "linked") return { kind: "unlinked", sinceMs: event.nowMs, backoffMs: policy.reconnectMinMs, retryAtMs: event.nowMs + policy.reconnectMinMs };
    if (event.nowMs - link.sinceMs >= policy.shortageBoundMs) return { kind: "expired", sinceMs: link.sinceMs, atMs: event.nowMs };
    const backoffMs = Math.min(Math.max(link.backoffMs * 2, policy.reconnectMinMs), policy.reconnectMaxMs);
    return { kind: "unlinked", sinceMs: link.sinceMs, backoffMs, retryAtMs: event.nowMs + backoffMs };
  }
  return link.kind === "unlinked" && event.nowMs - link.sinceMs >= policy.shortageBoundMs ? { kind: "expired", sinceMs: link.sinceMs, atMs: event.nowMs } : link;
}""",
        """    if (link.kind === "linked") return { kind: "unlinked", sinceMs: event.nowMs, backoffMs: policy.reconnectMinMs, retryAtMs: documentLinkRetryAtMs(event.nowMs, event.nowMs, policy.reconnectMinMs, policy) };
    if (event.nowMs - link.sinceMs >= policy.shortageBoundMs) return { kind: "expired", sinceMs: link.sinceMs, atMs: event.nowMs };
    const backoffMs = Math.min(Math.max(link.backoffMs * 2, policy.reconnectMinMs), policy.reconnectMaxMs);
    return { kind: "unlinked", sinceMs: link.sinceMs, backoffMs, retryAtMs: documentLinkRetryAtMs(link.sinceMs, event.nowMs, backoffMs, policy) };
  }
  return link.kind === "unlinked" && event.nowMs >= documentLinkCeilingAtMs(link.sinceMs, policy) ? { kind: "expired", sinceMs: link.sinceMs, atMs: event.nowMs } : link;
}

/** 🔁️ When the next attempt of a shortage that began at `sinceMs` is due: one backoff after `nowMs`, never past the bound — twin of
 * the kernel's `DocumentLinkShortagePolicy::retry_at`. */
export function documentLinkRetryAtMs(sinceMs: number, nowMs: number, backoffMs: number, policy: DocumentLinkShortagePolicy = DOCUMENT_LINK_SHORTAGE_POLICY): number {
  return Math.min(nowMs + backoffMs, sinceMs + policy.shortageBoundMs);
}

/** ⏳️ The instant a shortage that began at `sinceMs` ends although no attempt answered: the bound plus one capped backoff — twin of
 * the kernel's `DocumentLinkShortagePolicy::ceiling_at`. */
export function documentLinkCeilingAtMs(sinceMs: number, policy: DocumentLinkShortagePolicy = DOCUMENT_LINK_SHORTAGE_POLICY): number {
  return sinceMs + policy.shortageBoundMs + policy.reconnectMaxMs;
}""",
    ),
    (
        """/** ⏳️ When an unlinked link expires unless it relinks first. */
export function documentLinkExpiresAtMs(link: DocumentLink, policy: DocumentLinkShortagePolicy = DOCUMENT_LINK_SHORTAGE_POLICY): number | undefined {
  return link.kind === "unlinked" ? link.sinceMs + policy.shortageBoundMs : undefined;
}""",
        """/** ⏳️ The latest instant an unlinked link can live: its ceiling. A failed attempt at the bound ends it earlier. */
export function documentLinkExpiresAtMs(link: DocumentLink, policy: DocumentLinkShortagePolicy = DOCUMENT_LINK_SHORTAGE_POLICY): number | undefined {
  return link.kind === "unlinked" ? documentLinkCeilingAtMs(link.sinceMs, policy) : undefined;
}""",
    ),
]

RUST_LAW_EDITS = [
    (
        """    let capped = DocumentLink::Unlinked { since_ms: 0, backoff_ms: DOCUMENT_LINK_SHORTAGE_POLICY.reconnect_max_ms, retry_at_ms: 70_000 };
    assert_eq!(capped.next_deadline_ms(&DOCUMENT_LINK_SHORTAGE_POLICY), Some(DOCUMENT_LINK_SHORTAGE_POLICY.shortage_bound_ms), "expiry wins over a later retry");""",
        """    let hung = DocumentLink::Unlinked { since_ms: 0, backoff_ms: DOCUMENT_LINK_SHORTAGE_POLICY.reconnect_max_ms, retry_at_ms: u64::MAX };
    assert_eq!(hung.next_deadline_ms(&DOCUMENT_LINK_SHORTAGE_POLICY), Some(DOCUMENT_LINK_SHORTAGE_POLICY.shortage_bound_ms + DOCUMENT_LINK_SHORTAGE_POLICY.reconnect_max_ms), "the ceiling wins over a retry that never comes");
    let mut walked = DocumentLink::Linked.apply(&DOCUMENT_LINK_SHORTAGE_POLICY, DocumentLinkEvent::Failed { now_ms: 0 });
    while let Some(retry_at_ms) = walked.retry_at_ms().filter(|at| *at < DOCUMENT_LINK_SHORTAGE_POLICY.shortage_bound_ms) {
        walked = walked.apply(&DOCUMENT_LINK_SHORTAGE_POLICY, DocumentLinkEvent::Failed { now_ms: retry_at_ms });
    }
    assert_eq!(walked.retry_at_ms(), Some(DOCUMENT_LINK_SHORTAGE_POLICY.shortage_bound_ms), "the last attempt of every shortage runs at the bound");""",
    ),
]

SCHEMA_OLD = "a shortage longer than the bound expires the link (long offline periods are refused, never silently absorbed)"
SCHEMA_NEW = "no retry is scheduled past the bound, so the last attempt of every shortage runs at the bound; a failed attempt at or past the bound expires the link, and an attempt that never answers expires it at the ceiling (bound plus one capped backoff) — long offline periods are refused, never silently absorbed"


def policy_of(fixture):
    return fixture["policy"]


def model(policy, link, event):
    lo, hi, bound = policy["reconnectMinMs"], policy["reconnectMaxMs"], policy["shortageBoundMs"]
    now = event["nowMs"]
    if link["kind"] in ("expired", "revoked"):
        return link
    if event["kind"] == "refused":
        return {"kind": "revoked", "atMs": now}
    if event["kind"] == "restored":
        return {"kind": "linked"}
    if event["kind"] == "failed":
        if link["kind"] == "linked":
            return {"kind": "unlinked", "sinceMs": now, "backoffMs": lo, "retryAtMs": min(now + lo, now + bound)}
        since = link["sinceMs"]
        if now - since >= bound:
            return {"kind": "expired", "sinceMs": since, "atMs": now}
        backoff = min(max(link["backoffMs"] * 2, lo), hi)
        return {"kind": "unlinked", "sinceMs": since, "backoffMs": backoff, "retryAtMs": min(now + backoff, since + bound)}
    if link["kind"] == "unlinked" and now >= link["sinceMs"] + bound + hi:
        return {"kind": "expired", "sinceMs": link["sinceMs"], "atMs": now}
    return link


def expect(policy, link):
    status = {"linked": "linked", "unlinked": "reconnecting", "expired": "link-expired", "revoked": "access-revoked"}[link["kind"]]
    out = {"state": link, "status": status, "admitsLocalEdits": link["kind"] in ("linked", "unlinked")}
    if link["kind"] == "unlinked":
        out["expiresAtMs"] = link["sinceMs"] + policy["shortageBoundMs"] + policy["reconnectMaxMs"]
    return out


def vector(policy, vector_id, opened_at, events):
    link = {"kind": "unlinked", "sinceMs": opened_at, "backoffMs": 0, "retryAtMs": opened_at}
    steps = []
    for kind, now in events:
        event = {"kind": kind, "nowMs": now}
        link = model(policy, link, event)
        steps.append({"event": event, "expect": expect(policy, link)})
    return {"id": vector_id, "openedAtMs": opened_at, "steps": steps}


def fixture_text(fixture):
    policy = policy_of(fixture)
    vectors = []
    for existing in fixture["vectors"]:
        events = [(step["event"]["kind"], step["event"]["nowMs"]) for step in existing["steps"]]
        if existing["id"] == "the-backoff-doubles-to-its-cap-and-an-unlinked-open-expires":
            existing_id = "the-backoff-doubles-to-its-cap-and-the-last-attempt-runs-at-the-bound"
            events = [event for event in events if event[0] != "tick"] + [("tick", 59999), ("tick", 60000), ("failed", 60000)]
            vectors.append(vector(policy, existing_id, existing["openedAtMs"], events))
            continue
        vectors.append(vector(policy, existing["id"], existing["openedAtMs"], events))
    vectors.insert(1, vector(policy, "a-cut-shorter-than-the-bound-relinks-at-the-bound", 0, [("restored", 100), ("failed", 10000), ("failed", 10500), ("failed", 11500), ("failed", 13500), ("failed", 17500), ("failed", 25500), ("failed", 41500), ("tick", 69999), ("tick", 70000), ("restored", 70200)]))
    vectors.insert(3, vector(policy, "an-attempt-that-never-answers-expires-at-the-ceiling", 0, [("failed", 0), ("failed", 500), ("failed", 1500), ("failed", 3500), ("failed", 7500), ("failed", 15500), ("failed", 31500), ("tick", 60000), ("tick", 89999), ("tick", 90000), ("restored", 90100)]))
    return json.dumps({**fixture, "vectors": vectors}, indent=2, ensure_ascii=False) + "\n"


def replaced(path, source, edits):
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}× in {path}: {old[:80]!r}")
        source = source.replace(old, new)
    return source


def main():
    apply = "--apply" in sys.argv
    plans = []
    for path, edits in ((KERNEL, KERNEL_EDITS), (TWIN, TWIN_EDITS), (RUST_LAW, RUST_LAW_EDITS)):
        before = path.read_text(encoding="utf-8")
        plans.append((path, before, replaced(path, before, edits)))
    schema_before = SCHEMA.read_text(encoding="utf-8")
    plans.append((SCHEMA, schema_before, replaced(SCHEMA, schema_before, [(SCHEMA_OLD, SCHEMA_NEW)])))
    fixture_before = FIXTURE.read_text(encoding="utf-8")
    plans.append((FIXTURE, fixture_before, fixture_text(json.loads(fixture_before))))
    for path, before, after in plans:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), str(path.relative_to(ROOT)), str(path.relative_to(ROOT)) + " (patched)", n=1))
        if apply:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'APPLIED' if apply else 'DRY RUN'}: {len(plans)} files")


if __name__ == "__main__":
    main()
