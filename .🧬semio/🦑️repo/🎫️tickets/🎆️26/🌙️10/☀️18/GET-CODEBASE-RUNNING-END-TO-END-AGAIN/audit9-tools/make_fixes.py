"""🩹 Rebuilds audit9's corrected copies from the lanes' CURRENT staged batches with anchored edits (each anchor exactly once, exit 1 = anchor lost).
usage: python3 -I make_fixes.py   (rewrites audit9-tools/staged/{multiwriter,jloss}: files/ + manifest `new` hashes; base/, stage.py and verify.sh stay the lane's)
multiwriter: a builder that changed a stub of its own owners (added, removed, re-targeted, other kind/config/native root/tag) refuses to merge onto a head in which a peer changed any owner (symmetric to the peer-side `structure` rule; the peer computed against the old stubs).
jloss: an in-process replay after `UserDropped` is attempted only on a volume with persistent event history whose history is the one the stream started on (device UUID non-empty and unchanged), as the supervisor's `--since` restart already requires; otherwise the drop stays a loss."""
import hashlib, json, pathlib, shutil, sys
C = pathlib.Path(__file__).resolve().parent.parent
OUT = C / "audit9-tools" / "staged"
sha = lambda data: hashlib.sha256(data).hexdigest()

MERGE_JS = [(
    ' for(const name of[...patch.drop,...patch.upserts.keys(),...patch.append.keys()])if(!isPseudoOwnerV1(name))ours.add(name);\n',
    ' for(const name of[...patch.drop,...patch.upserts.keys(),...patch.append.keys()])if(!isPseudoOwnerV1(name))ours.add(name);\n if(theirs.size)for(const name of sorted(ours))if(stubOf(mine.owners[name])!==stubOf(base.owners[name]))return{conflict:"structure",owner:name};\n',
), (
    ' * 🧮️ Validates the delta of one writer',
    ' * 🧮️ Validates (symmetrically for stubs: a peer\'s stub change and, once a peer changed any owner, a stub change of this writer both refuse, since every computation reads all stubs) the delta of one writer',
)]
MERGE_PY = [(
    '    for name in [*patch["drop"], *patch["upserts"]]:\n        if not pseudo(name):\n            ours.add(name)\n',
    '    for name in [*patch["drop"], *patch["upserts"]]:\n        if not pseudo(name):\n            ours.add(name)\n    if theirs:\n        for name in sorted(ours):\n            if stub(mine["owners"].get(name)) != stub(base["owners"].get(name)):\n                return {"conflict": "structure", "owner": name}\n',
), (
    'environment, a peer changed an owner\'s stub (presence, root, kind, targets, config, native root, tag), ',
    'environment, a peer changed an owner\'s stub (presence, root, kind, targets, config, native root, tag) or the writer changed\none of its owners\' stubs while a peer changed any owner, ',
)]
MERGE_TEST = [(
    'test("every decision over 600 seeded worlds equals the Python model, and every outcome kind occurs",()=>{',
    'test("a writer that added, removed or re-targeted a project refuses to merge onto a peer that changed any owner, and merges onto a peer that changed none",()=>{\n'
    ' const [entry]=seeded(1,11),base=entry!.base,copy=(side:Side):Side=>JSON.parse(JSON.stringify(side));\n'
    ' const head=copy(base);head.owners.n1={...base.owners.n1,fragment:hex("peer")};head.rows.n1=["n1/peer"];\n'
    ' const added=copy(base);added.owners.m9={...base.owners.n0,root:"pkg/m9",fragment:hex("m9")};added.rows.m9=["m9/a"];\n'
    ' expect(decide({base,head,mine:added,patch:{drop:[],upserts:{m9:["m9/a"]}}})).toEqual({conflict:"structure",owner:"m9"});\n'
    ' const removed=copy(base);delete removed.owners.n3;delete removed.rows.n3;\n'
    ' expect(decide({base,head,mine:removed,patch:{drop:["n3"],upserts:{}}})).toEqual({conflict:"structure",owner:"n3"});\n'
    ' const targeted=copy(base);targeted.owners.n4={...base.owners.n4,fragment:hex("t"),targets:["build","lint"]};\n'
    ' expect(decide({base,head,mine:targeted,patch:{drop:[],upserts:{n4:["n4/t"]}}})).toEqual({conflict:"structure",owner:"n4"});\n'
    ' const quiet=copy(base);quiet.rows["@env"]=["env/peer"];\n'
    ' expect((decide({base,head:quiet,mine:added,patch:{drop:[],upserts:{m9:["m9/a"]}}}) as any).conflict).toBeNull();\n'
    '});\n\n'
    'test("every decision over 600 seeded worlds equals the Python model, and every outcome kind occurs",()=>{',
)]
MERGE_SCHEMA = [(
    'a peer changed the stub of an owner (`stubFields`: presence, root, kind, targets, configuration, native root, tag), which every computation reads (`structure`);',
    'a peer changed the stub of an owner (`stubFields`: presence, root, kind, targets, configuration, native root, tag), which every computation reads, or the builder changed the stub of one of its own owners (added, removed or re-shaped a project) while a peer changed any owner, whose computation read the old stubs (`structure`);',
)]

WATCH_RS = [(
    '        replayed: AtomicU64,\n    }',
    '        replayed: AtomicU64,\n        uuid: String,\n    }',
), (
    '            replayed: AtomicU64::new(0),\n        }));',
    '            replayed: AtomicU64::new(0),\n            uuid: uuid.clone(),\n        }));',
), (
    '        let since = if attempts > RESYNCS_BEFORE_LOSS {',
    '        let since = if attempts > RESYNCS_BEFORE_LOSS || !fsevent::replayable(&state.uuid, &device_uuid(&state.root)) {',
), (
    '        if trail.len() >= capacity { trail.front().copied().unwrap_or(start).max(start) } else { start }\n    }',
    '        if trail.len() >= capacity { trail.front().copied().unwrap_or(start).max(start) } else { start }\n    }\n\n'
    '    /// 🗃 Whether a dropped delivery can be replayed from the volume history: only on a volume that keeps a persistent event history (FSEvents names it by a device UUID; none on a read-only volume or one with `.fseventsd/no_log`) and only while that history is the one the stream started on (a reset history gets a new UUID). Otherwise the drop stays a loss.\n'
    '    pub fn replayable(started: &str, now: &str) -> bool {\n        !started.is_empty() && started == now\n    }',
), (
    '        assert_eq!(fsevent::resume(&std::collections::VecDeque::new(), 4, 7), 7);\n    }',
    '        assert_eq!(fsevent::resume(&std::collections::VecDeque::new(), 4, 7), 7);\n    }\n\n'
    '    #[test]\n    fn drops_replay_only_from_the_persistent_history_the_stream_started_on() {\n'
    '        assert!(fsevent::replayable("A3E1", "A3E1"));\n        assert!(!fsevent::replayable("", ""));\n        assert!(!fsevent::replayable("A3E1", ""));\n        assert!(!fsevent::replayable("A3E1", "77B0"));\n    }',
), (
    '    /// ⏪ Replaces the stream by one that replays the volume history after the last safely delivered event id.',
    '    /// ⏪ Replaces the stream by one that replays the volume history after the last safely delivered event id (a loss instead when the volume keeps no persistent history or it was reset: `fsevent::replayable`).',
)]


def edit(text, edits, what):
    for old, new in edits:
        if text.count(old) != 1:
            sys.exit(f"anchor lost in {what}: {old[:90]!r}")
        text = text.replace(old, new)
    return text


def rebuild(name, source, plan):
    src, dst = C / source, OUT / name
    shutil.rmtree(dst, ignore_errors=True)
    shutil.copytree(src, dst, ignore=shutil.ignore_patterns("make.py", "manifest.json.pre-*", "new"))
    manifest = json.loads((src / "manifest.json").read_text(encoding="utf8"))
    for suffix, edits in plan.items():
        rel = next(r for r in manifest if r.endswith(suffix))
        path = dst / "files" / manifest[rel]["file"]
        path.write_text(edit(path.read_text(encoding="utf8"), edits, f"{name}:{suffix}"), encoding="utf8")
    return dst, manifest


def finish(dst, manifest):
    for rel, row in manifest.items():
        row["new"] = sha((dst / "files" / row["file"]).read_bytes())
    (dst / "manifest.json").write_text(json.dumps(manifest, indent=1, ensure_ascii=False), encoding="utf8")
    print("[DEBUG] rebuilt", dst.name, len(manifest), "files")


dst, manifest = rebuild("multiwriter", "builderlock-tools/staged/multiwriter", {
    "🔀️merge/🟨️.mjs": MERGE_JS,
    "🔀️merge/🧪️tests/🐍️oracle/🐍️.py": MERGE_PY,
    "🔀️merge/🧪️tests/🟦️.ts": MERGE_TEST,
    "🔀️merge/🧬️schema/🔣️.json": MERGE_SCHEMA,
})
finish(dst, manifest)

dst, manifest = rebuild("jloss", "jloss-tools/staged/jloss", {"🦀️rust/🦀️.rs": WATCH_RS})
rs_rel = next(r for r in manifest if r.endswith("🦀️rust/🦀️.rs"))
pin_rel = next(r for r in manifest if r.endswith("🦀️rust/🔣️.json"))
new_rs = (dst / "files" / manifest[rs_rel]["file"]).read_bytes()
pin_path = dst / "files" / manifest[pin_rel]["file"]
pin = json.loads(pin_path.read_text(encoding="utf8"))
pin["sources"][0].update(sha256=sha(new_rs), bytes=len(new_rs))
pin_path.write_bytes((json.dumps(pin, indent=1, ensure_ascii=False) + "\n").encode())
finish(dst, manifest)
