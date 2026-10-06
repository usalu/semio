"""👉️ Wave A (design §22.2): the history wire status names the review's next problem.

`HistoryPatch.timeTravel.nextProblem?: { mutationId, store? }` — kernel type, schema of record, TS twin, wire fixture, and
the guest status that fills it; plus the kernel history notice row of the store's `history.unit-spans-documents` refusal
(design §22.1, S5-STORE's cross-document unit), so it exists before the store raises it. Loaded by `🧪️s5-runtime-land.py`.
"""

FW = "🧰️framework/🔨️modules"
KERNEL = f"{FW}/🎠️kernel"
TT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs"

KERNEL_RS = [
    (
        """/// ⏪️ The live history-edit session of one instance, as every host renders its band: identity and generation (every
/// `historyEdit*` verb may echo `generation`; a stale one is `timeTravel.stale`), stage, the edited mutation and its
/// label, replay progress, the report's worst severity, whether that report blocks finalizing, the last fault code, how
/// many drafts are accepted, what a review shows and whether a replay can be run again.
""",
        """/// ⛔️ The first mutation, in replay order, whose outcome blocks finalizing a history edit: its id and the composed member
/// store that holds it (`<slot>/<childId>`, design §12; absent for the document's own) — exactly the arguments of
/// `historyEditBegin`, so a host's "Next problem" control dispatches them and never computes them (design §22.2).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryTimeTravelProblem {
    pub mutation_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<String>,
}

/// ⏪️ The live history-edit session of one instance, as every host renders its band: identity and generation (every
/// `historyEdit*` verb may echo `generation`; a stale one is `timeTravel.stale`), stage, the edited mutation and its
/// label, replay progress, the report's worst severity, whether that report blocks finalizing, the last fault code, how
/// many drafts are accepted, what a review shows, whether a replay can be run again and the first mutation that blocks
/// finalizing.
""",
    ),
    (
        """    /// 🔁️ Whether `historyEditRerun` would start a replay now (a cancelled or faulted replay left drafts to replay).
    #[serde(default)]
    #[value(default)]
    pub rerunnable: bool,
}
""",
        """    /// 🔁️ Whether `historyEditRerun` would start a replay now (a cancelled or faulted replay left drafts to replay).
    #[serde(default)]
    #[value(default)]
    pub rerunnable: bool,
    /// 👉️ The first mutation whose replay outcome blocks finalizing; present exactly while `blocking`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub next_problem: Option<HistoryTimeTravelProblem>,
}
""",
    ),
]

KERNEL_RS += [
    (
        """/// (an open tool transaction, an exhausted or still replaying history, a whole-document load in flight, a history step whose
/// replay would leave errors), by fault code — every shell shows these bytes; `{n}` is the edit count a refusal names. Fixture
/// `🧫️fixtures/🧫️history-notices/🔣️.json`; TS twin `HISTORY_NOTICE_LABELS`.
pub const HISTORY_NOTICE_LABELS: [(&str, &str, &str); 7] = [""",
        """/// (an open tool transaction, an exhausted or still replaying history, a whole-document load in flight, a history step whose
/// replay would leave errors, a withdrawal of a step that also changed other documents), by fault code — every shell shows
/// these bytes; `{n}` is the edit count a refusal names. Fixture `🧫️fixtures/🧫️history-notices/🔣️.json`; TS twin
/// `HISTORY_NOTICE_LABELS`.
pub const HISTORY_NOTICE_LABELS: [(&str, &str, &str); 8] = [""",
    ),
    (
        """    ("history.step-blocked", "Later mutations would end with errors — fix or withdraw them first.", "Spätere Mutationen würden mit Fehlern enden — zuerst beheben oder zurückziehen."),
];""",
        """    ("history.step-blocked", "Later mutations would end with errors — fix or withdraw them first.", "Spätere Mutationen würden mit Fehlern enden — zuerst beheben oder zurückziehen."),
    ("history.unit-spans-documents", "This step also changed other documents, so it cannot be withdrawn on its own yet.", "Dieser Schritt hat auch andere Dokumente geändert und lässt sich daher noch nicht einzeln zurückziehen."),
];""",
    ),
]

KERNEL_TS = [
    (
        """/** ⏪️ The live history-edit session of one instance, mirrored from Rust `HistoryTimeTravel`; absent while none is open. */
export type HistoryTimeTravel = {
""",
        """/** ⛔️ The first mutation whose replay outcome blocks finalizing, mirrored from Rust `HistoryTimeTravelProblem`: the
 * arguments of `historyEditBegin` (`store` = the composed member store that holds it, absent for the document's own). */
export type HistoryTimeTravelProblem = {
  readonly mutationId: string;
  readonly store?: string;
};

/** ⏪️ The live history-edit session of one instance, mirrored from Rust `HistoryTimeTravel`; absent while none is open. */
export type HistoryTimeTravel = {
""",
    ),
    (
        """  /** 🔁️ Whether `historyEditRerun` would start a replay now. */
  readonly rerunnable?: boolean;
};
""",
        """  /** 🔁️ Whether `historyEditRerun` would start a replay now. */
  readonly rerunnable?: boolean;
  /** 👉️ The first mutation whose replay outcome blocks finalizing; present exactly while `blocking`. */
  readonly nextProblem?: HistoryTimeTravelProblem;
};
""",
    ),
]

KERNEL_TS += [
    (
        """  { code: "history.step-blocked", en: "Later mutations would end with errors — fix or withdraw them first.", de: "Spätere Mutationen würden mit Fehlern enden — zuerst beheben oder zurückziehen." },
] as const;""",
        """  { code: "history.step-blocked", en: "Later mutations would end with errors — fix or withdraw them first.", de: "Spätere Mutationen würden mit Fehlern enden — zuerst beheben oder zurückziehen." },
  { code: "history.unit-spans-documents", en: "This step also changed other documents, so it cannot be withdrawn on its own yet.", de: "Dieser Schritt hat auch andere Dokumente geändert und lässt sich daher noch nicht einzeln zurückziehen." },
] as const;""",
    ),
]

KERNEL_NOTICES_FIXTURE = [
    (
        """and this replica's own history step (a deferred undo, redo, checkout or finalize) whose replay would leave errors (gap N17, audit W1E-3). The Rust kernel""",
        """this replica's own history step (a deferred undo, redo, checkout or finalize) whose replay would leave errors (gap N17, audit W1E-3), and the withdrawal of a step that also changed other documents (design §22.1: a cross-document unit is not withdrawn on its own). The Rust kernel""",
    ),
    (
        """    { "code": "history.step-blocked", "en": "Later mutations would end with errors — fix or withdraw them first.", "de": "Spätere Mutationen würden mit Fehlern enden — zuerst beheben oder zurückziehen." }
  ]""",
        """    { "code": "history.step-blocked", "en": "Later mutations would end with errors — fix or withdraw them first.", "de": "Spätere Mutationen würden mit Fehlern enden — zuerst beheben oder zurückziehen." },
    { "code": "history.unit-spans-documents", "en": "This step also changed other documents, so it cannot be withdrawn on its own yet.", "de": "Dieser Schritt hat auch andere Dokumente geändert und lässt sich daher noch nicht einzeln zurückziehen." }
  ]""",
    ),
]

KERNEL_SCHEMA = [
    (
        """        "rerunnable": { "type": "boolean" }
      }
    },
    "HistoryPatch": {""",
        """        "rerunnable": { "type": "boolean" },
        "nextProblem": { "$ref": "#/definitions/HistoryTimeTravelProblem" }
      }
    },
    "HistoryTimeTravelProblem": {
      "type": "object",
      "additionalProperties": false,
      "required": ["mutationId"],
      "properties": {
        "mutationId": { "type": "string", "minLength": 1 },
        "store": { "type": "string", "pattern": "^[^/]+/.+$" }
      }
    },
    "HistoryPatch": {""",
    ),
]

KERNEL_FIXTURE = [
    (
        """"timeTravel": { "sessionId": "1", "generation": 2, "stage": "reviewing", "worst": "fatal", "blocking": true, "acceptedCount": 2, "review": "blocked", "rerunnable": false }""",
        """"timeTravel": { "sessionId": "1", "generation": 2, "stage": "reviewing", "worst": "fatal", "blocking": true, "acceptedCount": 2, "review": "blocked", "rerunnable": false, "nextProblem": { "mutationId": "e-3#0" } }""",
    ),
    (
        """      "keys": ["seq:11"]
    }
  ],
  "invalid": [""",
        """      "keys": ["seq:11"]
    },
    {
      "id": "a-blocked-review-of-a-composed-member-names-its-store",
      "patch": {
        "cursor": 12,
        "commandFilter": "all",
        "timeTravel": { "sessionId": "3", "generation": 5, "stage": "reviewing", "worst": "error", "blocking": true, "acceptedCount": 1, "review": "blocked", "rerunnable": false, "nextProblem": { "mutationId": "c-7", "store": "content/flow-content-1" } }
      },
      "keys": []
    }
  ],
  "invalid": [""",
    ),
    (
        """"reason": "a member store is its `<slot>/<childId>` text" }
  ]""",
        """"reason": "a member store is its `<slot>/<childId>` text" },
    { "id": "next-problem-without-mutation", "patch": { "cursor": 1, "timeTravel": { "sessionId": "1", "generation": 0, "stage": "reviewing", "blocking": true, "nextProblem": { "store": "content/flow-content-1" } } }, "reason": "a next problem names its mutation" },
    { "id": "next-problem-not-an-object", "patch": { "cursor": 1, "timeTravel": { "sessionId": "1", "generation": 0, "stage": "reviewing", "blocking": true, "nextProblem": "e-3#0" } }, "reason": "a next problem is its mutation id and store, the arguments of historyEditBegin" }
  ]""",
    ),
]

TT_RS = [
    (
        "use semio_framework::kernel::{HistoryMutationEntry, HistoryMutationMessage, HistoryReprojection, HistoryReprojectionKind, HistoryTimeTravel, HistoryTimeTravelReview, HistoryTimeTravelStage, RequestId};",
        "use semio_framework::kernel::{HistoryMutationEntry, HistoryMutationMessage, HistoryReprojection, HistoryReprojectionKind, HistoryTimeTravel, HistoryTimeTravelProblem, HistoryTimeTravelReview, HistoryTimeTravelStage, RequestId};",
    ),
    (
        """    /// ⏪️ The session status every host renders its band from (design §10); `None` while inactive.
    pub fn status(&self) -> Option<HistoryTimeTravel> {""",
        """    /// 👉️ The first mutation, in replay order, whose outcome blocks finalizing (`MergePolicy::Normal`, the floor
    /// `ReplayReport::blocks_finalize` reads): what the status names as the next problem (design §22.2); `None` while the
    /// session's report does not block.
    fn next_problem(&self) -> Option<String> {
        self.session.report.as_ref()?.outcomes.iter().find(|outcome| outcome.worst.is_some_and(|worst| protocol::MergePolicy::Normal.rejects(worst))).map(|outcome| outcome.mutation_id.0.clone())
    }

    /// ⏪️ The session status every host renders its band from (design §10); `None` while inactive.
    pub fn status(&self) -> Option<HistoryTimeTravel> {""",
    ),
    (
        """            rerunnable: session.rerun_refusal().is_none(),
        })
    }
""",
        """            rerunnable: session.rerun_refusal().is_none(),
            next_problem: self.next_problem().map(|mutation_id| HistoryTimeTravelProblem { mutation_id, store: self.member_store() }),
        })
    }
""",
    ),
    (
        """        let next_problem = session.report.as_ref().and_then(|report| report.outcomes.iter().find(|outcome| outcome.worst.is_some_and(|worst| protocol::MergePolicy::Normal.rejects(worst)))).map(|outcome| outcome.mutation_id.0.clone());
""",
        """        let next_problem = self.next_problem();
""",
    ),
]


def files(_root):
    return {
        f"{KERNEL}/🦀️.rs": KERNEL_RS,
        f"{KERNEL}/🟦️.ts": KERNEL_TS,
        f"{KERNEL}/🧬️schema/🔣️history-patch/🔣️.json": KERNEL_SCHEMA,
        f"{KERNEL}/🧫️fixtures/🧫️history-patch/🔣️.json": KERNEL_FIXTURE,
        f"{KERNEL}/🧫️fixtures/🧫️history-notices/🔣️.json": KERNEL_NOTICES_FIXTURE,
        TT: TT_RS,
    }
