import sys,re
S="/tmp/claude-1000/-home-deck-semio/0ec9c8a1-06e8-45c6-b0fa-6d0c7cb86d6b/scratchpad/vocab-r1.txt"
R=[
("(tag 0, formerly `NoMutation`, is retired rather than reassigned)","(tag 0 is retired rather than reassigned)"),
("(tag 0, formerly `NoMutation`) is retired rather than reused","(tag 0) is retired rather than reused"),
("(0 was\n/// `NoMutation`'s, dropped by the `26/08/29/S-END-TO-END` mutation-leaf migration; the remaining\n/// tags are left as they were rather than renumbered down, since nothing needs them contiguous).","(tag 0 is retired; the\n/// remaining tags are not renumbered down, since nothing needs them contiguous)."),
("(shifted down by one from the pre-migration 1-10 numbering: `NoMutation`'s tag `0` had no\n/// leaf and is gone with the variant).","(tag 0 has no leaf and is retired)."),
("`0` is retired along with\n/// `NoMutation`, not reused)","`0` is retired, not reused)"),
("(ticket\n//! `26/08/29/S-END-TO-END`): `NoMutation` was dropped (`#[derive(dsl::Mutations)]` requires every\n//! variant to wrap exactly one leaf payload and a unit variant wraps none; `no` is not an approved\n//! semantic verb either), and every remaining variant now wraps its own `dsl::MutationLeaf` struct\n//! instead of carrying its fields as a struct-literal directly.","(ticket\n//! `26/08/29/S-END-TO-END`): `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one\n//! leaf payload, so no unit variant exists and every variant wraps its own `dsl::MutationLeaf` struct\n//! instead of carrying its fields as a struct-literal directly."),
("the W1b `SetSnapshot`-only\n//! scaffold: scalar","the whole-snapshot-only\n//! scaffold: scalar"),
("replacing\n//! the W1b `SetSnapshot`-only scaffold.","replacing\n//! the whole-snapshot-only scaffold."),
("(e.g. `SetSnapshot`'s whole snapshot, `InsertChannel`'s channel)","(e.g. a whole snapshot, `InsertChannel`'s channel)"),
("`SetSnapshot{snapshot}` reaches","a whole-snapshot payload reaches"),
("`SetSnapshot`'s whole snapshot, and every","a whole snapshot payload, and every"),
("only `SetSnapshot`/\n/// `InsertBlock`/`SetBlockContent`'s","only `InsertBlock`/\n/// `SetBlockContent`'s"),
("Beyond the baseline `SetSnapshot`, this\n/// addresses","This\n/// addresses"),
("`SetSnapshot{snapshot: SemioVideoSnapshot}` alone would hit","a whole-`SemioVideoSnapshot` payload alone would hit"),
("`SetSnapshot`'s whole `SemioVideoSnapshot`, and","a whole `SemioVideoSnapshot`, and"),
("e.g. `SetSnapshot`'s whole\n/// snapshot — never","e.g. a whole\n/// snapshot — never"),
("a sparse\n//! `SetSnapshot` plus insert","a sparse\n//! whole-snapshot replacement plus insert"),
("//! 🪆️ Mutation-leaf migration: `NoMutation` is dropped (the derive requires every variant to wrap\n//! exactly one leaf payload, and `no` is not an approved semantic verb).","//! 🪆️ Mutation-leaf migration: the derive requires every variant to wrap\n//! exactly one leaf payload, so there is no unit variant."),
("`object`, `kit` — no `NoMutation`/`Default` of their own any more, so their harmless\n//! representative case below is `SetSnapshot` with a default snapshot); `flow` is not yet migrated\n//! (still carries its own `NoMutation`/`Default`), so its representative case uses `::default()`","`object`, `kit` — no unit/`Default` variant of their own, so their harmless\n//! representative case below wraps a default snapshot); `flow` is not yet migrated\n//! (still carries its own unit/`Default` variant), so its representative case uses `::default()`"),
("All 19 top-level [`SemioMutation`] tags (`SetSnapshot` and each of the 18 wrapped-kind","All 19 top-level [`SemioMutation`] tags (the whole-snapshot tag and each of the 18 wrapped-kind"),
("`0` = `SetSnapshot`, `1..=18`","`0` = the whole-snapshot tag, `1..=18`"),
("for `SetSnapshot`, the wrapped snapshot's own real","for the whole-snapshot tag, the wrapped snapshot's own real"),
("even though `NoMutation` is no longer an enum variant (`no` is not an\n/// approved semantic verb for `#[derive(dsl::Mutations)]`)","even though the identity scenario is not an enum variant (`no` is not an\n/// approved semantic verb for `#[derive(dsl::Mutations)]`)"),
("`NoMutation` was dropped:\n/// `#[derive(dsl::Mutations)]` requires","`#[derive(dsl::Mutations)]`\n/// requires"),
("`NoMutation` is dropped —\n//! `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one leaf payload, and a unit\n//! variant wraps none. Every former `None => vec![BcfMutation::NoMutation]` inverse fallback below","`#[derive(dsl::Mutations)]`\n//! requires every variant to wrap exactly one leaf payload, and a unit\n//! variant wraps none. Every inverse fallback below"),
("`Vec::new()` (the semantic replacement for `NoMutation`)","`Vec::new()` (no inverse steps needed)"),
("`SetSnapshot`/`SetPaneObjects`-as-whole-doc-","whole-snapshot/`SetPaneObjects`-as-whole-doc-"),
("(`SetSnapshot` is banned outright —","(whole-snapshot replace is banned outright —"),
("including the banned `SetSnapshot` whole-document-replace variant — is gone","including the banned whole-document-replace variant — is gone"),
("The whole-document `SetSnapshot` mutation that used to","The whole-document mutation that used to"),
("(`SetSnapshot` — see","(whole-snapshot replace — see"),
("matches the migration recipe's `SetSnapshot`/\n    /// `SetSnapshotJson` collapse.","matches the migration recipe's whole-snapshot\n    /// collapse."),
("emits (`SetSnapshot` is banned — see","emits (whole-snapshot replace is banned — see"),
("(`SetSnapshot` is banned outright) —","(whole-snapshot replace is banned outright) —"),
("is banned `SetSnapshot`-shaped vocabulary","is banned whole-snapshot-shaped vocabulary"),
("//#region 🔖️SetSnapshot","//#region 🔖️WholeSnapshot"),("//#endregion 🔖️SetSnapshot","//#endregion 🔖️WholeSnapshot"),
]
for l in open(S,encoding="utf-8"):
    f="/home/deck/semio/"+l.split("\t")[1]
    if "vcs" in f and "no-operation" in f or "set-snapshot" in f: continue
    t=open(f,encoding="utf-8").read();o=t
    for a,b in R:t=t.replace(a,b)
    if t!=o:open(f,"w",encoding="utf-8").write(t)
