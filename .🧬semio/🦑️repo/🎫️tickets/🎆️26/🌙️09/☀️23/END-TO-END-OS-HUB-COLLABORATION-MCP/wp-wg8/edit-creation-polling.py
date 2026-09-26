#!/usr/bin/env python3
"""WG8 s12: the native creation door follows a creation while the hub answers (shared `🌱️creation-polling` contract, S15's
React fix) instead of concluding `indeterminate` after a fixed 120 s — cross-shell run 9: the hub finished a 2d.puzzle
creation after ~275 s while the native door had said "unknown" at 120 s."""
import pathlib

ELEMENTS = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements")
HUB = ELEMENTS / "🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs"
HUB_TESTS = ELEMENTS / "🔗️HubConnection/🧪️tests/🔬️wgpu-unit/🦀️.rs"
SHELL = ELEMENTS / "🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"


def edit(path, pairs):
    text = path.read_text()
    for old, new in pairs:
        assert text.count(old) == 1, (path.name, old[:100])
        text = text.replace(old, new)
    path.write_text(text)


edit(HUB, [
    (
        "/// cancel was asked and sent, and the opening of its result. `deadline_at_ms` bounds the whole\n/// operation; past it the outcome is `Indeterminate`, never silently `Failed`.\n",
        "/// cancel was asked and sent, and the opening of its result. The door follows the creation for as\n/// long as the hub answers ([`space_artifact_creation_unreachable`] since `last_answered_at_ms`), polling\n/// with the shared backoff (`polls`); only a hub silent past the bound makes it `Indeterminate`.\n",
    ),
    (
        "    pub deadline_at_ms: u64,\n    pub next_poll_at_ms: u64,\n}\n\n/// 🌱️ The open space's artifact-creation door",
        "    pub last_answered_at_ms: u64,\n    pub polls: u32,\n    pub next_poll_at_ms: u64,\n}\n\n/// 🌱️ The open space's artifact-creation door",
    ),
    (
        "/// ⏱️ The whole-operation bound and the poll cadence — the browser worker's\n/// `SPACE_ARTIFACT_CREATION_DEADLINE_MS` / `SPACE_ARTIFACT_CREATION_POLL_MS` (`🏪️store/👷️worker/🟦️.ts`).\npub const HUB_ARTIFACT_CREATION_DEADLINE_MS: u64 = 120_000;\npub const HUB_ARTIFACT_CREATION_POLL_MS: u64 = 100;\n",
        "/// ⏱️ How a client follows one space artifact creation — the shared contract\n/// `🏪️store/👷️worker/🌱️creation-polling/🔣️.json`, read by React's `spaceArtifactCreationPollDelayV1` /\n/// `spaceArtifactCreationUnreachableV1` and by this door.\n#[derive(Debug, Deserialize)]\n#[serde(rename_all = \"camelCase\")]\npub struct SpaceArtifactCreationPollingV1 {\n    pub poll_initial_ms: u64,\n    pub poll_max_ms: u64,\n    pub unreachable_bound_ms: u64,\n}\n\n/// 📜️ The shared polling contract, parsed once from the file both renderers read.\npub static SPACE_ARTIFACT_CREATION_POLLING_V1: std::sync::LazyLock<SpaceArtifactCreationPollingV1> =\n    std::sync::LazyLock::new(|| serde_json::from_str(include_str!(\"../../../../../../🏪️store/👷️worker/🌱️creation-polling/🔣️.json\")).expect(\"the shared creation-polling contract parses\"));\n\n/// ⏳️ The wait before status poll `attempt` (0-based): `pollInitialMs` doubling up to `pollMaxMs`.\npub fn space_artifact_creation_poll_delay_ms(attempt: u32) -> u64 {\n    let contract = &*SPACE_ARTIFACT_CREATION_POLLING_V1;\n    contract.poll_max_ms.min(contract.poll_initial_ms.saturating_mul(1u64 << attempt.min(30)))\n}\n\n/// 🛑️ Whether a creation must be concluded `Indeterminate`: the hub has not answered for the contract's bound.\npub fn space_artifact_creation_unreachable(last_answered_at_ms: u64, now_ms: u64) -> bool {\n    now_ms.saturating_sub(last_answered_at_ms) >= SPACE_ARTIFACT_CREATION_POLLING_V1.unreachable_bound_ms\n}\n",
    ),
])

edit(HUB_TESTS, [
    ("cancel_sent: false, ready, opening, deadline_at_ms: u64::MAX, next_poll_at_ms: 0 });", "cancel_sent: false, ready, opening, last_answered_at_ms: 0, polls: 0, next_poll_at_ms: 0 });"),
    ("ready: None, opening: HubArtifactOpening::Idle, deadline_at_ms: u64::MAX, next_poll_at_ms: 0 });", "ready: None, opening: HubArtifactOpening::Idle, last_answered_at_ms: 0, polls: 0, next_poll_at_ms: 0 });"),
])
HUB_TESTS.write_text(HUB_TESTS.read_text().rstrip("\n") + r'''

/// ⏱️ The door follows a creation exactly as the browser does: every poll delay and the unreachable bound
/// come from the shared contract (`🏪️store/👷️worker/🌱️creation-polling/🔣️.json`), recomputed here from its
/// raw numbers — a slow hub that still answers is waited for, never concluded.
#[test]
fn the_creation_door_follows_the_shared_polling_contract() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🏪️store/👷️worker/🌱️creation-polling/🔣️.json")).expect("contract");
    let (initial, maximum, bound) = (contract["pollInitialMs"].as_u64().unwrap(), contract["pollMaxMs"].as_u64().unwrap(), contract["unreachableBoundMs"].as_u64().unwrap());
    let mut expected = initial;
    for attempt in 0..40 {
        assert_eq!(space_artifact_creation_poll_delay_ms(attempt), expected.min(maximum), "attempt {attempt}");
        expected = expected.saturating_mul(2).min(maximum);
    }
    assert!(!space_artifact_creation_unreachable(1_000, 1_000 + bound - 1), "an answer inside the bound keeps the creation followed");
    assert!(space_artifact_creation_unreachable(1_000, 1_000 + bound), "a hub silent for the whole bound is unreachable");
    assert!(!space_artifact_creation_unreachable(1_000, 500), "a clock read before the answer is never unreachable");
}
''')

edit(SHELL, [
    (
        "    FRAMEWORK_HUB_PANEL_ID, HUB_ARTIFACT_CREATION_DEADLINE_MS, HUB_ARTIFACT_CREATION_POLL_MS, ",
        "    FRAMEWORK_HUB_PANEL_ID, ",
    ),
    (
        "            deadline_at_ms: now_ms.saturating_add(HUB_ARTIFACT_CREATION_DEADLINE_MS),\n            next_poll_at_ms: now_ms,\n",
        "            last_answered_at_ms: now_ms,\n            polls: 0,\n            next_poll_at_ms: now_ms,\n",
    ),
    (
        "    /// cancellation, or a status poll no sooner than `HUB_ARTIFACT_CREATION_POLL_MS` after the last —\n    /// and opens the artifact once the receipt is `Ready`. Past `HUB_ARTIFACT_CREATION_DEADLINE_MS` the\n    /// outcome is `Indeterminate` (the hub may still finish it), never `Failed`.",
        "    /// cancellation, or a status poll after the shared backoff ([`crate::hub_connection::space_artifact_creation_poll_delay_ms`]) —\n    /// and opens the artifact once the receipt is `Ready`. The creation is followed for as long as the hub\n    /// answers; only a hub silent past the shared bound makes it `Indeterminate` (the hub may still finish\n    /// it), never `Failed` — React's worker follows the same contract (`🌱️creation-polling`).",
    ),
    (
        "        if now_ms >= operation.deadline_at_ms {\n            if let Some(current) = self.hub_workspace.creation.operation.as_mut() {\n                current.phase = SpaceArtifactCreationPhaseV1::Indeterminate;",
        "        if crate::hub_connection::space_artifact_creation_unreachable(operation.last_answered_at_ms, now_ms) {\n            if let Some(current) = self.hub_workspace.creation.operation.as_mut() {\n                current.phase = SpaceArtifactCreationPhaseV1::Indeterminate;",
    ),
    (
        "            current.next_poll_at_ms = Self::directory_now_ms().saturating_add(HUB_ARTIFACT_CREATION_POLL_MS);\n            match receipt {\n                Ok(status) if",
        "            let answered_at_ms = Self::directory_now_ms();\n            current.next_poll_at_ms = answered_at_ms.saturating_add(crate::hub_connection::space_artifact_creation_poll_delay_ms(current.polls));\n            current.polls = current.polls.saturating_add(1);\n            if receipt.is_ok() {\n                current.last_answered_at_ms = answered_at_ms;\n            }\n            match receipt {\n                Ok(status) if",
    ),
])
print("creation polling parity applied")
