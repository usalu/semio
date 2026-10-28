"""🏷 Rebuilds staged batch `status` from the CURRENT live files: lease status and order diagnostics speak modes and priorities in the chosen language (geteilt/exklusiv, interaktiv/Agent/Hintergrund) and never print a counter of an unknown total as "N of 0"."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from stagelib import build, sub, after
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", "..", "..", ".."))
L = "🧰️framework/🔨️modules/🏃️process/🔒️leases/"
ORDER_S, ORDER = L + "🧭️order/🧬️schema/🔣️.json", L + "🧭️order/🟦️.ts"
STATUS_S, STATUS = L + "📋️status/🧬️schema/🔣️.json", L + "📋️status/🟦️.ts"
STAND_S, STAND_T = L + "📈️standing/🧬️schema/🔣️.json", L + "📈️standing/🧪️tests/🟦️.ts"
MODES6 = '''        "modeShared": {
          "en": "shared",
          "de": "geteilt"
        },
        "modeExclusive": {
          "en": "exclusive",
          "de": "exklusiv"
        },
'''
def order_s(t): return sub(t, '        "blocker": {\n          "en": ", held by process', MODES6 + '        "blocker": {\n          "en": ", held by process')
def order(t):
    t = sub(t, 'blocker:catalogue.blocker[language]},values={held:', 'blocker:catalogue.blocker[language]},mode=(value:string):string=>value==="shared"?catalogue.modeShared[language]:value==="exclusive"?catalogue.modeExclusive[language]:value,values={held:')
    t = sub(t, 'heldMode:row.heldMode,', 'heldMode:mode(row.heldMode),')
    return sub(t, 'requestedMode:row.requestedMode,', 'requestedMode:mode(row.requestedMode),')
def status_s(t): return sub(t, '        "holder": {\n          "en": "HELD ', MODES6 + '        "holder": {\n          "en": "HELD ')
def status(t):
    t = sub(t, 'lines.push(fill(text.holder,{mode:row.mode,', 'lines.push(fill(text.holder,{mode:modeText(row.mode),')
    t = sub(t, 'lines.push(fill(text.waiter,{mode:row.mode,', 'lines.push(fill(text.waiter,{mode:modeText(row.mode),')
    t = sub(t, 'command:(row.priority===undefined?"":`  [${row.priority}]`)+command(row)', 'command:(row.priority===undefined?"":`  [${leaseStandingTextV1(language,`priority.${row.priority}`)}]`)+command(row)')
    t = sub(t, 'name=(row:{resource:string;hash:string})=>', 'modeText=(mode:LeaseModeV1):string=>mode==="shared"?text.modeShared:text.modeExclusive,name=(row:{resource:string;hash:string})=>')
    t = sub(t, 'standing.done===undefined?say("holderPhase",{phase:standing.phase}):say("holderProgress",{phase:standing.phase,done:standing.done,total:standing.total??0})', 'standing.done===undefined?say("holderPhase",{phase:standing.phase}):standing.total===undefined||standing.total===0?say("holderCount",{phase:standing.phase,done:standing.done}):say("holderProgress",{phase:standing.phase,done:standing.done,total:standing.total})')
    return sub(t, 'say("status.standing",{priority:standing.priority??"?",', 'say("status.standing",{priority:standing.priority===undefined?"?":say(`priority.${standing.priority}`),')
def stand_s(t):
    return sub(t, '''    "holderProgress": {
     "en": "{phase}: {done} of {total}",
     "de": "{phase}: {done} von {total}"
    },
''', '''    "holderProgress": {
     "en": "{phase}: {done} of {total}",
     "de": "{phase}: {done} von {total}"
    },
    "holderCount": {
     "en": "{phase}: {done}",
     "de": "{phase}: {done}"
    },
    "priority.interactive": {
     "en": "interactive",
     "de": "interaktiv"
    },
    "priority.agent": {
     "en": "agent",
     "de": "Agent"
    },
    "priority.background": {
     "en": "background",
     "de": "Hintergrund"
    },
''')
TEST = r'''
test("lease status and order diagnostics name modes and priorities in the chosen language and never print a counter of an unknown total, checked against SQLite", async () => {
  const { formatLeaseViolationV1 } = await import("../../🧭️order/🟦️.ts"), orderSchema = (await import("../../🧭️order/🧬️schema/🔣️.json")).default, db = new Database(":memory:");
  const word = (json: unknown, key: string, language: string) => (db.query("select json_extract(?1,'$.\"'||?2||'\".'||?3) w").get(JSON.stringify(json), key, language) as { w: string }).w;
  const holder = { resource: "native-fixed-image:/x/y", hash: "a".repeat(64), mode: "exclusive" as const, pid: 7, task: 1, sinceMs: 0, heldSeconds: 12, class: "image", rank: 7, source: "registry" as const, standing: { state: "progressing" as const, priority: "agent" as const, phase: "dependencies:generation-verify", done: 81920, total: 0, implementationAgeSeconds: 3 } };
  const waiter = { resource: "x", hash: "b".repeat(64), mode: "shared" as const, pid: 8, task: 2, sinceMs: 0, waitedSeconds: 4, class: "image", rank: 7, source: "queue" as const, blockers: [] as string[], priority: "background" as const };
  const report = { version: 1 as const, directory: "/d", atMs: 0, waitFor: [], cycles: [], violations: [], holders: [holder, { ...holder, mode: "shared" as const, standing: undefined }], waiters: [waiter] };
  try {
    for (const language of ["en", "de"] as const) {
      const lines = formatLeaseReportV1(language, report as never), first = language === "en" ? "HELD" : "GEHALTEN", second = language === "en" ? "WAITING" : "WARTET";
      expect(lines[1], language).toMatch(new RegExp(`^${first}\\s+${word(statusSchema.definitions.Messages.const, "modeExclusive", language)} `, "u"));
      expect(lines[3], language).toMatch(new RegExp(`^${first}\\s+${word(statusSchema.definitions.Messages.const, "modeShared", language)} `, "u"));
      expect(lines[4], language).toMatch(new RegExp(`^${second}\\s+${word(statusSchema.definitions.Messages.const, "modeShared", language)} `, "u"));
      expect(lines[4], language).toContain(`[${word(schema.definitions.Messages.const, "priority.background", language)}]`);
      expect(lines[2], language).toContain(word(schema.definitions.Messages.const, "priority.agent", language));
      expect(lines[2], language).toContain("dependencies:generation-verify: 81920");
      expect(lines[2], language).not.toMatch(/ (?:of|von) 0\b/u);
      const violation = formatLeaseViolationV1(language, { kind: "out-of-order", held: "h", heldClass: "image", heldRank: 7, heldMode: "shared", requested: "r", requestedClass: "cargo", requestedRank: 2, requestedMode: "exclusive" } as never);
      expect(violation, language).toContain(`${word(orderSchema.definitions.Messages.const, "modeShared", language)}`);
      if (language === "de") { expect(violation).toContain("geteilt gehalten"); expect(violation).not.toMatch(/\bshared\b|\bexclusive\b/u); expect(lines.join("\n")).not.toMatch(/\bshared\b|\bexclusive\b/u); }
    }
    expect(word(statusSchema.definitions.Messages.const, "modeShared", "de")).toBe("geteilt");
    expect(word(statusSchema.definitions.Messages.const, "modeExclusive", "de")).toBe("exklusiv");
  } finally { db.close(); }
});
'''
def stand_t(t): return t if t.endswith("\n") is False and False else t.rstrip("\n") + "\n" + TEST
VERIFY = r'''PY=python3
"$PY" -I "$HERE/stage.py" apply --root "$ROOT"
cd "$ROOT/🧰️framework/🔨️modules/🏃️process/🔒️leases"
for d in 📈️standing/🧪️tests 🧭️order/🧪️tests; do (cd "$d" && SEMIO_TEST_ARTIFACT_DIR="${SEMIO_TEST_ARTIFACT_DIR:?set a short artifact dir under the ticket generated folder}" bun test ./🟦️.ts) || exit 1; done
echo "status batch: GREEN"
'''
DOC = "🏷 Staged batch `status` (lane progress): lease status and order diagnostics speak modes and priorities in the chosen language and never print a counter of an unknown total as `N of 0`."
if __name__ == "__main__":
    build(sys.argv[2] if len(sys.argv) > 2 else "status", ROOT, [(ORDER_S, order_s, after("umlaut", ORDER_S)), (ORDER, order), (STATUS_S, status_s), (STATUS, status), (STAND_S, stand_s), (STAND_T, stand_t)], VERIFY, DOC)
