#!/usr/bin/env python3
"""Apply from the repo root in a patch window. `MemberHistoryDictionaryOwner`/`VerifiedMemberHistoryDictionary` expose `pub` fields
(`owners`, `pending`, `lookup`) whose types are private, so the companion test crate (`os-store-member-open-tests`, 41 E0446-style
"type is private" errors) cannot read the owner's retained state it is designed to assert. Fail-closed: exact anchors, one each."""
base="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/"
def rep(p,pairs):
    s=open(base+p).read()
    for a,b in pairs:
        assert s.count(a)==1,(p,a)
        s=s.replace(a,b)
    open(base+p,"w").write(s)
rep("🦀️.rs",[("#[path = \"📇️index/🦀️.rs\"]\nmod index;","#[path = \"📇️index/🦀️.rs\"]\npub mod index;"),("\nstruct DictionaryOwners {","\npub struct DictionaryOwners {"),("\nstruct PendingByte {","\npub struct PendingByte {"),("\nstruct DictionaryLookup {","\npub struct DictionaryLookup {")])
rep("📇️index/🦀️.rs",[("pub(super) struct DictionaryRange {","pub struct DictionaryRange {"),("pub(super) enum DictionaryIndexError {","pub enum DictionaryIndexError {"),("pub(super) struct RetainedDictionaryIndex {","pub struct RetainedDictionaryIndex {")])
