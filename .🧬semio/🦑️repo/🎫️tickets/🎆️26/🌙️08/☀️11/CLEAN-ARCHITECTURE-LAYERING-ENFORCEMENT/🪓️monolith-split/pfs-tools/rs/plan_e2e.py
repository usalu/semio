SEE = ["//!", "//! @see ../🦀️.rs — the end to end suite that mounts this module", "//! @see ../../../🧫️fixtures/📚️catalog/🔣️.json — the catalog played"]
SPEC = dict(
    target="e2e",
    hub_keep_ranges=[(1, 29)],
    hub_doc_after=8,
    hub_doc=[
        "//!",
        "//! The suite is split into the child folders next to this file: `🧰️harness` (scratch, the booted server, the wire, presence and answers), `🎮️learner-journey`,",
        "//! `🏆️challenge-sheets`, `🛠️operator` and `🛡️edge` (the tests); this file mounts them and is the crate root of the `end_to_end` test.",
    ],
    children=[
        dict(mod="harness", folder="🧰️harness", ranges=[(30, 409)],
             doc=["//! 🧰️ The harness of the proctor end to end suite: a scratch directory per test, the server booted on an ephemeral port, the wire with the §9a encoding, the websocket client of the presence rooms, and the builders of answers and runs."] + SEE),
        dict(mod="learner_journey", glob=False, folder="🎮️learner-journey", ranges=[(410, 777)],
             doc=["//! 🎮️ The proctor end to end, the learner journey: a learner plays through the real API and the facts survive a restart, the API serves the CDN site across origins, learners share presence and cursors, a production proctor opens presence to the site origin only and learners see what the others think live."] + SEE),
        dict(mod="challenge_sheets", glob=False, folder="🏆️challenge-sheets", ranges=[(778, 1165)],
             doc=["//! 🏆️ The proctor end to end, the challenge sheets: every challenge deals its own sheet and earns its own points, expert and hard runs clock and score from the instants the device claims, an easy run questions far off cards until it closes and switching the challenge ends in runs exhausted."] + SEE),
        dict(mod="operator", folder="🛠️operator", ranges=[(1166, 1380)],
             doc=["//! 🛠️ The proctor end to end, the operator: the built binary probes, backs up while serving and restores elsewhere, erases one learner on request and prunes the registrations nobody played under."] + SEE),
        dict(mod="edge", folder="🛡️edge", ranges=[(1381, 1771)],
             doc=["//! 🛡️ The proctor end to end, the edge: the gate of a proctor behind its terminating proxy — enrollment keys, refused ids, body limits, per-address allowances, sign-up counting, its own routes only, socket limits and trickling bodies."] + SEE),
    ],
)
