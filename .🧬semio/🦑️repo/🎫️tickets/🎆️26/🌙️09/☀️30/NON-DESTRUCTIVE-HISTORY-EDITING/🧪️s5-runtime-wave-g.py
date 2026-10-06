"""🫧️ Wave G (tests only; K3 law, first run 06:13 `test-plugin-laws-6.txt`: 3 passed / 1 failed):

`the_snapshot_mutation_replaces_the_whole_root_and_inverts_to_the_base` ended on a clause no contract carries — "unknown
members are refused". `transient_root!` parses with `JsonMemberPolicy::Reject`, which is the authority over REPEATED member
names (`🎒️pack/🔤️json/🦀️.rs` ≈ :1806 `DuplicateMember`); a member the root does not declare is the value derive's matter
and it reads past it, for every root alike. Every assertion before the clause passed in that run. The clause now states
what the macro promises: a repeated member is refused.

Loaded by `🧪️s5-runtime-land.py`.
"""

OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
LAW = f"{OSM}/🔌️plugin/🪟️window/🫧️transient/🧪️tests/🧪️transient-root/🦀️.rs"

LAW_RS = [
    (
        """/// ⚖️ LAW: the one mutation's wire is `{"kind":"snapshot","transient":…}` in text and binary alike, it parses back, installs
/// its root over any base, and its inverse restores that base.
""",
        """/// ⚖️ LAW: the one mutation's wire is `{"kind":"snapshot","transient":…}` in text and binary alike, it parses back, installs
/// its root over any base, and its inverse restores that base; a wire naming one member twice is refused.
""",
    ),
    (
        """    assert!(ProbeTransientMutation::parse_op(r#"{"kind":"snapshot","transient":{"label":"x","count":1,"extra":0}}"#).is_err(), "unknown members are refused");
""",
        """    assert!(ProbeTransientMutation::parse_op(r#"{"kind":"snapshot","transient":{"label":"x","count":1,"count":2}}"#).is_err(), "a repeated member is refused");
""",
    ),
]


def files(_root):
    return {LAW: LAW_RS}
