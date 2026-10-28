#!/usr/bin/env python3
"""🎰️ dashplan-tools: a Bazel slot holds the package and the plan record of its LAST selection only, so the slot order no longer prefers a slot that merely remembers the key (the nested dashboard run and the course glued themselves onto one slot and evicted each other's plan record on every run).
Usage: python3 -I edit.py <root> apply-tests|apply-code|apply|check|revert. Anchored, all-or-nothing, idempotent; backups under dashplan-tools/base/<n> (first apply only)."""
import json, os, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
Z = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions"
README = "🧰️framework/🛍️products/🦑️repo/README.md"

RANK_OLD = ("/** 🥇️ Orders every slot by how well it serves a selection: the slot whose last selection it is (warm server, warm package), then slots that still keep its package, then slots never used, then the least recently used; ties go to the slot the agent session prefers (its preference modulo the slot count) and then keep the slot number order, so concurrent agents start on different slots. */\n"
 "export function rankBazelSlotsV1(slots:readonly string[],states:Readonly<Record<string,BazelSlotStateV1|undefined>>,key:string,preference?:number):readonly string[]{\n"
 " const tier=(slot:string)=>{const state=states[slot];return state===undefined?2:state.key===key?0:state.keys.includes(key)?1:3;},index=(slot:string)=>preference===undefined?Number(slot.slice(1)):(slots.indexOf(slot)-preference%slots.length+slots.length)%slots.length,recency=(left:string,right:string)=>{const first=tier(left);return first<2?states[right]!.usedAt-states[left]!.usedAt:first===3?states[left]!.usedAt-states[right]!.usedAt:0;};\n")
RANK_NEW = ("/** 🥇️ Orders every slot by how well it serves a selection: the slot whose last selection it is (warm server, and the package and plan record it still holds), then slots never used, then the least recently used; a slot holds the package of its last selection only, so a slot that merely remembers the key in `keys` is an ordinary used slot and choosing it would evict the plan record of the selection it holds. Ties go to the slot the agent session prefers (its preference modulo the slot count) and then keep the slot number order, so concurrent agents start on different slots. */\n"
 "export function rankBazelSlotsV1(slots:readonly string[],states:Readonly<Record<string,BazelSlotStateV1|undefined>>,key:string,preference?:number):readonly string[]{\n"
 " const tier=(slot:string)=>{const state=states[slot];return state===undefined?1:state.key===key?0:2;},index=(slot:string)=>preference===undefined?Number(slot.slice(1)):(slots.indexOf(slot)-preference%slots.length+slots.length)%slots.length,recency=(left:string,right:string)=>{const first=tier(left);return first===0?states[right]!.usedAt-states[left]!.usedAt:first===2?states[left]!.usedAt-states[right]!.usedAt:0;};\n")

LAW = '''test("Bazel slot choice: selections that once shared a slot, and selections that alternate, each keep a slot of their own and stay warm",async()=>{
 const a="0000000000000001",b="0000000000000002",c="0000000000000003",plan={slots:3,services:0},written:Record<string,BazelSlotStateV1>={s0:nextBazelSlotStateV1("s0",nextBazelSlotStateV1("s0",undefined,a,1),b,2)},holds=new Set<string>();let clock=10;
 const port={async claim(slot:string){if(holds.has(slot))return undefined;holds.add(slot);return ()=>holds.delete(slot);},async gate(){return ()=>undefined;},queued(){return 0;},async spare(){return 0;},async floorSpare(){return Infinity;},enqueue(){return {first:()=>true,leave(){}};},async read(slot:string){return written[slot] as never;},async write(slot:string,state:unknown){written[slot]=state as BazelSlotStateV1;},async sleep(){},now:()=>clock++,progress(){}};
 const signal=new AbortController().signal,seen:Record<string,{slot:string;mode:string}[]>={[a]:[],[b]:[],[c]:[]};
 for(let round=0;round<4;round++)for(const key of[a,b,c]){const session=await acquireBazelSessionV1(port as never,plan,{language:"en",key,signal});await session.release();seen[key]!.push({slot:session.slot,mode:session.mode});}
 for(const key of[a,b,c]){expect(new Set(seen[key]!.map(row=>row.slot)).size,key+" never moves").toBe(1);expect(seen[key]!.slice(1).map(row=>row.mode),key+" stays warm").toEqual(["warm","warm","warm"]);}
 expect(new Set([a,b,c].map(key=>seen[key]![0]!.slot)).size).toBe(3);
});

'''
LAW_ANCHOR = 'test("launcher dedupe turns identical large launcher copies of an idle output base'

EDITS = [
 ("tests", Z + "/🧪️tests/🟦️.ts", [
  ('test("the slot order follows warm, kept, unused, then least recently used and agrees with an SQL ORDER BY"', 'test("the slot order follows warm, unused, then least recently used and agrees with an SQL ORDER BY"'),
  ("tier=state===undefined?2:state.key===row.key?0:state.keys.includes(row.key)?1:3;", "tier=state===undefined?1:state.key===row.key?0:2;"),
  ("CASE WHEN tier IN (0,1) THEN -used WHEN tier=3 THEN used ELSE 0 END", "CASE WHEN tier=0 THEN -used WHEN tier=2 THEN used ELSE 0 END"),
  (LAW_ANCHOR, LAW + LAW_ANCHOR),
 ]),
 ("tests", Z + "/🧪️tests/🧫️fixtures/🔣️.json", "FIXTURE"),
 ("code", Z + "/🟦️.ts", [(RANK_OLD, RANK_NEW)]),
 ("code", Z + "/🧬️schema/🔣️.json", [("the keys of the packages it still keeps, newest first, and the time", "the keys of the selections it served before, newest first (a record only: its package holds the last selection alone), and the time")]),
 ("code", README, [("a session prefers the slot whose last selection it is, then a slot that keeps the package, then a never-used slot, then the least recently used, so an edit-run-edit loop keeps its warm server.", "a session prefers the slot whose last selection it is (a slot keeps the package and the plan record of its last selection only), then a never-used slot, then the least recently used, so an edit-run-edit loop keeps its warm server and two alternating selections never evict each other.")]),
]


def fixture(path):
    return subprocess.run([sys.executable, "-I", os.path.join(HERE, "rank_oracle.py"), "emit", path], capture_output=True, check=True).stdout.decode("utf8")


def main():
    root, mode = sys.argv[1], sys.argv[2]
    chosen = {"apply-tests": {"tests"}, "apply-code": {"code"}, "apply": {"tests", "code"}}.get(mode, set())
    plans, bad = [], []
    for index, (part, rel, pairs) in enumerate(EDITS):
        path = os.path.join(root, rel)
        text = open(path, encoding="utf8").read()
        new = text
        if mode in ("apply-tests", "apply-code", "apply", "check"):
            if pairs == "FIXTURE":
                want = fixture(path) if part in chosen or mode == "check" else text
                if mode == "check" and want == text:
                    pass
                new = want
            else:
                for old, replacement in pairs:
                    count = new.count(old)
                    if replacement in new and count == 0:
                        continue
                    if count == 1:
                        new = new.replace(old, replacement) if part in chosen else new
                    elif mode != "check" or part in chosen:
                        bad.append("%s: anchor %r found %d times" % (rel, old[:60], count))
        plans.append((index, rel, path, text, new))
    if mode == "revert":
        for index, rel, path, _, _ in plans:
            backup = os.path.join(HERE, "base", str(index))
            if os.path.exists(backup):
                shutil.copy2(backup, path)
                print("restored", rel)
        return
    for line in bad:
        print("REFUSED", line)
    if bad:
        sys.exit(1)
    for index, rel, path, text, new in plans:
        state = "CHANGE" if new != text else "UNCHANGED"
        print(state, rel)
        if mode != "check" and new != text:
            backup = os.path.join(HERE, "base", str(index))
            if not os.path.exists(backup):
                os.makedirs(os.path.dirname(backup), exist_ok=True)
                shutil.copy2(path, backup)
            open(path + ".tmp", "w", encoding="utf8").write(new)
            os.replace(path + ".tmp", path)


main()
